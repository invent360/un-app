-- Remove job context and duration tracking from sync_jobs
ALTER TABLE sync_jobs DROP job_context;
ALTER TABLE sync_jobs DROP duration_ms;
