//! Home Page API client for CMS operations.
//!
//! This module provides server functions for managing home page content,
//! leveraging the existing schema client infrastructure.

use super::section_types::*;
use super::schema_types::*;
use leptos::prelude::*;
use server_fn::codec::PostUrl;

// ========================================
// Server Functions for Home Page
// ========================================

/// Fetch home page content
#[server(GetHomePage, "/api", endpoint = "get_home_page")]
pub async fn get_home_page() -> Result<HomePageResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // List items with home_page schema
        let params = ContentItemListParams {
            schema_id: Some(HOME_PAGE_SCHEMA_ID.to_string()),
            status: None,
            search: None,
            is_featured: None,
            page: 1,
            per_page: 1,
        };

        let list = client.list_items(params).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if list.items.is_empty() {
            // Return default empty home page if none exists
            return Ok(HomePageResponse {
                id: None,
                data: HomePageData::default(),
                status: ContentItemStatus::Draft,
                version: 0,
                has_published_version: false,
            });
        }

        // Get full detail of the first (and only) item
        let detail = client.get_item(&list.items[0].id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let data = HomePageData::from_json(&detail.item.data);

        Ok(HomePageResponse {
            id: Some(detail.item.id),
            data,
            status: detail.item.status,
            version: detail.item.version,
            has_published_version: detail.item.published_version.is_some(),
        })
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Save home page content (create or update)
#[server(SaveHomePage, "/api", endpoint = "save_home_page", input = PostUrl)]
pub async fn save_home_page(
    id: Option<String>,
    data: String,  // JSON stringified HomePageData
    translations: Option<String>,  // JSON stringified translations
    change_summary: Option<String>,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Parse and validate the data
        let home_data: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid home page data: {}", e)))?;

        // Validate sections exist
        let sections = home_data.get("sections")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ServerFnError::new("Home page must have sections array"))?;

        if sections.is_empty() {
            return Err(ServerFnError::new("Home page must have at least one section"));
        }

        // Validate each section
        for section in sections {
            validate_home_section(section)?;
        }

        let translations_value = translations
            .map(|t| serde_json::from_str(&t))
            .transpose()
            .map_err(|e| ServerFnError::new(format!("Invalid translations JSON: {}", e)))?;

        let request = UpsertContentItemRequest {
            schema_id: HOME_PAGE_SCHEMA_ID.to_string(),
            slug: Some("home".to_string()),
            data: home_data,
            translations: translations_value,
            is_featured: Some(true),
            display_order: Some(1),
            change_summary,
            task_status: None,
        };

        let response = if let Some(existing_id) = id {
            // Update existing
            client.update_item(&existing_id, request).await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            // Create new
            client.create_item(request).await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        Ok(response.id)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Publish home page
#[server(PublishHomePage, "/api", endpoint = "publish_home_page")]
pub async fn publish_home_page(id: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        client.publish_item(&id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(true)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Get home page versions (for history/rollback)
#[server(GetHomePageVersions, "/api", endpoint = "get_home_page_versions")]
pub async fn get_home_page_versions(id: String) -> Result<Vec<ContentItemVersion>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let versions = client.get_item_versions(&id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(versions)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Revert home page to a specific version
#[server(RevertHomePage, "/api", endpoint = "revert_home_page")]
pub async fn revert_home_page(id: String, version: i32) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let item = client.revert_item(&id, version).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(item.id)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Create preview URL for home page content
/// Generates a signed preview token and returns the full preview URL
#[server(CreateHomePreview, "/api", endpoint = "create_home_preview")]
pub async fn create_home_preview(content_id: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use uno_api::auth::{generate_preview_token, PreviewTokenPayload};

        // Get the uno-app URL from environment
        let uno_app_url = std::env::var("UNO_APP_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        // Get the preview secret key (shared between admin and app)
        let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
            .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

        // Generate a signed preview token with content_id and schema_id
        let payload = PreviewTokenPayload::new(&content_id, HOME_PAGE_SCHEMA_ID);
        let preview_token = generate_preview_token(&payload, preview_secret.as_bytes());

        // Return preview URL pointing to home page with preview_token
        Ok(format!("{}/?preview_token={}", uno_app_url, preview_token))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

// ========================================
// Response Types
// ========================================

/// Response from get_home_page
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HomePageResponse {
    pub id: Option<String>,
    pub data: HomePageData,
    pub status: ContentItemStatus,
    pub version: i32,
    pub has_published_version: bool,
}

// ========================================
// Validation Helpers
// ========================================

#[cfg(feature = "ssr")]
fn validate_home_section(section: &serde_json::Value) -> Result<(), ServerFnError> {
    let section_type = section.get("section_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ServerFnError::new("Section must have a section_type"))?;

    match section_type {
        "hero" => {
            let headline = section.get("headline")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if headline.trim().is_empty() {
                return Err(ServerFnError::new("Hero section requires a headline"));
            }
        }
        "how_it_works" => {
            let title = section.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if title.trim().is_empty() {
                return Err(ServerFnError::new("How It Works section requires a title"));
            }
        }
        "earnings" => {
            let title = section.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if title.trim().is_empty() {
                return Err(ServerFnError::new("Earnings section requires a title"));
            }
        }
        "testimonials" => {
            let title = section.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if title.trim().is_empty() {
                return Err(ServerFnError::new("Testimonials section requires a title"));
            }
        }
        _ => {
            return Err(ServerFnError::new(format!("Unknown section type: {}", section_type)));
        }
    }

    Ok(())
}

// ========================================
// Server Function Registration
// ========================================

/// Force home server functions to be registered
/// Call this from main.rs to ensure inventory picks up the server functions
#[cfg(feature = "ssr")]
pub fn register_home_server_fns() {
    use server_fn::ServerFn;

    // Reference the URL to ensure the server function is linked
    println!("Registering home page server functions:");
    println!("  GetHomePage: {}", GetHomePage::url());
    println!("  SaveHomePage: {}", SaveHomePage::url());
    println!("  PublishHomePage: {}", PublishHomePage::url());
    println!("  GetHomePageVersions: {}", GetHomePageVersions::url());
    println!("  RevertHomePage: {}", RevertHomePage::url());
    println!("  CreateHomePreview: {}", CreateHomePreview::url());
}
