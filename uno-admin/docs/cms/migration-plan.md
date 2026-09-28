# Schema-Driven CMS Migration Plan

> **Status:** In Progress
> **Target:** Full schema-driven content management
> **Risk Level:** Low (not in production)

---

## Overview

Migrate from hardcoded content types (TaskContent, GuideContent, etc.) to a generic schema-driven system where content types are defined via configuration.

---

## Current State

### Database
```
page_contents
├── id SERIAL
├── content_type VARCHAR(50)      # "task", "guide", "faq", "error"
├── slug VARCHAR(100)
├── status VARCHAR(20)
├── content JSONB                  # Type-specific structure
├── translations JSONB
├── translation_status JSONB
├── ... (versioning, timestamps)
```

### Code (uno-app)
```rust
// Hardcoded structs
pub struct TaskContent { title, description, earnings_estimate, ... }
pub struct GuideContent { title, steps, duration_minutes, ... }
pub struct ErrorContent { error_code, title, solution, ... }
```

### Problems
1. Adding new content type = code changes + deployment
2. No field-level validation configuration
3. No conditional field visibility
4. No schema-aware form rendering
5. Front page content type not supported

---

## Target State

### Database
```
content_schemas
├── id VARCHAR(50) PRIMARY KEY     # "task", "guide", "front"
├── name VARCHAR(100)              # "Task", "Guide", "Front Page"
├── name_plural VARCHAR(100)
├── description TEXT
├── icon VARCHAR(50)
├── fields JSONB                   # Array of FieldDefinition
├── settings JSONB                 # SchemaSettings
├── version INT
├── created_at, updated_at

content_items (replaces page_contents)
├── id UUID PRIMARY KEY
├── schema_id VARCHAR(50)          # FK to content_schemas
├── slug VARCHAR(255)
├── display_order INT
├── status content_status
├── version INT
├── data JSONB                     # Generic field values
├── created_at, updated_at, ...
```

### Code
```rust
// Generic - one system handles all
pub struct ContentSchema { id, fields: Vec<FieldDefinition>, settings }
pub struct ContentItem { schema_id, data: HashMap<String, FieldValue> }
pub enum FieldValue { Text(String), Number(f64), Repeater(Vec<...>), ... }
```

---

## Migration Phases

### Phase 1: Database Schema (uno-app)
**Duration:** 1 day

1. Create `content_schemas` table
2. Create new `content_items` table (parallel to page_contents)
3. Add field type enums
4. Create migration functions

### Phase 2: Core Types (uno-app)
**Duration:** 2 days

1. Define FieldType enum and all variants
2. Define FieldDefinition struct
3. Define ContentSchema struct
4. Define FieldValue enum
5. Define ContentItem struct
6. Implement validation logic

### Phase 3: Schema Definitions (uno-app)
**Duration:** 1 day

1. Define Task schema
2. Define Guide schema
3. Define FAQ schema
4. Define Error schema
5. Define Front Page schema (NEW)
6. Seed schemas to database

### Phase 4: Repository Layer (uno-app)
**Duration:** 2 days

1. Create SchemaRepository
2. Update ContentRepository for generic items
3. Implement validation against schema
4. Update version history logic

### Phase 5: API Layer (uno-app)
**Duration:** 2 days

1. Add schema endpoints (GET /schemas, GET /schemas/:id)
2. Update content endpoints for generic structure
3. Update validation in handlers
4. Maintain backward compatibility temporarily

### Phase 6: Data Migration (uno-app)
**Duration:** 1 day

1. Migrate existing page_contents → content_items
2. Verify data integrity
3. Update foreign keys

### Phase 7: Admin Frontend (uno-admin)
**Duration:** 3-4 days

1. Create FieldRenderer component
2. Create schema-driven ContentForm
3. Update editor to use dynamic forms
4. Update list view
5. Remove hardcoded form logic

### Phase 8: Cleanup
**Duration:** 1 day

1. Remove old page_contents table (after verification)
2. Remove hardcoded content type structs
3. Update documentation

---

## Detailed Implementation

### Step 1: Database Migration

