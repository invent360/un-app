-- Migration 00062: R5-07 Finance Authoritative Improvements
-- Implements:
-- 1. Change ON DELETE CASCADE to RESTRICT for financial history preservation
-- 2. Add alert_policies table for configurable thresholds
-- 3. Add quarantine_status for ambiguous manual allocations
-- 4. Add reward_source tracking for event uniqueness

-- ============================================
-- PRESERVE FINANCIAL HISTORY
-- ============================================
-- R5-07: Remove financial-history cascade deletion with licence deletion
-- Privacy erasure must not erase earned obligations

-- Drop and recreate constraint on allocation_ledger
ALTER TABLE allocation_ledger
DROP CONSTRAINT IF EXISTS allocation_ledger_license_id_fkey;

ALTER TABLE allocation_ledger
ADD CONSTRAINT allocation_ledger_license_id_fkey
FOREIGN KEY (license_id) REFERENCES licenses(id) ON DELETE RESTRICT;

COMMENT ON CONSTRAINT allocation_ledger_license_id_fkey ON allocation_ledger IS
'R5-07: Financial history preserved on license deletion; use compensating entries';

-- Drop and recreate constraint on uno_credit_expenditure
ALTER TABLE uno_credit_expenditure
DROP CONSTRAINT IF EXISTS uno_credit_expenditure_license_id_fkey;

ALTER TABLE uno_credit_expenditure
ADD CONSTRAINT uno_credit_expenditure_license_id_fkey
FOREIGN KEY (license_id) REFERENCES licenses(id) ON DELETE RESTRICT;

COMMENT ON CONSTRAINT uno_credit_expenditure_license_id_fkey ON uno_credit_expenditure IS
'R5-07: Credit expenditure preserved on license deletion';

-- ============================================
-- CONFIGURABLE ALERT POLICIES
-- ============================================
-- R5-07: Replace hard-coded $5.60 threshold with configurable policy

