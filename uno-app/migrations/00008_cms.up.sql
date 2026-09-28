-- CMS (Content Management System) Tables
-- Schema-driven content management with UUID IDs, translations, and versioning

-- ============================================
-- ENUM TYPES
-- ============================================

DO $$ BEGIN
    CREATE TYPE content_status AS ENUM (
        'draft',
        'pending_review',
        'approved',
        'published',
        'scheduled',
        'archived'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- CONTENT SCHEMAS TABLE
-- ============================================
-- Defines content types (task, guide, faq, etc.) with their field definitions

CREATE TABLE IF NOT EXISTS content_schemas (
    id VARCHAR(50) PRIMARY KEY,                    -- Slug used as ID (e.g., 'task', 'guide')
    name VARCHAR(100) NOT NULL,                    -- Singular display name
    name_plural VARCHAR(100) NOT NULL,             -- Plural display name
    description TEXT,                               -- Schema description
    icon VARCHAR(50),                               -- Icon identifier
    fields JSONB NOT NULL DEFAULT '[]',            -- Field definitions array
    settings JSONB NOT NULL DEFAULT '{}',          -- Schema settings (has_slug, title_field, etc.)
    version INT NOT NULL DEFAULT 1,                -- Schema version
    is_system BOOLEAN DEFAULT false,               -- System schema (not user-deletable)
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- CONTENT ITEMS TABLE
-- ============================================
-- Stores actual content with translations and versioning

CREATE TABLE IF NOT EXISTS content_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_id VARCHAR(50) NOT NULL REFERENCES content_schemas(id) ON DELETE CASCADE,
    slug VARCHAR(255),
    display_order INT DEFAULT 0,
    status content_status NOT NULL DEFAULT 'draft',
    version INT NOT NULL DEFAULT 1,

    -- Content data
    data JSONB NOT NULL DEFAULT '{}',              -- Primary content in English
    translations JSONB DEFAULT '{}',               -- Translations by locale: {"es": {...}, "fr": {...}}
    translation_status JSONB DEFAULT '{}',         -- Status per locale: {"es": "complete", "fr": "partial"}

    -- Display flags
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,

    -- Publishing
    published_version INT,                          -- Which version is published
    published_at TIMESTAMPTZ,
    published_by VARCHAR(100),
    publish_at TIMESTAMPTZ,                         -- Scheduled publish time
    unpublish_at TIMESTAMPTZ,                       -- Scheduled unpublish time

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by VARCHAR(100),
    updated_by VARCHAR(100),

    CONSTRAINT unique_schema_slug UNIQUE (schema_id, slug)
);

-- ============================================
-- SCHEDULED PUBLISHES TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS scheduled_publishes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_item_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    scheduled_at TIMESTAMPTZ NOT NULL,
    action VARCHAR(20) NOT NULL DEFAULT 'publish',
    executed_at TIMESTAMPTZ,
    status VARCHAR(20) DEFAULT 'pending',
    created_by VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- CONTENT RELATIONS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS content_relations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_item_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    target_item_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    relation_type VARCHAR(50) NOT NULL DEFAULT 'related',
    sort_order INT DEFAULT 0,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_relation UNIQUE (source_item_id, target_item_id, relation_type),
    CONSTRAINT no_self_relation CHECK (source_item_id != target_item_id)
);

-- ============================================
-- TESTIMONIALS TABLE
-- ============================================
-- Testimonials for home page (injected into home sections)

CREATE TABLE IF NOT EXISTS testimonials (
    id SERIAL PRIMARY KEY,
    quote TEXT NOT NULL,
    author_name VARCHAR(100) NOT NULL,
    author_location VARCHAR(100),
    author_avatar VARCHAR(500),
    rating SMALLINT DEFAULT 5 CHECK (rating >= 1 AND rating <= 5),
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    display_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- FAQ ITEMS TABLE
-- ============================================
-- Legacy FAQ items table for direct FAQ queries

CREATE TABLE IF NOT EXISTS faq_items (
    id SERIAL PRIMARY KEY,
    category VARCHAR(50) NOT NULL,
    display_order INT DEFAULT 0,
    question_en TEXT NOT NULL,
    answer_en TEXT NOT NULL,
    translations JSONB DEFAULT '{}',
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- INDEXES
-- ============================================

-- Content schemas indexes
CREATE INDEX IF NOT EXISTS idx_content_schemas_system ON content_schemas(is_system);

-- Content items indexes
CREATE INDEX IF NOT EXISTS idx_content_items_schema ON content_items(schema_id);
CREATE INDEX IF NOT EXISTS idx_content_items_slug ON content_items(slug);
CREATE INDEX IF NOT EXISTS idx_content_items_status ON content_items(status);
CREATE INDEX IF NOT EXISTS idx_content_items_featured ON content_items(is_featured);
CREATE INDEX IF NOT EXISTS idx_content_items_active ON content_items(is_active);
CREATE INDEX IF NOT EXISTS idx_content_items_published ON content_items(published_at);
CREATE INDEX IF NOT EXISTS idx_content_items_display_order ON content_items(display_order);
CREATE INDEX IF NOT EXISTS idx_content_items_publish_at ON content_items(publish_at) WHERE publish_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_content_items_unpublish_at ON content_items(unpublish_at) WHERE unpublish_at IS NOT NULL;

-- Scheduled publishes indexes
CREATE INDEX IF NOT EXISTS idx_scheduled_publishes_item ON scheduled_publishes(content_item_id);
CREATE INDEX IF NOT EXISTS idx_scheduled_publishes_scheduled ON scheduled_publishes(scheduled_at);
CREATE INDEX IF NOT EXISTS idx_scheduled_publishes_status ON scheduled_publishes(status);

-- Content relations indexes
CREATE INDEX IF NOT EXISTS idx_content_relations_source ON content_relations(source_item_id);
CREATE INDEX IF NOT EXISTS idx_content_relations_target ON content_relations(target_item_id);
CREATE INDEX IF NOT EXISTS idx_content_relations_type ON content_relations(relation_type);

-- Testimonials indexes
CREATE INDEX IF NOT EXISTS idx_testimonials_active ON testimonials(is_active);
CREATE INDEX IF NOT EXISTS idx_testimonials_featured ON testimonials(is_featured);
CREATE INDEX IF NOT EXISTS idx_testimonials_order ON testimonials(display_order);

-- FAQ items indexes
CREATE INDEX IF NOT EXISTS idx_faq_items_category ON faq_items(category);
CREATE INDEX IF NOT EXISTS idx_faq_items_active ON faq_items(is_active);
CREATE INDEX IF NOT EXISTS idx_faq_items_featured ON faq_items(is_featured);
CREATE INDEX IF NOT EXISTS idx_faq_items_order ON faq_items(display_order);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE content_schemas IS 'Defines content types with their field structures and settings';
COMMENT ON TABLE content_items IS 'Schema-driven content with translations and versioning';
COMMENT ON TABLE content_relations IS 'Links content items together for related content';
COMMENT ON TABLE testimonials IS 'User testimonials for home page display';
