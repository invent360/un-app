-- Drop indexes (ScyllaDB requires explicit drops)
DROP INDEX IF EXISTS idx_agents_email;
DROP INDEX IF EXISTS idx_agents_country;
DROP INDEX IF EXISTS idx_agents_is_active;
DROP INDEX IF EXISTS idx_licenses_license_id;
DROP INDEX IF EXISTS idx_licenses_node_id;
DROP INDEX IF EXISTS idx_licenses_agent_id;
DROP INDEX IF EXISTS idx_licenses_ulo_name;
DROP INDEX IF EXISTS idx_licenses_is_online;
DROP INDEX IF EXISTS idx_licenses_is_active;
DROP INDEX IF EXISTS idx_nodes_node_id;
DROP INDEX IF EXISTS idx_nodes_is_active;
DROP INDEX IF EXISTS idx_rewards_user_id;
DROP INDEX IF EXISTS idx_rewards_license_id;
DROP INDEX IF EXISTS idx_rewards_node_id;
DROP INDEX IF EXISTS idx_rewards_type;
DROP INDEX IF EXISTS idx_rewards_completed_at;
