-- Migration 00020: Identity and Session Management
-- Phase 2 (P2-01): Implement verified contact/login flow with trusted sessions

-- ============================================
-- USER IDENTITIES TABLE
-- ============================================
-- Stores user identity linked to JWT sessions via 'sub' claim
-- Supports account linking from multiple providers

CREATE TABLE IF NOT EXISTS user_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- External provider identity (from JWT 'sub' claim)
    provider VARCHAR(50) NOT NULL,           -- e.g., 'unetwork', 'worldmobile'
    provider_user_id VARCHAR(255) NOT NULL,  -- The 'sub' claim from JWT

    -- User profile (editable)
    display_name VARCHAR(255),
    email VARCHAR(255),
    email_verified BOOLEAN NOT NULL DEFAULT false,
    phone VARCHAR(50),
    phone_verified BOOLEAN NOT NULL DEFAULT false,

    -- Account status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_suspended BOOLEAN NOT NULL DEFAULT false,
    suspension_reason TEXT,
    suspended_at TIMESTAMPTZ,
    suspended_by VARCHAR(255),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,

    -- Unique constraint: one identity per provider/user combination
    CONSTRAINT uq_provider_user UNIQUE (provider, provider_user_id)
);

CREATE INDEX IF NOT EXISTS idx_user_identities_email ON user_identities(email) WHERE email IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_user_identities_provider ON user_identities(provider);
CREATE INDEX IF NOT EXISTS idx_user_identities_active ON user_identities(is_active, is_suspended);

-- ============================================
-- ACTIVE SESSIONS TABLE
-- ============================================
-- Tracks active sessions for revocation support
-- Sessions can be revoked by user or admin

CREATE TABLE IF NOT EXISTS active_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Link to user identity
    user_id UUID NOT NULL REFERENCES user_identities(id) ON DELETE CASCADE,

    -- Session identification (from JWT)
    session_token_hash VARCHAR(64) NOT NULL,  -- SHA256 hash of JWT 'jti' or token

    -- Session metadata
    issued_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Device/client info
    user_agent TEXT,
    ip_address INET,
    device_fingerprint VARCHAR(64),

    -- MFA status
    mfa_verified BOOLEAN NOT NULL DEFAULT false,
    mfa_verified_at TIMESTAMPTZ,

    -- Revocation
    is_revoked BOOLEAN NOT NULL DEFAULT false,
    revoked_at TIMESTAMPTZ,
    revoked_by VARCHAR(255),
    revocation_reason VARCHAR(100),  -- 'logout', 'expired', 'admin_revoke', 'security'

    -- Unique active session per token
    CONSTRAINT uq_session_token UNIQUE (session_token_hash)
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON active_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON active_sessions(expires_at) WHERE is_revoked = false;
CREATE INDEX IF NOT EXISTS idx_sessions_revoked ON active_sessions(is_revoked, revoked_at);

-- ============================================
-- SESSION BLACKLIST TABLE
-- ============================================
-- For fast revocation checks (JWT 'jti' blacklist)
-- Entries expire after max token lifetime

CREATE TABLE IF NOT EXISTS session_blacklist (
    token_hash VARCHAR(64) PRIMARY KEY,
    blacklisted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    reason VARCHAR(100) NOT NULL,
    blacklisted_by VARCHAR(255)
);

CREATE INDEX IF NOT EXISTS idx_blacklist_expires ON session_blacklist(expires_at);

-- ============================================
-- AUDIT LOG TABLE
-- ============================================
-- Immutable audit log for compliance and forensics

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL PRIMARY KEY,

    -- Event identification
    event_type VARCHAR(100) NOT NULL,        -- e.g., 'auth.login', 'user.update', 'permission.change'
    event_category VARCHAR(50) NOT NULL,     -- 'auth', 'user', 'permission', 'content', 'finance', 'system'

    -- Actor (who performed the action)
    actor_type VARCHAR(50) NOT NULL,         -- 'user', 'system', 'worker', 'admin'
    actor_id VARCHAR(255),                   -- User ID or system identifier
    actor_ip INET,
    actor_user_agent TEXT,

    -- Target (what was affected)
    resource_type VARCHAR(100),              -- e.g., 'user', 'session', 'license', 'content'
    resource_id VARCHAR(255),

    -- Event details
    action VARCHAR(50) NOT NULL,             -- 'create', 'read', 'update', 'delete', 'login', 'logout'
    outcome VARCHAR(20) NOT NULL,            -- 'success', 'failure', 'denied', 'error'
    outcome_reason TEXT,

    -- Event data (structured)
    event_data JSONB,                        -- Additional context (changes, before/after, etc.)

    -- Timestamps
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Immutability marker
    is_immutable BOOLEAN NOT NULL DEFAULT true
);

