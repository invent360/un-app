//! Application configuration

#[cfg(feature = "ssr")]
mod settings;

#[cfg(feature = "ssr")]
pub use settings::*;
