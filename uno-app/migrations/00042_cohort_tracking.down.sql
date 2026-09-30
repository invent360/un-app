-- Rollback: Cohort Tracking

DROP TRIGGER IF EXISTS cohort_daily_updated ON cohort_daily_activity;
DROP TRIGGER IF EXISTS participant_cohorts_updated ON participant_cohorts;
DROP FUNCTION IF EXISTS calculate_cohort_analytics(DATE, DATE);
DROP FUNCTION IF EXISTS check_d7_completion(UUID);

DROP TABLE IF EXISTS cohort_analytics;
DROP TABLE IF EXISTS cohort_notifications;
DROP TABLE IF EXISTS cohort_daily_activity;
DROP TABLE IF EXISTS participant_cohorts;
