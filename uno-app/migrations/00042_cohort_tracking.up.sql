-- Migration: Cohort Tracking (Phase 7)
-- P7-05: D1 installation, D3 activity, D7 productivity, D30 retention tracking

-- Participant cohorts - tracks D1/D7/D30 milestones
CREATE TABLE IF NOT EXISTS participant_cohorts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Participant info
    user_id VARCHAR(255) NOT NULL,
    license_id VARCHAR(66) NOT NULL,

    -- Cohort date (activation date, used as denominator anchor)
    cohort_date DATE NOT NULL,

    -- D1: Installation milestone
    -- Participant installs and runs the app
    d1_target_date DATE NOT NULL,
    d1_completed BOOLEAN NOT NULL DEFAULT FALSE,
    d1_completed_at TIMESTAMPTZ,

    -- D3: Activity/data-cost milestone
    -- Participant shows activity indicating actual usage
    d3_target_date DATE NOT NULL,
    d3_completed BOOLEAN NOT NULL DEFAULT FALSE,
    d3_completed_at TIMESTAMPTZ,
    d3_activity_count INT NOT NULL DEFAULT 0,
    d3_data_used_bytes BIGINT NOT NULL DEFAULT 0,

    -- D7: Productivity milestone
    -- Activity on at least 4 of 7 days with required status/rules
    d7_target_date DATE NOT NULL,
    d7_completed BOOLEAN NOT NULL DEFAULT FALSE,
    d7_completed_at TIMESTAMPTZ,
    d7_active_days INT NOT NULL DEFAULT 0,
    d7_active_dates DATE[] DEFAULT '{}',
    d7_required_days INT NOT NULL DEFAULT 4, -- Configurable: default 4 of 7

    -- D30: Retention milestone
    -- Uses original activated-cohort denominator
    d30_target_date DATE NOT NULL,
    d30_completed BOOLEAN NOT NULL DEFAULT FALSE,
    d30_completed_at TIMESTAMPTZ,
    d30_active_days INT NOT NULL DEFAULT 0,
    d30_productive_days INT NOT NULL DEFAULT 0, -- Days meeting productivity criteria

    -- Overall status
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'churned', 'completed', 'paused')),
    churned_at TIMESTAMPTZ,
    churn_reason VARCHAR(100),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    UNIQUE(user_id, license_id)
);

-- Daily activity log for cohort tracking
CREATE TABLE IF NOT EXISTS cohort_daily_activity (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cohort_id UUID NOT NULL REFERENCES participant_cohorts(id) ON DELETE CASCADE,

    -- Activity date
    activity_date DATE NOT NULL,

    -- Activity metrics
    sessions_count INT NOT NULL DEFAULT 0,
    active_minutes INT NOT NULL DEFAULT 0,
    data_shared_bytes BIGINT NOT NULL DEFAULT 0,
    tasks_completed INT NOT NULL DEFAULT 0,
    earnings_micros BIGINT NOT NULL DEFAULT 0,

    -- Status for the day
    is_productive BOOLEAN NOT NULL DEFAULT FALSE, -- Met productivity criteria
    productivity_score INT NOT NULL DEFAULT 0, -- 0-100 score

    -- Device info
    device_type VARCHAR(20),
    app_version VARCHAR(20),
    os_version VARCHAR(20),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(cohort_id, activity_date)
);

-- Cohort notifications for D1/D3/D7/D30 reminders
CREATE TABLE IF NOT EXISTS cohort_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cohort_id UUID NOT NULL REFERENCES participant_cohorts(id) ON DELETE CASCADE,

    -- Notification type
    notification_type VARCHAR(20) NOT NULL CHECK (notification_type IN ('d1_reminder', 'd3_reminder', 'd7_reminder', 'd30_check', 'd7_congrats', 'd30_congrats')),

    -- Scheduling
    scheduled_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',

    -- Delivery
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('email', 'push', 'sms', 'in_app')),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed', 'skipped', 'cancelled')),

    -- Delivery details
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    failure_reason TEXT,
    skip_reason TEXT,

    -- Tracking
    external_id VARCHAR(255), -- ID from email/SMS provider
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,

    -- Consent check
    consent_checked BOOLEAN NOT NULL DEFAULT FALSE,
    consent_valid BOOLEAN,

    -- Deduplication
    idempotency_key VARCHAR(100) NOT NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(idempotency_key)
);

