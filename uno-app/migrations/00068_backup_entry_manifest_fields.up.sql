-- B4 FIX: Add original filename and storage URL fields to backup entries
-- These fields are needed to reconstruct the manifest hash during verification

ALTER TABLE media_backup_entries
    ADD COLUMN IF NOT EXISTS original_filename VARCHAR(500),
    ADD COLUMN IF NOT EXISTS original_storage_url VARCHAR(2000);

COMMENT ON COLUMN media_backup_entries.original_filename IS 'Original filename from media asset (for manifest hash verification)';
COMMENT ON COLUMN media_backup_entries.original_storage_url IS 'Original storage URL from media asset (for manifest hash verification and restoration)';
