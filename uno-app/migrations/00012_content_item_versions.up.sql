-- Content Item Versions table for schema-driven CMS
-- Tracks version history for content items

CREATE TABLE IF NOT EXISTS content_item_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    version_number INT NOT NULL,
    data JSONB NOT NULL DEFAULT '{}',
    translations JSONB,
    change_summary TEXT,
    created_by VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(content_id, version_number)
);

CREATE INDEX IF NOT EXISTS idx_content_item_versions_content_id ON content_item_versions(content_id);
CREATE INDEX IF NOT EXISTS idx_content_item_versions_created_at ON content_item_versions(created_at DESC);
