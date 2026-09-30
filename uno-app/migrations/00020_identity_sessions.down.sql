-- Migration 00020 Down: Remove Identity and Session Management Tables

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_expired_sessions();
DROP FUNCTION IF EXISTS cleanup_expired_blacklist();

-- Drop indexes
DROP INDEX IF EXISTS idx_gate_history_gate;
DROP INDEX IF EXISTS idx_audit_log_category;
DROP INDEX IF EXISTS idx_audit_log_resource;
DROP INDEX IF EXISTS idx_audit_log_actor;
DROP INDEX IF EXISTS idx_audit_log_type;
DROP INDEX IF EXISTS idx_audit_log_time;
DROP INDEX IF EXISTS idx_blacklist_expires;
DROP INDEX IF EXISTS idx_sessions_revoked;
DROP INDEX IF EXISTS idx_sessions_expires;
DROP INDEX IF EXISTS idx_sessions_user;
DROP INDEX IF EXISTS idx_user_identities_active;
DROP INDEX IF EXISTS idx_user_identities_provider;
DROP INDEX IF EXISTS idx_user_identities_email;

-- Drop tables (in dependency order)
DROP TABLE IF EXISTS launch_gate_history;
DROP TABLE IF EXISTS launch_gates;
DROP TABLE IF EXISTS audit_log;
DROP TABLE IF EXISTS session_blacklist;
DROP TABLE IF EXISTS active_sessions;
DROP TABLE IF EXISTS user_identities;
