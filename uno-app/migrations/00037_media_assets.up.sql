-- Phase 6: Media Assets (P6-01, P6-02, P6-03)
--
-- Tracks media assets with state machine, ownership, quotas, and access grants.
-- Supports local persistent storage without cloud dependencies.

-- Asset state enum
CREATE TYPE asset_state AS ENUM (
    'uploading',   -- Upload in progress
    'ready',       -- Successfully uploaded and verified
    'quarantined', -- Failed validation, held for review
    'missing',     -- Expected but not found on disk
    'deleted'      -- Soft-deleted, pending cleanup
);

-- Media assets tracking table
CREATE TABLE IF NOT EXISTS media_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- File-storage integration
    resource_id VARCHAR(128) NOT NULL,      -- Matches file-storage resource_id pattern
    filename VARCHAR(255) NOT NULL,          -- UUID-based filename from file-storage
    original_filename VARCHAR(255),          -- User's original filename
    storage_url VARCHAR(512) NOT NULL UNIQUE, -- local://resource_id/filename
    display_url VARCHAR(512) NOT NULL,       -- Browser-accessible URL

    -- File metadata
    mime_type VARCHAR(100) NOT NULL,
    file_size BIGINT NOT NULL,
    sha256_hash CHAR(64),                    -- For integrity verification
    width INT,                                -- For images
    height INT,                               -- For images

    -- State tracking (P6-03)
    state asset_state NOT NULL DEFAULT 'uploading',
    state_reason TEXT,                        -- Why in current state
    state_changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ownership and access (P6-04)
    owner_id VARCHAR(255),                    -- User/entity that owns this asset
    owner_type VARCHAR(50),                   -- 'user', 'content_item', 'testimonial', 'license'

    -- Content categorization
    category VARCHAR(50),                     -- 'profile', 'document', 'media', 'cms'
    alt_text TEXT,                            -- Accessibility alt text

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    deleted_by VARCHAR(255),

    -- Constraints
    CONSTRAINT valid_resource_id CHECK (
        resource_id ~ '^[a-zA-Z0-9_-]{1,128}$'
    ),
    CONSTRAINT valid_file_size CHECK (file_size >= 0),
    CONSTRAINT valid_dimensions CHECK (
        (width IS NULL AND height IS NULL) OR
        (width > 0 AND height > 0)
    )
);

-- Media asset access grants for private assets (P6-04)
CREATE TABLE IF NOT EXISTS media_asset_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES media_assets(id) ON DELETE CASCADE,
    grantee_id VARCHAR(255) NOT NULL,         -- User/role/service receiving access
    grantee_type VARCHAR(50) NOT NULL,        -- 'user', 'role', 'service'
    permission VARCHAR(50) NOT NULL,           -- 'read', 'write', 'delete'
    expires_at TIMESTAMPTZ,                    -- NULL = no expiry
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    CONSTRAINT unique_grant UNIQUE (asset_id, grantee_id, grantee_type, permission),
    CONSTRAINT valid_permission CHECK (permission IN ('read', 'write', 'delete'))
);

-- Upload quotas per resource owner (P6-02)
CREATE TABLE IF NOT EXISTS media_quotas (
    owner_id VARCHAR(255) NOT NULL,
    owner_type VARCHAR(50) NOT NULL,

    -- Limits
    max_total_bytes BIGINT NOT NULL DEFAULT 104857600,   -- 100MB default
    max_file_size BIGINT NOT NULL DEFAULT 10485760,      -- 10MB default
    max_file_count INT NOT NULL DEFAULT 50,
    max_inodes INT NOT NULL DEFAULT 100,                 -- Max total files including versions

    -- Current usage
    current_bytes BIGINT NOT NULL DEFAULT 0,
    current_count INT NOT NULL DEFAULT 0,

    -- Timestamps
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (owner_id, owner_type),
    CONSTRAINT valid_limits CHECK (
        max_total_bytes > 0 AND max_file_size > 0 AND max_file_count > 0
    ),
    CONSTRAINT valid_usage CHECK (
        current_bytes >= 0 AND current_count >= 0
    )
);

