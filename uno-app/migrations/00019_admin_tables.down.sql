-- Migration 00019 Down: Remove Admin Tables

-- Remove indexes first
DROP INDEX IF EXISTS idx_licenses_marketplace;
DROP INDEX IF EXISTS idx_licenses_synced_at;
DROP INDEX IF EXISTS idx_licenses_is_online;
DROP INDEX IF EXISTS idx_licenses_agent_id;

DROP INDEX IF EXISTS idx_sync_jobs_next_retry;
DROP INDEX IF EXISTS idx_sync_jobs_job_type;
DROP INDEX IF EXISTS idx_sync_jobs_target_date;
DROP INDEX IF EXISTS idx_sync_jobs_status;

DROP INDEX IF EXISTS idx_license_analytics_composite;
DROP INDEX IF EXISTS idx_license_analytics_date;
DROP INDEX IF EXISTS idx_license_analytics_license;

DROP INDEX IF EXISTS idx_rewards_created_at;
DROP INDEX IF EXISTS idx_rewards_completed_at;
DROP INDEX IF EXISTS idx_rewards_reward_type;
DROP INDEX IF EXISTS idx_rewards_node_id;
DROP INDEX IF EXISTS idx_rewards_license_id;
DROP INDEX IF EXISTS idx_rewards_user_id;

DROP INDEX IF EXISTS idx_nodes_is_active;
DROP INDEX IF EXISTS idx_nodes_node_id;

DROP INDEX IF EXISTS idx_agents_referral_code;
DROP INDEX IF EXISTS idx_agents_is_active;
DROP INDEX IF EXISTS idx_agents_country;
DROP INDEX IF EXISTS idx_agents_email;

-- Drop tables
DROP TABLE IF EXISTS sync_jobs;
DROP TABLE IF EXISTS license_analytics;
DROP TABLE IF EXISTS rewards;
DROP TABLE IF EXISTS nodes;

-- Remove agent_id foreign key before dropping agents table
ALTER TABLE licenses DROP COLUMN IF EXISTS agent_id;
DROP TABLE IF EXISTS agents;

-- Note: We don't remove the additional license columns
-- as they may contain data and other parts of the system may depend on them
-- (ulo_name, uno_share, agent_share, ulo_share, is_online, alias,
-- is_on_marketplace, is_on_uno_marketplace, is_published, marketplace_status,
-- marketplace_referral_code, synced_at, validation_last_success_at, settings,
-- activation_by, activation_postponed_ms, lease_user_id)
