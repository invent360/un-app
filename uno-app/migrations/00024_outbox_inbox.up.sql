-- Migration 00024: Outbox and Inbox for Transactional Event Publishing
-- Phase 3 (P3-01, P3-04): Durable work with exactly-once semantics
--
-- Outbox Pattern: Domain events are persisted atomically with business transactions.
-- A background worker reads the outbox and publishes events to external systems.
--
-- Inbox Pattern: Inbound events are deduplicated before processing.
-- Prevents duplicate handling of replayed or retried events.

-- ============================================
-- OUTBOX TABLE
-- ============================================
-- Events to be published to external systems or processed by workers.
-- Persisted in the same transaction as the domain mutation.

CREATE TABLE IF NOT EXISTS outbox (
    id BIGSERIAL PRIMARY KEY,

    -- Event identification
    event_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,           -- e.g., 'license.claimed', 'consent.granted'
    event_version INT NOT NULL DEFAULT 1,        -- Schema version for backwards compatibility

    -- Aggregate reference (what entity this event is about)
    aggregate_type VARCHAR(100) NOT NULL,        -- e.g., 'license', 'user', 'consent'
    aggregate_id VARCHAR(255) NOT NULL,          -- ID of the aggregate

    -- Event payload
    payload JSONB NOT NULL,                      -- Event data (versioned schema)
    metadata JSONB,                              -- Additional context (correlation_id, causation_id, etc.)

    -- Idempotency
    idempotency_key VARCHAR(255),                -- Optional dedup key for event creation

    -- Publishing state
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    published_at TIMESTAMPTZ,
    publish_attempts INT NOT NULL DEFAULT 0,
    last_error TEXT,
    next_retry_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_outbox_status CHECK (
        status IN ('pending', 'publishing', 'published', 'failed', 'dead_letter')
    )
);

-- Index for event dequeue (pending events ordered by creation time)
CREATE INDEX IF NOT EXISTS idx_outbox_dequeue
ON outbox (status, created_at ASC)
WHERE status IN ('pending', 'failed');

-- Index for retry scheduling
CREATE INDEX IF NOT EXISTS idx_outbox_retry
ON outbox (next_retry_at)
WHERE status = 'failed' AND next_retry_at IS NOT NULL;

-- Index for aggregate event history
CREATE INDEX IF NOT EXISTS idx_outbox_aggregate
ON outbox (aggregate_type, aggregate_id, created_at DESC);

-- Unique constraint on idempotency key
CREATE UNIQUE INDEX IF NOT EXISTS idx_outbox_idempotency
ON outbox (idempotency_key)
WHERE idempotency_key IS NOT NULL;

-- Index for event type filtering
CREATE INDEX IF NOT EXISTS idx_outbox_event_type
ON outbox (event_type, status);

-- ============================================
-- INBOX TABLE
-- ============================================
-- Processed inbound events for idempotent handling.
-- Before processing an event, check if it's already in the inbox.

CREATE TABLE IF NOT EXISTS inbox (
    id BIGSERIAL PRIMARY KEY,

    -- Event identification (from external source)
    event_id VARCHAR(255) NOT NULL,              -- External event ID
    source VARCHAR(100) NOT NULL,                -- Event source system

    -- Processing state
    status VARCHAR(20) NOT NULL DEFAULT 'received',
    processed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Event data (stored for debugging/replay)
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,

    -- Timestamps
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Composite unique constraint for deduplication
    CONSTRAINT uq_inbox_event UNIQUE (source, event_id),

    CONSTRAINT valid_inbox_status CHECK (
        status IN ('received', 'processing', 'processed', 'failed', 'skipped')
    )
);

-- Index for pending inbox processing
CREATE INDEX IF NOT EXISTS idx_inbox_pending
ON inbox (status, received_at ASC)
WHERE status IN ('received', 'failed');

-- Index for processed events cleanup
CREATE INDEX IF NOT EXISTS idx_inbox_processed
ON inbox (processed_at)
WHERE status = 'processed';

-- ============================================
-- WORKER LEASES TABLE
-- ============================================
-- Track worker instances for distributed processing.
-- Workers claim leases on jobs and heartbeat to maintain them.

