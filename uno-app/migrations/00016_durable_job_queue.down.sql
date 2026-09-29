-- Migration: 00016_durable_job_queue (down)
-- Rollback durable job queue tables

DROP TRIGGER IF EXISTS trigger_job_queue_updated_at ON job_queue;
DROP FUNCTION IF EXISTS update_job_queue_updated_at();

DROP TABLE IF EXISTS health_checks;
DROP TABLE IF EXISTS sync_checkpoints;
DROP TABLE IF EXISTS job_queue;
