-- Rollback Phase 9: Pilot gates and evidence requirements

DROP TABLE IF EXISTS emergency_pauses;
DROP TABLE IF EXISTS gate_enablement_log;
DROP TABLE IF EXISTS gate_evidence_requirements;

-- Remove pilot gates (keep existing gates)
DELETE FROM launch_gates WHERE gate_name IN (
    'pilot_market_ph',
    'pilot_market_bd',
    'production_cutover',
    'metrics_enabled',
    'pilot_enrollment'
);
