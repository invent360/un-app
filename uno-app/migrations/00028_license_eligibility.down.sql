-- Rollback migration 00028: License Eligibility Rules

-- Drop eligibility check log
DROP INDEX IF EXISTS idx_elig_check_failures;
DROP INDEX IF EXISTS idx_elig_check_country;
DROP INDEX IF EXISTS idx_elig_check_license;
DROP TABLE IF EXISTS eligibility_check_log;

-- Remove license eligibility columns
DROP INDEX IF EXISTS idx_licenses_eligibility_ruleset;
ALTER TABLE licenses DROP COLUMN IF EXISTS requires_verification;
ALTER TABLE licenses DROP COLUMN IF EXISTS allowed_device_types;
ALTER TABLE licenses DROP COLUMN IF EXISTS allowed_task_types;
ALTER TABLE licenses DROP COLUMN IF EXISTS blocked_countries;
ALTER TABLE licenses DROP COLUMN IF EXISTS allowed_countries;
ALTER TABLE licenses DROP COLUMN IF EXISTS eligibility_ruleset_id;

-- Drop eligibility rulesets
DROP INDEX IF EXISTS idx_eligibility_rulesets_name;
DROP TABLE IF EXISTS eligibility_rulesets;
