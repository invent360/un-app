-- Rollback Phase 6: Testimonial Consent and FAQ Unification

-- Drop audit index for media
DROP INDEX IF EXISTS idx_audit_logs_media;

-- Drop disk threshold alerts
DROP INDEX IF EXISTS idx_disk_alerts_type;
DROP INDEX IF EXISTS idx_disk_alerts_active;
DROP TABLE IF EXISTS disk_threshold_alerts;

-- Remove FAQ unification columns
DROP INDEX IF EXISTS idx_faq_items_content_item;
ALTER TABLE faq_items
DROP COLUMN IF EXISTS synced_at,
DROP COLUMN IF EXISTS content_item_id;

-- Remove testimonial consent columns
DROP INDEX IF EXISTS idx_testimonials_consent_withdrawn;
DROP INDEX IF EXISTS idx_testimonials_consent_version;
DROP INDEX IF EXISTS idx_testimonials_consent;

ALTER TABLE testimonials
DROP COLUMN IF EXISTS avatar_asset_id,
DROP COLUMN IF EXISTS external_id,
DROP COLUMN IF EXISTS verified_by,
DROP COLUMN IF EXISTS verified_at,
DROP COLUMN IF EXISTS source_platform,
DROP COLUMN IF EXISTS consent_version,
DROP COLUMN IF EXISTS consent_withdrawn_at,
DROP COLUMN IF EXISTS consent_proof_url,
DROP COLUMN IF EXISTS consent_ip,
DROP COLUMN IF EXISTS consent_source,
DROP COLUMN IF EXISTS consent_given_at,
DROP COLUMN IF EXISTS consent_given;

