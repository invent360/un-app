# uno-app Codebase Analysis & Documentation

## Executive Summary

uno-app is a full-stack Rust web application built with the Leptos framework for license management and claiming. It provides a wizard-based UI for users to claim licenses with different revenue split options (50:50, 55:45, 60:40), backed by a PostgreSQL database and secured admin APIs.

---

## 1. Architecture & Layers

### 1.1 Technology Stack

| Layer | Technology |
|-------|------------|
| Frontend | Leptos (Rust WASM), SCSS |
| Backend | Actix-web, Leptos SSR |
| Database | PostgreSQL with SQLx |
| Authentication | HMAC-SHA256 (Admin), CSRF (Public) |
| Internationalization | PHF static maps (9 languages) |
| External Services | MaxMind GeoLite2 (GeoIP) |

### 1.2 Directory Structure

```
uno-app/
├── src/
│   ├── api/                    # Leptos server functions
│   │   └── licenses.rs         # License API endpoints
│   ├── components/             # UI Components
│   │   ├── wizard/             # Multi-step wizard
│   │   │   ├── mod.rs          # Wizard state machine
│   │   │   └── stages/         # Wizard stages (location, select, review, claim, success)
│   │   ├── license_card.rs     # License display card
│   │   └── ...
│   ├── locales/                # i18n translations
│   │   ├── mod.rs              # Translation lookup
│   │   ├── en.rs, de.rs, ...   # 9 language files
│   ├── server/                 # Server-side code
│   │   ├── app.rs              # ServiceFactory setup
│   │   ├── handlers/           # HTTP handlers
│   │   │   └── licenses_handler.rs
│   │   ├── middleware/         # HMAC auth, rate limiting
│   │   ├── repositories/       # Database access
│   │   └── services/           # Business logic
│   │       └── license_service.rs
│   ├── types/                  # Shared type definitions
│   │   ├── license.rs          # License, SplitType, ClaimResponse
│   │   └── error.rs            # AppError
│   └── lib.rs, main.rs
├── style/
│   └── main.scss               # Global styles
├── uno-api/                    # External API crate (trait definitions)
└── docs/                       # Documentation
```

### 1.3 Architectural Layers

```
┌─────────────────────────────────────────────────────────────┐
│                     PRESENTATION LAYER                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Wizard    │  │  Components │  │   Locales (i18n)    │  │
│  │   Stages    │  │  (Cards,    │  │   9 Languages       │  │
│  │             │  │   Buttons)  │  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      API LAYER                               │
│  ┌─────────────────────┐  ┌─────────────────────────────┐   │
│  │  Leptos Server Fns  │  │   Actix HTTP Handlers       │   │
│  │  (src/api/)         │  │   (src/server/handlers/)    │   │
│  │  - get_variants     │  │   - GET /api/v1/licenses/*  │   │
│  │  - claim_license    │  │   - POST /api/v1/admin/*    │   │
│  └─────────────────────┘  └─────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    SERVICE LAYER                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              ServiceFactory (DI Container)           │    │
│  │  ┌─────────────────┐  ┌─────────────────────────┐   │    │
│  │  │ LicenseService  │  │   GeoIP Service         │   │    │
│  │  │ - claim_license │  │   - Country detection   │   │    │
│  │  │ - get_variants  │  │                         │   │    │
│  │  └─────────────────┘  └─────────────────────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  REPOSITORY LAYER                            │
│  ┌─────────────────────────────────────────────────────┐    │
│  │    uno-api Traits    │    uno-app Implementations   │    │
│  │  ┌────────────────┐  │  ┌────────────────────────┐  │    │
│  │  │LicenseRepository│ │  │ PostgresLicenseRepo    │  │    │
│  │  │  (trait)        │ │  │   (impl)               │  │    │
│  │  └────────────────┘  │  └────────────────────────┘  │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    DATA LAYER                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              PostgreSQL Database                     │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────┐   │    │
│  │  │  licenses   │  │ split_type  │  │   Other    │   │    │
│  │  │   table     │  │   enum      │  │   tables   │   │    │
│  │  └─────────────┘  └─────────────┘  └────────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### 1.4 Key Design Patterns

| Pattern | Implementation |
|---------|----------------|
| **Repository Pattern** | `LicenseRepository` trait in uno-api, implemented in uno-app |
| **Dependency Injection** | `ServiceFactory` holds all services, injected via Actix `web::Data` |
| **Server Functions** | Leptos `#[server]` macros for type-safe client-server calls |
| **State Machine** | Wizard component with discrete stages and transitions |
| **Type-safe SQL** | SQLx compile-time query verification |

