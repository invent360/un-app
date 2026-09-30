# P0-01: Route, Function, and Service Inventory

**Phase 0 Task** | **Status**: Complete | **Date**: 2026-09-29

This document provides a comprehensive inventory of all routes, server functions, WebSocket handlers, repositories, and schedulers in the u-network codebase.

---

## Table of Contents

1. [UNO-APP](#uno-app)
   - [HTTP Routes](#uno-app-http-routes)
   - [Leptos Server Functions](#uno-app-server-functions)
   - [Repositories](#uno-app-repositories)
   - [Background Schedulers](#uno-app-schedulers)
2. [UNO-ADMIN](#uno-admin)
   - [HTTP Routes (via Server Functions)](#uno-admin-http-routes)
   - [WebSocket Handlers](#uno-admin-websocket-handlers)
   - [Repository Traits & Implementations](#uno-admin-repositories)
   - [Sync Services & Schedulers](#uno-admin-sync-services)

---

## UNO-APP

### UNO-APP HTTP Routes

Routes configured in `uno-app/src/server/handlers/mod.rs` and file handlers.

| Path | Method | Handler | Auth Required | Notes |
|------|--------|---------|---------------|-------|
| `/health` | GET | `health_handler::health_check` | No | Health check with DB connectivity |
| `/health/live` | GET | `health_handler::liveness` | No | Simple liveness probe |
| `/api/v1/licenses/variants` | GET | `licenses_handler::get_variants` | No | Get available license split options |
| `/api/v1/licenses/claim` | POST | `licenses_handler::claim_license` | No | Claim a license (body: ClaimRequest) |
| `/files/{resource_id}/{filename}` | GET | `file_handler::serve_local_file` | No | Serve local storage files directly |
| `/api/files/display-url` | GET | `file_handler::get_display_url` | No | Convert storage URL to display URL |
| `/api/files/display-urls` | POST | `file_handler::batch_display_urls` | No | Batch convert storage URLs |
| `/api/files/serve/{storage_url}` | GET | `file_handler::get_file` | No | Redirect to signed URL or serve local |
| `/api/admin/files/upload` | POST | `file_handler::upload_files` | **AdminAuth** | Upload files to storage (multipart) |
| `/api/admin/files/{resource_id}` | DELETE | `file_handler::delete_files` | **AdminAuth** | Delete all files for a resource |

### UNO-APP Server Functions

Leptos server functions defined with `#[server]` macro. Located in `uno-app/src/api/`.

| Function | Endpoint | Module | Auth Required | Description |
|----------|----------|--------|---------------|-------------|
| `get_faqs` | `/api/get_faqs` | `faq.rs` | No | Fetch published FAQ content |
| `get_faqs_preview` | `/api/get_faqs_preview` | `faq.rs` | Preview Token | Fetch FAQ preview content |
| `get_home_content` | `/api/get_home_content` | `home.rs` | No | Fetch published home page content |
| `get_home_content_preview` | `/api/get_home_content_preview` | `home.rs` | Preview Token | Fetch home page preview content |
| `get_guides` | `/api/get_guides` | `guides.rs` | No | Fetch published guides |
| `get_guides_preview` | `/api/get_guides_preview` | `guides.rs` | Preview Token | Fetch guides preview |
| `get_guide_by_slug` | `/api/get_guide_by_slug` | `guides.rs` | No | Get single guide by slug |
| `get_guide_by_slug_preview` | `/api/get_guide_by_slug_preview` | `guides.rs` | Preview Token | Get guide preview by slug |
| `get_tasks` | `/api/get_tasks` | `tasks.rs` | No | Fetch published tasks |
| `get_tasks_preview` | `/api/get_tasks_preview` | `tasks.rs` | Preview Token | Fetch tasks preview |
| `get_task_by_slug` | `/api/get_task_by_slug` | `tasks.rs` | No | Get single task by slug |
| `get_task_by_slug_preview` | `/api/get_task_by_slug_preview` | `tasks.rs` | Preview Token | Get task preview by slug |
| `get_user_referrals` | `/api/get_user_referrals` | `referrals.rs` | JWT Token | Get referrals for authenticated user |
| `get_license_stats` | `/api/get_license_stats` | `stats.rs` | No | Get license statistics summary |
| `send_chat_message` | `/api/send_chat_message` | `chatbot.rs` | No | Send message to AI chatbot |
| `submit_cms_review` | `/api/submit_cms_review` | `cms_review.rs` | No | Submit CMS content for review |
| `get_cms_reviews` | `/api/get_cms_reviews` | `cms_review.rs` | HMAC Admin | List CMS reviews (admin) |
| `update_cms_review_status` | `/api/update_cms_review_status` | `cms_review.rs` | HMAC Admin | Update review status (admin) |

### UNO-APP Repositories

Repository implementations in `uno-app/src/server/repositories/`.

| Repository | Struct | Trait | Methods | Notes |
|------------|--------|-------|---------|-------|
| **License Repository** | `PostgresLicenseRepository` | `LicenseRepository` (uno-api) | `insert`, `insert_batch`, `get_by_id`, `get_by_lease_code`, `get_first_unclaimed`, `get_first_unclaimed_any`, `get_by_split_type`, `get_unclaimed`, `count_total`, `count_unclaimed`, `count_unclaimed_by_split`, `claim`, `lease_code_exists`, `delete`, `delete_batch`, `get_summary`, `search` | Main license operations for claiming |
| **Audit Repository (Legacy)** | `AuditRepositoryImpl` | `AuditRepository` | `create`, `list`, `get_by_entity` | CMS audit logging (audit_logs table) |
| **Immutable Audit Repository** | `ImmutableAuditRepositoryImpl` | `ImmutableAuditRepository` | `log_event`, `get_events_by_actor`, `get_events_by_resource`, `get_events_by_type`, `get_events_by_category` | Compliance audit logging (audit_log table) |
| **Claim Repository** | `ClaimRepositoryImpl` | `ClaimRepository` | `create`, `get_by_id`, `get_by_license_id`, `list_by_user` | License claim tracking |
| **Session Repository** | `SessionRepositoryImpl` | `SessionRepository` | `create`, `get_by_id`, `get_by_user_id`, `delete`, `delete_expired` | User session management |
| **Launch Gate Repository** | `LaunchGateRepositoryImpl` | `LaunchGateRepository` | `get_active_gates`, `check_gate`, `update_gate_status` | Feature gate management |

### UNO-APP Schedulers

Background tasks and schedulers in `uno-app/src/server/scheduler/`.

| Scheduler | Module | Interval | Description |
|-----------|--------|----------|-------------|
| **Content Scheduler** | `scheduler/mod.rs` | 5 minutes | Schedules content for publication based on `publish_at` timestamps |

---

## UNO-ADMIN

### UNO-ADMIN HTTP Routes

Routes are implemented as Leptos server functions with `/api` prefix.

#### Analytics Handlers (`handler/analytics_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `GetDailyIncentiveTotals` | `/api/get_daily_incentive_totals` | Session | Daily incentive totals for last 30 days |
| `GetUnoAppReferrals` | `/api/get_uno_app_referrals` | Session | Fetch referrals from uno-app |
| `GetUnoAppVisitorStats` | `/api/get_uno_app_visitor_stats` | Session | Fetch visitor stats from uno-app |

#### Dashboard Handlers (`handler/dashboard_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `GetAgentsOverviewFromDb` | `/api/get_agents_overview_from_db` | Session | Agents with licenses for commission calc |
| `GetRewardsFromDb` | `/api/get_rewards_from_db` | Session | Rewards for dashboard charts |

#### License Handlers (`handler/license_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `ListLicenses` | `/api/list_licenses` | Session | List all licenses with pagination |
| `GetLicense` | `/api/get_license` | Session | Get single license by ID |
| `CreateLicense` | `/api/create_license` | Session | Create new license |
| `UpdateLicense` | `/api/update_license` | Session | Update existing license |
| `DeleteLicense` | `/api/delete_license` | Session | Delete a license |
| `ImportLicenses` | `/api/import_licenses` | Session | Bulk import licenses from CSV |
| `PublishToMarketplace` | `/api/publish_to_marketplace` | Session | Publish licenses to uno-app marketplace |
| `GetMarketplaceLicenses` | `/api/get_marketplace_licenses` | Session | Get licenses on marketplace |
| `UnpublishFromMarketplace` | `/api/unpublish_from_marketplace` | Session | Remove from marketplace |

#### Sync Job Handlers (`handler/sync_job_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `ListSyncJobs` | `/api/list_sync_jobs` | Session | List all sync jobs |
| `GetSyncJob` | `/api/get_sync_job` | Session | Get single job by ID |
| `RunSyncJob` | `/api/run_sync_job` | Session | Create and enqueue rewards sync job |
| `RerunSyncJob` | `/api/rerun_sync_job` | Session | Re-run existing job |
| `DeleteSyncJob` | `/api/delete_sync_job` | Session | Delete a sync job |
| `RunLicenseSyncJob` | `/api/run_license_sync_job` | Session | Create and enqueue license sync job |
| `ProcessPendingRetries` | `/api/process_pending_retries` | Session | Process failed jobs |
| `GetJobsBadgeCount` | `/api/get_jobs_badge_count` | Session | Get badge count for completed jobs |

#### Rewards Handlers (`handler/rewards_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `SyncRewardsIfStale` | `/api/sync_rewards_if_stale` | Session | Auto-sync rewards if data is stale |
| `GetLicenseRewardsSummary` | `/api/get_license_rewards_summary` | Session | Rewards summary for single license |
| `GetLicensesRewardsSummary` | `/api/get_licenses_rewards_summary` | Session | Batch rewards summary |
| `GetLicenseDailyRewards` | `/api/get_license_daily_rewards` | Session | Daily rewards for charts (30 days) |
| `GetAgentDailyRewards` | `/api/get_agent_daily_rewards` | Session | Daily rewards for agent (30 days) |

#### File Handlers (`handler/file_handler.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `UploadFile` | `/api/upload_file` | Session | Upload file to storage |
| `GetSignedUrl` | `/api/get_signed_url` | Session | Get signed URL for file |

#### Schema/CMS Handlers (`api/schema_client.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `GetSchemas` | `/api/get_schemas` | Session | List all content schemas |
| `GetSchema` | `/api/get_schema` | Session | Get single schema |
| `CreateSchema` | `/api/create_schema` | Session | Create new schema |
| `UpdateSchema` | `/api/update_schema` | Session | Update schema |
| `DeleteSchema` | `/api/delete_schema` | Session | Delete schema |
| `ListContentItems` | `/api/list_content_items` | Session | List content items |
| `GetContentItem` | `/api/get_content_item` | Session | Get content item |
| `SaveContentItem` | `/api/save_content_item` | Session | Create/update content item |
| `DeleteContentItem` | `/api/delete_content_item` | Session | Delete content item |
| `PublishContentItem` | `/api/publish_content_item` | Session | Publish content |
| `ArchiveContentItem` | `/api/archive_content_item` | Session | Archive content |
| `GetContentItemVersions` | `/api/get_content_item_versions` | Session | Get version history |
| `RevertContentItem` | `/api/revert_content_item` | Session | Revert to version |

#### Home Page Handlers (`api/home_client.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `GetHomePage` | `/api/get_home_page` | Session | Get home page content |
| `SaveHomePage` | `/api/save_home_page` | Session | Save home page |
| `PublishHomePage` | `/api/publish_home_page` | Session | Publish home page |
| `GetHomePageVersions` | `/api/get_home_page_versions` | Session | Home page history |
| `RevertHomePage` | `/api/revert_home_page` | Session | Revert home page |
| `CreateHomePreview` | `/api/create_home_preview` | Session | Generate preview URL |

#### FAQ Handlers (`api/faq_client.rs`)

| Function | Endpoint | Auth | Description |
|----------|----------|------|-------------|
| `GetFaqPage` | `/api/get_faq_page` | Session | Get FAQ content |
| `SaveFaqPage` | `/api/save_faq_page` | Session | Save FAQ content |
| `PublishFaqPage` | `/api/publish_faq_page` | Session | Publish FAQ |
| `GetFaqPageVersions` | `/api/get_faq_page_versions` | Session | FAQ history |
| `RevertFaqPage` | `/api/revert_faq_page` | Session | Revert FAQ |
| `CreateFaqPreview` | `/api/create_faq_preview` | Session | Generate FAQ preview URL |

### UNO-ADMIN WebSocket Handlers

WebSocket implementation in `uno-admin/src/ws/handler.rs`.

| Endpoint | Handler | Auth | Description |
|----------|---------|------|-------------|
| `/ws` | `ws_handler` | No | WebSocket upgrade and connection management |

#### WebSocket Message Types

| Event Type | Direction | Payload | Description |
|------------|-----------|---------|-------------|
| `ping` | Client -> Server | `{}` | Keep-alive ping |
| `pong` | Server -> Client | `{}` | Pong response |
| `job_update` | Server -> Client | `SyncJobEntity` | Job status update broadcast |
| `job_deleted` | Server -> Client | `{ job_id: String }` | Job deletion broadcast |
| `subscribe` | Client -> Server | `{ topics: Vec<String> }` | Subscribe to topics |
| `unsubscribe` | Client -> Server | `{ topics: Vec<String> }` | Unsubscribe from topics |

#### Broadcast Functions

| Function | Module | Description |
|----------|--------|-------------|
| `broadcast_job_update` | `ws/handler.rs` | Broadcast job status change to all connected clients |
| `broadcast_job_deleted` | `ws/handler.rs` | Broadcast job deletion to all connected clients |

### UNO-ADMIN Repositories

#### Repository Traits (`repository/traits/mod.rs`)

| Trait | Methods | Implementations |
|-------|---------|-----------------|
| `AgentRepositoryTrait` | `save_agent`, `update_agent`, `get_agent_by_id`, `get_agent_by_email`, `list_agents`, `delete_agent` | `PgAgentRepository`, `ScyllaAgentRepository` |
| `LicenseRepositoryTrait` | `create_license`, `update_license`, `get_license_by_id`, `get_license_by_license_id`, `list_licenses`, `get_licenses_by_agent_id`, `delete_license`, `upsert_license_from_api`, `set_is_published`, `update_marketplace_status`, `get_marketplace_licenses`, `unpublish_from_marketplace` | `PgLicenseRepository`, `ScyllaLicenseRepository` |
| `RewardRepositoryTrait` | `upsert_rewards`, `list_rewards`, `list_rewards_by_license_id`, `list_rewards_by_license_id_and_date`, `get_total_earnings_by_license_id`, `count_rewards_by_license_id`, `get_daily_totals` | `PgRewardRepository`, `ScyllaRewardRepository` |
| `SyncJobRepositoryTrait` | `create_job`, `get_job_by_id`, `get_job_by_date`, `list_jobs`, `list_jobs_by_status`, `list_pending_retries`, `mark_running`, `mark_completed_with_context`, `mark_failed`, `reset_job`, `update_job_context`, `delete_job` | `PgSyncJobRepository`, `ScyllaSyncJobRepository` |

#### PostgreSQL Implementations (`repository/postgres/`)

| Repository | File | Database Table |
|------------|------|----------------|
| `PgAgentRepository` | `agent.rs` | `agents` |
| `PgLicenseRepository` | `license.rs` | `licenses` (admin) |
| `PgRewardRepository` | `reward.rs` | `rewards` |
| `PgSyncJobRepository` | `sync_job.rs` | `sync_jobs` |

### UNO-ADMIN Sync Services

Services in `uno-admin/src/logic/`.

#### RewardsSyncService (`rewards_sync_service.rs`)

| Method | Description |
|--------|-------------|
| `new(pool)` | Create service instance |
| `sync_rewards_for_date(date)` | Sync rewards from Unity API for specific date |
| `create_pending_job(date)` | Create pending job for background execution |
| `execute_job(job_id, date)` | Execute rewards sync job |
| `reset_and_execute(job_id, date)` | Reset and re-execute job |
| `list_jobs()` | List all sync jobs |
| `get_job(id)` | Get job by ID |
| `delete_job(id)` | Delete a job |
| `process_pending_retries()` | Process failed jobs |

#### LicenseSyncService (`license_sync_service.rs`)

| Method | Description |
|--------|-------------|
| `new(pool)` | Create service instance |
| `sync_all_licenses()` | Sync all licenses from Unetwork API |
| `create_pending_job()` | Create pending job |
| `execute_job(job_id)` | Execute license sync job |
| `reset_and_execute(job_id)` | Reset and re-execute |
| `is_running()` | Check if sync is running |
| `resume_failed_job(id)` | Resume failed job |
| `rerun_job(id)` | Rerun specific job |

#### ReferralSyncService (`referral_sync_service.rs`)

| Method | Description |
|--------|-------------|
| `new(pool)` | Create service instance |
| `push_agents_as_referrals()` | Push local agents to uno-app |
| `pull_referrals_as_agents()` | Pull referrals from uno-app as agents |
| `sync()` | Bi-directional sync (push then pull) |

#### MarketplaceService (`marketplace_service.rs`)

| Method | Description |
|--------|-------------|
| `new(pool)` | Create service instance |
| `publish_to_marketplace(ids)` | Publish licenses to uno-app |
| `poll_claimed_licenses(since)` | Poll for claimed licenses |
| `poll_and_sync()` | Full poll and sync |
| `get_marketplace_licenses()` | Get local marketplace licenses |
| `sync_and_get_licenses()` | Sync and return enriched licenses |
| `unpublish_from_marketplace(ids)` | Remove from marketplace |

#### Background Job System

| Component | Module | Description |
|-----------|--------|-------------|
| `JobCommand` | `job_queue.rs` | Enum: `RewardsSync`, `LicenseSync`, `RerunJob` |
| `enqueue(cmd)` | `job_queue.rs` | Add command to job queue |
| `init_job_queue()` | `job_queue.rs` | Initialize mpsc channel (called once at startup) |
| `start_job_worker()` | `job_worker.rs` | Spawn background worker task |

#### Health & Monitoring (`logic/health.rs`)

| Component | Description |
|-----------|-------------|
| `HealthService` | Service for health checks |
| `HealthCheck` | Individual health check result |
| `HealthStatus` | Overall health status |
| `StartupChecks` | Checks run at application startup |

---

## Summary Statistics

| Category | UNO-APP | UNO-ADMIN |
|----------|---------|-----------|
| HTTP Routes | 10 | N/A (via server functions) |
| Server Functions | 16 | 55+ |
| Repositories | 5 | 4 traits, 4 Postgres impls |
| Background Workers | 1 (Content Scheduler) | 1 (Job Worker) |
| Sync Services | 0 | 4 |
| WebSocket Handlers | 0 | 1 |

---

## File References

### UNO-APP
- Routes: `/uno-app/src/server/handlers/mod.rs`
- Server Functions: `/uno-app/src/api/*.rs`
- Repositories: `/uno-app/src/server/repositories/*.rs`
- Scheduler: `/uno-app/src/server/scheduler/mod.rs`
- File Handler: `/uno-app/src/server/handlers/file_handler.rs`

### UNO-ADMIN
- Handlers: `/uno-admin/src/handler/*.rs`
- API Clients: `/uno-admin/src/api/*.rs`
- WebSocket: `/uno-admin/src/ws/handler.rs`
- Repository Traits: `/uno-admin/src/repository/traits/mod.rs`
- Repository Impl: `/uno-admin/src/repository/postgres/*.rs`
- Sync Services: `/uno-admin/src/logic/*_sync_service.rs`
- Job Queue: `/uno-admin/src/logic/job_queue.rs`
- Job Worker: `/uno-admin/src/logic/job_worker.rs`
- Main: `/uno-admin/src/main.rs`
