-- Rollback migration 00025: Service Identities

DROP TRIGGER IF EXISTS trigger_update_service_identity_timestamp ON service_identities;
DROP FUNCTION IF EXISTS update_service_identity_timestamp();

DROP INDEX IF EXISTS idx_service_request_log_cleanup;
DROP INDEX IF EXISTS idx_service_request_log_rate_limit;
DROP TABLE IF EXISTS service_request_log;

DROP INDEX IF EXISTS idx_service_identity_audit_service;
DROP TABLE IF EXISTS service_identity_audit;

DROP INDEX IF EXISTS idx_service_identities_type;
DROP INDEX IF EXISTS idx_service_identities_service_id;
DROP TABLE IF EXISTS service_identities;
