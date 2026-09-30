-- Rollback: Schema Reconciliation
-- Note: GENERATED columns cannot be easily removed, they would need table rebuild
-- This rollback removes non-generated additions only

-- Remove added columns from cohort_daily_activity
ALTER TABLE cohort_daily_activity DROP COLUMN IF EXISTS day_number;
-- Note: data_collected_bytes and recorded_at are GENERATED columns, cannot easily drop

-- Remove added columns from cohort_analytics
-- Note: Most are GENERATED columns, cannot easily drop
ALTER TABLE cohort_analytics DROP COLUMN IF EXISTS avg_d7_active_days;
ALTER TABLE cohort_analytics DROP COLUMN IF EXISTS avg_d30_active_days;
ALTER TABLE cohort_analytics DROP COLUMN IF EXISTS total_earnings_micros;
ALTER TABLE cohort_analytics DROP COLUMN IF EXISTS country_code;

-- Drop functions
DROP FUNCTION IF EXISTS calculate_exit_balance(VARCHAR);

-- Restore original calculate_cohort_analytics (return UUID version)
CREATE OR REPLACE FUNCTION calculate_cohort_analytics(p_cohort_date DATE, p_snapshot_date DATE DEFAULT CURRENT_DATE)
RETURNS UUID AS $$
DECLARE
    v_id UUID;
    v_total INT;
    v_d1 INT;
    v_d3 INT;
    v_d7 INT;
    v_d30 INT;
    v_churned INT;
BEGIN
    SELECT
        COUNT(*),
        COUNT(*) FILTER (WHERE d1_completed),
        COUNT(*) FILTER (WHERE d3_completed),
        COUNT(*) FILTER (WHERE d7_completed),
        COUNT(*) FILTER (WHERE d30_completed),
        COUNT(*) FILTER (WHERE status = 'churned')
    INTO v_total, v_d1, v_d3, v_d7, v_d30, v_churned
    FROM participant_cohorts
    WHERE cohort_date = p_cohort_date;

    INSERT INTO cohort_analytics (
        snapshot_date, cohort_date,
        total_activated, d1_completed, d3_completed, d7_completed, d30_completed, churned,
        d1_rate, d3_rate, d7_rate, d30_rate, churn_rate
    ) VALUES (
        p_snapshot_date, p_cohort_date,
        v_total, v_d1, v_d3, v_d7, v_d30, v_churned,
        CASE WHEN v_total > 0 THEN (v_d1::DECIMAL / v_total * 100) ELSE 0 END,
        CASE WHEN v_total > 0 THEN (v_d3::DECIMAL / v_total * 100) ELSE 0 END,
        CASE WHEN v_total > 0 THEN (v_d7::DECIMAL / v_total * 100) ELSE 0 END,
        CASE WHEN v_total > 0 THEN (v_d30::DECIMAL / v_total * 100) ELSE 0 END,
        CASE WHEN v_total > 0 THEN (v_churned::DECIMAL / v_total * 100) ELSE 0 END
    )
    ON CONFLICT (snapshot_date, cohort_date) DO UPDATE SET
        total_activated = EXCLUDED.total_activated,
        d1_completed = EXCLUDED.d1_completed,
        d3_completed = EXCLUDED.d3_completed,
        d7_completed = EXCLUDED.d7_completed,
        d30_completed = EXCLUDED.d30_completed,
        churned = EXCLUDED.churned,
        d1_rate = EXCLUDED.d1_rate,
        d3_rate = EXCLUDED.d3_rate,
        d7_rate = EXCLUDED.d7_rate,
        d30_rate = EXCLUDED.d30_rate,
        churn_rate = EXCLUDED.churn_rate
    RETURNING id INTO v_id;

    RETURN v_id;
END;
$$ LANGUAGE plpgsql;
