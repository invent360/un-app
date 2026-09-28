//! Integration tests for UnetworkClient.
//!
//! These tests require a valid JWT token and network access.
//!
//! # Running the tests
//!
//! ```bash
//! export UNETWORK_JWT_TOKEN="your-jwt-token"
//! cargo test --features full -- --ignored
//! ```
//!
//! # Environment Variables
//!
//! - `UNETWORK_JWT_TOKEN` - Required. JWT bearer token for authentication.
//! - `UNETWORK_API_URL` - Optional. Custom API URL (default: https://api.unityedge.io).

// This module requires both client and services features
#![cfg(all(feature = "client", feature = "services"))]

use std::sync::Arc;

use uno_api::client::UnetworkClient;
use uno_api::config::UnetworkConfig;
use uno_api::models::request::{GetAllLicenseIdsRequest, GetLicensesRequest};
use uno_api::services::unetwork::{UnetworkLicenseService, UnetworkLicenseTrait};

/// Create a test client from environment variables.
fn create_test_client() -> Option<UnetworkClient> {
    let token = std::env::var("UNETWORK_JWT_TOKEN").ok()?;
    let mut config = UnetworkConfig::new(token);

    if let Ok(url) = std::env::var("UNETWORK_API_URL") {
        config = config.with_base_url(url);
    }

    Some(UnetworkClient::new(config))
}

/// Create a test service wrapping the client.
fn create_test_service() -> Option<UnetworkLicenseService<UnetworkClient>> {
    let client = create_test_client()?;
    Some(UnetworkLicenseService::new(Arc::new(client)))
}

// ============================================================================
// Direct Client Tests
// ============================================================================

#[tokio::test]
#[ignore] // Requires UNETWORK_JWT_TOKEN env var
async fn test_get_all_license_ids() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_all_license_ids(GetAllLicenseIdsRequest::new())
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let ids = result.unwrap();
    println!("Found {} license IDs", ids.len());

    // Verify structure of returned data
    if !ids.is_empty() {
        let first = &ids[0];
        assert!(!first.id.is_empty(), "License ID should not be empty");
        println!("First license: id={}, alias={:?}, is_leased={}",
            first.id, first.alias, first.is_leased);
    }
}

#[tokio::test]
#[ignore]
async fn test_get_leased_license_ids() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_all_license_ids(GetAllLicenseIdsRequest::new().with_leased(true))
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let ids = result.unwrap();
    println!("Found {} leased license IDs", ids.len());

    // All returned should be leased
    for id in &ids {
        assert!(id.is_leased, "Expected all licenses to be leased, but {} is not", id.id);
    }
}

#[tokio::test]
#[ignore]
async fn test_get_bound_and_grouped_license_ids() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_all_license_ids(
            GetAllLicenseIdsRequest::new()
                .with_leased(true)
                .with_bound(true)
                .with_grouped(true)
        )
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let ids = result.unwrap();
    println!("Found {} leased, bound, and grouped license IDs", ids.len());
}

#[tokio::test]
#[ignore]
async fn test_get_all_license_groups() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client.get_all_license_groups().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let groups = result.unwrap();
    println!("Found {} license groups", groups.len());

    for group in &groups {
        println!(
            "Group '{}': {} licenses, {} online",
            group.name,
            group.license_count,
            group.online_count()
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_get_licenses_paginated() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_licenses(GetLicensesRequest::new().with_pagination(1, 10))
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let licenses = result.unwrap();
    println!("Found {} licenses on page 1", licenses.len());

    if let Some(first) = licenses.first() {
        println!("Total count: {:?}", first.total_count);
        println!("First license: id={}, alias={:?}, is_online={:?}",
            first.id, first.alias, first.is_online);
    }
}

#[tokio::test]
#[ignore]
async fn test_get_licenses_with_filters() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_licenses(
            GetLicensesRequest::new()
                .with_role("uno")
                .with_leased(true)
                .with_grouped(true)
                .with_bound(true)
                .with_enabled(true)
                .with_pagination(1, 20)
        )
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let licenses = result.unwrap();
    println!("Found {} filtered licenses", licenses.len());
}

#[tokio::test]
#[ignore]
async fn test_get_licenses_count() {
    let client = create_test_client().expect("UNETWORK_JWT_TOKEN not set");

    let result = client
        .get_licenses_count(GetLicensesRequest::new())
        .await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let count = result.unwrap();
    println!("Total license count: {}", count);
}

