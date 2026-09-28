-- Seed Data
-- Consolidated: RBAC, licenses, and content schemas

-- ============================================
-- SYSTEM ROLES
-- ============================================

INSERT INTO roles (name, description, is_system)
VALUES
    ('super_admin', 'Full system access with all permissions', true),
    ('admin', 'Administrative access to manage content and users', true),
    ('editor', 'Can create and edit content', true),
    ('viewer', 'Read-only access to content', true),
    ('user', 'Standard user role', true)
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- PERMISSIONS
-- ============================================

-- Content permissions
INSERT INTO permissions (name, description, resource, action) VALUES
    ('content.create', 'Create new content items', 'content', 'create'),
    ('content.read', 'View content items', 'content', 'read'),
    ('content.update', 'Edit existing content items', 'content', 'update'),
    ('content.delete', 'Delete content items', 'content', 'delete'),
    ('content.publish', 'Publish content items', 'content', 'publish')
ON CONFLICT (name) DO NOTHING;

-- User management permissions
INSERT INTO permissions (name, description, resource, action) VALUES
    ('users.create', 'Create new users', 'users', 'create'),
    ('users.read', 'View user profiles', 'users', 'read'),
    ('users.update', 'Edit user profiles', 'users', 'update'),
    ('users.delete', 'Delete users', 'users', 'delete'),
    ('users.manage_roles', 'Assign roles to users', 'users', 'manage_roles')
ON CONFLICT (name) DO NOTHING;

-- License permissions
INSERT INTO permissions (name, description, resource, action) VALUES
    ('licenses.create', 'Create license types', 'licenses', 'create'),
    ('licenses.read', 'View licenses', 'licenses', 'read'),
    ('licenses.update', 'Edit license types', 'licenses', 'update'),
    ('licenses.delete', 'Delete license types', 'licenses', 'delete'),
    ('licenses.manage_claims', 'Manage license claims', 'licenses', 'manage_claims')
ON CONFLICT (name) DO NOTHING;

-- System permissions
INSERT INTO permissions (name, description, resource, action) VALUES
    ('system.settings', 'Manage system settings', 'system', 'settings'),
    ('system.audit_logs', 'View audit logs', 'system', 'audit_logs'),
    ('system.analytics', 'View analytics and reports', 'system', 'analytics')
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- ROLE PERMISSION ASSIGNMENTS
-- ============================================

-- Super Admin gets all permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'super_admin'
ON CONFLICT DO NOTHING;

-- Admin gets most permissions except system settings
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'admin'
AND p.name NOT IN ('system.settings')
ON CONFLICT DO NOTHING;

-- Editor gets content permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'editor'
AND p.resource = 'content'
ON CONFLICT DO NOTHING;

-- Viewer gets read permissions only
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'viewer'
AND p.action = 'read'
ON CONFLICT DO NOTHING;

-- ============================================
-- CONTENT SCHEMAS
-- ============================================
-- Note: id is the slug (e.g., 'home', 'task', 'guide')

INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, version, is_system)
VALUES (
    'home',
    'Home',
    'Home',
    'Homepage with sections (Hero, How It Works, Earnings, Testimonials)',
    'home',
    '[
        {"key": "title", "label": "Title", "field_type": {"type": "text", "config": {"max_length": 100}}, "show_in_list": true, "translatable": true},
        {"key": "description", "label": "Description", "field_type": {"type": "rich_text", "config": {}}, "translatable": true},
        {"key": "sections", "label": "Sections", "required": true, "field_type": {"type": "repeater", "config": {"fields": [
            {"key": "section_type", "label": "Section Type", "required": true, "field_type": {"type": "select", "config": {"options": [{"label": "Hero", "value": "hero"}, {"label": "How It Works", "value": "how_it_works"}, {"label": "Earnings", "value": "earnings"}, {"label": "Testimonials", "value": "testimonials"}]}}},
            {"key": "title", "label": "Title", "field_type": {"type": "text", "config": {"max_length": 100}}, "translatable": true},
            {"key": "description", "label": "Description", "field_type": {"type": "rich_text", "config": {}}, "translatable": true},
            {"key": "display_order", "label": "Display Order", "required": true, "field_type": {"type": "number", "config": {"min": 1, "step": 1}}},
            {"key": "is_visible", "label": "Visible", "field_type": {"type": "boolean", "config": {"default": true}}},
            {"key": "data", "label": "Type-Specific Data", "field_type": {"type": "json", "config": {}}}
        ], "min_items": 1, "orderable": true, "item_label": "Section", "collapsible": true}}, "translatable": true}
    ]'::jsonb,
    '{"has_slug": true, "orderable": true, "singleton": true, "versioned": true, "slug_field": "title", "title_field": "title", "translatable": true, "preview_fields": ["title"]}'::jsonb,
    1,
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, version, is_system)
VALUES (
    'task',
    'Task',
    'Tasks',
    'Earning tasks for users',
    'briefcase',
    '[
        {"key": "title", "label": "Title", "required": true, "field_type": {"type": "text", "config": {"max_length": 100}}, "searchable": true, "show_in_list": true, "translatable": true},
        {"key": "description", "label": "Description", "required": true, "field_type": {"type": "rich_text", "config": {}}, "searchable": true, "translatable": true},
        {"key": "image", "label": "Cover Image", "required": false, "field_type": {"type": "media", "config": {"allowed_types": ["image"]}}},
        {"key": "task_status", "label": "Task Status", "required": true, "field_type": {"type": "select", "config": {"default": "active", "options": [{"label": "Active", "value": "active"}, {"label": "Coming Soon", "value": "coming_soon"}, {"label": "Deprecated", "value": "deprecated"}]}}, "show_in_list": true},
        {"key": "difficulty", "label": "Difficulty", "field_type": {"type": "select", "config": {"options": [{"label": "Easy", "value": "easy"}, {"label": "Medium", "value": "medium"}, {"label": "Hard", "value": "hard"}]}}, "show_in_list": true},
        {"key": "duration", "label": "Duration", "field_type": {"type": "text", "config": {"placeholder": "e.g., Always running, On-demand"}}},
        {"key": "earnings", "label": "Earnings Tiers", "field_type": {"type": "repeater", "config": {"fields": [
            {"key": "name", "label": "Tier Name", "required": true, "field_type": {"type": "text", "config": {}}},
            {"key": "min_earnings", "label": "Min Earnings", "required": true, "field_type": {"type": "number", "config": {"min": 0, "unit": "$", "precision": 2}}},
            {"key": "max_earnings", "label": "Max Earnings", "required": true, "field_type": {"type": "number", "config": {"min": 0, "unit": "$", "precision": 2}}},
            {"key": "period", "label": "Period", "field_type": {"type": "select", "config": {"default": "month", "options": [{"label": "Per Hour", "value": "hour"}, {"label": "Per Day", "value": "day"}, {"label": "Per Week", "value": "week"}, {"label": "Per Month", "value": "month"}]}}},
            {"key": "features", "label": "Features", "field_type": {"type": "list", "config": {}}},
            {"key": "is_popular", "label": "Popular", "field_type": {"type": "boolean", "config": {}}}
        ], "min_items": 1, "orderable": true, "item_label": "Tier", "collapsible": true}}},
        {"key": "requirements", "label": "Requirements", "field_type": {"type": "list", "config": {}}, "translatable": true}
    ]'::jsonb,
    '{"has_slug": true, "orderable": true, "versioned": true, "slug_field": "title", "title_field": "title", "translatable": true, "preview_fields": ["title", "task_status", "difficulty"]}'::jsonb,
    1,
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, version, is_system)
VALUES (
    'guide',
    'Guide',
    'Guides',
    'Multi-section tutorials and documentation',
    'book-open',
    '[
        {"key": "name", "label": "Page Name", "required": true, "field_type": {"type": "text", "config": {"max_length": 100}}, "searchable": true, "show_in_list": true, "translatable": true},
        {"key": "description", "label": "Page Description", "required": true, "field_type": {"type": "rich_text", "config": {}}, "searchable": true, "translatable": true},
        {"key": "sections", "label": "Sections", "required": true, "field_type": {"type": "repeater", "config": {"fields": [
            {"key": "slug", "label": "Slug", "required": true, "field_type": {"type": "text", "config": {"pattern": "^[a-z0-9-]+$", "max_length": 50}}},
            {"key": "title", "label": "Section Title", "required": true, "field_type": {"type": "text", "config": {"max_length": 100}}, "translatable": true},
            {"key": "description", "label": "Section Description", "field_type": {"type": "rich_text", "config": {}}, "translatable": true},
            {"key": "cover_images", "label": "Cover Images", "field_type": {"type": "list", "config": {"item_type": "media"}}},
            {"key": "stages", "label": "Stages", "field_type": {"type": "repeater", "config": {"fields": [
                {"key": "order", "label": "Order", "field_type": {"type": "number", "config": {"min": 1, "step": 1}}},
                {"key": "title", "label": "Stage Title", "required": true, "field_type": {"type": "text", "config": {"max_length": 100}}, "translatable": true},
                {"key": "description", "label": "Description", "required": true, "field_type": {"type": "rich_text", "config": {}}, "translatable": true},
                {"key": "images", "label": "Stage Images", "field_type": {"type": "list", "config": {"item_type": "media"}}}
            ], "min_items": 1, "orderable": true, "item_label": "Stage", "collapsible": true}}}
        ], "min_items": 1, "orderable": true, "item_label": "Section", "collapsible": true}}, "translatable": true}
    ]'::jsonb,
    '{"has_slug": true, "orderable": true, "versioned": true, "slug_field": "name", "title_field": "name", "translatable": true, "preview_fields": ["name"]}'::jsonb,
    1,
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, version, is_system)
VALUES (
    'faq',
    'FAQ',
    'FAQs',
    'Frequently Asked Questions',
    'help-circle',
    '[
        {"key": "category", "label": "Category", "required": true, "field_type": {"type": "select", "config": {"options": [{"label": "General", "value": "general"}, {"label": "Earnings", "value": "earning"}, {"label": "Technical", "value": "technical"}, {"label": "Account", "value": "account"}, {"label": "Payment", "value": "payment"}]}}, "show_in_list": true},
        {"key": "question", "label": "Question", "required": true, "field_type": {"type": "text", "config": {"max_length": 500}}, "searchable": true, "show_in_list": true, "translatable": true},
        {"key": "answer", "label": "Answer", "required": true, "field_type": {"type": "rich_text", "config": {}}, "searchable": true, "translatable": true},
        {"key": "tags", "label": "Tags", "field_type": {"type": "list", "config": {}}}
    ]'::jsonb,
    '{"orderable": true, "versioned": true, "title_field": "question", "translatable": true, "preview_fields": ["question", "category"]}'::jsonb,
    1,
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO content_schemas (id, name, name_plural, description, icon, fields, settings, version, is_system)
VALUES (
    'error',
    'Error',
    'Errors',
    'Error documentation',
    'alert-triangle',
    '[
        {"key": "error_code", "label": "Error Code", "required": true, "field_type": {"type": "text", "config": {"pattern": "^[A-Z0-9_]+$", "placeholder": "e.g., E001, NETWORK_TIMEOUT"}}, "show_in_list": true},
        {"key": "title", "label": "Title", "required": true, "field_type": {"type": "text", "config": {"max_length": 100}}, "searchable": true, "show_in_list": true, "translatable": true},
        {"key": "description", "label": "Description", "required": true, "field_type": {"type": "rich_text", "config": {}}, "searchable": true, "translatable": true},
        {"key": "severity", "label": "Severity", "field_type": {"type": "select", "config": {"options": [{"label": "Info", "value": "info"}, {"label": "Warning", "value": "warning"}, {"label": "Error", "value": "error"}, {"label": "Critical", "value": "critical"}]}}, "show_in_list": true},
        {"key": "causes", "label": "Possible Causes", "field_type": {"type": "list", "config": {}}, "translatable": true},
        {"key": "solutions", "label": "Solutions", "field_type": {"type": "repeater", "config": {"fields": [
            {"key": "title", "label": "Solution Title", "required": true, "field_type": {"type": "text", "config": {}}},
            {"key": "steps", "label": "Steps", "required": true, "field_type": {"type": "list", "config": {}}}
        ], "orderable": true, "item_label": "Solution", "collapsible": true}}, "translatable": true}
    ]'::jsonb,
    '{"has_slug": true, "orderable": false, "versioned": true, "slug_field": "error_code", "title_field": "title", "translatable": true, "preview_fields": ["error_code", "title", "severity"]}'::jsonb,
    1,
    true
) ON CONFLICT (id) DO NOTHING;

-- ============================================
-- SAMPLE TESTIMONIALS
-- ============================================

INSERT INTO testimonials (quote, author_name, author_location, rating, is_featured, is_active, display_order)
VALUES
    ('This app has completely changed how I earn passive income. Easy setup and consistent earnings!', 'Sarah M.', 'United States', 5, true, true, 1),
    ('I was skeptical at first, but the results speak for themselves. Highly recommended!', 'James K.', 'United Kingdom', 5, true, true, 2),
    ('Simple interface, great support team, and reliable payments every month.', 'Maria L.', 'Spain', 5, false, true, 3)
ON CONFLICT DO NOTHING;
