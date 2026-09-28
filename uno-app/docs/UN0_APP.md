# UNO App - Architecture Overview

A comprehensive full-stack Leptos web application for license distribution with SSR/CSR hybrid rendering.

---

## Project Statistics

- **Total Rust Files**: 189
- **Languages Supported**: 9
- **Framework**: Leptos (Rust reactive web framework)
- **Backend**: Actix-web
- **Database**: PostgreSQL (via sqlx)

---

## Directory Structure

```
src/
├── api/                    # Server functions (9 files) - RPC bridge
├── app.rs                  # Root App component & routing
├── components/             # UI components (68 files)
│   ├── chatbot/           # Chat support widget
│   ├── common/            # Shared UI primitives (20 files)
│   ├── faq/               # FAQ section components
│   ├── layout/            # Header, footer, nav
│   ├── license/           # License-related UI
│   ├── sections/          # CMS-driven page sections
│   ├── stats/             # Analytics/stats dashboard
│   ├── tasks/             # Task management UI
│   └── wizard/            # License claim wizard (14 files)
├── config/                # App configuration (SSR only)
├── features/              # Feature-specific modules
│   └── license/           # License feature components
├── hooks/                 # Custom Leptos hooks (8 files)
├── lib.rs                 # Library root & WASM entry point
├── locales/               # i18n translations (9 languages)
├── main.rs                # SSR/CSR entry points
├── routes/                # Page route components (9 files)
├── server/                # Backend-only (SSR feature)
│   ├── adapters/         # Data adapters
│   ├── app/              # ServiceFactory for DI
│   ├── db/               # PostgreSQL connection pool
│   ├── extractors/       # Request extractors
│   ├── geoip.rs          # GeoIP service
│   ├── handlers/         # HTTP request handlers (14 files)
│   ├── middleware/       # Request middleware (11 files)
│   ├── repositories/     # Database access layer (14 files)
│   ├── scheduler/        # Background jobs
│   ├── secrets.rs        # Secret management
│   ├── services/         # Business logic layer (10 files)
│   └── utils/            # Utility functions
└── types/                 # Shared types (14 files)
```

---

## Component Architecture

### Component Hierarchy

```
App (app.rs)
├── ThemeProvider (ember-fx)
├── ToastProvider (ember-fx)
├── AppRouter
│   ├── Header
│   ├── main.app-container
│   │   ├── Routes
│   │   │   ├── HomePage
│   │   │   │   ├── HeroSection (CMS-driven)
│   │   │   │   ├── HowItWorksSection (CMS-driven)
│   │   │   │   ├── EarningsSection (CMS-driven)
│   │   │   │   └── TestimonialsSection (CMS-driven)
│   │   │   ├── ClaimPage (/claim/:id)
│   │   │   ├── TasksPage
│   │   │   │   └── TaskList -> TaskCard
│   │   │   ├── GuidesPage
│   │   │   ├── FaqPage
│   │   │   │   ├── SearchBar
│   │   │   │   └── CategoryTabs + FaqItems
│   │   │   ├── ContactPage
│   │   │   ├── ReferralsPage
│   │   │   ├── PreviewPage
│   │   │   └── DebugPage
│   │   └── Footer
│   ├── ChatWidget
│   ├── LocalePopup
│   └── ClaimWizard (Modal Overlay)
│       ├── ProgressBar
│       ├── ReviewStage
│       │   └── VariantCard
│       ├── ClaimStage
│       │   └── LicenseReveal
│       ├── DownloadStage
│       │   └── DownloadButtons
│       ├── SignupStage
│       ├── ActivateStage
│       └── WhatNextStage
```

### Key Component Design Patterns

- **Modal Wizards**: ClaimWizard uses reactive state management with RwSignals for stage navigation, form validation, and async operations
- **CMS-Driven Sections**: Home page sections are fetched from schema-driven CMS and rendered dynamically based on section type
- **Responsive Cards**: License variant cards, task cards use consistent styling with conditional features
- **Accessibility**: ARIA labels, semantic HTML, keyboard navigation in wizard stages

