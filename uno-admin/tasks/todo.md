# Bug Fix: Charts Not Showing Recent Data (2026-04-18)

## Problem Analysis

**Symptoms:**
- Earnings History table shows data for 2026-04-18, 04-17, 04-16, etc.
- Charts (Daily Rewards History, Uptime History, Daily Earnings) don't show this data

**Root Causes Identified:**

1. **AgentEarningsChart** (`agents/detail.rs`)
   - Fetches data filtered to **last 7 days only** via `get_agent_daily_rewards`
   - Should show last 30 days like other charts

2. **MiniBarChart** (`agents/detail.rs`)
   - Shows only 7 days (acceptable for mini preview)

3. **DailyRewardsChart** (`licenses/detail.rs`)
   - Takes last 30 days ✓ (correct)
   - But may have sorting issues with new data

4. **Date Handling Consistency**
   - Some charts sort ascending then reverse, others don't sort
   - Need consistent approach across all charts

---

## Implementation Plan

### Phase 1: Fix AgentEarningsChart to show 30 days
- [ ] Update `get_agent_daily_rewards` handler to fetch 30 days instead of 7
- [ ] Update chart label from "Last 7 Days" to "Last 30 Days"

### Phase 2: Verify date sorting in all charts
- [ ] DailyRewardsChart - verify 2026-04-18 appears
- [ ] UptimeChart - verify recent dates appear
- [ ] AgentEarningsChart - verify 2026-04-18 appears after fix

### Phase 3: Test and verify
- [ ] Check server logs for data being fetched
- [ ] Verify all charts show 2026-04-18 data
- [ ] Verify 30-day range displays correctly

---

## Files to Modify

| File | Change |
|------|--------|
| `src/handler/rewards_handler.rs` | Change 7-day filter to 30-day filter |
| `src/ui/pages/agents/detail.rs` | Update "Last 7 Days" label to "Last 30 Days" |

---

## Verification Checklist

- [x] AgentEarningsChart shows 30 days of data (29 data points returned)
- [x] DailyRewardsChart shows 2026-04-18 (116 data points available)
- [x] All charts render correctly with date labels
- [x] Server logs confirm data fetch includes recent dates

---

## Implementation Results

### Changes Made

1. **`src/handler/rewards_handler.rs`** (lines 485-510)
   - Changed `Duration::days(7)` → `Duration::days(30)`
   - Renamed `seven_days_ago` → `thirty_days_ago`
   - Updated comment to reflect 30-day range

2. **`src/ui/pages/agents/detail.rs`** (lines 669, 831)
   - Updated chart title: "Daily Earnings (Last 7 Days)" → "Daily Earnings (Last 30 Days)"
   - Updated empty state message: "No rewards data for the last 7 days" → "No rewards data for the last 30 days"

### Verification

- Server logs confirm: `[REWARDS] Returning 29 daily data points for agent`
- Previously: Only 6 data points were returned
- License-level: 116 data points available (showing last 30 days correctly)

### Root Cause

The `get_agent_daily_rewards` handler was filtering data to only the last 7 days, while:
- The Earnings History table showed ALL data (no date filter)
- The DailyRewardsChart correctly showed last 30 days

This caused 2026-04-18 to appear in the table but not in the agent earnings chart.

---

# Rewards Batch Insertion Implementation (2026-06-06)

## Summary
Implement batch insertion of 11,731 reward records from `/data/rewards/` chunk files into ScyllaDB.

## Analysis Completed

### 1. ScyllaDB Schema (`migrations/0004_rewards.up.cql`)
```sql
CREATE TABLE IF NOT EXISTS rewards (
    id text PRIMARY KEY,
    user_id text, type text, description text,
    node_id text, license_id text, license_lease_id text,
    task_key text, task_metadata text,
    completed_at timestamp, created_at timestamp,
    amount_micros bigint
);
```

### 2. Chunk File Format
- 24 files: `rewards_chunk_0000.json` to `rewards_chunk_0023.json`
- Wrapper: `{ chunk_index, record_count, date_range, records: [...] }`
- camelCase JSON fields → snake_case DB columns

### 3. Data Statistics
- **Total Records**: 11,731
- **Date Range**: Nov 30, 2025 → June 6, 2026

## Implementation Plan

- [x] Add `RewardChunk` struct for chunk format parsing
- [x] Implement `seed_rewards_from_chunks()` method
- [x] Add duplicate detection (upsert pattern)
- [x] Progress logging per chunk
- [ ] Test and verify data integrity

