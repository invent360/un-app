# P0-01: Configuration and Secrets Inventory

**Phase 0 Task**: Secrets & Configuration Inventory
**Status**: Complete
**Generated**: 2026-09-29

---

## Overview

This document provides a comprehensive inventory of all environment variables, configuration structs, secrets, and feature flags used across the u-network codebase. The applications covered are:

- **uno-app**: Main web application (Leptos SSR/WASM)
- **uno-admin**: Admin dashboard (Leptos SSR)
- **uno-api**: Shared API client library
- **file-storage**: File storage abstraction layer

---

## 1. uno-app Configuration

### 1.1 Database Configuration

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `DATABASE_URL` | env | **Yes** | PostgreSQL connection string | `postgres://postgres:postgres@localhost:5432/uno_app` | Contains credentials. Use secrets manager in production. |
| `DATABASE_MAX_CONNECTIONS` | env | No | Connection pool size | `10` | - |

**Configuration Struct**: `uno-app/src/config/settings.rs` - `DatabaseConfig`

### 1.2 Authentication & Security

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `CSRF_SECRET_KEY` | env | **Yes (prod)** | CSRF token signing | None | Must be at least 32 characters. Must not start with "dev-". |
| `SESSION_SECRET` | env | **Yes (prod)** | Session cookie signing | Dev default | Must be at least 32 characters in production. |
| `ADMIN_API_KEY` | env | **Yes** | Machine-to-machine API authentication | None | Must be at least 32 characters. Must not be "dev-admin-key". |
| `ADMIN_CLIENT_ID` | env | No | Client ID for admin API calls | - | Used for HMAC request signing. |
| `ADMIN_SECRET_KEY` | env | No | Secret key for admin API calls | - | Used for HMAC request signing. SENSITIVE. |
| `PREVIEW_SECRET_KEY` | env | No | Content preview authentication | - | Allows preview of unpublished content. |

**Configuration Struct**: `uno-app/src/server/secrets.rs` - `AppSecrets`, `SecretsManager`

### 1.3 Session Verification (JWT)

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `UNO_SESSION_PUBLIC_KEYS` | env | **Yes** | JSON map of kid -> PEM public keys | None | RSA-256 public keys for JWT verification. |
| `UNO_SESSION_ISSUER` | env | **Yes** | Expected JWT issuer | None | Must match token issuer claim. |
| `UNO_SESSION_AUDIENCE` | env | **Yes** | Expected JWT audience | None | Must match token audience claim. |
| `UNO_PUBLIC_ORIGIN` | env | **Yes** | Public origin URL | None | Must start with https:// or http://localhost:. No trailing slash. |

**Configuration Struct**: `uno-api/src/auth/session.rs` - `SessionVerifier`

### 1.4 Environment Mode

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `RUST_ENV` | env | No | Environment mode | - | Set to "production" for production mode. |
| `LEPTOS_ENV` | env | No | Leptos environment | - | Set to "PROD" for production mode. |
| `UNO_UI_ONLY` | env | No | UI-only development mode | `false` | Set to "true" to skip database. Dev only. |
| `LEPTOS_SITE_ADDR` | env | No | Server bind address | From Leptos config | Format: `127.0.0.1:3000`. |

### 1.5 GeoIP Configuration

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `GEOIP_DATABASE_PATH` | env | No | Path to MaxMind GeoIP database | `data/GeoLite2-Country.mmdb` | - |

### 1.6 Logging

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `RUST_LOG` | env | No | Logging level | `info` | Standard Rust logging filter. |
| `LOG_FORMAT` | env | No | Log output format | `json` | Set to "text" for human-readable. |

### 1.7 Background Scheduler

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `SCHEDULER_INTERVAL_SECS` | env | No | Content scheduler interval | - | Background job interval. |

### 1.8 Secrets Path

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `SECRETS_PATH` | env | No | Base path for file-based secrets | None | For Docker/Kubernetes secrets (e.g., `/run/secrets`). |

---