---

## State Management

### Leptos Signals (Reactive Primitives)

| Signal Type | Purpose |
|-------------|---------|
| `RwSignal<T>` | Reactive read-write signals for mutable state |
| `Memo<T>` | Derived signals (computed values) |
| `Resource<K,T>` | Server function wrappers with async/loading states |
| `Effect` | Side effect subscriptions |

### Context Providers

1. **Locale Context** (`use_locale.rs`)
   - Current language/locale
   - Locale from geo-IP detection
   - Bilingual country support

2. **Theme Context** (`use_theme.rs`)
   - Current theme (dark-blue, light, etc.)
   - ember-fx integration for CSS injection
   - Theme persistence

3. **Wizard Context** (`wizard/state.rs`)
   - License claim wizard global state
   - Multi-stage navigation
   - Referral code validation
   - License variant selection

4. **ember-fx Toast Context**
   - Global notifications
   - Toast positioning & queue

### Wizard State Structure

```rust
ClaimWizardState {
    stage: RwSignal<ClaimWizardStage>,
    selected_variant: RwSignal<Option<LicenseVariant>>,
    lease_code: RwSignal<String>,
    terms_accepted: RwSignal<bool>,
    claimed_license_id: RwSignal<Option<String>>,
    claimed_license_key: RwSignal<Option<String>>,
    is_claiming: RwSignal<bool>,
    is_confirming: RwSignal<bool>,
    is_confirmed: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    is_open: RwSignal<bool>,
    variants: RwSignal<Vec<LicenseVariant>>,
    has_referral: RwSignal<bool>,
    referral_code: RwSignal<String>,
    referral_valid: RwSignal<Option<bool>>,
    referral_checking: RwSignal<bool>,
    global_availability: RwSignal<AvailabilityStatus>,
    is_loading_variants: RwSignal<bool>,
}
```

---

## Routing

### Client-Side Routes (leptos_router)

| Path | Component | Purpose |
|------|-----------|---------|
| `/` | HomePage | Landing page with CMS content |
| `/claim/:id` | ClaimPage | License claim by ID |
| `/tasks` | TasksPage | Task list & management |
| `/guides` | GuidesPage | How-to guides |
| `/faq` | FaqPage | FAQ search & browse |
| `/contact` | ContactPage | Contact form |
| `/referrals` | ReferralsPage | Referral program info |
| `/preview` | PreviewPage | CMS preview mode |
| `/debug` | DebugPage | Debug utilities |
| `/*` | NotFound | 404 fallback |

### Server Routes (Actix-web)

**Public API** (`/api/v1`)
- `/licenses/variants` - Get available license splits
- `/licenses/claim` - POST to claim license
- `/faq` - Get all FAQs
- `/faq/search` - Search FAQs
- `/faq/{id}` - Get FAQ by ID
- `/stats/visitors` - Visitor statistics
- `/contents/{type}` - CMS content by type
- `/schemas` - Get CMS schemas
- `/items/{schema_id}` - Get schema items

**Admin API** (`/api/v1/admin`)
- `/licenses` - POST to publish licenses
- `/licenses/import` - CSV bulk import
- `/licenses/search` - Search licenses
- `/licenses/claimed` - Get claimed licenses
- `/contents/*` - CMS admin operations
- `/reviews/*` - Content review workflow
- `/audit-logs` - Audit trail
- `/roles` - RBAC management
- `/items/*` - Schema-driven CMS admin

**Preview** (`/api/v1/preview/{token}`)
- Unauthenticated preview of draft content

---

## API/Backend Integration

### Leptos Server Functions (RPC-style)

Located in `src/api/`, these functions compile to both:
- **Server side**: Actual implementations with database access
- **Client side**: Remote procedure calls via `/api` endpoint