-- Partition-friendly indexes (by time)
CREATE INDEX IF NOT EXISTS idx_audit_log_time ON audit_log(occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_type ON audit_log(event_type, occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_actor ON audit_log(actor_id, occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_resource ON audit_log(resource_type, resource_id, occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_category ON audit_log(event_category, occurred_at DESC);

-- ============================================
-- LAUNCH GATES TABLE
-- ============================================
-- Server-enforced pause controls for features
-- Phase 2 (P2-07)

CREATE TABLE IF NOT EXISTS launch_gates (
    id SERIAL PRIMARY KEY,

    -- Gate identification
    gate_name VARCHAR(100) UNIQUE NOT NULL,  -- e.g., 'issuance', 'marketplace', 'funding', 'referral_payments'
    gate_description TEXT,

    -- Current state
    is_enabled BOOLEAN NOT NULL DEFAULT false,

    -- State change tracking
    enabled_at TIMESTAMPTZ,
    enabled_by VARCHAR(255),
    enabled_reason TEXT,

    disabled_at TIMESTAMPTZ,
    disabled_by VARCHAR(255),
    disabled_reason TEXT,

    -- Expiry (auto-disable)
    expires_at TIMESTAMPTZ,

    -- Evidence (for compliance)
    evidence_url TEXT,
    evidence_notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- LAUNCH GATE HISTORY TABLE
-- ============================================
-- Immutable history of gate state changes

CREATE TABLE IF NOT EXISTS launch_gate_history (
    id BIGSERIAL PRIMARY KEY,
    gate_id INT NOT NULL REFERENCES launch_gates(id) ON DELETE CASCADE,

    -- State change
    previous_state BOOLEAN,
    new_state BOOLEAN NOT NULL,

    -- Actor
    changed_by VARCHAR(255) NOT NULL,
    change_reason TEXT NOT NULL,

    -- Evidence
    evidence_url TEXT,

    -- Timestamp
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_gate_history_gate ON launch_gate_history(gate_id, changed_at DESC);

-- ============================================
-- SEED DEFAULT LAUNCH GATES
-- ============================================
-- All gates start disabled (safe by default)

INSERT INTO launch_gates (gate_name, gate_description, is_enabled, disabled_reason)
VALUES
    ('issuance', 'License issuance and marketplace claims', false, 'Initial deployment - awaiting P4 gate'),
    ('marketplace', 'Public marketplace listing', false, 'Initial deployment - awaiting P4 gate'),
    ('funding', 'Credit funding and payments', false, 'Initial deployment - awaiting P5 gate'),
    ('referral_payments', 'Referral commission payments', false, 'Initial deployment - awaiting P5 gate'),
    ('campaigns', 'Marketing campaigns and outreach', false, 'Initial deployment - awaiting P8 gate'),
    ('uploads', 'User file uploads', false, 'Initial deployment - awaiting P6 gate')
ON CONFLICT (gate_name) DO NOTHING;

-- ============================================
-- HELPER FUNCTIONS
-- ============================================

-- Function to clean expired blacklist entries
CREATE OR REPLACE FUNCTION cleanup_expired_blacklist()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM session_blacklist WHERE expires_at < NOW();
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to clean expired sessions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS INTEGER AS $$
DECLARE
    updated_count INTEGER;
BEGIN
    UPDATE active_sessions
    SET is_revoked = true,
        revoked_at = NOW(),
        revocation_reason = 'expired'
    WHERE expires_at < NOW() AND is_revoked = false;
    GET DIAGNOSTICS updated_count = ROW_COUNT;
    RETURN updated_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE user_identities IS 'User identity records linked to external providers via JWT sub claim';
COMMENT ON TABLE active_sessions IS 'Active user sessions with revocation support';
COMMENT ON TABLE session_blacklist IS 'Fast lookup table for revoked JWT tokens';
COMMENT ON TABLE audit_log IS 'Immutable audit log for compliance and forensics';
COMMENT ON TABLE launch_gates IS 'Server-enforced feature toggles with evidence tracking';
COMMENT ON TABLE launch_gate_history IS 'Immutable history of launch gate state changes';
