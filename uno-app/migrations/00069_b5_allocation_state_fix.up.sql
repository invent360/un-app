-- B5: Fix allocation state names to match schema
--
-- The calculate_exit_balance function was using incorrect state names:
-- - 'settled' should be 'paid'
-- - 'pending' should be 'accrued'
-- - 'reversed' doesn't exist in schema
--
-- Schema defines: allocation_state ENUM ('accrued', 'payable', 'paid')

-- ============================================
-- FIX calculate_exit_balance - USE CORRECT STATE NAMES
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
    v_license_owner VARCHAR(255);
BEGIN
    -- SECURITY - Verify user owns this license before calculating balance
    SELECT issued_to INTO v_license_owner
    FROM licenses
    WHERE id = p_license_id::uuid;

    IF v_license_owner IS NULL THEN
        RAISE EXCEPTION 'License % not found', p_license_id;
    END IF;

    IF v_license_owner != p_user_id THEN
        RAISE EXCEPTION 'SECURITY: User % does not own license % (owned by %)',
            p_user_id, p_license_id, v_license_owner;
    END IF;

    -- B5 FIX: Get paid (settled) balance from allocation_ledger
    -- Using correct state name 'paid' instead of 'settled'
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'paid';

    -- B5 FIX: Get pending (unsettled) earnings - accrued + payable
    -- 'accrued' = recorded but not yet approved
    -- 'payable' = approved for payment but not yet paid
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_pending
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state IN ('accrued', 'payable');

    -- B5 FIX: Deductions from negative allocations (reversals use negative ulo_micros)
    -- There's no 'reversed' state - reversals are separate negative entries
    SELECT COALESCE(SUM(ABS(ulo_micros)), 0) INTO v_deductions
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND ulo_micros < 0;

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
        GREATEST(v_balance + v_pending - v_deductions, 0),
        v_has_pending,
        v_cooldown;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION calculate_exit_balance IS 'B5 FIX: Calculate exit balance using correct allocation_state enum values (accrued, payable, paid)';
