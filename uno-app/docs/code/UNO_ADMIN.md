# UNO-ADMIN - Architecture Overview

A full-stack admin dashboard built with Leptos for managing the UNO license ecosystem. Features real-time synchronization, WebSocket-based job monitoring, and a ScyllaDB persistence layer.

---

## Project Statistics

- **Total Rust Files**: 165
- **Framework**: Leptos 0.8.2 (Rust + WASM)
- **Server**: Actix-web 4 (SSR)
- **Database**: ScyllaDB (distributed NoSQL)
- **Real-time**: WebSocket via Actix-ws
- **Authentication**: SIWE (Sign-In with Ethereum)
- **UI Library**: ember-fx (Prime design system)

---

## Directory Structure

```
uno-admin/
├── src/
│   ├── main.rs              # Server entry point with background schedulers
│   ├── lib.rs               # Library root & module exports
│   ├── app.rs               # Main Leptos app component with routing
│   ├── api/                 # API clients & authentication (13 files)
│   │   ├── mod.rs           # Module exports
│   │   ├── config.rs        # Unity API configuration
│   │   ├── types.rs         # Request/response types
│   │   ├── handlers.rs      # API request handlers
│   │   ├── http_client.rs   # HTTP client (SSR-only)
│   │   ├── uno_client.rs    # UNO API client for license management
│   │   ├── content_client.rs # Content CMS client for uno-app
│   │   ├── schema_client.rs  # Schema-driven CMS client
│   │   ├── schema_types.rs   # Schema and content item types
│   │   ├── section_types.rs  # Generic section types
│   │   ├── faq_client.rs     # FAQ page management
│   │   ├── faq_types.rs      # FAQ-specific types
│   │   ├── home_client.rs    # Home page management
│   │   └── wallet_auth.rs    # SIWE authentication service
│   ├── db/                  # Database layer (8 files)
│   │   ├── mod.rs           # Database connection & global session
│   │   ├── connection.rs    # ScyllaDB connection setup
│   │   ├── error.rs         # Database error types
│   │   ├── seeder.rs        # Database seeding utilities
│   │   └── migrations/      # Migration system
│   │       ├── mod.rs       # Migration runner coordination
│   │       ├── loader.rs    # Loads CQL migration files
│   │       ├── runner.rs    # Executes migrations
│   │       ├── tracker.rs   # Tracks migration history
│   │       └── types.rs     # Migration type definitions
│   ├── models/              # Data models (6 files)
│   │   ├── mod.rs           # Model re-exports
│   │   ├── entity.rs        # Database entity models
│   │   ├── license.rs       # License configuration & summaries
│   │   ├── agent.rs         # Agent performance & ULO data
│   │   ├── node.rs          # Node data & summaries
│   │   └── revenue.rs       # Revenue models & allocations
│   ├── repository/          # Data access layer (14 files)
│   │   ├── mod.rs           # Repository exports
│   │   ├── traits/          # Repository interfaces (7 files)
│   │   │   ├── mod.rs
│   │   │   ├── license_repository_trait.rs
│   │   │   ├── agent_repository_trait.rs
│   │   │   ├── node_repository_trait.rs
│   │   │   ├── reward_repository_trait.rs
│   │   │   ├── license_analytics_repository_trait.rs
│   │   │   └── sync_job_repository_trait.rs
│   │   └── scylla/          # ScyllaDB implementations (7 files)
│   │       ├── mod.rs
│   │       ├── license_repository.rs
│   │       ├── agent_repository.rs
│   │       ├── node_repository.rs
│   │       ├── reward_repository.rs
│   │       ├── license_analytics_repository.rs
│   │       └── sync_job_repository.rs
│   ├── logic/               # Business logic (12 files)
│   │   ├── mod.rs           # Logic service exports
│   │   ├── license_logic.rs # License computation logic
│   │   ├── user_logic.rs    # User authentication & role logic
│   │   ├── dashboard_overview.rs # Dashboard statistics
│   │   ├── agent_logic.rs   # Agent performance calculations
│   │   ├── node_logic.rs    # Node analytics
│   │   ├── rewards_sync_service.rs # Syncs rewards (hourly)
│   │   ├── license_sync_service.rs # Syncs licenses (10 min)
│   │   ├── marketplace_service.rs  # Syncs claimed licenses
│   │   ├── referral_sync_service.rs # Syncs referrals
│   │   ├── job_queue.rs     # MPSC channel-based job queue
│   │   └── job_worker.rs    # Background worker
│   ├── handler/             # Server functions (9 files)
│   │   ├── mod.rs           # Handler exports
│   │   ├── license_handler.rs
│   │   ├── node_handler.rs
│   │   ├── agent_handler.rs
│   │   ├── user_handler.rs
│   │   ├── rewards_handler.rs
│   │   ├── sync_job_handler.rs
│   │   ├── dashboard_handler.rs
│   │   ├── analytics_handler.rs
│   │   └── file_handler.rs
│   ├── ui/                  # UI components (95 files)
│   │   ├── mod.rs           # UI module exports
│   │   ├── pages/           # Page components (35 files)
│   │   ├── components/      # UI components (60 files)
│   │   ├── state/           # Global app state (3 files)
│   │   ├── context/         # Context providers (3 files)
│   │   └── hooks/           # Custom hooks (2 files)
│   ├── ws/                  # WebSocket (3 files)
│   │   ├── mod.rs           # WebSocket exports
│   │   ├── broadcaster.rs   # tokio::sync::broadcast for job events
│   │   └── handler.rs       # WebSocket endpoint handler
│   ├── storage/             # Local storage (2 files)
│   │   ├── mod.rs
│   │   └── local_store.rs
│   ├── routes/              # Route definitions (1 file)
│   └── utils/               # Utilities (2 files)
│       ├── mod.rs
│       └── base64_converter.rs
├── assets/                  # Static assets
├── style/                   # SCSS stylesheets
├── migrations/              # CQL migration files
└── Cargo.toml               # Dependencies & features
```

