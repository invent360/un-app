-- Rollback migration 00027: License Publication and Withdrawal Authority

-- Drop audit log
DROP INDEX IF EXISTS idx_pub_audit_license;
DROP INDEX IF EXISTS idx_pub_audit_correlation;
DROP TABLE IF EXISTS license_publication_audit;

-- Remove license publication columns
DROP INDEX IF EXISTS idx_licenses_pub_status;
ALTER TABLE licenses DROP CONSTRAINT IF EXISTS valid_license_pub_status;
ALTER TABLE licenses DROP COLUMN IF EXISTS withdrawal_reason;
ALTER TABLE licenses DROP COLUMN IF EXISTS withdrawn_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS withdrawn_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS publication_correlation_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS published_by;
ALTER TABLE licenses DROP COLUMN IF EXISTS published_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS publication_status;

-- Drop publication items
DROP INDEX IF EXISTS idx_pub_items_license;
DROP INDEX IF EXISTS idx_pub_items_batch;
DROP TABLE IF EXISTS license_publication_items;

-- Drop publication batches
DROP INDEX IF EXISTS idx_pub_batches_recent;
DROP INDEX IF EXISTS idx_pub_batches_publisher;
DROP TABLE IF EXISTS license_publication_batches;