```sql
-- File: migrations/20260601100001_schema_driven_cms.up.sql

-- Content status enum (if not exists)
DO $$ BEGIN
    CREATE TYPE content_status AS ENUM (
        'draft', 'pending_review', 'approved', 'published', 'scheduled', 'archived'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Field type enum
CREATE TYPE field_type AS ENUM (
    'text', 'rich_text', 'number', 'boolean', 'select', 'multi_select',
    'date', 'media', 'media_list', 'list', 'group', 'repeater',
    'reference', 'reference_list'
);

-- Content schemas table
CREATE TABLE content_schemas (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    name_plural VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(50),
    fields JSONB NOT NULL DEFAULT '[]',
    settings JSONB NOT NULL DEFAULT '{}',
    version INT NOT NULL DEFAULT 1,
    is_system BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Content items table (generic)
CREATE TABLE content_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_id VARCHAR(50) NOT NULL REFERENCES content_schemas(id),
    slug VARCHAR(255),
    display_order INT NOT NULL DEFAULT 0,
    status content_status NOT NULL DEFAULT 'draft',
    version INT NOT NULL DEFAULT 1,

    -- Generic data storage
    data JSONB NOT NULL DEFAULT '{}',

    -- Translations (same structure as data, per locale)
    translations JSONB DEFAULT '{}',
    translation_status JSONB DEFAULT '{}',

    -- Publishing
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    published_version INT,
    published_at TIMESTAMPTZ,
    published_by VARCHAR(100),

    -- Scheduling
    publish_at TIMESTAMPTZ,
    unpublish_at TIMESTAMPTZ,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100),
    updated_by VARCHAR(100),

    CONSTRAINT content_items_unique_slug UNIQUE (schema_id, slug)
);

-- Indexes
CREATE INDEX idx_content_items_schema ON content_items(schema_id);
CREATE INDEX idx_content_items_status ON content_items(status);
CREATE INDEX idx_content_items_schema_status ON content_items(schema_id, status) WHERE is_active = true;
CREATE INDEX idx_content_items_order ON content_items(schema_id, display_order);
CREATE INDEX idx_content_items_published ON content_items(published_at) WHERE status = 'published';
CREATE INDEX idx_content_items_scheduled ON content_items(publish_at) WHERE publish_at IS NOT NULL;

-- Full-text search
CREATE INDEX idx_content_items_data ON content_items USING GIN (data);
CREATE INDEX idx_content_items_translations ON content_items USING GIN (translations);

-- Content versions for new system
CREATE TABLE content_item_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    version_number INT NOT NULL,
    data JSONB NOT NULL,
    translations JSONB,
    change_summary TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100),

    UNIQUE(content_id, version_number)
);

CREATE INDEX idx_content_versions_content ON content_item_versions(content_id);

-- Update trigger for content_items
CREATE OR REPLACE FUNCTION update_content_items_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER content_items_updated_at
    BEFORE UPDATE ON content_items
    FOR EACH ROW
    EXECUTE FUNCTION update_content_items_timestamp();

-- Update trigger for content_schemas
CREATE TRIGGER content_schemas_updated_at
    BEFORE UPDATE ON content_schemas
    FOR EACH ROW
    EXECUTE FUNCTION update_content_items_timestamp();
```

### Step 2: Seed Schema Definitions

