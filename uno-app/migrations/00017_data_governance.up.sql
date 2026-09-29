-- =============================================================================
-- Migration: 00017_data_governance.up.sql
-- Description: Data Governance and Launch Gates (Phase 11)
-- =============================================================================

-- GOV-10: Launch Gates Schema
-- Implements 8 launch gates as product features with dated evidence

CREATE TABLE IF NOT EXISTS launch_gates (
    id SERIAL PRIMARY KEY,
    gate_name VARCHAR(100) NOT NULL UNIQUE,
    gate_category VARCHAR(50) NOT NULL,  -- 'software', 'commercial', 'operational'
    description TEXT,
    required BOOLEAN DEFAULT true,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Evidence records for each gate
CREATE TABLE IF NOT EXISTS launch_gate_evidence (
    id SERIAL PRIMARY KEY,
    gate_id INT NOT NULL REFERENCES launch_gates(id),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- 'pending', 'in_progress', 'passed', 'failed', 'waived'
    evidence_type VARCHAR(50) NOT NULL,  -- 'test_run', 'manual_verification', 'document', 'metric'
    evidence_data JSONB,
    notes TEXT,
    verified_by VARCHAR(255),
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Constraint: status must be valid
ALTER TABLE launch_gate_evidence
ADD CONSTRAINT valid_gate_status
CHECK (status IN ('pending', 'in_progress', 'passed', 'failed', 'waived'));

-- Indexes for launch gates
CREATE INDEX IF NOT EXISTS idx_launch_gate_evidence_gate ON launch_gate_evidence(gate_id);
CREATE INDEX IF NOT EXISTS idx_launch_gate_evidence_status ON launch_gate_evidence(status);

-- Insert the 8 standard launch gates
INSERT INTO launch_gates (gate_name, gate_category, description, required, sort_order) VALUES
    ('security_audit', 'software', 'All privileged endpoints deny unauthorized callers', true, 1),
    ('concurrent_claims', 'software', 'Concurrent sessions never acquire same credential as different owners', true, 2),
    ('data_integrity', 'software', 'Database operations reconcile correctly under load', true, 3),
    ('privacy_compliance', 'software', 'No raw credentials in logs, PII handling compliant', true, 4),
    ('pilot_readiness', 'commercial', '30-user pilot readiness across target markets', true, 5),
    ('support_capacity', 'operational', 'Support infrastructure and processes in place', true, 6),
    ('monitoring_setup', 'operational', 'Monitoring, alerting, and incident response configured', true, 7),
    ('legal_review', 'commercial', 'Terms, agreements, and compliance documents approved', true, 8)
ON CONFLICT (gate_name) DO NOTHING;

-- =============================================================================
-- GOV-14: Funnel Stage Tracking
-- Store funnel stage as measured, not claimed
-- =============================================================================

CREATE TABLE IF NOT EXISTS funnel_stages (
    id SERIAL PRIMARY KEY,
    stage_name VARCHAR(50) NOT NULL UNIQUE,
    stage_order INT NOT NULL,
    description TEXT,
    is_conversion_point BOOLEAN DEFAULT false
);

-- Insert standard funnel stages
INSERT INTO funnel_stages (stage_name, stage_order, description, is_conversion_point) VALUES
    ('visit', 1, 'User visited the site', false),
    ('view_info', 2, 'User viewed license information', false),
    ('start_claim', 3, 'User started claim process', false),
    ('economics_review', 4, 'User reviewed economics disclosure', false),
    ('terms_accepted', 5, 'User accepted terms and conditions', false),
    ('reserved', 6, 'License reserved for user', false),
    ('claimed', 7, 'License successfully claimed', true),
    ('activated', 8, 'Device activated with license', true),
    ('d1_check', 9, 'Day 1 retention check', false),
    ('d7_check', 10, 'Day 7 retention check', false),
    ('d30_check', 11, 'Day 30 retention check', true)
ON CONFLICT (stage_name) DO NOTHING;

-- User journey tracking with measured timestamps
CREATE TABLE IF NOT EXISTS user_journey (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(64) NOT NULL,
    visitor_id INT REFERENCES visitors(id),
    license_id UUID REFERENCES licenses(id),
    stage_id INT NOT NULL REFERENCES funnel_stages(id),
    measured_at TIMESTAMPTZ DEFAULT NOW(),
    metadata JSONB,
    CONSTRAINT unique_session_stage UNIQUE (session_id, stage_id)
);

-- Indexes for funnel analytics
CREATE INDEX IF NOT EXISTS idx_user_journey_session ON user_journey(session_id);
CREATE INDEX IF NOT EXISTS idx_user_journey_license ON user_journey(license_id);
CREATE INDEX IF NOT EXISTS idx_user_journey_stage ON user_journey(stage_id);
CREATE INDEX IF NOT EXISTS idx_user_journey_measured ON user_journey(measured_at);

-- =============================================================================
-- GOV-02: Data Retention Policies
-- =============================================================================

CREATE TABLE IF NOT EXISTS retention_policies (
    id SERIAL PRIMARY KEY,
    table_name VARCHAR(100) NOT NULL UNIQUE,
    retention_days INT,  -- NULL = indefinite
    archive_before_delete BOOLEAN DEFAULT false,
    deletion_condition TEXT NOT NULL,
    last_cleanup_at TIMESTAMPTZ,
    rows_deleted_last INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert standard retention policies
INSERT INTO retention_policies (table_name, retention_days, archive_before_delete, deletion_condition) VALUES
    ('visitors', 90, false, 'last_seen_at < NOW() - INTERVAL ''90 days'''),
    ('health_checks', 7, false, 'checked_at < NOW() - INTERVAL ''7 days'' AND id NOT IN (SELECT id FROM health_checks ORDER BY checked_at DESC LIMIT 1000)'),
    ('audit_logs', 730, true, 'created_at < NOW() - INTERVAL ''730 days'''),
    ('job_queue', 30, false, 'status IN (''completed'', ''failed'', ''dead_letter'') AND completed_at < NOW() - INTERVAL ''30 days'''),
    ('license_reservations', 7, false, 'status = ''expired'' AND expires_at < NOW() - INTERVAL ''7 days'''),
    ('user_journey', 365, false, 'measured_at < NOW() - INTERVAL ''365 days'''),
    ('allocation_ledger', NULL, false, 'FALSE')  -- Never delete, immutable audit trail
ON CONFLICT (table_name) DO UPDATE SET
    retention_days = EXCLUDED.retention_days,
    deletion_condition = EXCLUDED.deletion_condition;

-- =============================================================================
-- GOV-03: IP Privacy - Add anonymization column
-- =============================================================================

-- Add columns to track IP anonymization status
ALTER TABLE visitors
ADD COLUMN IF NOT EXISTS ip_anonymized BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS ip_hash VARCHAR(16);

-- Create index for anonymized IPs
CREATE INDEX IF NOT EXISTS idx_visitors_ip_hash ON visitors(ip_hash) WHERE ip_hash IS NOT NULL;

-- =============================================================================
-- GOV-11: Weekly Operating Rhythm
-- =============================================================================

CREATE TABLE IF NOT EXISTS operating_metrics (
    id SERIAL PRIMARY KEY,
    week_start DATE NOT NULL,
    week_number INT NOT NULL,
    year INT NOT NULL,
    metrics JSONB NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by VARCHAR(255),
    CONSTRAINT unique_week UNIQUE (year, week_number)
);

-- Standard metrics template (stored as JSONB)
COMMENT ON TABLE operating_metrics IS 'Weekly operating metrics for GOV-11 operating rhythm. Metrics JSONB includes: active_licenses, new_claims, churn_rate, support_tickets, revenue_pool, etc.';

-- =============================================================================
-- GOV-13: Forecast vs Observed Separation
-- =============================================================================

CREATE TABLE IF NOT EXISTS forecast_observations (
    id SERIAL PRIMARY KEY,
    week_start DATE NOT NULL,
    scenario_name VARCHAR(100) NOT NULL,
    field_name VARCHAR(100) NOT NULL,
    forecast_value DECIMAL(18, 2),
    observed_value DECIMAL(18, 2),
    variance_pct DECIMAL(8, 4),
    notes TEXT,
    recorded_at TIMESTAMPTZ DEFAULT NOW(),
    recorded_by VARCHAR(255),
    CONSTRAINT unique_observation UNIQUE (week_start, scenario_name, field_name)
);

CREATE INDEX IF NOT EXISTS idx_forecast_observations_week ON forecast_observations(week_start);
CREATE INDEX IF NOT EXISTS idx_forecast_observations_scenario ON forecast_observations(scenario_name);

-- =============================================================================
-- Archive tables for retention with archiving
-- =============================================================================

CREATE TABLE IF NOT EXISTS audit_logs_archive (
    LIKE audit_logs INCLUDING ALL
);

COMMENT ON TABLE audit_logs_archive IS 'Archive of audit logs older than retention period (730 days)';