-- Cohort analytics snapshots (for reporting)
CREATE TABLE IF NOT EXISTS cohort_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Snapshot date
    snapshot_date DATE NOT NULL,
    cohort_date DATE NOT NULL, -- Which cohort

    -- Counts
    total_activated INT NOT NULL DEFAULT 0,
    d1_completed INT NOT NULL DEFAULT 0,
    d3_completed INT NOT NULL DEFAULT 0,
    d7_completed INT NOT NULL DEFAULT 0,
    d30_completed INT NOT NULL DEFAULT 0,
    churned INT NOT NULL DEFAULT 0,

    -- Rates
    d1_rate DECIMAL(5,2), -- D1 / Total
    d3_rate DECIMAL(5,2), -- D3 / Total
    d7_rate DECIMAL(5,2), -- D7 / Total
    d30_rate DECIMAL(5,2), -- D30 / Total
    churn_rate DECIMAL(5,2), -- Churned / Total

    -- Country breakdown
    country_stats JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(snapshot_date, cohort_date)
);

-- Indexes
CREATE INDEX idx_cohort_user ON participant_cohorts(user_id);
CREATE INDEX idx_cohort_license ON participant_cohorts(license_id);
CREATE INDEX idx_cohort_date ON participant_cohorts(cohort_date);
CREATE INDEX idx_cohort_status ON participant_cohorts(status);
CREATE INDEX idx_cohort_d1_pending ON participant_cohorts(d1_target_date) WHERE d1_completed = FALSE;
CREATE INDEX idx_cohort_d7_pending ON participant_cohorts(d7_target_date) WHERE d7_completed = FALSE;
CREATE INDEX idx_cohort_d30_pending ON participant_cohorts(d30_target_date) WHERE d30_completed = FALSE;

CREATE INDEX idx_cohort_daily_cohort ON cohort_daily_activity(cohort_id);
CREATE INDEX idx_cohort_daily_date ON cohort_daily_activity(activity_date);

CREATE INDEX idx_cohort_notif_cohort ON cohort_notifications(cohort_id);
CREATE INDEX idx_cohort_notif_pending ON cohort_notifications(status, scheduled_at) WHERE status = 'pending';
CREATE INDEX idx_cohort_notif_type ON cohort_notifications(notification_type, status);

CREATE INDEX idx_cohort_analytics_date ON cohort_analytics(snapshot_date);
CREATE INDEX idx_cohort_analytics_cohort ON cohort_analytics(cohort_date);

-- Trigger for updated_at
CREATE TRIGGER participant_cohorts_updated
    BEFORE UPDATE ON participant_cohorts
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

CREATE TRIGGER cohort_daily_updated
    BEFORE UPDATE ON cohort_daily_activity
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

-- Function to check D7 completion
CREATE OR REPLACE FUNCTION check_d7_completion(p_cohort_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    v_cohort participant_cohorts%ROWTYPE;
    v_active_days INT;
BEGIN
    SELECT * INTO v_cohort FROM participant_cohorts WHERE id = p_cohort_id;

    IF v_cohort IS NULL OR v_cohort.d7_completed THEN
        RETURN v_cohort.d7_completed;
    END IF;

    -- Count productive days in D1-D7 window
    SELECT COUNT(DISTINCT activity_date) INTO v_active_days
    FROM cohort_daily_activity
    WHERE cohort_id = p_cohort_id
      AND is_productive = TRUE
      AND activity_date >= v_cohort.cohort_date
      AND activity_date <= v_cohort.d7_target_date;

    -- Update cohort
    UPDATE participant_cohorts
    SET d7_active_days = v_active_days,
        d7_completed = (v_active_days >= v_cohort.d7_required_days),
        d7_completed_at = CASE WHEN v_active_days >= v_cohort.d7_required_days THEN NOW() ELSE NULL END,
        updated_at = NOW()
    WHERE id = p_cohort_id;

    RETURN v_active_days >= v_cohort.d7_required_days;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate cohort analytics
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
    -- Get counts
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

    -- Upsert analytics
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

-- Comments
COMMENT ON TABLE participant_cohorts IS 'Tracks participant D1/D3/D7/D30 milestones from activation';
COMMENT ON TABLE cohort_daily_activity IS 'Daily activity log for productivity tracking';
COMMENT ON TABLE cohort_notifications IS 'Scheduled notifications for cohort milestones';
COMMENT ON TABLE cohort_analytics IS 'Aggregated cohort analytics snapshots';
COMMENT ON COLUMN participant_cohorts.d7_required_days IS 'Number of productive days required in D1-D7 window (default 4)';
