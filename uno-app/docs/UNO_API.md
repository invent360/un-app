# UNO-API - Architecture Overview

A modular, trait-based Rust library providing reusable license management functionality for the UNO platform. Designed for dependency injection into host applications.

---

## Project Statistics

- **Language**: Rust 2021 Edition
- **License**: MIT
- **Architecture**: Modular, trait-based, dependency injection
- **Async Support**: Full async-await via tokio and async-trait

---

## Directory Structure

```
src/
├── lib.rs                 # Library root, module declarations
├── error.rs               # Error type definitions (DbError, AuthError, ApiError)
├── config/                # Configuration modules
│   ├── mod.rs             # Module exports
│   ├── uno_app_config.rs  # HMAC-based auth config for uno-app
│   └── unetwork_config.rs # JWT Bearer auth config for Unetwork API
├── models/                # Data types and DTOs
│   ├── mod.rs             # Module exports
│   ├── license.rs         # License, SplitType, ClaimRequest/Response
│   ├── variant.rs         # VariantDto, VariantInput
│   ├── marketplace.rs     # ClaimedLicenseDto, ReferralDto, VisitorStats
│   ├── import.rs          # PublishResult, CsvImportRequest, ImportError
│   ├── request/           # Unetwork request models
│   │   ├── mod.rs
│   │   └── unetwork.rs    # GetAllLicenseIdsRequest, GetRewardsRequest, etc.
│   └── response/          # Unetwork response models
│       ├── mod.rs
│       └── unetwork.rs    # UnetworkLicense, LicenseGroup, RewardAllocation
├── traits/                # Repository interfaces
│   ├── mod.rs             # Module exports
│   └── license_repository.rs  # Abstract database operations
├── auth/                  # Authentication & security
│   ├── mod.rs             # Module exports + sign_request/verify_request
│   ├── hmac.rs            # HMAC-SHA256 signing/verification
│   ├── signed_request.rs  # Request wrapper with signature
│   ├── client_registry.rs # Thread-safe client credential store
│   └── preview_token.rs   # Time-limited preview tokens
├── client/                # HTTP clients
│   ├── mod.rs             # Module exports
│   ├── uno_app_client.rs  # HMAC-authenticated client for uno-app
│   └── unetwork_client.rs # JWT-authenticated client for Unetwork API
└── services/              # Business logic
    ├── mod.rs             # Module exports
    ├── license_admin.rs   # License management orchestration
    ├── csv_import.rs      # CSV parsing and validation
    └── unetwork/          # Unetwork API wrappers
        ├── mod.rs
        ├── licenses_service.rs  # License service with convenience methods
        └── rewards_service.rs   # Rewards service with convenience methods
```

---

## Core Architecture

