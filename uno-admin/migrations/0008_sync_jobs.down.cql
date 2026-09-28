-- Drop sync_jobs table indexes
DROP INDEX IF EXISTS idx_sync_jobs_status;
DROP INDEX IF EXISTS idx_sync_jobs_target_date;
DROP INDEX IF EXISTS idx_sync_jobs_job_type;

-- Drop sync_jobs table
DROP TABLE IF EXISTS sync_jobs;
