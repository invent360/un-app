//! UNO Web Application - License Distribution Platform
//!
//! A Leptos + Actix-web full-stack application for managing Unetwork license distribution.

// Increase recursion limit for Leptos view type complexity
#![recursion_limit = "512"]

#[cfg(all(feature = "debug-routes", not(debug_assertions)))]
compile_error!("debug-routes must not be enabled in release artifacts");

pub mod app;
pub mod config;
pub mod locales;
pub mod types;

// Client-side modules (WASM)
pub mod components;
pub mod features;
pub mod hooks;
pub mod routes;

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
