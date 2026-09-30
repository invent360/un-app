-- Migration 00025: Service Identities for Machine-to-Machine Authentication
-- Phase 3 (P3-03): Durable service identity management with key rotation
--
-- Service identities allow machines and services to authenticate with the API
-- using HMAC-signed requests. Keys can be rotated without downtime.

-- ============================================
-- SERVICE IDENTITIES TABLE
-- ============================================
-- Each service identity represents a machine or service that can authenticate.
-- Services have an active key and optionally a previous key for rotation.

CREATE TABLE IF NOT EXISTS service_identities (
    id BIGSERIAL PRIMARY KEY,

    -- Identity identification
    service_id VARCHAR(100) NOT NULL UNIQUE,        -- Unique identifier (e.g., 'uno-admin', 'sync-worker')
    service_name VARCHAR(255) NOT NULL,             -- Human-readable name
    service_type VARCHAR(50) NOT NULL,              -- Type: 'internal', 'external', 'worker'

    -- Active key (current key for signing)
    active_key_hash VARCHAR(128) NOT NULL,          -- SHA256 hash of the active secret key
    active_key_hint VARCHAR(8) NOT NULL,            -- First 8 chars of key hash for identification
    active_key_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Previous key (still valid during rotation window)
    previous_key_hash VARCHAR(128),
    previous_key_hint VARCHAR(8),
    previous_key_created_at TIMESTAMPTZ,
    previous_key_expires_at TIMESTAMPTZ,            -- When the old key stops being valid

    -- Authorization
    roles TEXT[] NOT NULL DEFAULT '{}',             -- Allowed roles: 'admin', 'sync', 'worker', etc.
    allowed_scopes TEXT[] NOT NULL DEFAULT '{}',    -- Fine-grained permissions
    rate_limit_tier VARCHAR(20) DEFAULT 'standard', -- Rate limit tier: 'low', 'standard', 'high', 'unlimited'

    -- IP restrictions (optional)
    allowed_ips CIDR[] DEFAULT NULL,                -- If set, only these IPs can use this identity

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    suspended_at TIMESTAMPTZ,
    suspended_reason TEXT,

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100),                        -- Who created this identity
    last_used_at TIMESTAMPTZ,
    total_requests BIGINT NOT NULL DEFAULT 0,

    -- Metadata
    description TEXT,
    metadata JSONB
);

-- Index for quick lookups by service_id
CREATE UNIQUE INDEX IF NOT EXISTS idx_service_identities_service_id
ON service_identities (service_id)
WHERE is_active = TRUE;

-- Index for service type filtering
CREATE INDEX IF NOT EXISTS idx_service_identities_type
ON service_identities (service_type, is_active);

-- ============================================
-- SERVICE IDENTITY AUDIT LOG
-- ============================================
-- Track all changes to service identities for compliance.

CREATE TABLE IF NOT EXISTS service_identity_audit (
    id BIGSERIAL PRIMARY KEY,
    service_id VARCHAR(100) NOT NULL,
    action VARCHAR(50) NOT NULL,                    -- 'created', 'key_rotated', 'suspended', 'activated', etc.
    actor VARCHAR(100),                             -- Who performed the action
    details JSONB,                                  -- Additional context
    ip_address INET,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding audit trail for a service
CREATE INDEX IF NOT EXISTS idx_service_identity_audit_service
ON service_identity_audit (service_id, created_at DESC);

-- ============================================
-- SERVICE REQUEST LOG
-- ============================================
-- Track recent requests for monitoring and rate limiting.
-- Designed for short retention (24-48 hours).

CREATE TABLE IF NOT EXISTS service_request_log (
    id BIGSERIAL PRIMARY KEY,
    service_id VARCHAR(100) NOT NULL,
    endpoint VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    status_code INT,
    response_time_ms INT,
    ip_address INET,
    user_agent VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for rate limiting (requests by service in time window)
CREATE INDEX IF NOT EXISTS idx_service_request_log_rate_limit
ON service_request_log (service_id, created_at DESC);

-- Index for cleanup of old records
CREATE INDEX IF NOT EXISTS idx_service_request_log_cleanup
ON service_request_log (created_at);

-- ============================================
-- FUNCTIONS
-- ============================================

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_service_identity_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for updated_at
DROP TRIGGER IF EXISTS trigger_update_service_identity_timestamp ON service_identities;
CREATE TRIGGER trigger_update_service_identity_timestamp
BEFORE UPDATE ON service_identities
FOR EACH ROW
EXECUTE FUNCTION update_service_identity_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE service_identities IS 'Machine service identities for M2M authentication';
COMMENT ON COLUMN service_identities.service_id IS 'Unique identifier used in API requests';
COMMENT ON COLUMN service_identities.active_key_hash IS 'SHA256 hash of current secret key (never store plaintext)';
COMMENT ON COLUMN service_identities.active_key_hint IS 'First 8 chars of hash for key identification';
COMMENT ON COLUMN service_identities.previous_key_hash IS 'Hash of previous key, valid until previous_key_expires_at';
COMMENT ON COLUMN service_identities.roles IS 'Array of role names this service can assume';
COMMENT ON COLUMN service_identities.allowed_scopes IS 'Fine-grained permissions (e.g., licenses:read, sync:write)';
COMMENT ON COLUMN service_identities.rate_limit_tier IS 'Rate limiting tier for this service';
COMMENT ON COLUMN service_identities.allowed_ips IS 'CIDR ranges that can use this identity (null = any)';

COMMENT ON TABLE service_identity_audit IS 'Audit trail for service identity changes';
COMMENT ON TABLE service_request_log IS 'Short-term request log for monitoring/rate limiting';
