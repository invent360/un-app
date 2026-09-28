//! HTTP client for uno-api.
//!
//! This module provides clients for communicating with APIs:
//!
//! - `UnoApiClient` - HMAC-authenticated client for uno-app
//! - `UnetworkClient` - JWT-authenticated client for Unetwork API
//!
//! # Example (UnoApiClient)
//!
//! ```ignore
//! use uno_api::client::{UnoApiClient, ClientConfig};
//! use uno_api::models::*;
//!
//! let config = ClientConfig::new(
//!     "https://api.example.com",
//!     "my_client_id",
//!     b"my_secret_key",
//! );
//!
//! let client = UnoApiClient::new(config);
//!
//! // Publish licenses
//! let request = PublishLicensesRequest::new(vec![
//!     LicenseInput { user_share: 50, operator_share: 50, duration_months: 12, ..Default::default() },
//! ]);
//! let result = client.publish_licenses(request).await?;
//! ```
//!
//! # Example (UnetworkClient)
//!
//! ```ignore
//! use uno_api::client::UnetworkClient;
//! use uno_api::config::UnetworkConfig;
//! use uno_api::models::request::GetAllLicenseIdsRequest;
//! use uno_api::services::unetwork::UnetworkLicenseTrait;
//!
//! let config = UnetworkConfig::new("your-jwt-token");
//! let client = UnetworkClient::new(config);
//!
//! let ids = client.get_all_license_ids(GetAllLicenseIdsRequest::new()).await?;
//! ```

mod uno_app_client;
mod unetwork_client;

pub use crate::config::ClientConfig;
pub use crate::config::UnetworkConfig;
pub use uno_app_client::UnoApiClient;
pub use unetwork_client::UnetworkClient;
