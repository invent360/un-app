-- Rollback migration 00031: Agent Workflow

-- Drop functions
DROP FUNCTION IF EXISTS auto_lift_expired_suspensions;
DROP FUNCTION IF EXISTS lift_agent_suspension;
DROP FUNCTION IF EXISTS suspend_agent;
DROP FUNCTION IF EXISTS reject_agent;
DROP FUNCTION IF EXISTS approve_agent;

-- Drop applications table
DROP INDEX IF EXISTS idx_agent_applications_agent;
DROP TABLE IF EXISTS agent_applications;

-- Drop status history
DROP INDEX IF EXISTS idx_agent_status_history_admin;
DROP INDEX IF EXISTS idx_agent_status_history_agent;
DROP TABLE IF EXISTS agent_status_history;

-- Remove agent columns
DROP INDEX IF EXISTS idx_agents_suspension_expiry;
DROP INDEX IF EXISTS idx_agents_pending;
DROP INDEX IF EXISTS idx_agents_status;
ALTER TABLE agents DROP COLUMN IF EXISTS last_activity_at;
ALTER TABLE agents DROP COLUMN IF EXISTS license_count;
ALTER TABLE agents DROP COLUMN IF EXISTS onboarding_completed_at;
ALTER TABLE agents DROP COLUMN IF EXISTS onboarding_completed;
ALTER TABLE agents DROP COLUMN IF EXISTS termination_reason;
ALTER TABLE agents DROP COLUMN IF EXISTS terminated_by;
ALTER TABLE agents DROP COLUMN IF EXISTS terminated_at;
ALTER TABLE agents DROP COLUMN IF EXISTS suspension_lifted_by;
ALTER TABLE agents DROP COLUMN IF EXISTS suspension_lifted_at;
ALTER TABLE agents DROP COLUMN IF EXISTS suspension_ends_at;
ALTER TABLE agents DROP COLUMN IF EXISTS suspension_reason;
ALTER TABLE agents DROP COLUMN IF EXISTS suspended_by;
ALTER TABLE agents DROP COLUMN IF EXISTS suspended_at;
ALTER TABLE agents DROP COLUMN IF EXISTS rejection_reason;
ALTER TABLE agents DROP COLUMN IF EXISTS rejected_by;
ALTER TABLE agents DROP COLUMN IF EXISTS rejected_at;
ALTER TABLE agents DROP COLUMN IF EXISTS approval_notes;
ALTER TABLE agents DROP COLUMN IF EXISTS approved_by;
ALTER TABLE agents DROP COLUMN IF EXISTS approved_at;
ALTER TABLE agents DROP COLUMN IF EXISTS status;

-- Drop status enum
DROP TYPE IF EXISTS agent_status;
