-- Rollback Phase 6: CMS Immutable Versions

-- Drop cache invalidations
DROP TABLE IF EXISTS cache_invalidations;

-- Drop locale reviews
DROP TRIGGER IF EXISTS trg_locale_review_updated ON content_locale_reviews;
DROP FUNCTION IF EXISTS update_locale_review_timestamp();
DROP TABLE IF EXISTS content_locale_reviews;
DROP TYPE IF EXISTS locale_review_status;

-- Remove preview token revocation columns
ALTER TABLE preview_tokens
DROP COLUMN IF EXISTS revoke_reason,
DROP COLUMN IF EXISTS revoked_by,
DROP COLUMN IF EXISTS revoked_at;

DROP INDEX IF EXISTS idx_preview_tokens_version_revoked;
DROP INDEX IF EXISTS idx_preview_tokens_active;

-- Remove concurrency control from content_items
DROP TRIGGER IF EXISTS trg_content_items_etag ON content_items;
DROP FUNCTION IF EXISTS generate_content_etag();

ALTER TABLE content_items
DROP COLUMN IF EXISTS lock_expires_at,
DROP COLUMN IF EXISTS lock_holder,
DROP COLUMN IF EXISTS etag;

DROP INDEX IF EXISTS idx_content_items_lock_expires;

-- Remove immutable version protections
DROP TRIGGER IF EXISTS trg_prevent_immutable_delete ON content_item_versions;
DROP TRIGGER IF EXISTS trg_prevent_immutable_update ON content_item_versions;
DROP FUNCTION IF EXISTS prevent_immutable_version_delete();
DROP FUNCTION IF EXISTS prevent_immutable_version_update();

DROP INDEX IF EXISTS idx_content_item_versions_published;

ALTER TABLE content_item_versions
DROP COLUMN IF EXISTS snapshot_hash,
DROP COLUMN IF EXISTS published_at,
DROP COLUMN IF EXISTS is_immutable;

