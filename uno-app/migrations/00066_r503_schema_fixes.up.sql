-- Migration: R5-03 Schema Fixes
-- Fixes remaining schema reconciliation issues

-- ============================================
-- Fix calculate_exit_balance function
-- Bug: Used license_code but column is license_id
-- ============================================

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
    -- R5-03 FIX: Use license_id (correct column name), not license_code
    -- Get settled balance from allocation_ledger
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'settled';

    -- Get pending (unsettled) earnings
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_pending
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'pending';

    -- Check for deductions (chargebacks, reversals)
    SELECT COALESCE(SUM(ABS(ulo_micros)), 0) INTO v_deductions
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'reversed';

    -- Check for pending tasks (jobs in queue for this user)
    SELECT EXISTS(
        SELECT 1 FROM job_queue
        WHERE payload->>'user_id' = p_user_id
          AND status IN ('pending', 'claimed')
    ) INTO v_has_pending;

    -- Calculate cooldown (30 days from claim)
    SELECT issued_at + INTERVAL '30 days' INTO v_cooldown
    FROM licenses
    WHERE id = p_license_id::uuid;

    RETURN QUERY SELECT
        v_balance,
        v_pending,
        v_deductions,
        GREATEST(v_balance - v_pending - v_deductions, 0),
        v_has_pending,
        v_cooldown;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION calculate_exit_balance IS 'R5-03 FIX: Calculate exit balance using allocation_ledger.license_id and proper state filtering';

-- ============================================
-- Update default share splits for NEW licenses
-- OLD: uno_share=47, agent_share=3, ulo_share=50
-- NEW: uno_share=40, agent_share=10 (referral/reserve), ulo_share=50
-- ============================================
-- Note: This only changes defaults for NEW licenses, existing values are unchanged

ALTER TABLE licenses
    ALTER COLUMN uno_share SET DEFAULT 40.0;

ALTER TABLE licenses
    ALTER COLUMN agent_share SET DEFAULT 10.0;

-- ulo_share stays at 50.0

COMMENT ON COLUMN licenses.uno_share IS 'R5-03: UNO share percentage (default 40% for new offers)';
COMMENT ON COLUMN licenses.agent_share IS 'R5-03: Referral/reserve share percentage (default 10% for new offers)';
COMMENT ON COLUMN licenses.ulo_share IS 'ULO share percentage (default 50%)';
