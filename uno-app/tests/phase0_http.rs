#![cfg(feature = "ssr")]

use actix_web::{
    http::{Method, StatusCode},
    test, web, App,
};
use uno_app::server::{app::ServiceFactory, handlers::configure_api_routes};

/// Starts the actual route registration against a migrated PostgreSQL fixture.
/// A missing database is a test failure, never a skipped assertion.
#[actix_web::test]
async fn privileged_routes_deny_unauthenticated_requests_without_writes() {
    const KEY: &str = "synthetic-phase0-test-key-that-is-not-for-production";
    std::env::set_var("ADMIN_API_KEY", KEY);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL test fixture required");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("migrated PostgreSQL fixture must be available");
    let count_before: i64 = sqlx::query_scalar("SELECT count(*) FROM licenses")
        .fetch_one(&pool)
        .await
        .expect("licence table must exist");

    let factory = ServiceFactory::new(pool.clone());
    assert!(matches!(
        factory.license_service.reserve_next_available().await,
        Err(uno_app::types::AppError::LicenseUnavailable(_))
    ));
    assert!(matches!(
        factory
            .license_service
            .get_license_by_code("unowned-code")
            .await,
        Err(uno_app::types::AppError::LicenseUnavailable(_))
    ));
    let service = test::init_service(
        App::new()
            .app_data(web::Data::new(factory))
            .configure(configure_api_routes),
    )
    .await;
    for (method, path) in [
        (Method::POST, "/api/v1/admin/licenses"),
        (Method::POST, "/api/v1/admin/licenses/import"),
        (Method::POST, "/api/v1/admin/contents"),
        (Method::POST, "/api/v1/admin/summary"),
        (Method::DELETE, "/api/v1/admin/licenses"),
    ] {
        let request = test::TestRequest::default()
            .method(method.clone())
            .uri(path)
            .set_payload("{}")
            .to_request();
        let response = test::call_service(&service, request).await;
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "{method} {path}"
        );
    }
    let claim_request = test::TestRequest::post()
        .uri("/api/v1/licenses/claim")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"lease_code":"unowned-code","device_id":null}"#)
        .to_request();
    let claim_response = test::call_service(&service, claim_request).await;
    assert_eq!(claim_response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let count_after: i64 = sqlx::query_scalar("SELECT count(*) FROM licenses")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count_before, count_after);
}
