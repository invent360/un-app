-- Migration 00029: License Issuance Model Enhancement
-- Phase 4 (P4-04): Reservation and issuance model with locking
--
-- Provides:
-- - Issuance state machine (draft -> published -> reserved -> issued -> claimed)
-- - Eligibility context capture at reservation time
-- - Enhanced locking and concurrency handling
-- - Issuance audit trail

-- ============================================
-- ISSUANCE STATE ENUM
-- ============================================
-- Represents the lifecycle state of a license

DO $$ BEGIN
    CREATE TYPE issuance_state AS ENUM (
        'draft',        -- Created but not published
        'published',    -- Available for reservation
        'reserved',     -- Temporarily held for a user
        'issued',       -- Assigned to a user (pending acceptance)
        'claimed',      -- User has accepted/claimed
        'cancelled',    -- Reservation/issuance was cancelled
        'expired',      -- License validity period ended
        'withdrawn'     -- Administratively withdrawn
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- ISSUANCE COLUMNS ON LICENSES
-- ============================================

-- Issuance state (replaces simple claimed boolean for more granular tracking)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issuance_state issuance_state DEFAULT 'draft';

-- Issuance tracking
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_to VARCHAR(255);  -- User ID

-- Context at issuance time (for audit/analytics)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_country CHAR(2);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_device_type VARCHAR(20);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_app_version VARCHAR(20);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS issued_ip_address INET;

-- Cancellation tracking
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS cancelled_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS cancelled_by VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS cancellation_reason TEXT;

-- Sync issuance_state with existing claimed boolean
UPDATE licenses SET issuance_state =
    CASE
        WHEN claimed = true THEN 'claimed'::issuance_state
        WHEN publication_status = 'published' THEN 'published'::issuance_state
        WHEN publication_status = 'withdrawn' THEN 'withdrawn'::issuance_state
        WHEN reserved_until IS NOT NULL AND reserved_until > NOW() THEN 'reserved'::issuance_state
        ELSE 'draft'::issuance_state
    END
WHERE issuance_state IS NULL OR issuance_state = 'draft';

-- Index for issuance state queries
CREATE INDEX IF NOT EXISTS idx_licenses_issuance_state
ON licenses (issuance_state);

-- Composite index for finding available licenses
CREATE INDEX IF NOT EXISTS idx_licenses_issuable
ON licenses (issuance_state, valid_from, valid_to, split_type)
WHERE issuance_state = 'published';

-- ============================================
-- RESERVATION CONTEXT COLUMNS
-- ============================================
-- Capture eligibility context at reservation time

ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS user_id VARCHAR(255);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS country_code CHAR(2);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS device_type VARCHAR(20);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS app_version VARCHAR(20);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS os_version VARCHAR(20);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS ip_address INET;
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS eligibility_ruleset_id INT REFERENCES eligibility_rulesets(id);
ALTER TABLE license_reservations ADD COLUMN IF NOT EXISTS referral_code VARCHAR(50);

-- Index for finding reservations by user
CREATE INDEX IF NOT EXISTS idx_reservations_user
ON license_reservations (user_id, status);

-- ============================================
-- ISSUANCE LOG TABLE
-- ============================================
-- Detailed log of all issuance state transitions

CREATE TABLE IF NOT EXISTS license_issuance_log (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,

    -- State transition
    from_state issuance_state,
    to_state issuance_state NOT NULL,

    -- Actor
    actor_type VARCHAR(20) NOT NULL,  -- 'user', 'admin', 'system', 'scheduler'
    actor_id VARCHAR(255),

    -- Context
    reservation_id UUID,
    session_token VARCHAR(64),

    -- Eligibility context at time of action
    country_code CHAR(2),
    device_type VARCHAR(20),
    ip_address INET,

    -- Result
    success BOOLEAN NOT NULL DEFAULT TRUE,
    error_code VARCHAR(50),
    error_message TEXT,

    -- Metadata
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding transitions by license
CREATE INDEX IF NOT EXISTS idx_issuance_log_license
ON license_issuance_log (license_id, created_at DESC);

-- Index for finding transitions by actor
CREATE INDEX IF NOT EXISTS idx_issuance_log_actor
ON license_issuance_log (actor_id, created_at DESC)
WHERE actor_id IS NOT NULL;

-- Index for failed transitions
CREATE INDEX IF NOT EXISTS idx_issuance_log_failures
ON license_issuance_log (error_code, created_at DESC)
WHERE success = FALSE;

-- ============================================
-- CONCURRENCY HELPERS
-- ============================================

-- Function to atomically transition issuance state
CREATE OR REPLACE FUNCTION transition_issuance_state(
    p_license_id VARCHAR(66),
    p_from_state issuance_state,
    p_to_state issuance_state,
    p_actor_id VARCHAR(255),
    p_actor_type VARCHAR(20)
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_state issuance_state;
    v_updated BOOLEAN := FALSE;
BEGIN
    -- Lock and check current state
    SELECT issuance_state INTO v_current_state
    FROM licenses
    WHERE id = p_license_id
    FOR UPDATE;

    IF v_current_state IS NULL THEN
        RAISE EXCEPTION 'License not found: %', p_license_id;
    END IF;

    IF v_current_state = p_from_state THEN
        -- Perform transition
        UPDATE licenses
        SET issuance_state = p_to_state
        WHERE id = p_license_id;

        v_updated := TRUE;

        -- Log the transition
        INSERT INTO license_issuance_log (
            license_id, from_state, to_state, actor_id, actor_type, success
        ) VALUES (
            p_license_id, p_from_state, p_to_state, p_actor_id, p_actor_type, TRUE
        );
    ELSE
        -- Log failed transition
        INSERT INTO license_issuance_log (
            license_id, from_state, to_state, actor_id, actor_type, success, error_code, error_message
        ) VALUES (
            p_license_id, v_current_state, p_to_state, p_actor_id, p_actor_type, FALSE,
            'invalid_state', format('Expected state %s but found %s', p_from_state, v_current_state)
        );
    END IF;

    RETURN v_updated;
END;
$$ LANGUAGE plpgsql;

-- Function to find and reserve an available license atomically
CREATE OR REPLACE FUNCTION reserve_available_license(
    p_split_type VARCHAR(20),
    p_session_token VARCHAR(64),
    p_user_id VARCHAR(255),
    p_country_code CHAR(2),
    p_device_type VARCHAR(20),
    p_expires_at TIMESTAMPTZ
) RETURNS TABLE(
    license_id VARCHAR(66),
    lease_code VARCHAR(64),
    split_type VARCHAR(20)
) AS $$
DECLARE
    v_license_id VARCHAR(66);
    v_lease_code VARCHAR(64);
    v_split_type VARCHAR(20);
    v_now TIMESTAMPTZ := NOW();
BEGIN
    -- Find and lock first available license
    IF p_split_type IS NOT NULL THEN
        SELECT l.id, l.lease_code, l.split_type::text
        INTO v_license_id, v_lease_code, v_split_type
        FROM licenses l
        WHERE l.issuance_state = 'published'
          AND l.claimed = false
          AND (l.reserved_until IS NULL OR l.reserved_until < v_now)
          AND l.valid_from <= v_now
          AND l.valid_to > v_now
          AND l.split_type = p_split_type::split_type
          AND l.is_quarantined = false
          AND (l.publication_status = 'published' OR l.publication_status IS NULL)
        ORDER BY l.created_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED;
    ELSE
        SELECT l.id, l.lease_code, l.split_type::text
        INTO v_license_id, v_lease_code, v_split_type
        FROM licenses l
        WHERE l.issuance_state = 'published'
          AND l.claimed = false
          AND (l.reserved_until IS NULL OR l.reserved_until < v_now)
          AND l.valid_from <= v_now
          AND l.valid_to > v_now
          AND l.is_quarantined = false
          AND (l.publication_status = 'published' OR l.publication_status IS NULL)
        ORDER BY l.created_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED;
    END IF;

    IF v_license_id IS NULL THEN
        RETURN;
    END IF;

    -- Update license with reservation
    UPDATE licenses
    SET reserved_until = p_expires_at,
        reservation_token = p_session_token,
        issuance_state = 'reserved'
    WHERE id = v_license_id;

    -- Insert/update reservation record
    INSERT INTO license_reservations (
        license_id, session_token, user_id, reserved_at, expires_at, status,
        country_code, device_type
    ) VALUES (
        v_license_id, p_session_token, p_user_id, v_now, p_expires_at, 'active',
        p_country_code, p_device_type
    )
    ON CONFLICT (license_id) DO UPDATE
    SET session_token = p_session_token,
        user_id = p_user_id,
        reserved_at = v_now,
        expires_at = p_expires_at,
        status = 'active',
        released_at = NULL,
        country_code = p_country_code,
        device_type = p_device_type;

    -- Log the reservation
    INSERT INTO license_issuance_log (
        license_id, from_state, to_state, actor_id, actor_type,
        session_token, country_code, device_type, success
    ) VALUES (
        v_license_id, 'published', 'reserved', p_user_id, 'user',
        p_session_token, p_country_code, p_device_type, TRUE
    );

    -- Return result
    RETURN QUERY SELECT v_license_id, v_lease_code, v_split_type;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TYPE issuance_state IS 'License issuance lifecycle: draft -> published -> reserved -> issued -> claimed';

COMMENT ON COLUMN licenses.issuance_state IS 'Current state in the issuance lifecycle';
COMMENT ON COLUMN licenses.issued_at IS 'When license was issued (assigned) to a user';
COMMENT ON COLUMN licenses.issued_to IS 'User ID the license was issued to';
COMMENT ON COLUMN licenses.issued_country IS 'Country code at issuance time for compliance';

COMMENT ON COLUMN license_reservations.eligibility_ruleset_id IS 'Which eligibility ruleset was applied';
COMMENT ON COLUMN license_reservations.referral_code IS 'Referral code provided at reservation';

COMMENT ON TABLE license_issuance_log IS 'Audit trail of all issuance state transitions';
COMMENT ON FUNCTION transition_issuance_state IS 'Atomically transition license issuance state with logging';
COMMENT ON FUNCTION reserve_available_license IS 'Find and reserve an available license atomically with SKIP LOCKED';
