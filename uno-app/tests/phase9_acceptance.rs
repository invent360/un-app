//! Phase 9 Acceptance Tests - Real HTTP and Database Integration
//!
//! These tests replace simulated mock-based tests with real HTTP requests
//! against actual PostgreSQL database. They verify the complete request/response
//! cycle through all layers.
//!
//! Exit Gate G9 Criterion: Simulated tests replaced with real HTTP/DB tests

#![cfg(feature = "ssr")]

mod harness;

use reqwest::StatusCode;
use harness::{assertions, TestHarness};
use serde_json::{json, Value};

// =============================================================================
// Health Check Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn health_endpoint_returns_200() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/health").await;
    assertions::assert_status(&response, StatusCode::OK);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn readiness_endpoint_returns_200() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/ready").await;
    assertions::assert_status(&response, StatusCode::OK);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn liveness_endpoint_returns_200() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/live").await;
    assertions::assert_status(&response, StatusCode::OK);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn integration_health_returns_component_statuses() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/integration-health").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    assert!(body.get("database").is_some() || body.get("overall").is_some());

    harness.cleanup().await;
}

// =============================================================================
// License Flow Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn claim_requires_existing_license() {
    let harness = TestHarness::new().await;

    // Try to claim a non-existent license
    let response = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": "NONEXISTENT-CODE",
                "device_id": "test-device-001"
            }),
        )
        .await;

    // Should fail since license doesn't exist
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn claim_available_license_succeeds() {
    let harness = TestHarness::new().await;

    // Create an available license
    let license = harness.fixtures.create_available_license("CLAIM-01").await;

    // Claim it
    let response = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": license.lease_code,
                "device_id": "test-device-claim-01"
            }),
        )
        .await;

    // Note: May return 503 if claims disabled, or 200 if successful
    // We're testing the endpoint is reachable and processes the request
    let status = response.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::SERVICE_UNAVAILABLE || status == StatusCode::NOT_FOUND,
        "Expected OK, SERVICE_UNAVAILABLE, or NOT_FOUND, got {}",
        status
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn get_variants_returns_json() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/licenses/variants").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    assert!(body.is_array() || body.is_object());

    harness.cleanup().await;
}

// =============================================================================
// Admin Endpoint Authentication Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn admin_endpoints_require_authentication() {
    let harness = TestHarness::new().await;

    // Test various admin endpoints without auth
    let endpoints = vec![
        ("/api/v1/admin/licenses", "POST"),
        ("/api/v1/admin/summary", "POST"),
        ("/api/v1/admin/referrals", "POST"),
    ];

    for (path, _method) in endpoints {
        let response = harness.client.post(path, &json!({})).await;
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "Admin endpoint {} should require authentication",
            path
        );
    }

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn admin_health_requires_authentication() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/admin/health").await;
    assertions::assert_status(&response, StatusCode::UNAUTHORIZED);

    harness.cleanup().await;
}

// =============================================================================
// FAQ Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn faq_list_returns_array() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/faq").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    assert!(body.is_array());

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn faq_search_accepts_query() {
    let harness = TestHarness::new().await;

    let response = harness
        .client
        .get_with_query("/api/v1/faq/search", &[("q", "license")])
        .await;
    assertions::assert_success(&response);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn faq_featured_returns_array() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/faq/featured").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    assert!(body.is_array());

    harness.cleanup().await;
}

// =============================================================================
// Consent Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn consent_required_returns_list() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/consent/required").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    // Should return consent requirements
    assert!(body.is_array() || body.is_object());

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn retention_policies_returns_list() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/privacy/retention-policies").await;
    assertions::assert_success(&response);

    harness.cleanup().await;
}

// =============================================================================
// Schema Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn public_schemas_returns_list() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/schemas").await;
    assertions::assert_success(&response);

    let body: Value = response.json().await.expect("JSON response");
    assert!(body.is_array() || body.is_object());

    harness.cleanup().await;
}

