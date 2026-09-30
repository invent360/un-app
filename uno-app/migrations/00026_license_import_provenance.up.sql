-- Migration 00026: License Import Provenance Tracking
-- Phase 4 (P4-01): Versioned import contract with source identity and provenance
--
-- Tracks every import batch with:
-- - Source identity and provider
-- - Original upload retention
-- - Per-row validation results
-- - Exact accepted/rejected/unchanged totals

-- ============================================
-- IMPORT BATCHES TABLE
-- ============================================
-- Each import operation creates a batch record for audit trail

CREATE TABLE IF NOT EXISTS license_import_batches (
    id BIGSERIAL PRIMARY KEY,
    batch_id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),

    -- Source identity (who performed the import)
    imported_by VARCHAR(255) NOT NULL,           -- Admin user ID or service account
    source_system VARCHAR(100) NOT NULL,         -- 'admin_portal', 'api', 'sync_worker'
    source_ip INET,                              -- IP address of importer

    -- Provider information
    provider_id VARCHAR(100) NOT NULL,           -- External provider identifier
    provider_name VARCHAR(255),                  -- Human-readable provider name
    provider_reference VARCHAR(255),             -- External reference/order ID

    -- Import configuration
    import_mode VARCHAR(20) NOT NULL DEFAULT 'execute',  -- 'validate_only', 'execute'
    has_header BOOLEAN NOT NULL DEFAULT TRUE,
    contract_version VARCHAR(20) NOT NULL DEFAULT '1.0',

    -- Original file retention
    original_filename VARCHAR(255),
    original_content_hash VARCHAR(64) NOT NULL,  -- SHA256 of original content
    original_row_count INT NOT NULL,
    original_stored_path VARCHAR(500),           -- Path to stored original (restricted access)

    -- Results summary
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    total_rows INT NOT NULL DEFAULT 0,
    accepted_count INT NOT NULL DEFAULT 0,
    rejected_count INT NOT NULL DEFAULT 0,
    unchanged_count INT NOT NULL DEFAULT 0,      -- Already existed, no update needed
    quarantined_count INT NOT NULL DEFAULT 0,    -- Accepted but flagged for review

    -- Timing
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    processing_time_ms INT,

    -- Error summary
    error_message TEXT,

    CONSTRAINT valid_import_mode CHECK (
        import_mode IN ('validate_only', 'execute')
    ),
    CONSTRAINT valid_import_status CHECK (
        status IN ('pending', 'validating', 'executing', 'completed', 'failed', 'cancelled')
    ),
    CONSTRAINT valid_row_counts CHECK (
        accepted_count + rejected_count + unchanged_count + quarantined_count <= total_rows
    )
);

-- Index for finding batches by importer
CREATE INDEX IF NOT EXISTS idx_import_batches_importer
ON license_import_batches (imported_by, started_at DESC);

-- Index for finding batches by provider
CREATE INDEX IF NOT EXISTS idx_import_batches_provider
ON license_import_batches (provider_id, started_at DESC);

-- Index for finding recent batches
CREATE INDEX IF NOT EXISTS idx_import_batches_recent
ON license_import_batches (started_at DESC);

-- ============================================
-- IMPORT ROW RESULTS TABLE
-- ============================================
-- Per-row validation and import results

CREATE TABLE IF NOT EXISTS license_import_rows (
    id BIGSERIAL PRIMARY KEY,
    batch_id UUID NOT NULL REFERENCES license_import_batches(batch_id) ON DELETE CASCADE,

    -- Row identification
    row_number INT NOT NULL,
    original_row_data JSONB NOT NULL,            -- Original CSV row as JSON

    -- Validation result
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    license_id VARCHAR(66),                      -- If created/matched
    lease_code VARCHAR(64),                      -- Parsed lease code

    -- Parsed fields (for audit)
    parsed_valid_from TIMESTAMPTZ,
    parsed_valid_to TIMESTAMPTZ,
    parsed_split_type VARCHAR(10),

    -- Agreement provenance
    agreement_version INT,
    agreement_snapshot JSONB,                    -- Frozen agreement at import time

    -- Validation details
    validation_errors JSONB,                     -- Array of validation error messages
    validation_warnings JSONB,                   -- Array of warnings (non-fatal)

    -- Action taken
    action VARCHAR(20),                          -- 'created', 'updated', 'skipped', 'quarantined', 'rejected'
    action_reason TEXT,

    -- Timing
    processed_at TIMESTAMPTZ,

    CONSTRAINT valid_row_status CHECK (
        status IN ('pending', 'valid', 'invalid', 'quarantined')
    ),
    CONSTRAINT valid_row_action CHECK (
        action IS NULL OR action IN ('created', 'updated', 'skipped', 'quarantined', 'rejected')
    )
);

-- Index for finding rows by batch
CREATE INDEX IF NOT EXISTS idx_import_rows_batch
ON license_import_rows (batch_id, row_number);

-- Index for finding failed rows
CREATE INDEX IF NOT EXISTS idx_import_rows_invalid
ON license_import_rows (batch_id, status)
WHERE status IN ('invalid', 'quarantined');

-- ============================================
-- LICENSE PROVENANCE TRACKING
-- ============================================
-- Track the origin of each license for audit

ALTER TABLE licenses ADD COLUMN IF NOT EXISTS import_batch_id UUID;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS import_row_number INT;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS provider_id VARCHAR(100);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS provider_reference VARCHAR(255);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS source_credential VARCHAR(255);  -- External credential reference
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS agreement_version_at_import INT;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS quarantine_reason TEXT;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS is_quarantined BOOLEAN NOT NULL DEFAULT FALSE;

-- Index for finding licenses by import batch
CREATE INDEX IF NOT EXISTS idx_licenses_import_batch
ON licenses (import_batch_id)
WHERE import_batch_id IS NOT NULL;

-- Index for finding quarantined licenses
CREATE INDEX IF NOT EXISTS idx_licenses_quarantined
ON licenses (is_quarantined)
WHERE is_quarantined = TRUE;

-- ============================================
-- QUARANTINE REVIEW TABLE
-- ============================================
-- Track quarantine review decisions

CREATE TABLE IF NOT EXISTS license_quarantine_reviews (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,
    reviewed_by VARCHAR(255) NOT NULL,
    action VARCHAR(20) NOT NULL,                 -- 'approve', 'reject', 'escalate'
    reason TEXT NOT NULL,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_review_action CHECK (
        action IN ('approve', 'reject', 'escalate')
    )
);

-- Index for review history
CREATE INDEX IF NOT EXISTS idx_quarantine_reviews_license
ON license_quarantine_reviews (license_id, reviewed_at DESC);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE license_import_batches IS 'Import batch records with provenance tracking';
COMMENT ON COLUMN license_import_batches.original_content_hash IS 'SHA256 hash for detecting duplicate imports';
COMMENT ON COLUMN license_import_batches.contract_version IS 'Import contract version for backwards compatibility';
COMMENT ON COLUMN license_import_batches.quarantined_count IS 'Licenses accepted but flagged for manual review';

COMMENT ON TABLE license_import_rows IS 'Per-row validation and import results';
COMMENT ON COLUMN license_import_rows.agreement_snapshot IS 'Immutable snapshot of agreement at import time';

COMMENT ON COLUMN licenses.import_batch_id IS 'Reference to import batch for provenance';
COMMENT ON COLUMN licenses.source_credential IS 'External credential reference from provider';
COMMENT ON COLUMN licenses.is_quarantined IS 'License requires manual review before use';

COMMENT ON TABLE license_quarantine_reviews IS 'Audit trail for quarantine resolution decisions';
