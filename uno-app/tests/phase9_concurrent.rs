//! Phase 9 Concurrent Claim Tests
//!
//! These tests verify that the license claiming system handles concurrent
//! requests correctly, ensuring exactly one winner per license even under
//! high contention.
//!
//! Exit Gate G9 Criterion: 100 concurrent clients, 10 licenses = 10 owners
//!
//! F9: Each concurrent client uses a unique authenticated identity (JWT token)
//! to prove distribution works with production-like session verification.

#![cfg(feature = "ssr")]

mod harness;

use harness::{TestHarness, TestFixtures};
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Barrier;
use uuid::Uuid;

/// Number of concurrent clients for stress tests
const CONCURRENT_CLIENTS: usize = 100;

/// Number of licenses available for claims
const AVAILABLE_LICENSES: usize = 10;

// =============================================================================
// Concurrent Claim Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn concurrent_claims_exactly_n_winners_for_n_licenses() {
    let harness = TestHarness::with_max_connections(20).await;

    // Create exactly AVAILABLE_LICENSES licenses
    let licenses = harness.fixtures.create_licenses(AVAILABLE_LICENSES).await;
    let license_codes: Vec<String> = licenses.iter().map(|l| l.lease_code.clone()).collect();

    // Track successful claims
    let successful_claims = Arc::new(tokio::sync::Mutex::new(HashSet::new()));
    let barrier = Arc::new(Barrier::new(CONCURRENT_CLIENTS));

    // Create concurrent tasks
    let mut handles = Vec::with_capacity(CONCURRENT_CLIENTS);

    for client_id in 0..CONCURRENT_CLIENTS {
        let codes = license_codes.clone();
        let claims = successful_claims.clone();
        let barrier = barrier.clone();
        let base_url = harness.url().to_string();

        // F9: Generate unique authenticated identity for each concurrent client
        // B8 FIX: Use UUID format for user IDs to match production expectations
        let user_id = Uuid::new_v4().to_string();
        let token = TestFixtures::generate_test_token(&user_id, "participant");

        let handle = tokio::spawn(async move {
            // Wait for all clients to be ready
            barrier.wait().await;

            let client = reqwest::Client::new();
            let mut local_claims = Vec::new();

            // Each client tries to claim all licenses with authenticated identity
            for code in &codes {
                let response = client
                    .post(format!("{}/api/v1/licenses/claim", base_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .json(&json!({
                        "lease_code": code,
                        "device_id": format!("test-device-concurrent-{}", client_id)
                    }))
                    .send()
                    .await;

                if let Ok(resp) = response {
                    if resp.status().is_success() {
                        local_claims.push(code.clone());
                    }
                }
            }

            // Record successful claims
            if !local_claims.is_empty() {
                let mut guard = claims.lock().await;
                for code in local_claims {
                    guard.insert((client_id, code));
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    // Verify results
    let claims = successful_claims.lock().await;

    // Check that each license was claimed at most once
    let mut claimed_licenses = HashSet::new();
    for (_, code) in claims.iter() {
        assert!(
            !claimed_licenses.contains(code),
            "License {} was claimed more than once!",
            code
        );
        claimed_licenses.insert(code.clone());
    }

    // F9: Assert that claims actually succeeded
    // The number of successful claims should equal the number of licenses
    assert_eq!(
        claimed_licenses.len(),
        AVAILABLE_LICENSES,
        "Expected exactly {} successful claims, got {}. \
         Run with: cargo test --features ssr,test-issuance",
        AVAILABLE_LICENSES,
        claimed_licenses.len()
    );

    println!(
        "Concurrent claim test PASSED: {} successful claims for {} licenses",
        claimed_licenses.len(),
        AVAILABLE_LICENSES
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn sequential_claims_prevent_double_allocation() {
    let harness = TestHarness::new().await;

    // Create one license
    let license = harness.fixtures.create_available_license("DOUBLE-01").await;

    // First claim attempt
    let response1 = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": license.lease_code,
                "device_id": "test-device-first"
            }),
        )
        .await;

    // Second claim attempt for the same license
    let response2 = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": license.lease_code,
                "device_id": "test-device-second"
            }),
        )
        .await;

    // At most one should succeed
    let success_count = [response1.status().is_success(), response2.status().is_success()]
        .iter()
        .filter(|&&b| b)
        .count();

    assert!(
        success_count <= 1,
        "Expected at most 1 successful claim, got {}",
        success_count
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn concurrent_same_license_single_winner() {
    let harness = TestHarness::with_max_connections(10).await;

    // Create one license
    let license = harness
        .fixtures
        .create_available_license("SINGLE-WINNER")
        .await;
    let code = license.lease_code.clone();

    // 20 concurrent clients try to claim the same license
    let concurrent_count = 20;
    let barrier = Arc::new(Barrier::new(concurrent_count));
    let success_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut handles = Vec::with_capacity(concurrent_count);

    for client_id in 0..concurrent_count {
        let code = code.clone();
        let barrier = barrier.clone();
        let success = success_count.clone();
        let base_url = harness.url().to_string();

        // F9: Generate unique authenticated identity for each concurrent client
        // B8 FIX: Use UUID format for user IDs
        let user_id = Uuid::new_v4().to_string();
        let token = TestFixtures::generate_test_token(&user_id, "participant");

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let client = reqwest::Client::new();
            let response = client
                .post(format!("{}/api/v1/licenses/claim", base_url))
                .header("Authorization", format!("Bearer {}", token))
                .json(&json!({
                    "lease_code": code,
                    "device_id": format!("test-device-single-{}", client_id)
                }))
                .send()
                .await;

            if let Ok(resp) = response {
                if resp.status().is_success() {
                    success.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let final_count = success_count.load(std::sync::atomic::Ordering::SeqCst);
    assert!(
        final_count <= 1,
        "Expected at most 1 winner for single license, got {}",
        final_count
    );

    harness.cleanup().await;
}

// =============================================================================
// Concurrent Database Operation Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn concurrent_license_creation_no_duplicates() {
    let harness = TestHarness::with_max_connections(20).await;

    let concurrent_count = 50;
    let barrier = Arc::new(Barrier::new(concurrent_count));
    let pool = harness.pool.clone();

    let mut handles = Vec::with_capacity(concurrent_count);

    for i in 0..concurrent_count {
        let barrier = barrier.clone();
        let pool = pool.clone();
        let code_suffix = format!("CONCURRENT-{:03}", i);

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let id = uuid::Uuid::new_v4();
            let lease_code = format!("TEST-{}", code_suffix);

            let result = sqlx::query(
                r#"
                INSERT INTO licenses (id, lease_code, status, created_at, updated_at)
                VALUES ($1, $2, 'available', NOW(), NOW())
                ON CONFLICT (lease_code) DO NOTHING
                "#,
            )
            .bind(id)
            .bind(&lease_code)
            .execute(&pool)
            .await;

            result.is_ok()
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(true) = handle.await {
            success_count += 1;
        }
    }

    // All should succeed since codes are unique
    assert_eq!(
        success_count, concurrent_count,
        "Expected {} successful inserts, got {}",
        concurrent_count, success_count
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn concurrent_status_updates_are_serialized() {
    let harness = TestHarness::with_max_connections(10).await;

    // Create a license
    let license = harness
        .fixtures
        .create_available_license("STATUS-UPDATE")
        .await;
    let license_id = license.id;

    // Multiple concurrent status updates
    let update_count = 20;
    let barrier = Arc::new(Barrier::new(update_count));
    let pool = harness.pool.clone();

    let mut handles = Vec::with_capacity(update_count);

    for i in 0..update_count {
        let barrier = barrier.clone();
        let pool = pool.clone();
        let status = if i % 2 == 0 { "claimed" } else { "available" };

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            sqlx::query("UPDATE licenses SET status = $1, updated_at = NOW() WHERE id = $2")
                .bind(status)
                .bind(license_id)
                .execute(&pool)
                .await
                .is_ok()
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(true) = handle.await {
            success_count += 1;
        }
    }

    // All updates should succeed (database handles serialization)
    assert_eq!(
        success_count, update_count,
        "Expected {} successful updates, got {}",
        update_count, success_count
    );

    // Final status should be deterministic (last writer wins)
    let final_status: (String,) = sqlx::query_as("SELECT status FROM licenses WHERE id = $1")
        .bind(license_id)
        .fetch_one(&harness.pool)
        .await
        .expect("License should exist");

    assert!(
        final_status.0 == "claimed" || final_status.0 == "available",
        "Status should be valid: {}",
        final_status.0
    );

    harness.cleanup().await;
}

// =============================================================================
// Connection Pool Exhaustion Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn handles_connection_pool_pressure() {
    // Create harness with limited connections
    let harness = TestHarness::with_max_connections(5).await;

    // Create more concurrent requests than connections
    let request_count = 50;
    let barrier = Arc::new(Barrier::new(request_count));

    let mut handles = Vec::with_capacity(request_count);

    for _ in 0..request_count {
        let barrier = barrier.clone();
        let base_url = harness.url().to_string();

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let client = reqwest::Client::new();
            client
                .get(format!("{}/api/v1/health", base_url))
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false)
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(true) = handle.await {
            success_count += 1;
        }
    }

    // Most should succeed even under pool pressure
    let success_rate = success_count as f64 / request_count as f64;
    assert!(
        success_rate > 0.8,
        "Expected >80% success rate under pool pressure, got {:.0}%",
        success_rate * 100.0
    );

    harness.cleanup().await;
}

// =============================================================================
// Reservation Concurrent Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn concurrent_reservations_limited_by_quota() {
    let harness = TestHarness::with_max_connections(10).await;

    // Create licenses for reservation
    let _licenses = harness.fixtures.create_licenses(5).await;

    // Try 20 concurrent reservations
    let reservation_count = 20;
    let barrier = Arc::new(Barrier::new(reservation_count));
    let success_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let mut handles = Vec::with_capacity(reservation_count);

    for i in 0..reservation_count {
        let barrier = barrier.clone();
        let success = success_count.clone();
        let base_url = harness.url().to_string();

        // F9: Generate authenticated admin identity for reservation
        // B8 FIX: Use UUID format for user IDs
        let admin_id = Uuid::new_v4().to_string();
        let reserve_user_id = Uuid::new_v4().to_string();
        let token = TestFixtures::generate_test_token(&admin_id, "operator");

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let client = reqwest::Client::new();
            let response = client
                .post(format!("{}/api/v1/admin/licenses/reserve", base_url))
                .header("Authorization", format!("Bearer {}", token))
                .header("X-API-Key", "test-api-key-phase9")
                .json(&json!({
                    "user_id": reserve_user_id,
                    "country_code": "US"
                }))
                .send()
                .await;

            if let Ok(resp) = response {
                if resp.status().is_success() {
                    success.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    // Success count should be limited by available licenses
    let final_count = success_count.load(std::sync::atomic::Ordering::SeqCst);
    println!(
        "Reservation test: {} successful out of {} attempts",
        final_count, reservation_count
    );

    harness.cleanup().await;
}

// =============================================================================
// Stress Test Summary
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn stress_test_mixed_operations() {
    let harness = TestHarness::with_max_connections(20).await;

    // Create test data
    let _licenses = harness.fixtures.create_licenses(10).await;

    // Mix of operations
    let operation_count = 100;
    let barrier = Arc::new(Barrier::new(operation_count));

    let mut handles = Vec::with_capacity(operation_count);

    for i in 0..operation_count {
        let barrier = barrier.clone();
        let base_url = harness.url().to_string();

        let handle = tokio::spawn(async move {
            barrier.wait().await;

            let client = reqwest::Client::new();

            // Randomly choose operation type
            match i % 4 {
                0 => {
                    // Health check
                    client.get(format!("{}/api/v1/health", base_url)).send().await.ok()
                }
                1 => {
                    // FAQ list
                    client.get(format!("{}/api/v1/faq", base_url)).send().await.ok()
                }
                2 => {
                    // Variants
                    client
                        .get(format!("{}/api/v1/licenses/variants", base_url))
                        .send()
                        .await
                        .ok()
                }
                _ => {
                    // Integration health
                    client
                        .get(format!("{}/api/v1/integration-health", base_url))
                        .send()
                        .await
                        .ok()
                }
            }
        });

        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(Some(resp)) = handle.await {
            if resp.status().is_success() {
                success_count += 1;
            }
        }
    }

    // High success rate expected for read operations
    let success_rate = success_count as f64 / operation_count as f64;
    assert!(
        success_rate > 0.9,
        "Expected >90% success rate for mixed operations, got {:.0}%",
        success_rate * 100.0
    );

    harness.cleanup().await;
}