## 2. uno-admin Configuration

### 2.1 ScyllaDB Configuration

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `SCYLLA_DB_HOST` | env | No | ScyllaDB host | `127.0.0.1` | - |
| `SCYLLA_DB_PORT` | env | No | ScyllaDB port | `9042` | - |
| `SCYLLA_DB_KEYSPACE` | env | No | ScyllaDB keyspace | `unity_dashboard` | Hyphens converted to underscores. |
| `SCYLLA_DB_USERNAME` | env | No | ScyllaDB username | None | Only if authentication enabled. |
| `SCYLLA_DB_PASSWORD` | env | No | ScyllaDB password | None | SENSITIVE. Only if authentication enabled. |
| `SCYLLA_DB_REPLICATION_FACTOR` | env | No | Replication factor | `1` | Increase for production. |
| `SCYLLA_KEYSPACE` | env | No | Keyspace (health check) | None | Alias used in health checks. |
| `SCYLLA_URI` | env | No | ScyllaDB URI (health check) | None | Used in startup checks. |

**Configuration Struct**: `uno-admin/src/db/connection.rs` - `DatabaseConfig`

### 2.2 PostgreSQL Configuration (postgres-db feature)

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `DATABASE_URL` | env | **Yes** | PostgreSQL connection string | None | Contains credentials. SENSITIVE. |
| `DATABASE_MAX_CONNECTIONS` | env | No | Connection pool size | `10` | - |
| `DATABASE_ACQUIRE_TIMEOUT` | env | No | Pool acquire timeout (secs) | `5` | - |
| `DATABASE_IDLE_TIMEOUT` | env | No | Idle connection timeout (secs) | `600` | - |
| `PG_MIGRATIONS_DIR` | env | No | PostgreSQL migrations directory | `../uno-app/migrations` | Shared with uno-app. |

**Configuration Struct**: `uno-admin/src/db/connection.rs` - `PostgresConfig`

### 2.3 Database Operations

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `RUN_MIGRATIONS` | env | No | Run migrations on startup | `true` | Set to "false" to skip. |
| `DESTROY_TABLES` | env | No | Drop all tables on startup | `false` | **DANGEROUS**. Development only. |
| `SKIP_SEED` | env | No | Skip database seeding | `false` | Set to "true" to skip. |
| `MIGRATIONS_DIR` | env | No | ScyllaDB migrations directory | `./migrations` | - |

### 2.4 Data Seeding

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `REWARDS_JSON_PATH` | env | No | Rewards data file path | `./data/incentives/rewards.json` | - |
| `REWARDS_CHUNKS_DIR` | env | No | Rewards chunks directory | `./data/rewards` | - |

### 2.5 UNO App Integration

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `UNO_API_URL` | env | No | UNO App API URL | `http://localhost:3000` | - |
| `UNO_CLIENT_ID` | env | No | UNO App client ID | - | Fallback for ADMIN_CLIENT_ID. |
| `UNO_SECRET_KEY` | env | No | UNO App secret key | - | Fallback for ADMIN_SECRET_KEY. SENSITIVE. |
| `UNO_APP_URL` | env | No | UNO App public URL | `http://localhost:3000` | Used for preview links. |
| `PREVIEW_SECRET_KEY` | env | No | Preview authentication | None | For content preview. |

### 2.6 Unity API Configuration

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `API_TOKEN` | env | No | Unity API JWT token | None | Takes precedence over UNITY_JWT_TOKEN. SENSITIVE. |
| `UNITY_JWT_TOKEN` | env | No | Unity API JWT token | None | Fallback if API_TOKEN not set. SENSITIVE. |
| `UNITY_API_URL` | env | No | Unity API base URL | `https://api.unityedge.io` | - |
| `UNITY_API_KEY` | env | No | Unity API key | Publishable key | Public key, safe to expose. |

**Configuration Struct**: `uno-admin/src/api/config.rs` - `UnityApiConfig`

