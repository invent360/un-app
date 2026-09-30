-- Rollback Phase 6: Media Backup Metadata

-- Drop storage health metrics
DROP INDEX IF EXISTS idx_storage_metrics_unhealthy;
DROP INDEX IF EXISTS idx_storage_metrics_mount_time;
DROP TABLE IF EXISTS storage_health_metrics;

-- Drop restore entries
DROP INDEX IF EXISTS idx_restore_entries_status;
DROP TABLE IF EXISTS media_restore_entries;

-- Drop restores
DROP INDEX IF EXISTS idx_media_restores_backup;
DROP INDEX IF EXISTS idx_media_restores_status;
DROP TABLE IF EXISTS media_restores;
DROP TYPE IF EXISTS restore_status;

-- Drop backup entries
DROP INDEX IF EXISTS idx_backup_entries_hash;
DROP INDEX IF EXISTS idx_backup_entries_asset;
DROP TABLE IF EXISTS media_backup_entries;

-- Drop backups
DROP INDEX IF EXISTS idx_media_backups_completed;
DROP INDEX IF EXISTS idx_media_backups_type;
DROP INDEX IF EXISTS idx_media_backups_status;
DROP TABLE IF EXISTS media_backups;
DROP TYPE IF EXISTS backup_type;
DROP TYPE IF EXISTS backup_status;