```rust
// License operations
get_variants()                          // Get available split options
claim_by_split_type(split, device_id)   // Auto-claim license
claim_license(lease_code, device_id)    // Claim by code
get_claim_data(lease_code)              // Get claim page data

// FAQ operations
get_faqs(query, category, locale)       // Get FAQs with filtering
get_faq_preview(token, locale)          // Preview draft content

// Content operations
get_home_with_preview(token, locale)    // Get home page sections
get_guides(locale)                      // Get guides
get_tasks(locale)                       // Get tasks

// Stats
get_stats(period)                       // Get statistics

// Referrals
validate_referral_code(code)            // Check referral validity

// Chat
send_chat_message(message, context)     // Chat completions
```

### Backend Architecture

```
Handler (Actix-web route)
  ↓
Service (business logic)
  ↓
Repository (database)
  ↓
PostgreSQL
```

### Authentication

- **HMAC-based** for admin endpoints using `uno-api` ClientRegistry
- **No auth required** for public endpoints
- **Preview tokens** signed with PREVIEW_SECRET_KEY for draft content

### Error Handling

Unified `AppError` enum with HTTP status code mapping:
- BadRequest → 400
- Unauthorized → 401
- Forbidden → 403
- NotFound → 404
- Conflict → 409
- Database errors → 500
- ValidationError → 400

---

## Shared Types/Models

### License Domain (`types/license.rs`)

```rust
SplitType              // 50:50, 55:45, 60:40 revenue splits
License                // Full license database model
LicenseDto             // DTO for API responses
LicenseVariant         // UI-friendly variant with availability
AvailabilityStatus     // Available|AllClaimed|AllExpired|NoneInSystem
ClaimRequest           // POST body for claiming
ClaimResponse          // Claim result with license_id & license_key
ClaimPageData          // Data for claim page display
LicenseSummary         // Statistics summary
```

### Content Domain (`types/content.rs`)

```rust
ContentStatus         // Draft|PendingReview|Approved|Published|Archived
ContentType           // Task|Guide|Faq|Error
PageContent           // CMS content page
PageContentResponse   // API response format
```

### FAQ Domain (`types/faq.rs`)

```rust
FaqItem               // DB row with translations
FaqItemResponse       // API response (localized)
FaqCategory           // Category definition
FaqListResponse       // Paginated list
FaqSearchParams       // Query filters
```

### Schema Domain (`types/schema.rs`)

```rust
Schema                // CMS schema definition
SchemaField           // Field configuration (type, validation)
ContentItem           // Content instance of schema
ItemTranslation       // Multi-language content
```

### Other Domains

- **Stats** (`types/stats.rs`) - NetworkStats, VisitorStats, CountryStats
- **Error** (`types/error.rs`) - AppError, ErrorResponse
- **RBAC** (`types/rbac.rs`) - Role, Permission, UserPermissions
- **Audit** (`types/audit.rs`) - AuditLog, AuditEntry

---

## Custom Hooks

| Hook | File | Purpose |
|------|------|---------|
| `use_api()` | `use_api.rs` | Wraps server functions with loading/error states |
| `use_api_auto()` | `use_api.rs` | Auto-executes server functions on mount |
| `use_clipboard()` | `use_clipboard.rs` | Copy-to-clipboard with browser API |
| `use_debounce()` | `use_debounce.rs` | Debounce signal updates |
| `use_throttle()` | `use_debounce.rs` | Throttle rapid updates |
| `use_lazy_load()` | `use_lazy_load.rs` | Lazy load images with Intersection Observer |
| `use_locale()` | `use_locale.rs` | Access/set current locale |
| `use_theme()` | `use_theme.rs` | Access/toggle theme |
| `use_validation()` | `use_validation.rs` | Form validation with rules |

---

## Server Layer

### Services (`src/server/services/`)

| Service | Responsibility |
|---------|----------------|
| LicenseService | License claiming, availability checks |
| FaqServiceImpl | FAQ retrieval & searching |
| StatsServiceImpl | Statistics aggregation |
| ContentServiceImpl | Legacy CMS operations |
| ContentItemServiceImpl | Schema-driven CMS items |
| SchemaServiceImpl | Schema management |
| AuditServiceImpl | Audit log persistence |
| RbacServiceImpl | Permission checking |
| ChatbotService | LLM integration |