## Implementation Complete

### Changes to `src/db/seeder.rs`

1. **Added structs** (lines 33-47):
   - `RewardChunk`: Wrapper for chunk file format
   - `ChunkDateRange`: Date range metadata

2. **Added `seed_rewards_from_chunks()` method** (lines 108-213):
   - Reads all `rewards_chunk_*.json` files from directory
   - Sorts by filename for consistent ordering
   - Fetches existing IDs for duplicate detection (`get_existing_reward_ids`)
   - Inserts non-duplicate records
   - Logs progress per chunk

3. **Added `get_existing_reward_ids()` helper** (lines 216-245):
   - Queries all existing reward IDs into HashSet
   - Used for O(1) duplicate checking

4. **Updated `seed_all()`** (lines 318-335):
   - Now also seeds from `REWARDS_CHUNKS_DIR` (default: `./data/rewards`)

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `REWARDS_JSON_PATH` | `./data/incentives/rewards.json` | Single JSON file seeding |
| `REWARDS_CHUNKS_DIR` | `./data/rewards` | Chunk files directory |

### Usage

The seeder will automatically process chunk files when the app starts:
```bash
# Set custom path if needed
export REWARDS_CHUNKS_DIR=/Users/admin/Documents/unetwork/uno-admin/data/rewards

# Run the application (seeder runs on startup)
cargo run
```

## Execution Results (2026-06-06)

### Standalone Python Seeder
Due to a missing `ember-fx-core` crate in the main build, a Python seeder was used:

```bash
python3 scripts/seed_rewards.py
```

### Results
| Metric | Value |
|--------|-------|
| **Pre-existing records** | 8,534 |
| **New records inserted** | 3,197 |
| **Duplicates skipped** | 8,534 |
| **Errors** | 0 |
| **Total in database** | 11,731 |

### Verification
```sql
SELECT COUNT(*) FROM unity_dashboard.rewards;
-- Result: 11731
```

All 11,731 reward records from the JSON chunks are now in the database.

---

# Bug Investigation: Sync Job Stuck in "Running" State (2026-06-10)

## Problem Summary

A Rewards sync job from 2026-06-09 is stuck indefinitely showing a spinner (status = "running"), preventing subsequent jobs from completing properly. The Job Execution Console displays "Waiting for logs..." but never receives any data.

**Screenshots analyzed:**
- `/imgs/Screenshot 2026-06-10 at 11.32.59.png`
- `/imgs/Screenshot 2026-06-10 at 11.13.33.png`

**Observed behavior:**
- Rewards job (2026-06-09 06:59:53) shows spinning status indicator
- Records: 0/0, Duration: - (never completed)
- Multiple Licenses jobs show "failed" with error "API_TOKEN environment variable not set"
- Console shows "Waiting for logs..." indefinitely

---

## Root Cause Analysis

### Primary Root Cause: Orphaned Running Job (Zombie Job)

The job is a **stale/orphaned job** from the previous day (2026-06-09) that never transitioned from "running" to a terminal state ("completed" or "failed").

**Why this happens:**

1. **Sync execution happens within HTTP request context** (`src/handler/sync_job_handler.rs:42-56`)
   ```rust
   #[server(RunSyncJob, "/api")]
   pub async fn run_sync_job(target_date: String) -> Result<SyncJobEntity, ServerFnError> {
       service.sync_rewards_for_date(&target_date).await  // Entire sync runs here
   }
   ```

2. **Job lifecycle in `rewards_sync_service.rs:61-126`:**
   - Job created with status "pending"
   - Immediately marked as "running" (`mark_running`)
   - `execute_sync` called (can take indefinite time)
   - On success: `mark_completed_with_context`
   - On error: `handle_failure` → `mark_failed`

3. **No recovery mechanism exists for:**
   - HTTP request timeouts (client disconnects)
   - Server restarts mid-execution
   - Process crashes during sync
   - Indefinite hangs in external API calls

### Secondary Issues Identified

**1. No timeout on external API calls** (`rewards_sync_service.rs:243-272`)
```rust
let response = client
    .post(&url)
    .header("apikey", &config.api_key)
    // ... no timeout configured
    .send()
    .await  // Can hang indefinitely
```

**2. Logs not persisted on early errors** (`rewards_sync_service.rs:152-154`)
```rust
if !config.has_token() {
    logs.push(Self::log_line("ERROR: API_TOKEN environment variable not set"));
    return Err("API_TOKEN environment variable not set".to_string());
    // logs vector is dropped, never saved to DB
}
```

