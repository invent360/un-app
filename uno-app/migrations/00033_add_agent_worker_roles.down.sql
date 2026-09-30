-- Remove new roles: country_agent and integration_worker

-- Remove role_permissions first (due to foreign key constraints)
DELETE FROM role_permissions
WHERE role_id IN (
    SELECT id FROM roles WHERE name IN ('country_agent', 'integration_worker')
);

-- Remove the roles
DELETE FROM roles WHERE name IN ('country_agent', 'integration_worker');
