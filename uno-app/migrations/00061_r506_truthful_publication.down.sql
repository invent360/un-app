-- Down migration for 00061_r506_truthful_publication

-- Drop triggers
DROP TRIGGER IF EXISTS sync_cursors_updated ON sync_cursors;
DROP TRIGGER IF EXISTS publication_quarantine_updated ON publication_quarantine;
DROP FUNCTION IF EXISTS update_quarantine_timestamp();

-- Drop functions
DROP FUNCTION IF EXISTS fetch_licenses_for_sync(TIMESTAMPTZ, VARCHAR(100), INT);
DROP FUNCTION IF EXISTS quarantine_license(VARCHAR(100), VARCHAR(50), VARCHAR(50), TEXT, JSONB, JSONB);
DROP FUNCTION IF EXISTS update_sync_cursor(VARCHAR(50), TIMESTAMPTZ, VARCHAR(100), BIGINT, INT);
DROP FUNCTION IF EXISTS get_sync_cursor(VARCHAR(50));

-- Drop publication item columns
ALTER TABLE license_publication_items DROP COLUMN IF EXISTS quarantine_id;
ALTER TABLE license_publication_items DROP COLUMN IF EXISTS input_license_id;
ALTER TABLE license_publication_items DROP COLUMN IF EXISTS input_index;

-- Drop tables
DROP TABLE IF EXISTS sync_cursors;
DROP TABLE IF EXISTS publication_quarantine;

-- Drop license source columns
ALTER TABLE licenses DROP COLUMN IF EXISTS source_record_id;
ALTER TABLE licenses DROP COLUMN IF EXISTS source_imported_at;
ALTER TABLE licenses DROP COLUMN IF EXISTS source_version;
ALTER TABLE licenses DROP COLUMN IF EXISTS source_system;

-- Drop exact share columns
ALTER TABLE licenses DROP COLUMN IF EXISTS agent_share_pct;
ALTER TABLE licenses DROP COLUMN IF EXISTS ulo_share_pct;
ALTER TABLE licenses DROP COLUMN IF EXISTS uno_share_pct;
