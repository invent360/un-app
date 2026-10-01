-- F6: PostgreSQL Consolidation Fixes - Rollback
-- Restores previous versions of functions (without F6 fixes)

-- Restore calculate_exit_balance WITHOUT ownership check (previous version)
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

-- Restore atomic_pilot_enroll WITHOUT participant insertion
DROP FUNCTION IF EXISTS atomic_pilot_enroll(VARCHAR, CHAR, VARCHAR, VARCHAR, VARCHAR);

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
