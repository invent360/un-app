-- Phase 6: CMS Immutable Versions (P6-06)
--
-- Adds immutable version snapshots, optimistic concurrency control,
-- preview token revocation, and per-locale review workflow.

-- =============================================================================
-- Part 1: Immutable Version Snapshots
-- =============================================================================

-- Add immutability columns to content_item_versions
ALTER TABLE content_item_versions
ADD COLUMN IF NOT EXISTS is_immutable BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS snapshot_hash CHAR(64);  -- SHA-256 of version data

-- Index for finding published versions
CREATE INDEX IF NOT EXISTS idx_content_item_versions_published
ON content_item_versions(content_id, published_at DESC)
WHERE published_at IS NOT NULL;

-- Trigger to prevent modification of immutable versions
CREATE OR REPLACE FUNCTION prevent_immutable_version_update()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.is_immutable = TRUE THEN
        RAISE EXCEPTION 'Cannot modify immutable version (id: %, version: %)',
            OLD.id, OLD.version_number;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_prevent_immutable_update
    BEFORE UPDATE ON content_item_versions
    FOR EACH ROW
    EXECUTE FUNCTION prevent_immutable_version_update();

-- Trigger to prevent deletion of immutable versions
CREATE OR REPLACE FUNCTION prevent_immutable_version_delete()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.is_immutable = TRUE THEN
        RAISE EXCEPTION 'Cannot delete immutable version (id: %, version: %)',
            OLD.id, OLD.version_number;
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_prevent_immutable_delete
    BEFORE DELETE ON content_item_versions
    FOR EACH ROW
    EXECUTE FUNCTION prevent_immutable_version_delete();

-- =============================================================================
-- Part 2: Optimistic Concurrency Control
-- =============================================================================

-- Add concurrency control columns to content_items
ALTER TABLE content_items
ADD COLUMN IF NOT EXISTS etag VARCHAR(64),           -- Hash for optimistic concurrency
ADD COLUMN IF NOT EXISTS lock_holder VARCHAR(255),   -- User/session holding edit lock
ADD COLUMN IF NOT EXISTS lock_expires_at TIMESTAMPTZ;

-- Index for finding expired locks
CREATE INDEX IF NOT EXISTS idx_content_items_lock_expires
ON content_items(lock_expires_at)
WHERE lock_holder IS NOT NULL;

-- Function to generate etag from content
CREATE OR REPLACE FUNCTION generate_content_etag()
RETURNS TRIGGER AS $$
BEGIN
    -- Generate etag from version number and updated_at
    NEW.etag = encode(
        sha256(
            (COALESCE(NEW.current_version, 0)::TEXT ||
             COALESCE(NEW.updated_at::TEXT, NOW()::TEXT))::BYTEA
        ),
        'hex'
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_content_items_etag
    BEFORE INSERT OR UPDATE ON content_items
    FOR EACH ROW
    EXECUTE FUNCTION generate_content_etag();

-- =============================================================================
-- Part 3: Preview Token Revocation
-- =============================================================================

-- Add revocation columns to preview_tokens
ALTER TABLE preview_tokens
ADD COLUMN IF NOT EXISTS revoked_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS revoked_by VARCHAR(255),
ADD COLUMN IF NOT EXISTS revoke_reason TEXT;

-- Index for finding active (non-revoked) tokens
CREATE INDEX IF NOT EXISTS idx_preview_tokens_active
ON preview_tokens(token)
WHERE revoked_at IS NULL;

-- Index for finding revoked tokens by version
CREATE INDEX IF NOT EXISTS idx_preview_tokens_version_revoked
ON preview_tokens(version_id, revoked_at)
WHERE revoked_at IS NOT NULL;

-- =============================================================================
-- Part 4: Per-Locale Review Workflow
-- =============================================================================

-- Review status for locale-specific translations
CREATE TYPE locale_review_status AS ENUM (
    'pending',           -- Awaiting review
    'in_review',         -- Currently being reviewed
    'approved',          -- Translation approved
    'changes_requested', -- Changes needed
    'rejected'           -- Translation rejected
);

-- Per-locale review tracking
CREATE TABLE IF NOT EXISTS content_locale_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Reference to version and locale
    version_id UUID NOT NULL REFERENCES content_item_versions(id) ON DELETE CASCADE,
    locale VARCHAR(10) NOT NULL,  -- e.g., 'ar', 'bn', 'es'

    -- Review status
    status locale_review_status NOT NULL DEFAULT 'pending',

    -- Review details
    reviewer_id VARCHAR(255),
    review_notes TEXT,
    reviewed_at TIMESTAMPTZ,

    -- Translation quality metrics
    translation_score INT CHECK (translation_score >= 0 AND translation_score <= 100),

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- One review per version per locale
    CONSTRAINT unique_version_locale_review UNIQUE (version_id, locale)
);

-- Indexes for locale reviews
CREATE INDEX idx_content_locale_reviews_status
ON content_locale_reviews(status);

CREATE INDEX idx_content_locale_reviews_locale
ON content_locale_reviews(locale, status);

CREATE INDEX idx_content_locale_reviews_reviewer
ON content_locale_reviews(reviewer_id)
WHERE reviewer_id IS NOT NULL;

CREATE INDEX idx_content_locale_reviews_pending
ON content_locale_reviews(locale, created_at)
WHERE status = 'pending';

-- Trigger to update timestamp
CREATE OR REPLACE FUNCTION update_locale_review_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_locale_review_updated
    BEFORE UPDATE ON content_locale_reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_locale_review_timestamp();

-- =============================================================================
-- Part 5: Cache Invalidation Tracking
-- =============================================================================

-- Track cache invalidations for CDN/edge coordination
CREATE TABLE IF NOT EXISTS cache_invalidations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- What was invalidated
    resource_type VARCHAR(50) NOT NULL,  -- 'content_item', 'media_asset', 'faq', etc.
    resource_id VARCHAR(255) NOT NULL,   -- UUID or identifier
    cache_key VARCHAR(512),              -- Optional specific cache key

    -- Invalidation scope
    locale VARCHAR(10),                  -- NULL = all locales
    scope VARCHAR(50) NOT NULL DEFAULT 'full',  -- 'full', 'metadata', 'content'

    -- Processing status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    processed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    -- Prevent duplicates within short window
    CONSTRAINT unique_recent_invalidation UNIQUE (resource_type, resource_id, locale, scope)
);

-- Index for processing pending invalidations
CREATE INDEX idx_cache_invalidations_pending
ON cache_invalidations(created_at)
WHERE status = 'pending';

-- Index for resource lookup
CREATE INDEX idx_cache_invalidations_resource
ON cache_invalidations(resource_type, resource_id);

