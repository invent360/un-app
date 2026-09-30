-- Phase 9: Pilot gates and evidence requirements
-- Adds pilot-specific launch gates and evidence tracking for controlled rollout

-- Add pilot-specific gates
INSERT INTO launch_gates (gate_name, gate_description, is_enabled)
VALUES
    ('pilot_market_ph', 'Pilot market: Philippines', false),
    ('pilot_market_bd', 'Pilot market: Bangladesh', false),
    ('production_cutover', 'Production cutover completed', false),
    ('metrics_enabled', 'Prometheus metrics collection enabled', true),
    ('pilot_enrollment', 'New pilot participant enrollment enabled', false)
ON CONFLICT (gate_name) DO UPDATE SET
    gate_description = EXCLUDED.gate_description;

-- Evidence requirements for gate enablement
CREATE TABLE IF NOT EXISTS gate_evidence_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gate_name VARCHAR(100) NOT NULL REFERENCES launch_gates(gate_name),
    requirement_type VARCHAR(50) NOT NULL,
    requirement_description TEXT NOT NULL,
    is_mandatory BOOLEAN NOT NULL DEFAULT true,
    evidence_url TEXT,
    evidence_hash VARCHAR(64),  -- SHA256 hash of evidence file
    verified_at TIMESTAMPTZ,
    verified_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(gate_name, requirement_type)
);

-- Create index for efficient lookups
CREATE INDEX IF NOT EXISTS idx_gate_evidence_gate_name ON gate_evidence_requirements(gate_name);
CREATE INDEX IF NOT EXISTS idx_gate_evidence_verified ON gate_evidence_requirements(verified_at) WHERE verified_at IS NOT NULL;

-- Insert standard evidence requirements for production cutover
INSERT INTO gate_evidence_requirements (gate_name, requirement_type, requirement_description, is_mandatory)
VALUES
    ('production_cutover', 'migration_rehearsal', 'Migration rehearsal completed with reconciliation report', true),
    ('production_cutover', 'rollback_tested', 'Rollback procedure tested with RTO < 4 hours', true),
    ('production_cutover', 'security_scan', 'Container security scan passed (no CRITICAL)', true),
    ('production_cutover', 'load_test', 'Load test results meeting p95 latency targets', true),
    ('production_cutover', 'monitoring_verified', 'Prometheus metrics and alerts verified', true),
    ('production_cutover', 'release_manifest', 'Release manifest with all artifact hashes', true)
ON CONFLICT (gate_name, requirement_type) DO NOTHING;

-- Insert pilot market requirements
INSERT INTO gate_evidence_requirements (gate_name, requirement_type, requirement_description, is_mandatory)
VALUES
    ('pilot_market_ph', 'localization_ready', 'Content localized for Philippines market', true),
    ('pilot_market_ph', 'support_ready', 'Support team trained for Philippines timezone', true),
    ('pilot_market_ph', 'payment_ready', 'GCash/Maya payment integration tested', false),
    ('pilot_market_bd', 'localization_ready', 'Content localized for Bangladesh market', true),
    ('pilot_market_bd', 'support_ready', 'Support team trained for Bangladesh timezone', true),
    ('pilot_market_bd', 'payment_ready', 'bKash payment integration tested', false)
ON CONFLICT (gate_name, requirement_type) DO NOTHING;

-- Gate enablement audit log
CREATE TABLE IF NOT EXISTS gate_enablement_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    gate_name VARCHAR(100) NOT NULL,
    action VARCHAR(20) NOT NULL CHECK (action IN ('enable', 'disable', 'pause')),
    actor VARCHAR(255) NOT NULL,
    reason TEXT,
    evidence_snapshot JSONB,  -- Snapshot of evidence state at enablement
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_gate_enablement_gate_name ON gate_enablement_log(gate_name);
CREATE INDEX IF NOT EXISTS idx_gate_enablement_created ON gate_enablement_log(created_at);

-- Emergency pause tracking
CREATE TABLE IF NOT EXISTS emergency_pauses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    initiated_by VARCHAR(255) NOT NULL,
    reason TEXT NOT NULL,
    gates_paused TEXT[] NOT NULL,  -- Array of gate names paused
    paused_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resumed_at TIMESTAMPTZ,
    resumed_by VARCHAR(255),
    resume_reason TEXT
);

CREATE INDEX IF NOT EXISTS idx_emergency_pauses_active ON emergency_pauses(paused_at) WHERE resumed_at IS NULL;

COMMENT ON TABLE gate_evidence_requirements IS 'Phase 9: Evidence requirements for launch gate enablement';
COMMENT ON TABLE gate_enablement_log IS 'Phase 9: Audit log for gate enable/disable actions';
COMMENT ON TABLE emergency_pauses IS 'Phase 9: Emergency pause tracking for incident response';
