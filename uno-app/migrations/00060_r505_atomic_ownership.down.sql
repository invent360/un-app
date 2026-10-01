-- Down migration for 00060_r505_atomic_ownership

-- Drop functions
DROP FUNCTION IF EXISTS complete_license_release(VARCHAR(66));
DROP FUNCTION IF EXISTS confirm_upstream_release(VARCHAR(66), VARCHAR(255), TEXT);
DROP FUNCTION IF EXISTS request_license_release(VARCHAR(66), VARCHAR(255), TEXT);
DROP FUNCTION IF EXISTS atomic_confirm_license(VARCHAR(66), TEXT, VARCHAR(255), TEXT, INT);
DROP FUNCTION IF EXISTS atomic_reserve_license(TEXT, BIGINT, TEXT, TEXT, TIMESTAMPTZ);
DROP FUNCTION IF EXISTS get_locked_capacity(BIGINT);

-- Drop view
DROP VIEW IF EXISTS license_capacity_summary;

-- Drop reservation columns
ALTER TABLE license_reservations DROP COLUMN IF EXISTS credential_revealed_at;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS credential_revealed;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS eligibility_checked_at;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS eligibility_ruleset_id;
ALTER TABLE license_reservations DROP COLUMN IF EXISTS eligibility_passed;

-- Drop license columns
ALTER TABLE licenses DROP COLUMN IF EXISTS is_pending_release;
ALTER TABLE licenses DROP COLUMN IF EXISTS referral_frozen_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS referral_frozen;
ALTER TABLE licenses DROP COLUMN IF EXISTS cooldown_until;
ALTER TABLE licenses DROP COLUMN IF EXISTS upstream_release_confirmed_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS release_requested_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS release_status;

-- Drop enum
DROP TYPE IF EXISTS release_status;