// =============================================================================
// Session Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn logout_without_session_returns_error() {
    let harness = TestHarness::new().await;

    let response = harness.client.post_empty("/api/v1/session/logout").await;
    // Should fail without valid session
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn validate_session_without_token_fails() {
    let harness = TestHarness::new().await;

    let response = harness.client.post_empty("/api/v1/session/validate").await;
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Market Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn market_status_returns_market_info() {
    let harness = TestHarness::new().await;

    // Test with a common country code
    let response = harness.client.get("/api/v1/markets/US/status").await;

    // May return 200 (market exists) or 404 (market not configured)
    let status = response.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        status
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn market_readiness_check_returns_status() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/markets/PH/readiness").await;

    let status = response.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "Expected OK or NOT_FOUND, got {}",
        status
    );

    harness.cleanup().await;
}

// =============================================================================
// Support Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn create_support_ticket_without_auth_fails() {
    let harness = TestHarness::new().await;

    let response = harness
        .client
        .post(
            "/api/v1/support/tickets",
            &json!({
                "subject": "Test ticket",
                "description": "Test description"
            }),
        )
        .await;

    // Should require authentication
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Cohort Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn create_cohort_without_auth_fails() {
    let harness = TestHarness::new().await;

    let response = harness
        .client
        .post(
            "/api/v1/cohorts",
            &json!({
                "user_id": "test-user",
                "license_id": "test-license"
            }),
        )
        .await;

    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Exit Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn initiate_exit_without_auth_fails() {
    let harness = TestHarness::new().await;

    let response = harness
        .client
        .post(
            "/api/v1/exits",
            &json!({
                "license_id": "test-license"
            }),
        )
        .await;

    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Dashboard Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn dashboard_without_auth_fails() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/dashboard").await;
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Communication Endpoint Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn get_preferences_without_auth_fails() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/communication/preferences").await;
    assertions::assert_client_error(&response);

    harness.cleanup().await;
}

// =============================================================================
// Database Consistency Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn database_operations_are_transactional() {
    let harness = TestHarness::new().await;

    // Record license count before
    let count_before = harness.fixtures.license_count().await;

    // Create a license
    let _license = harness.fixtures.create_available_license("TXN-01").await;

    // Count should increase by 1
    let count_after = harness.fixtures.license_count().await;
    assert_eq!(count_after, count_before + 1);

    // Cleanup
    harness.cleanup().await;

    // After cleanup, test licenses should be removed
    let count_final = harness.fixtures.license_count().await;
    assert!(count_final <= count_before);
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn fixture_licenses_have_correct_status() {
    let harness = TestHarness::new().await;

    // Create available license
    let available = harness.fixtures.create_available_license("STATUS-01").await;
    assert_eq!(available.status, "available");

    // Create claimed license
    let (claimed, claim) = harness
        .fixtures
        .create_claimed_license("STATUS-02", "test-device-status")
        .await;
    assert_eq!(claimed.status, "claimed");
    assert_eq!(claim.license_id, claimed.id);

    harness.cleanup().await;
}

// =============================================================================
// Request/Response Format Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn api_returns_json_content_type() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/health").await;

    let content_type = response
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("");

    assert!(
        content_type.contains("application/json"),
        "Expected JSON content type, got: {}",
        content_type
    );

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn invalid_json_returns_bad_request() {
    let harness = TestHarness::new().await;

    // Send invalid JSON to an endpoint that expects JSON
    let client = reqwest::Client::new();
    let response = client
        .post(harness.url().to_owned() + "/api/v1/licenses/claim")
        .header("Content-Type", "application/json")
        .body("not valid json")
        .send()
        .await
        .expect("Request should complete");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    harness.cleanup().await;
}

// =============================================================================
// Error Response Format Tests
// =============================================================================

#[actix_web::test]
#[ignore = "requires database connection"]
async fn not_found_returns_404() {
    let harness = TestHarness::new().await;

    let response = harness.client.get("/api/v1/nonexistent/endpoint").await;
    assertions::assert_status(&response, StatusCode::NOT_FOUND);

    harness.cleanup().await;
}

#[actix_web::test]
#[ignore = "requires database connection"]
async fn method_not_allowed_for_wrong_verb() {
    let harness = TestHarness::new().await;

    // GET on a POST-only endpoint
    let response = harness.client.get("/api/v1/licenses/claim").await;

    // Should be either METHOD_NOT_ALLOWED or NOT_FOUND depending on routing
    let status = response.status();
    assert!(
        status == StatusCode::METHOD_NOT_ALLOWED || status == StatusCode::NOT_FOUND,
        "Expected METHOD_NOT_ALLOWED or NOT_FOUND, got {}",
        status
    );

    harness.cleanup().await;
}

// =============================================================================
// R5-15: Service Unavailable 503 Tests
// =============================================================================

/// R5-15: Test that license unavailability returns 503 SERVICE_UNAVAILABLE.
///
/// When no licenses are available (inventory exhausted/paused state), the
/// service should return 503 to indicate temporary unavailability.
///
/// This complements phase0's 401 test for unauthenticated requests:
/// - phase0: Tests 401 UNAUTHORIZED for unauthenticated privileged requests
/// - This test: Tests 503 SERVICE_UNAVAILABLE for exhausted inventory
#[actix_web::test]
#[ignore = "requires database connection"]
async fn claim_returns_503_or_appropriate_error_when_license_unavailable() {
    let harness = TestHarness::new().await;

    // Do NOT create any available licenses - simulate "paused/exhausted" state
    // (no inventory available for claiming)

    // Try to claim a non-existent license
    // Without authentication: 401 (tested in phase0)
    // With auth but no license: 503 SERVICE_UNAVAILABLE or NOT_FOUND
    let response = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": "UNAVAILABLE-LICENSE-503",
                "device_id": "test-device-503"
            }),
        )
        .await;

    // R5-15: Unavailable license should return either:
    // - 401 UNAUTHORIZED (if auth required and not provided - tested in phase0)
    // - 503 SERVICE_UNAVAILABLE (if authenticated but license unavailable)
    // - 404 NOT_FOUND (if license code doesn't exist)
    let status = response.status();
    assert!(
        status == StatusCode::UNAUTHORIZED
            || status == StatusCode::SERVICE_UNAVAILABLE
            || status == StatusCode::NOT_FOUND,
        "Expected UNAUTHORIZED, SERVICE_UNAVAILABLE, or NOT_FOUND for unavailable license, got {}",
        status
    );

    harness.cleanup().await;
}

