-- Rollback License Lifecycle Migration

-- Drop tables
DROP TABLE IF EXISTS license_reservations;
DROP TABLE IF EXISTS agreement_versions;

-- Remove indexes
DROP INDEX IF EXISTS idx_licenses_available;
DROP INDEX IF EXISTS idx_licenses_referral;
DROP INDEX IF EXISTS idx_licenses_reserved_until;
DROP INDEX IF EXISTS idx_licenses_split_type;
DROP INDEX IF EXISTS idx_licenses_valid_to;
DROP INDEX IF EXISTS idx_licenses_valid_from;
DROP INDEX IF EXISTS idx_licenses_claimed;
DROP INDEX IF EXISTS idx_licenses_lease_code;

-- Remove columns (in reverse order of addition)
ALTER TABLE licenses DROP COLUMN IF EXISTS referral_agreement_version;
ALTER TABLE licenses DROP COLUMN IF EXISTS referral_attributed_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS referral_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS reservation_token;
ALTER TABLE licenses DROP COLUMN IF EXISTS reserved_until;
ALTER TABLE licenses DROP COLUMN IF EXISTS device_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS bound_to_device;
ALTER TABLE licenses DROP COLUMN IF EXISTS claimed_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS claimed_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS claimed;
ALTER TABLE licenses DROP COLUMN IF EXISTS split_type;
ALTER TABLE licenses DROP COLUMN IF EXISTS valid_to;
ALTER TABLE licenses DROP COLUMN IF EXISTS valid_from;
ALTER TABLE licenses DROP COLUMN IF EXISTS lease_code;

-- Drop enum type
DROP TYPE IF EXISTS split_type;
