-- R3-10: Lease, fence and recover jobs/events
-- Adds fencing tokens to prevent stale worker writes
-- See: v2_relaunch_implementation_plan.md Gate C

-- ============================================
-- 1. ADD FENCING TOKENS TO JOB QUEUE
-- ============================================
-- claim_generation prevents stale writes even if worker IDs collide

ALTER TABLE job_queue
ADD COLUMN IF NOT EXISTS claim_generation INT NOT NULL DEFAULT 0;

COMMENT ON COLUMN job_queue.claim_generation IS 'R3-10: Incremented on each claim, prevents stale writes';

-- ============================================
-- 2. ADD FENCING TOKENS TO OUTBOX
-- ============================================
-- Tracks which worker claimed the event for publishing

ALTER TABLE outbox
ADD COLUMN IF NOT EXISTS claimed_by VARCHAR(36),
ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS claim_expires_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS claim_generation INT NOT NULL DEFAULT 0;

COMMENT ON COLUMN outbox.claimed_by IS 'R3-10: Worker ID holding the publishing lease';
COMMENT ON COLUMN outbox.claimed_at IS 'R3-10: When the event was claimed for publishing';
COMMENT ON COLUMN outbox.claim_expires_at IS 'R3-10: When the publishing lease expires';
COMMENT ON COLUMN outbox.claim_generation IS 'R3-10: Incremented on each claim, prevents stale writes';

-- ============================================
-- 3. INDEXES FOR FENCING
-- ============================================

-- Expired publishing claims (for recovery)
CREATE INDEX IF NOT EXISTS idx_outbox_claim_expired
ON outbox (claim_expires_at)
WHERE status = 'publishing' AND claim_expires_at IS NOT NULL;

-- Worker's active claims
CREATE INDEX IF NOT EXISTS idx_outbox_claimed_by
ON outbox (claimed_by)
WHERE claimed_by IS NOT NULL;

-- ============================================
-- 4. FUNCTION TO CLAIM WITH FENCING
-- ============================================
-- Atomic claim that increments generation

CREATE OR REPLACE FUNCTION claim_job_with_fencing(
    p_job_id VARCHAR(36),
    p_worker_id VARCHAR(36),
    p_lease_duration_secs INT
)
RETURNS TABLE (
    claimed BOOLEAN,
    job_type VARCHAR,
    payload JSONB,
    generation INT
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_new_generation INT;
    v_job_type VARCHAR;
    v_payload JSONB;
BEGIN
    -- Attempt to claim with atomic generation increment
    UPDATE job_queue
    SET
        status = 'running',
        worker_id = p_worker_id,
        lease_expires_at = NOW() + (p_lease_duration_secs || ' seconds')::INTERVAL,
        claim_generation = claim_generation + 1,
        started_at = COALESCE(started_at, NOW()),
        attempts = attempts + 1
    WHERE id = p_job_id
      AND (status = 'pending' OR (status = 'failed' AND retry_at <= NOW()))
    RETURNING
        job_queue.claim_generation,
        job_queue.job_type,
        job_queue.payload
    INTO v_new_generation, v_job_type, v_payload;

    IF FOUND THEN
        RETURN QUERY SELECT true, v_job_type, v_payload, v_new_generation;
    ELSE
        RETURN QUERY SELECT false, NULL::VARCHAR, NULL::JSONB, 0;
    END IF;
END;
$$;

-- ============================================
-- 5. FUNCTION TO COMPLETE WITH FENCING
-- ============================================
-- Only the lease holder with correct generation can complete

CREATE OR REPLACE FUNCTION complete_job_with_fencing(
    p_job_id VARCHAR(36),
    p_worker_id VARCHAR(36),
    p_generation INT,
    p_result JSONB,
    p_outcome VARCHAR
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
DECLARE
    v_updated BOOLEAN;
BEGIN
    UPDATE job_queue
    SET
        status = 'completed',
        completed_at = NOW(),
        result = p_result,
        outcome = p_outcome,
        outcome_verified_at = NOW(),
        worker_id = NULL,
        lease_expires_at = NULL
    WHERE id = p_job_id
      AND worker_id = p_worker_id
      AND claim_generation = p_generation
      AND lease_expires_at > NOW()  -- Lease must still be valid
      AND outcome IS NULL;          -- Prevent double-completion

    GET DIAGNOSTICS v_updated = ROW_COUNT;
    RETURN v_updated > 0;
END;
$$;

-- ============================================
-- 6. FUNCTION TO FAIL WITH FENCING
-- ============================================

CREATE OR REPLACE FUNCTION fail_job_with_fencing(
    p_job_id VARCHAR(36),
    p_worker_id VARCHAR(36),
    p_generation INT,
    p_error TEXT,
    p_retry BOOLEAN,
    p_retry_delay_secs INT
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
DECLARE
    v_updated BOOLEAN;
    v_max_attempts INT := 5;
    v_attempts INT;
BEGIN
    -- Get current attempts
    SELECT attempts INTO v_attempts FROM job_queue WHERE id = p_job_id;

    IF p_retry AND v_attempts < v_max_attempts THEN
        -- Mark for retry
        UPDATE job_queue
        SET
            status = 'failed',
            error = p_error,
            retry_at = NOW() + (p_retry_delay_secs || ' seconds')::INTERVAL,
            worker_id = NULL,
            lease_expires_at = NULL
        WHERE id = p_job_id
          AND worker_id = p_worker_id
          AND claim_generation = p_generation
          AND lease_expires_at > NOW();
    ELSE
        -- Move to dead letter
        UPDATE job_queue
        SET
            status = 'dead_letter',
            error = p_error,
            worker_id = NULL,
            lease_expires_at = NULL
        WHERE id = p_job_id
          AND worker_id = p_worker_id
          AND claim_generation = p_generation
          AND lease_expires_at > NOW();
    END IF;

    GET DIAGNOSTICS v_updated = ROW_COUNT;
    RETURN v_updated > 0;
END;
$$;

-- ============================================
-- 7. EVENT RECOVERY FOR STALE PUBLISHING
-- ============================================

CREATE OR REPLACE FUNCTION recover_stale_publishing_events(
    p_stale_threshold_secs INT DEFAULT 300
)
RETURNS INT
LANGUAGE plpgsql
AS $$
DECLARE
    v_recovered INT;
BEGIN
    UPDATE outbox
    SET
        status = 'pending',
        claimed_by = NULL,
        claimed_at = NULL,
        claim_expires_at = NULL,
        error = 'Recovered from stale publishing state'
    WHERE status = 'publishing'
      AND claim_expires_at IS NOT NULL
      AND claim_expires_at < NOW();

    GET DIAGNOSTICS v_recovered = ROW_COUNT;
    RETURN v_recovered;
END;
$$;

-- ============================================
-- 8. COMMENTS
-- ============================================

COMMENT ON FUNCTION claim_job_with_fencing IS 'R3-10: Atomically claim job with fencing token';
COMMENT ON FUNCTION complete_job_with_fencing IS 'R3-10: Complete job only if holding valid lease';
COMMENT ON FUNCTION fail_job_with_fencing IS 'R3-10: Fail job only if holding valid lease';
COMMENT ON FUNCTION recover_stale_publishing_events IS 'R3-10: Recover events stuck in publishing state';
