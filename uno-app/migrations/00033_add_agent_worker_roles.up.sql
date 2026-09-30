-- Add new roles: country_agent and integration_worker
-- These roles extend the existing RBAC system for Phase 2 completion

-- Insert new roles into the roles table
INSERT INTO roles (name, description, is_system)
VALUES
    ('country_agent', 'Country-level agent for regional management and oversight', true),
    ('integration_worker', 'Background service worker for external integrations and sync jobs', true)
ON CONFLICT (name) DO UPDATE
SET description = EXCLUDED.description,
    is_system = EXCLUDED.is_system;

-- Assign permissions to country_agent role
-- Country agents have read access similar to regular agents, plus country-specific capabilities
DO $$
DECLARE
    v_role_id INT;
    v_permission_id INT;
BEGIN
    -- Get country_agent role ID
    SELECT id INTO v_role_id FROM roles WHERE name = 'country_agent';

    IF v_role_id IS NOT NULL THEN
        -- Grant agent:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'agent:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant license:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'license:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant support:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'support:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant user:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'user:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant referral:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'referral:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;
    END IF;
END $$;

-- Assign permissions to integration_worker role
-- Integration workers can execute background jobs and manage integrations
DO $$
DECLARE
    v_role_id INT;
    v_permission_id INT;
BEGIN
    -- Get integration_worker role ID
    SELECT id INTO v_role_id FROM roles WHERE name = 'integration_worker';

    IF v_role_id IS NOT NULL THEN
        -- Grant worker:execute permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'worker:execute';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant integration:read permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'integration:read';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Grant integration:manage permission
        SELECT id INTO v_permission_id FROM permissions WHERE name = 'integration:manage';
        IF v_permission_id IS NOT NULL THEN
            INSERT INTO role_permissions (role_id, permission_id)
            VALUES (v_role_id, v_permission_id)
            ON CONFLICT DO NOTHING;
        END IF;
    END IF;
END $$;
