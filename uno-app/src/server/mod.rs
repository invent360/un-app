//! Server-side modules (Actix-web handlers, services, repositories)

pub mod adapters;
pub mod app;
pub mod db;
pub mod geoip;
pub mod handlers;
pub mod services;
pub mod repositories;
pub mod middleware;
pub mod extractors;
pub mod secrets;
pub mod scheduler;
pub mod utils;
