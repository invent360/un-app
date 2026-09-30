-- Migration 00021 Down: Remove Consent and Privacy Tables

DROP INDEX IF EXISTS idx_deletion_log_time;
DROP INDEX IF EXISTS idx_deletion_log_request;
DROP INDEX IF EXISTS idx_data_requests_deadline;
DROP INDEX IF EXISTS idx_data_requests_status;
DROP INDEX IF EXISTS idx_data_requests_user;
DROP INDEX IF EXISTS idx_user_consents_active;
DROP INDEX IF EXISTS idx_user_consents_version;
DROP INDEX IF EXISTS idx_user_consents_user;
DROP INDEX IF EXISTS idx_consent_versions_active;
DROP INDEX IF EXISTS idx_consent_versions_type;

DROP TABLE IF EXISTS data_deletion_log;
DROP TABLE IF EXISTS data_access_requests;
DROP TABLE IF EXISTS data_retention_policies;
DROP TABLE IF EXISTS user_consents;
DROP TABLE IF EXISTS consent_versions;