-- Asset versions for tracking changes (P6-03)
CREATE TABLE IF NOT EXISTS media_asset_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES media_assets(id) ON DELETE CASCADE,
    version_number INT NOT NULL DEFAULT 1,

    -- Version-specific storage
    storage_url VARCHAR(512) NOT NULL,
    display_url VARCHAR(512) NOT NULL,

    -- Version metadata
    file_size BIGINT NOT NULL,
    sha256_hash CHAR(64) NOT NULL,

    -- Change tracking
    change_summary TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    CONSTRAINT unique_version UNIQUE (asset_id, version_number)
);

-- Indexes for performance
CREATE INDEX idx_media_assets_resource ON media_assets(resource_id);
CREATE INDEX idx_media_assets_owner ON media_assets(owner_id, owner_type) WHERE owner_id IS NOT NULL;
CREATE INDEX idx_media_assets_state ON media_assets(state);
CREATE INDEX idx_media_assets_category ON media_assets(category) WHERE category IS NOT NULL;
CREATE INDEX idx_media_assets_created ON media_assets(created_at DESC);
CREATE INDEX idx_media_assets_not_deleted ON media_assets(id) WHERE deleted_at IS NULL;

CREATE INDEX idx_media_asset_grants_grantee ON media_asset_grants(grantee_id, grantee_type);
CREATE INDEX idx_media_asset_grants_expires ON media_asset_grants(expires_at) WHERE expires_at IS NOT NULL;

CREATE INDEX idx_media_asset_versions_asset ON media_asset_versions(asset_id, version_number DESC);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_media_asset_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_media_asset_updated
    BEFORE UPDATE ON media_assets
    FOR EACH ROW
    EXECUTE FUNCTION update_media_asset_timestamp();

-- Trigger to update state_changed_at when state changes
CREATE OR REPLACE FUNCTION update_media_asset_state_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.state IS DISTINCT FROM NEW.state THEN
        NEW.state_changed_at = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_media_asset_state_changed
    BEFORE UPDATE ON media_assets
    FOR EACH ROW
    EXECUTE FUNCTION update_media_asset_state_timestamp();

-- Trigger to update quota usage on insert
CREATE OR REPLACE FUNCTION update_quota_on_insert()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.owner_id IS NOT NULL AND NEW.owner_type IS NOT NULL THEN
        INSERT INTO media_quotas (owner_id, owner_type, current_bytes, current_count, updated_at)
        VALUES (NEW.owner_id, NEW.owner_type, NEW.file_size, 1, NOW())
        ON CONFLICT (owner_id, owner_type) DO UPDATE SET
            current_bytes = media_quotas.current_bytes + NEW.file_size,
            current_count = media_quotas.current_count + 1,
            updated_at = NOW();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_media_asset_quota_insert
    AFTER INSERT ON media_assets
    FOR EACH ROW
    WHEN (NEW.state = 'ready')
    EXECUTE FUNCTION update_quota_on_insert();

-- Trigger to update quota usage on soft delete
CREATE OR REPLACE FUNCTION update_quota_on_delete()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.owner_id IS NOT NULL AND OLD.owner_type IS NOT NULL AND OLD.deleted_at IS NULL AND NEW.deleted_at IS NOT NULL THEN
        UPDATE media_quotas SET
            current_bytes = GREATEST(0, current_bytes - OLD.file_size),
            current_count = GREATEST(0, current_count - 1),
            updated_at = NOW()
        WHERE owner_id = OLD.owner_id AND owner_type = OLD.owner_type;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_media_asset_quota_delete
    AFTER UPDATE ON media_assets
    FOR EACH ROW
    EXECUTE FUNCTION update_quota_on_delete();
