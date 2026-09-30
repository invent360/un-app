-- Rollback Phase 6: Media Assets

DROP TRIGGER IF EXISTS trg_media_asset_quota_delete ON media_assets;
DROP TRIGGER IF EXISTS trg_media_asset_quota_insert ON media_assets;
DROP TRIGGER IF EXISTS trg_media_asset_state_changed ON media_assets;
DROP TRIGGER IF EXISTS trg_media_asset_updated ON media_assets;

DROP FUNCTION IF EXISTS update_quota_on_delete();
DROP FUNCTION IF EXISTS update_quota_on_insert();
DROP FUNCTION IF EXISTS update_media_asset_state_timestamp();
DROP FUNCTION IF EXISTS update_media_asset_timestamp();

DROP TABLE IF EXISTS media_asset_versions;
DROP TABLE IF EXISTS media_quotas;
DROP TABLE IF EXISTS media_asset_grants;
DROP TABLE IF EXISTS media_assets;

DROP TYPE IF EXISTS asset_state;