```sql
-- File: migrations/20260601100002_seed_schemas.up.sql

-- Task Schema
INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, is_system) VALUES (
    'task',
    'Task',
    'Tasks',
    'Earning tasks for users',
    'briefcase',
    '[
        {
            "key": "title",
            "label": "Title",
            "field_type": {"type": "text", "config": {"max_length": 100}},
            "required": true,
            "translatable": true,
            "show_in_list": true,
            "searchable": true
        },
        {
            "key": "description",
            "label": "Description",
            "field_type": {"type": "rich_text", "config": {}},
            "required": true,
            "translatable": true,
            "searchable": true
        },
        {
            "key": "image",
            "label": "Cover Image",
            "field_type": {"type": "media", "config": {"allowed_types": ["image"]}},
            "required": false
        },
        {
            "key": "task_status",
            "label": "Task Status",
            "field_type": {"type": "select", "config": {
                "options": [
                    {"value": "active", "label": "Active"},
                    {"value": "coming_soon", "label": "Coming Soon"},
                    {"value": "deprecated", "label": "Deprecated"}
                ],
                "default": "active"
            }},
            "required": true,
            "show_in_list": true
        },
        {
            "key": "difficulty",
            "label": "Difficulty",
            "field_type": {"type": "select", "config": {
                "options": [
                    {"value": "easy", "label": "Easy"},
                    {"value": "medium", "label": "Medium"},
                    {"value": "hard", "label": "Hard"}
                ]
            }},
            "required": false,
            "show_in_list": true
        },
        {
            "key": "duration",
            "label": "Duration",
            "field_type": {"type": "text", "config": {"placeholder": "e.g., Always running, On-demand"}},
            "required": false
        },
        {
            "key": "earnings",
            "label": "Earnings Tiers",
            "field_type": {"type": "repeater", "config": {
                "item_label": "Tier",
                "orderable": true,
                "collapsible": true,
                "min_items": 1,
                "fields": [
                    {"key": "name", "label": "Tier Name", "field_type": {"type": "text"}, "required": true},
                    {"key": "min_earnings", "label": "Min Earnings", "field_type": {"type": "number", "config": {"min": 0, "unit": "$", "precision": 2}}, "required": true},
                    {"key": "max_earnings", "label": "Max Earnings", "field_type": {"type": "number", "config": {"min": 0, "unit": "$", "precision": 2}}, "required": true},
                    {"key": "period", "label": "Period", "field_type": {"type": "select", "config": {"options": [{"value": "hour", "label": "Per Hour"}, {"value": "day", "label": "Per Day"}, {"value": "week", "label": "Per Week"}, {"value": "month", "label": "Per Month"}], "default": "month"}}},
                    {"key": "features", "label": "Features", "field_type": {"type": "list", "config": {}}},
                    {"key": "is_highlighted", "label": "Highlight", "field_type": {"type": "boolean"}}
                ]
            }},
            "required": false
        },
        {
            "key": "requirements",
            "label": "Requirements",
            "field_type": {"type": "list", "config": {}},
            "required": false,
            "translatable": true
        }
    ]'::jsonb,
    '{
        "has_slug": true,
        "slug_field": "title",
        "title_field": "title",
        "preview_fields": ["title", "task_status", "difficulty"],
        "orderable": true,
        "translatable": true,
        "versioned": true
    }'::jsonb,
    true
);

-- Guide Schema
INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, is_system) VALUES (
    'guide',
    'Guide',
    'Guides',
    'Step-by-step tutorials',
    'book-open',
    '[
        {
            "key": "title",
            "label": "Title",
            "field_type": {"type": "text", "config": {"max_length": 100}},
            "required": true,
            "translatable": true,
            "show_in_list": true,
            "searchable": true
        },
        {
            "key": "description",
            "label": "Description",
            "field_type": {"type": "rich_text", "config": {}},
            "required": true,
            "translatable": true,
            "searchable": true
        },
        {
            "key": "thumbnail",
            "label": "Thumbnail",
            "field_type": {"type": "media", "config": {"allowed_types": ["image"], "aspect_ratio": "16:9"}},
            "required": false
        },
        {
            "key": "video_url",
            "label": "Video URL",
            "field_type": {"type": "text", "config": {"placeholder": "https://youtube.com/..."}},
            "required": false
        },
        {
            "key": "difficulty",
            "label": "Difficulty",
            "field_type": {"type": "select", "config": {
                "options": [
                    {"value": "easy", "label": "Easy"},
                    {"value": "medium", "label": "Medium"},
                    {"value": "hard", "label": "Hard"}
                ]
            }},
            "required": false,
            "show_in_list": true
        },
        {
            "key": "duration_minutes",
            "label": "Duration (minutes)",
            "field_type": {"type": "number", "config": {"min": 1, "max": 120, "step": 1, "unit": "min"}},
            "required": false
        },
        {
            "key": "is_featured",
            "label": "Featured",
            "field_type": {"type": "boolean", "config": {}},
            "required": false,
            "show_in_list": true
        },
        {
            "key": "requirements",
            "label": "Requirements",
            "field_type": {"type": "list", "config": {}},
            "required": false,
            "translatable": true
        },
        {
            "key": "steps",
            "label": "Steps",
            "field_type": {"type": "repeater", "config": {
                "item_label": "Step",
                "orderable": true,
                "collapsible": true,
                "min_items": 1,
                "fields": [
                    {"key": "title", "label": "Step Title", "field_type": {"type": "text"}, "required": true},
                    {"key": "description", "label": "Instructions", "field_type": {"type": "rich_text"}, "required": true},
                    {"key": "image", "label": "Screenshot", "field_type": {"type": "media", "config": {"allowed_types": ["image"]}}},
                    {"key": "video", "label": "Video", "field_type": {"type": "media", "config": {"allowed_types": ["video"]}}}
                ]
            }},
            "required": true,
            "translatable": true
        },
        {
            "key": "related_tasks",
            "label": "Related Tasks",
            "field_type": {"type": "reference_list", "config": {"allowed_types": ["task"], "display_field": "title"}},
            "required": false
        }
    ]'::jsonb,
    '{
        "has_slug": true,
        "slug_field": "title",
        "title_field": "title",
        "preview_fields": ["title", "difficulty", "is_featured"],
        "orderable": true,
        "translatable": true,
        "versioned": true
    }'::jsonb,
    true
);

-- FAQ Schema
INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, is_system) VALUES (
    'faq',
    'FAQ',
    'FAQs',
    'Frequently Asked Questions',
    'help-circle',
    '[
        {
            "key": "category",
            "label": "Category",
            "field_type": {"type": "select", "config": {
                "options": [
                    {"value": "general", "label": "General"},
                    {"value": "earning", "label": "Earnings"},
                    {"value": "technical", "label": "Technical"},
                    {"value": "account", "label": "Account"},
                    {"value": "payment", "label": "Payment"}
                ]
            }},
            "required": true,
            "show_in_list": true
        },
        {
            "key": "question",
            "label": "Question",
            "field_type": {"type": "text", "config": {"max_length": 500}},
            "required": true,
            "translatable": true,
            "show_in_list": true,
            "searchable": true
        },
        {
            "key": "answer",
            "label": "Answer",
            "field_type": {"type": "rich_text", "config": {}},
            "required": true,
            "translatable": true,
            "searchable": true
        },
        {
            "key": "tags",
            "label": "Tags",
            "field_type": {"type": "list", "config": {}},
            "required": false
        },
        {
            "key": "related_guides",
            "label": "Related Guides",
            "field_type": {"type": "reference_list", "config": {"allowed_types": ["guide"], "display_field": "title"}},
            "required": false
        }
    ]'::jsonb,
    '{
        "title_field": "question",
        "preview_fields": ["question", "category"],
        "orderable": true,
        "translatable": true,
        "versioned": true
    }'::jsonb,
    true
);

-- Error Schema
INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, is_system) VALUES (
    'error',
    'Error',
    'Errors',
    'Error documentation',
    'alert-triangle',
    '[
        {
            "key": "error_code",
            "label": "Error Code",
            "field_type": {"type": "text", "config": {"pattern": "^[A-Z0-9_]+$", "placeholder": "e.g., E001, NETWORK_TIMEOUT"}},
            "required": true,
            "show_in_list": true
        },
        {
            "key": "title",
            "label": "Title",
            "field_type": {"type": "text", "config": {"max_length": 100}},
            "required": true,
            "translatable": true,
            "show_in_list": true,
            "searchable": true
        },
        {
            "key": "description",
            "label": "Description",
            "field_type": {"type": "rich_text", "config": {}},
            "required": true,
            "translatable": true,
            "searchable": true
        },
        {
            "key": "severity",
            "label": "Severity",
            "field_type": {"type": "select", "config": {
                "options": [
                    {"value": "info", "label": "Info"},
                    {"value": "warning", "label": "Warning"},
                    {"value": "error", "label": "Error"},
                    {"value": "critical", "label": "Critical"}
                ]
            }},
            "required": false,
            "show_in_list": true
        },
        {
            "key": "causes",
            "label": "Possible Causes",
            "field_type": {"type": "list", "config": {}},
            "required": false,
            "translatable": true
        },
        {
            "key": "solutions",
            "label": "Solutions",
            "field_type": {"type": "repeater", "config": {
                "item_label": "Solution",
                "orderable": true,
                "collapsible": true,
                "fields": [
                    {"key": "title", "label": "Solution Title", "field_type": {"type": "text"}, "required": true},
                    {"key": "steps", "label": "Steps", "field_type": {"type": "list"}, "required": true},
                    {"key": "success_rate", "label": "Success Rate", "field_type": {"type": "number", "config": {"min": 0, "max": 100, "unit": "%"}}}
                ]
            }},
            "required": false,
            "translatable": true
        },
        {
            "key": "related_errors",
            "label": "Related Errors",
            "field_type": {"type": "reference_list", "config": {"allowed_types": ["error"], "display_field": "error_code"}},
            "required": false
        }
    ]'::jsonb,
    '{
        "has_slug": true,
        "slug_field": "error_code",
        "title_field": "title",
        "preview_fields": ["error_code", "title", "severity"],
        "orderable": false,
        "translatable": true,
        "versioned": true
    }'::jsonb,
    true
);

-- Front Page Schema (NEW!)
INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, is_system) VALUES (
    'front',
    'Front Page',
    'Front Pages',
    'Homepage sections',
    'home',
    '[
        {
            "key": "title",
            "label": "Section Title",
            "field_type": {"type": "text", "config": {"max_length": 100}},
            "required": true,
            "translatable": true,
            "show_in_list": true
        },
        {
            "key": "description",
            "label": "Section Description",
            "field_type": {"type": "rich_text", "config": {}},
            "required": false,
            "translatable": true
        },
        {
            "key": "slides",
            "label": "Slides",
            "field_type": {"type": "repeater", "config": {
                "item_label": "Slide",
                "orderable": true,
                "collapsible": true,
                "min_items": 1,
                "max_items": 10,
                "fields": [
                    {"key": "title", "label": "Slide Title", "field_type": {"type": "text", "config": {"max_length": 100}}, "required": true},
                    {"key": "description", "label": "Slide Description", "field_type": {"type": "text", "config": {"max_length": 500, "multiline": true}}},
                    {"key": "image", "label": "Image", "field_type": {"type": "media", "config": {"allowed_types": ["image"], "aspect_ratio": "16:9"}}, "required": true},
                    {"key": "cta_text", "label": "Button Text", "field_type": {"type": "text", "config": {"max_length": 50}}},
                    {"key": "cta_action", "label": "Button Action", "field_type": {"type": "select", "config": {"options": [{"value": "navigate", "label": "Navigate to page"}, {"value": "external", "label": "External link"}, {"value": "modal", "label": "Open modal"}]}}},
                    {"key": "cta_target", "label": "Button Target", "field_type": {"type": "text", "config": {"placeholder": "/guides/setup or https://..."}}}
                ]
            }},
            "required": true,
            "translatable": true
        }
    ]'::jsonb,
    '{
        "has_slug": true,
        "slug_field": "title",
        "title_field": "title",
        "preview_fields": ["title"],
        "orderable": true,
        "translatable": true,
        "versioned": true
    }'::jsonb,
    true
);
```

