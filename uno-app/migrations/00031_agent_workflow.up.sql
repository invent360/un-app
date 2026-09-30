-- Migration 00031: Agent Approval/Rejection/Suspension Workflow
-- Phase 4 (P4-06): Agent lifecycle management
--
-- Provides:
-- - Agent status lifecycle (pending -> approved/rejected -> suspended)
-- - Approval and rejection workflow
-- - Suspension with reason and duration
-- - Status change audit trail

-- ============================================
-- AGENT STATUS ENUM
-- ============================================

DO $$ BEGIN
    CREATE TYPE agent_status AS ENUM (
        'pending_approval',  -- Newly registered, awaiting review
        'approved',          -- Active and can operate
        'rejected',          -- Application rejected
        'suspended',         -- Temporarily suspended
        'terminated'         -- Permanently terminated
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- AGENT WORKFLOW COLUMNS
-- ============================================

-- Status (replaces simple is_active)
ALTER TABLE agents ADD COLUMN IF NOT EXISTS status agent_status DEFAULT 'pending_approval';

-- Approval workflow
ALTER TABLE agents ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS approved_by VARCHAR(255);
ALTER TABLE agents ADD COLUMN IF NOT EXISTS approval_notes TEXT;

-- Rejection workflow
ALTER TABLE agents ADD COLUMN IF NOT EXISTS rejected_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS rejected_by VARCHAR(255);
ALTER TABLE agents ADD COLUMN IF NOT EXISTS rejection_reason TEXT;

-- Suspension workflow
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspended_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspended_by VARCHAR(255);
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspension_reason TEXT;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspension_ends_at TIMESTAMPTZ;  -- NULL = indefinite
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspension_lifted_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS suspension_lifted_by VARCHAR(255);

-- Termination
ALTER TABLE agents ADD COLUMN IF NOT EXISTS terminated_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS terminated_by VARCHAR(255);
ALTER TABLE agents ADD COLUMN IF NOT EXISTS termination_reason TEXT;

-- Additional metadata
ALTER TABLE agents ADD COLUMN IF NOT EXISTS onboarding_completed BOOLEAN DEFAULT FALSE;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS onboarding_completed_at TIMESTAMPTZ;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS license_count INT DEFAULT 0;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS last_activity_at TIMESTAMPTZ;

-- Sync existing is_active to status
UPDATE agents
SET status =
    CASE
        WHEN is_active = false AND deleted_at IS NOT NULL THEN 'terminated'::agent_status
        WHEN is_active = true THEN 'approved'::agent_status
        ELSE 'pending_approval'::agent_status
    END
WHERE status IS NULL;

-- Index for status queries
CREATE INDEX IF NOT EXISTS idx_agents_status ON agents(status);

-- Index for pending approvals
CREATE INDEX IF NOT EXISTS idx_agents_pending ON agents(created_at DESC)
WHERE status = 'pending_approval';

-- Index for suspended agents with expiry
CREATE INDEX IF NOT EXISTS idx_agents_suspension_expiry ON agents(suspension_ends_at)
WHERE status = 'suspended' AND suspension_ends_at IS NOT NULL;

-- ============================================
-- AGENT STATUS HISTORY TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS agent_status_history (
    id BIGSERIAL PRIMARY KEY,
    agent_id VARCHAR(36) NOT NULL REFERENCES agents(id),

    -- Status change
    from_status agent_status,
    to_status agent_status NOT NULL,

    -- Who made the change
    changed_by VARCHAR(255) NOT NULL,
    change_reason TEXT,

    -- Additional context
    metadata JSONB,
    ip_address INET,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for history by agent
CREATE INDEX IF NOT EXISTS idx_agent_status_history_agent
ON agent_status_history (agent_id, created_at DESC);

-- Index for admin actions
CREATE INDEX IF NOT EXISTS idx_agent_status_history_admin
ON agent_status_history (changed_by, created_at DESC);

-- ============================================
-- AGENT APPLICATION TABLE
-- ============================================
-- Stores original application data for review

CREATE TABLE IF NOT EXISTS agent_applications (
    id BIGSERIAL PRIMARY KEY,
    agent_id VARCHAR(36) NOT NULL REFERENCES agents(id),

    -- Application data
    business_name VARCHAR(255),
    business_type VARCHAR(100),
    business_registration VARCHAR(100),
    tax_id VARCHAR(100),
    business_address TEXT,
    business_phone VARCHAR(50),
    website VARCHAR(255),

    -- Contact person
    contact_name VARCHAR(255),
    contact_position VARCHAR(100),
    contact_phone VARCHAR(50),
    contact_email VARCHAR(255),

    -- License plan
    requested_license_count INT DEFAULT 0,
    requested_split_type VARCHAR(20),

    -- Documents (references to file storage)
    documents JSONB,

    -- Review
    reviewer_id VARCHAR(255),
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,

    -- Metadata
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT
);

-- Index for applications by agent
CREATE UNIQUE INDEX IF NOT EXISTS idx_agent_applications_agent
ON agent_applications (agent_id);

-- ============================================
-- FUNCTIONS
-- ============================================

-- Function to approve an agent
CREATE OR REPLACE FUNCTION approve_agent(
    p_agent_id VARCHAR(36),
    p_approver_id VARCHAR(255),
    p_notes TEXT DEFAULT NULL
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_status agent_status;
BEGIN
    -- Get current status with lock
    SELECT status INTO v_current_status
    FROM agents WHERE id = p_agent_id FOR UPDATE;

    IF v_current_status IS NULL THEN
        RAISE EXCEPTION 'Agent not found: %', p_agent_id;
    END IF;

    IF v_current_status != 'pending_approval' THEN
        RAISE EXCEPTION 'Agent % is not pending approval (current: %)', p_agent_id, v_current_status;
    END IF;

    -- Update agent
    UPDATE agents SET
        status = 'approved',
        is_active = TRUE,
        approved_at = NOW(),
        approved_by = p_approver_id,
        approval_notes = p_notes,
        updated_at = NOW()
    WHERE id = p_agent_id;

    -- Log status change
    INSERT INTO agent_status_history (agent_id, from_status, to_status, changed_by, change_reason)
    VALUES (p_agent_id, v_current_status, 'approved', p_approver_id, COALESCE(p_notes, 'Agent approved'));

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Function to reject an agent
CREATE OR REPLACE FUNCTION reject_agent(
    p_agent_id VARCHAR(36),
    p_rejector_id VARCHAR(255),
    p_reason TEXT
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_status agent_status;
BEGIN
    SELECT status INTO v_current_status
    FROM agents WHERE id = p_agent_id FOR UPDATE;

    IF v_current_status IS NULL THEN
        RAISE EXCEPTION 'Agent not found: %', p_agent_id;
    END IF;

    IF v_current_status != 'pending_approval' THEN
        RAISE EXCEPTION 'Agent % is not pending approval (current: %)', p_agent_id, v_current_status;
    END IF;

    UPDATE agents SET
        status = 'rejected',
        is_active = FALSE,
        rejected_at = NOW(),
        rejected_by = p_rejector_id,
        rejection_reason = p_reason,
        updated_at = NOW()
    WHERE id = p_agent_id;

    INSERT INTO agent_status_history (agent_id, from_status, to_status, changed_by, change_reason)
    VALUES (p_agent_id, v_current_status, 'rejected', p_rejector_id, p_reason);

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Function to suspend an agent
CREATE OR REPLACE FUNCTION suspend_agent(
    p_agent_id VARCHAR(36),
    p_suspender_id VARCHAR(255),
    p_reason TEXT,
    p_duration_days INT DEFAULT NULL  -- NULL = indefinite
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_status agent_status;
    v_ends_at TIMESTAMPTZ;
BEGIN
    SELECT status INTO v_current_status
    FROM agents WHERE id = p_agent_id FOR UPDATE;

    IF v_current_status IS NULL THEN
        RAISE EXCEPTION 'Agent not found: %', p_agent_id;
    END IF;

    IF v_current_status NOT IN ('approved', 'suspended') THEN
        RAISE EXCEPTION 'Agent % cannot be suspended (current: %)', p_agent_id, v_current_status;
    END IF;

    v_ends_at := CASE WHEN p_duration_days IS NOT NULL
                      THEN NOW() + (p_duration_days || ' days')::INTERVAL
                      ELSE NULL END;

    UPDATE agents SET
        status = 'suspended',
        is_active = FALSE,
        suspended_at = NOW(),
        suspended_by = p_suspender_id,
        suspension_reason = p_reason,
        suspension_ends_at = v_ends_at,
        suspension_lifted_at = NULL,
        suspension_lifted_by = NULL,
        updated_at = NOW()
    WHERE id = p_agent_id;

    INSERT INTO agent_status_history (
        agent_id, from_status, to_status, changed_by, change_reason, metadata
    ) VALUES (
        p_agent_id, v_current_status, 'suspended', p_suspender_id, p_reason,
        jsonb_build_object('duration_days', p_duration_days, 'ends_at', v_ends_at)
    );

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Function to lift suspension
CREATE OR REPLACE FUNCTION lift_agent_suspension(
    p_agent_id VARCHAR(36),
    p_lifter_id VARCHAR(255),
    p_reason TEXT DEFAULT 'Suspension lifted'
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_status agent_status;
BEGIN
    SELECT status INTO v_current_status
    FROM agents WHERE id = p_agent_id FOR UPDATE;

    IF v_current_status IS NULL THEN
        RAISE EXCEPTION 'Agent not found: %', p_agent_id;
    END IF;

    IF v_current_status != 'suspended' THEN
        RAISE EXCEPTION 'Agent % is not suspended (current: %)', p_agent_id, v_current_status;
    END IF;

    UPDATE agents SET
        status = 'approved',
        is_active = TRUE,
        suspension_lifted_at = NOW(),
        suspension_lifted_by = p_lifter_id,
        updated_at = NOW()
    WHERE id = p_agent_id;

    INSERT INTO agent_status_history (agent_id, from_status, to_status, changed_by, change_reason)
    VALUES (p_agent_id, v_current_status, 'approved', p_lifter_id, p_reason);

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Function to auto-lift expired suspensions
CREATE OR REPLACE FUNCTION auto_lift_expired_suspensions()
RETURNS INT AS $$
DECLARE
    v_count INT := 0;
    v_agent RECORD;
BEGIN
    FOR v_agent IN
        SELECT id FROM agents
        WHERE status = 'suspended'
          AND suspension_ends_at IS NOT NULL
          AND suspension_ends_at <= NOW()
        FOR UPDATE SKIP LOCKED
    LOOP
        PERFORM lift_agent_suspension(v_agent.id, 'system', 'Suspension period expired');
        v_count := v_count + 1;
    END LOOP;

    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TYPE agent_status IS 'Agent lifecycle status: pending_approval, approved, rejected, suspended, terminated';

COMMENT ON COLUMN agents.status IS 'Current status in agent lifecycle';
COMMENT ON COLUMN agents.suspension_ends_at IS 'When suspension automatically ends (NULL = indefinite)';
COMMENT ON COLUMN agents.onboarding_completed IS 'Whether agent has completed onboarding steps';

COMMENT ON TABLE agent_status_history IS 'Audit trail of all agent status changes';
COMMENT ON TABLE agent_applications IS 'Original agent application data for review';

COMMENT ON FUNCTION approve_agent IS 'Approve a pending agent application';
COMMENT ON FUNCTION reject_agent IS 'Reject a pending agent application';
COMMENT ON FUNCTION suspend_agent IS 'Suspend an active agent with optional duration';
COMMENT ON FUNCTION lift_agent_suspension IS 'Lift an agent suspension';
COMMENT ON FUNCTION auto_lift_expired_suspensions IS 'Automatically lift expired suspensions (run periodically)';
