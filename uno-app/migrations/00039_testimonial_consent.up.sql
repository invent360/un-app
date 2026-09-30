-- Phase 6: Testimonial Consent and FAQ Unification (P6-06)
--
-- Adds GDPR-compliant consent tracking to testimonials and
-- unifies FAQ items with the CMS content system.

-- =============================================================================
-- Part 1: Testimonial Consent Tracking
-- =============================================================================

-- Add consent columns to testimonials
ALTER TABLE testimonials
ADD COLUMN IF NOT EXISTS consent_given BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS consent_given_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS consent_source VARCHAR(50),           -- 'form', 'email', 'verbal', 'import'
ADD COLUMN IF NOT EXISTS consent_ip INET,                      -- IP address at consent time
ADD COLUMN IF NOT EXISTS consent_proof_url VARCHAR(512),       -- Link to proof document
ADD COLUMN IF NOT EXISTS consent_withdrawn_at TIMESTAMPTZ,     -- When consent was withdrawn
ADD COLUMN IF NOT EXISTS consent_version VARCHAR(20);          -- Version of consent form used

-- Add source/verification columns
ALTER TABLE testimonials
ADD COLUMN IF NOT EXISTS source_platform VARCHAR(50),          -- 'app', 'email', 'social', 'interview'
ADD COLUMN IF NOT EXISTS verified_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS verified_by VARCHAR(255),
ADD COLUMN IF NOT EXISTS external_id VARCHAR(255);             -- ID from external system

-- Add link to media asset for avatar
ALTER TABLE testimonials
ADD COLUMN IF NOT EXISTS avatar_asset_id UUID REFERENCES media_assets(id) ON DELETE SET NULL;

-- Index for consent compliance queries
CREATE INDEX IF NOT EXISTS idx_testimonials_consent
ON testimonials(consent_given, consent_given_at);

-- Index for finding testimonials needing consent re-verification
CREATE INDEX IF NOT EXISTS idx_testimonials_consent_version
ON testimonials(consent_version)
WHERE consent_given = TRUE;

-- Index for withdrawn consent
CREATE INDEX IF NOT EXISTS idx_testimonials_consent_withdrawn
ON testimonials(consent_withdrawn_at)
WHERE consent_withdrawn_at IS NOT NULL;

-- =============================================================================
-- Part 2: FAQ Unification with CMS
-- =============================================================================

-- Add optional link to CMS content items
-- Allows FAQs to be managed through the full CMS workflow
ALTER TABLE faq_items
ADD COLUMN IF NOT EXISTS content_item_id UUID REFERENCES content_items(id) ON DELETE SET NULL,
ADD COLUMN IF NOT EXISTS synced_at TIMESTAMPTZ;  -- Last sync from CMS

-- Index for FAQ-CMS integration
CREATE INDEX IF NOT EXISTS idx_faq_items_content_item
ON faq_items(content_item_id)
WHERE content_item_id IS NOT NULL;

-- =============================================================================
-- Part 3: Disk Threshold Alerts
-- =============================================================================

-- Track disk space warnings for media storage (P6-08)
CREATE TABLE IF NOT EXISTS disk_threshold_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Alert details
    mount_path VARCHAR(512) NOT NULL,
    threshold_percent INT NOT NULL,       -- e.g., 80, 90, 95
    current_percent INT NOT NULL,
    free_bytes BIGINT NOT NULL,
    total_bytes BIGINT NOT NULL,

    -- Alert state
    alert_type VARCHAR(20) NOT NULL,      -- 'warning', 'critical', 'recovered'
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by VARCHAR(255),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT valid_threshold CHECK (
        threshold_percent > 0 AND threshold_percent <= 100
    ),
    CONSTRAINT valid_percent CHECK (
        current_percent >= 0 AND current_percent <= 100
    )
);

-- Index for active alerts
CREATE INDEX IF NOT EXISTS idx_disk_alerts_active
ON disk_threshold_alerts(mount_path, created_at DESC)
WHERE resolved_at IS NULL;

-- Index for alert type
CREATE INDEX IF NOT EXISTS idx_disk_alerts_type
ON disk_threshold_alerts(alert_type, created_at DESC);

-- =============================================================================
-- Part 4: Audit Logging Enhancement for Media
-- =============================================================================

-- Add media-specific audit event types if needed
-- This uses the existing audit_logs table with category filtering

-- Index for media audit queries (if not already exists)
CREATE INDEX IF NOT EXISTS idx_audit_logs_media
ON audit_logs(resource_type, event_type, created_at DESC)
WHERE resource_type IN ('media_asset', 'media_backup', 'media_quota');

