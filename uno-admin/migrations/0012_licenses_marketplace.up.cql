-- Add marketplace fields to licenses table
ALTER TABLE licenses ADD is_on_marketplace boolean;
ALTER TABLE licenses ADD marketplace_status text;
ALTER TABLE licenses ADD marketplace_referral_code text;