---

## Architecture

### Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     UI Layer (Leptos)                        │
│  Pages, Components, Forms, State, Context, Hooks            │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   Handler Layer (Server Fns)                 │
│  license_handler, rewards_handler, sync_job_handler, etc.   │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Logic Layer                               │
│  Sync Services, Job Queue, Business Logic                   │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                Repository Layer (Traits)                     │
│  License, Agent, Node, Reward, SyncJob Repositories         │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│               Database Layer (ScyllaDB)                      │
│  Distributed NoSQL with migrations & seeding                │
└─────────────────────────────────────────────────────────────┘
```

### Cross-Cutting Concerns

```
┌─────────────────────────────────────────────────────────────┐
│  API Layer: uno_client, content_client, schema_client       │
│  (External communication with uno-app, Unetwork API)        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  WebSocket Layer: broadcaster, handler                      │
│  (Real-time job status updates)                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  Authentication: wallet_auth (SIWE with Web3)               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│  Storage: local_store (browser localStorage)                │
└─────────────────────────────────────────────────────────────┘
```

---

## Routing

### Client-Side Routes

| Route | Page | Purpose |
|-------|------|---------|
| `/` | HomePage | Dashboard with overview stats |
| `/analytics` | AnalyticsPage | Detailed analytics for rewards/uptime |
| `/rewards` | RewardsPage | Rewards tracking and distribution |
| `/licenses` | LicenseListPage | License list with filters |
| `/licenses/:id` | LicenseDetailPage | License details and history |
| `/agents` | AgentListPage | Agent performance overview |
| `/agents/:name` | AgentDetailPage | Agent details and earnings |
| `/nodes` | NodeListPage | Node status list |
| `/nodes/:id` | NodeDetailPage | Node details and licenses |
| `/marketplace` | MarketplaceListPage | Marketplace license listing |
| `/cms` | CmsLandingPage | CMS landing page |
| `/content` | ContentListPage | Content list |
| `/content/home` | HomePageEditor | Home page CMS editor |
| `/content/faq` | FaqPageEditor | FAQ page CMS editor |
| `/content/:id` | ContentEditorPage | Generic content editor |
| `/schemas` | SchemaListPage | Schema management |
| `/schemas/:id` | SchemaEditorPage | Schema editor |
| `/reviews` | ReviewsDashboardPage | Content review workflow |
| `/publish` | PublishQueuePage | Publish queue management |
| `/jobs` | JobsListPage | Background job monitoring |
| `/settings` | SettingsPage | User preferences |
| `/audit` | AuditLogPage | Audit log viewer |

### Route Definition (app.rs)

```rust
Router {
    Routes {
        Route(path: "")              -> HomePage
        Route(path: "marketplace")   -> MarketplaceListPage
        Route(path: "analytics")     -> AnalyticsPage
        Route(path: "rewards")       -> RewardsPage
        Route(path: "nodes")         -> NodeListPage
        Route(path: "nodes/:id")     -> NodeDetailPage
        Route(path: "licenses")      -> LicenseListPage
        Route(path: "licenses/:id")  -> LicenseDetailPage
        Route(path: "agents")        -> AgentListPage
        Route(path: "agents/:name")  -> AgentDetailPage
        Route(path: "cms")           -> CmsLandingPage
        Route(path: "content")       -> ContentListPage
        Route(path: "content/home")  -> HomePageEditor
        Route(path: "content/faq")   -> FaqPageEditor
        Route(path: "content/:id")   -> ContentEditorPage
        Route(path: "schemas")       -> SchemaListPage
        Route(path: "schemas/:id")   -> SchemaEditorPage
        Route(path: "reviews")       -> ReviewsDashboardPage
        Route(path: "publish")       -> PublishQueuePage
        Route(path: "audit")         -> AuditLogPage
        Route(path: "jobs")          -> JobsListPage
        Route(path: "settings")      -> SettingsPage
    }
}
```

---

## Background Sync Services

### Scheduled Tasks (main.rs)

| Service | Interval | Purpose |
|---------|----------|---------|
| **RewardsSyncService** | 1 minute | Syncs rewards from Unity API to ScyllaDB |
| **LicenseSyncService** | 10 minutes | Syncs licenses from Unetwork API |
| **MarketplaceService** | 5 minutes | Syncs claimed licenses from uno-app |
| **ReferralSyncService** | 5 minutes | Bi-directional referral sync |
| **Job Watchdog** | 60 seconds | Marks stuck jobs as failed |

### Job Queue Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    HTTP Request                              │
│  POST /api/sync-job/run                                     │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                Create Pending Job                            │
│  service.create_pending_job(&date).await?                   │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                Enqueue Command                               │
│  enqueue(JobCommand::RewardsSync { job_id, target_date })   │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Job Worker (mpsc receiver)                      │
│  Runs in background, processes queue                        │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Execute Sync Service                            │
│  RewardsSyncService::execute_job(&job_id, &date).await?     │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Update Database                                 │
│  repository.update_job_status(completed).await?             │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Broadcast via WebSocket                         │
│  broadcast_job_update(&completed_job)                       │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Client Receives Update                          │
│  use_job_websocket() hook receives event                    │
└─────────────────────────────────────────────────────────────┘
```

