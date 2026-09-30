-- Migration 00019: Admin Tables for uno-admin
-- Phase 1 (P1-05): Create PostgreSQL tables for uno-admin (migrated from Scylla)

-- ============================================
-- AGENTS TABLE
-- ============================================
-- Represents operators/agents managing licenses

CREATE TABLE IF NOT EXISTS agents (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    country VARCHAR(100),
    commission_percent DECIMAL(5,2) NOT NULL DEFAULT 3.0,
    referral_code VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_agents_email ON agents(email) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_agents_country ON agents(country);
CREATE INDEX IF NOT EXISTS idx_agents_is_active ON agents(is_active);
CREATE INDEX IF NOT EXISTS idx_agents_referral_code ON agents(referral_code);

-- ============================================
-- NODES TABLE
-- ============================================
-- Aggregated node data for admin dashboard

CREATE TABLE IF NOT EXISTS nodes (
    id VARCHAR(36) PRIMARY KEY,
    node_id VARCHAR(66) NOT NULL,
    name VARCHAR(255),
    total_licenses INT NOT NULL DEFAULT 0,
    online_licenses INT NOT NULL DEFAULT 0,
    total_earnings_micros BIGINT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_nodes_node_id ON nodes(node_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_nodes_is_active ON nodes(is_active);

-- ============================================
-- REWARDS TABLE
-- ============================================
-- Reward transactions for licenses

CREATE TABLE IF NOT EXISTS rewards (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    reward_type VARCHAR(50) NOT NULL,
    description TEXT,
    node_id VARCHAR(66) NOT NULL,
    license_id VARCHAR(66) NOT NULL,
    license_lease_id VARCHAR(255),
    task_key VARCHAR(255),
    task_metadata JSONB,
    completed_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    amount_micros BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_rewards_user_id ON rewards(user_id);
CREATE INDEX IF NOT EXISTS idx_rewards_license_id ON rewards(license_id);
CREATE INDEX IF NOT EXISTS idx_rewards_node_id ON rewards(node_id);
CREATE INDEX IF NOT EXISTS idx_rewards_reward_type ON rewards(reward_type);
CREATE INDEX IF NOT EXISTS idx_rewards_completed_at ON rewards(completed_at DESC);
CREATE INDEX IF NOT EXISTS idx_rewards_created_at ON rewards(created_at DESC);

-- ============================================
-- LICENSE ANALYTICS TABLE
-- ============================================
-- Daily analytics per license (time-series data)

CREATE TABLE IF NOT EXISTS license_analytics (
    id SERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,
    date DATE NOT NULL,
    uptime DECIMAL(5,4) NOT NULL DEFAULT 0,
    required_uptime DECIMAL(5,4) NOT NULL DEFAULT 0.75,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_license_analytics UNIQUE (license_id, date)
);

CREATE INDEX IF NOT EXISTS idx_license_analytics_license ON license_analytics(license_id);
CREATE INDEX IF NOT EXISTS idx_license_analytics_date ON license_analytics(date DESC);
CREATE INDEX IF NOT EXISTS idx_license_analytics_composite ON license_analytics(license_id, date DESC);

-- ============================================
-- SYNC JOBS TABLE
-- ============================================
-- Background job tracking for admin sync operations
-- Note: This is separate from job_queue (general purpose)
-- as it tracks specific sync workflows with richer metadata

CREATE TABLE IF NOT EXISTS sync_jobs (
    id VARCHAR(36) PRIMARY KEY,
    job_type VARCHAR(100) NOT NULL,
    target_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    attempt_count INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 3,
    records_fetched INT NOT NULL DEFAULT 0,
    records_inserted INT NOT NULL DEFAULT 0,
    error_message TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    next_retry_at TIMESTAMPTZ,
    job_context JSONB,
    duration_ms BIGINT,
    job_logs TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_sync_status CHECK (
        status IN ('pending', 'running', 'completed', 'failed', 'retrying')
    )
);

CREATE INDEX IF NOT EXISTS idx_sync_jobs_status ON sync_jobs(status);
CREATE INDEX IF NOT EXISTS idx_sync_jobs_target_date ON sync_jobs(target_date);
CREATE INDEX IF NOT EXISTS idx_sync_jobs_job_type ON sync_jobs(job_type);
CREATE INDEX IF NOT EXISTS idx_sync_jobs_next_retry ON sync_jobs(next_retry_at)
    WHERE status = 'retrying' AND next_retry_at IS NOT NULL;

-- ============================================
-- ADD ADMIN COLUMNS TO LICENSES
-- ============================================
-- Additional columns needed by uno-admin for license management

DO $$
BEGIN
    -- Agent assignment
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'agent_id'
    ) THEN
        ALTER TABLE licenses ADD COLUMN agent_id VARCHAR(36) REFERENCES agents(id);
    END IF;

    -- ULO name (operator name)
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'ulo_name'
    ) THEN
        ALTER TABLE licenses ADD COLUMN ulo_name VARCHAR(255);
    END IF;

    -- Share splits
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'uno_share'
    ) THEN
        ALTER TABLE licenses ADD COLUMN uno_share DECIMAL(5,2) DEFAULT 47.0;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'agent_share'
    ) THEN
        ALTER TABLE licenses ADD COLUMN agent_share DECIMAL(5,2) DEFAULT 3.0;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'ulo_share'
    ) THEN
        ALTER TABLE licenses ADD COLUMN ulo_share DECIMAL(5,2) DEFAULT 50.0;
    END IF;

    -- Online status
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'is_online'
    ) THEN
        ALTER TABLE licenses ADD COLUMN is_online BOOLEAN DEFAULT false;
    END IF;

    -- Alias
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'alias'
    ) THEN
        ALTER TABLE licenses ADD COLUMN alias VARCHAR(255);
    END IF;

    -- Marketplace status fields
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'is_on_marketplace'
    ) THEN
        ALTER TABLE licenses ADD COLUMN is_on_marketplace BOOLEAN DEFAULT false;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'is_on_uno_marketplace'
    ) THEN
        ALTER TABLE licenses ADD COLUMN is_on_uno_marketplace BOOLEAN DEFAULT false;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'is_published'
    ) THEN
        ALTER TABLE licenses ADD COLUMN is_published BOOLEAN DEFAULT false;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'marketplace_status'
    ) THEN
        ALTER TABLE licenses ADD COLUMN marketplace_status VARCHAR(50);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'marketplace_referral_code'
    ) THEN
        ALTER TABLE licenses ADD COLUMN marketplace_referral_code VARCHAR(50);
    END IF;

    -- Sync timestamp
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'synced_at'
    ) THEN
        ALTER TABLE licenses ADD COLUMN synced_at TIMESTAMPTZ;
    END IF;

    -- Validation timestamp
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'validation_last_success_at'
    ) THEN
        ALTER TABLE licenses ADD COLUMN validation_last_success_at TIMESTAMPTZ;
    END IF;

    -- Settings JSON
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'settings'
    ) THEN
        ALTER TABLE licenses ADD COLUMN settings JSONB;
    END IF;

    -- Activation fields
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'activation_by'
    ) THEN
        ALTER TABLE licenses ADD COLUMN activation_by VARCHAR(255);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'activation_postponed_ms'
    ) THEN
        ALTER TABLE licenses ADD COLUMN activation_postponed_ms BIGINT;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'licenses' AND column_name = 'lease_user_id'
    ) THEN
        ALTER TABLE licenses ADD COLUMN lease_user_id VARCHAR(255);
    END IF;
END $$;

-- Indexes for new columns
CREATE INDEX IF NOT EXISTS idx_licenses_agent_id ON licenses(agent_id);
CREATE INDEX IF NOT EXISTS idx_licenses_is_online ON licenses(is_online);
CREATE INDEX IF NOT EXISTS idx_licenses_synced_at ON licenses(synced_at);
CREATE INDEX IF NOT EXISTS idx_licenses_marketplace ON licenses(is_on_marketplace, is_on_uno_marketplace);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE agents IS 'Operators/agents managing licenses for uno-admin';
COMMENT ON TABLE nodes IS 'Aggregated node data for admin dashboard';
COMMENT ON TABLE rewards IS 'Reward transactions per license';
COMMENT ON TABLE license_analytics IS 'Daily uptime analytics per license';
COMMENT ON TABLE sync_jobs IS 'Background sync job tracking for admin operations';