### Step 3: Data Migration Function

```sql
-- File: migrations/20260601100003_migrate_content_data.up.sql

-- Migrate existing page_contents to content_items
INSERT INTO content_items (
    id,
    schema_id,
    slug,
    display_order,
    status,
    version,
    data,
    translations,
    translation_status,
    is_featured,
    is_active,
    published_version,
    published_at,
    published_by,
    publish_at,
    unpublish_at,
    created_at,
    updated_at,
    created_by,
    updated_by
)
SELECT
    gen_random_uuid(),
    content_type,
    slug,
    display_order,
    CASE status
        WHEN 'draft' THEN 'draft'::content_status
        WHEN 'pending_review' THEN 'pending_review'::content_status
        WHEN 'approved' THEN 'approved'::content_status
        WHEN 'published' THEN 'published'::content_status
        WHEN 'archived' THEN 'archived'::content_status
        ELSE 'draft'::content_status
    END,
    version,
    content,
    translations,
    translation_status,
    is_featured,
    is_active,
    published_version,
    published_at,
    published_by,
    publish_at,
    unpublish_at,
    created_at,
    updated_at,
    created_by,
    updated_by
FROM page_contents
WHERE content_type IN ('task', 'guide', 'faq', 'error');

-- Migrate content versions
INSERT INTO content_item_versions (
    id,
    content_id,
    version_number,
    data,
    translations,
    change_summary,
    created_at,
    created_by
)
SELECT
    gen_random_uuid(),
    ci.id,
    cv.version,
    cv.content,
    cv.translations,
    cv.change_summary,
    cv.created_at,
    cv.created_by
FROM content_versions cv
JOIN page_contents pc ON cv.content_id = pc.id
JOIN content_items ci ON ci.slug = pc.slug AND ci.schema_id = pc.content_type;
```

