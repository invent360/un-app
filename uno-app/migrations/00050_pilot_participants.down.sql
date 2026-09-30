-- Rollback Phase 9: Pilot participant tracking

DROP TRIGGER IF EXISTS trg_update_cohort_size ON pilot_participants;
DROP FUNCTION IF EXISTS update_cohort_size();

DROP TABLE IF EXISTS pilot_metrics_snapshots;
DROP TABLE IF EXISTS pilot_activities;
DROP TABLE IF EXISTS pilot_state_transitions;
DROP TABLE IF EXISTS pilot_participants;
DROP TABLE IF EXISTS pilot_cohorts;
DROP TYPE IF EXISTS pilot_participant_state;
