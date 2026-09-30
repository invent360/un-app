-- Phase 6: Media Backup Metadata (P6-08)
--
-- Tracks backup jobs, manifests, and restore operations for media assets.
-- Supports encrypted backups with hash verification and RPO/RTO demonstration.

-- =============================================================================
-- Part 1: Backup Job Status
-- =============================================================================

CREATE TYPE backup_status AS ENUM (
    'pending',      -- Scheduled but not started
    'running',      -- Currently in progress
    'completed',    -- Successfully finished
    'failed',       -- Failed with error
    'cancelled'     -- Manually cancelled
);

CREATE TYPE backup_type AS ENUM (
    'full',         -- Complete backup of all assets
    'incremental',  -- Only changed files since last backup
    'differential'  -- Changes since last full backup
);

-- =============================================================================
-- Part 2: Backup Jobs
-- =============================================================================

CREATE TABLE IF NOT EXISTS media_backups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Backup identification
    backup_name VARCHAR(255) NOT NULL,
    backup_type backup_type NOT NULL DEFAULT 'full',

    -- Source and destination
    source_path VARCHAR(512) NOT NULL,
    destination_path VARCHAR(512) NOT NULL,
    encryption_key_id VARCHAR(128),           -- Reference to encryption key

    -- Status tracking
    status backup_status NOT NULL DEFAULT 'pending',
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Statistics
    total_files INT NOT NULL DEFAULT 0,
    processed_files INT NOT NULL DEFAULT 0,
    total_bytes BIGINT NOT NULL DEFAULT 0,
    processed_bytes BIGINT NOT NULL DEFAULT 0,
    skipped_files INT NOT NULL DEFAULT 0,

    -- Integrity
    manifest_hash CHAR(64),                   -- SHA-256 of manifest
    archive_hash CHAR(64),                    -- SHA-256 of backup archive

    -- Base backup for incremental/differential
    base_backup_id UUID REFERENCES media_backups(id) ON DELETE SET NULL,

    -- RPO/RTO tracking
    rpo_seconds INT,                          -- Recovery Point Objective achieved
    estimated_rto_seconds INT,                -- Estimated Recovery Time Objective

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    -- Constraints
    CONSTRAINT valid_progress CHECK (
        processed_files <= total_files AND processed_bytes <= total_bytes
    )
);

-- Indexes for backup queries
CREATE INDEX idx_media_backups_status ON media_backups(status, created_at DESC);
CREATE INDEX idx_media_backups_type ON media_backups(backup_type, created_at DESC);
CREATE INDEX idx_media_backups_completed ON media_backups(completed_at DESC)
WHERE status = 'completed';

-- =============================================================================
-- Part 3: Backup Manifest Entries
-- =============================================================================

CREATE TABLE IF NOT EXISTS media_backup_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_id UUID NOT NULL REFERENCES media_backups(id) ON DELETE CASCADE,

    -- File identification
    asset_id UUID NOT NULL REFERENCES media_assets(id) ON DELETE CASCADE,
    relative_path VARCHAR(512) NOT NULL,      -- Path within backup

    -- File metadata at backup time
    file_size BIGINT NOT NULL,
    sha256_hash CHAR(64) NOT NULL,
    mime_type VARCHAR(100),

    -- Backup details
    compressed_size BIGINT,                   -- Size after compression
    encrypted BOOLEAN NOT NULL DEFAULT FALSE,

    -- Verification
    verified_at TIMESTAMPTZ,
    verification_status VARCHAR(20),          -- 'valid', 'corrupted', 'missing'

    -- Timestamps
    backed_up_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT unique_backup_asset UNIQUE (backup_id, asset_id)
);

-- Index for manifest lookups
CREATE INDEX idx_backup_entries_asset ON media_backup_entries(asset_id);
CREATE INDEX idx_backup_entries_hash ON media_backup_entries(sha256_hash);

-- =============================================================================
-- Part 4: Restore Operations
-- =============================================================================

CREATE TYPE restore_status AS ENUM (
    'pending',
    'running',
    'verifying',    -- Verifying restored files
    'completed',
    'failed',
    'partial'       -- Some files restored, some failed
);

CREATE TABLE IF NOT EXISTS media_restores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backup_id UUID NOT NULL REFERENCES media_backups(id) ON DELETE CASCADE,

    -- Restore details
    restore_path VARCHAR(512) NOT NULL,
    restore_mode VARCHAR(20) NOT NULL DEFAULT 'selective',  -- 'full', 'selective'

    -- Status
    status restore_status NOT NULL DEFAULT 'pending',
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Progress
    total_files INT NOT NULL DEFAULT 0,
    restored_files INT NOT NULL DEFAULT 0,
    failed_files INT NOT NULL DEFAULT 0,
    total_bytes BIGINT NOT NULL DEFAULT 0,
    restored_bytes BIGINT NOT NULL DEFAULT 0,

    -- Verification
    verification_errors TEXT[],               -- Array of verification failures

    -- RTO tracking (actual recovery time)
    rto_seconds INT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255)
);

-- Index for restore queries
CREATE INDEX idx_media_restores_status ON media_restores(status, created_at DESC);
CREATE INDEX idx_media_restores_backup ON media_restores(backup_id);

-- =============================================================================
-- Part 5: Restore Entry Tracking
-- =============================================================================

CREATE TABLE IF NOT EXISTS media_restore_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    restore_id UUID NOT NULL REFERENCES media_restores(id) ON DELETE CASCADE,
    backup_entry_id UUID NOT NULL REFERENCES media_backup_entries(id) ON DELETE CASCADE,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- 'pending', 'restored', 'verified', 'failed'
    error_message TEXT,

    -- Verification
    restored_hash CHAR(64),                   -- Hash of restored file
    hash_match BOOLEAN,                       -- Does restored hash match backup hash?

    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    CONSTRAINT unique_restore_entry UNIQUE (restore_id, backup_entry_id)
);

CREATE INDEX idx_restore_entries_status ON media_restore_entries(status);

-- =============================================================================
-- Part 6: Storage Health Metrics
-- =============================================================================

-- Track storage health over time for capacity planning
CREATE TABLE IF NOT EXISTS storage_health_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Mount identification
    mount_path VARCHAR(512) NOT NULL,
    volume_label VARCHAR(100),

    -- Capacity metrics
    total_bytes BIGINT NOT NULL,
    used_bytes BIGINT NOT NULL,
    available_bytes BIGINT NOT NULL,
    inode_total BIGINT,
    inode_used BIGINT,

    -- Health indicators
    read_latency_ms INT,
    write_latency_ms INT,
    is_healthy BOOLEAN NOT NULL DEFAULT TRUE,
    health_notes TEXT,

    -- Timestamp
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for time-series queries
CREATE INDEX idx_storage_metrics_mount_time
ON storage_health_metrics(mount_path, recorded_at DESC);

-- Partial index for unhealthy mounts
CREATE INDEX idx_storage_metrics_unhealthy
ON storage_health_metrics(mount_path, recorded_at DESC)
WHERE is_healthy = FALSE;