### System Integration Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Admin Applications                       │
│                  (unity-dashboard, etc.)                    │
│  Uses: UnoApiClient (HMAC-signed requests)                 │
└────────────────────────┬────────────────────────────────────┘
                         │ HMAC-signed HTTP requests
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                uno-app (Host Server)                        │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ HTTP Endpoints (routes/handlers)                     │  │
│  │ ┌────────────────────────────────────────────────┐   │  │
│  │ │ Request Auth: ClientRegistry + verify_request │   │  │
│  │ └────────────────────────────────────────────────┘   │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │ Injected repositories                   │
│                   ▼                                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Services (business logic)                            │  │
│  │ - LicenseAdminService                                │  │
│  │ - UnetworkLicenseService                             │  │
│  │ - UnetworkRewardsService                             │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │ Uses trait methods                      │
│                   ▼                                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Repository Implementations (PostgreSQL, etc.)        │  │
│  │ Implements: LicenseRepository trait                  │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │                                         │
│                   ▼                                         │
│              PostgreSQL Database                           │
└─────────────────────────────────────────────────────────────┘
                         │ HTTP requests (Bearer token)
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Unetwork License API                           │
│         (https://api.unityedge.io)                         │
│  Uses: UnetworkClient (JWT Bearer authentication)          │
│  Endpoints: RPC functions + Edge Functions                 │
└─────────────────────────────────────────────────────────────┘
```

### Request Flow Pattern

```
Handler (HTTP endpoint)
  ↓
ClientRegistry.verify_request() (HMAC validation)
  ↓
Service (business logic)
  ↓
Repository (trait implementation)
  ↓
Database / External API
```

---

## Error Handling

### Three-Layer Error Hierarchy

| Layer | Type | Purpose |
|-------|------|---------|
| Database | `DbError` | Database operation errors |
| Application | `ApiError` | High-level application errors |
| Authentication | `AuthError` | Auth/authorization errors |

### DbError Variants

```rust
pub enum DbError {
    NotFound(String),        // Record not found
    DuplicateEntry(String),  // Constraint violation
    ConnectionError(String), // Connection issues
    QueryError(String),      // Query execution error
    TransactionError(String),// Transaction failure
    Other(String),           // Generic database errors
}
```

### AuthError Variants

```rust
pub enum AuthError {
    InvalidSignature,        // HMAC signature mismatch
    TimestampExpired,        // Request too old (replay protection)
    UnknownClient(String),   // Client not registered
    MissingAuth,             // Missing auth header
    MalformedAuth(String),   // Invalid auth format
}
```

### ApiError Variants

```rust
pub enum ApiError {
    Database(DbError),       // Database operation failed
    Auth(AuthError),         // Authentication failed
    Validation(String),      // Input validation error
    CsvParse(String),        // CSV parsing error
    Http(String),            // HTTP client error
    Serialization(String),   // JSON serialization error
    NotFound(String),        // Resource not found
    Conflict(String),        // Resource conflict
    Internal(String),        // Internal server error
}
```

### HTTP Status Code Mapping

| HTTP Status | Error Type |
|-------------|------------|
| 401 | `AuthError::InvalidSignature` |
| 403 | `AuthError::MissingAuth` |
| 404 | `ApiError::NotFound` |
| 409 | `ApiError::Conflict` |
| 422 | `ApiError::Validation` |
| 5xx | `ApiError::Http` / `ApiError::Internal` |

---

## Configuration

### ClientConfig (HMAC Authentication)

For uno-app API client connections:

```rust
pub struct ClientConfig {
    pub base_url: String,           // Base URL of uno-app API
    pub client_id: String,          // Client identifier
    pub secret_key: String,         // HMAC secret
    pub timeout: Duration,          // Request timeout (default 30s)
    pub max_request_age_secs: u64,  // Replay protection window (default 300s)
    pub api_prefix: String,         // API version prefix (default /api/v1/admin)
}
```

**Builder Methods:**
- `with_timeout(duration)` - Set request timeout
- `with_max_request_age(secs)` - Set replay protection window
- `with_api_prefix(prefix)` - Set API prefix
- `url(endpoint)` - Build full endpoint URL

### UnetworkConfig (JWT Authentication)

For Unetwork API connections:

```rust
pub struct UnetworkConfig {
    pub base_url: String,    // Default: https://api.unityedge.io
    pub jwt_token: String,   // JWT Bearer token
    pub timeout: Duration,   // Request timeout (default 30s)
}
```

**URL Builders:**
- `rpc_url(function)` - PostgREST RPC endpoint (`/rest/v1/rpc/{function}`)
- `functions_url(function)` - Edge Functions endpoint (`/functions/v1/{function}`)

---

## Data Models

### License Domain (`models/license.rs`)

```rust
// Revenue split options
pub enum SplitType {
    Split5050,  // 50:50 user:operator
    Split5545,  // 55:45 user:operator
    Split6040,  // 60:40 user:operator
}

// License status filter
pub enum LicenseStatus {
    All,
    Available,
    Claimed,
    Expired,
    Revoked,
}

// Database entity
pub struct License {
    pub id: Uuid,
    pub lease_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub split_type: SplitType,
    pub claimed: bool,
    pub bound_to_device: bool,
    pub device_id: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// API response DTO
pub struct LicenseDto {
    pub id: String,
    pub lease_code: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub split_type: SplitType,
    pub claimed: bool,
    pub device_id: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub is_valid: bool,      // Computed
    pub is_expired: bool,    // Computed
}

// Claim request/response
pub struct ClaimRequest {
    pub lease_code: String,
    pub device_id: Option<String>,
}

pub struct ClaimResponse {
    pub success: bool,
    pub license: Option<LicenseDto>,
    pub error: Option<String>,
}

// Statistics
pub struct LicenseSummary {
    pub total: i64,
    pub claimed: i64,
    pub available: i64,
    pub expired: i64,
    pub by_split_type: HashMap<SplitType, i64>,
}
```

**Helper Functions:**
- `generate_lease_code()` - Creates `XXXX-XXXX-XXXX` code (no ambiguous chars: 0, O, I, L, 1)

### Variant Domain (`models/variant.rs`)

```rust
pub struct VariantDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub available_licenses: i32,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct VariantInput {
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub active: bool,
}
```

### Marketplace Domain (`models/marketplace.rs`)

```rust
pub struct ClaimedLicenseDto {
    pub license_id: String,
    pub lease_code: String,
    pub claim_token: String,
    pub claimed_at: DateTime<Utc>,
    pub device_id: Option<String>,
}

pub struct ReferralDto {
    pub id: Uuid,
    pub code: String,
    pub user_id: Option<Uuid>,
    pub status: String,
    pub expires_at: Option<DateTime<Utc>>,
}

pub struct VisitorStatsResponse {
    pub country_code: String,
    pub visitor_count: i64,
    pub period: String,
}
```

### Import/Export Domain (`models/import.rs`)

```rust
pub struct PublishLicensesRequest {
    pub licenses: Vec<LicenseInput>,
    pub idempotency_key: Option<String>,
}

pub struct PublishResult {
    pub created: i32,
    pub failed: i32,
    pub errors: Vec<PublishError>,
}

pub struct CsvImportRequest {
    pub csv_data: String,
    pub has_header: bool,
}

pub struct ImportResult {
    pub parsed: i32,
    pub inserted: i32,
    pub failed: i32,
    pub errors: Vec<ImportError>,
}
```

### Unetwork Response Models (`models/response/unetwork.rs`)

```rust
pub struct UnetworkLicense {
    pub id: Uuid,
    pub node_id: Option<String>,
    pub owner_wallet: Option<String>,
    pub alias: Option<String>,
    pub is_bound: bool,
    pub bound_at: Option<DateTime<Utc>>,
    pub activation_id: Option<Uuid>,
    pub activation_expires: Option<DateTime<Utc>>,
    pub is_online: bool,
    pub uptime_percentage: Option<f64>,
    pub lease_id: Option<Uuid>,
    pub lease_expires: Option<DateTime<Utc>>,
    pub lease_split: Option<f64>,
}

pub struct LicenseGroup {
    pub id: Uuid,
    pub name: String,
    pub licenses: Vec<UnetworkLicense>,
    pub total_count: i32,
}

pub struct RewardAllocation {
    pub id: Uuid,
    pub license_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub tx_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct RewardBalance {
    pub license_id: Uuid,
    pub total_earned: f64,
    pub total_paid: f64,
    pub pending: f64,
    pub currency: String,
}

pub struct LeaseDetails {
    pub id: Uuid,
    pub license_id: Uuid,
    pub lessee_wallet: String,
    pub split_percentage: f64,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub uptime_percentage: Option<f64>,
    pub uptime_requirement: f64,
}
```

---

## Traits (Repository Interfaces)

### LicenseRepository

Abstract database layer for license operations:

```rust
#[async_trait]
pub trait LicenseRepository: Send + Sync {
    // Insert operations
    async fn insert(&self, license: License) -> Result<License, DbError>;
    async fn insert_batch(&self, licenses: Vec<License>) -> Result<Vec<License>, DbError>;

    // Read operations
    async fn get_by_id(&self, id: Uuid) -> Result<Option<License>, DbError>;
    async fn get_by_lease_code(&self, code: &str) -> Result<Option<License>, DbError>;
    async fn get_first_unclaimed(&self, split: SplitType) -> Result<Option<License>, DbError>;
    async fn get_first_unclaimed_any(&self) -> Result<Option<License>, DbError>;
    async fn get_by_split_type(&self, split: SplitType) -> Result<Vec<License>, DbError>;
    async fn get_unclaimed(&self) -> Result<Vec<License>, DbError>;

    // Count operations
    async fn count_total(&self) -> Result<i64, DbError>;
    async fn count_unclaimed(&self) -> Result<i64, DbError>;
    async fn count_unclaimed_by_split(&self, split: SplitType) -> Result<i64, DbError>;

    // Update operations
    async fn claim(&self, id: Uuid, device_id: Option<String>) -> Result<License, DbError>;

    // Check operations
    async fn lease_code_exists(&self, code: &str) -> Result<bool, DbError>;

    // Delete operations
    async fn delete(&self, id: Uuid) -> Result<bool, DbError>;
    async fn delete_batch(&self, ids: Vec<Uuid>) -> Result<i32, DbError>;

    // Query operations
    async fn get_summary(&self) -> Result<LicenseSummary, DbError>;
    async fn search(&self, filters: LicenseFilters, pagination: PaginationParams)
        -> Result<Paginated<License>, DbError>;
}
```

### LicenseFilters

Query filter builder:

```rust
pub struct LicenseFilters {
    pub split_type: Option<SplitType>,
    pub claimed: Option<bool>,
    pub bound_to_device: Option<bool>,
    pub lease_code_search: Option<String>,
    pub include_expired: bool,
}

impl LicenseFilters {
    pub fn new() -> Self;
    pub fn with_split_type(self, split: SplitType) -> Self;
    pub fn with_claimed(self, claimed: bool) -> Self;
    pub fn unclaimed_only(self) -> Self;
}
```

### UnetworkLicenseTrait

External API operations:

```rust
#[async_trait]
pub trait UnetworkLicenseTrait: Send + Sync {
    async fn get_all_license_ids(&self, req: GetAllLicenseIdsRequest)
        -> Result<Vec<LicenseIdItem>, ApiError>;
    async fn get_all_license_groups(&self, req: GetAllLicenseGroupsRequest)
        -> Result<Vec<LicenseGroup>, ApiError>;
    async fn get_licenses(&self, req: GetLicensesRequest)
        -> Result<Vec<UnetworkLicense>, ApiError>;
    async fn get_lease_details(&self, req: GetLeaseDetailsRequest)
        -> Result<Option<LeaseDetails>, ApiError>;
}
```

### UnetworkRewardsTrait

Rewards API operations:

```rust
#[async_trait]
pub trait UnetworkRewardsTrait: Send + Sync {
    async fn get_rewards(&self, req: GetRewardsRequest)
        -> Result<Vec<RewardAllocation>, ApiError>;
    async fn get_balance(&self, req: GetRewardsBalanceRequest)
        -> Result<Option<RewardBalance>, ApiError>;
}
```

---

## Authentication

### HMAC-SHA256 Authentication (`auth/hmac.rs`)

**Algorithm:** HMAC-SHA256

**Signature Computation:**
```
signature = HMAC-SHA256(secret_key, "client_id:timestamp:nonce:payload_json")
```

**Functions:**
```rust
pub fn sign_payload<T: Serialize>(
    client_id: &str,
    secret_key: &[u8],
    timestamp: i64,
    nonce: &str,
    payload: &T,
) -> Result<String, ApiError>;

pub fn verify_signature<T: Serialize>(
    client_id: &str,
    secret_key: &[u8],
    timestamp: i64,
    nonce: &str,
    payload: &T,
    signature: &str,
    max_age_secs: u64,
) -> Result<(), AuthError>;

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool;  // Timing-attack resistant
```

### SignedRequest (`auth/signed_request.rs`)

Request wrapper with signature:

```rust
pub struct SignedRequest<T> {
    pub client_id: String,
    pub timestamp: i64,      // Unix seconds
    pub nonce: String,       // 16 random bytes, hex-encoded
    pub signature: String,   // Hex-encoded HMAC
    pub payload: T,
}

impl<T: Serialize> SignedRequest<T> {
    pub fn sign(client_id: &str, secret_key: &[u8], payload: T) -> Result<Self, ApiError>;
    pub fn with_timestamp(self, timestamp: i64) -> Self;  // For testing
}

pub fn generate_nonce() -> String;  // 16 random bytes, hex-encoded
```

### ClientRegistry (`auth/client_registry.rs`)

Thread-safe client credential store:

```rust
pub struct ClientCredentials {
    pub client_id: String,
    pub secret_key: Vec<u8>,
    pub roles: Vec<String>,
    pub active: bool,
}

pub struct ClientRegistry {
    clients: RwLock<HashMap<String, ClientCredentials>>,
}

impl ClientRegistry {
    pub fn new() -> Self;
    pub fn register(&self, client_id: &str, secret_key: &[u8]) -> Result<(), ApiError>;
    pub fn register_credentials(&self, creds: ClientCredentials) -> Result<(), ApiError>;
    pub fn get(&self, client_id: &str) -> Option<ClientCredentials>;
    pub fn exists(&self, client_id: &str) -> bool;
    pub fn deactivate(&self, client_id: &str) -> bool;
    pub fn activate(&self, client_id: &str) -> bool;
    pub fn remove(&self, client_id: &str) -> bool;
    pub fn list_clients(&self) -> Vec<String>;
}
```

### Preview Tokens (`auth/preview_token.rs`)

Time-limited tokens for draft content preview:

```rust
pub struct PreviewTokenPayload {
    pub content_id: String,
    pub schema_id: String,
    pub expires_at: i64,
    pub version: Option<i32>,
}

pub fn generate_preview_token(
    secret_key: &[u8],
    content_id: &str,
    schema_id: &str,
    ttl_secs: u64,
    version: Option<i32>,
) -> Result<String, ApiError>;

pub fn validate_preview_token(
    secret_key: &[u8],
    token: &str,
) -> Result<PreviewTokenPayload, AuthError>;
```

**Token Format:** `base64(json_payload).hex_signature`

**Default TTL:** 3600 seconds (1 hour)

---

## HTTP Clients

### UnoApiClient (`client/uno_app_client.rs`)

HMAC-authenticated HTTP client for uno-app API:

```rust
pub struct UnoApiClient {
    config: ClientConfig,
    http: reqwest::Client,
}

impl UnoApiClient {
    pub fn new(config: ClientConfig) -> Self;

    // License Operations
    pub async fn publish_licenses(&self, req: PublishLicensesRequest)
        -> Result<PublishResult, ApiError>;
    pub async fn import_csv(&self, req: CsvImportRequest)
        -> Result<ImportResult, ApiError>;
    pub async fn get_license(&self, id: Uuid)
        -> Result<Option<LicenseDto>, ApiError>;
    pub async fn search_licenses(&self, filters: LicenseFilters, pagination: PaginationParams)
        -> Result<Paginated<LicenseDto>, ApiError>;
    pub async fn revoke_licenses(&self, req: RevokeRequest)
        -> Result<RevokeResult, ApiError>;
    pub async fn revoke_license(&self, id: Uuid)
        -> Result<bool, ApiError>;

    // Variant Operations
    pub async fn list_variants(&self)
        -> Result<Vec<VariantDto>, ApiError>;
    pub async fn get_variant_summary(&self)
        -> Result<VariantSummary, ApiError>;
    pub async fn upsert_variant(&self, input: VariantInput)
        -> Result<VariantDto, ApiError>;

    // Marketplace Operations
    pub async fn get_claimed_licenses(&self, req: GetClaimedLicensesRequest)
        -> Result<Vec<ClaimedLicenseDto>, ApiError>;
    pub async fn get_referrals(&self)
        -> Result<Vec<ReferralDto>, ApiError>;
    pub async fn sync_referrals(&self, req: SyncReferralsRequest)
        -> Result<i32, ApiError>;
    pub async fn get_visitor_stats(&self, req: GetVisitorStatsRequest)
        -> Result<Vec<VisitorStatsResponse>, ApiError>;

    // Health
    pub async fn health(&self) -> Result<bool, ApiError>;
}
```

### UnetworkClient (`client/unetwork_client.rs`)

JWT Bearer-authenticated HTTP client for Unetwork API:

```rust
pub struct UnetworkClient {
    config: UnetworkConfig,
    http: reqwest::Client,
}

impl UnetworkClient {
    pub fn new(config: UnetworkConfig) -> Self;
    pub fn with_http_client(config: UnetworkConfig, http: reqwest::Client) -> Self;
    pub fn jwt_token(&self) -> &str;
    pub fn set_jwt_token(&mut self, token: String);
    pub fn has_token(&self) -> bool;
}

// Implements UnetworkLicenseTrait
impl UnetworkLicenseTrait for UnetworkClient {
    async fn get_all_license_ids(&self, req: GetAllLicenseIdsRequest)
        -> Result<Vec<LicenseIdItem>, ApiError>;
    async fn get_all_license_groups(&self, req: GetAllLicenseGroupsRequest)
        -> Result<Vec<LicenseGroup>, ApiError>;
    async fn get_licenses(&self, req: GetLicensesRequest)
        -> Result<Vec<UnetworkLicense>, ApiError>;
    async fn get_lease_details(&self, req: GetLeaseDetailsRequest)
        -> Result<Option<LeaseDetails>, ApiError>;
}

// Implements UnetworkRewardsTrait
impl UnetworkRewardsTrait for UnetworkClient {
    async fn get_rewards(&self, req: GetRewardsRequest)
        -> Result<Vec<RewardAllocation>, ApiError>;
    async fn get_balance(&self, req: GetRewardsBalanceRequest)
        -> Result<Option<RewardBalance>, ApiError>;
}
```

**Endpoint Patterns:**
- RPC: `POST /rest/v1/rpc/{function}` (PostgREST)
- Functions: `POST /functions/v1/{function}` (Edge Functions)

---

## Services

### LicenseAdminService (`services/license_admin.rs`)

Orchestrates license management operations:

```rust
pub struct LicenseAdminService {
    repository: Arc<dyn LicenseRepository>,
}

impl LicenseAdminService {
    pub fn new(repository: Arc<dyn LicenseRepository>) -> Self;

    pub async fn publish_licenses(&self, req: PublishLicensesRequest)
        -> Result<PublishResult, ApiError>;

    pub async fn import_csv(&self, req: CsvImportRequest)
        -> Result<ImportResult, ApiError>;

    pub async fn get_license(&self, id: Uuid)
        -> Result<Option<LicenseDto>, ApiError>;

    pub async fn get_license_by_code(&self, code: &str)
        -> Result<Option<LicenseDto>, ApiError>;

    pub async fn claim_license(&self, req: ClaimRequest)
        -> Result<ClaimResponse, ApiError>;

    pub async fn search_licenses(&self, filters: LicenseFilters, pagination: PaginationParams)
        -> Result<Paginated<LicenseDto>, ApiError>;

    pub async fn revoke_licenses(&self, req: RevokeRequest)
        -> Result<RevokeResult, ApiError>;

    pub async fn get_summary(&self)
        -> Result<LicenseSummary, ApiError>;

    pub async fn get_available_by_split(&self, split: SplitType)
        -> Result<i64, ApiError>;
}
```

### CSV Import (`services/csv_import.rs`)

CSV parsing and validation:

```rust
pub struct ParsedCsv {
    pub licenses: Vec<LicenseInput>,
    pub parse_errors: Vec<ImportError>,
    pub total_rows: i32,
}

pub fn parse_csv(csv_data: &str, has_header: bool) -> Result<ParsedCsv, ApiError>;
```

**Features:**
- Auto-detects columns (case-insensitive)
- Supports column aliases (e.g., "leasecode", "lease_code", "code")
- Multiple date formats: ISO (YYYY-MM-DD), DD/MM/YYYY, MM/DD/YYYY
- Positional fallback for headless CSV
- Error recovery: continues parsing on bad rows

### UnetworkLicenseService (`services/unetwork/licenses_service.rs`)

Wrapper around UnetworkLicenseTrait with convenience methods:

```rust
pub struct UnetworkLicenseService<P: UnetworkLicenseTrait> {
    provider: Arc<P>,
}

impl<P: UnetworkLicenseTrait> UnetworkLicenseService<P> {
    pub fn new(provider: Arc<P>) -> Self;

    // Filtered queries
    pub async fn get_leased_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError>;
    pub async fn get_bound_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError>;
    pub async fn get_grouped_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError>;
    pub async fn get_active_license_ids(&self) -> Result<Vec<LicenseIdItem>, ApiError>;

    // License queries
    pub async fn get_enabled_licenses(&self, skip: i32, take: i32)
        -> Result<Vec<UnetworkLicense>, ApiError>;
    pub async fn get_licenses_by_role(&self, role: &str, skip: i32, take: i32)
        -> Result<Vec<UnetworkLicense>, ApiError>;
    pub async fn find_license_by_id(&self, id: Uuid)
        -> Result<Option<UnetworkLicense>, ApiError>;

    // Group queries
    pub async fn get_licenses_by_group(&self, group_id: Uuid)
        -> Result<Vec<UnetworkLicense>, ApiError>;
    pub async fn find_group_by_name(&self, name: &str)
        -> Result<Option<LicenseGroup>, ApiError>;

    // Status queries
    pub async fn get_online_licenses(&self) -> Result<Vec<UnetworkLicense>, ApiError>;
    pub async fn get_underperforming_licenses(&self) -> Result<Vec<UnetworkLicense>, ApiError>;

    // Statistics
    pub async fn get_statistics(&self) -> Result<LicenseStatistics, ApiError>;
}

pub struct LicenseStatistics {
    pub total_groups: i32,
    pub total_licenses: i32,
    pub online_count: i32,
    pub leased_count: i32,
    pub bound_count: i32,
    pub average_uptime: f64,
}
```

### UnetworkRewardsService (`services/unetwork/rewards_service.rs`)

Wrapper around UnetworkRewardsTrait with convenience methods:

```rust
pub struct UnetworkRewardsService<P: UnetworkRewardsTrait> {
    provider: Arc<P>,
}

impl<P: UnetworkRewardsTrait> UnetworkRewardsService<P> {
    pub fn new(provider: Arc<P>) -> Self;

    pub async fn get_rewards(&self, license_id: Uuid, skip: i32, take: i32)
        -> Result<Vec<RewardAllocation>, ApiError>;

    pub async fn get_all_rewards(&self, license_id: Uuid)
        -> Result<Vec<RewardAllocation>, ApiError>;

    pub async fn get_balance(&self, license_id: Uuid)
        -> Result<Option<RewardBalance>, ApiError>;

    pub async fn get_pending_rewards(&self, license_id: Uuid)
        -> Result<Vec<RewardAllocation>, ApiError>;

    pub async fn get_paid_rewards(&self, license_id: Uuid)
        -> Result<Vec<RewardAllocation>, ApiError>;

    pub async fn get_total_earned(&self, license_id: Uuid)
        -> Result<f64, ApiError>;

    pub async fn get_total_pending(&self, license_id: Uuid)
        -> Result<f64, ApiError>;
}
```

---

## Feature Flags

| Feature | Dependencies | Purpose |
|---------|--------------|---------|
| `services` (default) | csv, validator | CSV parsing, validation |
| `auth` | hmac, sha2, hex, base64 | HMAC authentication |
| `client` | reqwest, tokio (implies auth) | HTTP clients |
| `full` | all above | Everything enabled |

**Cargo.toml Configuration:**
```toml
[features]
default = ["services"]
services = ["csv", "validator"]
auth = ["hmac", "sha2", "hex", "base64"]
client = ["reqwest", "tokio", "auth"]
full = ["services", "auth", "client"]
```

---

## API Endpoints

### UNO-APP Endpoints (Provided by uno-app)

All authenticated with HMAC-SHA256 via ClientRegistry.

**License Management:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/api/v1/admin/licenses` | Publish batch |
| POST | `/api/v1/admin/licenses/import` | Import from CSV |
| GET | `/api/v1/admin/licenses/{id}` | Get license |
| POST | `/api/v1/admin/licenses/search` | Search with filters |
| DELETE | `/api/v1/admin/licenses` | Revoke batch |

**Variants:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/v1/admin/variants` | List all |
| GET | `/api/v1/admin/variants/summary` | Get statistics |
| POST | `/api/v1/admin/variants` | Create/update |

**Marketplace:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/api/v1/admin/licenses/claimed` | Get claimed licenses |
| POST | `/api/v1/admin/referrals` | Get referrals |
| POST | `/api/v1/admin/referrals/sync` | Sync referrals |
| POST | `/api/v1/admin/stats/visitors` | Get visitor stats |

**Health:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/api/v1/admin/health` | Health check |

### Unetwork API Endpoints (External Service)

All authenticated with Bearer JWT token.

**Licenses:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/rest/v1/rpc/licenses_get_all_ids` | Get all license IDs |
| POST | `/functions/v1/license_groups_get_all` | Get all groups |
| POST | `/functions/v1/licenses_get_licenses` | Get licenses paginated |
| POST | `/rest/v1/rpc/get_lease_id_by_license` | Get lease details |

**Rewards:**
| Method | Endpoint | Purpose |
|--------|----------|---------|
| POST | `/rest/v1/rpc/rewards_get_allocations` | Get allocations |
| POST | `/rest/v1/rpc/rewards_get_balance` | Get balance |

---

## Design Patterns

### 1. Dependency Injection

Services accept `Arc<dyn Trait>` for repositories:

```rust
let repo = Arc::new(PostgresLicenseRepository::new(pool));
let service = LicenseAdminService::new(repo);
```

### 2. Trait-Based Abstraction

Multiple implementations possible for each trait:
- `LicenseRepository` - Database operations
- `UnetworkLicenseTrait` - License API operations
- `UnetworkRewardsTrait` - Rewards API operations

### 3. Builder Pattern

Fluent configuration and filter building:

```rust
let config = ClientConfig::new(base_url, client_id, secret)
    .with_timeout(Duration::from_secs(60))
    .with_api_prefix("/api/v2/admin");

let filters = LicenseFilters::new()
    .with_split_type(SplitType::Split5050)
    .unclaimed_only();
```

### 4. Wrapper Pattern

Services wrap trait providers with convenience methods:

```rust
let client = Arc::new(UnetworkClient::new(config));
let service = UnetworkLicenseService::new(client);

// Direct trait method
client.get_all_license_ids(req).await?;

// Convenience method
service.get_leased_license_ids().await?;
```

### 5. Error Mapping

Domain errors mapped to application errors to HTTP status:

```rust
DbError::NotFound → ApiError::NotFound → HTTP 404
DbError::DuplicateEntry → ApiError::Conflict → HTTP 409
AuthError::InvalidSignature → HTTP 401
```

---

## Integration with uno-app

uno-api provides traits and services that uno-app implements:

```rust
// uno-app implements the repository trait
struct PostgresLicenseRepository { pool: PgPool }

#[async_trait]
impl LicenseRepository for PostgresLicenseRepository {
    async fn insert(&self, license: License) -> Result<License, DbError> {
        sqlx::query_as!(...)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::QueryError(e.to_string()))
    }
    // ... other methods
}

// uno-app creates services with DI
let repo = Arc::new(PostgresLicenseRepository::new(pool));
let service = LicenseAdminService::new(repo);

// uno-app exposes HTTP endpoints
async fn publish_handler(
    req: SignedRequest<PublishLicensesRequest>,
    registry: Data<ClientRegistry>,
    service: Data<LicenseAdminService>,
) -> Result<Json<PublishResult>, ApiError> {
    // Verify HMAC signature
    verify_request(&req, &registry, 300)?;

    // Execute business logic
    let result = service.publish_licenses(req.payload).await?;

    Ok(Json(result))
}
```

---

## Notable Features

| Feature | Details |
|---------|---------|
| **Lease Code Generation** | `XXXX-XXXX-XXXX` format, excludes ambiguous chars (0, O, I, L, 1) |
| **Flexible ID Handling** | UUID or hex string (up to 64 chars), auto-truncation |
| **Multi-Format Dates** | ISO 8601, DD/MM/YYYY, MM/DD/YYYY, converts to UTC |
| **Revenue Splits** | 50:50, 55:45, 60:40 with `user_share()` / `operator_share()` |
| **Uptime Tracking** | `meets_uptime_requirement()` enforcement |
| **Pagination** | Page-based (`offset()`/`limit()`) and skip/take models |
| **Replay Protection** | Configurable timestamp window (default 5 minutes) |
| **Timing-Attack Resistance** | `constant_time_eq()` for signature comparison |

---

## External Dependencies

### Always Required
- `async-trait` - Async trait implementations
- `serde` / `serde_json` - Serialization
- `chrono` - Datetime handling
- `uuid` - UUID generation
- `rand` - Random number generation
- `thiserror` - Error macros
- `tracing` - Logging

### With `services` Feature
- `csv` - CSV parsing
- `validator` - Input validation

### With `auth` Feature
- `hmac` - HMAC computation
- `sha2` - SHA-256 hashing
- `hex` - Hex encoding
- `base64` - Base64 encoding

### With `client` Feature
- `reqwest` - HTTP client
- `tokio` - Async runtime
