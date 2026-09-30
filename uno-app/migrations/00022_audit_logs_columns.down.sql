-- Migration 00022 Down: Remove added audit_logs columns

DROP INDEX IF EXISTS idx_audit_logs_actor;

ALTER TABLE audit_logs DROP COLUMN IF EXISTS metadata;
ALTER TABLE audit_logs DROP COLUMN IF EXISTS actor_name;
ALTER TABLE audit_logs DROP COLUMN IF EXISTS actor_id;
