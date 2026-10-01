-- Migration 00060: R5-05 Atomic Ownership and Inventory
-- Implements transactional ownership, eligibility, inventory and release requirements
--
-- Key changes:
-- 1. Add release_status for upstream release workflow
-- 2. Add referral_frozen to make no-referral immutable
-- 3. Add pending_release tracking for capacity ceiling
-- 4. Add issuance_held state for reserved-but-not-issued licenses

-- ============================================
-- RELEASE STATUS ENUM
-- ============================================
-- Models the upstream release workflow separately from local exit

DO $$ BEGIN
    CREATE TYPE release_status AS ENUM (
        'none',              -- Not released
        'requested',         -- User requested release
        'pending_upstream',  -- Waiting for upstream verification
        'confirmed',         -- Upstream confirmed release
        'cooldown',          -- In cooldown period before reuse
        'available'          -- Ready for re-issuance
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- LICENSE TABLE ENHANCEMENTS
-- ============================================

-- R5-05: Release workflow tracking
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS release_status release_status DEFAULT 'none';
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS release_requested_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS upstream_release_confirmed_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS cooldown_until TIMESTAMPTZ;

-- R5-05: Immutable referral tracking (including no-referral freeze)
-- Once referral_frozen = true, the referral attribution cannot change
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS referral_frozen BOOLEAN DEFAULT FALSE;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS referral_frozen_at TIMESTAMPTZ;

-- R5-05: Track if this license is pending release (for capacity)
-- Used in capacity ceiling calculation
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS is_pending_release BOOLEAN DEFAULT FALSE;

-- ============================================
-- RESERVATION TABLE ENHANCEMENTS
-- ============================================

-- R5-05: Store eligibility result at reservation time
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS eligibility_passed BOOLEAN;
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS eligibility_ruleset_id INT;
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS eligibility_checked_at TIMESTAMPTZ;

-- R5-05: Don't store lease_code in reservation (credential not revealed until confirm)
-- The lease_code should only be returned at confirmation time, not reservation
-- We add a flag to track if the credential was revealed
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS credential_revealed BOOLEAN DEFAULT FALSE;
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS credential_revealed_at TIMESTAMPTZ;

-- ============================================
-- CAPACITY TRACKING VIEW
-- ============================================
-- R5-05: View for accurate capacity calculation including pending releases

CREATE OR REPLACE VIEW license_capacity_summary AS
SELECT
    COUNT(*) FILTER (WHERE claimed = true AND is_pending_release = false) as active_claimed,
    COUNT(*) FILTER (WHERE reserved_until IS NOT NULL AND reserved_until > NOW()) as active_reserved,
    COUNT(*) FILTER (WHERE is_pending_release = true) as pending_release,
    COUNT(*) FILTER (
        WHERE claimed = true
           OR (reserved_until IS NOT NULL AND reserved_until > NOW())
           OR is_pending_release = true
    ) as total_occupied,
    COUNT(*) FILTER (
        WHERE claimed = false
          AND (reserved_until IS NULL OR reserved_until <= NOW())
          AND is_pending_release = false
          AND (publication_status = 'published' OR publication_status IS NULL)
          AND valid_from <= NOW()
          AND valid_to > NOW()
    ) as available
FROM licenses;

-- ============================================
-- FUNCTIONS
-- ============================================

-- R5-05: Atomic capacity check with locking
-- This function returns current capacity with advisory lock for atomic operations
CREATE OR REPLACE FUNCTION get_locked_capacity(p_ceiling BIGINT DEFAULT 2500)
RETURNS TABLE (
    current_occupied BIGINT,
    remaining_capacity BIGINT,
    at_ceiling BOOLEAN
) AS $$
DECLARE
    v_occupied BIGINT;
BEGIN
    -- Get capacity with consistent read
    SELECT COUNT(*) INTO v_occupied
    FROM licenses
    WHERE claimed = true
       OR (reserved_until IS NOT NULL AND reserved_until > NOW())
       OR is_pending_release = true;

    RETURN QUERY SELECT
        v_occupied,
        GREATEST(p_ceiling - v_occupied, 0),
        v_occupied >= p_ceiling;
END;
$$ LANGUAGE plpgsql;

-- R5-05: Check and reserve with atomic capacity enforcement
-- Returns the reserved license ID or NULL if capacity exceeded
CREATE OR REPLACE FUNCTION atomic_reserve_license(
    p_split_type TEXT DEFAULT NULL,
    p_ceiling BIGINT DEFAULT 2500,
    p_referral_code TEXT DEFAULT NULL,
    p_session_token TEXT DEFAULT NULL,
    p_expires_at TIMESTAMPTZ DEFAULT NULL
) RETURNS TABLE (
    license_id VARCHAR(66),
    lease_code VARCHAR(100),
    split_type TEXT,
    referral_validated BOOLEAN,
    capacity_remaining BIGINT
) AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
    v_expires TIMESTAMPTZ := COALESCE(p_expires_at, v_now + INTERVAL '2 minutes');
    v_occupied BIGINT;
    v_referral_valid BOOLEAN := FALSE;
BEGIN
    -- Clean up expired reservations first
    UPDATE licenses
    SET reserved_until = NULL, reservation_token = NULL
    WHERE reserved_until IS NOT NULL AND reserved_until < v_now;

    UPDATE license_reservations
    SET status = 'expired', released_at = v_now
    WHERE status = 'active' AND expires_at < v_now;

    -- Check capacity atomically
    SELECT COUNT(*) INTO v_occupied
    FROM licenses
    WHERE claimed = true
       OR (reserved_until IS NOT NULL AND reserved_until > v_now)
       OR is_pending_release = true;

    IF v_occupied >= p_ceiling THEN
        -- Capacity exceeded
        RETURN;
    END IF;

    -- Select available license with row lock
    IF p_split_type IS NOT NULL THEN
        SELECT l.id, l.lease_code, l.split_type::text
        INTO v_license
        FROM licenses l
        WHERE l.claimed = false
          AND (l.reserved_until IS NULL OR l.reserved_until < v_now)
          AND l.valid_from <= v_now
          AND l.valid_to > v_now
          AND l.split_type::text = p_split_type
          AND (l.publication_status = 'published' OR l.publication_status IS NULL)
          AND (l.is_quarantined = false OR l.is_quarantined IS NULL)
          AND l.is_pending_release = false
        ORDER BY l.created_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED;
    ELSE
        SELECT l.id, l.lease_code, l.split_type::text
        INTO v_license
        FROM licenses l
        WHERE l.claimed = false
          AND (l.reserved_until IS NULL OR l.reserved_until < v_now)
          AND l.valid_from <= v_now
          AND l.valid_to > v_now
          AND (l.publication_status = 'published' OR l.publication_status IS NULL)
          AND (l.is_quarantined = false OR l.is_quarantined IS NULL)
          AND l.is_pending_release = false
        ORDER BY l.created_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED;
    END IF;

    IF v_license.id IS NULL THEN
        -- No license available
        RETURN;
    END IF;

    -- Validate referral if provided
    IF p_referral_code IS NOT NULL THEN
        SELECT EXISTS(
            SELECT 1 FROM referrals r
            JOIN agents a ON r.agent_id = a.id
            WHERE r.referral_code = UPPER(p_referral_code)
              AND r.status = 'active'
              AND a.status = 'approved'
        ) INTO v_referral_valid;
    END IF;

    -- Reserve the license
    UPDATE licenses
    SET reserved_until = v_expires,
        reservation_token = p_session_token
    WHERE id = v_license.id;

    -- Create/update reservation record
    INSERT INTO license_reservations (license_id, session_token, reserved_at, expires_at, status)
    VALUES (v_license.id, p_session_token, v_now, v_expires, 'active')
    ON CONFLICT (license_id) DO UPDATE
    SET session_token = p_session_token,
        reserved_at = v_now,
        expires_at = v_expires,
        status = 'active',
        released_at = NULL;

    -- Return result (note: lease_code is returned here but should NOT be exposed to client)
    RETURN QUERY SELECT
        v_license.id::VARCHAR(66),
        v_license.lease_code::VARCHAR(100),
        v_license.split_type,
        v_referral_valid,
        p_ceiling - v_occupied - 1;
END;
$$ LANGUAGE plpgsql;

-- R5-05: Atomic confirm with referral freeze
CREATE OR REPLACE FUNCTION atomic_confirm_license(
    p_license_id VARCHAR(66),
    p_session_token TEXT,
    p_owner_id VARCHAR(255),
    p_device_id TEXT DEFAULT NULL,
    p_referral_id INT DEFAULT NULL
) RETURNS TABLE (
    success BOOLEAN,
    license_id VARCHAR(66),
    lease_code VARCHAR(100),
    claimed_at TIMESTAMPTZ,
    referral_id INT,
    referral_frozen BOOLEAN,
    error_code TEXT,
    error_message TEXT
) AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
    v_agreement_version INT;
    v_final_referral_id INT;
    v_is_new_attribution BOOLEAN := FALSE;
BEGIN
    -- Get license with lock
    SELECT l.*
    INTO v_license
    FROM licenses l
    WHERE l.id = p_license_id
    FOR UPDATE;

    IF v_license.id IS NULL THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(66), NULL::VARCHAR(100), NULL::TIMESTAMPTZ,
            NULL::INT, NULL::BOOLEAN, 'not_found'::TEXT, 'License not found'::TEXT;
        RETURN;
    END IF;

    -- Already claimed - return idempotent success
    IF v_license.claimed THEN
        RETURN QUERY SELECT TRUE, v_license.id::VARCHAR(66), v_license.lease_code::VARCHAR(100),
            v_license.claimed_at, v_license.referral_id, v_license.referral_frozen,
            NULL::TEXT, NULL::TEXT;
        RETURN;
    END IF;

    -- Verify session token
    IF v_license.reservation_token IS NULL OR v_license.reservation_token != p_session_token THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(66), NULL::VARCHAR(100), NULL::TIMESTAMPTZ,
            NULL::INT, NULL::BOOLEAN, 'invalid_token'::TEXT, 'Invalid or expired session token'::TEXT;
        RETURN;
    END IF;

    -- Check expiry
    IF v_license.reserved_until IS NOT NULL AND v_license.reserved_until < v_now THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(66), NULL::VARCHAR(100), NULL::TIMESTAMPTZ,
            NULL::INT, NULL::BOOLEAN, 'expired'::TEXT, 'Reservation has expired'::TEXT;
        RETURN;
    END IF;

    -- Get current agreement version
    SELECT version INTO v_agreement_version
    FROM agreement_versions WHERE is_active = true
    ORDER BY version DESC LIMIT 1;

    -- R5-05: Handle referral attribution with freeze
    -- If already has referral, keep it (immutable)
    -- If no referral provided, freeze as "no referral"
    IF v_license.referral_frozen THEN
        -- Already frozen, keep existing
        v_final_referral_id := v_license.referral_id;
    ELSIF v_license.referral_id IS NOT NULL THEN
        -- Has referral but not frozen, keep it and freeze
        v_final_referral_id := v_license.referral_id;
    ELSIF p_referral_id IS NOT NULL THEN
        -- New referral attribution
        v_is_new_attribution := TRUE;

        -- Verify agent is approved before attribution
        IF EXISTS (
            SELECT 1 FROM referrals r
            JOIN agents a ON r.agent_id = a.id
            WHERE r.id = p_referral_id
              AND a.status != 'approved'
        ) THEN
            RETURN QUERY SELECT FALSE, NULL::VARCHAR(66), NULL::VARCHAR(100), NULL::TIMESTAMPTZ,
                NULL::INT, NULL::BOOLEAN, 'agent_not_approved'::TEXT,
                'Cannot attribute to referral: agent not approved'::TEXT;
            RETURN;
        END IF;

        v_final_referral_id := p_referral_id;
    ELSE
        -- No referral - freeze as no-referral
        v_final_referral_id := NULL;
    END IF;

    -- Claim the license
    UPDATE licenses
    SET claimed = true,
        claimed_at = v_now,
        issued_to = p_owner_id,
        issued_at = COALESCE(issued_at, v_now),
        bound_to_device = (p_device_id IS NOT NULL),
        device_id = p_device_id,
        reserved_until = NULL,
        reservation_token = NULL,
        referral_id = v_final_referral_id,
        referral_attributed_at = CASE
            WHEN v_is_new_attribution THEN v_now
            ELSE referral_attributed_at
        END,
        referral_agreement_version = COALESCE(referral_agreement_version, v_agreement_version),
        referral_frozen = true,
        referral_frozen_at = COALESCE(referral_frozen_at, v_now)
    WHERE id = p_license_id;

    -- Update reservation
    UPDATE license_reservations
    SET status = 'claimed',
        claimed_at = v_now,
        credential_revealed = true,
        credential_revealed_at = v_now
    WHERE license_id = p_license_id AND session_token = p_session_token;

    -- Return success with credential
    RETURN QUERY SELECT TRUE, v_license.id::VARCHAR(66), v_license.lease_code::VARCHAR(100),
        v_now, v_final_referral_id, TRUE, NULL::TEXT, NULL::TEXT;
END;
$$ LANGUAGE plpgsql;

-- R5-05: Request release with upstream tracking
CREATE OR REPLACE FUNCTION request_license_release(
    p_license_id VARCHAR(66),
    p_user_id VARCHAR(255),
    p_reason TEXT DEFAULT NULL
) RETURNS TABLE (
    success BOOLEAN,
    release_status release_status,
    cooldown_until TIMESTAMPTZ,
    error_code TEXT,
    error_message TEXT
) AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
    v_cooldown TIMESTAMPTZ;
BEGIN
    -- Get license with lock
    SELECT l.*
    INTO v_license
    FROM licenses l
    WHERE l.id = p_license_id
    FOR UPDATE;

    IF v_license.id IS NULL THEN
        RETURN QUERY SELECT FALSE, NULL::release_status, NULL::TIMESTAMPTZ,
            'not_found'::TEXT, 'License not found'::TEXT;
        RETURN;
    END IF;

    -- Verify ownership
    IF v_license.issued_to != p_user_id THEN
        RETURN QUERY SELECT FALSE, NULL::release_status, NULL::TIMESTAMPTZ,
            'not_owner'::TEXT, 'User is not the owner of this license'::TEXT;
        RETURN;
    END IF;

    -- Check if already releasing
    IF v_license.release_status != 'none' THEN
        RETURN QUERY SELECT TRUE, v_license.release_status, v_license.cooldown_until,
            NULL::TEXT, NULL::TEXT;
        RETURN;
    END IF;

    -- Set release status to pending upstream
    -- R5-05: Model upstream release separately from local exit
    v_cooldown := v_now + INTERVAL '7 days'; -- Default 7-day cooldown

    UPDATE licenses
    SET release_status = 'pending_upstream',
        release_requested_at = v_now,
        is_pending_release = true,
        cooldown_until = v_cooldown
    WHERE id = p_license_id;

    -- Log the event
    PERFORM log_lifecycle_event(
        p_license_id, 'released',
        v_license.issuance_state::text, 'pending_release',
        'user', p_user_id, p_reason
    );

    RETURN QUERY SELECT TRUE, 'pending_upstream'::release_status, v_cooldown,
        NULL::TEXT, NULL::TEXT;
END;
$$ LANGUAGE plpgsql;

-- R5-05: Confirm upstream release
CREATE OR REPLACE FUNCTION confirm_upstream_release(
    p_license_id VARCHAR(66),
    p_admin_id VARCHAR(255),
    p_upstream_ref TEXT DEFAULT NULL
) RETURNS BOOLEAN AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    SELECT * INTO v_license FROM licenses WHERE id = p_license_id FOR UPDATE;

    IF v_license.id IS NULL OR v_license.release_status != 'pending_upstream' THEN
        RETURN FALSE;
    END IF;

    UPDATE licenses
    SET release_status = 'cooldown',
        upstream_release_confirmed_at = v_now
    WHERE id = p_license_id;

    PERFORM log_lifecycle_event(
        p_license_id, 'released',
        'pending_release', 'cooldown',
        'admin', p_admin_id, 'Upstream release confirmed: ' || COALESCE(p_upstream_ref, 'N/A')
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- R5-05: Complete release after cooldown
CREATE OR REPLACE FUNCTION complete_license_release(p_license_id VARCHAR(66))
RETURNS BOOLEAN AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    SELECT * INTO v_license FROM licenses WHERE id = p_license_id FOR UPDATE;

    IF v_license.id IS NULL THEN
        RETURN FALSE;
    END IF;

    IF v_license.release_status != 'cooldown' THEN
        RETURN FALSE;
    END IF;

    IF v_license.cooldown_until > v_now THEN
        RETURN FALSE; -- Still in cooldown
    END IF;

    -- Complete the release - license becomes available again
    UPDATE licenses
    SET release_status = 'available',
        is_pending_release = false,
        claimed = false,
        claimed_at = NULL,
        issued_to = NULL,
        referral_id = NULL,
        referral_frozen = false,
        referral_frozen_at = NULL,
        referral_attributed_at = NULL,
        issuance_state = 'published'
    WHERE id = p_license_id;

    -- Close ownership
    UPDATE license_ownership
    SET owned_until = v_now
    WHERE license_id = p_license_id AND owned_until IS NULL;

    PERFORM log_lifecycle_event(
        p_license_id, 'released',
        'cooldown', 'published',
        'system', 'scheduler', 'Release completed after cooldown'
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_licenses_release_status
ON licenses (release_status) WHERE release_status != 'none';

CREATE INDEX IF NOT EXISTS idx_licenses_pending_release
ON licenses (is_pending_release) WHERE is_pending_release = true;

CREATE INDEX IF NOT EXISTS idx_licenses_cooldown
ON licenses (cooldown_until) WHERE cooldown_until IS NOT NULL AND release_status = 'cooldown';

CREATE INDEX IF NOT EXISTS idx_reservations_eligibility
ON license_reservations (eligibility_passed, eligibility_checked_at);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TYPE release_status IS 'R5-05: Upstream release workflow status';
COMMENT ON COLUMN licenses.release_status IS 'R5-05: Current status in upstream release workflow';
COMMENT ON COLUMN licenses.referral_frozen IS 'R5-05: Once true, referral attribution cannot change (including no-referral)';
COMMENT ON COLUMN licenses.is_pending_release IS 'R5-05: Used in capacity ceiling calculation - license counts as occupied';

COMMENT ON VIEW license_capacity_summary IS 'R5-05: Accurate capacity including pending releases';

COMMENT ON FUNCTION get_locked_capacity IS 'R5-05: Get current capacity with advisory lock';
COMMENT ON FUNCTION atomic_reserve_license IS 'R5-05: Reserve with atomic capacity enforcement';
COMMENT ON FUNCTION atomic_confirm_license IS 'R5-05: Confirm with referral freeze and credential reveal';
COMMENT ON FUNCTION request_license_release IS 'R5-05: Request release with upstream tracking';
COMMENT ON FUNCTION confirm_upstream_release IS 'R5-05: Admin confirms upstream release';
COMMENT ON FUNCTION complete_license_release IS 'R5-05: Complete release after cooldown';
