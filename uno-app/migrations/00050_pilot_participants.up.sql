-- Phase 9: Pilot participant tracking
-- Enables controlled rollout with 30 participants per market cohort

-- Pilot cohorts for grouping participants
CREATE TABLE IF NOT EXISTS pilot_cohorts (
    id VARCHAR(50) PRIMARY KEY,  -- e.g., "ph-2026-q1", "bd-2026-q1"
    market_code VARCHAR(10) NOT NULL,
    cohort_name VARCHAR(100) NOT NULL,
    target_size INT NOT NULL DEFAULT 30,
    current_size INT NOT NULL DEFAULT 0,
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN NOT NULL DEFAULT false,
    success_criteria JSONB,  -- D7/D30 retention targets, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pilot_cohorts_market ON pilot_cohorts(market_code);
CREATE INDEX IF NOT EXISTS idx_pilot_cohorts_active ON pilot_cohorts(is_active) WHERE is_active = true;

-- Pilot participant states
-- invited -> registered -> license_claimed -> d7_check -> d30_check -> graduated/dropped
CREATE TYPE pilot_participant_state AS ENUM (
    'invited',
    'registered',
    'license_claimed',
    'd7_active',
    'd7_inactive',
    'd30_active',
    'd30_inactive',
    'graduated',
    'dropped',
    'withdrawn'
);

-- Pilot participants
CREATE TABLE IF NOT EXISTS pilot_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_user_id VARCHAR(255) NOT NULL,
    market_code VARCHAR(10) NOT NULL,
    cohort_id VARCHAR(50) NOT NULL REFERENCES pilot_cohorts(id),
    state pilot_participant_state NOT NULL DEFAULT 'invited',

    -- Invitation tracking
    invitation_code VARCHAR(50) UNIQUE,
    invitation_channel VARCHAR(50),  -- sms, email, referral

    -- Milestone timestamps
    invited_at TIMESTAMPTZ,
    registered_at TIMESTAMPTZ,
    license_claimed_at TIMESTAMPTZ,
    d7_check_at TIMESTAMPTZ,
    d30_check_at TIMESTAMPTZ,
    graduated_at TIMESTAMPTZ,

    -- Outcomes
    d7_outcome VARCHAR(50),  -- active, inactive
    d30_outcome VARCHAR(50),  -- active, inactive

    -- Engagement metrics
    sessions_count INT DEFAULT 0,
    activities_count INT DEFAULT 0,
    support_tickets_count INT DEFAULT 0,

    -- License reference
    license_id UUID,

    -- Notes
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_user_per_cohort UNIQUE (external_user_id, cohort_id)
);

CREATE INDEX IF NOT EXISTS idx_pilot_participants_cohort ON pilot_participants(cohort_id);
CREATE INDEX IF NOT EXISTS idx_pilot_participants_market ON pilot_participants(market_code);
CREATE INDEX IF NOT EXISTS idx_pilot_participants_state ON pilot_participants(state);
CREATE INDEX IF NOT EXISTS idx_pilot_participants_user ON pilot_participants(external_user_id);
CREATE INDEX IF NOT EXISTS idx_pilot_participants_d7 ON pilot_participants(d7_check_at)
    WHERE state = 'license_claimed' AND d7_check_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_pilot_participants_d30 ON pilot_participants(d30_check_at)
    WHERE state IN ('d7_active', 'd7_inactive') AND d30_check_at IS NULL;

-- Pilot participant state transitions
CREATE TABLE IF NOT EXISTS pilot_state_transitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    participant_id UUID NOT NULL REFERENCES pilot_participants(id),
    from_state pilot_participant_state,
    to_state pilot_participant_state NOT NULL,
    triggered_by VARCHAR(50) NOT NULL,  -- system, admin, user
    actor VARCHAR(255),
    reason TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pilot_transitions_participant ON pilot_state_transitions(participant_id);
CREATE INDEX IF NOT EXISTS idx_pilot_transitions_created ON pilot_state_transitions(created_at);

-- Pilot activity log
CREATE TABLE IF NOT EXISTS pilot_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    participant_id UUID NOT NULL REFERENCES pilot_participants(id),
    activity_type VARCHAR(50) NOT NULL,  -- login, task_complete, support_contact, feedback
    activity_data JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_pilot_activities_participant ON pilot_activities(participant_id);
CREATE INDEX IF NOT EXISTS idx_pilot_activities_type ON pilot_activities(activity_type);
CREATE INDEX IF NOT EXISTS idx_pilot_activities_recorded ON pilot_activities(recorded_at);

-- Pilot metrics snapshots (for reporting)
CREATE TABLE IF NOT EXISTS pilot_metrics_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cohort_id VARCHAR(50) NOT NULL REFERENCES pilot_cohorts(id),
    snapshot_date DATE NOT NULL,

    -- Funnel metrics
    total_invited INT NOT NULL DEFAULT 0,
    total_registered INT NOT NULL DEFAULT 0,
    total_claimed INT NOT NULL DEFAULT 0,
    total_d7_active INT NOT NULL DEFAULT 0,
    total_d7_inactive INT NOT NULL DEFAULT 0,
    total_d30_active INT NOT NULL DEFAULT 0,
    total_d30_inactive INT NOT NULL DEFAULT 0,
    total_graduated INT NOT NULL DEFAULT 0,
    total_dropped INT NOT NULL DEFAULT 0,

    -- Engagement metrics
    avg_sessions_per_user DECIMAL(10,2),
    avg_activities_per_user DECIMAL(10,2),
    support_ticket_rate DECIMAL(5,4),  -- tickets per user

    -- Retention metrics
    d7_retention_rate DECIMAL(5,4),
    d30_retention_rate DECIMAL(5,4),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_cohort_snapshot UNIQUE (cohort_id, snapshot_date)
);

CREATE INDEX IF NOT EXISTS idx_pilot_metrics_cohort ON pilot_metrics_snapshots(cohort_id);
CREATE INDEX IF NOT EXISTS idx_pilot_metrics_date ON pilot_metrics_snapshots(snapshot_date);

-- Initial pilot cohorts
INSERT INTO pilot_cohorts (id, market_code, cohort_name, target_size, start_date)
VALUES
    ('ph-pilot-1', 'PH', 'Philippines Pilot Cohort 1', 30, '2026-10-01'),
    ('bd-pilot-1', 'BD', 'Bangladesh Pilot Cohort 1', 30, '2026-10-15')
ON CONFLICT (id) DO NOTHING;

-- Function to update cohort current_size
CREATE OR REPLACE FUNCTION update_cohort_size()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE pilot_cohorts
        SET current_size = current_size + 1, updated_at = NOW()
        WHERE id = NEW.cohort_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE pilot_cohorts
        SET current_size = current_size - 1, updated_at = NOW()
        WHERE id = OLD.cohort_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to maintain cohort size
DROP TRIGGER IF EXISTS trg_update_cohort_size ON pilot_participants;
CREATE TRIGGER trg_update_cohort_size
    AFTER INSERT OR DELETE ON pilot_participants
    FOR EACH ROW EXECUTE FUNCTION update_cohort_size();

COMMENT ON TABLE pilot_cohorts IS 'Phase 9: Pilot cohort definitions for controlled rollout';
COMMENT ON TABLE pilot_participants IS 'Phase 9: Individual pilot participant tracking';
COMMENT ON TABLE pilot_state_transitions IS 'Phase 9: State machine history for pilot participants';
COMMENT ON TABLE pilot_activities IS 'Phase 9: Engagement activity log for pilot participants';
COMMENT ON TABLE pilot_metrics_snapshots IS 'Phase 9: Daily metrics snapshots for reporting';
