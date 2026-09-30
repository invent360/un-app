-- Rollback migration 00023: Extended RBAC

-- Remove role-permission assignments for new permissions
DELETE FROM role_permissions WHERE permission_id IN (
    SELECT id FROM permissions WHERE name IN (
        'license:read', 'license:claim', 'license:admin', 'license:import', 'license:revoke',
        'finance:read', 'finance:write', 'finance:approve', 'finance:reconcile',
        'support:read', 'support:write', 'support:escalate',
        'agent:read', 'agent:manage', 'referral:read', 'referral:approve',
        'user:read', 'user:manage', 'user:suspend',
        'gate:manage', 'worker:execute', 'integration:read', 'integration:manage'
    )
);

-- Remove new permissions
DELETE FROM permissions WHERE name IN (
    'license:read', 'license:claim', 'license:admin', 'license:import', 'license:revoke',
    'finance:read', 'finance:write', 'finance:approve', 'finance:reconcile',
    'support:read', 'support:write', 'support:escalate',
    'agent:read', 'agent:manage', 'referral:read', 'referral:approve',
    'user:read', 'user:manage', 'user:suspend',
    'gate:manage', 'worker:execute', 'integration:read', 'integration:manage'
);

-- Remove new roles (keeping original content roles)
DELETE FROM roles WHERE name IN (
    'operator', 'participant', 'support', 'agent', 'finance', 'worker'
);

-- Drop user_roles table
DROP TABLE IF EXISTS user_roles;