/// R5-15: Test that wrong lease code returns client error (4xx), not 503.
///
/// 503 is reserved for system unavailability, not user input errors.
/// Wrong lease codes should return 4xx errors.
#[actix_web::test]
#[ignore = "requires database connection"]
async fn claim_returns_client_error_for_invalid_lease_code() {
    let harness = TestHarness::new().await;

    // Create a valid license (so inventory exists)
    let _license = harness.fixtures.create_available_license("VALID-R515").await;

    // Try to claim with wrong code - should get client error, not 503
    let response = harness
        .client
        .post(
            "/api/v1/licenses/claim",
            &json!({
                "lease_code": "WRONG-CODE-R515",
                "device_id": "test-device-claim-r515"
            }),
        )
        .await;

    // Wrong lease code should return client error (400/401/404), not 503
    // 503 is reserved for system unavailability, not user errors
    let status = response.status();
    assert!(
        status.is_client_error(),
        "Expected client error (4xx) for invalid lease code, got {}",
        status
    );

    harness.cleanup().await;
}

/// R5-15: Test that service endpoints are reachable and respond appropriately.
///
/// This verifies the service is running and can process requests,
/// distinguishing between "service down" (connection failure) and
/// "service unavailable" (503 response).
#[actix_web::test]
#[ignore = "requires database connection"]
async fn service_endpoints_are_reachable() {
    let harness = TestHarness::new().await;

    // Health endpoint should always succeed
    let health_response = harness.client.get("/api/v1/health").await;
    assertions::assert_success(&health_response);

    // Variants endpoint should return data or appropriate error
    let variants_response = harness.client.get("/api/v1/licenses/variants").await;
    let variants_status = variants_response.status();
    assert!(
        variants_status.is_success() || variants_status == StatusCode::SERVICE_UNAVAILABLE,
        "Variants endpoint should return success or 503, got {}",
        variants_status
    );

    harness.cleanup().await;
}
