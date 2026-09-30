-- R4-05: Rollback media visibility and integrity

-- Drop view
DROP VIEW IF EXISTS accessible_media_assets;

-- Drop functions
DROP FUNCTION IF EXISTS update_media_visibility(UUID, asset_visibility, VARCHAR);
DROP FUNCTION IF EXISTS verify_media_integrity(UUID, CHAR);
DROP FUNCTION IF EXISTS check_media_visibility(UUID, VARCHAR, BOOLEAN);

-- Drop indexes
DROP INDEX IF EXISTS idx_media_assets_owner_visibility;
DROP INDEX IF EXISTS idx_media_assets_visibility;

-- Remove columns
ALTER TABLE media_assets
DROP COLUMN IF EXISTS original_hash,
DROP COLUMN IF EXISTS visibility;

-- Drop enum type
DROP TYPE IF EXISTS asset_visibility;
