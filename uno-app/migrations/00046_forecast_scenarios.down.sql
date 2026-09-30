-- Phase 8: Forecast Scenarios - Rollback

DROP TRIGGER IF EXISTS trg_forecast_tasks_updated ON forecast_tasks;
DROP TRIGGER IF EXISTS trg_forecast_scenarios_updated ON forecast_scenarios;
DROP FUNCTION IF EXISTS update_forecast_timestamp;
DROP FUNCTION IF EXISTS export_scenario;
DROP FUNCTION IF EXISTS run_forecast;
DROP TABLE IF EXISTS forecast_scenario_snapshots;
DROP TABLE IF EXISTS forecast_golden_fixtures;
DROP TABLE IF EXISTS forecast_results;
DROP TABLE IF EXISTS forecast_tasks;
DROP TABLE IF EXISTS forecast_scenarios;
DROP TYPE IF EXISTS forecast_status;