### Sync Service Implementation

```rust
// RewardsSyncService (logic/rewards_sync_service.rs)
pub struct RewardsSyncService {
    pool: Arc<Session>,
    api_config: UnityApiConfig,
}

impl RewardsSyncService {
    pub async fn execute_job(&self, job_id: &str, target_date: &str) -> Result<(), Error> {
        // 1. Mark job as running
        self.update_job_status(job_id, "running").await?;

        // 2. Fetch rewards from Unity API
        let allocations = self.fetch_allocations(target_date).await?;

        // 3. Store in ScyllaDB
        let inserted = self.store_rewards(allocations).await?;

        // 4. Mark job as completed
        self.complete_job(job_id, inserted).await?;

        // 5. Broadcast completion
        broadcast_job_update(&job);

        Ok(())
    }
}
```

---

## API Integrations

### External APIs

| Client | Target | Authentication | Purpose |
|--------|--------|----------------|---------|
| `UnoLicenseClient` | Unetwork API | JWT Bearer | Fetch licenses, analytics |
| `ContentClient` | uno-app CMS | HMAC-signed | CRUD content items |
| `SchemaClient` | uno-app Schemas | HMAC-signed | Schema management |
| `FaqClient` | uno-app FAQ | HMAC-signed | FAQ page management |
| `HomeClient` | uno-app Home | HMAC-signed | Home page management |
| `WalletAuthService` | Web3 | SIWE | Wallet authentication |

### API Client Configuration