// ============================================================================
// Service Layer Tests
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_service_get_leased_license_ids() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    let result = service.get_leased_license_ids().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let ids = result.unwrap();
    println!("Service found {} leased license IDs", ids.len());
}

#[tokio::test]
#[ignore]
async fn test_service_get_active_license_ids() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    let result = service.get_active_license_ids().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let ids = result.unwrap();
    println!("Service found {} active (leased, bound, grouped) license IDs", ids.len());
}

#[tokio::test]
#[ignore]
async fn test_service_get_online_licenses() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    let result = service.get_online_licenses().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let online = result.unwrap();
    println!("Service found {} online licenses", online.len());

    // All should be online
    for license in &online {
        assert!(
            license.is_online.unwrap_or(false),
            "Expected all to be online, but {} is not",
            license.id
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_service_get_statistics() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    let result = service.get_statistics().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let stats = result.unwrap();

    println!("Statistics:");
    println!("  Total groups: {}", stats.total_groups);
    println!("  Total licenses: {}", stats.total_licenses);
    println!("  Online: {} ({:.1}%)", stats.online_count, stats.online_percentage());
    println!("  Leased: {}", stats.leased_count);
    println!("  Bound: {}", stats.bound_count);
    println!("  Average uptime: {:.1}%", stats.average_uptime_percentage());
}

#[tokio::test]
#[ignore]
async fn test_service_find_group_by_name() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    // First get all groups to find a valid name
    let groups = service.get_all_license_groups().await.unwrap();

    if let Some(first_group) = groups.first() {
        let name = &first_group.name;
        let result = service.find_group_by_name(name).await;

        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let found = result.unwrap();
        assert!(found.is_some(), "Expected to find group '{}'", name);
        assert_eq!(found.unwrap().name, *name);
        println!("Found group by name: {}", name);
    }

    // Test non-existent group
    let not_found = service.find_group_by_name("NonExistentGroup12345").await.unwrap();
    assert!(not_found.is_none(), "Expected not to find non-existent group");
}

#[tokio::test]
#[ignore]
async fn test_service_find_license_by_id() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    // First get all IDs to find a valid one
    let ids = service.get_all_license_ids(GetAllLicenseIdsRequest::new()).await.unwrap();

    if let Some(first_id) = ids.first() {
        let license_id = &first_id.id;
        let result = service.find_license_by_id(license_id).await;

        assert!(result.is_ok(), "Failed: {:?}", result.err());
        let found = result.unwrap();
        assert!(found.is_some(), "Expected to find license '{}'", license_id);
        assert_eq!(found.unwrap().id, *license_id);
        println!("Found license by ID: {}", license_id);
    }

    // Test non-existent license
    let not_found = service.find_license_by_id("0x0000000000000000000000000000000000000000000000000000000000000000").await.unwrap();
    assert!(not_found.is_none(), "Expected not to find non-existent license");
}

#[tokio::test]
#[ignore]
async fn test_service_get_underperforming_licenses() {
    let service = create_test_service().expect("UNETWORK_JWT_TOKEN not set");

    let result = service.get_underperforming_licenses().await;

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    let underperforming = result.unwrap();
    println!("Found {} underperforming licenses", underperforming.len());

    // Log some details
    for license in underperforming.iter().take(5) {
        println!(
            "  {} - uptime: {:.1}%, min required: {:.1}%",
            license.alias.as_deref().unwrap_or(&license.id),
            license.uptime_percentage(),
            license.lease_min_uptime_percentage.unwrap_or(0.0)
        );
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_missing_token_error() {
    let config = UnetworkConfig::default(); // Empty token
    let client = UnetworkClient::new(config);

    let result = client
        .get_all_license_ids(GetAllLicenseIdsRequest::new())
        .await;

    assert!(result.is_err(), "Expected error for missing token");
    match result {
        Err(uno_api::error::ApiError::Auth(_)) => {
            println!("Correctly got auth error for missing token");
        }
        Err(e) => panic!("Expected Auth error, got: {:?}", e),
        Ok(_) => panic!("Expected error, got success"),
    }
}

#[tokio::test]
#[ignore]
async fn test_invalid_token_error() {
    let config = UnetworkConfig::new("invalid-token-12345");
    let client = UnetworkClient::new(config);

    let result = client
        .get_all_license_ids(GetAllLicenseIdsRequest::new())
        .await;

    // Should get an auth error (401)
    assert!(result.is_err(), "Expected error for invalid token");
    println!("Got expected error for invalid token: {:?}", result.err());
}
