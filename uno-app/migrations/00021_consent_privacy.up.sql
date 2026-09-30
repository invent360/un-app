-- Migration 00021: Consent and Privacy Controls
-- Phase 2 (P2-06): Implement versioned consent, retention policies, and privacy workflows

-- ============================================
-- CONSENT VERSIONS TABLE
-- ============================================
-- Tracks versions of consent documents (terms, privacy policy, etc.)

CREATE TABLE IF NOT EXISTS consent_versions (
    id SERIAL PRIMARY KEY,
    consent_type VARCHAR(50) NOT NULL,       -- 'terms_of_service', 'privacy_policy', 'marketing', 'data_sharing'
    version VARCHAR(20) NOT NULL,            -- Semantic version: '1.0.0'
    title VARCHAR(255) NOT NULL,
    content_hash VARCHAR(64) NOT NULL,       -- SHA256 of content for integrity
    content_url TEXT,                        -- URL to full document
    summary TEXT,                            -- Brief summary of changes

    -- Effective dates
    effective_from TIMESTAMPTZ NOT NULL,
    effective_to TIMESTAMPTZ,                -- NULL = currently active

    -- Requirements
    requires_explicit_consent BOOLEAN NOT NULL DEFAULT true,
    is_mandatory BOOLEAN NOT NULL DEFAULT true,  -- Must accept to use service

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255) NOT NULL,

    CONSTRAINT uq_consent_version UNIQUE (consent_type, version)
);

CREATE INDEX IF NOT EXISTS idx_consent_versions_type ON consent_versions(consent_type);
CREATE INDEX IF NOT EXISTS idx_consent_versions_active ON consent_versions(consent_type, effective_from, effective_to);

-- ============================================
-- USER CONSENTS TABLE
-- ============================================
-- Records user consent decisions (immutable)

CREATE TABLE IF NOT EXISTS user_consents (
    id BIGSERIAL PRIMARY KEY,

    -- User reference
    user_id UUID NOT NULL REFERENCES user_identities(id) ON DELETE CASCADE,

    -- Consent version
    consent_version_id INT NOT NULL REFERENCES consent_versions(id),

    -- Decision
    consented BOOLEAN NOT NULL,              -- true = accepted, false = declined
    consent_method VARCHAR(50) NOT NULL,     -- 'explicit_click', 'checkbox', 'implicit', 'withdrawal'

    -- Context
    ip_address INET,
    user_agent TEXT,
    session_id UUID,

    -- Timestamp (immutable)
    consented_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- For withdrawal tracking
    withdrawn_at TIMESTAMPTZ,
    withdrawal_reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_user_consents_user ON user_consents(user_id, consented_at DESC);
CREATE INDEX IF NOT EXISTS idx_user_consents_version ON user_consents(consent_version_id);
CREATE INDEX IF NOT EXISTS idx_user_consents_active ON user_consents(user_id, consent_version_id)
    WHERE withdrawn_at IS NULL;

-- ============================================
-- DATA RETENTION POLICIES TABLE
-- ============================================
-- Defines retention periods for different data categories

CREATE TABLE IF NOT EXISTS data_retention_policies (
    id SERIAL PRIMARY KEY,

    -- Category identification
    data_category VARCHAR(100) UNIQUE NOT NULL,  -- 'operational', 'marketing', 'audit', 'financial', 'support'
    description TEXT,

    -- Retention rules
    retention_days INT NOT NULL,                 -- Number of days to retain
    anonymize_after_days INT,                    -- Days before anonymization (NULL = delete)
    requires_consent BOOLEAN NOT NULL DEFAULT false,

    -- Exceptions
    legal_hold_exempt BOOLEAN NOT NULL DEFAULT false,  -- If true, can be extended by legal hold

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(255)
);

-- Seed default retention policies
INSERT INTO data_retention_policies (data_category, description, retention_days, anonymize_after_days, requires_consent, legal_hold_exempt)
VALUES
    ('operational', 'Core operational data required for service', 365 * 7, NULL, false, true),
    ('financial', 'Financial records and transactions', 365 * 7, NULL, false, true),
    ('audit', 'Security audit logs', 365 * 3, NULL, false, true),
    ('marketing', 'Marketing preferences and interactions', 365 * 2, 365, true, false),
    ('support', 'Support tickets and communications', 365 * 3, 365 * 5, false, false),
    ('analytics', 'Usage analytics and metrics', 365 * 2, 90, true, false),
    ('session', 'Session and authentication data', 90, NULL, false, false)
ON CONFLICT (data_category) DO NOTHING;

-- ============================================
-- DATA ACCESS REQUESTS TABLE
-- ============================================
-- Tracks GDPR/privacy data access requests

CREATE TABLE IF NOT EXISTS data_access_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Requester
    user_id UUID NOT NULL REFERENCES user_identities(id) ON DELETE CASCADE,

    -- Request details
    request_type VARCHAR(50) NOT NULL,        -- 'export', 'delete', 'rectify', 'restrict'
    request_reason TEXT,

    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',  -- 'pending', 'processing', 'completed', 'rejected'

    -- Processing
    assigned_to VARCHAR(255),
    processing_notes TEXT,

    -- Completion
    completed_at TIMESTAMPTZ,
    completed_by VARCHAR(255),
    completion_notes TEXT,

    -- For exports
    export_url TEXT,
    export_expires_at TIMESTAMPTZ,

    -- For rejections
    rejection_reason TEXT,

    -- Timestamps
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Compliance tracking
    regulatory_deadline TIMESTAMPTZ,          -- GDPR requires response within 30 days

    CONSTRAINT valid_request_type CHECK (
        request_type IN ('export', 'delete', 'rectify', 'restrict')
    ),
    CONSTRAINT valid_status CHECK (
        status IN ('pending', 'processing', 'completed', 'rejected', 'cancelled')
    )
);