```rust
// Unity API Configuration (api/config.rs)
pub struct UnityApiConfig {
    pub base_url: String,      // https://api.unityedge.io
    pub api_key: Option<String>,
    pub jwt_token: String,
}

impl UnityApiConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            base_url: env::var("UNITY_API_URL")?,
            api_key: env::var("UNITY_API_KEY").ok(),
            jwt_token: env::var("UNITY_JWT_TOKEN")?,
        })
    }

    pub fn rpc_url(&self, function: &str) -> String {
        format!("{}/rest/v1/rpc/{}", self.base_url, function)
    }

    pub fn functions_url(&self, function: &str) -> String {
        format!("{}/functions/v1/{}", self.base_url, function)
    }
}
```

### UNO Client (uno-api integration)

```rust
// api/uno_client.rs
use uno_api::client::UnoApiClient;
use uno_api::config::ClientConfig;

pub struct UnoLicenseClient {
    client: UnoApiClient,
}

impl UnoLicenseClient {
    pub fn from_env() -> Result<Self, Error> {
        let config = ClientConfig::new(
            &env::var("UNO_API_URL")?,
            &env::var("ADMIN_CLIENT_ID")?,
            &env::var("ADMIN_SECRET_KEY")?,
        );
        Ok(Self {
            client: UnoApiClient::new(config),
        })
    }

    pub async fn list_licenses(&self, page: i32, page_size: i32)
        -> Result<Vec<LicenseDto>, Error>
    {
        self.client.search_licenses(
            LicenseFilters::new(),
            PaginationParams::new(page, page_size),
        ).await.map(|p| p.items)
    }
}
```

---

## Authentication

### SIWE (Sign-In with Ethereum)

```rust
// api/wallet_auth.rs
pub struct Web3AuthRequest {
    pub chain: String,        // "ethereum", "polygon", etc.
    pub message: String,      // SIWE message
    pub signature: String,    // Signature from wallet
}

pub struct Web3AuthResponse {
    pub access_token: String,
    pub user: UnetworkUser,
}

pub struct UnetworkUser {
    pub id: String,
    pub role: String,           // "admin", "agent", "ulo"
    pub email: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
}

pub struct WalletAuthService;

impl WalletAuthService {
    pub async fn authenticate(req: Web3AuthRequest) -> Result<Web3AuthResponse, AuthError> {
        // 1. Build SIWE message
        let message = build_unetwork_siwe_message(&req.message)?;

        // 2. Verify signature
        let recovered_address = verify_signature(&message, &req.signature)?;

        // 3. Lookup or create user
        let user = find_or_create_user(&recovered_address).await?;

        // 4. Generate access token
        let token = generate_jwt(&user)?;

        Ok(Web3AuthResponse {
            access_token: token,
            user,
        })
    }
}
```

### User Roles

| Role | Permissions |
|------|-------------|
| **Admin** | Full dashboard access, all CRUD operations |
| **Agent** | View and manage own agents, view licenses |
| **ULO** | View own licenses and rewards |

### User Role Context

```rust
// ui/context/user_role_context.rs
#[derive(Clone)]
pub struct UserRoleContext {
    pub user: RwSignal<Option<UnetworkUser>>,
    pub is_admin: Memo<bool>,
    pub is_agent: Memo<bool>,
    pub is_ulo: Memo<bool>,
}

impl UserRoleContext {
    pub fn provide() {
        let user = RwSignal::new(None);
        let is_admin = Memo::new(move |_| {
            user.get().map(|u| u.role == "admin").unwrap_or(false)
        });
        // ... other memos

        provide_context(UserRoleContext { user, is_admin, is_agent, is_ulo });
    }
}
```

---

## Database (ScyllaDB)

### Connection Setup

```rust
// db/connection.rs
pub async fn create_session() -> Result<Session, DbError> {
    let host = env::var("SCYLLA_DB_HOST")?;
    let port = env::var("SCYLLA_DB_PORT")?.parse()?;
    let keyspace = env::var("SCYLLA_DB_KEYSPACE")?;
    let username = env::var("SCYLLA_DB_USERNAME")?;
    let password = env::var("SCYLLA_DB_PASSWORD")?;

    let session = SessionBuilder::new()
        .known_node(format!("{}:{}", host, port))
        .user(username, password)
        .build()
        .await?;

    session.use_keyspace(&keyspace, false).await?;

    Ok(session)
}
```