### Repositories (`src/server/repositories/`)

| Repository | Methods |
|------------|---------|
| LicenseRepository | claim, get_available, get_by_code |
| FaqRepository | find_by_id, search, list |
| StatsRepository | get_visitor_stats, get_country_stats |
| ContentRepository | create, update, publish, archive |
| ClaimRepository | create, get, update, set_referral |
| AuditRepository | insert, list, get_history |
| RbacRepository | get_permissions, assign_role |
| SchemaRepository | create, get, list, delete |
| ContentItemRepository | create, update, publish, get_versions |

### Middleware (`src/server/middleware/`)

| Middleware | Purpose |
|------------|---------|
| RequestLogger | Request/response logging |
| VisitorTracker | Track unique visitors |
| AuthMiddleware | HMAC signature validation |
| RbacMiddleware | Permission-based access control |
| CsrfMiddleware | CSRF token validation |
| CorsMiddleware | CORS policy enforcement |
| CompressionMiddleware | Gzip response compression |
| RateLimitMiddleware | Request throttling |
| AuditMiddleware | Track admin actions |
| SecurityMiddleware | Security headers |
| ValidationMiddleware | Input validation |

---

## Entry Points

### SSR Entry Point (`main.rs`)

```
main() [actix_web::main macro]
  ↓
Initialize logging
  ↓
Load environment variables (dotenvy)
  ↓
Register server functions
  ↓
Connect to PostgreSQL (optional for UI-only mode)
  ↓
Create ServiceFactory (DI container)
  ↓
Start background content scheduler
  ↓
Configure Actix-web routes
  ↓
Add middleware (logging, visitor tracking)
  ↓
Bind to listen address & run
```

### CSR/WASM Entry Point (`lib.rs`)

```rust
#[wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
```

### App Initialization (`app.rs`)

```
App component
  ↓
Provide meta context (MetaTags)
  ↓
Provide locale context (use_locale hook)
  ↓
Provide theme context (use_theme hook)
  ↓
Wrap with ember-fx: ThemeProvider + ToastProvider
  ↓
Render AppRouter with routes
```

---

## Feature Flags

| Flag | Purpose |
|------|---------|
| `ssr` | Server-side rendering (Actix-web) |
| `csr` | Client-side rendering (pure WASM) |
| `hydrate` | Hybrid mode (SSR + WASM hydration) |

Conditional compilation:
```rust
#[cfg(feature = "ssr")]     // Server-only code
#[cfg(feature = "hydrate")] // Client-side code
```

---

## Localization

### Supported Languages (9)

| Code | Language |
|------|----------|
| `en` | English |
| `es` | Spanish |
| `tl` | Tagalog/Filipino |
| `hi` | Hindi |
| `sw` | Swahili |
| `pt` | Portuguese |
| `fr` | French |
| `ar` | Arabic |
| `id` | Indonesian |

### Bilingual Countries

Special handling for countries with two official languages:
- **India**: English + Hindi
- **Kenya**: English + Swahili
- **Philippines**: English + Tagalog
- **Tanzania**: Swahili + English

### Static PHF Maps

Each language file exports compile-time translation lookup:

```rust
pub static TRANSLATIONS: phf::Map<&str, &str> = phf_map! {
    "nav.home" => "Home",
    "nav.faq" => "FAQ",
    // ...
};
```

### Internationalization (`intl.rs`)

- `format_number()` - Number formatting
- `format_currency()` - Currency formatting
- `format_date()` - Date localization
- `pluralize()` - Plural category selection

### Runtime Usage

```rust
let locale = use_locale();  // Current locale
let text = t("nav.home");   // Translate key
set_locale("es");           // Change language
```

---

## Data Flow Examples

### License Claiming Flow

