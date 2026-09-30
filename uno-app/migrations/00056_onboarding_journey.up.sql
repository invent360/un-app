-- R3-13: Onboarding journey state machine
-- =============================================================================
-- This migration creates tables for tracking user onboarding progress
-- and funnel analytics events.
-- =============================================================================

-- User onboarding progress table
-- Stores the current state of each user's onboarding journey
CREATE TABLE IF NOT EXISTS onboarding_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    session_id UUID,
    state_json JSONB NOT NULL DEFAULT '{"state":"landing"}',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Indexes for onboarding progress
CREATE INDEX IF NOT EXISTS idx_onboarding_progress_user ON onboarding_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_onboarding_progress_session ON onboarding_progress(session_id);
CREATE INDEX IF NOT EXISTS idx_onboarding_progress_completed ON onboarding_progress(completed_at) WHERE completed_at IS NOT NULL;

-- User journey events table
-- Records funnel stage events for analytics
CREATE TABLE IF NOT EXISTS user_journey_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID,
    visitor_id VARCHAR(255),
    license_id UUID,
    stage_id VARCHAR(100) NOT NULL,
    measured_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB
);

-- Indexes for journey events analytics
CREATE INDEX IF NOT EXISTS idx_journey_events_session ON user_journey_events(session_id);
CREATE INDEX IF NOT EXISTS idx_journey_events_visitor ON user_journey_events(visitor_id);
CREATE INDEX IF NOT EXISTS idx_journey_events_license ON user_journey_events(license_id);
CREATE INDEX IF NOT EXISTS idx_journey_events_stage ON user_journey_events(stage_id);
CREATE INDEX IF NOT EXISTS idx_journey_events_measured ON user_journey_events(measured_at);
CREATE INDEX IF NOT EXISTS idx_journey_events_measured_stage ON user_journey_events(measured_at, stage_id);

-- Add retention policy for journey events (90 days)
INSERT INTO retention_policies (table_name, retention_days, archive_before_delete, deletion_condition)
VALUES ('user_journey_events', 90, false, 'measured_at < NOW() - INTERVAL ''90 days''')
ON CONFLICT (table_name) DO UPDATE
SET retention_days = 90,
    deletion_condition = 'measured_at < NOW() - INTERVAL ''90 days''';
