-- R3-13: Onboarding journey state machine rollback
-- =============================================================================

-- Remove retention policy
DELETE FROM retention_policies WHERE table_name = 'user_journey_events';

-- Drop journey events table
DROP TABLE IF EXISTS user_journey_events;

-- Drop onboarding progress table
DROP TABLE IF EXISTS onboarding_progress;
