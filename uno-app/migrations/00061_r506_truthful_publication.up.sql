-- Migration 00061: R5-06 Truthful Publication and Synchronization
-- Implements truthful mappings, per-item outcomes, and compound cursor support
--
-- Key changes:
-- 1. Add exact share percentages (not just collapsed enum)
-- 2. Add source provenance tracking
-- 3. Add publication quarantine for invalid items
-- 4. Add compound cursor support for sync

-- ============================================
-- EXACT SHARE PERCENTAGES
-- ============================================
-- R5-06: Preserve exact percentages instead of collapsing to enum

ALTER TABLE licenses ADD COLUMN IF NOT EXISTS uno_share_pct NUMERIC(5,2);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS ulo_share_pct NUMERIC(5,2);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS agent_share_pct NUMERIC(5,2);

COMMENT ON COLUMN licenses.uno_share_pct IS 'R5-06: Exact UNO share percentage (e.g., 47.50, not collapsed to enum)';
COMMENT ON COLUMN licenses.ulo_share_pct IS 'R5-06: Exact ULO share percentage';
COMMENT ON COLUMN licenses.agent_share_pct IS 'R5-06: Exact agent share percentage';

-- ============================================
-- SOURCE PROVENANCE TRACKING
-- ============================================
-- R5-06: Maintain source/version provenance in imports

ALTER TABLE licenses ADD COLUMN IF NOT EXISTS source_system VARCHAR(50);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS source_version VARCHAR(50);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS source_imported_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS source_record_id VARCHAR(100);

COMMENT ON COLUMN licenses.source_system IS 'R5-06: Origin system (e.g., "unetwork", "manual", "csv_import")';
COMMENT ON COLUMN licenses.source_version IS 'R5-06: Version of the source data at import time';
COMMENT ON COLUMN licenses.source_imported_at IS 'R5-06: When the source data was imported';
COMMENT ON COLUMN licenses.source_record_id IS 'R5-06: Original record ID in source system';

-- ============================================
-- PUBLICATION QUARANTINE
-- ============================================
-- R5-06: Incomplete upstream records are quarantined with reasons

CREATE TABLE IF NOT EXISTS publication_quarantine (
    id BIGSERIAL PRIMARY KEY,

    -- Source identification
    source_license_id VARCHAR(100) NOT NULL,
    source_system VARCHAR(50) NOT NULL,
    source_record JSONB,

    -- Quarantine reason
    reason_code VARCHAR(50) NOT NULL,
    reason_message TEXT,

    -- Validation failures
    validation_errors JSONB DEFAULT '[]',

    -- State
    status VARCHAR(20) NOT NULL DEFAULT 'quarantined' CHECK (status IN ('quarantined', 'resolved', 'rejected')),
    resolved_at TIMESTAMPTZ,
    resolved_by VARCHAR(255),
    resolution_notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(source_license_id, source_system)
);

CREATE INDEX IF NOT EXISTS idx_quarantine_status ON publication_quarantine(status);
CREATE INDEX IF NOT EXISTS idx_quarantine_reason ON publication_quarantine(reason_code);
CREATE INDEX IF NOT EXISTS idx_quarantine_created ON publication_quarantine(created_at DESC);

COMMENT ON TABLE publication_quarantine IS 'R5-06: Incomplete/invalid upstream records quarantined with reasons';

-- ============================================
-- COMPOUND CURSOR TABLE
-- ============================================
-- R5-06: Persist and transmit both cursor parts for >2,500 records

CREATE TABLE IF NOT EXISTS sync_cursors (
    id BIGSERIAL PRIMARY KEY,

    -- Cursor identification
    sync_type VARCHAR(50) NOT NULL,
    direction VARCHAR(10) NOT NULL DEFAULT 'forward' CHECK (direction IN ('forward', 'backward')),

    -- Compound cursor values
    -- R5-06: Query lexicographic continuation with both parts
    cursor_timestamp TIMESTAMPTZ,
    cursor_id VARCHAR(100),
    cursor_sequence BIGINT,

    -- High-water mark for tie-breaking
    high_water_mark JSONB,

    -- Progress tracking
    processed_count BIGINT NOT NULL DEFAULT 0,
    last_batch_size INT,

    -- State
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'failed', 'paused')),
    error_message TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,

    UNIQUE(sync_type, direction)
);

COMMENT ON TABLE sync_cursors IS 'R5-06: Compound cursors for deterministic pagination across restarts';

-- ============================================
-- PUBLICATION BATCH OUTCOMES
-- ============================================
-- R5-06: Return per-item batch outcomes correlated to input IDs

ALTER TABLE license_publication_items ADD COLUMN IF NOT EXISTS input_index INT;
ALTER TABLE license_publication_items ADD COLUMN IF NOT EXISTS input_license_id VARCHAR(100);
ALTER TABLE license_publication_items ADD COLUMN IF NOT EXISTS quarantine_id BIGINT REFERENCES publication_quarantine(id);

COMMENT ON COLUMN license_publication_items.input_index IS 'R5-06: Index in the input batch for correlation';
COMMENT ON COLUMN license_publication_items.input_license_id IS 'R5-06: Original license ID from input for correlation';

-- ============================================
-- FUNCTIONS
-- ============================================

