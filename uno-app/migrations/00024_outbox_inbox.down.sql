-- Rollback migration 00024: Outbox and Inbox

-- Drop sync_checkpoints enhancements
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS source_system;
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS cursor_version;
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS high_water_mark;
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS last_sync_at;
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS last_error;
ALTER TABLE sync_checkpoints DROP COLUMN IF EXISTS error_count;

-- Drop job_queue enhancements
DROP INDEX IF EXISTS idx_job_queue_correlation;
ALTER TABLE job_queue DROP COLUMN IF EXISTS causation_id;
ALTER TABLE job_queue DROP COLUMN IF EXISTS correlation_id;
ALTER TABLE job_queue DROP COLUMN IF EXISTS outcome;
ALTER TABLE job_queue DROP COLUMN IF EXISTS outcome_verified_at;
ALTER TABLE job_queue DROP COLUMN IF EXISTS payload_version;

-- Drop worker_leases
DROP INDEX IF EXISTS idx_worker_leases_type;
DROP INDEX IF EXISTS idx_worker_leases_expiry;
DROP TABLE IF EXISTS worker_leases;

-- Drop inbox
DROP INDEX IF EXISTS idx_inbox_processed;
DROP INDEX IF EXISTS idx_inbox_pending;
DROP TABLE IF EXISTS inbox;

-- Drop outbox
DROP INDEX IF EXISTS idx_outbox_event_type;
DROP INDEX IF EXISTS idx_outbox_idempotency;
DROP INDEX IF EXISTS idx_outbox_aggregate;
DROP INDEX IF EXISTS idx_outbox_retry;
DROP INDEX IF EXISTS idx_outbox_dequeue;
DROP TABLE IF EXISTS outbox;
