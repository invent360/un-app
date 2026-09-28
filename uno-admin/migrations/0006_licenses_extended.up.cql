-- Add new columns to licenses table from LicenseApi
-- These fields enable storing full license data synced from Unity API

ALTER TABLE licenses ADD owner_wallet_address text;
ALTER TABLE licenses ADD device_id text;
ALTER TABLE licenses ADD device_name text;
ALTER TABLE licenses ADD activation_start_at timestamp;
ALTER TABLE licenses ADD activation_end_at timestamp;
ALTER TABLE licenses ADD activation_by text;
ALTER TABLE licenses ADD activation_postponed_ms bigint;
ALTER TABLE licenses ADD lease_user_id text;
ALTER TABLE licenses ADD lease_share_percentage double;
ALTER TABLE licenses ADD lease_min_uptime_percentage double;
ALTER TABLE licenses ADD lease_from timestamp;
ALTER TABLE licenses ADD lease_to timestamp;
ALTER TABLE licenses ADD validation_last_success_at timestamp;
ALTER TABLE licenses ADD settings text;
ALTER TABLE licenses ADD synced_at timestamp;