### 2.7 Admin Authentication

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `ADMIN_CLIENT_ID` | env | No | Admin client ID | - | - |
| `ADMIN_SECRET_KEY` | env | No | Admin secret key | - | SENSITIVE. |
| `ADMIN_API_KEY` | env | No | Admin API key | - | SENSITIVE. |

---

## 3. uno-api (Shared Library) Configuration

### 3.1 Unetwork API Client

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `UNETWORK_API_URL` | env | No | Unetwork API base URL | `https://api.unityedge.io` | - |
| `UNETWORK_API_KEY` | env | **Yes (for upstream)** | Unetwork API key | None | Required for upstream operations. |

**Configuration Struct**: `uno-api/src/config/unetwork_config.rs` - `UnetworkConfig`

### 3.2 UNO App Client

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| Base URL | programmatic | - | API base URL | `http://localhost:3000` | - |
| Client ID | programmatic | - | Client identifier | - | - |
| Secret Key | programmatic | - | HMAC signing key | - | SENSITIVE. |
| API Key | programmatic | - | Bearer token | - | SENSITIVE. |

**Configuration Struct**: `uno-api/src/config/uno_app_config.rs` - `ClientConfig`

---

## 4. file-storage Configuration

### 4.1 Backend Selection

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `FILE_STORAGE_BACKEND` | env | No | Storage backend type | `local` | Options: `gcs`, `local`, `s3` (future), `azure` (future). |

### 4.2 Google Cloud Storage (GCS)

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `GCS_BUCKET_NAME` | env | **Yes (for GCS)** | GCS bucket name | None | - |
| `GCS_SERVICE_ACCOUNT_KEY` | env | Conditional | Path to service account JSON | None | Either this or GCS_SERVICE_ACCOUNT_JSON required. SENSITIVE. |
| `GCS_SERVICE_ACCOUNT_JSON` | env | Conditional | Service account JSON string | None | Either this or GCS_SERVICE_ACCOUNT_KEY required. SENSITIVE. |

### 4.3 Local Storage

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `FILE_STORAGE_LOCAL_PATH` | env | **Yes (for local)** | Base path for file storage | None | - |
| `FILE_STORAGE_LOCAL_URL` | env | No | Base URL for serving files | `/files` | - |
| `FILE_STORAGE_VOLUME_ID` | env | No | Volume identifier check | None | Production security check. |

### 4.4 AWS S3 (Future)

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `AWS_S3_BUCKET` | env | **Yes (for S3)** | S3 bucket name | None | - |
| `AWS_ACCESS_KEY_ID` | env | **Yes (for S3)** | AWS access key | None | SENSITIVE. |
| `AWS_SECRET_ACCESS_KEY` | env | **Yes (for S3)** | AWS secret key | None | SENSITIVE. |

### 4.5 Azure Blob (Future)

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `AZURE_STORAGE_ACCOUNT` | env | **Yes (for Azure)** | Storage account name | None | - |
| `AZURE_STORAGE_KEY` | env | **Yes (for Azure)** | Storage account key | None | SENSITIVE. |

### 4.6 Common Settings

| Variable | Source | Required | Purpose | Default | Security Notes |
|----------|--------|----------|---------|---------|----------------|
| `FILE_MAX_SIZE` | env | No | Maximum file size (bytes) | `10485760` (10MB) | - |
| `FILE_SIGNED_URL_EXPIRY_SECS` | env | No | Signed URL expiry | `604800` (7 days) | - |

**Configuration Struct**: `file-storage/src/config/builder.rs` - `StorageConfigBuilder`, `GcsConfig`, `LocalConfig`

---

## 5. Feature Flags

### 5.1 uno-app Features

| Feature | Purpose | Enabled By Default |
|---------|---------|-------------------|
| `csr` | Client-side rendering (WASM) | No |
| `hydrate` | SSR + hydration | No |
| `ssr` | Server-side rendering | No |
| `debug-routes` | Debug API routes | **No - NEVER enable in production** |

### 5.2 uno-admin Features

