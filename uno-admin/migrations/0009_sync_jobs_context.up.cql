-- Add job context and duration tracking to sync_jobs
ALTER TABLE sync_jobs ADD job_context text;
ALTER TABLE sync_jobs ADD duration_ms bigint;
