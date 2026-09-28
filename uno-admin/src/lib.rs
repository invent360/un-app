#![recursion_limit = "512"]

pub mod app;
pub mod ui;
pub mod api;
pub mod models;
pub mod storage;
pub mod logic;
pub mod handler;
pub mod db;

#[cfg(feature = "ssr")]
pub mod repository;

#[cfg(feature = "ssr")]
pub mod ws;

#[cfg(feature = "ssr")]
pub mod utils;

// Re-export ui modules for convenience
pub use ui::pages;
pub use ui::components;
pub use ui::context;
pub use ui::state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
