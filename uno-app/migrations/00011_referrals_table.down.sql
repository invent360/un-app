-- Drop referrals table and indexes
DROP INDEX IF EXISTS idx_referrals_email;
DROP INDEX IF EXISTS idx_referrals_status;
DROP INDEX IF EXISTS idx_referrals_code;
DROP TABLE IF EXISTS referrals;
