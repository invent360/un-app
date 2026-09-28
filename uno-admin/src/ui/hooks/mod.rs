//! UI hooks for reusable stateful logic

#[cfg(target_arch = "wasm32")]
pub mod use_job_websocket;

#[cfg(target_arch = "wasm32")]
pub use use_job_websocket::*;
