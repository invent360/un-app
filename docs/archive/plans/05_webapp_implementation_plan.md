# UNO Web Application Implementation Plan

## Following CLAUDE.md Workflow Principles

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Database Schema Design](#2-database-schema-design)
3. [Backend API Design](#3-backend-api-design)
4. [Frontend Architecture](#4-frontend-architecture)
5. [License Claim Wizard](#5-license-claim-wizard)
6. [Public Statistics Dashboard](#6-public-statistics-dashboard)
7. [Support & FAQ System](#7-support--faq-system)
8. [Task Categories Page](#8-task-categories-page)
9. [Localization System](#9-localization-system)
10. [Infrastructure & Deployment](#10-infrastructure--deployment)
11. [Implementation Phases](#11-implementation-phases)
12. [Verification Gates](#12-verification-gates)

---

## 1. Project Overview

### Core Principles (from CLAUDE.md)

| Principle | Application |
|-----------|-------------|
| **Plan Mode Default** | This document serves as the detailed spec before implementation |
| **Simplicity First** | Minimal features for MVP, expand later |
| **Verification Before Done** | Each phase has validation gates |
| **Demand Elegance** | Clean architecture, no hacky solutions |

### Design Decisions Applied

| Conflict | Resolution Applied |
|----------|-------------------|
| **Community Strategy** | No Discord/WhatsApp/Telegram - Website FAQ + In-app messaging + Direct contact only |
| **User Data Collection** | Anonymous until license claim - No email/phone collection |
| **License Database** | Import from Unetwork admin dashboard CSV exports |
| **Support Chatbot** | Custom in-built module (no third-party Discord bots) |

### Tech Stack

```
+-------------------------------------------------------------------+
|                       TECHNOLOGY STACK                            |
+-------------------------------------------------------------------+
|                                                                   |
|   FRONTEND:                                                       |
|   - Framework: Leptos (Rust WASM)                                 |
|   - Styling: Tailwind CSS                                         |
|   - Charts: Plotters / Charming (Rust-native)                     |
|   - i18n: leptos-i18n                                             |
|                                                                   |
|   BACKEND:                                                        |
|   - Framework: Actix-web (Rust)                                   |
|   - Database: PostgreSQL + SQLx                                   |
|   - Cache: Redis (for session/rate limiting)                      |
|   - File Storage: S3-compatible (for CSV imports)                 |
|                                                                   |
|   INFRASTRUCTURE:                                                 |
|   - Container: Docker + Kubernetes                                |
|   - CDN: Cloudflare                                               |
|   - Hosting: GKE / EKS / DigitalOcean K8s                         |
|   - Analytics: Custom (privacy-first)                             |
|                                                                   |
+-------------------------------------------------------------------+
```

### UI Component Library & Theming

The application reuses the **ember-fx** component library with multi-theme support.

**Component Source:** `/Users/admin/Dev-x/polkanight/ember/ember-fx/crates`
**Theme Presets:** `/Users/admin/Dev-x/polkanight/ember/ember-fx/crates/styles/themes/presets/ant/`

#### Available Themes

| Theme | Source | Color Scheme | Description |
|-------|--------|--------------|-------------|
| `unetwork` | Custom | Dark | Unetwork brand (teal/cyan accent) - **Default** |
| `dark` | Ant Design | Dark | Standard dark theme |
| `light` | Ant Design | Light | Standard light theme |
| `glass` | Ant Design | Dark | Glass morphism effects |

#### Theme Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                         THEME SYSTEM ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐         │
│   │  Theme Presets  │    │  ThemeRegistry  │    │  ThemeContext   │         │
│   │   (JSON files)  │───►│   (Rust API)    │───►│  (Leptos Signal)│         │
│   └─────────────────┘    └─────────────────┘    └────────┬────────┘         │
│                                                          │                  │
│   Themes:                                                │                  │
│   • unetwork.json (custom)                               ▼                  │
│   • dark.json (ant)                          ┌─────────────────────┐        │
│   • light.json (ant)                         │   CSS Variables     │        │
│   • glass.json (ant)                         │   --fx-color-*      │        │
│                                              └─────────────────────┘        │
│                                                          │                  │
│                                                          ▼                  │
│                                              ┌─────────────────────┐        │
│                                              │   UI Components     │        │
│                                              │   (ember-fx)        │        │
│                                              └─────────────────────┘        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Unetwork Theme Definition

Create custom theme at: `assets/themes/unetwork.json`

```json
{
  "name": "unetwork",
  "displayName": "Unetwork",
  "colorScheme": "dark",
  "designSystem": "ant",
  "default": true,
  "colors": {
    "primary": "#00D4AA",
    "primaryBg": "#0A0E17",
    "primaryHover": "#00E5B8",
    "primaryActive": "#00C49A",
    "primaryBorder": "rgba(0, 212, 170, 0.3)",

    "secondary": "#6366F1",
    "secondaryHover": "#818CF8",

    "accent": "#00D4AA",
    "accentHover": "#00E5B8",

    "success": "#22C55E",
    "successBg": "rgba(34, 197, 94, 0.15)",
    "warning": "#F59E0B",
    "warningBg": "rgba(245, 158, 11, 0.15)",
    "error": "#EF4444",
    "errorBg": "rgba(239, 68, 68, 0.15)",
    "info": "#3B82F6",
    "infoBg": "rgba(59, 130, 246, 0.15)",

    "text": "#FFFFFF",
    "textSecondary": "#94A3B8",
    "textTertiary": "#64748B",
    "textDisabled": "#475569",

    "border": "#1E293B",
    "borderSecondary": "#334155",

    "bgContainer": "#0F1420",
    "bgElevated": "#1A2235",
    "bgSpotlight": "#151B2B",
    "bgMask": "rgba(0, 0, 0, 0.6)"
  },
  "radius": {
    "xs": "4px",
    "sm": "6px",
    "md": "12px",
    "lg": "20px",
    "xl": "28px",
    "full": "9999px"
  },
  "spacing": {
    "xxs": "4px",
    "xs": "8px",
    "sm": "12px",
    "md": "16px",
    "lg": "24px",
    "xl": "32px",
    "xxl": "48px"
  },
  "effects": {
    "shadowSm": "0 1px 2px rgba(0, 0, 0, 0.3)",
    "shadowMd": "0 4px 12px rgba(0, 0, 0, 0.4)",
    "shadowLg": "0 8px 24px rgba(0, 0, 0, 0.5)",
    "shadowGlow": "0 0 30px rgba(0, 212, 170, 0.3)",
    "blur": "12px",
    "blurLg": "24px"
  },
  "motion": {
    "fast": "150ms",
    "normal": "250ms",
    "slow": "350ms",
    "easeOut": "cubic-bezier(0.215, 0.61, 0.355, 1)",
    "easeInOut": "cubic-bezier(0.4, 0, 0.2, 1)"
  },
  "font": {
    "family": "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    "familyMono": "'JetBrains Mono', 'Fira Code', monospace",
    "size": "14px",
    "sizeSm": "12px",
    "sizeLg": "16px",
    "sizeXl": "20px",
    "size2xl": "24px",
    "size3xl": "32px"
  }
}
```

#### Theme Switching Implementation

```rust
// src/theme/mod.rs

use leptos::*;
use ember_fx_styles::ThemeRegistry;

/// Available themes for the application
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AppTheme {
    #[default]
    Unetwork,  // Custom dark theme (default)
    Dark,      // Ant Design dark
    Light,     // Ant Design light
    Glass,     // Glass morphism
}

impl AppTheme {
    pub fn name(&self) -> &'static str {
        match self {
            AppTheme::Unetwork => "unetwork",
            AppTheme::Dark => "dark",
            AppTheme::Light => "light",
            AppTheme::Glass => "glass",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppTheme::Unetwork => "Unetwork",
            AppTheme::Dark => "Dark",
            AppTheme::Light => "Light",
            AppTheme::Glass => "Glass",
        }
    }

    pub fn is_dark(&self) -> bool {
        !matches!(self, AppTheme::Light)
    }

    pub fn all() -> &'static [AppTheme] {
        &[AppTheme::Unetwork, AppTheme::Dark, AppTheme::Light, AppTheme::Glass]
    }
}

/// Theme context provider
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Load saved theme from localStorage or use default
    let initial_theme = use_context::<AppTheme>()
        .unwrap_or_default();

    let (theme, set_theme) = create_signal(initial_theme);

    // Provide theme context
    provide_context(theme);
    provide_context(set_theme);

    // Effect to update CSS when theme changes
    create_effect(move |_| {
        let current_theme = theme.get();

        // Load theme CSS from registry
        if let Some(css) = ThemeRegistry::load_theme_css("ant", current_theme.name()) {
            inject_theme_css(&css);
        }

        // Update <html> data attribute for CSS selectors
        if let Some(document) = document() {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", current_theme.name());
                let _ = html.set_attribute(
                    "data-color-scheme",
                    if current_theme.is_dark() { "dark" } else { "light" }
                );
            }
        }

        // Persist to localStorage
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("uno-theme", current_theme.name());
        }
    });

    view! {
        {children()}
    }
}

/// Hook to access current theme
pub fn use_theme() -> (ReadSignal<AppTheme>, WriteSignal<AppTheme>) {
    let theme = expect_context::<ReadSignal<AppTheme>>();
    let set_theme = expect_context::<WriteSignal<AppTheme>>();
    (theme, set_theme)
}

/// Inject theme CSS into document head
fn inject_theme_css(css: &str) {
    if let Some(document) = document() {
        // Remove existing theme style
        if let Some(existing) = document.get_element_by_id("uno-theme-css") {
            existing.remove();
        }

        // Create new style element
        if let Ok(style) = document.create_element("style") {
            let _ = style.set_attribute("id", "uno-theme-css");
            style.set_text_content(Some(css));

            if let Some(head) = document.head() {
                let _ = head.append_child(&style);
            }
        }
    }
}
```

#### Theme Switcher Component

```rust
// src/components/common/theme_switcher.rs

use leptos::*;
use crate::theme::{AppTheme, use_theme};

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    view! {
        <div class="fx-theme-switcher">
            <For
                each=move || AppTheme::all().iter().copied()
                key=|t| t.name()
                children=move |t| {
                    let is_active = move || theme.get() == t;
                    view! {
                        <button
                            class="fx-theme-option"
                            class:active=is_active
                            on:click=move |_| set_theme.set(t)
                            title=t.display_name()
                        >
                            <span class="fx-theme-icon">
                                {match t {
                                    AppTheme::Unetwork => "🌊",
                                    AppTheme::Dark => "🌙",
                                    AppTheme::Light => "☀️",
                                    AppTheme::Glass => "✨",
                                }}
                            </span>
                            <span class="fx-theme-label">{t.display_name()}</span>
                        </button>
                    }
                }
            />
        </div>
    }
}

/// Compact theme toggle (light/dark only)
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = use_theme();

    let toggle = move |_| {
        set_theme.update(|t| {
            *t = if t.is_dark() { AppTheme::Light } else { AppTheme::Unetwork }
        });
    };

    view! {
        <button
            class="fx-btn-ant fx-btn-ant-icon"
            on:click=toggle
            title="Toggle theme"
        >
            <Show
                when=move || theme.get().is_dark()
                fallback=|| view! { <SunIcon /> }
            >
                <MoonIcon />
            </Show>
        </button>
    }
}
```

#### Ember-FX Component Usage

```rust
// src/components/license/variant_card.rs

use leptos::*;
use ember_fx_components::{
    Card, CardHeader, CardContent, CardFooter,
    Button, ButtonVariant,
    Progress, ProgressVariant,
    Badge, BadgeVariant,
    Typography, TypographyVariant,
};

#[component]
pub fn VariantCard(
    #[prop(into)] variant: LicenseVariant,
    #[prop(into)] on_claim: Callback<i32>,
) -> impl IntoView {
    let progress = move || {
        (variant.claimed_count as f64 / variant.total_quantity as f64) * 100.0
    };

    let remaining = move || variant.total_quantity - variant.claimed_count;

    view! {
        <Card elevated=true>
            <CardHeader>
                <div class="flex items-center justify-between">
                    <Typography variant=TypographyVariant::H4>
                        {format!("{}:{} Split", variant.user_share, variant.operator_share)}
                    </Typography>

                    <Show when=move || variant.is_featured>
                        <Badge variant=BadgeVariant::Success>
                            "Special Offer"
                        </Badge>
                    </Show>
                </div>
            </CardHeader>

            <CardContent>
                <Typography variant=TypographyVariant::Body color="secondary">
                    {format!("You keep {}% of earnings", variant.user_share)}
                </Typography>

                <div class="mt-4">
                    <Progress
                        value=progress()
                        variant=ProgressVariant::Primary
                        show_label=true
                    />
                    <Typography variant=TypographyVariant::Caption class="mt-1">
                        {format!("{}/{} claimed • {} remaining",
                            variant.claimed_count,
                            variant.total_quantity,
                            remaining()
                        )}
                    </Typography>
                </div>

                <div class="mt-4 space-y-2">
                    <div class="flex items-center gap-2">
                        <CalendarIcon />
                        <span>{format!("{} month lease", variant.lease_duration_months)}</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <ChartIcon />
                        <span>{format!("${:.2} - ${:.2}/month",
                            variant.min_monthly_earnings,
                            variant.max_monthly_earnings
                        )}</span>
                    </div>
                </div>
            </CardContent>

            <CardFooter>
                <Button
                    variant=ButtonVariant::Primary
                    full_width=true
                    on_click=move |_| on_claim.call(variant.id)
                >
                    "Claim This License"
                </Button>
            </CardFooter>
        </Card>
    }
}
```

#### Project Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Ember-FX component library
ember-fx-components = { path = "../Dev-x/polkanight/ember/ember-fx/crates/components" }
ember-fx-styles = { path = "../Dev-x/polkanight/ember/ember-fx/crates/styles", features = ["ant"] }
ember-fx-core = { path = "../Dev-x/polkanight/ember/ember-fx/crates/core" }

# Or via git (for deployment)
# ember-fx-components = { git = "https://github.com/polkanight/ember-fx", branch = "main" }
```

#### Updated Project Structure

```text
uno-app/src/
├── theme/                         # Theme system
│   ├── mod.rs                     # ThemeProvider, use_theme hook
│   ├── switcher.rs                # ThemeSwitcher component
│   └── presets/                   # Custom theme definitions
│       └── unetwork.json
│
├── components/                    # App-specific components
│   ├── layout/
│   │   ├── header.rs              # Uses ThemeSwitcher
│   │   └── ...
│   ├── license/
│   │   ├── variant_card.rs        # Uses ember-fx Card, Button, Progress
│   │   └── ...
│   └── ...
│
└── ...
```

### Modular Architecture

The application uses a **unified full-stack Leptos + Actix-web** project with modular internal structure:

**Project:** `/Users/admin/Documents/Unetwork/uno-app`
**Template:** `cargo leptos new --git leptos-rs/start-actix`
**Run:** `cargo leptos serve`

The internal code is organized into modules following domain-driven design principles:
- **Handlers**: HTTP request handling (thin layer)
- **Services**: Business logic (testable, no I/O knowledge)
- **Repositories**: Database access (SQLx)
- **Components**: UI components (Leptos)

```text
+================================================================================+
|                      INTERNAL MODULE ARCHITECTURE                              |
+================================================================================+

uno-app/src/                     # Main source directory
│
│  ┌─────────────────────────────────────────────────────────────────────┐
│  │                         SHARED MODULES                              │
│  │                    (Used by both client & server)                   │
│  └─────────────────────────────────────────────────────────────────────┘
│
├── lib.rs                             # Crate entry, feature gates
├── app.rs                             # Main Leptos App component
│
├── types/                             # Shared type definitions
│   ├── mod.rs
│   ├── license.rs                     # LicenseId, ClaimToken, Variant
│   ├── stats.rs                       # PerformanceStats, VisitorStats
│   ├── faq.rs                         # FaqItem, FaqCategory
│   ├── chatbot.rs                     # Intent, ChatMessage
│   └── error.rs                       # AppError, ApiError
│
├── config/                            # Configuration
│   ├── mod.rs
│   └── settings.rs                    # App settings (env-based)
│
│  ┌─────────────────────────────────────────────────────────────────────┐
│  │                      CLIENT-SIDE MODULES                            │
│  │                   (Compiled to WASM, hydrated)                      │
│  └─────────────────────────────────────────────────────────────────────┘
│
├── routes/                            # Leptos page components
├── components/                        # Leptos UI components
├── hooks/                             # Custom Leptos hooks
│
│  ┌─────────────────────────────────────────────────────────────────────┐
│  │                      SERVER-SIDE MODULES                            │
│  │               (Actix-web, only runs on server)                      │
│  └─────────────────────────────────────────────────────────────────────┘
│
├── server/                            # Server-only code
│   ├── mod.rs
│   │
│   ├── handlers/                      # HTTP handlers (*_handler.rs)
│   │   ├── mod.rs
│   │   ├── licenses_handler.rs        # /api/v1/licenses/*
│   │   ├── stats_handler.rs           # /api/v1/stats/*
│   │   ├── faq_handler.rs             # /api/v1/faq
│   │   ├── chatbot_handler.rs         # /api/v1/chatbot/*
│   │   └── admin_handler.rs           # /api/v1/admin/*
│   │
│   ├── services/                      # Business logic layer
│   │   ├── mod.rs
│   │   ├── license_service.rs         # License claiming logic
│   │   ├── variant_service.rs         # Variant aggregation
│   │   ├── stats_service.rs           # Statistics aggregation
│   │   ├── faq_service.rs             # FAQ retrieval
│   │   ├── chatbot_service.rs         # Intent matching
│   │   └── csv_import_service.rs      # CSV import logic
│   │
│   ├── repositories/                  # Database access layer
│   │   ├── mod.rs
│   │   ├── license_repository.rs
│   │   ├── variant_repository.rs
│   │   ├── claim_repository.rs
│   │   ├── stats_repository.rs
│   │   └── faq_repository.rs
│   │
│   ├── middleware/                    # Actix middleware (*_middleware.rs)
│   │   ├── mod.rs
│   │   ├── auth_middleware.rs         # Admin authentication
│   │   ├── rate_limit_middleware.rs   # Rate limiting
│   │   ├── cors_middleware.rs         # CORS handling
│   │   ├── visitor_middleware.rs      # Visitor tracking
│   │   └── security_middleware.rs     # Security headers
│   │
│   ├── extractors/                    # Custom Actix extractors
│   │   ├── mod.rs
│   │   ├── locale.rs                  # Language extraction
│   │   └── geo.rs                     # IP geolocation
│   │
│   └── db/                            # Database setup
│       ├── mod.rs
│       ├── pool.rs                    # Connection pool (SQLx)
│       └── migrations/                # SQL migration files
│
├── api/                               # Leptos server functions
│   ├── mod.rs
│   ├── licenses.rs                    # #[server] fns for licenses
│   ├── stats.rs                       # #[server] fns for stats
│   └── faq.rs                         # #[server] fns for FAQ
│
└── main.rs                            # Application entry point
```

### Module Dependency Flow

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MODULE DEPENDENCIES                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   main.rs (Entry Point)                                                     │
│       │                                                                     │
│       ├──► config/         (Settings, environment)                          │
│       │                                                                     │
│       ├──► server/         (Actix-web server setup)                         │
│       │       │                                                             │
│       │       ├──► handlers/     ──► services/     ──► repositories/        │
│       │       │                                              │              │
│       │       ├──► middleware/                               │              │
│       │       │                                              │              │
│       │       └──► db/pool.rs  ◄─────────────────────────────┘              │
│       │                                                                     │
│       └──► app.rs (Leptos App)                                              │
│               │                                                             │
│               ├──► routes/                                                  │
│               │                                                             │
│               ├──► components/                                              │
│               │                                                             │
│               └──► api/ (server functions)  ──► services/                   │
│                                                                             │
│   Shared: types/ (used by all modules)                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Security by Design

```text
+------------------------------------------------------------------+
|                     SECURITY ARCHITECTURE                        |
+------------------------------------------------------------------+

1. INPUT VALIDATION (types/mod.rs + handlers)
   ├── All user input validated at API boundary
   ├── Type-safe validation with serde + validator crate
   ├── Sanitization of all string inputs
   └── No raw SQL - SQLx compile-time checked queries

2. RATE LIMITING (server/middleware/rate_limit_middleware.rs)
   ├── Per-IP rate limits (configurable)
   ├── Claim endpoint: 5 requests/minute
   ├── API endpoints: 100 requests/minute
   └── Admin endpoints: 10 requests/minute

3. AUTHENTICATION (server/middleware/auth_middleware.rs)
   ├── Admin routes protected with API key
   ├── No user authentication required (anonymous)
   ├── API keys stored as bcrypt hashes
   └── Key rotation support

4. HEADERS (server/middleware/security_middleware.rs)
   ├── Strict-Transport-Security (HSTS)
   ├── Content-Security-Policy (CSP)
   ├── X-Content-Type-Options: nosniff
   ├── X-Frame-Options: DENY
   └── Referrer-Policy: strict-origin

5. DATA PROTECTION
   ├── No personal data collected (by design)
   ├── Session fingerprints are hashed (SHA-256)
   ├── Claim tokens are cryptographically random
   └── Database connections use TLS

6. DEPENDENCY SECURITY
   ├── cargo-audit in CI pipeline
   ├── Minimal dependency surface per crate
   ├── No unsafe code (deny(unsafe_code))
   └── Regular dependency updates

+------------------------------------------------------------------+
```

### Performance Optimizations

```
+------------------------------------------------------------------+
|                   PERFORMANCE ARCHITECTURE                       |
+------------------------------------------------------------------+

1. COMPILE-TIME (Cargo.toml)
   [profile.release]
   lto = "fat"                 # Link-time optimization
   codegen-units = 1           # Better optimization
   panic = "abort"             # Smaller binary
   strip = true                # Strip debug symbols

2. DATABASE (server/db/)
   ├── Connection pooling (deadpool-postgres)
   ├── Prepared statements (SQLx compile-time)
   ├── Indexes on hot paths
   ├── Claim operation: single atomic query
   └── Read replicas for stats queries

3. CACHING (server/services/)
   ├── Variant list: 5 min TTL
   ├── FAQ items: 1 hour TTL
   ├── Stats: 15 min TTL
   └── Cache invalidation on mutation

4. API (server/handlers/)
   ├── Response compression (gzip/brotli)
   ├── ETag headers for conditional requests
   ├── Streaming responses for large data
   └── Async handlers (Tokio)

5. FRONTEND (routes/ + components/)
   ├── WASM streaming compilation
   ├── Code splitting per route
   ├── Lazy loading of components
   ├── Optimized Tailwind (purged CSS)
   └── Static asset caching (1 year)

6. BENCHMARKS (tests/benches/)
   ├── Claim latency target: < 50ms p99
   ├── Variant list: < 10ms p99
   ├── Stats endpoint: < 100ms p99
   └── Memory: < 128MB per instance

+------------------------------------------------------------------+
```

---

## 2. Database Schema Design

### Based on Unetwork CSV Export Structure

The CSV export from Unetwork admin dashboard contains:

```csv
id,nodeId,ownerWalletAddress,deviceName,activationStartAt,activationEndAt,isActive,isLeased,leaseFrom,leaseTo,leaseSharePercentage,leaseMinUptimePercentage,uptime
```

### Complete Database Schema

```sql
-- =================================================================
-- SCHEMA: License Management System
-- =================================================================

-- Table 1: Raw License Import (mirrors CSV)
-- Purpose: Store imported licenses from Unetwork admin dashboard
-- =================================================================
CREATE TABLE licenses (
    id VARCHAR(66) PRIMARY KEY,                    -- License ID (hex string from CSV)
    node_id VARCHAR(66) NOT NULL,                  -- Node ID (hex string)
    owner_wallet_address VARCHAR(42) NOT NULL,     -- Ethereum address
    device_name VARCHAR(255),                      -- Optional device name
    activation_start_at TIMESTAMP WITH TIME ZONE,
    activation_end_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    is_leased BOOLEAN DEFAULT false,
    lease_from TIMESTAMP WITH TIME ZONE,
    lease_to TIMESTAMP WITH TIME ZONE,
    lease_share_percentage DECIMAL(5,2),           -- e.g., 60.00 for 60%
    lease_min_uptime_percentage DECIMAL(5,2),      -- e.g., 75.00 for 75%
    uptime DECIMAL(5,2) DEFAULT 0,

    -- Metadata
    imported_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Indexes
    CONSTRAINT valid_share CHECK (lease_share_percentage >= 0 AND lease_share_percentage <= 100),
    CONSTRAINT valid_uptime CHECK (lease_min_uptime_percentage >= 0 AND lease_min_uptime_percentage <= 100)
);

CREATE INDEX idx_licenses_is_leased ON licenses(is_leased);
CREATE INDEX idx_licenses_share_percentage ON licenses(lease_share_percentage);
CREATE INDEX idx_licenses_node_id ON licenses(node_id);

-- =================================================================
-- Table 2: License Variants (Aggregated View)
-- Purpose: Group licenses by split type for UI display
-- =================================================================
CREATE TABLE license_variants (
    id SERIAL PRIMARY KEY,

    -- Split configuration
    user_share_percentage INT NOT NULL,            -- e.g., 60 for 60:40
    operator_share_percentage INT NOT NULL,        -- e.g., 40 for 60:40

    -- Lease terms
    lease_duration_months INT NOT NULL DEFAULT 12,
    min_uptime_percentage DECIMAL(5,2) NOT NULL DEFAULT 75.00,

    -- Inventory
    total_quantity INT NOT NULL DEFAULT 0,
    claimed_count INT NOT NULL DEFAULT 0,

    -- Earnings estimates (pulled from business model)
    min_monthly_earnings DECIMAL(10,2),            -- User's minimum expected
    max_monthly_earnings DECIMAL(10,2),            -- User's maximum expected

    -- Display
    display_name VARCHAR(100),                     -- e.g., "Premium 60:40 Split"
    display_order INT DEFAULT 0,                   -- For UI sorting
    is_featured BOOLEAN DEFAULT false,             -- Special offer badge
    status VARCHAR(20) DEFAULT 'active',           -- active, depleted, hidden

    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT valid_split CHECK (user_share_percentage + operator_share_percentage = 100),
    CONSTRAINT unique_split UNIQUE (user_share_percentage, operator_share_percentage, lease_duration_months)
);

-- =================================================================
-- Table 3: License Claims (Anonymous)
-- Purpose: Track claimed licenses without personal data
-- =================================================================
CREATE TABLE license_claims (
    id SERIAL PRIMARY KEY,

    -- References
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id),
    variant_id INT NOT NULL REFERENCES license_variants(id),

    -- Claim details (NO personal data)
    claim_token VARCHAR(64) UNIQUE NOT NULL,       -- For user to retrieve if needed
    claimed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Session info (anonymized)
    session_fingerprint VARCHAR(64),               -- Hashed browser fingerprint
    country_code VARCHAR(2),                       -- From IP geolocation

    -- Status
    status VARCHAR(20) DEFAULT 'claimed',          -- claimed, activated, expired

    CONSTRAINT one_claim_per_license UNIQUE (license_id)
);

CREATE INDEX idx_claims_variant_id ON license_claims(variant_id);
CREATE INDEX idx_claims_claimed_at ON license_claims(claimed_at);
CREATE INDEX idx_claims_country ON license_claims(country_code);

-- =================================================================
-- Table 4: Visitor Statistics
-- Purpose: Track anonymous visitor counts by country
-- =================================================================
CREATE TABLE visitor_stats (
    id SERIAL PRIMARY KEY,

    country_code VARCHAR(2) NOT NULL,
    visit_date DATE NOT NULL,
    page_path VARCHAR(255) NOT NULL DEFAULT '/',

    -- Counters
    visit_count INT DEFAULT 1,
    unique_visitors INT DEFAULT 1,

    -- Metadata
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT unique_daily_stats UNIQUE (country_code, visit_date, page_path)
);

CREATE INDEX idx_visitor_stats_date ON visitor_stats(visit_date);
CREATE INDEX idx_visitor_stats_country ON visitor_stats(country_code);

-- =================================================================
-- Table 5: Performance Statistics (for public dashboard)
-- Purpose: Store anonymized aggregate performance data
-- =================================================================
CREATE TABLE performance_stats (
    id SERIAL PRIMARY KEY,

    -- Time period
    period_type VARCHAR(10) NOT NULL,              -- daily, weekly, monthly
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,

    -- Aggregated metrics (anonymized)
    total_active_licenses INT DEFAULT 0,
    avg_earnings DECIMAL(10,2),
    median_earnings DECIMAL(10,2),
    top_earnings DECIMAL(10,2),

    -- Task breakdown
    telemetry_earnings DECIMAL(10,2),
    cli_testing_earnings DECIMAL(10,2),
    connectivity_earnings DECIMAL(10,2),

    -- Tier distribution
    poor_tier_count INT DEFAULT 0,
    average_tier_count INT DEFAULT 0,
    acceptable_tier_count INT DEFAULT 0,
    ok_tier_count INT DEFAULT 0,
    good_tier_count INT DEFAULT 0,
    very_good_tier_count INT DEFAULT 0,
    excellent_tier_count INT DEFAULT 0,
    champion_tier_count INT DEFAULT 0,

    -- Metadata
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT unique_period UNIQUE (period_type, period_start)
);

CREATE INDEX idx_perf_stats_period ON performance_stats(period_type, period_start);

-- =================================================================
-- Table 6: FAQ Content
-- Purpose: Store FAQ questions and answers for each language
-- =================================================================
CREATE TABLE faq_items (
    id SERIAL PRIMARY KEY,

    -- Categorization
    category VARCHAR(50) NOT NULL,                 -- getting_started, earnings, technical, privacy
    display_order INT DEFAULT 0,

    -- Content (default English)
    question_en TEXT NOT NULL,
    answer_en TEXT NOT NULL,

    -- Localized content (JSONB for flexibility)
    translations JSONB DEFAULT '{}',
    -- Example: {"es": {"question": "...", "answer": "..."}, "hi": {...}}

    -- Metadata
    is_featured BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_faq_category ON faq_items(category);

-- =================================================================
-- Table 7: Chatbot Knowledge Base
-- Purpose: Store chatbot responses and intents
-- =================================================================
CREATE TABLE chatbot_knowledge (
    id SERIAL PRIMARY KEY,

    -- Intent matching
    intent VARCHAR(100) NOT NULL,                  -- earnings_question, withdrawal_help, etc.
    keywords TEXT[] NOT NULL,                      -- Array of trigger keywords

    -- Response (default English)
    response_en TEXT NOT NULL,

    -- Localized responses
    translations JSONB DEFAULT '{}',

    -- Escalation
    requires_human BOOLEAN DEFAULT false,
    escalation_trigger TEXT,

    -- Metadata
    priority INT DEFAULT 0,                        -- Higher = check first
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_chatbot_intent ON chatbot_knowledge(intent);
CREATE INDEX idx_chatbot_keywords ON chatbot_knowledge USING GIN(keywords);

-- =================================================================
-- Table 8: Contact Information
-- Purpose: Store operator contact details
-- =================================================================
CREATE TABLE contact_info (
    id SERIAL PRIMARY KEY,

    contact_type VARCHAR(20) NOT NULL,             -- phone, email, address
    contact_value TEXT NOT NULL,
    display_label VARCHAR(100),

    -- Localization
    country_code VARCHAR(2),                       -- For country-specific contacts

    -- Display
    display_order INT DEFAULT 0,
    is_active BOOLEAN DEFAULT true,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =================================================================
-- VIEWS for Application Use
-- =================================================================

-- View: Available license variants with counts
CREATE VIEW v_available_variants AS
SELECT
    lv.id,
    lv.user_share_percentage,
    lv.operator_share_percentage,
    lv.lease_duration_months,
    lv.min_uptime_percentage,
    lv.total_quantity,
    lv.claimed_count,
    (lv.total_quantity - lv.claimed_count) AS remaining,
    lv.min_monthly_earnings,
    lv.max_monthly_earnings,
    lv.display_name,
    lv.is_featured,
    lv.status,
    lv.display_order
FROM license_variants lv
WHERE lv.status = 'active'
  AND (lv.total_quantity - lv.claimed_count) > 0
ORDER BY lv.display_order, lv.user_share_percentage DESC;

-- View: Unclaimed licenses by variant
CREATE VIEW v_unclaimed_licenses AS
SELECT
    l.id AS license_id,
    l.node_id,
    l.lease_share_percentage,
    l.lease_min_uptime_percentage,
    l.lease_from,
    l.lease_to,
    lv.id AS variant_id
FROM licenses l
LEFT JOIN license_claims lc ON l.id = lc.license_id
LEFT JOIN license_variants lv ON l.lease_share_percentage = lv.user_share_percentage
WHERE l.is_active = true
  AND l.is_leased = false
  AND lc.id IS NULL;

-- =================================================================
-- FUNCTIONS
-- =================================================================

-- Function: Claim a license (atomic operation)
CREATE OR REPLACE FUNCTION claim_license(
    p_variant_id INT,
    p_session_fingerprint VARCHAR(64),
    p_country_code VARCHAR(2)
) RETURNS TABLE(
    success BOOLEAN,
    license_id VARCHAR(66),
    claim_token VARCHAR(64),
    error_message TEXT
) AS $$
DECLARE
    v_license_id VARCHAR(66);
    v_claim_token VARCHAR(64);
BEGIN
    -- Generate claim token
    v_claim_token := encode(gen_random_bytes(32), 'hex');

    -- Find and lock an unclaimed license for this variant
    SELECT l.id INTO v_license_id
    FROM licenses l
    LEFT JOIN license_claims lc ON l.id = lc.license_id
    WHERE l.is_active = true
      AND l.is_leased = false
      AND lc.id IS NULL
      AND l.lease_share_percentage = (
          SELECT user_share_percentage FROM license_variants WHERE id = p_variant_id
      )
    LIMIT 1
    FOR UPDATE SKIP LOCKED;

    IF v_license_id IS NULL THEN
        RETURN QUERY SELECT false, NULL::VARCHAR(66), NULL::VARCHAR(64), 'No licenses available for this variant'::TEXT;
        RETURN;
    END IF;

    -- Create claim record
    INSERT INTO license_claims (license_id, variant_id, claim_token, session_fingerprint, country_code)
    VALUES (v_license_id, p_variant_id, v_claim_token, p_session_fingerprint, p_country_code);

    -- Update license status
    UPDATE licenses SET is_leased = true, updated_at = NOW() WHERE id = v_license_id;

    -- Update variant claimed count
    UPDATE license_variants SET claimed_count = claimed_count + 1, updated_at = NOW() WHERE id = p_variant_id;

    RETURN QUERY SELECT true, v_license_id, v_claim_token, NULL::TEXT;
END;
$$ LANGUAGE plpgsql;

-- Function: Sync variants from licenses table
CREATE OR REPLACE FUNCTION sync_license_variants() RETURNS void AS $$
BEGIN
    -- Insert or update variants based on distinct share percentages in licenses
    INSERT INTO license_variants (user_share_percentage, operator_share_percentage, total_quantity, lease_duration_months, min_uptime_percentage)
    SELECT
        l.lease_share_percentage::INT,
        (100 - l.lease_share_percentage)::INT,
        COUNT(*)::INT,
        EXTRACT(MONTH FROM AGE(l.lease_to, l.lease_from))::INT,
        l.lease_min_uptime_percentage
    FROM licenses l
    WHERE l.lease_share_percentage IS NOT NULL
    GROUP BY l.lease_share_percentage, l.lease_min_uptime_percentage, EXTRACT(MONTH FROM AGE(l.lease_to, l.lease_from))
    ON CONFLICT (user_share_percentage, operator_share_percentage, lease_duration_months)
    DO UPDATE SET
        total_quantity = EXCLUDED.total_quantity,
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- =================================================================
-- SEED DATA
-- =================================================================

-- FAQ seed data
INSERT INTO faq_items (category, display_order, question_en, answer_en, translations, is_featured) VALUES
('getting_started', 1, 'How much will I actually earn?',
 'Earnings depend on your device type and split rate. WiFi-only devices: $2.00-$5.40/month. Phone with SIM (typical): $4.50-$7.20/month. Phone with SIM (excellent): $12.50-$18.00/month. Multiple devices = multiple earnings!',
 '{"es": {"question": "Cuanto ganare realmente?", "answer": "..."}}', true),

('getting_started', 2, 'Why is there a $1.99/month fee?',
 'The fee is paid by the operator (not you!) and covers operational costs. You earn $2.00-$18.00/month based on your split rate and device performance.',
 '{}', true),

('earnings', 1, 'What split rate should I look for?',
 'Higher first number = more for you! 60:40 (you keep 60%), 55:45 (you keep 55%), 52:48 (you keep 52%), 50:50 (you keep 50%).',
 '{}', true),

('technical', 1, 'Will it drain my battery?',
 'No. Optimized for less than 5% battery impact. Most users do not notice any difference.',
 '{}', false),

('privacy', 1, 'Is my data safe?',
 'Yes. No personal data is collected or transmitted. Only anonymized network telemetry.',
 '{}', true);

-- Chatbot knowledge seed data
INSERT INTO chatbot_knowledge (intent, keywords, response_en, priority) VALUES
('earnings_question', ARRAY['earn', 'money', 'income', 'how much', 'earnings', 'make money'],
 'Earnings range from $4-30/month depending on your device and tasks enabled. At 60:40 split, you keep 60% - so if your device generates $20/month, you keep $12! Check our FAQ for detailed earnings tables.',
 10),

('withdrawal_help', ARRAY['withdraw', 'cash out', 'get paid', 'payout', 'withdrawal'],
 'To withdraw: Open the Unetwork app > Wallet tab > Click Withdraw > Choose method (bank or crypto) > Enter amount (min $5) > Confirm. Processing takes 1-3 business days.',
 10),

('license_help', ARRAY['license', 'claim', 'register', 'activate', 'key'],
 'After claiming your license on this website, open the Unetwork app > Go to Licenses > Tap "Add License" > Paste your license key. The license will activate automatically.',
 10),

('contact_human', ARRAY['human', 'support', 'help', 'agent', 'person', 'talk to'],
 'For direct support, you can use the in-app messaging feature in the Unetwork app, or contact us via the details on our Contact page. We typically respond within 4 hours.',
 5);

-- Contact info seed data
INSERT INTO contact_info (contact_type, contact_value, display_label, display_order) VALUES
('email', 'support@unetwork-operator.com', 'Email Support', 1),
('phone', '+1-XXX-XXX-XXXX', 'Phone (US)', 2);
```

---

## 3. Backend API Design

### API Endpoints

```yaml
# =================================================================
# API SPECIFICATION
# =================================================================

openapi: 3.0.0
info:
  title: UNO License Operator API
  version: 1.0.0

paths:
  # -----------------------------------------------------------------
  # License Variants
  # -----------------------------------------------------------------
  /api/v1/licenses/variants:
    get:
      summary: Get available license variants
      description: Returns list of license variants with availability counts
      parameters:
        - name: lang
          in: query
          schema:
            type: string
            default: en
      responses:
        200:
          description: List of available variants
          content:
            application/json:
              schema:
                type: object
                properties:
                  variants:
                    type: array
                    items:
                      $ref: '#/components/schemas/LicenseVariant'

  # -----------------------------------------------------------------
  # License Claim
  # -----------------------------------------------------------------
  /api/v1/licenses/claim:
    post:
      summary: Claim a license
      description: Claims an available license from specified variant
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required:
                - variant_id
              properties:
                variant_id:
                  type: integer
                fingerprint:
                  type: string
                  description: Hashed browser fingerprint (optional)
      responses:
        200:
          description: License claimed successfully
          content:
            application/json:
              schema:
                type: object
                properties:
                  success:
                    type: boolean
                  license_key:
                    type: string
                  claim_token:
                    type: string
                    description: Token to retrieve license later
                  variant:
                    $ref: '#/components/schemas/LicenseVariant'
        400:
          description: No licenses available

  # -----------------------------------------------------------------
  # Public Statistics
  # -----------------------------------------------------------------
  /api/v1/stats/public:
    get:
      summary: Get public performance statistics
      parameters:
        - name: period
          in: query
          schema:
            type: string
            enum: [daily, weekly, monthly]
            default: weekly
      responses:
        200:
          description: Anonymized performance statistics
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/PublicStats'

  /api/v1/stats/visitors:
    get:
      summary: Get visitor statistics by country
      parameters:
        - name: days
          in: query
          schema:
            type: integer
            default: 30
      responses:
        200:
          description: Visitor counts by country
          content:
            application/json:
              schema:
                type: object
                properties:
                  total_visitors:
                    type: integer
                  by_country:
                    type: array
                    items:
                      type: object
                      properties:
                        country_code:
                          type: string
                        country_name:
                          type: string
                        visitor_count:
                          type: integer

  # -----------------------------------------------------------------
  # FAQ
  # -----------------------------------------------------------------
  /api/v1/faq:
    get:
      summary: Get FAQ items
      parameters:
        - name: lang
          in: query
          schema:
            type: string
            default: en
        - name: category
          in: query
          schema:
            type: string
      responses:
        200:
          description: FAQ items grouped by category
          content:
            application/json:
              schema:
                type: object
                properties:
                  categories:
                    type: array
                    items:
                      type: object
                      properties:
                        name:
                          type: string
                        items:
                          type: array
                          items:
                            $ref: '#/components/schemas/FAQItem'

  # -----------------------------------------------------------------
  # Chatbot
  # -----------------------------------------------------------------
  /api/v1/chatbot/message:
    post:
      summary: Send message to chatbot
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required:
                - message
              properties:
                message:
                  type: string
                session_id:
                  type: string
                lang:
                  type: string
                  default: en
      responses:
        200:
          description: Chatbot response
          content:
            application/json:
              schema:
                type: object
                properties:
                  response:
                    type: string
                  intent:
                    type: string
                  confidence:
                    type: number
                  requires_human:
                    type: boolean
                  suggested_actions:
                    type: array
                    items:
                      type: string

  # -----------------------------------------------------------------
  # Contact
  # -----------------------------------------------------------------
  /api/v1/contact:
    get:
      summary: Get contact information
      responses:
        200:
          description: Contact details
          content:
            application/json:
              schema:
                type: object
                properties:
                  contacts:
                    type: array
                    items:
                      $ref: '#/components/schemas/ContactInfo'

  # -----------------------------------------------------------------
  # Tasks Info
  # -----------------------------------------------------------------
  /api/v1/tasks:
    get:
      summary: Get available task types and their details
      parameters:
        - name: lang
          in: query
          schema:
            type: string
            default: en
      responses:
        200:
          description: Task categories and details
          content:
            application/json:
              schema:
                type: object
                properties:
                  tasks:
                    type: array
                    items:
                      $ref: '#/components/schemas/TaskInfo'

  # -----------------------------------------------------------------
  # Admin: CSV Import
  # -----------------------------------------------------------------
  /api/v1/admin/import-licenses:
    post:
      summary: Import licenses from CSV
      security:
        - AdminAuth: []
      requestBody:
        content:
          multipart/form-data:
            schema:
              type: object
              properties:
                file:
                  type: string
                  format: binary
      responses:
        200:
          description: Import results
          content:
            application/json:
              schema:
                type: object
                properties:
                  imported:
                    type: integer
                  skipped:
                    type: integer
                  errors:
                    type: array
                    items:
                      type: string

# =================================================================
# Components
# =================================================================
components:
  schemas:
    LicenseVariant:
      type: object
      properties:
        id:
          type: integer
        user_share_percentage:
          type: integer
        operator_share_percentage:
          type: integer
        lease_duration_months:
          type: integer
        min_uptime_percentage:
          type: number
        total_quantity:
          type: integer
        claimed_count:
          type: integer
        remaining:
          type: integer
        min_monthly_earnings:
          type: number
        max_monthly_earnings:
          type: number
        display_name:
          type: string
        is_featured:
          type: boolean

    PublicStats:
      type: object
      properties:
        period_type:
          type: string
        period_start:
          type: string
          format: date
        period_end:
          type: string
          format: date
        top_performers:
          type: array
          items:
            type: object
            properties:
              rank:
                type: integer
              earnings:
                type: number
              tier:
                type: string
        earnings_by_task:
          type: object
          properties:
            telemetry:
              type: number
            cli_testing:
              type: number
            connectivity:
              type: number
        tier_distribution:
          type: object
          additionalProperties:
            type: integer

    FAQItem:
      type: object
      properties:
        id:
          type: integer
        question:
          type: string
        answer:
          type: string
        is_featured:
          type: boolean

    ContactInfo:
      type: object
      properties:
        type:
          type: string
        value:
          type: string
        label:
          type: string

    TaskInfo:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        status:
          type: string
          enum: [active, coming_soon, always_active]
        type:
          type: string
          enum: [passive, active]
        description:
          type: string
        earnings_contribution:
          type: string
        requirements:
          type: array
          items:
            type: string
```

---

## 4. Frontend Architecture

### Project Location & Setup

```text
Location: /Users/admin/Documents/Unetwork/uno-app

Created with:
  cargo leptos new --git leptos-rs/start-actix

Run with:
  cargo leptos serve

Build for production:
  cargo leptos build --release
```

> **Note:** The project uses the `leptos-rs/start-actix` template which provides
> integrated Leptos + Actix-web with SSR support. This is a **single unified
> full-stack application** - no separate crates needed.

### Project Structure (uno-app)

```text
/Users/admin/Documents/Unetwork/uno-app/
├── Cargo.toml                     # Workspace + app dependencies
├── Cargo.lock
├── end2end/                       # E2E tests (Playwright)
│   └── ...
├── public/                        # Static assets (served as-is)
│   └── favicon.ico
├── style/                         # Stylesheets
│   └── main.scss
├── src/
│   ├── lib.rs                     # Crate entry
│   ├── app.rs                     # Main Leptos app component
│   │
│   ├── routes/                    # Page components
│   │   ├── mod.rs
│   │   ├── home.rs                # / - Landing page
│   │   ├── licenses.rs            # /licenses - License selection
│   │   ├── claim.rs               # /claim/:id - License reveal + optional help
│   │   ├── stats.rs               # /stats - Public dashboard
│   │   ├── tasks.rs               # /tasks - Task categories
│   │   ├── faq.rs                 # /faq - Interactive FAQ
│   │   └── contact.rs             # /contact - Contact info
│   │
│   ├── components/                # Reusable UI components
│   │   ├── mod.rs
│   │   │
│   │   ├── layout/                # Page layout
│   │   │   ├── mod.rs
│   │   │   ├── header.rs          # Logo, nav, language selector
│   │   │   ├── footer.rs          # Contact links, copyright
│   │   │   └── nav.rs             # Navigation menu
│   │   │
│   │   ├── license/               # License-related components
│   │   │   ├── mod.rs
│   │   │   ├── variant_card.rs    # License variant display card
│   │   │   ├── urgency_bar.rs     # Progress bar (claimed/remaining)
│   │   │   ├── license_reveal.rs  # License key + copy + download link
│   │   │   ├── help_accordion.rs  # Optional expandable help sections
│   │   │   └── android_download.rs # Android Play Store button
│   │   │
│   │   ├── stats/                 # Statistics components
│   │   │   ├── mod.rs
│   │   │   ├── top_performers.rs  # Top earners podium
│   │   │   ├── earnings_chart.rs  # Bar chart by task type
│   │   │   ├── period_toggle.rs   # Daily/Weekly/Monthly selector
│   │   │   ├── tier_breakdown.rs  # Tier distribution chart
│   │   │   └── visitor_map.rs     # Visitors by country
│   │   │
│   │   ├── faq/                   # FAQ components
│   │   │   ├── mod.rs
│   │   │   ├── accordion.rs       # Expandable FAQ item
│   │   │   ├── search_bar.rs      # FAQ search input
│   │   │   └── category_tabs.rs   # Category filter tabs
│   │   │
│   │   ├── chatbot/               # Chatbot widget
│   │   │   ├── mod.rs
│   │   │   ├── chat_widget.rs     # Floating widget (collapsed)
│   │   │   ├── chat_window.rs     # Expanded chat interface
│   │   │   └── message_bubble.rs  # Individual message
│   │   │
│   │   ├── tasks/                 # Task info components
│   │   │   ├── mod.rs
│   │   │   ├── task_card.rs       # Task info card
│   │   │   └── device_comparison.rs # WiFi vs Phone earnings table
│   │   │
│   │   └── common/                # Shared UI primitives
│   │       ├── mod.rs
│   │       ├── button.rs          # Primary/secondary buttons
│   │       ├── card.rs            # Card container
│   │       ├── modal.rs           # Modal dialog
│   │       ├── copy_button.rs     # Copy to clipboard with feedback
│   │       ├── accordion.rs       # Generic accordion component
│   │       ├── language_selector.rs # Language dropdown
│   │       ├── loading.rs         # Loading spinner/skeleton
│   │       └── icon.rs            # Icon component
│   │
│   ├── hooks/                     # Custom Leptos hooks
│   │   ├── mod.rs
│   │   ├── use_clipboard.rs       # Clipboard operations
│   │   ├── use_locale.rs          # Current locale state
│   │   └── use_api.rs             # API request hook
│   │
│   │
│   ├── api/                       # Server functions (Leptos server fns)
│   │   ├── mod.rs
│   │   ├── licenses.rs            # License-related server functions
│   │   ├── stats.rs               # Stats server functions
│   │   ├── faq.rs                 # FAQ server functions
│   │   └── chatbot.rs             # Chatbot server functions
│   │
│   ├── server/                    # Server-side only (Actix handlers)
│   │   ├── mod.rs
│   │   │
│   │   ├── handlers/              # REST API handlers
│   │   │   ├── mod.rs
│   │   │   ├── licenses_handler.rs    # /api/v1/licenses/*
│   │   │   ├── stats_handler.rs       # /api/v1/stats/*
│   │   │   ├── faq_handler.rs         # /api/v1/faq
│   │   │   ├── chatbot_handler.rs     # /api/v1/chatbot/*
│   │   │   └── admin_handler.rs       # /api/v1/admin/*
│   │   │
│   │   ├── services/              # Business logic layer
│   │   │   ├── mod.rs
│   │   │   ├── license_service.rs     # License claiming logic
│   │   │   ├── variant_service.rs     # Variant aggregation
│   │   │   ├── stats_service.rs       # Statistics aggregation
│   │   │   ├── faq_service.rs         # FAQ retrieval
│   │   │   ├── chatbot_service.rs     # Intent matching
│   │   │   └── csv_import_service.rs  # CSV import logic
│   │   │
│   │   ├── middleware/            # Actix middleware
│   │   │   ├── mod.rs
│   │   │   ├── auth_middleware.rs     # Admin authentication
│   │   │   ├── rate_limit_middleware.rs  # Rate limiting
│   │   │   ├── cors_middleware.rs     # CORS handling
│   │   │   ├── visitor_middleware.rs  # Visitor tracking
│   │   │   └── security_middleware.rs # Security headers
│   │   │
│   │   ├── repositories/          # Database access layer
│   │   │   ├── mod.rs
│   │   │   ├── license_repository.rs
│   │   │   ├── variant_repository.rs
│   │   │   ├── claim_repository.rs
│   │   │   ├── stats_repository.rs
│   │   │   └── faq_repository.rs
│   │   │
│   │   ├── extractors/            # Custom Actix extractors
│   │   │   ├── mod.rs
│   │   │   ├── locale.rs              # Language extraction
│   │   │   └── geo.rs                 # IP geolocation
│   │   │
│   │   └── db/                    # Database setup
│   │       ├── mod.rs
│   │       ├── pool.rs                # Connection pool
│   │       └── migrations/            # SQL migrations
│   │
│   └── main.rs                    # Application entry point
│
├── assets/                        # Static assets
│   ├── images/
│   │   ├── logo.svg
│   │   └── app-screenshots/
│   └── locales/                   # Translation files
│       ├── en.json
│       ├── tl.json                # Tagalog (Philippines)
│       ├── hi.json                # Hindi (India)
│       ├── sw.json                # Swahili (East Africa)
│       ├── es.json                # Spanish (LATAM)
│       ├── pt.json                # Portuguese (Brazil)
│       ├── fr.json                # French (West Africa)
│       ├── ar.json                # Arabic (MENA)
│       └── id.json                # Bahasa Indonesia
│
└── style/
    └── main.scss                  # Main stylesheet (SCSS)
```

### Request Flow Architecture

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                           REQUEST FLOW                                       │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   Browser Request                                                            │
│         │                                                                    │
│         ▼                                                                    │
│   ┌─────────────┐                                                            │
│   │   Actix     │  (main.rs - server entry)                                  │
│   │   Server    │                                                            │
│   └──────┬──────┘                                                            │
│          │                                                                   │
│          ├─────────────────────┬──────────────────────┐                      │
│          │                     │                      │                      │
│          ▼                     ▼                      ▼                      │
│   ┌─────────────┐       ┌─────────────┐       ┌─────────────┐                │
│   │   Leptos    │       │  /api/v1/*  │       │   Static    │                │
│   │   Routes    │       │   Routes    │       │   Assets    │                │
│   │   (SSR)     │       │   (REST)    │       │             │                │
│   └──────┬──────┘       └──────┬──────┘       └─────────────┘                │
│          │                     │                                             │
│          │                     ▼                                             │
│          │              ┌─────────────┐                                      │
│          │              │ Middleware  │                                      │
│          │              │ - Auth      │                                      │
│          │              │ - RateLimit │                                      │
│          │              │ - CORS      │                                      │
│          │              │ - Security  │                                      │
│          │              └──────┬──────┘                                      │
│          │                     │                                             │
│          │                     ▼                                             │
│          │              ┌─────────────┐                                      │
│          │              │  Handlers   │                                      │
│          │              │ *_handler.rs│                                      │
│          │              └──────┬──────┘                                      │
│          │                     │                                             │
│          ▼                     ▼                                             │
│   ┌─────────────┐       ┌─────────────┐                                      │
│   │   Server    │       │  Services   │                                      │
│   │  Functions  │       │ *_service.rs│                                      │
│   │   (RPC)     │       └──────┬──────┘                                      │
│   └──────┬──────┘              │                                             │
│          │                     │                                             │
│          └─────────┬───────────┘                                             │
│                    │                                                         │
│                    ▼                                                         │
│             ┌─────────────┐                                                  │
│             │Repositories │                                                  │
│             │*_repository │                                                  │
│             └──────┬──────┘                                                  │
│                    │                                                         │
│                    ▼                                                         │
│             ┌─────────────┐                                                  │
│             │ PostgreSQL  │                                                  │
│             │   (SQLx)    │                                                  │
│             └─────────────┘                                                  │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Page Routes

| Route | Component | Description |
|-------|-----------|-------------|
| `/` | `home.rs` | Landing page with intro + CTA |
| `/licenses` | `licenses.rs` | License variant selection |
| `/claim/:id` | `claim.rs` | License reveal + optional help accordion |
| `/stats` | `stats.rs` | Public performance dashboard |
| `/tasks` | `tasks.rs` | Task categories & earnings |
| `/faq` | `faq.rs` | Searchable FAQ with categories |
| `/contact` | `contact.rs` | Contact information |

### Key Component Details

```rust
// src/components/license/license_reveal.rs

/// Primary license reveal component shown after claiming
/// Contains:
/// - License key with copy button
/// - Android download button (prominent)
/// - Quick start summary
/// - License details (expandable)
/// - Help accordion (optional sections)

#[component]
pub fn LicenseReveal(
    license_key: String,
    claim_token: String,
    variant: LicenseVariant,
) -> impl IntoView {
    // Component implementation
}
```

```rust
// src/components/license/android_download.rs

/// Prominent Android download button
/// Links to Google Play Store

#[component]
pub fn AndroidDownload() -> impl IntoView {
    let play_store_url = "https://play.google.com/store/apps/details?id=com.unetwork";

    view! {
        <a
            href=play_store_url
            target="_blank"
            class="btn btn-primary btn-lg w-full"
        >
            <AndroidIcon />
            "Download for Android"
        </a>
    }
}
```

```rust
// src/components/license/help_accordion.rs

/// Optional help sections that expand when clicked
/// Keeps main flow simple - users can skip these

#[component]
pub fn HelpAccordion(license_key: String) -> impl IntoView {
    let sections = vec![
        ("How to download & install the app", download_content()),
        ("How to create your account", signup_content()),
        ("How to register your license", register_content(license_key)),
        ("How to contact support", support_content()),
        ("Troubleshooting & debugging tips", troubleshooting_content()),
    ];

    view! {
        <div class="space-y-2 mt-6">
            <p class="text-sm text-gray-500">"Need more help? (Optional)"</p>
            <For
                each=move || sections.clone()
                key=|(title, _)| title.to_string()
                children=|(title, content)| {
                    view! { <AccordionItem title=title>{content}</AccordionItem> }
                }
            />
        </div>
    }
}
```

---

## 5. License Claim Wizard

### Wizard Flow Specification

```
+================================================================================+
|                          LICENSE CLAIM WIZARD FLOW                             |
+================================================================================+

STEP 0: LANDING PAGE INTRO (/)
+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  +---------------------------------------------------------------------------+ |
|  |                                                                           | |
|  |   UNETWORK LICENSE OPERATOR                                               | |
|  |   ─────────────────────────                                               | |
|  |                                                                           | |
|  |   Turn your smartphone into a passive income machine.                     | |
|  |                                                                           | |
|  |   We are a licensed node operator on the Unetwork blockchain.             | |
|  |   By claiming a license through us, you join our network and              | |
|  |   earn money while your phone runs connectivity tests in the              | |
|  |   background.                                                             | |
|  |                                                                           | |
|  |   • No subscription fees for you (we pay the $1.99/month)                 | |
|  |   • Transparent splits from 50:50 to 60:40                                | |
|  |   • Earn $4-30/month depending on performance                             | |
|  |                                                                           | |
|  |                    [  GET YOUR FREE LICENSE  ]                            | |
|  |                                                                           | |
|  +---------------------------------------------------------------------------+ |
|                                                                                |
+--------------------------------------------------------------------------------+

STEP 1: SELECT LICENSE TYPE (/licenses)
+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  STEP 1 OF 7                               ○ ○ ○ ○ ○ ○ ○                       |
|                                                                                |
|  CHOOSE YOUR LICENSE                                                           |
|  ───────────────────                                                           |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  | ⭐ SPECIAL OFFER                                                        |   |
|  |                                                                         |   |
|  |   60:40 SPLIT                                                           |   |
|  |   You keep 60% of earnings                                              |   |
|  |                                                                         |   |
|  |   ████████████████░░░░  27/50 remaining                                 |   |
|  |                                                                         |   |
|  |   📅 12 month lease                                                     |   |
|  |   📈 Estimated: $5.40 - $18.00/month                                    |   |
|  |   ⏱️ Min uptime: 75%                                                    |   |
|  |                                                                         |   |
|  |   [  CLAIM THIS LICENSE  ]                                              |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   55:45 SPLIT                                                           |   |
|  |   You keep 55% of earnings                                              |   |
|  |                                                                         |   |
|  |   ██████████░░░░░░░░░░  33/100 remaining                                |   |
|  |                                                                         |   |
|  |   📅 12 month lease                                                     |   |
|  |   📈 Estimated: $4.95 - $16.50/month                                    |   |
|  |   ⏱️ Min uptime: 75%                                                    |   |
|  |                                                                         |   |
|  |   [  CLAIM THIS LICENSE  ]                                              |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   52:48 SPLIT                                                           |   |
|  |   You keep 52% of earnings                                              |   |
|  |                                                                         |   |
|  |   █████████████░░░░░░░  34/75 remaining                                 |   |
|  |                                                                         |   |
|  |   📅 12 month lease                                                     |   |
|  |   📈 Estimated: $4.68 - $15.60/month                                    |   |
|  |   ⏱️ Min uptime: 75%                                                    |   |
|  |                                                                         |   |
|  |   [  CLAIM THIS LICENSE  ]                                              |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
+--------------------------------------------------------------------------------+

STEP 2: LICENSE REVEALED (/claim/:variant_id) - PRIMARY PAGE
+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  🎉 YOUR LICENSE IS READY!                                                     |
|  ────────────────────────                                                      |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   YOUR LICENSE KEY:                                                     |   |
|  |                                                                         |   |
|  |   +-----------------------------------------------------------+         |   |
|  |   | 0x06c66b1076d43862fcd7c212641cfc5a45baf662777b51d4fb649.. | [📋]    |   |
|  |   +-----------------------------------------------------------+         |   |
|  |                                                    ✓ Click to copy      |   |
|  |                                                                         |   |
|  |   📋 Claim Token: abc123xyz789                                          |   |
|  |   (Save this to retrieve your license later)                            |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   📱 DOWNLOAD THE APP NOW                                               |   |
|  |   ────────────────────────                                              |   |
|  |                                                                         |   |
|  |   +-----------------------------------------------------------+         |   |
|  |   |                                                           |         |   |
|  |   |   [  🤖 DOWNLOAD FOR ANDROID  ]                           |         |   |
|  |   |                                                           |         |   |
|  |   |   ↳ play.google.com/store/apps/details?id=com.unetwork   |          |   |
|  |   |                                                           |         |   |
|  |   +-----------------------------------------------------------+         |   |
|  |                                                                         |   |
|  |   📌 Quick Start:                                                       |   |
|  |   1. Download app above                                                 |   |
|  |   2. Create account                                                     |   |
|  |   3. Go to Licenses → Add License → Paste your key                      |   |
|  |   4. Start earning!                                                     |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   LICENSE DETAILS                                           [?]         |   |
|  |   ───────────────                                                       |   |
|  |   Split Rate:      60:40 (You keep 60%)                                 |   |
|  |   Lease Duration:  12 months                                            |   |
|  |   Min Uptime:      75%                                                  |   |
|  |   Valid From:      Feb 22, 2026                                         |   |
|  |   Valid Until:     Feb 22, 2027                                         |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  ──────────────────────────────────────────────────────────────────────────────|
|                                                                                |
|  📖 NEED MORE HELP? (Optional - click to expand)                               |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |  ▶ How to download & install the app                                    |   |
|  +-------------------------------------------------------------------------+   |
|  |  ▶ How to create your account                                           |   |
|  +-------------------------------------------------------------------------+   |
|  |  ▶ How to register your license                                         |   |
|  +-------------------------------------------------------------------------+   |
|  |  ▶ How to contact support (in-app messaging)                            |   |
|  +-------------------------------------------------------------------------+   |
|  |  ▶ Troubleshooting & debugging tips                                     |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  [  VIEW FULL FAQ  ]              [  GO TO STATS DASHBOARD  ]                  |
|                                                                                |
+--------------------------------------------------------------------------------+

================================================================================
OPTIONAL EXPANDABLE SECTIONS (Accordion on same page)
================================================================================

When user clicks "▶ How to download & install the app":

+-------------------------------------------------------------------------+
|  ▼ How to download & install the app                                    |
|  ───────────────────────────────────────────────────────────────────────|
|                                                                         |
|   1. Click the "Download for Android" button above                      |
|      or search "Unetwork" in Google Play Store                          |
|                                                                         |
|   2. Install the app (requires ~50MB)                                   |
|                                                                         |
|   3. When prompted, allow these permissions:                            |
|      • Background activity (required for earning)                       |
|      • Network access                                                   |
|      • Location (optional, for connectivity tasks)                      |
|                                                                         |
|   💡 TIP: Keep the app running in background for maximum earnings       |
|                                                                         |
+-------------------------------------------------------------------------+

When user clicks "▶ How to create your account":

+-------------------------------------------------------------------------+
|  ▼ How to create your account                                           |
|  ───────────────────────────────────────────────────────────────────────|
|                                                                         |
|   1. Open the Unetwork app                                              |
|                                                                         |
|   2. Tap "Get Started" or "Create Account"                              |
|                                                                         |
|   3. Choose sign-up method:                                             |
|      • Connect existing crypto wallet (recommended)                     |
|      • Create new account with email                                    |
|                                                                         |
|   4. Complete verification                                              |
|                                                                         |
|   5. Set up withdrawal preferences (wallet address)                     |
|                                                                         |
+-------------------------------------------------------------------------+

When user clicks "▶ How to register your license":

+-------------------------------------------------------------------------+
|  ▼ How to register your license                                         |
|  ───────────────────────────────────────────────────────────────────────|
|                                                                         |
|   Your License Key (tap to copy):                                       |
|   +-----------------------------------------------------------+         |
|   | 0x06c66b1076d43862fcd7c212641cfc5a45baf662...             | [📋]    |
|   +-----------------------------------------------------------+         |
|                                                                         |
|   1. Open the Unetwork app                                              |
|   2. Navigate to "Licenses" tab at the bottom                           |
|   3. Tap "Add License" or the + button                                  |
|   4. Paste your license key                                             |
|   5. Tap "Activate"                                                     |
|                                                                         |
|   ✅ Once activated, your phone starts earning automatically!           |
|                                                                         |
+-------------------------------------------------------------------------+

When user clicks "▶ How to contact support":

+-------------------------------------------------------------------------+
|  ▼ How to contact support (in-app messaging)                            |
|  ───────────────────────────────────────────────────────────────────────|
|                                                                         |
|   📱 IN-APP MESSAGING (Fastest - response within 4 hours)               |
|   1. Open Unetwork app                                                  |
|   2. Go to Settings → Support                                           |
|   3. Tap "Message Operator"                                             |
|                                                                         |
|   📧 EMAIL: support@unetwork-operator.com                               |
|                                                                         |
|   📞 PHONE: +1-XXX-XXX-XXXX                                             |
|                                                                         |
|   💡 Check our FAQ first - most questions are answered there!           |
|                                                                         |
+-------------------------------------------------------------------------+

When user clicks "▶ Troubleshooting & debugging tips":

+-------------------------------------------------------------------------+
|  ▼ Troubleshooting & debugging tips                                     |
|  ───────────────────────────────────────────────────────────────────────|
|                                                                         |
|   ❌ App keeps stopping                                                 |
|   ✅ Disable battery optimization:                                      |
|      Settings → Apps → Unetwork → Battery → Unrestricted                |
|                                                                         |
|   ❌ License not activating                                             |
|   ✅ Check internet connection                                          |
|   ✅ Ensure full license key was copied                                 |
|   ✅ Restart the app                                                    |
|                                                                         |
|   ❌ Low earnings                                                       |
|   ✅ Use stable WiFi (preferred over mobile data)                       |
|   ✅ Keep app running 24/7                                              |
|   ✅ Enable all available tasks                                         |
|                                                                         |
|   ❌ Withdrawal issues                                                  |
|   ✅ Minimum withdrawal: $5                                             |
|   ✅ Verify wallet address                                              |
|   ✅ Contact support if pending > 3 days                                |
|                                                                         |
+-------------------------------------------------------------------------+

================================================================================
WIZARD FLOW SUMMARY (Simplified)
================================================================================

OLD FLOW (7 mandatory steps):
Step 1: Select License → Step 2: License Revealed → Step 3: Download App →
Step 4: Sign Up → Step 5: Register License → Step 6: Contact Info →
Step 7: Debugging Tips

NEW FLOW (2 steps + optional help):
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│   STEP 1: SELECT LICENSE (/licenses)                                    │
│   └── User selects a license variant                                    │
│                                                                         │
│   STEP 2: LICENSE REVEALED (/claim/:id) ← PRIMARY PAGE                  │
│   ├── License key with copy button                                      │
│   ├── Android download button (prominent)                               │
│   ├── Quick start summary (4 steps)                                     │
│   ├── License details (expandable)                                      │
│   └── OPTIONAL: Expandable help sections (accordion)                    │
│       ├── How to download & install                                     │
│       ├── How to create account                                         │
│       ├── How to register license                                       │
│       ├── How to contact support                                        │
│       └── Troubleshooting tips                                          │
│                                                                         │
│   ✅ User can leave after copying license - help is optional            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘

REMOVED OLD STEPS (now consolidated into expandable accordion):
- STEP 3: HOW TO DOWNLOAD & INSTALL APP
- STEP 4: HOW TO ONBOARD/SIGN UP
- STEP 5: HOW TO REGISTER LICENSE
- STEP 6: HOW TO CONTACT OPERATOR
- STEP 7: DEBUGGING TIPS
```

---

## 6. Public Statistics Dashboard

### Dashboard Specification

```
+================================================================================+
|                          PUBLIC STATISTICS DASHBOARD                           |
|                                    /stats                                      |
+================================================================================+

+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  COMMUNITY PERFORMANCE                                                         |
|  ─────────────────────                                                         |
|                                                                                |
|  [ Daily ]  [ Weekly ●]  [ Monthly ]                                           |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   🏆 TOP PERFORMERS THIS WEEK                                           |   |
|  |                                                                         |   |
|  |   +------------------+  +------------------+  +------------------+      |   |
|  |   |       🥇         |  |       🥈         |  |       🥉          |      |  |
|  |   |                  |  |                  |  |                  |      |   |
|  |   |     $27.50       |  |     $24.80       |  |     $22.10       |      |   |
|  |   |                  |  |                  |  |                  |      |   |
|  |   |  Champion Tier   |  |  Excellent Tier  |  | Very Good Tier   |      |   |
|  |   +------------------+  +------------------+  +------------------+      |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   📊 EARNINGS BY TASK TYPE                                              |   |
|  |                                                                         |   |
|  |   Telemetry          ████████████████████████████  $12.50 avg           |   |
|  |   CLI Testing        ██████████████████            $8.20 avg            |   |
|  |   Connectivity       ████████████                  $5.40 avg            |   |
|  |                                                                         |   |
|  |   (SMS Testing and Entropy coming soon)                                 |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   📈 WEEKLY EARNINGS DISTRIBUTION                                       |   |
|  |                                                                         |   |
|  |   $30 ┤                                              ╭───╮              |   |
|  |   $25 ┤                                    ╭─────────╯   │              |   |
|  |   $20 ┤                          ╭─────────╯             │              |   |
|  |   $15 ┤                ╭─────────╯                       │              |   |
|  |   $10 ┤      ╭─────────╯                                 │              |   |
|  |    $5 ┤──────╯                                           │              |   |
|  |    $0 ┼───────────────────────────────────────────────────              |   |
|  |        Mon    Tue    Wed    Thu    Fri    Sat    Sun                    |   |
|  |                                                                         |   | 
|  +-------------------------------------------------------------------------+   | 
|                                                                                |
|  +--------------------------------------+  +-------------------------------+   |
|  |                                      |  |                               |   |
|  |   📊 TIER DISTRIBUTION               |  |   📍 ACTIVE LICENSES          |   |
|  |                                      |  |                               |  |
|  |   Champion    ██         8%          |  |   Total Active: 1,247         |  |
|  |   Excellent   ████       12%         |  |                               |  |
|  |   Very Good   ██████     18%         |  |   By Region:                  |  |
|  |   Good        ████████   25%         |  |   🇵🇭 Philippines: 42%         |  |
|  |   OK          ██████     20%         |  |   🇮🇳 India: 28%               |  |
|  |   Acceptable  ████       10%         |  |   🇳🇬 Nigeria: 15%             |  |
|  |   Average     ██         5%          |  |   🇰🇪 Kenya: 8%                |  |
|  |   Poor        █          2%          |  |   🌍 Other: 7%                |  |
|  |                                      |  |                               |  |
|  +--------------------------------------+  +-------------------------------+  |
|                                                                               |
|  +-------------------------------------------------------------------------+  |
|  |                                                                         |  |
|  |   🌍 VISITOR STATISTICS (Last 30 Days)                                  |  |
|  |                                                                         |  |
|  |   Total Visitors: 15,432                                                |  |
|  |                                                                         |  |
|  |   🇵🇭 Philippines   ████████████████  5,200 (34%)                        |  |
|  |   🇮🇳 India         ██████████████    4,100 (27%)                        |  |
|  |   🇳🇬 Nigeria       ████████          2,300 (15%)                        |  |
|  |   🇵🇰 Pakistan      ██████            1,800 (12%)                        |  |
|  |   🇰🇪 Kenya         ████              1,100 (7%)                         |  |
|  |   🇬🇭 Ghana         ██                 500 (3%)                          |  |
|  |   🌍 Others         ██                 432 (2%)                         |  |
|  |                                                                         |  |
|  +-------------------------------------------------------------------------+  |
|                                                                               |
+-------------------------------------------------------------------------------+
```

---

## 7. Support & FAQ System

### FAQ Page Specification

```
+================================================================================+
|                              INTERACTIVE FAQ PAGE                              |
|                                     /faq                                       |
+================================================================================+

+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  FREQUENTLY ASKED QUESTIONS                                                    |
|  ──────────────────────────                                                    |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |  🔍 Search FAQ...                                              [Search] |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  [ All ]  [ Getting Started ● ]  [ Earnings ]  [ Technical ]  [ Privacy ]      |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  ▼ How much will I actually earn?                           ⭐ Featured |   |
|  |  ─────────────────────────────────────────────────────────────────────  |   |
|  |  Earnings depend on your device type and split rate:                    |   |
|  |                                                                         |   |
|  |  • WiFi-only devices: $2.00-$5.40/month                                 |   |
|  |  • Phone with SIM (typical): $4.50-$7.20/month                          |   |
|  |  • Phone with SIM (excellent): $12.50-$18.00/month                      |   |
|  |                                                                         |   |
|  |  Multiple devices = multiple earnings!                                  |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  ▶ Why is there a $1.99/month fee?                          ⭐ Featured |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  ▶ What split rate should I look for?                       ⭐ Featured |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  ▶ How do I withdraw my earnings?                                       |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  ... more questions ...                                                        |
|                                                                                |
+--------------------------------------------------------------------------------+
```

### Chatbot Widget Specification

```
+================================================================================+
|                            CHATBOT WIDGET (FLOATING)                           |
+================================================================================+

COLLAPSED STATE (Bottom-right corner of all pages):
+------------------+
|                  |
|   💬 Need help?  |
|                  |
+------------------+

EXPANDED STATE:
+----------------------------------------+
|  💬 UNO Support                    [X] |
|----------------------------------------|
|                                        |
|  Hi! I'm here to help you with:        |
|  • License questions                   |
|  • Earnings information                |
|  • Technical support                   |
|  • Withdrawal help                     |
|                                        |
|  What can I help you with?             |
|                                        |
|----------------------------------------|
|                                        |
|  USER: How much can I earn?            |
|                                        |
|  BOT: Earnings range from $4-30/month  |
|  depending on your device and tasks    |
|  enabled. At 60:40 split, you keep     |
|  60% - so if your device generates     |
|  $20/month, you keep $12!              |
|                                        |
|  Would you like to:                    |
|  • [View earnings table]               |
|  • [Claim a license]                   |
|  • [Read full FAQ]                     |
|                                        |
|----------------------------------------|
|  Type your message...          [Send]  |
+----------------------------------------+
```

### Chatbot Intent Handling

```
+------------------------------------------------------------------+
|                    CHATBOT INTENT MAPPING                        |
+------------------------------------------------------------------+

Intent: earnings_question
├── Keywords: earn, money, income, how much, earnings, make money
├── Response: Earnings table with splits
└── Actions: [View earnings] [Claim license]

Intent: withdrawal_help
├── Keywords: withdraw, cash out, get paid, payout
├── Response: Step-by-step withdrawal guide
└── Actions: [View withdrawal FAQ]

Intent: license_help
├── Keywords: license, claim, register, activate, key
├── Response: License registration steps
└── Actions: [Claim license] [View guide]

Intent: technical_issue
├── Keywords: not working, error, problem, issue, help
├── Response: Common troubleshooting steps
└── Actions: [View troubleshooting] [Contact support]
└── Escalation: If confidence < 70%, offer human contact

Intent: contact_human
├── Keywords: human, support, help, agent, person, talk to
├── Response: Contact options
└── Actions: [View contact page] [In-app messaging guide]
└── Note: No Discord/WhatsApp/Telegram - only email, phone, in-app
```

---

## 8. Task Categories Page

### Tasks Page Specification

```
+================================================================================+
|                              TASK CATEGORIES PAGE                              |
|                                    /tasks                                      |
+================================================================================+

+--------------------------------------------------------------------------------+
|  [Logo]                                    [Language Selector] [FAQ] [Contact] |
|                                                                                |
|  AVAILABLE TASKS & EARNINGS                                                    |
|  ──────────────────────────                                                    |
|                                                                                |
|  Maximize your earnings by enabling all available tasks in the Unetwork app.   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  🔗 CONNECTION TO UNETWORK (TELEMETRY)                                  |   |
|  |  ─────────────────────────────────────                                  |   |
|  |                                                                         |   |
|  |  Status: ✅ Always Active (Required)                                    |   |
|  |  Type: PASSIVE - No action required                                     |   |
|  |                                                                         |   |
|  |  Keeps your device connected to participate in all network tasks.       |   |
|  |  This is the base layer that enables all earnings.                      |   |
|  |                                                                         |   |
|  |  📊 Earnings Contribution: ~40-50% of total                             |   |
|  |  🔋 Battery Impact: 3-5% daily                                          |   |
|  |  📶 Data Usage: 50-100MB/month                                          |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  📞 CALLER ID TESTING (CLI)                                             |   |
|  |  ──────────────────────────                                             |   |
|  |                                                                         |   |
|  |  Status: ✅ Active                                                      |   |
|  |  Type: ACTIVE - Some interaction required                               |   |
|  |                                                                         |   |
|  |  Report caller ID information on incoming test calls.                   |   |
|  |  Helps telecom companies verify their network quality.                  |   |
|  |                                                                         |   |
|  |  📊 Earnings Contribution: +25-40% boost                                |   |
|  |  📱 Requirements: Phone with active SIM                                 |   |
|  |                                                                         |   |
|  |  HOW TO ENABLE:                                                         |   |
|  |  1. Open Unetwork app > Tasks                                           |   |
|  |  2. Tap "Caller ID Testing"                                             |   |
|  |  3. Grant phone permissions                                             |   |
|  |  4. Toggle ON                                                           |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  📍 CONNECTIVITY VERIFICATION                                           |   |
|  |  ────────────────────────────                                           |   |
|  |                                                                         |   |
|  |  Status: ✅ Available                                                   |   |
|  |  Type: PASSIVE - Automatic                                              |   |
|  |                                                                         |   |
|  |  Checks signal strength and contributes to network coverage mapping.    |   |
|  |                                                                         |   |
|  |  📊 Earnings Contribution: +10-15% boost                                |   |
|  |  📱 Requirements: Location permissions                                  |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  📱 SMS TESTING                                                         |   |
|  |  ──────────────                                                         |   |
|  |                                                                         |   |
|  |  Status: 🔜 Coming Soon                                                 |   |
|  |  Type: ACTIVE                                                           |   |
|  |                                                                         |   |
|  |  Report content of incoming test SMS messages.                          |   |
|  |  Will be available in a future app update.                              |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |  🎲 ENTROPY GENERATION                                                  |   |
|  |  ─────────────────────                                                  |   |
|  |                                                                         |   |
|  |  Status: 🔜 Coming Soon                                                 |   |
|  |  Type: PASSIVE                                                          |   |
|  |                                                                         |   |
|  |  Your phone's sensors generate random data for cryptographic uses.      |   |
|  |  Will be available in a future app update.                              |   |
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|  +-------------------------------------------------------------------------+   |
|  |                                                                         |   |
|  |   💰 EARNINGS BY DEVICE TYPE                                            |   |
|  |                                                                         |   |
|  |   +-------------------------------+-------------------------------+     |   |
|  |   | WiFi-Only Device              | Phone with Active SIM         |     |   |
|  |   |-------------------------------|-------------------------------|     |   |
|  |   | Tasks:                        | Tasks:                        |     |   |
|  |   | • Telemetry ✅                | • All WiFi tasks ✅           |     |   |
|  |   | • Connectivity ✅             | • Caller ID Testing ✅        |     |   |
|  |   | • Entropy (soon)              | • SMS Testing (soon)          |     |   |
|  |   |                               |                               |     |   |
|  |   | Device Earnings: $4-9/mo      | Device Earnings: $9-30/mo     |     |   |
|  |   |                               |                               |     |   |
|  |   | Your Share (60:40):           | Your Share (60:40):           |     |   |
|  |   | $2.40 - $5.40/month           | $5.40 - $18.00/month          |     |   | 
|  |   +-------------------------------+-------------------------------+     |   | 
|  |                                                                         |   |
|  +-------------------------------------------------------------------------+   |
|                                                                                |
|                           [  CLAIM YOUR LICENSE  ]                             |
|                                                                                |
+--------------------------------------------------------------------------------+
```

---

## 9. Localization System

### Supported Languages

```
+------------------------------------------------------------------+
|                    LOCALIZATION CONFIGURATION                    |
+------------------------------------------------------------------+

Primary Languages (Required):
├── en - English (default)
├── tl - Tagalog (Philippines)
├── hi - Hindi (India)
└── sw - Swahili (Kenya/Tanzania)

Secondary Languages (Phase 2):
├── es - Spanish (Latin America)
├── pt - Portuguese (Brazil)
├── fr - French (West Africa)
├── ar - Arabic (MENA)
└── id - Bahasa Indonesia

Language Detection Priority:
1. URL parameter (?lang=es)
2. Cookie (user preference)
3. Browser Accept-Language header
4. IP Geolocation -> Country -> Default language
5. Fallback: English

+------------------------------------------------------------------+
```

### i18n File Structure

```json
// locales/en.json
{
  "common": {
    "get_license": "Get Your Free License",
    "claim": "Claim This License",
    "next": "Next",
    "back": "Back",
    "copy": "Copy",
    "copied": "Copied!",
    "loading": "Loading...",
    "error": "Something went wrong"
  },
  "nav": {
    "home": "Home",
    "licenses": "Licenses",
    "stats": "Statistics",
    "tasks": "Tasks",
    "faq": "FAQ",
    "contact": "Contact"
  },
  "home": {
    "hero_title": "Turn Your Phone Into Passive Income",
    "hero_subtitle": "Join our network and earn $4-30/month automatically",
    "cta": "Get Your Free License"
  },
  "licenses": {
    "title": "Choose Your License",
    "split": "{{user}}:{{operator}} Split",
    "you_keep": "You keep {{percent}}% of earnings",
    "remaining": "{{remaining}}/{{total}} remaining",
    "estimated": "Estimated: ${{min}} - ${{max}}/month",
    "lease_duration": "{{months}} month lease",
    "min_uptime": "Min uptime: {{percent}}%",
    "featured": "Special Offer"
  },
  "claim": {
    "success_title": "Your License is Ready!",
    "license_key": "Your License Key:",
    "claim_token": "Claim Token (save this):",
    "details_title": "License Details",
    "valid_from": "Valid From",
    "valid_until": "Valid Until"
  },
  "stats": {
    "title": "Community Performance",
    "top_performers": "Top Performers This Week",
    "earnings_by_task": "Earnings by Task Type",
    "tier_distribution": "Tier Distribution",
    "visitor_stats": "Visitor Statistics"
  },
  "faq": {
    "title": "Frequently Asked Questions",
    "search_placeholder": "Search FAQ...",
    "categories": {
      "all": "All",
      "getting_started": "Getting Started",
      "earnings": "Earnings",
      "technical": "Technical",
      "privacy": "Privacy"
    }
  },
  "chatbot": {
    "title": "UNO Support",
    "greeting": "Hi! I'm here to help you with licenses, earnings, and technical support. What can I help you with?",
    "placeholder": "Type your message...",
    "send": "Send"
  },
  "contact": {
    "title": "Contact Us",
    "in_app": "In-App Messaging (Fastest)",
    "in_app_desc": "Use the Unetwork app's built-in messaging feature",
    "email": "Email",
    "phone": "Phone"
  }
}
```

---

## 10. Infrastructure & Deployment

### Architecture Diagram

```
+================================================================================+
|                         INFRASTRUCTURE ARCHITECTURE                            |
+================================================================================+

                              USERS
                                │
                                ▼
                    ┌───────────────────────┐
                    │     CLOUDFLARE        │
                    │   (CDN + WAF + DNS)   │
                    │                       │
                    │  • SSL Termination    │
                    │  • DDoS Protection    │
                    │  • Geo-routing        │
                    │  • Edge Caching       │
                    └───────────┬───────────┘
                                │
          ┌─────────────────────┼─────────────────────┐
          │                     │                     │
          ▼                     ▼                     ▼
    ┌───────────┐         ┌───────────┐         ┌───────────┐
    │  AMERICAS │         │   EUROPE  │         │ ASIA-PAC  │
    │ us-east-1 │         │ eu-west-1 │         │ ap-se-1   │
    └─────┬─────┘         └─────┬─────┘         └─────┬─────┘
          │                     │                     │
          └─────────────────────┼─────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │   KUBERNETES CLUSTER  │
                    │                       │
                    │  ┌─────────────────┐  │
                    │  │    INGRESS      │  │
                    │  │   (Traefik)     │  │
                    │  └────────┬────────┘  │
                    │           │           │
                    │  ┌────────┴──────┐    │
                    │  │               │    │
                    │  ▼               ▼    │
                    │ ┌───┐  ┌───┐   ┌───┐  │
                    │ │Web│  │API│   │Bot│  │
                    │ │App│  │Svc│   │Svc│  │
                    │ │x3 │  │x3 │   │x2 │  │
                    │ └─┬─┘  └─┬─┘   └─┬─┘  │
                    │   │      │       │    │
                    └───┼──────┼───────┼────┘
                        │      │       │
                        └──────┼───────┘
                               │
                    ┌──────────▼──────────┐
                    │   CLOUD SQL (HA)    │
                    │    PostgreSQL 15    │
                    │                     │
                    │  Primary + Replica  │
                    │  Automated backups  │
                    └─────────────────────┘

+================================================================================+
|                          LATENCY TARGETS                                       |
+================================================================================+

Region              │ Target Latency │ Strategy
────────────────────┼────────────────┼────────────────────────────────────
Americas            │     < 50ms     │ Direct to us-east-1
Europe              │    < 100ms     │ Via eu-west-1 edge
Asia-Pacific        │    < 100ms     │ Via ap-southeast-1 edge
Africa              │    < 200ms     │ Via eu-west-1 (closest)
Middle East         │    < 150ms     │ Via eu-west-1

+================================================================================+
```

### Kubernetes Manifests

> **Note:** Single unified deployment - uno-app serves both frontend (SSR) and API
> from one process using Leptos + Actix-web.

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: uno-app

---
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: uno-app
  namespace: uno-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: uno-app
  template:
    metadata:
      labels:
        app: uno-app
    spec:
      containers:
      - name: uno-app
        image: gcr.io/uno-operator/uno-app:latest
        ports:
        - containerPort: 3000
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-credentials
              key: url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: redis-credentials
              key: url
        - name: LEPTOS_SITE_ADDR
          value: "0.0.0.0:3000"
        - name: LEPTOS_ENV
          value: "production"
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "200m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5

---
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: uno-app
  namespace: uno-app
spec:
  selector:
    app: uno-app
  ports:
  - port: 80
    targetPort: 3000
  type: ClusterIP

---
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: uno-ingress
  namespace: uno-app
  annotations:
    kubernetes.io/ingress.class: traefik
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - uno-operator.com
    - www.uno-operator.com
    secretName: uno-tls
  rules:
  - host: uno-operator.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: uno-app
            port:
              number: 80
```

---

## 11. Implementation Phases

### Phase Overview

```
+================================================================================+
|                         IMPLEMENTATION PHASES                                   |
|                    (Simplified App Structure)                                   |
+================================================================================+

PHASE 1: FOUNDATION
├── Project setup (Leptos + Actix-web workspace)
├── Database schema & connection pool
├── CSV import service
├── Basic API handlers structure
└── Verification: Can import CSV and query licenses

PHASE 2: CORE LICENSE FLOW
├── /licenses route - License variant selection
├── /claim/:id route - License reveal + help
├── License components (variant_card, urgency_bar, license_reveal)
├── Copy to clipboard + download functionality
└── Verification: Full claim flow works

PHASE 3: CONTENT PAGES
├── / (home) - Landing page
├── /tasks - Task categories with earnings info
├── /faq - FAQ with search and categories
├── /contact - Contact information
├── /stats - Public statistics dashboard
└── Verification: All routes render correctly

PHASE 4: SUPPORT FEATURES
├── Chatbot widget (floating + expanded)
├── Help accordion on claim page
├── Android download button
├── Localization (9 languages)
└── Verification: Users can get help without Discord

PHASE 5: STATISTICS & POLISH
├── Stats components (charts, top performers, visitor map)
├── Period toggle (daily/weekly/monthly)
├── Middleware (rate limiting, CORS, security headers)
├── Final testing & bug fixes
└── Verification: Production ready

+================================================================================+
```

### Phase 1: Foundation

```
PHASE 1: FOUNDATION
================================================================================

Project Structure Setup
───────────────────────
[ ] Initialize Leptos workspace (uno-app/Cargo.toml)
[ ] Configure Actix-web integration
[ ] Set up SCSS compilation (style/main.scss)
[ ] Create basic src/ directory structure:
    - lib.rs, app.rs, main.rs
    - routes/, components/, hooks/, api/, server/

Database Layer
──────────────
[ ] PostgreSQL connection pool (server/db/pool.rs)
[ ] Create migrations (server/db/migrations/)
[ ] Implement repositories:
    - license_repository.rs
    - variant_repository.rs
    - claim_repository.rs

CSV Import Service
──────────────────
[ ] csv_import_service.rs - Parse Unetwork CSV format
[ ] Map fields to database schema
[ ] Handle duplicate detection
[ ] Admin import endpoint (admin_handler.rs)

Basic API Structure
───────────────────
[ ] licenses_handler.rs - GET /api/v1/licenses/variants
[ ] Server function wrappers (api/licenses.rs)
[ ] Health check endpoint

GATE 1 VERIFICATION:
[ ] Can import Unetwork CSV successfully
[ ] Variants are correctly aggregated
[ ] API returns license variants
```

### Phase 2: Core License Flow

```
PHASE 2: CORE LICENSE FLOW
================================================================================

License Selection Page (/licenses)
──────────────────────────────────
[ ] routes/licenses.rs - License selection page
[ ] components/license/variant_card.rs - Display card with:
    - Split percentage
    - Estimated earnings
    - Claim button
[ ] components/license/urgency_bar.rs - Progress bar (claimed/remaining)
[ ] license_service.rs - Business logic for variants

Claim Page (/claim/:id)
───────────────────────
[ ] routes/claim.rs - License reveal page
[ ] components/license/license_reveal.rs:
    - Display license key
    - Copy to clipboard button
    - Download link (Android APK)
[ ] components/license/help_accordion.rs - Expandable help sections
[ ] components/license/android_download.rs - Play Store button
[ ] POST /api/v1/licenses/claim endpoint
    - Atomic claim operation
    - Return license key + claim token

Shared Hooks & Components
─────────────────────────
[ ] hooks/use_clipboard.rs - Clipboard operations
[ ] components/common/copy_button.rs - Copy with feedback
[ ] components/common/button.rs - Primary/secondary buttons
[ ] components/common/card.rs - Card container

GATE 2 VERIFICATION:
[ ] Can select a license variant
[ ] Claim succeeds and shows license key
[ ] Copy button works (clipboard)
[ ] Help sections expand/collapse
[ ] Works on mobile devices
```

### Phase 3: Content Pages

```
PHASE 3: CONTENT PAGES
================================================================================

Landing Page (/)
────────────────
[ ] routes/home.rs - Landing page
[ ] Hero section with value proposition
[ ] CTA to /licenses

Layout Components
─────────────────
[ ] components/layout/header.rs - Logo, nav, language selector
[ ] components/layout/footer.rs - Contact links, copyright
[ ] components/layout/nav.rs - Navigation menu

Tasks Page (/tasks)
───────────────────
[ ] routes/tasks.rs - Task categories page
[ ] components/tasks/task_card.rs - Task info card
[ ] components/tasks/device_comparison.rs - WiFi vs Phone earnings

FAQ Page (/faq)
───────────────
[ ] routes/faq.rs - Interactive FAQ page
[ ] components/faq/accordion.rs - Expandable FAQ item
[ ] components/faq/search_bar.rs - FAQ search input
[ ] components/faq/category_tabs.rs - Category filter tabs
[ ] faq_handler.rs + faq_service.rs + faq_repository.rs

Stats Page (/stats)
───────────────────
[ ] routes/stats.rs - Public dashboard (basic structure)

Contact Page (/contact)
───────────────────────
[ ] routes/contact.rs - Contact information

GATE 3 VERIFICATION:
[ ] All routes render correctly
[ ] Navigation works between pages
[ ] FAQ search filters results
[ ] Task cards display earnings info
```

### Phase 4: Support Features

```
PHASE 4: SUPPORT FEATURES
================================================================================

Chatbot Widget
──────────────
[ ] components/chatbot/chat_widget.rs - Floating widget (collapsed)
[ ] components/chatbot/chat_window.rs - Expanded chat interface
[ ] components/chatbot/message_bubble.rs - Individual message
[ ] chatbot_handler.rs + chatbot_service.rs - Intent matching
[ ] api/chatbot.rs - Server functions

Localization
────────────
[ ] hooks/use_locale.rs - Current locale state
[ ] components/common/language_selector.rs - Language dropdown
[ ] server/extractors/locale.rs - Language extraction from headers
[ ] assets/locales/*.json - Translation files:
    - en.json (English)
    - tl.json (Tagalog)
    - hi.json (Hindi)
    - sw.json (Swahili)
    - es.json (Spanish)
    - pt.json (Portuguese)
    - fr.json (French)
    - ar.json (Arabic)
    - id.json (Bahasa Indonesia)

Common Components
─────────────────
[ ] components/common/modal.rs - Modal dialog
[ ] components/common/accordion.rs - Generic accordion
[ ] components/common/loading.rs - Loading spinner/skeleton
[ ] components/common/icon.rs - Icon component

GATE 4 VERIFICATION:
[ ] Chatbot responds to basic intents
[ ] Language switching works
[ ] All UI text translates correctly
[ ] Users can get help without Discord
```

### Phase 5: Statistics & Polish

```
PHASE 5: STATISTICS & POLISH
================================================================================

Stats Components
────────────────
[ ] components/stats/top_performers.rs - Top earners podium
[ ] components/stats/earnings_chart.rs - Bar chart by task type
[ ] components/stats/period_toggle.rs - Daily/Weekly/Monthly selector
[ ] components/stats/tier_breakdown.rs - Tier distribution chart
[ ] components/stats/visitor_map.rs - Visitors by country
[ ] stats_handler.rs + stats_service.rs + stats_repository.rs

Middleware
──────────
[ ] middleware/auth_middleware.rs - Admin authentication
[ ] middleware/rate_limit_middleware.rs - Rate limiting
[ ] middleware/cors_middleware.rs - CORS handling
[ ] middleware/visitor_middleware.rs - Visitor tracking
[ ] middleware/security_middleware.rs - Security headers

Extractors & Hooks
──────────────────
[ ] server/extractors/geo.rs - IP geolocation
[ ] hooks/use_api.rs - API request hook

Final Polish
────────────
[ ] End-to-end tests (end2end/)
[ ] Mobile responsiveness testing
[ ] Performance optimization
[ ] Error handling & edge cases

GATE 5 VERIFICATION:
[ ] Stats display correctly with period toggle
[ ] Charts render on all browsers
[ ] Rate limiting works
[ ] All middleware configured
[ ] E2E tests pass
```

---

## 12. Verification Gates

### Gate Definitions

```
+================================================================================+
|                           VERIFICATION GATES                                   |
|                    (Based on CLAUDE.md: Verification Before Done)              |
+================================================================================+

GATE 1: Foundation
──────────────────
Criteria:
[ ] CSV import parses all fields correctly
[ ] Variants aggregated by split percentage
[ ] Database connection pool works
[ ] API returns license variants

Test Commands:
$ curl -X POST /api/v1/admin/import -F "file=@licenses.csv"
$ curl /api/v1/licenses/variants

Expected Results:
- Import returns count of imported/skipped
- Variants show correct counts and split percentages

────────────────────────────────────────────────────────────────────────────────

GATE 2: Core License Flow
─────────────────────────
Criteria:
[ ] User can view license variants on /licenses
[ ] Claim succeeds and redirects to /claim/:id
[ ] License key displayed on claim page
[ ] Copy button works (clipboard feedback)
[ ] Help accordion expands/collapses
[ ] Android download button visible
[ ] Mobile responsive

Manual Test:
1. Visit /licenses
2. Click "Claim" on any variant card
3. Verify redirected to /claim/:id
4. Verify license key displayed
5. Copy license key
6. Expand help sections
7. Test on mobile viewport

────────────────────────────────────────────────────────────────────────────────

GATE 3: Content Pages
─────────────────────
Criteria:
[ ] Landing page (/) renders with CTA
[ ] Tasks page shows all task categories
[ ] FAQ page loads with search and category tabs
[ ] Contact page displays contact info
[ ] Stats page structure renders
[ ] Navigation works between all pages

Manual Test:
1. Visit each route: /, /licenses, /tasks, /faq, /contact, /stats
2. Use navigation menu
3. Verify header/footer on all pages

────────────────────────────────────────────────────────────────────────────────

GATE 4: Support Features
────────────────────────
Criteria:
[ ] Chatbot widget appears (floating)
[ ] Chatbot expands and responds to intents
[ ] Language selector changes locale
[ ] All UI text translates correctly
[ ] FAQ search filters questions
[ ] No Discord/WhatsApp/Telegram links

Manual Test:
1. Click chatbot widget
2. Ask "how much can I earn"
3. Switch language to Tagalog
4. Verify UI text changes
5. Search FAQ for "earnings"

────────────────────────────────────────────────────────────────────────────────

GATE 5: Statistics & Production Ready
─────────────────────────────────────
Criteria:
[ ] Stats page shows top performers
[ ] Period toggle (daily/weekly/monthly) works
[ ] Charts render correctly
[ ] Visitor map displays
[ ] Rate limiting configured
[ ] Security headers present
[ ] E2E tests pass

Test Commands:
$ curl /api/v1/stats/public?period=weekly
$ cargo test --workspace
$ npm run test:e2e

+================================================================================+
```

### Definition of Done

```
+------------------------------------------------------------------+
|                      DEFINITION OF DONE                          |
|               (Per CLAUDE.md: "Would a staff engineer approve?") |
+------------------------------------------------------------------+

For each feature/component:

1. FUNCTIONALITY
   [ ] Meets all acceptance criteria
   [ ] Edge cases handled
   [ ] Error states handled gracefully

2. CODE QUALITY
   [ ] No warnings or errors in build
   [ ] Follows project style guide
   [ ] No hardcoded secrets or API keys
   [ ] Comments where logic is complex

3. TESTING
   [ ] Unit tests pass
   [ ] Integration tests pass
   [ ] Manual testing completed
   [ ] Cross-browser tested (Chrome, Firefox, Safari)
   [ ] Mobile responsive verified

4. DOCUMENTATION
   [ ] API endpoints documented
   [ ] Complex logic explained
   [ ] README updated if needed

5. SECURITY
   [ ] No personal data collected (per requirements)
   [ ] Rate limiting in place
   [ ] Input validation on all endpoints
   [ ] XSS/CSRF protections

6. PERFORMANCE
   [ ] Page load < 3 seconds
   [ ] API response < 500ms
   [ ] No memory leaks
   [ ] Database queries optimized

+------------------------------------------------------------------+
```

---

## Summary

This implementation plan provides:

1. **Complete database schema** matching Unetwork CSV export format
2. **Full API specification** with all required endpoints
3. **Detailed UI wireframes** for all pages and components
4. **7-step license claim wizard** specification
5. **Public statistics dashboard** design
6. **Chatbot system** without Discord dependency
7. **Localization system** for 9 languages
8. **Infrastructure architecture** with multi-region HA
9. **Phased implementation** over 14 weeks
10. **Verification gates** per CLAUDE.md principles

The plan addresses all conflicts:
- No Discord/WhatsApp/Telegram (in-app + email + phone only)
- No email/phone collection (anonymous until claim)
- Database reflects Unetwork CSV structure
- Custom chatbot module (not third-party)

---

*Document Version: 1.0*
*Created: Following CLAUDE.md Workflow Principles*
*Source: Comprehensive Analysis of /plans + Web App Requirements*