---

## File Changes Required

### uno-app

| File | Action | Description |
|------|--------|-------------|
| `src/types/schema.rs` | CREATE | Field types, schema definitions |
| `src/types/content_item.rs` | CREATE | Generic content item types |
| `src/types/content.rs` | UPDATE | Keep for backward compat, deprecate |
| `src/server/repositories/schema_repository.rs` | CREATE | Schema CRUD |
| `src/server/repositories/content_item_repository.rs` | CREATE | Generic content CRUD |
| `src/server/services/schema_service.rs` | CREATE | Schema operations |
| `src/server/services/content_item_service.rs` | CREATE | Generic content operations |
| `src/server/handlers/schema_handler.rs` | CREATE | Schema API endpoints |
| `src/server/handlers/content_item_handler.rs` | CREATE | Generic content endpoints |
| `src/server/handlers/mod.rs` | UPDATE | Add new routes |

### uno-admin

| File | Action | Description |
|------|--------|-------------|
| `src/types/schema.rs` | CREATE | Mirror schema types |
| `src/types/field_value.rs` | CREATE | Field value types |
| `src/api/schema_client.rs` | CREATE | Schema API client |
| `src/ui/components/cms/field_renderer.rs` | CREATE | Dynamic field rendering |
| `src/ui/components/cms/fields/*.rs` | CREATE | Individual field components |
| `src/ui/pages/content/editor.rs` | UPDATE | Use schema-driven form |
| `src/ui/pages/content/list.rs` | UPDATE | Use schema for columns |

