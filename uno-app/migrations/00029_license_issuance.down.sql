-- Rollback migration 00029: License Issuance Model Enhancement

-- Drop functions
DROP FUNCTION IF EXISTS reserve_available_license;
DROP FUNCTION IF EXISTS transition_issuance_state;

-- Drop issuance log
DROP INDEX IF EXISTS idx_issuance_log_failures;
DROP INDEX IF EXISTS idx_issuance_log_actor;
DROP INDEX IF EXISTS idx_issuance_log_license;
DROP TABLE IF EXISTS license_issuance_log;

-- Remove reservation context columns
ALTER TABLE license_reservations DROP COLUMN IF EXISTS referral_code;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS eligibility_ruleset_id;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS ip_address;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS os_version;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS app_version;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS device_type;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS country_code;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS user_id;
DROP INDEX IF EXISTS idx_reservations_user;

-- Remove issuance columns from licenses
DROP INDEX IF EXISTS idx_licenses_issuable;
DROP INDEX IF EXISTS idx_licenses_issuance_state;
ALTER TABLE licenses DROP COLUMN IF EXISTS cancellation_reason;
ALTER TABLE licenses DROP COLUMN IF EXISTS cancelled_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS cancelled_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_ip_address;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_app_version;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_device_type;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_country;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_to;
ALTER TABLE licenses DROP COLUMN IF EXISTS issued_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS issuance_state;

-- Drop issuance state enum
DROP TYPE IF EXISTS issuance_state;
