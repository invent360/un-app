-- Rollback B4 backup entry manifest fields
ALTER TABLE media_backup_entries
    DROP COLUMN IF EXISTS original_storage_url,
    DROP COLUMN IF EXISTS original_filename;
