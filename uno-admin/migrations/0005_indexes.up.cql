-- Agent indexes
CREATE INDEX IF NOT EXISTS idx_agents_email ON agents (email);
CREATE INDEX IF NOT EXISTS idx_agents_country ON agents (country);
CREATE INDEX IF NOT EXISTS idx_agents_is_active ON agents (is_active);

-- License indexes
CREATE INDEX IF NOT EXISTS idx_licenses_license_id ON licenses (license_id);
CREATE INDEX IF NOT EXISTS idx_licenses_node_id ON licenses (node_id);
CREATE INDEX IF NOT EXISTS idx_licenses_agent_id ON licenses (agent_id);
CREATE INDEX IF NOT EXISTS idx_licenses_ulo_name ON licenses (ulo_name);
CREATE INDEX IF NOT EXISTS idx_licenses_is_online ON licenses (is_online);
CREATE INDEX IF NOT EXISTS idx_licenses_is_active ON licenses (is_active);

-- Node indexes
CREATE INDEX IF NOT EXISTS idx_nodes_node_id ON nodes (node_id);
CREATE INDEX IF NOT EXISTS idx_nodes_is_active ON nodes (is_active);

-- Rewards indexes
CREATE INDEX IF NOT EXISTS idx_rewards_user_id ON rewards (user_id);
CREATE INDEX IF NOT EXISTS idx_rewards_license_id ON rewards (license_id);
CREATE INDEX IF NOT EXISTS idx_rewards_node_id ON rewards (node_id);
CREATE INDEX IF NOT EXISTS idx_rewards_type ON rewards (type);
CREATE INDEX IF NOT EXISTS idx_rewards_completed_at ON rewards (completed_at);
