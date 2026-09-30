-- Migration 00027: License Publication and Withdrawal Authority
-- Phase 4 (P4-02): Publication workflow with correlation IDs and acknowledgements
--
-- Provides:
-- - Publication batches with stable correlation IDs
-- - Per-item acknowledgement before publication
-- - Withdrawal with atomic reservation blocking
-- - Audit trail for all publication actions

-- ============================================
-- PUBLICATION BATCHES TABLE
-- ============================================
-- Each publication operation creates a batch with a stable correlation ID

CREATE TABLE IF NOT EXISTS license_publication_batches (
    id BIGSERIAL PRIMARY KEY,
    correlation_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Publication metadata
    publisher_id VARCHAR(255) NOT NULL,          -- Admin user performing publication
    source_import_batch_id UUID,                 -- Reference to import batch (if from import)

    -- Operation type
    operation VARCHAR(20) NOT NULL,              -- 'publish', 'withdraw', 'dry_run'

    -- Counts
    total_items INT NOT NULL DEFAULT 0,
    acknowledged_count INT NOT NULL DEFAULT 0,   -- Items acknowledged by admin
    published_count INT NOT NULL DEFAULT 0,
    failed_count INT NOT NULL DEFAULT 0,
    pending_count INT NOT NULL DEFAULT 0,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',

    -- Timing
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,                 -- When admin acknowledged the batch
    completed_at TIMESTAMPTZ,

    -- Notes
    notes TEXT,
    error_message TEXT,

    CONSTRAINT valid_pub_operation CHECK (
        operation IN ('publish', 'withdraw', 'dry_run')
    ),
    CONSTRAINT valid_pub_status CHECK (
        status IN ('pending', 'acknowledged', 'processing', 'completed', 'failed', 'cancelled')
    )
);

-- Index for finding batches by publisher
CREATE INDEX IF NOT EXISTS idx_pub_batches_publisher
ON license_publication_batches (publisher_id, created_at DESC);

-- Index for recent batches
CREATE INDEX IF NOT EXISTS idx_pub_batches_recent
ON license_publication_batches (created_at DESC);

-- ============================================
-- PUBLICATION ITEMS TABLE
-- ============================================
-- Per-license publication status within a batch

CREATE TABLE IF NOT EXISTS license_publication_items (
    id BIGSERIAL PRIMARY KEY,
    correlation_id UUID NOT NULL REFERENCES license_publication_batches(correlation_id) ON DELETE CASCADE,
    license_id VARCHAR(66) NOT NULL,

    -- Item status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    acknowledged BOOLEAN NOT NULL DEFAULT FALSE,
    acknowledged_at TIMESTAMPTZ,

    -- Result
    result VARCHAR(20),                          -- 'success', 'failed', 'skipped'
    error_message TEXT,

    -- For withdrawals: track affected reservations
    affected_reservations INT DEFAULT 0,
    reservations_blocked BOOLEAN DEFAULT FALSE,

    -- Timing
    processed_at TIMESTAMPTZ,

    CONSTRAINT valid_item_status CHECK (
        status IN ('pending', 'acknowledged', 'processing', 'completed', 'failed')
    ),
    CONSTRAINT valid_item_result CHECK (
        result IS NULL OR result IN ('success', 'failed', 'skipped')
    )
);

-- Index for items by batch
CREATE INDEX IF NOT EXISTS idx_pub_items_batch
ON license_publication_items (correlation_id, status);

-- Index for items by license
CREATE INDEX IF NOT EXISTS idx_pub_items_license
ON license_publication_items (license_id, correlation_id);

-- ============================================
-- LICENSE PUBLICATION STATE
-- ============================================
-- Track publication status on the license itself

ALTER TABLE licenses ADD COLUMN IF NOT EXISTS publication_status VARCHAR(20) DEFAULT 'draft';
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS published_by VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS publication_correlation_id UUID;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS withdrawn_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS withdrawn_by VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS withdrawal_reason TEXT;

-- Index for finding licenses by publication status
CREATE INDEX IF NOT EXISTS idx_licenses_pub_status
ON licenses (publication_status);

-- Constraint for valid publication status
-- Note: Using a check constraint since we can't easily add an enum in migrations
ALTER TABLE licenses DROP CONSTRAINT IF EXISTS valid_license_pub_status;
ALTER TABLE licenses ADD CONSTRAINT valid_license_pub_status CHECK (
    publication_status IS NULL OR publication_status IN ('draft', 'pending', 'published', 'withdrawn')
);

-- ============================================
-- PUBLICATION AUDIT LOG
-- ============================================
-- Immutable audit trail for all publication actions

CREATE TABLE IF NOT EXISTS license_publication_audit (
    id BIGSERIAL PRIMARY KEY,
    correlation_id UUID NOT NULL,
    license_id VARCHAR(66),
    actor_id VARCHAR(255) NOT NULL,
    action VARCHAR(50) NOT NULL,                 -- 'acknowledge', 'publish', 'withdraw', 'retry', etc.
    details JSONB,
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for audit by correlation
CREATE INDEX IF NOT EXISTS idx_pub_audit_correlation
ON license_publication_audit (correlation_id, created_at DESC);

-- Index for audit by license
CREATE INDEX IF NOT EXISTS idx_pub_audit_license
ON license_publication_audit (license_id, created_at DESC);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE license_publication_batches IS 'Publication batches with stable correlation IDs';
COMMENT ON COLUMN license_publication_batches.correlation_id IS 'Stable ID for tracking publication across retries';
COMMENT ON COLUMN license_publication_batches.acknowledged_count IS 'Items explicitly acknowledged by admin before publish';

COMMENT ON TABLE license_publication_items IS 'Per-license publication status within a batch';
COMMENT ON COLUMN license_publication_items.acknowledged IS 'Admin explicitly acknowledged this item for publication';
COMMENT ON COLUMN license_publication_items.affected_reservations IS 'Count of active reservations affected by withdrawal';

COMMENT ON COLUMN licenses.publication_status IS 'draft=not published, pending=awaiting publish, published=live, withdrawn=removed';
COMMENT ON COLUMN licenses.publication_correlation_id IS 'Correlation ID of last publication batch';

COMMENT ON TABLE license_publication_audit IS 'Immutable audit trail for publication actions';
