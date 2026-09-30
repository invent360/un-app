-- R3-10: Rollback job fencing

-- Drop functions
DROP FUNCTION IF EXISTS recover_stale_publishing_events(INT);
DROP FUNCTION IF EXISTS fail_job_with_fencing(VARCHAR, VARCHAR, INT, TEXT, BOOLEAN, INT);
DROP FUNCTION IF EXISTS complete_job_with_fencing(VARCHAR, VARCHAR, INT, JSONB, VARCHAR);
DROP FUNCTION IF EXISTS claim_job_with_fencing(VARCHAR, VARCHAR, INT);

-- Drop indexes
DROP INDEX IF EXISTS idx_outbox_claimed_by;
DROP INDEX IF EXISTS idx_outbox_claim_expired;

-- Remove outbox columns
ALTER TABLE outbox
DROP COLUMN IF EXISTS claimed_by,
DROP COLUMN IF EXISTS claimed_at,
DROP COLUMN IF EXISTS claim_expires_at,
DROP COLUMN IF EXISTS claim_generation;

-- Remove job_queue column
ALTER TABLE job_queue
DROP COLUMN IF EXISTS claim_generation;