### Entity Models

**LicenseEntity:**
```rust
pub struct LicenseEntity {
    pub license_id: String,
    pub node_id: String,
    pub alias: Option<String>,
    pub agent_id: String,
    pub ulo_name: String,
    pub uno_share: f64,
    pub agent_share: f64,
    pub ulo_share: f64,
    pub uptime: f64,
    pub is_online: bool,
    pub lease_code: Option<String>,
    pub lease_share_percentage: f64,
    pub lease_from: Option<String>,
    pub lease_to: Option<String>,
    pub is_on_marketplace: bool,
    pub marketplace_status: Option<String>,
    pub synced_at: Option<String>,
}
```

**SyncJobEntity:**
```rust
pub struct SyncJobEntity {
    pub id: String,
    pub job_type: String,        // "rewards_sync" | "license_sync"
    pub status: String,          // "pending" | "running" | "completed" | "failed"
    pub target_date: String,
    pub records_fetched: i32,
    pub records_inserted: i32,
    pub error_message: Option<String>,
    pub job_context: Option<String>,  // JSON
    pub created_at: String,
    pub started_at: Option<String>,
    pub updated_at: String,
}
```

**RewardEntity:**
```rust
pub struct RewardEntity {
    pub license_id: String,
    pub date: String,
    pub amount_micros: i64,
    pub allocation_id: String,
    pub node_id: String,
    pub completed_at: String,
    pub created_at: String,
}
```

**AgentEntity:**
```rust
pub struct AgentEntity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub country: String,
    pub commission_percent: f64,
    pub referral_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

### Repository Traits

```rust
// repository/traits/license_repository_trait.rs
#[async_trait]
pub trait LicenseRepositoryTrait: Send + Sync {
    async fn get_all(&self) -> Result<Vec<LicenseEntity>, DbError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<LicenseEntity>, DbError>;
    async fn upsert(&self, license: &LicenseEntity) -> Result<(), DbError>;
    async fn upsert_batch(&self, licenses: &[LicenseEntity]) -> Result<(), DbError>;
    async fn delete(&self, id: &str) -> Result<(), DbError>;
    async fn count(&self) -> Result<i64, DbError>;
    async fn count_online(&self) -> Result<i64, DbError>;
}

// repository/traits/sync_job_repository_trait.rs
#[async_trait]
pub trait SyncJobRepositoryTrait: Send + Sync {
    async fn create(&self, job: &SyncJobEntity) -> Result<(), DbError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<SyncJobEntity>, DbError>;
    async fn list_recent(&self, limit: i32) -> Result<Vec<SyncJobEntity>, DbError>;
    async fn update_status(&self, id: &str, status: &str) -> Result<(), DbError>;
    async fn mark_completed(&self, id: &str, fetched: i32, inserted: i32) -> Result<(), DbError>;
    async fn mark_failed(&self, id: &str, error: &str) -> Result<(), DbError>;
    async fn find_stuck_jobs(&self, max_age_secs: i64) -> Result<Vec<SyncJobEntity>, DbError>;
}
```

### Migration System

```rust
// db/migrations/runner.rs
pub struct MigrationRunner {
    session: Arc<Session>,
    loader: MigrationLoader,
    tracker: MigrationTracker,
}

impl MigrationRunner {
    pub async fn run_pending(&self) -> Result<Vec<String>, MigrationError> {
        let applied = self.tracker.get_applied().await?;
        let all = self.loader.load_all()?;

        let pending: Vec<_> = all
            .iter()
            .filter(|m| !applied.contains(&m.name))
            .collect();

        for migration in pending {
            self.execute(migration).await?;
            self.tracker.mark_applied(&migration.name).await?;
        }

        Ok(pending.iter().map(|m| m.name.clone()).collect())
    }
}
```

---

## State Management

### Global App State

```rust
// ui/state/app_state.rs
#[derive(Clone)]
pub struct DashboardState {
    pub stats: RwSignal<DashboardStats>,
    pub revenue_data: RwSignal<RevenueData>,
    pub is_loading: RwSignal<bool>,
    pub jobs_new_count: RwSignal<u32>,
}

