-- Migration 00018 Down: Remove CMS Review Workflow Tables

DROP INDEX IF EXISTS idx_preview_tokens_expires_at;
DROP INDEX IF EXISTS idx_content_reviews_submitted_at;
DROP INDEX IF EXISTS idx_content_reviews_submitted_by;
DROP INDEX IF EXISTS idx_content_reviews_status;
DROP INDEX IF EXISTS idx_content_versions_content_id;

DROP TABLE IF EXISTS preview_tokens;
DROP TABLE IF EXISTS content_reviews;
DROP TABLE IF EXISTS content_versions;

DROP TYPE IF EXISTS review_status;

-- Note: We don't remove the status column from page_contents
-- as it may contain data and other code may depend on it
