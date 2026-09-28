-- Page Contents Table
-- Stores page content with versioning and scheduling support

CREATE TABLE IF NOT EXISTS page_contents (
    id SERIAL PRIMARY KEY,
    content_type VARCHAR(100) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    content JSONB NOT NULL DEFAULT '{}',
    translations JSONB DEFAULT '{}',
    translation_status JSONB DEFAULT '{}',
    display_order INT NOT NULL DEFAULT 0,
    is_featured BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    version INT NOT NULL DEFAULT 1,
    published_version INT,
    published_at TIMESTAMPTZ,
    published_by VARCHAR(255),
    publish_at TIMESTAMPTZ,
    unpublish_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),
    updated_by VARCHAR(255),

    CONSTRAINT unique_content_type_slug UNIQUE (content_type, slug)
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_page_contents_content_type ON page_contents(content_type);
CREATE INDEX IF NOT EXISTS idx_page_contents_slug ON page_contents(slug);
CREATE INDEX IF NOT EXISTS idx_page_contents_status ON page_contents(status);
CREATE INDEX IF NOT EXISTS idx_page_contents_is_active ON page_contents(is_active);
CREATE INDEX IF NOT EXISTS idx_page_contents_is_featured ON page_contents(is_featured);
CREATE INDEX IF NOT EXISTS idx_page_contents_display_order ON page_contents(display_order);
CREATE INDEX IF NOT EXISTS idx_page_contents_publish_at ON page_contents(publish_at);
CREATE INDEX IF NOT EXISTS idx_page_contents_unpublish_at ON page_contents(unpublish_at);
