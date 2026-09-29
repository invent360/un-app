-- =============================================================================
-- Migration: 00017_data_governance.down.sql
-- Description: Rollback Data Governance and Launch Gates
-- =============================================================================

-- Drop archive tables
DROP TABLE IF EXISTS audit_logs_archive;

-- Drop forecast observations
DROP TABLE IF EXISTS forecast_observations;

-- Drop operating metrics
DROP TABLE IF EXISTS operating_metrics;

-- Drop visitor anonymization columns
ALTER TABLE visitors DROP COLUMN IF EXISTS ip_anonymized;
ALTER TABLE visitors DROP COLUMN IF EXISTS ip_hash;

-- Drop retention policies
DROP TABLE IF EXISTS retention_policies;

-- Drop funnel tracking
DROP TABLE IF EXISTS user_journey;
DROP TABLE IF EXISTS funnel_stages;

-- Drop launch gates
DROP TABLE IF EXISTS launch_gate_evidence;
DROP TABLE IF EXISTS launch_gates;