**3. Concurrent job prevention blocks new jobs** (`rewards_sync_service.rs:65-70`)
```rust
let existing_jobs = self.job_repo.list_jobs_by_status("running").await?;
for job in existing_jobs {
    if job.target_date == target_date && job.job_type == "rewards_sync" {
        return Err(format!("Job already running for date: {}", target_date));
    }
}
```
A zombie job for a specific date will permanently block new jobs for that date.

---

## Evidence From API Response

The `/api/list_sync_jobs` response shows:
- Jobs with `"status": "failed"` have proper `error_message` fields
- The spinning job has `"status": "running"` with no `completed_at`, no `duration_ms`, `records_fetched: 0`, `records_inserted: 0`
- This confirms the job was marked as "running" but execution never completed

---

## Architectural Diagram

```
User clicks "Run Sync Now"
        │
        ▼
┌───────────────────────────────┐
│   HTTP POST /api/RunSyncJob   │
│   (sync execution within      │
│    request context)           │
└───────────────────────────────┘
        │
        ▼
┌───────────────────────────────┐
│   sync_rewards_for_date()     │
│                               │
│   1. Create job (pending)     │
│   2. mark_running() ──────────┼──▶ DB: status="running"
│   3. execute_sync() ──────────┼──▶ [CAN HANG HERE]
│      │                        │
│      ├─ fetch_allocations     │    - External API timeout
│      │  (no timeout!)         │    - Server restart
│      │                        │    - Process crash
│      └─ If success:           │    - HTTP request timeout
│         mark_completed()      │
│         If error:             │
│         mark_failed()         │
└───────────────────────────────┘

If execution interrupted at step 3:
  → Job remains "running" forever
  → UI shows infinite spinner
  → No automatic recovery
```

---

## Recommended Fixes

### Immediate Mitigation

1. **Manually fix the orphaned job:**
   - Delete the stuck job, OR
   - Update its status to "failed" in the database

### Short-term Fixes

1. **Add request timeout to external API calls:**
   ```rust
   let client = reqwest::Client::builder()
       .timeout(std::time::Duration::from_secs(30))
       .build()?;
   ```

2. **Add job timeout detection:**
   - If job is "running" for > X minutes, mark as "failed"
   - Implement a cleanup routine on startup

### Long-term Architectural Fixes

1. **Decouple job execution from HTTP request:**
   - Mark job as "pending" and return immediately
   - Process jobs via background task/scheduler
   - Use tokio::spawn for async execution

2. **Implement job watchdog:**
   ```rust
   // On startup or periodic timer
   async fn cleanup_stale_jobs(&self) {
       let running_jobs = self.job_repo.list_jobs_by_status("running").await?;
       let stale_threshold = Duration::minutes(5);

       for job in running_jobs {
           if job.started_at.is_some() {
               let started = parse_timestamp(&job.started_at.unwrap());
               if Utc::now() - started > stale_threshold {
                   self.job_repo.mark_failed(&job.id, "Job timed out").await?;
               }
           }
       }
   }
   ```

3. **Persist logs on all error paths:**
   ```rust
   if !config.has_token() {
       let error_logs = vec![Self::log_line("ERROR: API_TOKEN not set")].join("\n");
       self.job_repo.update_job_logs(&job_id, &error_logs).await?;
       return Err("API_TOKEN environment variable not set".to_string());
   }
   ```

---

## Files Investigated

| File | Purpose |
|------|---------|
| `src/ui/pages/jobs/list.rs` | UI rendering, StatusBadge shows spinner for "running" |
| `src/logic/rewards_sync_service.rs` | Core sync logic, job lifecycle |
| `src/handler/sync_job_handler.rs` | HTTP endpoints for sync operations |
| `src/repository/scylla/sync_job_repository.rs` | Database operations for jobs |
| `src/ui/hooks/use_job_websocket.rs` | 20-second polling for job updates |
| `src/api/config.rs` | API configuration, token checking |

---

## Status

**Root Cause Identified:** Orphaned/zombie job from interrupted execution (no recovery mechanism)

**Next Steps:**
- [ ] Delete or mark failed the stuck job (immediate)
- [ ] Add timeout to reqwest client (short-term)
- [ ] Implement job watchdog/cleanup (medium-term)
- [ ] Decouple execution from HTTP context (long-term)
