-- Rollback migration 00030: Ownership Validation

-- Drop functions
DROP FUNCTION IF EXISTS establish_ownership;
DROP FUNCTION IF EXISTS check_gates_for_operation;

-- Remove license columns
ALTER TABLE licenses DROP COLUMN IF EXISTS owner_verification_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS owner_verified;
ALTER TABLE licenses DROP COLUMN IF EXISTS verification_requirement_id;

-- Drop verification requirements
DROP TABLE IF EXISTS verification_requirements;

-- Drop gate validation log
DROP INDEX IF EXISTS idx_gate_validation_failures;
DROP INDEX IF EXISTS idx_gate_validation_license;
DROP TABLE IF EXISTS gate_validation_log;

-- Drop ownership table
DROP INDEX IF EXISTS idx_license_ownership_unique_current;
DROP INDEX IF EXISTS idx_license_ownership_owner;
DROP INDEX IF EXISTS idx_license_ownership_current;
DROP TABLE IF EXISTS license_ownership;