---

## Testing Checklist

- [ ] Schema CRUD operations work
- [ ] Content creation with schema validation
- [ ] Content update preserves field values
- [ ] Repeater fields work (add/remove/reorder)
- [ ] Media fields work with picker
- [ ] Reference fields work
- [ ] Translations work per-field
- [ ] Version history preserved
- [ ] Review workflow works
- [ ] Publishing works
- [ ] Scheduling works
- [ ] Existing content migrated correctly
- [ ] Public API serves content correctly
- [ ] Admin UI renders all field types

---

## Rollback Plan

If issues arise:
1. Keep `page_contents` table intact during migration
2. New system uses `content_items` table
3. Can revert by switching back to old handlers
4. Data remains in both tables during transition

---

## Timeline

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: Database | 1 day | None |
| Phase 2: Core Types | 2 days | Phase 1 |
| Phase 3: Schema Definitions | 1 day | Phase 2 |
| Phase 4: Repository | 2 days | Phase 3 |
| Phase 5: API Layer | 2 days | Phase 4 |
| Phase 6: Data Migration | 1 day | Phase 5 |
| Phase 7: Admin Frontend | 3-4 days | Phase 5 |
| Phase 8: Cleanup | 1 day | Phase 7 |
| **Total** | **~2 weeks** | |
