-- Migration 00023: Extended RBAC for Phase 2
-- Adds new roles, permissions, and user-role assignments

-- ============================================
-- USER ROLES JUNCTION TABLE
-- ============================================
-- Allows users to have multiple roles (beyond the JWT primary role)

CREATE TABLE IF NOT EXISTS user_roles (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES user_identities(id) ON DELETE CASCADE,
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_by VARCHAR(255),
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,  -- Optional expiry for temporary role assignments
    is_active BOOLEAN NOT NULL DEFAULT true,

    CONSTRAINT uq_user_role UNIQUE (user_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_user_roles_user ON user_roles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role ON user_roles(role_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_active ON user_roles(is_active) WHERE is_active = true;

-- ============================================
-- SEED SYSTEM ROLES
-- ============================================

INSERT INTO roles (name, description, is_system) VALUES
    -- Content management roles
    ('content_author', 'Can create and edit content drafts', true),
    ('reviewer', 'Can review and approve content', true),
    ('publisher', 'Can publish approved content', true),

    -- Administrative roles
    ('admin', 'Full administrative access (deprecated, use operator)', true),
    ('operator', 'Full system operator access with MFA required', true),

    -- User-facing roles
    ('participant', 'End user who can claim licenses', true),

    -- Support roles
    ('support', 'Customer support staff', true),
    ('agent', 'Country/regional agent for referrals', true),

    -- Finance roles
    ('finance', 'Financial operations and reporting', true),

    -- System roles
    ('worker', 'Background job worker/service account', true)
ON CONFLICT (name) DO UPDATE SET
    description = EXCLUDED.description,
    is_system = EXCLUDED.is_system;

-- ============================================
-- SEED PERMISSIONS
-- ============================================

-- Content permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('content:create', 'content', 'create', 'Create new content'),
    ('content:read', 'content', 'read', 'Read content'),
    ('content:update', 'content', 'update', 'Update content'),
    ('content:delete', 'content', 'delete', 'Delete content'),
    ('content:publish', 'content', 'publish', 'Publish content'),
    ('content:revert', 'content', 'revert', 'Revert content to previous version'),
    ('content:schedule', 'content', 'schedule', 'Schedule content publication')
ON CONFLICT (name) DO NOTHING;

-- Review permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('review:submit', 'review', 'submit', 'Submit content for review'),
    ('review:approve', 'review', 'approve', 'Approve reviewed content')
ON CONFLICT (name) DO NOTHING;

-- License permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('license:read', 'license', 'read', 'View license information'),
    ('license:claim', 'license', 'claim', 'Claim a license'),
    ('license:admin', 'license', 'admin', 'Administer licenses'),
    ('license:import', 'license', 'import', 'Import licenses from CSV'),
    ('license:revoke', 'license', 'revoke', 'Revoke licenses')
ON CONFLICT (name) DO NOTHING;

-- Finance permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('finance:read', 'finance', 'read', 'View financial data'),
    ('finance:write', 'finance', 'write', 'Create financial records'),
    ('finance:approve', 'finance', 'approve', 'Approve financial transactions'),
    ('finance:reconcile', 'finance', 'reconcile', 'Reconcile financial records')
ON CONFLICT (name) DO NOTHING;

-- Support permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('support:read', 'support', 'read', 'View support tickets'),
    ('support:write', 'support', 'write', 'Respond to support tickets'),
    ('support:escalate', 'support', 'escalate', 'Escalate support issues')
ON CONFLICT (name) DO NOTHING;

-- Agent permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('agent:read', 'agent', 'read', 'View agent information'),
    ('agent:manage', 'agent', 'manage', 'Manage agent accounts'),
    ('referral:read', 'referral', 'read', 'View referral data'),
    ('referral:approve', 'referral', 'approve', 'Approve referral payments')
ON CONFLICT (name) DO NOTHING;

-- User management permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('user:read', 'user', 'read', 'View user information'),
    ('user:manage', 'user', 'manage', 'Manage user accounts'),
    ('user:suspend', 'user', 'suspend', 'Suspend user accounts')
ON CONFLICT (name) DO NOTHING;

-- System permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('audit:read', 'audit', 'read', 'View audit logs'),
    ('settings:manage', 'settings', 'manage', 'Manage system settings'),
    ('gate:manage', 'gate', 'manage', 'Manage launch gates')
ON CONFLICT (name) DO NOTHING;

-- Worker permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('worker:execute', 'worker', 'execute', 'Execute background jobs'),
    ('integration:read', 'integration', 'read', 'View integration status'),
    ('integration:manage', 'integration', 'manage', 'Manage integrations')
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- ASSIGN PERMISSIONS TO ROLES
-- ============================================

-- Helper function to assign permissions
CREATE OR REPLACE FUNCTION assign_permission_to_role(role_name_param VARCHAR, permission_name_param VARCHAR)
RETURNS VOID AS $$
DECLARE
    role_id_var INT;
    permission_id_var INT;
BEGIN
    SELECT id INTO role_id_var FROM roles WHERE name = role_name_param;
    SELECT id INTO permission_id_var FROM permissions WHERE name = permission_name_param;

    IF role_id_var IS NOT NULL AND permission_id_var IS NOT NULL THEN
        INSERT INTO role_permissions (role_id, permission_id)
        VALUES (role_id_var, permission_id_var)
        ON CONFLICT (role_id, permission_id) DO NOTHING;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Operator role (full access)
SELECT assign_permission_to_role('operator', 'content:create');
SELECT assign_permission_to_role('operator', 'content:read');
SELECT assign_permission_to_role('operator', 'content:update');
SELECT assign_permission_to_role('operator', 'content:delete');
SELECT assign_permission_to_role('operator', 'content:publish');
SELECT assign_permission_to_role('operator', 'content:revert');
SELECT assign_permission_to_role('operator', 'content:schedule');
SELECT assign_permission_to_role('operator', 'review:submit');
SELECT assign_permission_to_role('operator', 'review:approve');
SELECT assign_permission_to_role('operator', 'license:read');
SELECT assign_permission_to_role('operator', 'license:claim');
SELECT assign_permission_to_role('operator', 'license:admin');
SELECT assign_permission_to_role('operator', 'license:import');
SELECT assign_permission_to_role('operator', 'license:revoke');
SELECT assign_permission_to_role('operator', 'finance:read');
SELECT assign_permission_to_role('operator', 'finance:write');
SELECT assign_permission_to_role('operator', 'finance:approve');
SELECT assign_permission_to_role('operator', 'finance:reconcile');
SELECT assign_permission_to_role('operator', 'support:read');
SELECT assign_permission_to_role('operator', 'support:write');
SELECT assign_permission_to_role('operator', 'support:escalate');
SELECT assign_permission_to_role('operator', 'agent:read');
SELECT assign_permission_to_role('operator', 'agent:manage');
SELECT assign_permission_to_role('operator', 'referral:read');
SELECT assign_permission_to_role('operator', 'referral:approve');
SELECT assign_permission_to_role('operator', 'user:read');
SELECT assign_permission_to_role('operator', 'user:manage');
SELECT assign_permission_to_role('operator', 'user:suspend');
SELECT assign_permission_to_role('operator', 'audit:read');
SELECT assign_permission_to_role('operator', 'settings:manage');
SELECT assign_permission_to_role('operator', 'gate:manage');
SELECT assign_permission_to_role('operator', 'worker:execute');
SELECT assign_permission_to_role('operator', 'integration:read');
SELECT assign_permission_to_role('operator', 'integration:manage');

-- Content author role
SELECT assign_permission_to_role('content_author', 'content:create');
SELECT assign_permission_to_role('content_author', 'content:read');
SELECT assign_permission_to_role('content_author', 'content:update');
SELECT assign_permission_to_role('content_author', 'review:submit');

-- Reviewer role
SELECT assign_permission_to_role('reviewer', 'content:read');
SELECT assign_permission_to_role('reviewer', 'review:approve');

-- Publisher role
SELECT assign_permission_to_role('publisher', 'content:read');
SELECT assign_permission_to_role('publisher', 'content:publish');
SELECT assign_permission_to_role('publisher', 'content:schedule');

-- Participant role
SELECT assign_permission_to_role('participant', 'license:read');
SELECT assign_permission_to_role('participant', 'license:claim');

-- Support role
SELECT assign_permission_to_role('support', 'content:read');
SELECT assign_permission_to_role('support', 'license:read');
SELECT assign_permission_to_role('support', 'finance:read');
SELECT assign_permission_to_role('support', 'support:read');
SELECT assign_permission_to_role('support', 'support:write');
SELECT assign_permission_to_role('support', 'support:escalate');
SELECT assign_permission_to_role('support', 'user:read');
SELECT assign_permission_to_role('support', 'agent:read');

-- Agent role
SELECT assign_permission_to_role('agent', 'license:read');
SELECT assign_permission_to_role('agent', 'support:read');
SELECT assign_permission_to_role('agent', 'user:read');
SELECT assign_permission_to_role('agent', 'agent:read');
SELECT assign_permission_to_role('agent', 'referral:read');

-- Finance role
SELECT assign_permission_to_role('finance', 'license:read');
SELECT assign_permission_to_role('finance', 'license:admin');
SELECT assign_permission_to_role('finance', 'finance:read');
SELECT assign_permission_to_role('finance', 'finance:write');
SELECT assign_permission_to_role('finance', 'finance:approve');
SELECT assign_permission_to_role('finance', 'finance:reconcile');
SELECT assign_permission_to_role('finance', 'agent:read');
SELECT assign_permission_to_role('finance', 'agent:manage');
SELECT assign_permission_to_role('finance', 'referral:read');
SELECT assign_permission_to_role('finance', 'referral:approve');

-- Worker role
SELECT assign_permission_to_role('worker', 'worker:execute');
SELECT assign_permission_to_role('worker', 'integration:read');
SELECT assign_permission_to_role('worker', 'integration:manage');
SELECT assign_permission_to_role('worker', 'license:read');
SELECT assign_permission_to_role('worker', 'finance:read');
SELECT assign_permission_to_role('worker', 'finance:write');

-- Clean up helper function
DROP FUNCTION IF EXISTS assign_permission_to_role(VARCHAR, VARCHAR);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE user_roles IS 'Junction table for user-role assignments (supports multiple roles per user)';
COMMENT ON COLUMN user_roles.expires_at IS 'Optional expiry for temporary role assignments';
