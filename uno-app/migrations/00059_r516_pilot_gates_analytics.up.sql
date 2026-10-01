-- R5-16: Pilot gates and analytics fixes
--
-- Addresses:
-- 1. Market quota fail-closed behavior (was fail-open)
-- 2. Six-market planning quotas (India 60, Philippines 60, Nigeria 50, Kenya 35, Bangladesh 30, Ghana 15)
-- 3. Atomic enrollment with capacity consumption
-- 4. D7/D30 from accepted-reward days (not sessions)

-- ============================================
-- 1. FIX MARKET QUOTA FUNCTION (FAIL-CLOSED)
-- ============================================
-- The consume_market_quota function must fail-closed when no quota is configured
-- rather than allowing unlimited access

CREATE OR REPLACE FUNCTION consume_market_quota(
    p_country_code CHAR(2),
    p_quota_type VARCHAR(50),
    p_user_id VARCHAR(255),
    p_license_id VARCHAR(66) DEFAULT NULL,
    p_action VARCHAR(50) DEFAULT 'claim'
)
RETURNS BOOLEAN AS $$
DECLARE
    v_quota market_quotas%ROWTYPE;
BEGIN
    -- Find applicable quota
    SELECT * INTO v_quota
    FROM market_quotas
    WHERE country_code = p_country_code
      AND quota_type = p_quota_type
      AND enabled = TRUE
      AND (period_start IS NULL OR period_start <= CURRENT_DATE)
      AND (period_end IS NULL OR period_end >= CURRENT_DATE)
    FOR UPDATE;

    -- R5-16: FAIL-CLOSED when no quota configured
    -- Unknown markets must be explicitly allowed, not implicitly open
    IF v_quota IS NULL THEN
        RAISE WARNING 'R5-16: No quota configured for market % quota_type %, denying request (fail-closed)',
            p_country_code, p_quota_type;
        RETURN FALSE;
    END IF;

    IF v_quota.current_value < v_quota.max_value THEN
        -- Consume quota atomically
        UPDATE market_quotas
        SET current_value = current_value + 1,
            exhausted_at = CASE WHEN current_value + 1 >= max_value THEN NOW() ELSE NULL END,
            updated_at = NOW()
        WHERE id = v_quota.id;

        INSERT INTO quota_consumption_log (quota_id, user_id, license_id, action, consumed_at)
        VALUES (v_quota.id, p_user_id, p_license_id, p_action, NOW());

        RETURN TRUE;
    END IF;

    -- Quota exhausted
    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION consume_market_quota IS 'R5-16: Atomically consume market quota with fail-closed behavior';

-- ============================================
-- 2. SIX-MARKET PLANNING QUOTAS
-- ============================================
-- India 60, Philippines 60, Nigeria 50, Kenya 35, Bangladesh 30, Ghana 15
-- These are pilot targets, not verified legal/task/redemption approval

-- First ensure the market_quotas table has the updated_at column
ALTER TABLE market_quotas ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- Clear existing pilot quotas to replace with correct values
DELETE FROM market_quotas WHERE quota_type = 'pilot_enrollment' AND period_start >= '2026-01-01';

-- Insert six-market planning quotas
INSERT INTO market_quotas (country_code, quota_type, max_value, current_value, period_start, period_end, enabled, notes)
VALUES
    ('IN', 'pilot_enrollment', 60, 0, NULL, NULL, TRUE, 'R5-16: India pilot target (60)'),
    ('PH', 'pilot_enrollment', 60, 0, NULL, NULL, TRUE, 'R5-16: Philippines pilot target (60)'),
    ('NG', 'pilot_enrollment', 50, 0, NULL, NULL, TRUE, 'R5-16: Nigeria pilot target (50)'),
    ('KE', 'pilot_enrollment', 35, 0, NULL, NULL, TRUE, 'R5-16: Kenya pilot target (35)'),
    ('BD', 'pilot_enrollment', 30, 0, NULL, NULL, TRUE, 'R5-16: Bangladesh pilot target (30)'),
    ('GH', 'pilot_enrollment', 15, 0, NULL, NULL, TRUE, 'R5-16: Ghana pilot target (15)')