```
1. User enters lease code in ClaimWizard
2. ClaimWizard calls claim_by_split_type() server function
3. Server function extracts ServiceFactory from request
4. Calls factory.license_service.claim_by_split_type()
5. Service calls factory.license_repository.claim_license()
6. Repository executes SQL UPDATE on licenses table
7. Returns License object
8. Service converts to ClaimResponse
9. Server function returns Result<ClaimResponse>
10. Client updates wizard state with claimed_license_key
11. Wizard advances to Claim stage, displays key
12. User can copy key or confirm with referral code
```

### Home Page CMS Content Flow

```
1. HomePage component loads
2. Calls get_home_with_preview() server function
3. Server checks preview_token if in preview mode
4. Fetches content_items from schema-driven CMS
5. Filters by schema_id="home" and display_order
6. Returns HomeSection array
7. Client renders appropriate section component
8. Each section displays CMS-managed content
```

### FAQ Search Flow

```
1. FaqPage component renders with SearchBar
2. User types query → use_debounce hook
3. Triggers get_faqs() server function
4. Server filters faq table by:
   - Query text (full-text search)
   - Category (if selected)
   - Locale (from request or param)
   - is_featured flag
5. Returns FaqListResponse with paginated items
6. Client renders CategoryTabs + FaqItems
```

---

## Database Schema

Key tables integrated via sqlx:

| Table | Purpose |
|-------|---------|
| `licenses` | License records (id, lease_code, split_type, valid_from/to, claimed, device_id) |
| `claims` | License claim records (license_id, referral_id, device_id) |
| `faqs` | FAQ items with translations (JSONB) |
| `page_contents` | Legacy CMS content |
| `content_items` | Schema-driven CMS items (JSONB data & translations) |
| `schemas` | CMS schema definitions (JSONB fields) |
| `audit_logs` | Action audit trail (old/new values as JSONB) |
| `visitors` | Visitor tracking |
| `referrals` | Referral codes |

---

## Design System

### ember-fx Integration

- **Design System**: Ant (Alibaba Ant Design)
- **Theme Provider**: Injects CSS variables for theming
- **Theme Persistence**: Saves to localStorage
- **Initial Theme**: `dark-blue` (UNO signature theme)

### Toast System

- Placement: TopRight
- Global queue management
- Used via `use_toast()` hook

---

## Performance Optimizations

### Lazy Loading
- Images use `LazyImage` component with Intersection Observer
- Translations use optional `lazy_loader` module for code splitting
- Routes are lazy-loaded via Leptos Router

### Memoization
- Computed signals via `Memo<T>` prevent unnecessary recalculation
- Effects subscribed only to changed dependencies

### Caching
- Wizard state persisted in context (app-level)
- Variant data fetched once on app init
- Locale/theme cached in localStorage

### Compression
- CompressionMiddleware enables gzip for responses
- Static assets served from `/pkg` and `/assets`

---

## Deployment

### Docker Support
- `Dockerfile` - Production build
- `Dockerfile.local` - Development with hot reload

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `DATABASE_URL` | PostgreSQL connection string |
| `RUST_ENV` | Environment mode (development/production) |
| `PREVIEW_SECRET_KEY` | CMS preview token signing key |

### Build Artifacts
- WASM binary compiled to `/pkg`
- Static assets in `/assets` and `/style`

---

## Library Dependencies

### Frontend
- `leptos` - Reactive framework
- `leptos_router` - Client-side routing
- `leptos_meta` - SEO meta tags
- `leptos_actix` - SSR bridge
- `ember-fx-components` - Design system & components
- `serde/serde_json` - Serialization
- `chrono` - Date/time handling
- `phf` - Perfect hash functions for translations
- `wasm_bindgen` - WASM integration

### Backend
- `actix-web` - Web framework
- `actix-files` - Static file serving
- `sqlx` - SQL toolkit with PostgreSQL
- `uno-api` - License management API
- `tracing` - Structured logging
- `uuid` - UUID generation
- `dotenvy` - Environment loading

### Both
- `thiserror` - Error handling
- `url` - URL parsing