---

## 2. Security Analysis

### 2.1 Authentication Mechanisms

#### Admin API (HMAC-SHA256)
```
Request Flow:
1. Client creates payload JSON
2. Client computes HMAC-SHA256(payload, secret_key)
3. Client sends SignedRequest { payload, signature, timestamp }
4. Server validates timestamp (within 5 minutes)
5. Server recomputes HMAC and compares signatures
6. Server processes request if valid
```

**Strengths:**
- Replay attack prevention via timestamp validation
- No credentials transmitted (only signature)
- Constant-time comparison prevents timing attacks

**Weaknesses:**
- Shared secret must be securely distributed
- No request/response encryption (relies on HTTPS)

#### Public API (CSRF Protection)
- Leptos built-in CSRF tokens for form submissions
- Cookie-based session management

### 2.2 Security Controls Matrix

| Control | Status | Notes |
|---------|--------|-------|
| HTTPS | Required | Assumed via deployment |
| HMAC Authentication | ✅ Implemented | Admin routes only |
| CSRF Protection | ✅ Implemented | Leptos server functions |
| Rate Limiting | ✅ Implemented | Configurable middleware |
| Input Validation | ✅ Implemented | Type-safe deserialization |
| SQL Injection | ✅ Protected | SQLx parameterized queries |
| XSS Prevention | ✅ Protected | Leptos escapes by default |
| Audit Logging | ⚠️ Partial | Server logs, no dedicated audit trail |
| Secret Management | ⚠️ Env vars | Consider vault integration |

### 2.3 Security Recommendations

1. **Add audit logging** - Track all admin operations with user/IP/timestamp
2. **Implement secret rotation** - HMAC keys should be rotatable without downtime
3. **Add request signing for responses** - Prevent response tampering
4. **Consider mTLS** - For admin API in production
5. **Add security headers** - CSP, HSTS, X-Frame-Options

---

## 3. Data Flow & Call Flow

### 3.1 License Claim Flow

```
┌──────────┐     ┌──────────┐     ┌──────────────┐     ┌─────────────┐     ┌──────────┐
│  Browser │     │  Leptos  │     │   Service    │     │ Repository  │     │ Database │
│   (UI)   │     │  Server  │     │   Factory    │     │   Layer     │     │  (Postgres)
└────┬─────┘     └────┬─────┘     └──────┬───────┘     └──────┬──────┘     └────┬─────┘
     │                │                   │                    │                 │
     │  1. Select     │                   │                    │                 │
     │  Split Type    │                   │                    │                 │
     │───────────────>│                   │                    │                 │
     │                │                   │                    │                 │
     │  2. call       │                   │                    │                 │
     │  claim_by_     │                   │                    │                 │
     │  split_type()  │                   │                    │                 │
     │───────────────>│                   │                    │                 │
     │                │  3. extract       │                    │                 │
     │                │  ServiceFactory   │                    │                 │
     │                │──────────────────>│                    │                 │
     │                │                   │                    │                 │
     │                │  4. license_      │                    │                 │
     │                │  service.claim_   │                    │                 │
     │                │  by_split_type()  │                    │                 │
     │                │──────────────────>│                    │                 │
     │                │                   │  5. get_first_     │                 │
     │                │                   │  unclaimed()       │                 │
     │                │                   │───────────────────>│                 │
     │                │                   │                    │  6. SELECT      │
     │                │                   │                    │  FROM licenses  │
     │                │                   │                    │  WHERE claimed  │
     │                │                   │                    │  = false        │
     │                │                   │                    │────────────────>│
     │                │                   │                    │                 │
     │                │                   │                    │<────────────────│
     │                │                   │<───────────────────│  7. License     │
     │                │                   │                    │                 │
     │                │                   │  8. claim()        │                 │
     │                │                   │───────────────────>│                 │
     │                │                   │                    │  9. UPDATE      │
     │                │                   │                    │  licenses SET   │
     │                │                   │                    │  claimed=true   │
     │                │                   │                    │────────────────>│
     │                │                   │                    │                 │
     │                │                   │                    │<────────────────│
     │                │                   │<───────────────────│  10. License    │
     │                │<──────────────────│                    │                 │
     │<───────────────│  11. ClaimResponse│                    │                 │
     │                │                   │                    │                 │
```

