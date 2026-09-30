-- Migration: Schema Reconciliation (Gate A - R3-02, R4-03)
-- Reconcile repository queries with existing migration schemas

-- ============================================
-- Fix cohort_daily_activity table
-- ============================================

-- Add day_number column (required by repository)
ALTER TABLE cohort_daily_activity
    ADD COLUMN IF NOT EXISTS day_number INT NOT NULL DEFAULT 1;

-- Add data_collected_bytes as alias/computed for data_shared_bytes
-- (Both names are semantically valid, support both)
ALTER TABLE cohort_daily_activity
    ADD COLUMN IF NOT EXISTS data_collected_bytes BIGINT GENERATED ALWAYS AS (data_shared_bytes) STORED;

-- Add recorded_at as alias for created_at
ALTER TABLE cohort_daily_activity
    ADD COLUMN IF NOT EXISTS recorded_at TIMESTAMPTZ GENERATED ALWAYS AS (created_at) STORED;

-- ============================================
-- Fix participant_cohorts table
-- ============================================

-- Repository doesn't use target dates, but they exist in schema - no change needed

-- ============================================
-- Fix cohort_analytics table
-- Repository expects different column names
-- ============================================

-- Add columns expected by repository CohortAnalytics struct
ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS total_participants INT GENERATED ALWAYS AS (total_activated) STORED;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS d1_completed_count INT GENERATED ALWAYS AS (d1_completed) STORED;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS d3_completed_count INT GENERATED ALWAYS AS (d3_completed) STORED;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS d7_completed_count INT GENERATED ALWAYS AS (d7_completed) STORED;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS d30_completed_count INT GENERATED ALWAYS AS (d30_completed) STORED;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS avg_d7_active_days DECIMAL(5,2) DEFAULT 0;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS avg_d30_active_days DECIMAL(5,2) DEFAULT 0;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS total_earnings_micros BIGINT DEFAULT 0;

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS country_code VARCHAR(10);

ALTER TABLE cohort_analytics
    ADD COLUMN IF NOT EXISTS calculated_at TIMESTAMPTZ GENERATED ALWAYS AS (created_at) STORED;

-- ============================================
-- Fix exit_repository calculate_exit_balance function
-- Bug: migration 00044 used ulo_amount_micros but column is ulo_micros
-- ============================================

-- Drop and recreate with correct column name and proper signature
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
    -- Get settled balance from allocation_ledger
    -- FIX: Use ulo_micros (correct column name), not ulo_amount_micros
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_code = p_license_id;

    -- Get pending (unsettled) earnings
    SELECT COALESCE(SUM(ulo_micros), 0) INTO v_pending
    FROM allocation_ledger
    WHERE license_code = p_license_id
      AND status = 'pending';

    -- Check for deductions (chargebacks, etc.)
    -- For now, return 0
    v_deductions := 0;

    -- Check for pending tasks (jobs in queue for this user)
    SELECT EXISTS(
        SELECT 1 FROM job_queue
        WHERE job_data->>'user_id' = p_user_id
          AND status IN ('pending', 'claimed')
    ) INTO v_has_pending;

    -- Calculate cooldown (30 days from claim)
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

-- ============================================
-- Fix calculate_cohort_analytics to return proper columns
-- ============================================

CREATE OR REPLACE FUNCTION calculate_cohort_analytics(p_cohort_date DATE, p_country_code VARCHAR DEFAULT NULL)
RETURNS cohort_analytics AS $$
DECLARE
    v_result cohort_analytics%ROWTYPE;
    v_total INT;
    v_d1 INT;
    v_d3 INT;
    v_d7 INT;
    v_d30 INT;
    v_avg_d7 DECIMAL(5,2);
    v_avg_d30 DECIMAL(5,2);
    v_earnings BIGINT;
BEGIN
    -- Get counts
    SELECT
        COUNT(*),
        COUNT(*) FILTER (WHERE d1_completed),
        COUNT(*) FILTER (WHERE d3_completed),
        COUNT(*) FILTER (WHERE d7_completed),
        COUNT(*) FILTER (WHERE d30_completed),
        COALESCE(AVG(d7_active_days), 0),
        COALESCE(AVG(d30_active_days), 0)
    INTO v_total, v_d1, v_d3, v_d7, v_d30, v_avg_d7, v_avg_d30
    FROM participant_cohorts
    WHERE cohort_date = p_cohort_date;

    -- Get total earnings
    SELECT COALESCE(SUM(earnings_micros), 0)
    INTO v_earnings
    FROM cohort_daily_activity cda
    JOIN participant_cohorts pc ON cda.cohort_id = pc.id
    WHERE pc.cohort_date = p_cohort_date;

    -- Construct result
    v_result.id := gen_random_uuid();
    v_result.snapshot_date := CURRENT_DATE;
    v_result.cohort_date := p_cohort_date;
    v_result.total_activated := v_total;
    v_result.d1_completed := v_d1;
    v_result.d3_completed := v_d3;
    v_result.d7_completed := v_d7;
    v_result.d30_completed := v_d30;
    v_result.churned := 0;
    v_result.d1_rate := CASE WHEN v_total > 0 THEN (v_d1::DECIMAL / v_total * 100) ELSE 0 END;
    v_result.d3_rate := CASE WHEN v_total > 0 THEN (v_d3::DECIMAL / v_total * 100) ELSE 0 END;
    v_result.d7_rate := CASE WHEN v_total > 0 THEN (v_d7::DECIMAL / v_total * 100) ELSE 0 END;
    v_result.d30_rate := CASE WHEN v_total > 0 THEN (v_d30::DECIMAL / v_total * 100) ELSE 0 END;
    v_result.avg_d7_active_days := v_avg_d7;
    v_result.avg_d30_active_days := v_avg_d30;
    v_result.total_earnings_micros := v_earnings;
    v_result.country_code := p_country_code;
    v_result.country_stats := '{}';
    v_result.created_at := NOW();

    RETURN v_result;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- Fix cohort_notifications to have skip_reason column
-- (Repository uses skip_reason but migration may have failure_reason)
-- ============================================

-- Ensure skip_reason column exists (alias for failure_reason in skip case)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'cohort_notifications' AND column_name = 'skip_reason'
    ) THEN
        -- The column is named skip_reason in our migration, but add alias if it doesn't exist
        ALTER TABLE cohort_notifications
            ADD COLUMN skip_reason TEXT;
    END IF;
END $$;

-- ============================================
-- Comments
-- ============================================

COMMENT ON COLUMN cohort_daily_activity.day_number IS 'Day number since cohort date (1 = first day)';
COMMENT ON COLUMN cohort_daily_activity.data_collected_bytes IS 'Alias for data_shared_bytes for repository compatibility';
COMMENT ON COLUMN cohort_daily_activity.recorded_at IS 'Alias for created_at for repository compatibility';
COMMENT ON FUNCTION calculate_exit_balance IS 'Calculate exit balance for license using allocation_ledger.ulo_micros';
