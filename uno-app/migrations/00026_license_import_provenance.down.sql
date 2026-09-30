-- Rollback migration 00026: License Import Provenance Tracking

-- Drop quarantine reviews
DROP INDEX IF EXISTS idx_quarantine_reviews_license;
DROP TABLE IF EXISTS license_quarantine_reviews;

-- Remove provenance columns from licenses
DROP INDEX IF EXISTS idx_licenses_quarantined;
DROP INDEX IF EXISTS idx_licenses_import_batch;
ALTER TABLE licenses DROP COLUMN IF EXISTS is_quarantined;
ALTER TABLE licenses DROP COLUMN IF EXISTS quarantine_reason;
ALTER TABLE licenses DROP COLUMN IF EXISTS agreement_version_at_import;
ALTER TABLE licenses DROP COLUMN IF EXISTS source_credential;
ALTER TABLE licenses DROP COLUMN IF EXISTS provider_reference;
ALTER TABLE licenses DROP COLUMN IF EXISTS provider_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS import_row_number;
ALTER TABLE licenses DROP COLUMN IF EXISTS import_batch_id;

-- Drop import rows
DROP INDEX IF EXISTS idx_import_rows_invalid;
DROP INDEX IF EXISTS idx_import_rows_batch;
DROP TABLE IF EXISTS license_import_rows;

-- Drop import batches
DROP INDEX IF EXISTS idx_import_batches_recent;
DROP INDEX IF EXISTS idx_import_batches_provider;
DROP INDEX IF EXISTS idx_import_batches_importer;
DROP TABLE IF EXISTS license_import_batches;