pub struct DashboardStats {
    pub total_licenses: usize,
    pub online_licenses: usize,
    pub total_agents: usize,
    pub total_ulos: usize,
    pub total_revenue_micros: i64,
}

pub struct RevenueData {
    pub daily_earnings: Vec<DailyEarnings>,
    pub total_earnings: i64,
    pub unclaimed_balance: i64,
}

// Provide globally
pub fn provide_dashboard_state() {
    let state = DashboardState {
        stats: RwSignal::new(DashboardStats::default()),
        revenue_data: RwSignal::new(RevenueData::default()),
        is_loading: RwSignal::new(false),
        jobs_new_count: RwSignal::new(0),
    };
    provide_context(state);
}

// Usage in components
let state = expect_context::<DashboardState>();
state.stats.set(new_stats);  // Triggers reactive updates
```

### Context Providers

```rust
// ui/context/theme_context.rs
#[derive(Clone)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub toggle: Callback<()>,
}

pub enum Theme {
    Light,
    Dark,
}

// ui/context/nav_position_context.rs
#[derive(Clone)]
pub struct NavPositionContext {
    pub position: RwSignal<NavPosition>,
    pub toggle: Callback<()>,
}

pub enum NavPosition {
    Sidebar,
    Bottom,
}
```

---

## WebSocket Real-time Updates

### Broadcaster

```rust
// ws/broadcaster.rs
use tokio::sync::broadcast;

pub struct JobBroadcaster {
    sender: broadcast::Sender<JobEvent>,
}

impl JobBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn broadcast(&self, event: JobEvent) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<JobEvent> {
        self.sender.subscribe()
    }
}

pub enum JobEvent {
    Started { job_id: String },
    Progress { job_id: String, progress: u32 },
    Completed { job_id: String, result: JobResult },
    Failed { job_id: String, error: String },
}
```

### WebSocket Handler

```rust
// ws/handler.rs
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    broadcaster: web::Data<JobBroadcaster>,
) -> Result<HttpResponse, Error> {
    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    actix_rt::spawn(async move {
        let mut rx = broadcaster.subscribe();

        while let Ok(event) = rx.recv().await {
            let json = serde_json::to_string(&event).unwrap();
            if session.text(json).await.is_err() {
                break;
            }
        }
    });

    Ok(response)
}
```

### Client Hook

```rust
// ui/hooks/use_job_websocket.rs
pub fn use_job_websocket() -> Signal<Option<JobEvent>> {
    let event = RwSignal::new(None);

    #[cfg(feature = "hydrate")]
    {
        use_effect(move |_| {
            let ws = WebSocket::new("/ws/jobs").unwrap();

            ws.set_onmessage(move |msg| {
                if let Ok(e) = serde_json::from_str(&msg.data()) {
                    event.set(Some(e));
                }
            });

            on_cleanup(move || ws.close());
        });
    }

    event.into()
}
```

---

## UI Components

### Component Hierarchy

```
App (root)
├── ThemeContextProvider
├── NavPositionContextProvider
├── UserRoleContextProvider
│   └── DashboardApp
│       └── Router
│           ├── AppShell (layout wrapper)
│           │   ├── Header
│           │   ├── SidebarNav / BottomNav
│           │   └── TopNav
│           └── Routes (all pages)
└── Page Components
    ├── LicensesOverviewSection
    │   └── LicenseTable, StatCards, Charts
    ├── AgentsOverviewSection
    │   └── AgentTable, TopPerformers
    ├── Forms (SchemaForm, MediaUpload, etc.)
    ├── Charts (AreaChart, LineChart, PieChart)
    └── Tables & Lists