CREATE TABLE IF NOT EXISTS alert_policies (
    id SERIAL PRIMARY KEY,

    -- Policy identification
    policy_code VARCHAR(50) NOT NULL UNIQUE,
    policy_name VARCHAR(100) NOT NULL,
    description TEXT,

    -- Policy parameters (JSONB for flexibility)
    parameters JSONB NOT NULL DEFAULT '{}',

    -- Threshold values (common pattern)
    threshold_micros BIGINT,
    threshold_percent NUMERIC(5,2),

    -- Period configuration
    period_type VARCHAR(20) CHECK (period_type IN ('lifetime', 'monthly', 'weekly', 'daily', 'rolling_days')),
    period_value INT, -- e.g., 30 for rolling_days

    -- Basis configuration (what the threshold applies to)
    basis VARCHAR(50), -- e.g., 'uno_net', 'ulo_earnings', 'pool_total'

    -- State
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    -- Versioning for audit
    version INT NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_alert_policies_code ON alert_policies(policy_code);
CREATE INDEX IF NOT EXISTS idx_alert_policies_active ON alert_policies(is_active) WHERE is_active = true;

COMMENT ON TABLE alert_policies IS 'R5-07: Configurable alert thresholds replacing hardcoded values';
COMMENT ON COLUMN alert_policies.threshold_micros IS 'Threshold in micros (e.g., 5600000 for $5.60)';
COMMENT ON COLUMN alert_policies.basis IS 'What the threshold measures: uno_net, ulo_earnings, pool_total, etc.';

-- Insert the break-even policy (replacing hardcoded $5.60)
INSERT INTO alert_policies (
    policy_code, policy_name, description,
    threshold_micros, period_type, basis,
    parameters
) VALUES (
    'license_break_even',
    'License Break-Even Alert',
    'Alert when UNO net contribution is below break-even threshold. Based on: 40% UNO share, $1.99 credits, $0.25 support cost per license per month.',
    5600000, -- $5.60 in micros
    'monthly',
    'uno_net',
    '{"credit_cost_micros": 1990000, "support_cost_micros": 250000, "uno_share_bps": 4000}'::jsonb
) ON CONFLICT (policy_code) DO NOTHING;

-- ============================================
-- ALLOCATION QUARANTINE FOR MANUAL ENTRIES
-- ============================================
-- R5-07: Quarantine ambiguous manual rows and use authorized compensating entries

-- Add quarantine status enum
DO $$ BEGIN
    CREATE TYPE allocation_quarantine_status AS ENUM (
        'none',           -- Normal allocation (automated or validated)
        'pending_review', -- Requires manual review
        'quarantined',    -- Flagged as problematic
        'resolved',       -- Issue resolved
        'rejected'        -- Rejected as invalid
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Add columns for quarantine tracking
ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS quarantine_status allocation_quarantine_status NOT NULL DEFAULT 'none';

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS quarantine_reason TEXT;

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS quarantine_reviewed_by VARCHAR(255);

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS quarantine_reviewed_at TIMESTAMPTZ;

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS quarantine_resolution_notes TEXT;

-- Index for finding quarantined allocations
CREATE INDEX IF NOT EXISTS idx_allocation_quarantine
ON allocation_ledger (quarantine_status)
WHERE quarantine_status != 'none';

COMMENT ON COLUMN allocation_ledger.quarantine_status IS 'R5-07: Status for ambiguous manual allocations requiring review';
COMMENT ON COLUMN allocation_ledger.quarantine_reason IS 'R5-07: Reason for quarantine (e.g., missing provider_id, duplicate event)';

-- ============================================
-- REWARD SOURCE TRACKING
-- ============================================
-- R5-07: Define uniqueness granularity for events covering multiple licences

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS reward_source VARCHAR(50);

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS reward_batch_id UUID;

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS is_batch_member BOOLEAN NOT NULL DEFAULT false;

COMMENT ON COLUMN allocation_ledger.reward_source IS 'R5-07: Source type: automatic, manual, compensating, batch';
COMMENT ON COLUMN allocation_ledger.reward_batch_id IS 'R5-07: Groups allocations from single event covering multiple licenses';
COMMENT ON COLUMN allocation_ledger.is_batch_member IS 'R5-07: True if allocation is part of a batch reward event';

-- Create partial unique index for batch events
-- One event_id per batch_id ensures no duplicate processing of batched rewards
CREATE UNIQUE INDEX IF NOT EXISTS idx_ledger_batch_event_unique
ON allocation_ledger (reward_batch_id, reward_event_id)
WHERE reward_batch_id IS NOT NULL AND reward_event_id IS NOT NULL;

-- ============================================
-- WIRING: AGREEMENT VERSION TO ALLOCATION
-- ============================================
-- R5-07: Wire actual reward path to correct versioned agreement

-- Ensure agreement_version column exists and is required for new allocations
ALTER TABLE allocation_ledger
ALTER COLUMN agreement_version SET NOT NULL;

-- Add agreement snapshot to preserve exact terms at allocation time
ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS agreement_snapshot JSONB;

COMMENT ON COLUMN allocation_ledger.agreement_snapshot IS 'R5-07: Snapshot of agreement terms at allocation time for audit';

-- ============================================
-- WIRING: ATTRIBUTED RECIPIENT
-- ============================================
-- R5-07: Ensure referral_agent_id is properly linked to settlement recipient

-- Add recipient tracking for each party's settlement
ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS ulo_recipient_id UUID;

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS uno_recipient_id UUID;

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS reserve_recipient_id UUID;

COMMENT ON COLUMN allocation_ledger.referral_agent_id IS 'R5-07: Agent receiving referral share (frozen at allocation time)';
COMMENT ON COLUMN allocation_ledger.ulo_recipient_id IS 'R5-07: User/participant receiving ULO share';
COMMENT ON COLUMN allocation_ledger.uno_recipient_id IS 'R5-07: UNO operator entity (usually system)';
COMMENT ON COLUMN allocation_ledger.reserve_recipient_id IS 'R5-07: Reserve pool recipient (for no-referral allocations)';

-- ============================================
-- FUNCTION: CHECK ALERT POLICY
-- ============================================

CREATE OR REPLACE FUNCTION check_alert_policy(
    p_license_id VARCHAR(66),
    p_policy_code VARCHAR(50)
) RETURNS TABLE (
    is_triggered BOOLEAN,
    threshold_micros BIGINT,
    actual_value_micros BIGINT,
    policy_name VARCHAR(100)
) AS $$
DECLARE
    v_policy alert_policies;
    v_actual BIGINT;
BEGIN
    -- Get policy
    SELECT * INTO v_policy
    FROM alert_policies
    WHERE policy_code = p_policy_code AND is_active = true;

    IF NOT FOUND THEN
        RETURN QUERY SELECT false, 0::BIGINT, 0::BIGINT, 'Policy not found'::VARCHAR(100);
        RETURN;
    END IF;

    -- Calculate actual value based on basis
    IF v_policy.basis = 'uno_net' THEN
        SELECT COALESCE(SUM(al.uno_micros), 0) - COALESCE((
            SELECT SUM(ce.amount_micros)
            FROM uno_credit_expenditure ce
            WHERE ce.license_id = p_license_id
        ), 0)
        INTO v_actual
        FROM allocation_ledger al
        WHERE al.license_id = p_license_id;
    ELSIF v_policy.basis = 'ulo_earnings' THEN
        SELECT COALESCE(SUM(ulo_micros), 0)
        INTO v_actual
        FROM allocation_ledger
        WHERE license_id = p_license_id;
    ELSIF v_policy.basis = 'pool_total' THEN
        SELECT COALESCE(SUM(pool_micros), 0)
        INTO v_actual
        FROM allocation_ledger
        WHERE license_id = p_license_id;
    ELSE
        v_actual := 0;
    END IF;

    RETURN QUERY SELECT
        v_actual < v_policy.threshold_micros,
        v_policy.threshold_micros,
        v_actual,
        v_policy.policy_name;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION check_alert_policy IS 'R5-07: Check if license triggers an alert policy threshold';

-- ============================================
-- FUNCTION: QUARANTINE ALLOCATION
-- ============================================

CREATE OR REPLACE FUNCTION quarantine_allocation(
    p_allocation_id UUID,
    p_reason TEXT,
    p_reviewer_id VARCHAR(255) DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    UPDATE allocation_ledger
    SET quarantine_status = 'pending_review',
        quarantine_reason = p_reason,
        quarantine_reviewed_by = p_reviewer_id,
        updated_at = NOW()
    WHERE id = p_allocation_id;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION quarantine_allocation IS 'R5-07: Mark an allocation for manual review';

-- ============================================
-- FUNCTION: RESOLVE QUARANTINE
-- ============================================

CREATE OR REPLACE FUNCTION resolve_quarantine(
    p_allocation_id UUID,
    p_resolution allocation_quarantine_status,
    p_reviewer_id VARCHAR(255),
    p_notes TEXT DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    IF p_resolution NOT IN ('resolved', 'rejected') THEN
        RAISE EXCEPTION 'Resolution must be either resolved or rejected';
    END IF;

    UPDATE allocation_ledger
    SET quarantine_status = p_resolution,
        quarantine_reviewed_by = p_reviewer_id,
        quarantine_reviewed_at = NOW(),
        quarantine_resolution_notes = p_notes,
        updated_at = NOW()
    WHERE id = p_allocation_id;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION resolve_quarantine IS 'R5-07: Resolve a quarantined allocation';

-- ============================================
-- VIEW: ALLOCATIONS REQUIRING REVIEW
-- ============================================

CREATE OR REPLACE VIEW allocations_pending_review AS
SELECT
    al.id,
    al.license_id,
    al.pool_micros,
    al.ulo_micros,
    al.uno_micros,
    al.referral_micros,
    al.reserve_micros,
    al.quarantine_status,
    al.quarantine_reason,
    al.reward_source,
    al.provider_id,
    al.reward_event_id,
    al.created_at,
    l.lease_code
FROM allocation_ledger al
LEFT JOIN licenses l ON l.id = al.license_id
WHERE al.quarantine_status IN ('pending_review', 'quarantined')
ORDER BY al.created_at DESC;

COMMENT ON VIEW allocations_pending_review IS 'R5-07: Allocations requiring manual review';

-- ============================================
-- TRIGGER: AUTO-QUARANTINE MANUAL ALLOCATIONS
-- ============================================

CREATE OR REPLACE FUNCTION auto_quarantine_check()
RETURNS TRIGGER AS $$
BEGIN
    -- R5-07: Auto-quarantine allocations without provider provenance
    IF NEW.provider_id IS NULL AND NEW.reward_event_id IS NULL THEN
        IF NEW.reward_source IS NULL OR NEW.reward_source = 'manual' THEN
            NEW.quarantine_status := 'pending_review';
            NEW.quarantine_reason := 'Manual allocation without provider/event provenance';
            NEW.reward_source := COALESCE(NEW.reward_source, 'manual');
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_auto_quarantine ON allocation_ledger;
CREATE TRIGGER trg_auto_quarantine
    BEFORE INSERT ON allocation_ledger
    FOR EACH ROW
    EXECUTE FUNCTION auto_quarantine_check();

COMMENT ON TRIGGER trg_auto_quarantine ON allocation_ledger IS
'R5-07: Automatically quarantine manual allocations for review';

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_allocation_reward_source ON allocation_ledger(reward_source);
CREATE INDEX IF NOT EXISTS idx_allocation_batch_id ON allocation_ledger(reward_batch_id) WHERE reward_batch_id IS NOT NULL;

-- ============================================
-- UPDATED_AT TRIGGER
-- ============================================

CREATE OR REPLACE FUNCTION update_alert_policy_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    NEW.version = OLD.version + 1;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_alert_policy_updated ON alert_policies;
CREATE TRIGGER trg_alert_policy_updated
    BEFORE UPDATE ON alert_policies
    FOR EACH ROW
    EXECUTE FUNCTION update_alert_policy_timestamp();
