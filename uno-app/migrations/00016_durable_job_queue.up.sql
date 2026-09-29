-- Migration: 00016_durable_job_queue
-- Description: Create durable PostgreSQL-backed job queue with worker leases
-- Phase 8: Sync & Reconciliation

-- Job queue table for durable job persistence
CREATE TABLE IF NOT EXISTS job_queue (
    id VARCHAR(36) PRIMARY KEY,
    job_type VARCHAR(100) NOT NULL,
    payload TEXT NOT NULL DEFAULT '{}',
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    priority INTEGER NOT NULL DEFAULT 1,
    worker_id VARCHAR(36),
    lease_expires_at TIMESTAMPTZ,
    error_message TEXT,
    retry_at TIMESTAMPTZ,
    idempotency_key VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    result TEXT,

    -- Constraints
    CONSTRAINT valid_status CHECK (status IN ('pending', 'running', 'completed', 'failed', 'dead_letter')),
    CONSTRAINT valid_attempts CHECK (attempts >= 0),
    CONSTRAINT valid_max_attempts CHECK (max_attempts > 0),
    CONSTRAINT valid_priority CHECK (priority >= 0 AND priority <= 3)
);

-- Unique constraint on idempotency key for exactly-once semantics
CREATE UNIQUE INDEX IF NOT EXISTS idx_job_queue_idempotency
ON job_queue (idempotency_key)
WHERE idempotency_key IS NOT NULL;

-- Index for dequeue query (pending jobs ordered by priority and creation time)
CREATE INDEX IF NOT EXISTS idx_job_queue_dequeue
ON job_queue (status, priority DESC, created_at ASC)
WHERE status IN ('pending', 'failed');

-- Index for retry query (failed jobs ready for retry)
CREATE INDEX IF NOT EXISTS idx_job_queue_retry
ON job_queue (retry_at)
WHERE status = 'failed' AND retry_at IS NOT NULL;

-- Index for expired lease reclamation
CREATE INDEX IF NOT EXISTS idx_job_queue_lease_expired
ON job_queue (lease_expires_at)
WHERE status = 'running';

-- Index for dead-letter inspection
CREATE INDEX IF NOT EXISTS idx_job_queue_dead_letter
ON job_queue (completed_at DESC)
WHERE status = 'dead_letter';

-- Index for job type filtering
CREATE INDEX IF NOT EXISTS idx_job_queue_type
ON job_queue (job_type, status);

-- Sync checkpoints table for cursor-based pagination persistence
CREATE TABLE IF NOT EXISTS sync_checkpoints (
    sync_id VARCHAR(100) PRIMARY KEY,
    sync_type VARCHAR(50) NOT NULL,
    cursor_json TEXT,
    records_processed BIGINT NOT NULL DEFAULT 0,
    records_reconciled BIGINT NOT NULL DEFAULT 0,
    is_reconciled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding checkpoints by type
CREATE INDEX IF NOT EXISTS idx_sync_checkpoints_type
ON sync_checkpoints (sync_type);

-- Health check tracking table
CREATE TABLE IF NOT EXISTS health_checks (
    id SERIAL PRIMARY KEY,
    check_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    message TEXT,
    duration_ms INTEGER,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Keep only last 1000 health check records
CREATE INDEX IF NOT EXISTS idx_health_checks_time
ON health_checks (checked_at DESC);

-- Trigger to auto-update updated_at
CREATE OR REPLACE FUNCTION update_job_queue_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_job_queue_updated_at
    BEFORE UPDATE ON job_queue
    FOR EACH ROW
    EXECUTE FUNCTION update_job_queue_updated_at();

-- Add helpful comments
COMMENT ON TABLE job_queue IS 'Durable job queue with worker leases, retries, and dead-letter support';
COMMENT ON COLUMN job_queue.idempotency_key IS 'Unique key for exactly-once job creation semantics';
COMMENT ON COLUMN job_queue.worker_id IS 'UUID of worker that has the lease (NULL if not running)';
COMMENT ON COLUMN job_queue.lease_expires_at IS 'When the worker lease expires (for dead worker detection)';
COMMENT ON COLUMN job_queue.retry_at IS 'When a failed job should be retried (exponential backoff)';
COMMENT ON TABLE sync_checkpoints IS 'Cursor-based sync checkpoints (persisted only after reconciliation)';
COMMENT ON TABLE health_checks IS 'Health probe history for diagnostics';