### 3.2 Admin License Upload Flow

```
┌──────────┐     ┌────────────┐     ┌────────────┐     ┌─────────────┐     ┌──────────┐
│ Postman/ │     │   HMAC     │     │  Admin     │     │ Repository  │     │ Database │
│  Admin   │     │ Middleware │     │  Handler   │     │   Layer     │     │          │
└────┬─────┘     └─────┬──────┘     └─────┬──────┘     └──────┬──────┘     └────┬─────┘
     │                 │                   │                   │                 │
     │ POST /api/v1/   │                   │                   │                 │
     │ admin/licenses  │                   │                   │                 │
     │ SignedRequest   │                   │                   │                 │
     │────────────────>│                   │                   │                 │
     │                 │                   │                   │                 │
     │                 │ Validate HMAC     │                   │                 │
     │                 │ signature &       │                   │                 │
     │                 │ timestamp         │                   │                 │
     │                 │                   │                   │                 │
     │                 │ (if valid)        │                   │                 │
     │                 │──────────────────>│                   │                 │
     │                 │                   │                   │                 │
     │                 │                   │ bulk_create()     │                 │
     │                 │                   │──────────────────>│                 │
     │                 │                   │                   │  INSERT INTO    │
     │                 │                   │                   │  licenses       │
     │                 │                   │                   │────────────────>│
     │                 │                   │                   │                 │
     │                 │                   │                   │<────────────────│
     │                 │                   │<──────────────────│                 │
     │                 │<──────────────────│                   │                 │
     │<────────────────│ { created: 20 }   │                   │                 │
     │                 │                   │                   │                 │
```

### 3.3 Data Models

#### License Table Schema
```sql
CREATE TABLE licenses (
    id UUID PRIMARY KEY,
    lease_code VARCHAR(50) UNIQUE NOT NULL,
    valid_from TIMESTAMPTZ NOT NULL,
    valid_to TIMESTAMPTZ NOT NULL,
    split_type split_type NOT NULL,  -- ENUM: '50:50', '55:45', '60:40'
    claimed BOOLEAN DEFAULT FALSE,
    bound_to_device BOOLEAN DEFAULT FALSE,
    device_id VARCHAR(255),
    claimed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### Key Type Definitions

```rust
// Split types for revenue sharing
pub enum SplitType {
    Split5050,  // User: 50%, Operator: 50%
    Split5545,  // User: 55%, Operator: 45%
    Split6040,  // User: 60%, Operator: 40%
}

// License claim request
pub struct ClaimRequest {
    pub lease_code: String,
    pub device_id: Option<String>,
}

// License claim response
pub struct ClaimResponse {
    pub success: bool,
    pub license: Option<LicenseDto>,
    pub license_id: Option<String>,
    pub license_key: Option<String>,
    pub error: Option<String>,
}
```

---

## 4. Server Interfaces Offered

### 4.1 Public API (Leptos Server Functions)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/GetVariants` | POST | Get available license split options with counts |
| `/api/ClaimBySplitType` | POST | Claim license by selecting a split type |
| `/api/ClaimLicense` | POST | Claim license by specific lease code |
| `/api/GetClaimData` | POST | Get license details by lease code |

