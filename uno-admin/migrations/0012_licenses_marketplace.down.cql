-- Remove marketplace fields from licenses table
ALTER TABLE licenses DROP is_on_marketplace;
ALTER TABLE licenses DROP marketplace_status;
ALTER TABLE licenses DROP marketplace_referral_code;
