# UNO Ecosystem Integration

This document explains how `uno-app`, `uno-api`, and `uno-admin` work together to form the complete UNO license management platform.

---

## Relationship Summary

| Project | Type | Role |
|---------|------|------|
| **uno-api** | Library crate | Provides traits, models, services, auth, HTTP clients |
| **uno-app** | Binary application | Implements traits, hosts HTTP server, connects to PostgreSQL |
| **uno-admin** | Binary application | Admin dashboard, syncs data, manages content via uno-app |

**uno-api** is a **reusable library** that can be embedded in multiple applications.

**uno-app** is the **production server** that implements uno-api's interfaces and serves the web application.

**uno-admin** is the **admin dashboard** that uses uno-api's HTTP clients to communicate with uno-app.

---

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    External Admin Apps                          │
│              (unity-dashboard, CLI tools, etc.)                 │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  uno-api::client::UnoApiClient                          │   │
│  │  - HMAC-signed HTTP requests                            │   │
│  │  - publish_licenses(), search_licenses(), etc.          │   │
│  └─────────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────────┘
                             │ HTTP (HMAC-signed)
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                         uno-app                                 │
│                    (Host Application)                           │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  HTTP Handlers (src/server/handlers/)                   │   │
│  │  - licenses_handler.rs                                  │   │
│  │  - Uses: uno-api::auth::ClientRegistry                  │   │
│  │  - Uses: uno-api::auth::verify_request()                │   │
│  └──────────────────────────┬──────────────────────────────┘   │
│                             │                                   │
│  ┌──────────────────────────▼──────────────────────────────┐   │
│  │  uno-api::services::LicenseAdminService                 │   │
│  │  - publish_licenses(), claim_license(), etc.            │   │
│  │  - Injected with: Arc<dyn LicenseRepository>            │   │
│  └──────────────────────────┬──────────────────────────────┘   │
│                             │                                   │
│  ┌──────────────────────────▼──────────────────────────────┐   │
│  │  uno-app::server::repositories::LicenseRepository       │   │
│  │  - IMPLEMENTS: uno-api::traits::LicenseRepository       │   │
│  │  - PostgreSQL queries via sqlx                          │   │
│  └──────────────────────────┬──────────────────────────────┘   │
│                             │                                   │
│                             ▼                                   │
│                        PostgreSQL                               │
└─────────────────────────────────────────────────────────────────┘
                             │
                             │ HTTP (JWT Bearer)
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Unetwork License API                         │
│               (https://api.unityedge.io)                        │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  uno-api::client::UnetworkClient                        │   │
│  │  - JWT Bearer authentication                            │   │
│  │  - get_all_license_ids(), get_rewards(), etc.           │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## What uno-api Provides

### 1. Trait Definitions (Interfaces)

```rust
// uno-api/src/traits/license_repository.rs
#[async_trait]
pub trait LicenseRepository: Send + Sync {
    async fn insert(&self, license: License) -> Result<License, DbError>;
    async fn insert_batch(&self, licenses: Vec<License>) -> Result<Vec<License>, DbError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<License>, DbError>;
    async fn get_by_lease_code(&self, code: &str) -> Result<Option<License>, DbError>;
    async fn claim(&self, id: Uuid, device_id: Option<String>) -> Result<License, DbError>;
    async fn count_unclaimed(&self) -> Result<i64, DbError>;
    async fn get_summary(&self) -> Result<LicenseSummary, DbError>;
    // ... more methods
}
```

### 2. Data Models

```rust
// uno-api/src/models/license.rs
pub enum SplitType { Split5050, Split5545, Split6040 }
pub struct License { id, lease_code, valid_from, valid_to, split_type, claimed, ... }
pub struct LicenseDto { ... }  // API response format
pub struct ClaimRequest { lease_code, device_id }
pub struct ClaimResponse { success, license, error }
pub struct LicenseSummary { total, claimed, available, expired, by_split_type }
```

### 3. Business Logic Services

```rust
// uno-api/src/services/license_admin.rs
pub struct LicenseAdminService {
    repository: Arc<dyn LicenseRepository>,
}

impl LicenseAdminService {
    pub fn new(repository: Arc<dyn LicenseRepository>) -> Self;
    pub async fn publish_licenses(&self, req: PublishLicensesRequest) -> Result<PublishResult, ApiError>;
    pub async fn claim_license(&self, req: ClaimRequest) -> Result<ClaimResponse, ApiError>;
    pub async fn search_licenses(&self, filters: LicenseFilters, pagination: PaginationParams) -> Result<Paginated<LicenseDto>, ApiError>;
    // ... more methods
}
```

### 4. Authentication

```rust
// uno-api/src/auth/
pub struct ClientRegistry { ... }      // Thread-safe credential store
pub struct SignedRequest<T> { ... }    // HMAC-signed request wrapper
pub fn verify_request<T>(...) -> Result<(), AuthError>;  // Signature verification
pub fn generate_preview_token(...) -> String;            // Preview tokens
```

### 5. HTTP Clients (for external consumers)

```rust
// uno-api/src/client/uno_app_client.rs
pub struct UnoApiClient { config, http }

impl UnoApiClient {
    pub async fn publish_licenses(&self, req) -> Result<PublishResult, ApiError>;
    pub async fn search_licenses(&self, filters, pagination) -> Result<Paginated<LicenseDto>, ApiError>;
    pub async fn get_claimed_licenses(&self, req) -> Result<Vec<ClaimedLicenseDto>, ApiError>;
    // ... all admin operations
}
```

### 6. Error Types

```rust
// uno-api/src/error.rs
pub enum DbError { NotFound, DuplicateEntry, ConnectionError, QueryError, ... }
pub enum AuthError { InvalidSignature, TimestampExpired, UnknownClient, ... }
pub enum ApiError { Database(DbError), Auth(AuthError), Validation, NotFound, ... }
```

---

## What uno-app Implements

### 1. Repository Implementations

```rust
// uno-app/src/server/repositories/license_repository.rs
use uno_api::traits::LicenseRepository;
use uno_api::models::License;
use uno_api::error::DbError;

pub struct PostgresLicenseRepository {
    pool: PgPool,
}

#[async_trait]
impl LicenseRepository for PostgresLicenseRepository {
    async fn insert(&self, license: License) -> Result<License, DbError> {
        sqlx::query_as!(
            License,
            r#"INSERT INTO licenses (id, lease_code, valid_from, valid_to, split_type, claimed)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING *"#,
            license.id,
            license.lease_code,
            license.valid_from,
            license.valid_to,
            license.split_type.to_db_str(),
            license.claimed
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }

    async fn claim(&self, id: Uuid, device_id: Option<String>) -> Result<License, DbError> {
        sqlx::query_as!(
            License,
            r#"UPDATE licenses
               SET claimed = true, device_id = $2, claimed_at = NOW()
               WHERE id = $1
               RETURNING *"#,
            id,
            device_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))
    }

    // ... implement all trait methods
}
```

### 2. Service Factory (Dependency Injection)

```rust
// uno-app/src/server/app/service_factory.rs
use uno_api::services::LicenseAdminService;
use uno_api::auth::ClientRegistry;

pub struct ServiceFactory {
    pub license_service: Arc<LicenseAdminService>,
    pub client_registry: Arc<ClientRegistry>,
    // ... other services
}

impl ServiceFactory {
    pub fn new(pool: PgPool) -> Self {
        // Create repository implementation
        let license_repo = Arc::new(PostgresLicenseRepository::new(pool.clone()));

        // Inject into uno-api service
        let license_service = Arc::new(LicenseAdminService::new(license_repo));

        // Create client registry for auth
        let client_registry = Arc::new(ClientRegistry::new());
        client_registry.register("unity-dashboard", secret_key).unwrap();

        Self {
            license_service,
            client_registry,
            // ...
        }
    }
}
```

### 3. HTTP Handlers

```rust
// uno-app/src/server/handlers/licenses_handler.rs
use uno_api::auth::{SignedRequest, verify_request};
use uno_api::models::{PublishLicensesRequest, PublishResult};

pub async fn publish_licenses(
    req: web::Json<SignedRequest<PublishLicensesRequest>>,
    factory: web::Data<ServiceFactory>,
) -> Result<HttpResponse, AppError> {
    // 1. Verify HMAC signature using uno-api
    verify_request(&req, &factory.client_registry, 300)
        .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    // 2. Execute business logic using uno-api service
    let result = factory.license_service
        .publish_licenses(req.payload.clone())
        .await
        .map_err(|e| AppError::from(e))?;

    // 3. Return response
    Ok(HttpResponse::Ok().json(result))
}
```

### 4. Route Configuration

```rust
// uno-app/src/server/handlers/mod.rs
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/admin")
            // All admin routes use HMAC auth from uno-api
            .route("/licenses", web::post().to(publish_licenses))
            .route("/licenses/import", web::post().to(import_csv))
            .route("/licenses/search", web::post().to(search_licenses))
            .route("/licenses/claimed", web::post().to(get_claimed_licenses))
            .route("/licenses/{id}", web::get().to(get_license))
            .route("/licenses/{id}", web::delete().to(revoke_license))
            .route("/variants", web::get().to(list_variants))
            .route("/health", web::get().to(health_check))
    );
}
```

---

## Integration Points Summary

| Layer | uno-api Provides | uno-app Implements/Uses |
|-------|------------------|-------------------------|
| **Traits** | `LicenseRepository` | `PostgresLicenseRepository` |
| **Services** | `LicenseAdminService` | Injects repo via DI |
| **Auth** | `ClientRegistry`, `verify_request()` | Registers clients, validates in handlers |
| **Models** | `License`, `ClaimRequest`, `SplitType`, etc. | Uses directly throughout |
| **Errors** | `DbError`, `ApiError`, `AuthError` | Maps to `AppError` → HTTP status |
| **Clients** | `UnoApiClient` | N/A (used by external apps) |

---

## Data Flow Examples

### License Publishing (Admin Operation)

```
1. unity-dashboard (external app)
   │
   │  // Uses uno-api client library
   │  let client = UnoApiClient::new(config);
   │  let result = client.publish_licenses(req).await?;
   │
   │  // Request is HMAC-signed automatically
   │  SignedRequest {
   │    client_id: "unity-dashboard",
   │    timestamp: 1695484800,
   │    nonce: "a1b2c3d4...",
   │    signature: "hmac_sha256(...)",
   │    payload: PublishLicensesRequest { licenses: [...] }
   │  }
   │
   ▼
2. uno-app HTTP Handler
   │
   │  POST /api/v1/admin/licenses
   │
   │  // Verify signature using uno-api
   │  uno_api::auth::verify_request(&req, &registry, 300)?;
   │
   ▼
3. uno-api::services::LicenseAdminService
   │
   │  service.publish_licenses(req.payload)
   │  - Validates date ranges (valid_from < valid_to)
   │  - Validates split types
   │  - Generates lease codes if not provided
   │  - Checks for duplicates
   │  - Calls repository.insert_batch()
   │
   ▼
4. uno-app::repositories::PostgresLicenseRepository
   │
   │  // Implements uno_api::traits::LicenseRepository
   │  async fn insert_batch(&self, licenses) {
   │      sqlx::query!("INSERT INTO licenses ...").execute(&self.pool)
   │  }
   │
   ▼
5. PostgreSQL
   │
   │  INSERT INTO licenses (id, lease_code, ...) VALUES ...
   │
   ▼
6. Response flows back
   │
   │  PublishResult { created: 10, failed: 0, errors: [] }
```

### License Claiming (Public Operation)

```
1. uno-app Frontend (Leptos)
   │
   │  // Server function call
   │  claim_by_split_type(SplitType::Split5050, device_id).await
   │
   ▼
2. uno-app Server Function
   │
   │  // Extract service from request context
   │  let factory = extract::<ServiceFactory>(&req)?;
   │
   ▼
3. uno-api::services::LicenseAdminService
   │
   │  service.claim_license(ClaimRequest { lease_code, device_id })
   │  - Validates license exists
   │  - Checks not already claimed
   │  - Checks within validity window
   │  - Calls repository.claim()
   │
   ▼
4. uno-app::repositories::PostgresLicenseRepository
   │
   │  async fn claim(&self, id, device_id) {
   │      sqlx::query!("UPDATE licenses SET claimed = true, ...")
   │  }
   │
   ▼
5. Response
   │
   │  ClaimResponse {
   │    success: true,
   │    license: Some(LicenseDto { lease_code: "ABCD-EFGH-IJKL", ... }),
   │    error: None
   │  }
```

---

## Authentication Flow

### HMAC Request Signing (Admin Endpoints)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Admin Client                                 │
│                                                                 │
│  // uno-api handles signing automatically                       │
│  let signed = SignedRequest::sign(client_id, secret_key, payload)?;  │
│                                                                 │
│  // Signature computation:                                      │
│  // HMAC-SHA256(secret, "client_id:timestamp:nonce:json(payload)")  │
│                                                                 │
│  SignedRequest {                                                │
│    client_id: "unity-dashboard",                                │
│    timestamp: 1695484800,        // Unix seconds                │
│    nonce: "a1b2c3d4e5f6...",     // 16 random bytes, hex       │
│    signature: "9f86d08...",       // HMAC-SHA256, hex           │
│    payload: { ... }                                             │
│  }                                                              │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    uno-app Handler                              │
│                                                                 │
│  // Registry was populated at startup                           │
│  let registry: &ClientRegistry = &factory.client_registry;     │
│                                                                 │
│  // Verify using uno-api function                               │
│  uno_api::auth::verify_request(&req, registry, 300)?;          │
│                                                                 │
│  // Verification steps:                                         │
│  // 1. Lookup client_id in registry                             │
│  // 2. Check client is active                                   │
│  // 3. Verify timestamp within 300s window (replay protection)  │
│  // 4. Recompute HMAC with stored secret                        │
│  // 5. Constant-time comparison of signatures                   │
│                                                                 │
│  // If valid, proceed with request                              │
│  let result = service.publish_licenses(req.payload).await?;    │
└─────────────────────────────────────────────────────────────────┘
```

### Client Registration (Startup)

```rust
// uno-app/src/main.rs or service_factory.rs
use uno_api::auth::ClientRegistry;

// Create registry
let registry = ClientRegistry::new();

// Register known clients
registry.register("unity-dashboard", b"secret_key_1")?;
registry.register("cli-tool", b"secret_key_2")?;
registry.register_credentials(ClientCredentials {
    client_id: "admin-api".to_string(),
    secret_key: env::var("ADMIN_SECRET")?.into_bytes(),
    roles: vec!["admin".to_string(), "publisher".to_string()],
    active: true,
})?;

// Store in app state
let factory = ServiceFactory {
    client_registry: Arc::new(registry),
    // ...
};
```

---

## Dependency Configuration

### uno-app Cargo.toml

```toml
[dependencies]
# Local path during development
uno-api = { path = "../uno-api", features = ["full"] }

# Or from git
# uno-api = { git = "https://github.com/org/uno-api", features = ["full"] }
```

### Feature Flags

| Feature | What it enables | When to use |
|---------|-----------------|-------------|
| `services` | CSV import, LicenseAdminService | Always (default) |
| `auth` | HMAC signing, ClientRegistry, preview tokens | Admin endpoints |
| `client` | UnoApiClient, UnetworkClient | External apps calling uno-app |
| `full` | All of the above | uno-app uses this |

---

## Shared Types Usage

### In uno-app handlers

```rust
use uno_api::models::{
    License,
    LicenseDto,
    LicenseInput,
    ClaimRequest,
    ClaimResponse,
    SplitType,
    LicenseStatus,
    LicenseFilters,
    PublishLicensesRequest,
    PublishResult,
    Paginated,
    PaginationParams,
};

use uno_api::traits::LicenseRepository;
use uno_api::services::LicenseAdminService;
use uno_api::auth::{ClientRegistry, SignedRequest, verify_request};
use uno_api::error::{ApiError, DbError, AuthError};
```

### In uno-app repositories

```rust
use uno_api::traits::LicenseRepository;
use uno_api::models::{License, LicenseSummary, LicenseFilters};
use uno_api::error::DbError;

#[async_trait]
impl LicenseRepository for PostgresLicenseRepository {
    // All return types are from uno-api
    async fn get_summary(&self) -> Result<LicenseSummary, DbError> {
        // Implementation using sqlx
    }
}
```

---

## Error Mapping

### uno-api errors to HTTP responses

```rust
// uno-app/src/types/error.rs
use uno_api::error::{ApiError, DbError, AuthError};

impl From<ApiError> for AppError {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Database(DbError::NotFound(msg)) => AppError::NotFound(msg),
            ApiError::Database(DbError::DuplicateEntry(msg)) => AppError::Conflict(msg),
            ApiError::Database(e) => AppError::InternalError(e.to_string()),
            ApiError::Auth(AuthError::InvalidSignature) => AppError::Unauthorized("Invalid signature".into()),
            ApiError::Auth(AuthError::TimestampExpired) => AppError::Unauthorized("Request expired".into()),
            ApiError::Auth(AuthError::UnknownClient(id)) => AppError::Unauthorized(format!("Unknown client: {}", id)),
            ApiError::Validation(msg) => AppError::BadRequest(msg),
            ApiError::NotFound(msg) => AppError::NotFound(msg),
            ApiError::Conflict(msg) => AppError::Conflict(msg),
            _ => AppError::InternalError(err.to_string()),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
```

---

## Testing Integration

### Unit Testing with Mocks

```rust
// uno-app/tests/license_service_test.rs
use uno_api::traits::LicenseRepository;
use uno_api::services::LicenseAdminService;
use uno_api::models::*;
use uno_api::error::DbError;

struct MockLicenseRepository {
    licenses: RwLock<Vec<License>>,
}

#[async_trait]
impl LicenseRepository for MockLicenseRepository {
    async fn insert(&self, license: License) -> Result<License, DbError> {
        self.licenses.write().unwrap().push(license.clone());
        Ok(license)
    }
    // ... mock other methods
}

#[tokio::test]
async fn test_publish_licenses() {
    let mock_repo = Arc::new(MockLicenseRepository::new());
    let service = LicenseAdminService::new(mock_repo);

    let req = PublishLicensesRequest {
        licenses: vec![LicenseInput::new(...)],
        idempotency_key: None,
    };

    let result = service.publish_licenses(req).await.unwrap();
    assert_eq!(result.created, 1);
    assert_eq!(result.failed, 0);
}
```

### Integration Testing

```rust
// uno-app/tests/integration/licenses_test.rs
use uno_api::client::UnoApiClient;
use uno_api::config::ClientConfig;

#[tokio::test]
async fn test_publish_and_claim_flow() {
    // Start test server
    let server = TestServer::spawn().await;

    // Create client pointing to test server
    let config = ClientConfig::new(
        &server.url(),
        "test-client",
        "test-secret",
    );
    let client = UnoApiClient::new(config);

    // Publish licenses
    let publish_result = client.publish_licenses(req).await.unwrap();
    assert_eq!(publish_result.created, 5);

    // Search for available
    let licenses = client.search_licenses(
        LicenseFilters::new().unclaimed_only(),
        PaginationParams::default(),
    ).await.unwrap();
    assert_eq!(licenses.items.len(), 5);
}
```

---

## Summary

| Aspect | uno-api | uno-app |
|--------|---------|---------|
| **Type** | Library crate | Binary application |
| **Purpose** | Reusable business logic | Production server |
| **Database** | Defines traits | Implements with PostgreSQL |
| **Auth** | Provides mechanisms | Configures and uses |
| **HTTP** | Provides clients | Serves endpoints |
| **Models** | Defines all types | Uses directly |
| **Services** | Implements logic | Injects dependencies |
| **Testing** | Mockable traits | Integration tests |

The separation allows:
1. **Reusability** - uno-api can be used in multiple applications
2. **Testability** - Traits can be mocked for unit tests
3. **Flexibility** - Different implementations (PostgreSQL, SQLite, etc.)
4. **Security** - Auth logic is centralized and tested
5. **Consistency** - Shared types prevent drift between client/server

---

# UNO-ADMIN Integration

This section explains how `uno-admin` (admin dashboard) integrates with `uno-app` and `uno-api`.

---

## Full Ecosystem Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        uno-admin                                │
│                   (Admin Dashboard)                             │
│                   ScyllaDB for caching                          │
│                                                                 │
│  ┌───────────────────┐  ┌───────────────────┐                  │
│  │  UnoLicenseClient │  │   ContentClient   │                  │
│  │  (uses uno-api)   │  │  (uses uno-api)   │                  │
│  └─────────┬─────────┘  └─────────┬─────────┘                  │
│            │                      │                             │
│            │ HMAC-signed          │ HMAC-signed                │
│            │                      │                             │
└────────────┼──────────────────────┼─────────────────────────────┘
             │                      │
             ▼                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                         uno-app                                 │
│                    (Host Server)                                │
│                    PostgreSQL                                   │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Admin API Endpoints (/api/v1/admin/*)                    │ │
│  │  - Verifies HMAC using uno-api::ClientRegistry            │ │
│  │  - Uses uno-api::LicenseAdminService                      │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Public API Endpoints (/api/v1/*)                         │ │
│  │  - License claiming, FAQ, content                         │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  PostgreSQL (licenses, content, referrals, etc.)          │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
             │
             │ Implements traits from
             ▼
┌─────────────────────────────────────────────────────────────────┐
│                         uno-api                                 │
│                    (Shared Library)                             │
│                                                                 │
│  Provides to uno-admin:                                         │
│  - UnoApiClient (HTTP client with HMAC signing)                │
│  - Models (License, ClaimRequest, SplitType, etc.)             │
│                                                                 │
│  Provides to uno-app:                                           │
│  - ClientRegistry (credential verification)                    │
│  - LicenseRepository trait                                     │
│  - LicenseAdminService (business logic)                        │
│  - Authentication (SignedRequest, verify_request)              │
└─────────────────────────────────────────────────────────────────┘
             │
             │ Also connects to
             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Unetwork License API                         │
│               (https://api.unityedge.io)                        │
│                                                                 │
│  uno-admin syncs:                                               │
│  - Licenses (via UnetworkClient)                               │
│  - Rewards (via Unity API)                                     │
│  - Stores in local ScyllaDB cache                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## uno-admin Integration Points

### 1. uno-admin → uno-api (Library Dependency)

uno-admin imports uno-api to use its HTTP clients:

```toml
# uno-admin/Cargo.toml
[dependencies]
uno-api = { path = "../uno-api", features = ["client"] }
```

**What uno-admin uses from uno-api:**

| Component | Purpose |
|-----------|---------|
| `UnoApiClient` | HMAC-authenticated HTTP client |
| `ClientConfig` | API configuration |
| `LicenseDto`, `ClaimRequest` | Shared data models |
| `SignedRequest` | Request signing |
| `UnetworkClient` | Unetwork API client |

### 2. uno-admin → uno-app (HTTP API Calls)

uno-admin calls uno-app's admin endpoints using uno-api's client:

```rust
// uno-admin/src/api/uno_client.rs
use uno_api::client::UnoApiClient;
use uno_api::config::ClientConfig;

pub struct UnoLicenseClient {
    client: UnoApiClient,
}

impl UnoLicenseClient {
    pub fn from_env() -> Result<Self, Error> {
        let config = ClientConfig::new(
            &env::var("UNO_API_URL")?,      // http://localhost:3000
            &env::var("ADMIN_CLIENT_ID")?,   // "uno-admin"
            &env::var("ADMIN_SECRET_KEY")?,  // shared secret
        );
        Ok(Self { client: UnoApiClient::new(config) })
    }

    // Calls uno-app: POST /api/v1/admin/licenses
    pub async fn publish_licenses(&self, req: PublishLicensesRequest)
        -> Result<PublishResult, Error>
    {
        self.client.publish_licenses(req).await
    }

    // Calls uno-app: POST /api/v1/admin/licenses/claimed
    pub async fn get_claimed_licenses(&self, req: GetClaimedLicensesRequest)
        -> Result<Vec<ClaimedLicenseDto>, Error>
    {
        self.client.get_claimed_licenses(req).await
    }

    // Calls uno-app: POST /api/v1/admin/referrals/sync
    pub async fn sync_referrals(&self, req: SyncReferralsRequest)
        -> Result<i32, Error>
    {
        self.client.sync_referrals(req).await
    }
}
```

### 3. uno-admin Content Client

uno-admin manages CMS content in uno-app:

```rust
// uno-admin/src/api/content_client.rs
pub struct ContentClient {
    base_url: String,
    client_id: String,
    secret_key: Vec<u8>,
    http: reqwest::Client,
}

impl ContentClient {
    // Calls uno-app: GET /api/v1/admin/items/{schema_id}
    pub async fn list_content(&self, schema_id: &str)
        -> Result<Vec<ContentItem>, Error>;

    // Calls uno-app: POST /api/v1/admin/items/{schema_id}
    pub async fn create_content(&self, req: UpsertContentRequest)
        -> Result<ContentItem, Error>;

    // Calls uno-app: PUT /api/v1/admin/items/{schema_id}/{id}
    pub async fn update_content(&self, id: &str, req: UpsertContentRequest)
        -> Result<ContentItem, Error>;

    // Calls uno-app: POST /api/v1/admin/items/{schema_id}/{id}/publish
    pub async fn publish_content(&self, schema_id: &str, id: &str)
        -> Result<ContentItem, Error>;
}
```

---

## Data Flow: uno-admin Operations

### Publishing Licenses via uno-admin

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. uno-admin UI: Admin clicks "Publish Licenses"                │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. uno-admin: Server function calls UnoLicenseClient            │
│                                                                 │
│    let client = UnoLicenseClient::from_env()?;                  │
│    let result = client.publish_licenses(req).await?;            │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. uno-api: UnoApiClient signs request with HMAC                │
│                                                                 │
│    SignedRequest {                                              │
│      client_id: "uno-admin",                                    │
│      timestamp: 1695484800,                                     │
│      nonce: "a1b2c3d4...",                                      │
│      signature: "hmac_sha256(...)",                             │
│      payload: PublishLicensesRequest { licenses: [...] }        │
│    }                                                            │
└─────────────────────────────┬───────────────────────────────────┘
                              │ HTTP POST /api/v1/admin/licenses
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. uno-app: Handler receives request                            │
│                                                                 │
│    // Verify signature using uno-api                            │
│    uno_api::auth::verify_request(&req, &registry, 300)?;        │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 5. uno-app: Uses uno-api service with injected repository       │
│                                                                 │
│    let service = LicenseAdminService::new(postgres_repo);       │
│    let result = service.publish_licenses(req.payload).await?;   │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 6. uno-app: PostgresLicenseRepository stores in database        │
│                                                                 │
│    sqlx::query!("INSERT INTO licenses ...").execute(&pool)     │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 7. Response: PublishResult { created: 10, failed: 0 }           │
└─────────────────────────────────────────────────────────────────┘
```

### Syncing Claimed Licenses (Background Job)

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. uno-admin: Background scheduler (every 5 min)                │
│                                                                 │
│    MarketplaceService::sync_claimed_licenses().await            │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. uno-admin: Calls uno-app via UnoApiClient                    │
│                                                                 │
│    client.get_claimed_licenses(GetClaimedLicensesRequest {      │
│        since: last_sync_time,                                   │
│        limit: 100,                                              │
│    }).await?                                                    │
└─────────────────────────────┬───────────────────────────────────┘
                              │ HMAC-signed HTTP
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. uno-app: Returns claimed licenses from PostgreSQL            │
│                                                                 │
│    Vec<ClaimedLicenseDto> {                                     │
│        license_id, lease_code, claim_token, claimed_at, ...     │
│    }                                                            │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. uno-admin: Stores in ScyllaDB for local caching              │
│                                                                 │
│    license_repo.upsert_batch(&claimed_licenses).await?;         │
└─────────────────────────────────────────────────────────────────┘
```

### Managing CMS Content

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. uno-admin UI: Admin edits FAQ content                        │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. uno-admin: ContentClient sends HMAC-signed request           │
│                                                                 │
│    content_client.update_content("faq-123", UpsertContentRequest {  │
│        schema_id: "faq",                                        │
│        data: { question: "...", answer: "..." },                │
│        translations: { es: {...}, fr: {...} },                  │
│    }).await?                                                    │
└─────────────────────────────┬───────────────────────────────────┘
                              │ HMAC-signed HTTP
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. uno-app: CMS endpoints handle content CRUD                   │
│                                                                 │
│    PUT /api/v1/admin/items/faq/faq-123                          │
│    - Validates against schema                                   │
│    - Stores in content_items table                              │
│    - Creates version history                                    │
│    - Returns updated ContentItem                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Background Sync Services in uno-admin

uno-admin runs scheduled background jobs to keep data synchronized:

| Service | Interval | Source | Destination | Purpose |
|---------|----------|--------|-------------|---------|
| **LicenseSyncService** | 10 min | Unetwork API | ScyllaDB | Sync all licenses |
| **RewardsSyncService** | 1 min | Unity API | ScyllaDB | Sync reward allocations |
| **MarketplaceService** | 5 min | uno-app | ScyllaDB | Sync claimed licenses |
| **ReferralSyncService** | 5 min | uno-app ↔ ScyllaDB | Bi-directional referral sync |

### Sync Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        uno-admin                                │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Background Schedulers (main.rs)                        │   │
│  │  - Runs on fixed intervals                              │   │
│  │  - Creates pending SyncJob                              │   │
│  │  - Enqueues to job queue                                │   │
│  └─────────────────────────┬───────────────────────────────┘   │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Job Queue (MPSC channel)                               │   │
│  │  - Decouples HTTP from long-running tasks               │   │
│  └─────────────────────────┬───────────────────────────────┘   │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Job Worker                                             │   │
│  │  - Processes jobs from queue                            │   │
│  │  - Calls appropriate sync service                       │   │
│  │  - Updates job status                                   │   │
│  │  - Broadcasts via WebSocket                             │   │
│  └─────────────────────────┬───────────────────────────────┘   │
│                            │                                    │
│            ┌───────────────┼───────────────┐                   │
│            ▼               ▼               ▼                   │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐           │
│  │ Unetwork API │ │  Unity API   │ │   uno-app    │           │
│  │ (licenses)   │ │  (rewards)   │ │  (claimed)   │           │
│  └──────────────┘ └──────────────┘ └──────────────┘           │
│            │               │               │                   │
│            └───────────────┼───────────────┘                   │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  ScyllaDB (local cache)                                 │   │
│  │  - licenses, rewards, agents, nodes tables              │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Authentication Between Systems

### Client Registration

uno-app registers uno-admin as an authorized client at startup:

```rust
// uno-app/src/server/app/service_factory.rs
let registry = ClientRegistry::new();

// Register uno-admin client
registry.register("uno-admin", b"shared_secret_key")?;

// Register other clients
registry.register("unity-dashboard", b"another_secret")?;
registry.register("cli-tool", b"cli_secret")?;
```

### uno-admin Configuration

```bash
# uno-admin/.env
UNO_API_URL=http://localhost:3000
ADMIN_CLIENT_ID=uno-admin
ADMIN_SECRET_KEY=shared_secret_key

# Must match what uno-app has registered
```

### Request Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        uno-admin                                │
│                                                                 │
│  Environment:                                                   │
│    ADMIN_CLIENT_ID=uno-admin                                    │
│    ADMIN_SECRET_KEY=shared_secret_key                           │
│                                                                 │
│  UnoApiClient creates SignedRequest:                            │
│    signature = HMAC-SHA256(                                     │
│      secret_key,                                                │
│      "uno-admin:timestamp:nonce:json(payload)"                  │
│    )                                                            │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         uno-app                                 │
│                                                                 │
│  Startup (registers clients):                                   │
│    registry.register("uno-admin", b"shared_secret_key")?;       │
│                                                                 │
│  Handler (verifies request):                                    │
│    uno_api::auth::verify_request(&req, &registry, 300)?;        │
│    // Checks: signature, timestamp freshness, client exists     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Shared Components via uno-api

| Component | uno-admin Uses | uno-app Uses |
|-----------|----------------|--------------|
| `UnoApiClient` | HTTP client to call uno-app | N/A |
| `ClientRegistry` | N/A | Verify incoming requests |
| `SignedRequest<T>` | Wrapped in client calls | Unwrapped in handlers |
| `verify_request()` | N/A | Validate signatures |
| `LicenseRepository` trait | N/A | Implements for PostgreSQL |
| `LicenseAdminService` | N/A | Uses with injected repo |
| `License`, `LicenseDto` | Receives from API | Stores/retrieves |
| `ClaimRequest/Response` | Sends/receives | Processes |
| `SplitType` | Displays in UI | Stores in DB |
| `DbError`, `ApiError` | Handles errors | Returns errors |

---

## Full Ecosystem Integration Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                      EXTERNAL USERS                             │
│                   (Browser, Mobile App)                         │
└─────────────────────────────┬───────────────────────────────────┘
                              │ Public endpoints
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         uno-app                                 │
│                                                                 │
│  Frontend: Leptos SSR/WASM (license claiming, FAQ, etc.)       │
│  Backend: Actix-web (public + admin APIs)                      │
│  Database: PostgreSQL (source of truth)                        │
│                                                                 │
│  Implements: uno-api traits (LicenseRepository)                │
│  Uses: uno-api services (LicenseAdminService)                  │
│  Verifies: uno-api auth (ClientRegistry)                       │
└─────────────────────────────┬───────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │ Admin API                     │ Traits/Services
              │ (HMAC-signed)                 │
              ▼                               ▼
┌──────────────────────────┐    ┌──────────────────────────────────┐
│       uno-admin          │    │           uno-api                │
│                          │    │                                  │
│  Dashboard UI (Leptos)   │    │  - UnoApiClient (HTTP client)   │
│  Background sync jobs    │◄───│  - ClientRegistry (auth)        │
│  ScyllaDB (local cache)  │    │  - LicenseRepository (trait)    │
│  WebSocket job updates   │    │  - LicenseAdminService          │
│                          │    │  - Models (shared types)        │
│  Uses: uno-api clients   │    │  - Error types                  │
└──────────────────────────┘    └──────────────────────────────────┘
              │
              │ Also syncs from
              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    External APIs                                │
│                                                                 │
│  Unetwork API: Licenses, nodes, leases                         │
│  Unity API: Rewards, allocations                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Environment Configuration Summary

### uno-app

```bash
# Database
DATABASE_URL=postgres://user:pass@localhost/uno

# Admin client registration (in code or config)
# Registers: uno-admin, unity-dashboard, cli-tool
```

### uno-admin

```bash
# Connect to uno-app
UNO_API_URL=http://localhost:3000
ADMIN_CLIENT_ID=uno-admin
ADMIN_SECRET_KEY=shared_secret_key

# Connect to Unetwork API
UNITY_API_URL=https://api.unityedge.io
UNITY_JWT_TOKEN=<jwt_token>

# Local cache
SCYLLA_DB_HOST=localhost
SCYLLA_DB_KEYSPACE=unity_dashboard
```

### uno-api (library)

```toml
# Used by uno-app
[dependencies]
uno-api = { path = "../uno-api", features = ["full"] }

# Used by uno-admin
[dependencies]
uno-api = { path = "../uno-api", features = ["client"] }
```

---

## Integration Summary Table

| From | To | Method | Purpose |
|------|----|--------|---------|
| uno-admin | uno-api | Cargo dependency | Use `UnoApiClient`, models |
| uno-admin | uno-app | HTTP (HMAC-signed) | Admin operations, CMS |
| uno-admin | Unetwork API | HTTP (JWT Bearer) | Sync licenses |
| uno-admin | Unity API | HTTP (JWT Bearer) | Sync rewards |
| uno-app | uno-api | Cargo dependency | Implement traits, use services |
| uno-app | PostgreSQL | sqlx | Store data |
| Browser | uno-app | HTTP/WebSocket | User-facing app |

---

## Benefits of This Architecture

1. **Separation of Concerns**
   - uno-api: Reusable business logic and types
   - uno-app: Production server with PostgreSQL
   - uno-admin: Admin dashboard with local caching

2. **Offline Resilience**
   - uno-admin caches data in ScyllaDB
   - Can display data even if uno-app is temporarily unavailable

3. **Security**
   - All admin operations require HMAC-signed requests
   - Centralized auth logic in uno-api
   - Client registration controls access

4. **Scalability**
   - Background jobs don't block HTTP requests
   - WebSocket for real-time updates
   - ScyllaDB for distributed caching

5. **Maintainability**
   - Shared types prevent drift
   - Trait-based abstractions for testing
   - Clear boundaries between systems