### 4.2 REST API (Actix Handlers)

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/api/v1/licenses/variants` | GET | None | List available split options |
| `/api/v1/licenses/claim` | POST | None | Claim a license |
| `/api/v1/admin/licenses` | POST | HMAC | Bulk upload licenses |
| `/api/v1/admin/licenses` | GET | HMAC | List all licenses (with filters) |
| `/api/v1/admin/licenses/{id}` | GET | HMAC | Get specific license |
| `/api/v1/admin/licenses/{id}` | DELETE | HMAC | Delete a license |

### 4.3 Request/Response Examples

#### Get Variants Response
```json
[
  {
    "id": 3,
    "user_share_percentage": 60,
    "operator_share_percentage": 40,
    "lease_duration_months": 12,
    "total_quantity": 100,
    "claimed_count": 45,
    "display_name": "60:40",
    "is_featured": true
  }
]
```

#### Bulk Upload Request (SignedRequest)
```json
{
  "payload": {
    "licenses": [
      {
        "lease_code": "LICENSE-001",
        "valid_from": "2024-01-01T00:00:00Z",
        "valid_to": "2025-01-01T00:00:00Z",
        "split_type": "60:40"
      }
    ]
  },
  "signature": "hmac_sha256_hex_signature",
  "timestamp": 1704067200
}
```

---

## 5. Clients Called

### 5.1 External Dependencies

| Service | Purpose | Integration |
|---------|---------|-------------|
| **PostgreSQL** | Primary data store | SQLx async driver |
| **MaxMind GeoLite2** | IP geolocation | Local database file |
| **uno-api crate** | Shared traits & models | Cargo workspace dependency |

### 5.2 uno-api Integration

The `uno-api` crate provides:
- `LicenseRepository` trait definition
- Shared model types (`License`, `SplitType`, `LicenseSummary`)
- The trait is implemented in uno-app's `license_repository.rs`

```rust
// uno-api defines the trait
pub trait LicenseRepository {
    async fn get_by_lease_code(&self, code: &str) -> Result<Option<License>>;
    async fn claim(&self, id: Uuid, device_id: Option<String>) -> Result<License>;
    async fn get_first_unclaimed(&self, split: SplitType) -> Result<Option<License>>;
    async fn get_summary(&self) -> Result<LicenseSummary>;
    // ...
}

// uno-app implements it
impl LicenseRepository for PostgresLicenseRepository {
    // SQLx queries against PostgreSQL
}
```

---

## 6. Project Score & Assessment

### Overall Score: **B+**

### 6.1 Strengths

| Category | Score | Details |
|----------|-------|---------|
| **Type Safety** | A | Full-stack Rust with compile-time SQL checking |
| **Architecture** | A- | Clean separation of concerns, repository pattern |
| **Security** | B+ | HMAC auth, CSRF, rate limiting, but missing audit logs |
| **Code Quality** | B+ | Well-organized, good naming, consistent patterns |
| **Internationalization** | A | 9 languages with PHF compile-time maps |
| **API Design** | B | Dual API (Leptos + REST), some redundancy |
| **Error Handling** | B | Consistent error types, could use more context |
| **Documentation** | C+ | Code comments present, external docs minimal |
| **Testing** | C | Limited test coverage visible |

### 6.2 Weaknesses

1. **Limited Test Coverage**
   - No visible unit tests for services
   - No integration tests for API endpoints
   - Missing property-based tests for core logic

2. **Documentation Gaps**
   - No API documentation (OpenAPI/Swagger)
   - Missing architecture decision records (ADRs)
   - No deployment documentation

3. **Operational Concerns**
   - No structured audit logging
   - Missing health check endpoints
   - No metrics/observability setup

4. **Code Duplication**
   - Type conversion functions between uno-api and uno-app types
   - Similar error handling patterns repeated

5. **Configuration Management**
   - Environment variables for secrets
   - No configuration validation at startup

### 6.3 Improvement Recommendations

#### High Priority
1. **Add comprehensive test suite** - Unit tests for services, integration tests for APIs
2. **Implement audit logging** - Track all mutations with user/IP/timestamp
3. **Add OpenAPI documentation** - Auto-generate from handlers
4. **Health check endpoints** - `/health` and `/ready` for orchestration

#### Medium Priority
5. **Unify type systems** - Consider single source of truth for models
6. **Add structured logging** - Use tracing with correlation IDs
7. **Implement metrics** - Prometheus metrics for observability
8. **Configuration validation** - Fail fast on invalid config

#### Low Priority
9. **Add rate limiting per-route** - Different limits for different endpoints
10. **Consider GraphQL** - Reduce API surface, better client experience
11. **Add caching layer** - Redis for frequently accessed data
12. **Implement webhook support** - Notify external systems of claims

---

## 7. Summary

uno-app is a well-architected Rust application demonstrating modern full-stack development practices. The use of Leptos for seamless SSR/CSR, combined with Actix-web's performance and SQLx's compile-time safety, creates a robust foundation.

The security model is solid for its intended use case, though production deployment would benefit from additional audit logging and observability features. The codebase is maintainable and follows consistent patterns, making it straightforward for new developers to understand.

**Key Takeaways:**
- Strong type safety from database to UI
- Clean architectural boundaries
- Security-conscious design
- Room for improvement in testing and documentation
- Production-ready with recommended enhancements

---

*Generated: 2026-05-24*
*Analyzed by: Claude Code*
