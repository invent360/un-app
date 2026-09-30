-- Rollback migration 00032: License Lifecycle

-- Drop functions
DROP FUNCTION IF EXISTS reactivate_license;
DROP FUNCTION IF EXISTS process_expired_licenses;
DROP FUNCTION IF EXISTS update_license_exposure;
DROP FUNCTION IF EXISTS release_license;
DROP FUNCTION IF EXISTS cancel_license;
DROP FUNCTION IF EXISTS log_lifecycle_event;

-- Drop view
DROP VIEW IF EXISTS license_exposure_summary;

-- Drop notifications table
DROP INDEX IF EXISTS idx_expiry_notifications_type;
DROP INDEX IF EXISTS idx_expiry_notifications_license;
DROP TABLE IF EXISTS license_expiry_notifications;

-- Drop lifecycle log
DROP INDEX IF EXISTS idx_lifecycle_log_actor;
DROP INDEX IF EXISTS idx_lifecycle_log_event;
DROP INDEX IF EXISTS idx_lifecycle_log_license;
DROP TABLE IF EXISTS license_lifecycle_log;

-- Remove license columns
ALTER TABLE licenses DROP COLUMN IF EXISTS last_exposure_calc_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS exposure_claimed_secs;
ALTER TABLE licenses DROP COLUMN IF EXISTS exposure_reserved_secs;
ALTER TABLE licenses DROP COLUMN IF EXISTS exposure_published_secs;
ALTER TABLE licenses DROP COLUMN IF EXISTS original_valid_to;
ALTER TABLE licenses DROP COLUMN IF EXISTS reactivated_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS reactivated_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS expiry_notification_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS expiry_notification_sent;
ALTER TABLE licenses DROP COLUMN IF EXISTS expired_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS release_reason;
ALTER TABLE licenses DROP COLUMN IF EXISTS released_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS released_at;

-- Drop lifecycle event enum
DROP TYPE IF EXISTS lifecycle_event;