ON CONFLICT (country_code, quota_type) WHERE period_start IS NULL AND period_end IS NULL
DO UPDATE SET
    max_value = EXCLUDED.max_value,
    notes = EXCLUDED.notes,
    updated_at = NOW();

-- ============================================
-- 3. ATOMIC ENROLLMENT FUNCTION
-- ============================================
-- Binds enrollment to market gate, cohort, and quota atomically

CREATE OR REPLACE FUNCTION atomic_pilot_enroll(
    p_user_id VARCHAR(255),
    p_market_code CHAR(2),
    p_cohort_id VARCHAR(50) DEFAULT NULL
)
RETURNS TABLE (
    enrolled BOOLEAN,
    cohort_id VARCHAR(50),
    rejection_reason TEXT
) AS $$
DECLARE
    v_master_gate BOOLEAN;
    v_market_gate BOOLEAN;
    v_cohort_row RECORD;
    v_has_capacity BOOLEAN;
    v_quota_consumed BOOLEAN;
BEGIN
    -- 1. Check master pilot_enrollment gate (fail-closed)
    SELECT is_enabled INTO v_master_gate
    FROM launch_gates
    WHERE gate_name = 'pilot_enrollment'
      AND (expires_at IS NULL OR expires_at > NOW());

    IF v_master_gate IS NULL OR v_master_gate = FALSE THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50), 'Master pilot_enrollment gate is disabled or missing'::TEXT;
        RETURN;
    END IF;

    -- 2. Check market-specific gate (fail-closed)
    SELECT is_enabled INTO v_market_gate
    FROM launch_gates
    WHERE gate_name = 'pilot_market_' || LOWER(p_market_code)
      AND (expires_at IS NULL OR expires_at > NOW());

    IF v_market_gate IS NULL OR v_market_gate = FALSE THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            ('Market gate pilot_market_' || LOWER(p_market_code) || ' is disabled or missing')::TEXT;
        RETURN;
    END IF;

    -- 3. Find active cohort with capacity
    SELECT pc.id, pc.target_size, pc.current_size
    INTO v_cohort_row
    FROM pilot_cohorts pc
    WHERE pc.market_code = p_market_code
      AND pc.is_active = TRUE
      AND pc.current_size < pc.target_size
      AND (p_cohort_id IS NULL OR pc.id = p_cohort_id)
    ORDER BY pc.start_date ASC
    LIMIT 1
    FOR UPDATE;

    IF v_cohort_row IS NULL THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50), 'No active cohort with capacity for market'::TEXT;
        RETURN;
    END IF;

    -- 4. Check and consume market quota atomically
    v_quota_consumed := consume_market_quota(p_market_code, 'pilot_enrollment', p_user_id, NULL, 'pilot_enroll');

    IF NOT v_quota_consumed THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50), 'Market pilot quota exhausted or not configured'::TEXT;
        RETURN;
    END IF;

    -- 5. Increment cohort size atomically
    UPDATE pilot_cohorts
    SET current_size = current_size + 1,
        updated_at = NOW()
    WHERE id = v_cohort_row.id
      AND current_size < target_size;

    IF NOT FOUND THEN
        -- Race condition: another enrollment took the last slot
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50), 'Cohort capacity race condition - retry'::TEXT;
        RETURN;
    END IF;

    -- 6. Success
    RETURN QUERY SELECT TRUE, v_cohort_row.id::VARCHAR(50), NULL::TEXT;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION atomic_pilot_enroll IS 'R5-16: Atomically enroll user in pilot with gate/quota/capacity checks';

-- ============================================
-- 4. D7/D30 FROM ACCEPTED-REWARD DAYS
-- ============================================
-- D7 = 4 distinct days with accepted rewards (not sessions/browser time)
-- Update the check function to verify reward acceptance