```

### Layout Components

| Component | File | Purpose |
|-----------|------|---------|
| AppShell | `layout/app_shell.rs` | Main container with sidebar/nav |
| Header | `layout/header.rs` | Page title and breadcrumbs |
| TopNav | `layout/top_nav.rs` | Top navigation bar |
| SidebarNav | `layout/sidebar_nav.rs` | Desktop sidebar |
| BottomNav | `layout/bottom_nav.rs` | Mobile navigation |

### Chart Components

| Component | File | Purpose |
|-----------|------|---------|
| AreaChart | `charts/area_chart.rs` | Time series visualization |
| BarChart | `charts/bar_chart.rs` | Categorical data |
| LineChart | `charts/line_chart.rs` | Multiple series lines |
| PieChart | `charts/pie_chart.rs` | Distribution/percentages |
| DailyEarningsChart | `charts/daily_earnings_chart.rs` | Revenue over time |
| MiniUptimeChart | `charts/mini_uptime_chart.rs` | Sparkline visualization |

### Form Components

| Component | File | Purpose |
|-----------|------|---------|
| SchemaForm | `forms/schema_form.rs` | Dynamic form from schema |
| FieldRenderer | `forms/field_renderer.rs` | Renders fields by type |
| TextField | `forms/text_field.rs` | String input |
| NumberField | `forms/number_field.rs` | Number input |
| BooleanField | `forms/boolean_field.rs` | Checkbox |
| SelectField | `forms/select_field.rs` | Dropdown select |
| ListField | `forms/list_field.rs` | Array editor |
| RepeaterField | `forms/repeater_field.rs` | Repeating sections |
| MediaField | `forms/media_field.rs` | File upload |
| RichTextField | `forms/rich_text_field.rs` | HTML editor |
| ReferencePicker | `forms/reference_picker.rs` | Related item picker |
| GcsMediaUpload | `forms/gcs_media_upload.rs` | GCS upload |

### Common Components

| Component | File | Purpose |
|-----------|------|---------|
| StatCard | `cards/stat_card.rs` | KPI display card |
| StatGrid | `cards/stat_grid.rs` | Grid of stat cards |
| Avatar | `common/avatar.rs` | User profile picture |
| Badge | `common/badge.rs` | Status indicator |
| Icon | `common/icon.rs` | Icon wrapper |
| Dropdown | `common/dropdown.rs` | Menu component |
| SearchBar | `common/search_bar.rs` | Search input |
| ThemeToggle | `common/theme_toggle.rs` | Dark/light switch |
| LoadingProgress | `common/loading_progress.rs` | Progress bar |
| ProgressSpinner | `common/progress_spinner.rs` | Loading spinner |

---

## Server Functions (Handlers)

### License Handler

```rust
// handler/license_handler.rs
#[server(ListLicenses, "/api")]
pub async fn list_licenses(
    page: i32,
    page_size: i32,
) -> Result<Vec<LicenseDto>, ServerFnError> {
    let pool = get_db_pool()?;
    let repo = LicenseRepository::new(pool);
    let licenses = repo.get_all().await?;
    Ok(licenses.into_iter().map(LicenseDto::from).collect())
}

#[server(GetLicenseDetail, "/api")]
pub async fn get_license_detail(id: String) -> Result<LicenseDto, ServerFnError> {
    let pool = get_db_pool()?;
    let repo = LicenseRepository::new(pool);
    let license = repo.get_by_id(&id).await?
        .ok_or(ServerFnError::new("License not found"))?;
    Ok(LicenseDto::from(license))
}
```

### Sync Job Handler

```rust
// handler/sync_job_handler.rs
#[server(RunSyncJob, "/api")]
pub async fn run_sync_job(
    job_type: String,
    target_date: String,
) -> Result<SyncJobDto, ServerFnError> {
    let pool = get_db_pool()?;
    let repo = SyncJobRepository::new(pool);

    // Create pending job
    let job = SyncJobEntity::new_pending(&job_type, &target_date);
    repo.create(&job).await?;

    // Enqueue for background processing
    enqueue(JobCommand::from_job_type(&job_type, &job.id, &target_date))?;

    Ok(SyncJobDto::from(job))
}

#[server(ListSyncJobs, "/api")]
pub async fn list_sync_jobs(limit: i32) -> Result<Vec<SyncJobDto>, ServerFnError> {
    let pool = get_db_pool()?;
    let repo = SyncJobRepository::new(pool);
    let jobs = repo.list_recent(limit).await?;
    Ok(jobs.into_iter().map(SyncJobDto::from).collect())
}
```

---

## Feature Flags

```toml
[features]
default = []

# Database seeding only
seeder = ["dep:scylla", "dep:tokio"]

# Client-side rendering
csr = ["leptos/csr", "ember-fx-components/csr"]

# Hydration (WASM client)
hydrate = ["leptos/hydrate"]

