-- Rollback: Remove extended license columns
ALTER TABLE licenses DROP owner_wallet_address;
ALTER TABLE licenses DROP device_id;
ALTER TABLE licenses DROP device_name;
ALTER TABLE licenses DROP activation_start_at;
ALTER TABLE licenses DROP activation_end_at;
ALTER TABLE licenses DROP activation_by;
ALTER TABLE licenses DROP activation_postponed_ms;
ALTER TABLE licenses DROP lease_user_id;
ALTER TABLE licenses DROP lease_share_percentage;
ALTER TABLE licenses DROP lease_min_uptime_percentage;
ALTER TABLE licenses DROP lease_from;
ALTER TABLE licenses DROP lease_to;
ALTER TABLE licenses DROP validation_last_success_at;
ALTER TABLE licenses DROP settings;
ALTER TABLE licenses DROP synced_at;
