//! Service implementations for uno-api.
//!
//! Services contain the business logic for license management operations.
//! They use repository traits for database operations, allowing the host
//! application to provide the actual implementations.

mod allocation;
mod csv_import;
pub mod forecast;
mod license_admin;
pub mod unetwork;

pub use allocation::*;
pub use csv_import::*;
pub use forecast::{simulate, validate};
pub use license_admin::*;
