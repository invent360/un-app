//! Repository traits for uno-api.
//!
//! These traits define the interfaces that must be implemented by the host
//! application (e.g., uno-app) to provide database operations.

mod license_repository;

pub use license_repository::{LicenseFilters, LicenseRepository};
