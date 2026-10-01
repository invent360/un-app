-- Rollback: R5-03 Admin License Schema Fix

DROP INDEX IF EXISTS idx_licenses_license_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS license_id;