-- R5-06: Get compound cursor for continuation
CREATE OR REPLACE FUNCTION get_sync_cursor(p_sync_type VARCHAR(50))
RETURNS TABLE (
    cursor_timestamp TIMESTAMPTZ,
    cursor_id VARCHAR(100),
    cursor_sequence BIGINT,
    processed_count BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT sc.cursor_timestamp, sc.cursor_id, sc.cursor_sequence, sc.processed_count
    FROM sync_cursors sc
    WHERE sc.sync_type = p_sync_type
      AND sc.status = 'active';
END;
$$ LANGUAGE plpgsql;

-- R5-06: Update compound cursor atomically
CREATE OR REPLACE FUNCTION update_sync_cursor(
    p_sync_type VARCHAR(50),
    p_timestamp TIMESTAMPTZ,
    p_id VARCHAR(100),
    p_sequence BIGINT DEFAULT NULL,
    p_batch_size INT DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    INSERT INTO sync_cursors (sync_type, cursor_timestamp, cursor_id, cursor_sequence, last_batch_size, processed_count)
    VALUES (p_sync_type, p_timestamp, p_id, p_sequence, p_batch_size, COALESCE(p_batch_size, 0))
    ON CONFLICT (sync_type, direction) DO UPDATE SET
        cursor_timestamp = EXCLUDED.cursor_timestamp,
        cursor_id = EXCLUDED.cursor_id,
        cursor_sequence = EXCLUDED.cursor_sequence,
        last_batch_size = EXCLUDED.last_batch_size,
        processed_count = sync_cursors.processed_count + COALESCE(EXCLUDED.last_batch_size, 0),
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- R5-06: Quarantine invalid license
CREATE OR REPLACE FUNCTION quarantine_license(
    p_source_license_id VARCHAR(100),
    p_source_system VARCHAR(50),
    p_reason_code VARCHAR(50),
    p_reason_message TEXT,
    p_source_record JSONB DEFAULT NULL,
    p_validation_errors JSONB DEFAULT '[]'
) RETURNS BIGINT AS $$
DECLARE
    v_id BIGINT;
BEGIN
    INSERT INTO publication_quarantine (
        source_license_id, source_system, source_record,
        reason_code, reason_message, validation_errors
    )
    VALUES (
        p_source_license_id, p_source_system, p_source_record,
        p_reason_code, p_reason_message, p_validation_errors
    )
    ON CONFLICT (source_license_id, source_system) DO UPDATE SET
        reason_code = EXCLUDED.reason_code,
        reason_message = EXCLUDED.reason_message,
        validation_errors = EXCLUDED.validation_errors,
        source_record = COALESCE(EXCLUDED.source_record, publication_quarantine.source_record),
        updated_at = NOW()
    RETURNING id INTO v_id;

    RETURN v_id;
END;
$$ LANGUAGE plpgsql;

-- R5-06: Fetch licenses with compound cursor for sync
-- Returns licenses using lexicographic ordering on (claimed_at, id)
CREATE OR REPLACE FUNCTION fetch_licenses_for_sync(
    p_cursor_timestamp TIMESTAMPTZ DEFAULT NULL,
    p_cursor_id VARCHAR(100) DEFAULT NULL,
    p_limit INT DEFAULT 100
) RETURNS TABLE (
    license_id VARCHAR(66),
    lease_code VARCHAR(100),
    claimed_at TIMESTAMPTZ,
    split_type TEXT,
    uno_share_pct NUMERIC(5,2),
    ulo_share_pct NUMERIC(5,2),
    agent_share_pct NUMERIC(5,2),
    issued_to VARCHAR(255),
    referral_id INT,
    is_last_batch BOOLEAN
) AS $$
DECLARE
    v_count INT;
BEGIN
    -- Get total count for determining if this is last batch
    SELECT COUNT(*) INTO v_count
    FROM licenses l
    WHERE l.claimed = true
      AND (
          p_cursor_timestamp IS NULL
          OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
      );

    RETURN QUERY
    SELECT
        l.id::VARCHAR(66) as license_id,
        l.lease_code::VARCHAR(100),
        l.claimed_at,
        l.split_type::text,
        l.uno_share_pct,
        l.ulo_share_pct,
        l.agent_share_pct,
        l.issued_to,
        l.referral_id,
        (ROW_NUMBER() OVER (ORDER BY l.claimed_at ASC, l.id ASC) = v_count) as is_last_batch
    FROM licenses l
    WHERE l.claimed = true
      AND (
          p_cursor_timestamp IS NULL
          OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
      )
    ORDER BY l.claimed_at ASC, l.id ASC
    LIMIT p_limit;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- INDEXES
-- ============================================

-- R5-06: Compound index for cursor-based pagination
CREATE INDEX IF NOT EXISTS idx_licenses_sync_cursor
ON licenses (claimed_at ASC, id ASC)
WHERE claimed = true;

-- R5-06: Index for source provenance queries
CREATE INDEX IF NOT EXISTS idx_licenses_source
ON licenses (source_system, source_imported_at DESC)
WHERE source_system IS NOT NULL;

-- ============================================
-- TRIGGER FOR updated_at
-- ============================================

CREATE OR REPLACE FUNCTION update_quarantine_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS publication_quarantine_updated ON publication_quarantine;
CREATE TRIGGER publication_quarantine_updated
    BEFORE UPDATE ON publication_quarantine
    FOR EACH ROW
    EXECUTE FUNCTION update_quarantine_timestamp();

DROP TRIGGER IF EXISTS sync_cursors_updated ON sync_cursors;
CREATE TRIGGER sync_cursors_updated
    BEFORE UPDATE ON sync_cursors
    FOR EACH ROW
    EXECUTE FUNCTION update_quarantine_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON FUNCTION get_sync_cursor IS 'R5-06: Get compound cursor for continuation';
COMMENT ON FUNCTION update_sync_cursor IS 'R5-06: Update compound cursor atomically';
COMMENT ON FUNCTION quarantine_license IS 'R5-06: Quarantine invalid license with reason';
COMMENT ON FUNCTION fetch_licenses_for_sync IS 'R5-06: Fetch licenses with compound cursor, lexicographic ordering';