| Feature | Purpose | Enabled By Default |
|---------|---------|-------------------|
| `csr` | Client-side rendering | No |
| `hydrate` | SSR + hydration | No |
| `seeder` | Standalone database seeding | No |
| `postgres-db` | Use PostgreSQL instead of ScyllaDB | No |

---

## 6. Configuration Structs Summary

### 6.1 uno-app

| Struct | File | Purpose |
|--------|------|---------|
| `DatabaseConfig` | `src/config/settings.rs` | Database connection settings |
| `AdminConfig` | `src/config/settings.rs` | Admin API key validation |
| `AppSettings` | `src/config/settings.rs` | Combined app settings |
| `AppSecrets` | `src/server/secrets.rs` | Secrets management (CSRF, session, DB) |
| `SecretsManager` | `src/server/secrets.rs` | Multi-source secret loading |
| `Secret` | `src/server/secrets.rs` | Redacted secret wrapper |
| `AdminAuth` | `src/server/middleware/auth_middleware.rs` | API key middleware |

### 6.2 uno-admin

| Struct | File | Purpose |
|--------|------|---------|
| `DatabaseConfig` | `src/db/connection.rs` | ScyllaDB connection settings |
| `PostgresConfig` | `src/db/connection.rs` | PostgreSQL connection settings |
| `UnityApiConfig` | `src/api/config.rs` | Unity API client configuration |

### 6.3 uno-api

| Struct | File | Purpose |
|--------|------|---------|
| `UnetworkConfig` | `src/config/unetwork_config.rs` | Unetwork API configuration |
| `ClientConfig` | `src/config/uno_app_config.rs` | UNO App client configuration |
| `SessionVerifier` | `src/auth/session.rs` | JWT session verification |
| `Principal` | `src/auth/session.rs` | Authenticated user claims |

### 6.4 file-storage

| Struct | File | Purpose |
|--------|------|---------|
| `StorageConfigBuilder` | `src/config/builder.rs` | Storage configuration builder |
| `StorageConfig` | `src/config/builder.rs` | Resolved storage configuration |
| `GcsConfig` | `src/config/builder.rs` | GCS-specific configuration |
| `LocalConfig` | `src/config/builder.rs` | Local storage configuration |
| `StorageBackend` | `src/config/builder.rs` | Backend type enum |

---

## 7. Security Recommendations

### 7.1 Required for Production

The following secrets MUST be properly configured for production deployments:

1. **Database credentials** (`DATABASE_URL`) - Use secrets manager
2. **CSRF secret** (`CSRF_SECRET_KEY`) - At least 32 characters, randomly generated
3. **Session secret** (`SESSION_SECRET`) - At least 32 characters, randomly generated
4. **Admin API key** (`ADMIN_API_KEY`) - At least 32 characters, randomly generated
5. **JWT verification keys** (`UNO_SESSION_PUBLIC_KEYS`) - RSA-256 public keys
6. **GCS service account** (`GCS_SERVICE_ACCOUNT_KEY` or `GCS_SERVICE_ACCOUNT_JSON`) - If using GCS

### 7.2 Never in Production

The following should NEVER be set in production:

1. `DESTROY_TABLES=true` - Will drop all database tables
2. `debug-routes` feature - Exposes sensitive debug endpoints
3. `UNO_UI_ONLY=true` - Bypasses database requirement
4. Default secrets starting with "dev-"

### 7.3 Secret Loading Priority

The `SecretsManager` loads secrets with the following priority:

1. **File-based secrets** (Docker/Kubernetes) - Path: `$SECRETS_PATH/{name}`
2. **Environment variables** - Exact name match
3. **Environment variables** - Uppercase name fallback

---

## 8. Sample .env Files

### 8.1 uno-app Development (.env.example)

```bash
# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/uno_app
DATABASE_MAX_CONNECTIONS=10

# Admin API Key (generate a secure random string for production)
ADMIN_API_KEY=your-secure-api-key-here

# Environment
LEPTOS_ENV=DEV
```

