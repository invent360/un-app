-- F6: PostgreSQL Consolidation Fixes
--
-- Fixes:
-- 1. calculate_exit_balance - add ownership check before allowing balance calculation
-- 2. atomic_pilot_enroll - include participant insertion for true atomicity

-- ============================================
-- 1. FIX calculate_exit_balance - ADD OWNERSHIP CHECK
-- ============================================
-- SECURITY: Users should only be able to query their own license balance
-- Without this check, any user could query any license's balance

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
    -- F6: SECURITY - Verify user owns this license before calculating balance
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

COMMENT ON FUNCTION calculate_exit_balance IS 'F6: Calculate exit balance with ownership verification';

-- ============================================
-- 2. FIX atomic_pilot_enroll - INCLUDE PARTICIPANT INSERTION
-- ============================================
-- ATOMICITY: The previous version consumed quota/capacity but didn't insert
-- the participant. If add_participant failed, capacity was leaked.
-- New version inserts participant within the same transaction.

DROP FUNCTION IF EXISTS atomic_pilot_enroll(VARCHAR, CHAR, VARCHAR);

CREATE OR REPLACE FUNCTION atomic_pilot_enroll(
    p_user_id VARCHAR(255),
    p_market_code CHAR(2),
    p_cohort_id VARCHAR(50) DEFAULT NULL,
    p_invitation_code VARCHAR(100) DEFAULT NULL,
    p_invitation_channel VARCHAR(50) DEFAULT NULL
)
RETURNS TABLE (
    enrolled BOOLEAN,
    cohort_id VARCHAR(50),
    rejection_reason TEXT,
    participant_id UUID  -- F6: Return inserted participant ID
) AS $$
DECLARE
    v_master_gate BOOLEAN;
    v_market_gate BOOLEAN;
    v_cohort_row RECORD;
    v_has_capacity BOOLEAN;
    v_quota_consumed BOOLEAN;
    v_existing_participant UUID;
    v_new_participant_id UUID;
BEGIN
    -- F6: Check if user already enrolled in this cohort
    IF p_cohort_id IS NOT NULL THEN
        SELECT id INTO v_existing_participant
        FROM pilot_participants
        WHERE external_user_id = p_user_id
          AND cohort_id = p_cohort_id;

        IF v_existing_participant IS NOT NULL THEN
            RETURN QUERY SELECT FALSE, p_cohort_id::VARCHAR(50),
                'User already enrolled in this cohort'::TEXT, v_existing_participant;
            RETURN;
        END IF;
    END IF;

    -- 1. Check master pilot_enrollment gate (fail-closed)
    SELECT is_enabled INTO v_master_gate
    FROM launch_gates
    WHERE gate_name = 'pilot_enrollment'
      AND (expires_at IS NULL OR expires_at > NOW());

    IF v_master_gate IS NULL OR v_master_gate = FALSE THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            'Master pilot_enrollment gate is disabled or missing'::TEXT, NULL::UUID;
        RETURN;
    END IF;

    -- 2. Check market-specific gate (fail-closed)
    SELECT is_enabled INTO v_market_gate
    FROM launch_gates
    WHERE gate_name = 'pilot_market_' || LOWER(p_market_code)
      AND (expires_at IS NULL OR expires_at > NOW());

    IF v_market_gate IS NULL OR v_market_gate = FALSE THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            ('Market gate pilot_market_' || LOWER(p_market_code) || ' is disabled or missing')::TEXT, NULL::UUID;
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
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            'No active cohort with capacity for market'::TEXT, NULL::UUID;
        RETURN;
    END IF;

    -- F6: Check for existing enrollment in the found cohort
    SELECT id INTO v_existing_participant
    FROM pilot_participants
    WHERE external_user_id = p_user_id
      AND cohort_id = v_cohort_row.id;

    IF v_existing_participant IS NOT NULL THEN
        RETURN QUERY SELECT FALSE, v_cohort_row.id::VARCHAR(50),
            'User already enrolled in target cohort'::TEXT, v_existing_participant;
        RETURN;
    END IF;

    -- 4. Check and consume market quota atomically
    v_quota_consumed := consume_market_quota(p_market_code, 'pilot_enrollment', p_user_id, NULL, 'pilot_enroll');

    IF NOT v_quota_consumed THEN
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            'Market pilot quota exhausted or not configured'::TEXT, NULL::UUID;
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
        RETURN QUERY SELECT FALSE, NULL::VARCHAR(50),
            'Cohort capacity race condition - retry'::TEXT, NULL::UUID;
        RETURN;
    END IF;

    -- F6: INSERT PARTICIPANT ATOMICALLY
    -- This ensures quota/capacity consumption and participant creation happen together
    v_new_participant_id := gen_random_uuid();

    INSERT INTO pilot_participants (
        id, external_user_id, market_code, cohort_id, state,
        invitation_code, invitation_channel, invited_at, created_at, updated_at
    )
    VALUES (
        v_new_participant_id, p_user_id, p_market_code, v_cohort_row.id, 'invited',
        p_invitation_code, p_invitation_channel, NOW(), NOW(), NOW()
    );

    -- 6. Success - return with participant ID
    RETURN QUERY SELECT TRUE, v_cohort_row.id::VARCHAR(50), NULL::TEXT, v_new_participant_id;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION atomic_pilot_enroll IS 'F6: Atomically enroll user with gate/quota/capacity checks AND participant insertion';
