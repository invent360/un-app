-- Migration: R5-03 Admin License Schema Fix
-- Adds license_id column required by uno-admin repository
-- license_id is the external identifier from the API (may differ from internal id)

-- ============================================
-- Add license_id column to licenses table
-- ============================================
-- The uno-admin repository expects both 'id' (internal PK) and 'license_id' (external identifier)
-- license_id is the external identifier from the upstream API/provider
-- It is writable (not generated) because it comes from external systems

ALTER TABLE licenses ADD COLUMN IF NOT EXISTS license_id VARCHAR(66);

-- Populate existing rows with id value as default
UPDATE licenses SET license_id = id WHERE license_id IS NULL;

-- Create index for license_id lookups (uno-admin queries by license_id)
CREATE INDEX IF NOT EXISTS idx_licenses_license_id ON licenses(license_id);

-- ============================================
-- Comments
-- ============================================
COMMENT ON COLUMN licenses.license_id IS 'R5-03: External license identifier from upstream API/provider, used by uno-admin repository';