CREATE TABLE IF NOT EXISTS worker_leases (
    worker_id UUID PRIMARY KEY,
    worker_name VARCHAR(255) NOT NULL,
    worker_type VARCHAR(50) NOT NULL,            -- 'job_processor', 'outbox_publisher', 'sync_worker'

    -- Lease state
    heartbeat_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    lease_expires_at TIMESTAMPTZ NOT NULL,

    -- Worker metadata
    host VARCHAR(255),
    process_id INT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Statistics
    jobs_processed BIGINT NOT NULL DEFAULT 0,
    jobs_failed BIGINT NOT NULL DEFAULT 0,
    last_job_at TIMESTAMPTZ
);

-- Index for expired lease detection
CREATE INDEX IF NOT EXISTS idx_worker_leases_expiry
ON worker_leases (lease_expires_at)
WHERE lease_expires_at < NOW();

-- Index for worker type filtering
CREATE INDEX IF NOT EXISTS idx_worker_leases_type
ON worker_leases (worker_type, heartbeat_at DESC);

-- ============================================
-- ENHANCE JOB_QUEUE TABLE
-- ============================================
-- Add columns for better event/outcome tracking

-- Add causation tracking (which event/job triggered this job)
ALTER TABLE job_queue ADD COLUMN IF NOT EXISTS causation_id VARCHAR(255);
ALTER TABLE job_queue ADD COLUMN IF NOT EXISTS correlation_id VARCHAR(255);

-- Add outcome tracking for ambiguous results
ALTER TABLE job_queue ADD COLUMN IF NOT EXISTS outcome VARCHAR(50);
ALTER TABLE job_queue ADD COLUMN IF NOT EXISTS outcome_verified_at TIMESTAMPTZ;

-- Add payload versioning
ALTER TABLE job_queue ADD COLUMN IF NOT EXISTS payload_version INT NOT NULL DEFAULT 1;

-- Index for correlation tracking
CREATE INDEX IF NOT EXISTS idx_job_queue_correlation
ON job_queue (correlation_id)
WHERE correlation_id IS NOT NULL;

-- ============================================
-- SYNC CURSOR IMPROVEMENTS
-- ============================================
-- Enhance sync_checkpoints for durable cursor management

-- Add source tracking
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS source_system VARCHAR(100);

-- Add cursor versioning (for schema migrations)
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS cursor_version INT NOT NULL DEFAULT 1;

-- Add high-water mark for equal-timestamp ordering
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS high_water_mark VARCHAR(255);

-- Add last successful sync timestamp
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMPTZ;

-- Add error tracking
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS last_error TEXT;
ALTER TABLE sync_checkpoints ADD COLUMN IF NOT EXISTS error_count INT NOT NULL DEFAULT 0;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE outbox IS 'Outbox for transactional event publishing (events persisted atomically with business transactions)';
COMMENT ON COLUMN outbox.event_version IS 'Schema version for payload backwards compatibility';
COMMENT ON COLUMN outbox.idempotency_key IS 'Prevents duplicate event creation for same logical operation';
COMMENT ON COLUMN outbox.metadata IS 'Context like correlation_id, causation_id, user_id, etc.';

COMMENT ON TABLE inbox IS 'Inbox for idempotent event consumption (prevents duplicate processing)';
COMMENT ON COLUMN inbox.source IS 'External system that sent the event';
COMMENT ON COLUMN inbox.event_id IS 'External event ID for deduplication';

COMMENT ON TABLE worker_leases IS 'Active worker instances with lease tracking for distributed processing';
COMMENT ON COLUMN worker_leases.heartbeat_at IS 'Last heartbeat timestamp (workers must heartbeat before lease_expires_at)';

COMMENT ON COLUMN job_queue.causation_id IS 'ID of event/job that caused this job to be created';
COMMENT ON COLUMN job_queue.correlation_id IS 'ID to correlate related jobs across the system';
COMMENT ON COLUMN job_queue.outcome IS 'Result classification: success, failure, timeout, unknown';
COMMENT ON COLUMN job_queue.payload_version IS 'Schema version of payload for backwards compatibility';
