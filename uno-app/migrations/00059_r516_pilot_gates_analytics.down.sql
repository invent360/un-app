-- R5-16: Rollback pilot gates and analytics fixes

-- Drop alert thresholds table
DROP TABLE IF EXISTS pilot_alert_thresholds;

-- Drop unified metrics view
DROP VIEW IF EXISTS v_pilot_cohort_metrics;

-- Drop sync trigger and function
DROP TRIGGER IF EXISTS trg_sync_pilot_from_cohort ON participant_cohorts;
DROP FUNCTION IF EXISTS sync_pilot_from_cohort();

-- Drop atomic enrollment function
DROP FUNCTION IF EXISTS atomic_pilot_enroll(VARCHAR, CHAR, VARCHAR);

-- Drop D7 reward check function
DROP FUNCTION IF EXISTS check_d7_from_rewards(UUID);

-- Remove added columns (but keep data - these are additive)
-- ALTER TABLE pilot_participants DROP COLUMN IF EXISTS cohort_participant_id;
-- ALTER TABLE participant_cohorts DROP COLUMN IF EXISTS d7_reward_days;
-- ALTER TABLE participant_cohorts DROP COLUMN IF EXISTS d30_reward_days;
-- ALTER TABLE cohort_daily_activity DROP COLUMN IF EXISTS reward_accepted;
-- ALTER TABLE cohort_daily_activity DROP COLUMN IF EXISTS reward_amount_micros;

-- Remove six-market planning quotas
DELETE FROM market_quotas WHERE notes LIKE 'R5-16:%';

-- Remove pilot market gates
DELETE FROM launch_gates WHERE gate_name LIKE 'pilot_market_%';

-- Restore original consume_market_quota function (fail-open behavior)
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
    SELECT * INTO v_quota
    FROM market_quotas
    WHERE country_code = p_country_code
      AND quota_type = p_quota_type
      AND enabled = TRUE
      AND (period_start IS NULL OR period_start <= CURRENT_DATE)
      AND (period_end IS NULL OR period_end >= CURRENT_DATE)
    FOR UPDATE;

    -- ORIGINAL: No quota configured = allow (fail-open)
    IF v_quota IS NULL THEN
        RETURN TRUE;
    END IF;

    IF v_quota.current_value < v_quota.max_value THEN
        UPDATE market_quotas
        SET current_value = current_value + 1,
            exhausted_at = CASE WHEN current_value + 1 >= max_value THEN NOW() ELSE NULL END
        WHERE id = v_quota.id;

        INSERT INTO quota_consumption_log (quota_id, user_id, license_id, action)
        VALUES (v_quota.id, p_user_id, p_license_id, p_action);

        RETURN TRUE;
    END IF;

    RETURN FALSE;
END;
$$ LANGUAGE plpgsql;
