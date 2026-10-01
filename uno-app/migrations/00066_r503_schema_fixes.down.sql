-- Rollback: R5-03 Schema Fixes
-- Restores original (buggy) versions

-- Restore original defaults
ALTER TABLE licenses
    ALTER COLUMN uno_share SET DEFAULT 47.0;

ALTER TABLE licenses
    ALTER COLUMN agent_share SET DEFAULT 3.0;

-- Restore original (buggy) function
DROP FUNCTION IF EXISTS calculate_exit_balance(VARCHAR, VARCHAR);

CREATE OR REPLACE FUNCTION calculate_exit_balance(p_user_id VARCHAR(255), p_license_id VARCHAR(66))
RETURNS TABLE (
    final_balance_micros BIGINT,
    pending_earnings_micros BIGINT,
    deductions_micros BIGINT,
    net_payout_micros BIGINT,
    has_pending_tasks BOOLEAN,
    cooldown_ends_at TIMESTAMPTZ
) AS $$
DECLARE
    v_balance BIGINT := 0;
    v_pending BIGINT := 0;
    v_deductions BIGINT := 0;
    v_has_pending BOOLEAN := FALSE;
    v_cooldown TIMESTAMPTZ;
BEGIN
    -- Original (buggy) - uses license_code instead of license_id
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_code = p_license_id;

    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_pending
    FROM allocation_ledger
    WHERE license_code = p_license_id
      AND status = 'pending';

    v_deductions := 0;

    SELECT EXISTS(
        SELECT 1 FROM job_queue
        WHERE job_data->>'user_id' = p_user_id
          AND status IN ('pending', 'claimed')
    ) INTO v_has_pending;

    SELECT issued_at + INTERVAL '30 days' INTO v_cooldown
    FROM licenses
    WHERE id = p_license_id;

    RETURN QUERY SELECT
        v_balance,
        v_pending,
        v_deductions,
        GREATEST(v_balance - v_pending - v_deductions, 0),
        v_has_pending,
        v_cooldown;
END;
$$ LANGUAGE plpgsql;
