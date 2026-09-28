-- Sync jobs table for tracking background job execution
CREATE TABLE IF NOT EXISTS sync_jobs (
    id text PRIMARY KEY,
    job_type text,
    target_date text,
    status text,
    attempt_count int,
    max_attempts int,
    records_fetched int,
    records_inserted int,
    error_message text,
    started_at timestamp,
    completed_at timestamp,
    next_retry_at timestamp,
    created_at timestamp,
    updated_at timestamp
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_sync_jobs_status ON sync_jobs (status);
CREATE INDEX IF NOT EXISTS idx_sync_jobs_target_date ON sync_jobs (target_date);
CREATE INDEX IF NOT EXISTS idx_sync_jobs_job_type ON sync_jobs (job_type);
