-- Migration 00022: Add missing columns to audit_logs table
-- These columns are expected by the CMS audit types but were missing from the original schema

-- Add actor_id column (string identifier instead of user_id integer)
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS actor_id VARCHAR(255);

-- Add actor_name column for display purposes
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS actor_name VARCHAR(255);

-- Add metadata column for additional context
ALTER TABLE audit_logs ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Create index on actor_id for efficient lookups
CREATE INDEX IF NOT EXISTS idx_audit_logs_actor ON audit_logs(actor_id);

-- Backfill actor_id from user_id where applicable
UPDATE audit_logs SET actor_id = user_id::text WHERE actor_id IS NULL AND user_id IS NOT NULL;
