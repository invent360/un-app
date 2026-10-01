-- B5: Rollback allocation state fix
-- Restore the incorrect function (for migration rollback only)

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

    -- Old incorrect state names
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'settled';

    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_pending
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'pending';

    SELECT COALESCE(SUM(ABS(ulo_micros)), 0) INTO v_deductions
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND state = 'reversed';

    SELECT EXISTS(
        SELECT 1 FROM job_queue
        WHERE payload->>'user_id' = p_user_id
          AND status IN ('pending', 'claimed')
    ) INTO v_has_pending;

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
