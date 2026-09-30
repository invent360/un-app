-- Migration 00030: Ownership Validation
-- Phase 4 (P4-05): Ownership checks and gate validation at issuance
--
-- Provides:
-- - License ownership tracking
-- - Identity verification requirements
-- - Gate validation audit trail
-- - Ownership transfer history

-- ============================================
-- LICENSE OWNERSHIP TABLE
-- ============================================
-- Tracks full ownership chain for each license

CREATE TABLE IF NOT EXISTS license_ownership (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,

    -- Owner identity
    owner_id VARCHAR(255) NOT NULL,           -- User ID or service identity
    owner_type VARCHAR(20) NOT NULL,          -- 'user', 'service', 'agent', 'system'

    -- Ownership period
    owned_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    owned_until TIMESTAMPTZ,                  -- NULL = current owner

    -- How ownership was acquired
    acquisition_type VARCHAR(30) NOT NULL,    -- 'issuance', 'transfer', 'claim', 'assignment'
    acquisition_source VARCHAR(255),          -- Previous owner ID or source system

    -- Verification at acquisition
    verified_identity BOOLEAN NOT NULL DEFAULT FALSE,
    verification_method VARCHAR(50),          -- 'sms', 'email', 'kyc', 'none'
    verification_id VARCHAR(255),             -- Reference to verification record

    -- Gate state at acquisition
    gates_checked JSONB,                      -- Which gates were validated

    -- Metadata
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding current owner
CREATE INDEX IF NOT EXISTS idx_license_ownership_current
ON license_ownership (license_id, owned_until)
WHERE owned_until IS NULL;

-- Index for owner history
CREATE INDEX IF NOT EXISTS idx_license_ownership_owner
ON license_ownership (owner_id, owned_from DESC);

-- Unique constraint: only one current owner per license
CREATE UNIQUE INDEX IF NOT EXISTS idx_license_ownership_unique_current
ON license_ownership (license_id)
WHERE owned_until IS NULL;

-- ============================================
-- GATE VALIDATION LOG
-- ============================================
-- Audit trail of gate checks for issuance operations

CREATE TABLE IF NOT EXISTS gate_validation_log (
    id BIGSERIAL PRIMARY KEY,

    -- What was being validated
    operation_type VARCHAR(30) NOT NULL,      -- 'issuance', 'reservation', 'claim', 'transfer'
    license_id VARCHAR(66),
    user_id VARCHAR(255),

    -- Which gates were checked
    gates_required TEXT[] NOT NULL,           -- Array of gate names required
    gates_passed TEXT[] NOT NULL,             -- Gates that passed
    gates_failed TEXT[],                      -- Gates that failed (if any)

    -- Result
    all_passed BOOLEAN NOT NULL,
    failure_reason TEXT,

    -- Context
    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding validations by license
CREATE INDEX IF NOT EXISTS idx_gate_validation_license
ON gate_validation_log (license_id, created_at DESC);

-- Index for finding failures
CREATE INDEX IF NOT EXISTS idx_gate_validation_failures
ON gate_validation_log (all_passed, created_at DESC)
WHERE all_passed = FALSE;

-- ============================================
-- VERIFICATION REQUIREMENTS TABLE
-- ============================================
-- Configurable verification requirements per context

CREATE TABLE IF NOT EXISTS verification_requirements (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,

    -- When this requirement applies
    applies_to VARCHAR(50) NOT NULL,          -- 'issuance', 'claim', 'transfer', 'all'

    -- What verification is needed
    require_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    require_phone_verified BOOLEAN NOT NULL DEFAULT FALSE,
    require_kyc_completed BOOLEAN NOT NULL DEFAULT FALSE,
    require_2fa_enabled BOOLEAN NOT NULL DEFAULT FALSE,

    -- Minimum account age
    min_account_age_days INT DEFAULT 0,

    -- Geographic restrictions
    allowed_countries TEXT[],
    blocked_countries TEXT[],

    -- Active status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default verification requirement
INSERT INTO verification_requirements (
    name, description, applies_to,
    require_email_verified, require_phone_verified
) VALUES (
    'standard_issuance',
    'Standard verification for license issuance',
    'issuance',
    TRUE, FALSE
) ON CONFLICT (name) DO NOTHING;

-- Strict verification for high-value operations
INSERT INTO verification_requirements (
    name, description, applies_to,
    require_email_verified, require_phone_verified, require_kyc_completed,
    min_account_age_days
) VALUES (
    'strict_issuance',
    'Strict verification for premium licenses',
    'issuance',
    TRUE, TRUE, FALSE,
    7
) ON CONFLICT (name) DO NOTHING;

-- ============================================
-- LICENSE VERIFICATION COLUMNS
-- ============================================

-- Track verification state on licenses
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS verification_requirement_id INT
    REFERENCES verification_requirements(id);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS owner_verified BOOLEAN DEFAULT FALSE;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS owner_verification_at TIMESTAMPTZ;

-- ============================================
-- FUNCTIONS
-- ============================================

-- Function to check if all required gates are open
CREATE OR REPLACE FUNCTION check_gates_for_operation(
    p_gates TEXT[],
    p_operation_type VARCHAR(30),
    p_license_id VARCHAR(66),
    p_user_id VARCHAR(255)
) RETURNS BOOLEAN AS $$
DECLARE
    v_gate TEXT;
    v_passed TEXT[] := ARRAY[]::TEXT[];
    v_failed TEXT[] := ARRAY[]::TEXT[];
    v_is_enabled BOOLEAN;
BEGIN
    FOREACH v_gate IN ARRAY p_gates
    LOOP
        SELECT is_enabled INTO v_is_enabled
        FROM launch_gates
        WHERE gate_name = v_gate
          AND (expires_at IS NULL OR expires_at > NOW());

        IF v_is_enabled IS TRUE THEN
            v_passed := array_append(v_passed, v_gate);
        ELSE
            v_failed := array_append(v_failed, v_gate);
        END IF;
    END LOOP;

    -- Log the validation
    INSERT INTO gate_validation_log (
        operation_type, license_id, user_id,
        gates_required, gates_passed, gates_failed,
        all_passed, failure_reason
    ) VALUES (
        p_operation_type, p_license_id, p_user_id,
        p_gates, v_passed, v_failed,
        array_length(v_failed, 1) IS NULL,
        CASE WHEN array_length(v_failed, 1) IS NOT NULL
             THEN 'Gates failed: ' || array_to_string(v_failed, ', ')
             ELSE NULL END
    );

    RETURN array_length(v_failed, 1) IS NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to establish ownership
CREATE OR REPLACE FUNCTION establish_ownership(
    p_license_id VARCHAR(66),
    p_owner_id VARCHAR(255),
    p_owner_type VARCHAR(20),
    p_acquisition_type VARCHAR(30),
    p_verified BOOLEAN DEFAULT FALSE,
    p_verification_method VARCHAR(50) DEFAULT NULL,
    p_gates_checked JSONB DEFAULT NULL
) RETURNS BIGINT AS $$
DECLARE
    v_ownership_id BIGINT;
BEGIN
    -- Close any existing ownership
    UPDATE license_ownership
    SET owned_until = NOW()
    WHERE license_id = p_license_id AND owned_until IS NULL;

    -- Create new ownership record
    INSERT INTO license_ownership (
        license_id, owner_id, owner_type,
        acquisition_type, verified_identity, verification_method,
        gates_checked
    ) VALUES (
        p_license_id, p_owner_id, p_owner_type,
        p_acquisition_type, p_verified, p_verification_method,
        p_gates_checked
    ) RETURNING id INTO v_ownership_id;

    -- Update license ownership tracking
    UPDATE licenses
    SET issued_to = p_owner_id,
        owner_verified = p_verified,
        owner_verification_at = CASE WHEN p_verified THEN NOW() ELSE NULL END
    WHERE id = p_license_id;

    RETURN v_ownership_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE license_ownership IS 'Complete ownership history for each license';
COMMENT ON COLUMN license_ownership.owner_type IS 'user, service, agent, or system';
COMMENT ON COLUMN license_ownership.acquisition_type IS 'How ownership was acquired: issuance, transfer, claim, assignment';
COMMENT ON COLUMN license_ownership.gates_checked IS 'JSON of gate states at acquisition time';

COMMENT ON TABLE gate_validation_log IS 'Audit trail of gate checks for operations';
COMMENT ON COLUMN gate_validation_log.gates_required IS 'All gates that were required to pass';
COMMENT ON COLUMN gate_validation_log.gates_failed IS 'Gates that blocked the operation';

COMMENT ON TABLE verification_requirements IS 'Configurable verification requirements';
COMMENT ON COLUMN verification_requirements.applies_to IS 'Operation type: issuance, claim, transfer, or all';

COMMENT ON FUNCTION check_gates_for_operation IS 'Check if required gates are open, logs result';
COMMENT ON FUNCTION establish_ownership IS 'Transfer or establish license ownership with audit';