### 8.2 uno-admin Development (.env)

```bash
# Unity API JWT Token (can be overridden with API_TOKEN)
API_TOKEN=your-jwt-token-here

UNO_API_URL=http://localhost:3000
UNO_CLIENT_ID=uno-admin
UNO_SECRET_KEY=your-secure-secret-key-here-change-in-production

# ScyllaDB Configuration
SCYLLA_DB_HOST=127.0.0.1
SCYLLA_DB_PORT=9042
SCYLLA_DB_KEYSPACE=unity_dashboard
SCYLLA_DB_REPLICATION_FACTOR=1

# Set to true to drop all tables on startup (use with caution!)
DESTROY_TABLES=false

# Set to false to skip migrations
RUN_MIGRATIONS=true

# File Storage (GCS)
FILE_STORAGE_BACKEND=gcs
GCS_BUCKET_NAME=your-bucket
GCS_SERVICE_ACCOUNT_KEY=/path/to/service-account.json
```

---

## 9. Appendix: Environment Variable Quick Reference

### All Variables by Category

**Database (PostgreSQL)**
- `DATABASE_URL`, `DATABASE_MAX_CONNECTIONS`, `DATABASE_ACQUIRE_TIMEOUT`, `DATABASE_IDLE_TIMEOUT`

**Database (ScyllaDB)**
- `SCYLLA_DB_HOST`, `SCYLLA_DB_PORT`, `SCYLLA_DB_KEYSPACE`, `SCYLLA_DB_USERNAME`, `SCYLLA_DB_PASSWORD`, `SCYLLA_DB_REPLICATION_FACTOR`

**Authentication**
- `ADMIN_API_KEY`, `ADMIN_CLIENT_ID`, `ADMIN_SECRET_KEY`, `CSRF_SECRET_KEY`, `SESSION_SECRET`, `PREVIEW_SECRET_KEY`

**Session JWT**
- `UNO_SESSION_PUBLIC_KEYS`, `UNO_SESSION_ISSUER`, `UNO_SESSION_AUDIENCE`, `UNO_PUBLIC_ORIGIN`

**External APIs**
- `UNO_API_URL`, `UNO_APP_URL`, `UNO_CLIENT_ID`, `UNO_SECRET_KEY`
- `UNITY_API_URL`, `UNITY_API_KEY`, `UNITY_JWT_TOKEN`, `API_TOKEN`
- `UNETWORK_API_URL`, `UNETWORK_API_KEY`

**File Storage**
- `FILE_STORAGE_BACKEND`, `FILE_STORAGE_LOCAL_PATH`, `FILE_STORAGE_LOCAL_URL`, `FILE_STORAGE_VOLUME_ID`
- `FILE_MAX_SIZE`, `FILE_SIGNED_URL_EXPIRY_SECS`

**GCS**
- `GCS_BUCKET_NAME`, `GCS_SERVICE_ACCOUNT_KEY`, `GCS_SERVICE_ACCOUNT_JSON`

**AWS (Future)**
- `AWS_S3_BUCKET`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`

**Azure (Future)**
- `AZURE_STORAGE_ACCOUNT`, `AZURE_STORAGE_KEY`

**Environment/Mode**
- `RUST_ENV`, `LEPTOS_ENV`, `LEPTOS_SITE_ADDR`, `UNO_UI_ONLY`

**Logging**
- `RUST_LOG`, `LOG_FORMAT`

**Operational**
- `RUN_MIGRATIONS`, `DESTROY_TABLES`, `SKIP_SEED`, `MIGRATIONS_DIR`, `PG_MIGRATIONS_DIR`
- `SCHEDULER_INTERVAL_SECS`, `SECRETS_PATH`, `GEOIP_DATABASE_PATH`

**Data Seeding**
- `REWARDS_JSON_PATH`, `REWARDS_CHUNKS_DIR`

---

*Document generated as part of Phase 0 infrastructure audit.*
