//! UNO-API: Modular License Management API
//!
//! This crate provides a reusable, trait-based API for license management
//! that can be integrated into different host applications.
//!
//! # Architecture
//!
//! The crate follows a dependency injection pattern:
//!
//! - **uno-api** defines repository traits and service implementations
//! - **Host application** (e.g., uno-app) provides repository implementations
//! - **Admin applications** (e.g., unity-dashboard) use the HTTP client
//!
//! ```text
//! ┌─────────────────────┐     ┌─────────────────────────────────────────┐
//! │  unity-dashboard    │     │                uno-api                  │
//! │  (Admin App)        │     │            (This Crate)                 │
//! │                     │     │                                         │
//! │  Uses:              │     │  Provides:                              │
//! │  - UnoApiClient     │────▶│  - Repository traits (interfaces)       │
//! │    (HTTP client)    │     │  - Service implementations              │
//! │                     │     │  - HMAC authentication                  │
//! │                     │     │  - CSV parsing                          │
//! └─────────────────────┘     │                                         │
//!                             │  Receives (from host):                  │
//!                             │  - Repository implementations           │
//!                             └─────────────────────────────────────────┘
//!                                              │
//!                                              ▼
//!                             ┌─────────────────────────────────────────┐
//!                             │              uno-app                    │
//!                             │           (Host Server)                 │
//!                             │                                         │
//!                             │  Provides:                              │
//!                             │  - PostgreSQL repository implementations│
//!                             │  - HTTP endpoints                       │
//!                             │  - Database connection pool             │
//!                             └─────────────────────────────────────────┘
//! ```
//!
//! # Features
//!
//! - `services` - Service implementations (CSV parsing, validation)
//! - `auth` - HMAC request signing and verification
//! - `client` - HTTP client for admin applications
//! - `full` - All features enabled
//!
//! # Usage
//!
//! ## In Host Application (uno-app)
//!
//! ```ignore
//! use uno_api::traits::{LicenseRepository, VariantRepository};
//! use uno_api::services::LicenseAdminService;
//! use std::sync::Arc;
//!
//! // Implement the traits
//! struct PostgresLicenseRepository { /* ... */ }
//! impl LicenseRepository for PostgresLicenseRepository { /* ... */ }
//!
//! // Create service with injected repositories
//! let license_repo = Arc::new(PostgresLicenseRepository::new(pool.clone()));
//! let variant_repo = Arc::new(PostgresVariantRepository::new(pool.clone()));
//! let service = LicenseAdminService::new(license_repo, variant_repo);
//! ```
//!
//! ## In Admin Application (unity-dashboard)
//!
//! ```ignore
//! use uno_api::client::{UnoApiClient, ClientConfig};
//!
//! let config = ClientConfig::new(
//!     "https://api.example.com",
//!     "client_id",
//!     b"secret_key",
//! );
//! let client = UnoApiClient::new(config);
//!
//! let result = client.publish_licenses(request).await?;
//! ```

pub mod config;
pub mod error;
pub mod models;
pub mod privacy;
pub mod traits;

#[cfg(feature = "services")]
pub mod services;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "client")]
pub mod client;

// Re-exports for convenience
pub use error::{ApiError, AuthError, DbError};
pub use models::*;
pub use traits::*;
