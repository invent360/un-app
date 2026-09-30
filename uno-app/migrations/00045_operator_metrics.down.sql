-- Phase 8: Operator Metrics - Rollback

DROP FUNCTION IF EXISTS record_exception;
DROP FUNCTION IF EXISTS aggregate_daily_metrics;
DROP TABLE IF EXISTS operator_margins;
DROP TABLE IF EXISTS operator_exceptions;
DROP TABLE IF EXISTS operator_metrics;
DROP TYPE IF EXISTS exception_severity;
