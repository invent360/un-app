-- Migration 00032: License Lifecycle - Cancel, Expiry, Release, Exposure
-- Phase 4 (P4-07): Full lifecycle tracking with exposure metrics
--
-- Provides:
-- - License cancellation with reason tracking
-- - Automatic expiry handling
-- - Release workflow for returning licenses
-- - Exposure tracking (time in each state)

-- ============================================
-- LIFECYCLE EVENT ENUM
-- ============================================

DO $$ BEGIN
    CREATE TYPE lifecycle_event AS ENUM (
        'created',           -- Initial creation
        'published',         -- Made available
        'reserved',          -- Temporarily held
        'issued',            -- Assigned to user
        'claimed',           -- User accepted
        'released',          -- User returned/released
        'cancelled',         -- Admin cancelled
        'expired',           -- Validity period ended
        'withdrawn',         -- Admin withdrew
        'reactivated'        -- Brought back from cancelled/expired
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- LIFECYCLE COLUMNS ON LICENSES
-- ============================================

-- Release tracking
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS released_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS released_by VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS release_reason TEXT;

-- Expiry handling
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS expired_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS expiry_notification_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS expiry_notification_at TIMESTAMPTZ;

-- Reactivation
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS reactivated_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS reactivated_by VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS original_valid_to TIMESTAMPTZ;  -- Before extension

-- Exposure tracking (cumulative time in states, in seconds)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS exposure_published_secs BIGINT DEFAULT 0;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS exposure_reserved_secs BIGINT DEFAULT 0;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS exposure_claimed_secs BIGINT DEFAULT 0;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS last_exposure_calc_at TIMESTAMPTZ;

-- ============================================
-- LICENSE LIFECYCLE LOG
-- ============================================
-- Complete audit trail of all lifecycle events

CREATE TABLE IF NOT EXISTS license_lifecycle_log (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,

    -- Event details
    event lifecycle_event NOT NULL,
    from_state VARCHAR(30),       -- Previous issuance_state
    to_state VARCHAR(30),         -- New issuance_state

    -- Actor
    actor_type VARCHAR(20) NOT NULL,  -- 'user', 'admin', 'system', 'scheduler'
    actor_id VARCHAR(255),

    -- Context
    reason TEXT,
    ip_address INET,
    user_agent TEXT,

    -- Exposure at this point
    exposure_snapshot JSONB,       -- Snapshot of exposure times at event

    -- Metadata
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for lifecycle by license
CREATE INDEX IF NOT EXISTS idx_lifecycle_log_license
ON license_lifecycle_log (license_id, created_at DESC);

-- Index for finding events by type
CREATE INDEX IF NOT EXISTS idx_lifecycle_log_event
ON license_lifecycle_log (event, created_at DESC);

-- Index for actor queries
CREATE INDEX IF NOT EXISTS idx_lifecycle_log_actor
ON license_lifecycle_log (actor_id, created_at DESC)
WHERE actor_id IS NOT NULL;

-- ============================================
-- EXPIRY NOTIFICATIONS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS license_expiry_notifications (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,
    owner_id VARCHAR(255),

    -- Notification type
    notification_type VARCHAR(30) NOT NULL,  -- '30_day', '7_day', '1_day', 'expired'
    days_until_expiry INT,

    -- Delivery
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivery_channel VARCHAR(20),  -- 'email', 'push', 'sms'
    delivery_status VARCHAR(20),   -- 'sent', 'delivered', 'failed'

    -- Reference
    notification_ref VARCHAR(255),  -- External notification system reference

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding notifications by license
CREATE INDEX IF NOT EXISTS idx_expiry_notifications_license
ON license_expiry_notifications (license_id, sent_at DESC);

-- Index for finding pending notifications
CREATE INDEX IF NOT EXISTS idx_expiry_notifications_type
ON license_expiry_notifications (notification_type, sent_at DESC);

-- ============================================
-- EXPOSURE SUMMARY VIEW
-- ============================================

CREATE OR REPLACE VIEW license_exposure_summary AS
SELECT
    l.id as license_id,
    l.issuance_state,
    l.created_at,
    l.published_at,
    l.claimed_at,
    l.released_at,
    l.expired_at,

    -- Calculate current exposure (including time in current state)
    l.exposure_published_secs +
        CASE WHEN l.issuance_state IN ('published', 'reserved') THEN
            EXTRACT(EPOCH FROM (NOW() - COALESCE(l.last_exposure_calc_at, l.published_at)))::BIGINT
        ELSE 0 END
    as total_published_secs,

    l.exposure_claimed_secs +
        CASE WHEN l.issuance_state = 'claimed' THEN
            EXTRACT(EPOCH FROM (NOW() - COALESCE(l.last_exposure_calc_at, l.claimed_at)))::BIGINT
        ELSE 0 END
    as total_claimed_secs,

    -- Time to claim (from publish to claim)
    CASE WHEN l.claimed_at IS NOT NULL AND l.published_at IS NOT NULL THEN
        EXTRACT(EPOCH FROM (l.claimed_at - l.published_at))::BIGINT
    END as time_to_claim_secs,

    -- Remaining validity
    CASE WHEN l.valid_to > NOW() THEN
        EXTRACT(EPOCH FROM (l.valid_to - NOW()))::BIGINT
    ELSE 0 END as remaining_validity_secs

FROM licenses l;

-- ============================================
-- FUNCTIONS
-- ============================================

-- Log a lifecycle event
CREATE OR REPLACE FUNCTION log_lifecycle_event(
    p_license_id VARCHAR(66),
    p_event lifecycle_event,
    p_from_state VARCHAR(30),
    p_to_state VARCHAR(30),
    p_actor_type VARCHAR(20),
    p_actor_id VARCHAR(255),
    p_reason TEXT DEFAULT NULL
) RETURNS BIGINT AS $$
DECLARE
    v_exposure JSONB;
    v_id BIGINT;
BEGIN
    -- Snapshot current exposure
    SELECT jsonb_build_object(
        'published_secs', exposure_published_secs,
        'reserved_secs', exposure_reserved_secs,
        'claimed_secs', exposure_claimed_secs
    ) INTO v_exposure
    FROM licenses WHERE id = p_license_id;

    INSERT INTO license_lifecycle_log (
        license_id, event, from_state, to_state,
        actor_type, actor_id, reason, exposure_snapshot
    ) VALUES (
        p_license_id, p_event, p_from_state, p_to_state,
        p_actor_type, p_actor_id, p_reason, v_exposure
    ) RETURNING id INTO v_id;

    RETURN v_id;
END;
$$ LANGUAGE plpgsql;

-- Cancel a license
CREATE OR REPLACE FUNCTION cancel_license(
    p_license_id VARCHAR(66),
    p_canceller_id VARCHAR(255),
    p_reason TEXT
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_state VARCHAR(30);
BEGIN
    SELECT issuance_state::text INTO v_current_state
    FROM licenses WHERE id = p_license_id FOR UPDATE;

    IF v_current_state IS NULL THEN
        RAISE EXCEPTION 'License not found: %', p_license_id;
    END IF;

    IF v_current_state IN ('cancelled', 'terminated') THEN
        RETURN FALSE;  -- Already cancelled
    END IF;

    -- Update exposure before state change
    PERFORM update_license_exposure(p_license_id);

    -- Cancel the license
    UPDATE licenses SET
        issuance_state = 'cancelled',
        cancelled_at = NOW(),
        cancelled_by = p_canceller_id,
        cancellation_reason = p_reason,
        is_active = FALSE
    WHERE id = p_license_id;

    -- Log the event
    PERFORM log_lifecycle_event(
        p_license_id, 'cancelled', v_current_state, 'cancelled',
        'admin', p_canceller_id, p_reason
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Release a license (user returns it)
CREATE OR REPLACE FUNCTION release_license(
    p_license_id VARCHAR(66),
    p_releaser_id VARCHAR(255),
    p_reason TEXT DEFAULT 'User released'
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_state VARCHAR(30);
    v_owner_id VARCHAR(255);
BEGIN
    SELECT issuance_state::text, issued_to INTO v_current_state, v_owner_id
    FROM licenses WHERE id = p_license_id FOR UPDATE;

    IF v_current_state IS NULL THEN
        RAISE EXCEPTION 'License not found: %', p_license_id;
    END IF;

    IF v_current_state NOT IN ('issued', 'claimed') THEN
        RAISE EXCEPTION 'License % is not issued/claimed (current: %)', p_license_id, v_current_state;
    END IF;

    -- Verify ownership
    IF v_owner_id != p_releaser_id THEN
        RAISE EXCEPTION 'User % is not the owner of license %', p_releaser_id, p_license_id;
    END IF;

    -- Update exposure
    PERFORM update_license_exposure(p_license_id);

    -- Release the license
    UPDATE licenses SET
        issuance_state = 'published',  -- Back to available
        released_at = NOW(),
        released_by = p_releaser_id,
        release_reason = p_reason,
        issued_to = NULL,
        claimed = FALSE,
        claimed_at = NULL
    WHERE id = p_license_id;

    -- Close ownership
    UPDATE license_ownership
    SET owned_until = NOW()
    WHERE license_id = p_license_id AND owned_until IS NULL;

    -- Log the event
    PERFORM log_lifecycle_event(
        p_license_id, 'released', v_current_state, 'published',
        'user', p_releaser_id, p_reason
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Update exposure counters
CREATE OR REPLACE FUNCTION update_license_exposure(p_license_id VARCHAR(66))
RETURNS VOID AS $$
DECLARE
    v_license RECORD;
    v_now TIMESTAMPTZ := NOW();
    v_delta_secs BIGINT;
BEGIN
    SELECT * INTO v_license FROM licenses WHERE id = p_license_id;

    IF v_license IS NULL THEN
        RETURN;
    END IF;

    v_delta_secs := EXTRACT(EPOCH FROM (v_now - COALESCE(v_license.last_exposure_calc_at, v_license.created_at)))::BIGINT;

    -- Update based on current state
    IF v_license.issuance_state = 'published' THEN
        UPDATE licenses SET
            exposure_published_secs = exposure_published_secs + v_delta_secs,
            last_exposure_calc_at = v_now
        WHERE id = p_license_id;
    ELSIF v_license.issuance_state = 'reserved' THEN
        UPDATE licenses SET
            exposure_reserved_secs = exposure_reserved_secs + v_delta_secs,
            last_exposure_calc_at = v_now
        WHERE id = p_license_id;
    ELSIF v_license.issuance_state = 'claimed' THEN
        UPDATE licenses SET
            exposure_claimed_secs = exposure_claimed_secs + v_delta_secs,
            last_exposure_calc_at = v_now
        WHERE id = p_license_id;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Process expired licenses
CREATE OR REPLACE FUNCTION process_expired_licenses()
RETURNS INT AS $$
DECLARE
    v_count INT := 0;
    v_license RECORD;
BEGIN
    FOR v_license IN
        SELECT id, issuance_state::text as state
        FROM licenses
        WHERE valid_to < NOW()
          AND issuance_state NOT IN ('expired', 'cancelled', 'withdrawn')
        FOR UPDATE SKIP LOCKED
    LOOP
        -- Update exposure first
        PERFORM update_license_exposure(v_license.id);

        -- Mark as expired
        UPDATE licenses SET
            issuance_state = 'expired',
            expired_at = NOW()
        WHERE id = v_license.id;

        -- Log the event
        PERFORM log_lifecycle_event(
            v_license.id, 'expired', v_license.state, 'expired',
            'system', 'scheduler', 'Validity period ended'
        );

        v_count := v_count + 1;
    END LOOP;

    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Reactivate an expired/cancelled license
CREATE OR REPLACE FUNCTION reactivate_license(
    p_license_id VARCHAR(66),
    p_reactivator_id VARCHAR(255),
    p_new_valid_to TIMESTAMPTZ,
    p_reason TEXT DEFAULT 'License reactivated'
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_state VARCHAR(30);
    v_old_valid_to TIMESTAMPTZ;
BEGIN
    SELECT issuance_state::text, valid_to INTO v_current_state, v_old_valid_to
    FROM licenses WHERE id = p_license_id FOR UPDATE;

    IF v_current_state IS NULL THEN
        RAISE EXCEPTION 'License not found: %', p_license_id;
    END IF;

    IF v_current_state NOT IN ('expired', 'cancelled') THEN
        RAISE EXCEPTION 'License % is not expired/cancelled (current: %)', p_license_id, v_current_state;
    END IF;

    -- Reactivate
    UPDATE licenses SET
        issuance_state = 'published',
        valid_to = p_new_valid_to,
        original_valid_to = v_old_valid_to,
        reactivated_at = NOW(),
        reactivated_by = p_reactivator_id,
        expired_at = NULL,
        cancelled_at = NULL,
        cancellation_reason = NULL
    WHERE id = p_license_id;

    -- Log the event
    PERFORM log_lifecycle_event(
        p_license_id, 'reactivated', v_current_state, 'published',
        'admin', p_reactivator_id, p_reason
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TYPE lifecycle_event IS 'All possible lifecycle state transitions';

COMMENT ON COLUMN licenses.exposure_published_secs IS 'Cumulative seconds in published state';
COMMENT ON COLUMN licenses.exposure_claimed_secs IS 'Cumulative seconds in claimed state';
COMMENT ON COLUMN licenses.last_exposure_calc_at IS 'When exposure was last calculated';
COMMENT ON COLUMN licenses.original_valid_to IS 'Original validity before reactivation/extension';

COMMENT ON TABLE license_lifecycle_log IS 'Complete audit trail of all lifecycle events';
COMMENT ON TABLE license_expiry_notifications IS 'Record of all expiry notifications sent';

COMMENT ON VIEW license_exposure_summary IS 'Real-time exposure metrics including current state';

COMMENT ON FUNCTION log_lifecycle_event IS 'Log a lifecycle event with exposure snapshot';
COMMENT ON FUNCTION cancel_license IS 'Cancel a license with reason';
COMMENT ON FUNCTION release_license IS 'User releases/returns a claimed license';
COMMENT ON FUNCTION update_license_exposure IS 'Update cumulative exposure counters';
COMMENT ON FUNCTION process_expired_licenses IS 'Mark expired licenses (run periodically)';
COMMENT ON FUNCTION reactivate_license IS 'Reactivate an expired or cancelled license';