CREATE INDEX IF NOT EXISTS idx_data_requests_user ON data_access_requests(user_id);
CREATE INDEX IF NOT EXISTS idx_data_requests_status ON data_access_requests(status, requested_at);
CREATE INDEX IF NOT EXISTS idx_data_requests_deadline ON data_access_requests(regulatory_deadline)
    WHERE status IN ('pending', 'processing');

-- ============================================
-- DATA DELETION LOG TABLE
-- ============================================
-- Audit trail for data deletions (immutable)

CREATE TABLE IF NOT EXISTS data_deletion_log (
    id BIGSERIAL PRIMARY KEY,

    -- Request reference
    request_id UUID REFERENCES data_access_requests(id),

    -- What was deleted
    data_category VARCHAR(100) NOT NULL,
    table_name VARCHAR(100) NOT NULL,
    record_count INT NOT NULL,

    -- Actor
    deleted_by VARCHAR(255) NOT NULL,
    deletion_method VARCHAR(50) NOT NULL,     -- 'hard_delete', 'soft_delete', 'anonymize'

    -- Timestamp
    deleted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_deletion_log_request ON data_deletion_log(request_id);
CREATE INDEX IF NOT EXISTS idx_deletion_log_time ON data_deletion_log(deleted_at DESC);

-- ============================================
-- SEED DEFAULT CONSENT VERSIONS
-- ============================================

INSERT INTO consent_versions (consent_type, version, title, content_hash, effective_from, requires_explicit_consent, is_mandatory, created_by)
VALUES
    ('terms_of_service', '1.0.0', 'Terms of Service', 'placeholder_hash_tos_v1', NOW(), true, true, 'system'),
    ('privacy_policy', '1.0.0', 'Privacy Policy', 'placeholder_hash_privacy_v1', NOW(), true, true, 'system'),
    ('marketing', '1.0.0', 'Marketing Communications', 'placeholder_hash_marketing_v1', NOW(), true, false, 'system')
ON CONFLICT (consent_type, version) DO NOTHING;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE consent_versions IS 'Versioned consent documents for regulatory compliance';
COMMENT ON TABLE user_consents IS 'Immutable record of user consent decisions';
COMMENT ON TABLE data_retention_policies IS 'Data category retention rules for privacy compliance';
COMMENT ON TABLE data_access_requests IS 'GDPR/privacy data subject requests (export, delete, etc.)';
COMMENT ON TABLE data_deletion_log IS 'Immutable audit trail of data deletions';
