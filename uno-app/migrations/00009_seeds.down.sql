-- Remove Seed Data
-- Order: reverse of insertion (respecting foreign key dependencies)

-- ============================================
-- REMOVE TESTIMONIALS
-- ============================================

DELETE FROM testimonials WHERE author_name IN ('Sarah M.', 'James K.', 'Maria L.');

-- ============================================
-- REMOVE CONTENT SCHEMAS
-- ============================================

DELETE FROM content_schemas WHERE id IN ('home', 'task', 'guide', 'faq', 'error');

-- ============================================
-- REMOVE RBAC DATA
-- ============================================

-- Remove role permissions for seeded roles
DELETE FROM role_permissions WHERE role_id IN (
    SELECT id FROM roles WHERE name IN ('super_admin', 'admin', 'editor', 'viewer', 'user')
);

-- Remove seeded permissions
DELETE FROM permissions WHERE name IN (
    'content.create', 'content.read', 'content.update', 'content.delete', 'content.publish',
    'users.create', 'users.read', 'users.update', 'users.delete', 'users.manage_roles',
    'licenses.create', 'licenses.read', 'licenses.update', 'licenses.delete', 'licenses.manage_claims',
    'system.settings', 'system.audit_logs', 'system.analytics'
);

-- Remove seeded roles
DELETE FROM roles WHERE name IN ('super_admin', 'admin', 'editor', 'viewer', 'user');
