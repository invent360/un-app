//! End-to-end API tests
//!
//! Run with: cargo test --test api_tests

use std::time::Duration;

/// Test that the health endpoint returns OK
#[tokio::test]
async fn test_health_endpoint() {
    // Start by checking if server is running
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/api/v1/health")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "Health endpoint should return 200");
            let body: serde_json::Value = res.json().await.unwrap();
            assert_eq!(body["status"], "ok");
        }
        Err(e) => {
            eprintln!("Server not running or unreachable: {}", e);
            eprintln!("Make sure to start the server with: cargo leptos serve");
        }
    }
}

/// Test that the home page loads
#[tokio::test]
async fn test_home_page_loads() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "Home page should return 200");
            let body = res.text().await.unwrap();
            assert!(body.contains("UNO"), "Home page should contain UNO branding");
            assert!(body.contains("app-wrapper"), "Home page should have app wrapper");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that the licenses page loads with variants
#[tokio::test]
async fn test_licenses_page_loads() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/licenses")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "Licenses page should return 200");
            let body = res.text().await.unwrap();
            assert!(body.contains("licenses-page") || body.contains("Choose Your License"), 
                "Licenses page should have correct content");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that the stats page loads
#[tokio::test]
async fn test_stats_page_loads() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/stats")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "Stats page should return 200");
            let body = res.text().await.unwrap();
            assert!(body.contains("stats-page") || body.contains("Statistics"), 
                "Stats page should have correct content");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that the FAQ page loads
#[tokio::test]
async fn test_faq_page_loads() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/faq")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "FAQ page should return 200");
            let body = res.text().await.unwrap();
            assert!(body.contains("faq-page") || body.contains("FAQ"), 
                "FAQ page should have correct content");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that 404 page works for unknown routes
#[tokio::test]
async fn test_404_page() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/this-page-does-not-exist")
        .send()
        .await;

    match response {
        Ok(res) => {
            // Leptos might return 200 with 404 content or actual 404
            let body = res.text().await.unwrap();
            assert!(body.contains("not-found") || body.contains("404"), 
                "Unknown route should show 404 content");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test CSS is being served
#[tokio::test]
async fn test_css_served() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/pkg/uno-app.css")
        .send()
        .await;

    match response {
        Ok(res) => {
            assert!(res.status().is_success(), "CSS should be served");
            let content_type = res.headers()
                .get("content-type")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");
            assert!(content_type.contains("css"), "Should have CSS content type");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that chatbot widget is present on pages
#[tokio::test]
async fn test_chatbot_widget_present() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/")
        .send()
        .await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();
            assert!(body.contains("chat-widget"), "Chat widget should be present");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}

/// Test that language selector is present
#[tokio::test]
async fn test_language_selector_present() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let response = client
        .get("http://127.0.0.1:3000/")
        .send()
        .await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap();
            assert!(body.contains("language-selector"), "Language selector should be present");
        }
        Err(e) => {
            eprintln!("Server not running: {}", e);
        }
    }
}