# Server-side rendering (full server)
ssr = [
    "dep:actix-web",
    "dep:actix-ws",
    "dep:scylla",
    "dep:tokio",
    "dep:uuid",
    "dep:uno-api",
    "dep:reqwest",
    "dep:ember-multichain",
    "dep:file-storage",
    "leptos/ssr",
    "leptos_router/ssr",
]
```

---

## Environment Variables

```bash
# Database (ScyllaDB)
SCYLLA_DB_HOST=localhost
SCYLLA_DB_PORT=9042
SCYLLA_DB_KEYSPACE=unity_dashboard
SCYLLA_DB_USERNAME=cassandra
SCYLLA_DB_PASSWORD=cassandra

# uno-api (License management)
UNO_API_URL=http://localhost:3000
ADMIN_CLIENT_ID=admin
ADMIN_SECRET_KEY=secret

# Unity API (Rewards)
UNITY_API_URL=https://api.unityedge.io
UNITY_JWT_TOKEN=<jwt_token>

# Content API (uno-app CMS)
UNO_CONTENT_API_URL=http://localhost:3000/cms

# File Storage (GCS/S3)
FILE_STORAGE_PROVIDER=gcs
GCS_PROJECT_ID=<project>
GCS_BUCKET_NAME=<bucket>

# Logging
LOG_FORMAT=json  # or pretty
RUST_LOG=warn,uno_admin=info
```

---

## Integration with UNO Ecosystem

### Library Dependencies

| Library | Purpose |
|---------|---------|
| **uno-api** | License management, HMAC authentication |
| **ember-multichain** | Web3 wallet authentication (SIWE) |
| **ember-fx-components** | UI component library (Prime design) |
| **file-storage** | GCS/S3 file uploads |

### External Services

| Service | Integration | Purpose |
|---------|-------------|---------|
| **uno-app** | HMAC-signed HTTP | Content CMS, claimed licenses |
| **Unetwork API** | JWT Bearer HTTP | Licenses, rewards, nodes |
| **Unity API** | JWT Bearer HTTP | Reward allocations |
| **ScyllaDB** | Native driver | Local data persistence |
| **GCS/S3** | file-storage lib | Media file storage |

---

## Design Patterns

### Architectural Patterns

1. **Layered Architecture** - UI → Handler → Logic → Repository → DB
2. **Repository Pattern** - Trait-based data access abstraction
3. **Service Pattern** - Business logic encapsulation (RewardsSyncService)
4. **Event-Driven** - WebSocket broadcasting for real-time updates
5. **Background Jobs** - Async processing via job queue + worker
6. **Server Functions** - Leptos' RPC mechanism for client-server
7. **Reactive State** - Leptos RwSignals for UI reactivity
8. **Dependency Injection** - Arc<Session> pattern for shared resources

### Key Design Decisions

1. **ScyllaDB for Offline Cache** - Distributed DB resilient to API outages
2. **Job Queue + Worker** - Decouples HTTP responses from long-running tasks
3. **WebSocket Broadcasting** - Real-time updates without polling
4. **Trait-Based Repositories** - Testable, swappable implementations
5. **Server Functions** - Type-safe client-server communication
6. **Background Schedulers** - Autonomous sync on fixed intervals

---

## Data Flow Summary

```
External APIs (Unetwork, Unity, uno-app)
    ↓
API Clients (uno_client, content_client, etc.)
    ↓
Schedulers / Server Functions / Job Queue
    ↓
Logic Services (business logic)
    ↓
Repositories (ScyllaDB abstraction)
    ↓
ScyllaDB (persistence)
    ↓
UI Components (reactive display)
    ↓
WebSocket (real-time updates)
    ↓
Browser (Leptos WASM client)
```

---

## Critical Files for Understanding

| Area | Key Files |
|------|-----------|
| **Sync Architecture** | `main.rs` → `logic/rewards_sync_service.rs` → `repository/` |
| **UI Rendering** | `app.rs` → `ui/pages/` → `ui/components/` |
| **Real-time Updates** | `ui/hooks/use_job_websocket.rs` → `ws/broadcaster.rs` |
| **Authentication** | `api/wallet_auth.rs` → `handler/user_handler.rs` |
| **Database** | `db/connection.rs` → `db/migrations/` → `repository/scylla/` |