CREATE OR REPLACE FUNCTION check_d7_from_rewards(p_cohort_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    v_cohort participant_cohorts%ROWTYPE;
    v_reward_days INT;
BEGIN
    SELECT * INTO v_cohort FROM participant_cohorts WHERE id = p_cohort_id;
    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    -- R5-16: Count distinct days with ACCEPTED rewards (not sessions)
    -- A "reward day" is a day where the user received at least one accepted reward
    SELECT COUNT(DISTINCT DATE(activity_date)) INTO v_reward_days
    FROM cohort_daily_activity
    WHERE cohort_id = p_cohort_id
      AND is_productive = TRUE
      AND reward_accepted = TRUE  -- Must have actual accepted reward
      AND activity_date >= v_cohort.cohort_date
      AND activity_date <= v_cohort.d7_target_date;

    -- Update completion status (4+ reward days = D7 complete)
    UPDATE participant_cohorts
    SET d7_completed = (v_reward_days >= 4),
        d7_completed_at = CASE WHEN v_reward_days >= 4 THEN NOW() ELSE NULL END,
        d7_reward_days = v_reward_days,
        updated_at = NOW()
    WHERE id = p_cohort_id;

    RETURN (v_reward_days >= 4);
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION check_d7_from_rewards IS 'R5-16: D7 from 4 distinct accepted-reward days, not sessions';

-- Add reward tracking column to cohort_daily_activity if not exists
ALTER TABLE cohort_daily_activity ADD COLUMN IF NOT EXISTS reward_accepted BOOLEAN DEFAULT FALSE;
ALTER TABLE cohort_daily_activity ADD COLUMN IF NOT EXISTS reward_amount_micros BIGINT DEFAULT 0;

COMMENT ON COLUMN cohort_daily_activity.reward_accepted IS 'R5-16: True if user received accepted reward on this day';
COMMENT ON COLUMN cohort_daily_activity.reward_amount_micros IS 'R5-16: Total accepted reward amount for this day in micros';

-- Add reward_days tracking to participant_cohorts
ALTER TABLE participant_cohorts ADD COLUMN IF NOT EXISTS d7_reward_days INT DEFAULT 0;
ALTER TABLE participant_cohorts ADD COLUMN IF NOT EXISTS d30_reward_days INT DEFAULT 0;

COMMENT ON COLUMN participant_cohorts.d7_reward_days IS 'R5-16: Distinct days with accepted rewards in D1-D7 window';
COMMENT ON COLUMN participant_cohorts.d30_reward_days IS 'R5-16: Distinct days with accepted rewards in D1-D30 window';

-- ============================================
-- 5. PILOT METRICS UNIFICATION
-- ============================================
-- Link pilot_participants to cohort analytics for D7/D30 consistency

ALTER TABLE pilot_participants ADD COLUMN IF NOT EXISTS cohort_participant_id UUID REFERENCES participant_cohorts(id);

COMMENT ON COLUMN pilot_participants.cohort_participant_id IS 'R5-16: Link to participant_cohorts for unified D7/D30 tracking';

-- Function to sync pilot participant state from cohort completion
CREATE OR REPLACE FUNCTION sync_pilot_from_cohort()
RETURNS TRIGGER AS $$
BEGIN
    -- When cohort D7 is completed, update pilot participant state
    IF NEW.d7_completed = TRUE AND OLD.d7_completed = FALSE THEN
        UPDATE pilot_participants
        SET state = 'd7_active',
            d7_check_at = NOW(),
            d7_outcome = 'active',
            updated_at = NOW()
        WHERE cohort_participant_id = NEW.id
          AND state IN ('license_claimed', 'registered');
    END IF;

    -- When cohort D30 is completed, update pilot participant state
    IF NEW.d30_completed = TRUE AND OLD.d30_completed = FALSE THEN
        UPDATE pilot_participants
        SET state = 'd30_active',
            d30_check_at = NOW(),
            d30_outcome = 'active',
            updated_at = NOW()
        WHERE cohort_participant_id = NEW.id
          AND state IN ('d7_active', 'd7_inactive');
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_sync_pilot_from_cohort ON participant_cohorts;
CREATE TRIGGER trg_sync_pilot_from_cohort
    AFTER UPDATE OF d7_completed, d30_completed ON participant_cohorts
    FOR EACH ROW
    EXECUTE FUNCTION sync_pilot_from_cohort();

COMMENT ON TRIGGER trg_sync_pilot_from_cohort ON participant_cohorts IS 'R5-16: Sync pilot participant state from cohort D7/D30 completion';

-- ============================================
-- 6. UPDATED PILOT COHORT METRICS VIEW
-- ============================================

CREATE OR REPLACE VIEW v_pilot_cohort_metrics AS
SELECT
    pc.market_code,
    pc.id AS cohort_id,
    pc.target_size,
    pc.current_size,
    pc.target_size - pc.current_size AS remaining_capacity,
    mq.max_value AS market_quota,
    mq.current_value AS market_used,
    mq.max_value - mq.current_value AS market_remaining,
    -- Gate status
    lg_master.is_enabled AS master_gate_enabled,
    lg_market.is_enabled AS market_gate_enabled,
    -- D7/D30 rates from participant_cohorts
    COALESCE(ca.d7_rate, 0) AS d7_rate,
    COALESCE(ca.d30_rate, 0) AS d30_rate,
    -- Counts
    (SELECT COUNT(*) FROM pilot_participants pp WHERE pp.cohort_id = pc.id) AS total_participants,
    (SELECT COUNT(*) FROM pilot_participants pp WHERE pp.cohort_id = pc.id AND pp.d7_outcome = 'active') AS d7_active_count,
    (SELECT COUNT(*) FROM pilot_participants pp WHERE pp.cohort_id = pc.id AND pp.d30_outcome = 'active') AS d30_active_count
FROM pilot_cohorts pc
LEFT JOIN market_quotas mq ON mq.country_code = pc.market_code AND mq.quota_type = 'pilot_enrollment'
LEFT JOIN launch_gates lg_master ON lg_master.gate_name = 'pilot_enrollment'
LEFT JOIN launch_gates lg_market ON lg_market.gate_name = 'pilot_market_' || LOWER(pc.market_code)
LEFT JOIN cohort_analytics ca ON ca.cohort_date = pc.start_date::DATE
WHERE pc.is_active = TRUE;

COMMENT ON VIEW v_pilot_cohort_metrics IS 'R5-16: Unified view of pilot cohort metrics, gates, and quotas';

-- ============================================
-- 7. PILOT MARKET GATES (INITIAL DISABLED)
-- ============================================
-- Start with Nigeria and Philippines for initial pilot, others disabled

INSERT INTO launch_gates (gate_name, gate_description, is_enabled, enabled_by, evidence_url)
VALUES
    ('pilot_market_ng', 'Nigeria pilot market gate', FALSE, 'system', NULL),
    ('pilot_market_ph', 'Philippines pilot market gate', FALSE, 'system', NULL),
    ('pilot_market_in', 'India pilot market gate', FALSE, 'system', NULL),
    ('pilot_market_ke', 'Kenya pilot market gate', FALSE, 'system', NULL),
    ('pilot_market_bd', 'Bangladesh pilot market gate', FALSE, 'system', NULL),
    ('pilot_market_gh', 'Ghana pilot market gate', FALSE, 'system', NULL)
ON CONFLICT (gate_name) DO NOTHING;

-- ============================================
-- 8. ALERT THRESHOLDS TABLE
-- ============================================
-- Track alert thresholds for pilot metrics

CREATE TABLE IF NOT EXISTS pilot_alert_thresholds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_name VARCHAR(100) NOT NULL,
    market_code CHAR(2),  -- NULL = global
    warning_threshold DECIMAL(10, 4),
    critical_threshold DECIMAL(10, 4),
    comparison VARCHAR(10) NOT NULL DEFAULT 'lt',  -- 'lt', 'gt', 'eq'
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    owner VARCHAR(255),
    runbook_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(metric_name, market_code)
);

COMMENT ON TABLE pilot_alert_thresholds IS 'R5-16: Alert thresholds for pilot metrics with owners and runbooks';

-- Insert default thresholds
INSERT INTO pilot_alert_thresholds (metric_name, market_code, warning_threshold, critical_threshold, comparison, owner, runbook_url)
VALUES
    ('d7_retention_rate', NULL, 60.0, 40.0, 'lt', 'analytics', NULL),
    ('d30_retention_rate', NULL, 50.0, 30.0, 'lt', 'analytics', NULL),
    ('enrollment_rate', NULL, 5.0, 2.0, 'lt', 'growth', NULL),
    ('quota_utilization', NULL, 80.0, 95.0, 'gt', 'operations', NULL),
    ('support_ticket_rate', NULL, 10.0, 20.0, 'gt', 'support', NULL)
ON CONFLICT (metric_name, market_code) DO NOTHING;
