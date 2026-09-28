//! UNO Web Application - License Distribution Platform
//!
//! A Leptos + Actix-web full-stack application for managing Unetwork license distribution.

// Increase recursion limit for Leptos view type complexity
#![recursion_limit = "512"]

pub mod app;
pub mod types;
pub mod config;
pub mod locales;

// Client-side modules (WASM)
pub mod routes;
pub mod components;
pub mod hooks;
pub mod features;

// Server function wrappers (used by both client and server)
pub mod api;

// Server-only modules
#[cfg(feature = "ssr")]
pub mod server;

// Re-export commonly used types
pub use types::*;

/// WASM hydration entry point
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
