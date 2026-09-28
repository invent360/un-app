//! FAQ Page API client for CMS operations.
//!
//! This module provides server functions for managing FAQ page content,
//! leveraging the existing schema client infrastructure.

use super::faq_types::*;
use super::schema_types::*;
use leptos::prelude::*;
use server_fn::codec::PostUrl;

// ========================================
// Server Functions for FAQ Page
// ========================================

/// Fetch FAQ page content
#[server(GetFaqPage, "/api", endpoint = "get_faq_page")]
pub async fn get_faq_page() -> Result<FaqPageResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // List items with faq schema
        let params = ContentItemListParams {
            schema_id: Some(FAQ_PAGE_SCHEMA_ID.to_string()),
            status: None,
            search: None,
            is_featured: None,
            page: 1,
            per_page: 1,
        };

        let list = client.list_items(params).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if list.items.is_empty() {
            // Return default empty FAQ page if none exists
            return Ok(FaqPageResponse {
                id: None,
                data: FaqPageData::new(),
                status: ContentItemStatus::Draft,
                version: 0,
                has_published_version: false,
            });
        }

        // Get full detail of the first (and only) item
        let detail = client.get_item(&list.items[0].id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let data = FaqPageData::from_json(&detail.item.data);

        Ok(FaqPageResponse {
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

/// Save FAQ page content (create or update)
#[server(SaveFaqPage, "/api", endpoint = "save_faq_page", input = PostUrl)]
pub async fn save_faq_page(
    id: Option<String>,
    data: String,  // JSON stringified FaqPageData
    translations: Option<String>,  // JSON stringified translations
    change_summary: Option<String>,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Parse and validate the data
        let faq_data: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid FAQ data: {}", e)))?;

        // Validate questions array exists
        let questions = faq_data.get("questions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ServerFnError::new("FAQ page must have questions array"))?;

        // Validate each question
        for (i, question) in questions.iter().enumerate() {
            validate_faq_question(question, i)?;
        }

        let translations_value = translations
            .map(|t| serde_json::from_str(&t))
            .transpose()
            .map_err(|e| ServerFnError::new(format!("Invalid translations JSON: {}", e)))?;

        let request = UpsertContentItemRequest {
            schema_id: FAQ_PAGE_SCHEMA_ID.to_string(),
            slug: Some("faq".to_string()),
            data: faq_data,
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

/// Publish FAQ page
#[server(PublishFaqPage, "/api", endpoint = "publish_faq_page")]
pub async fn publish_faq_page(id: String) -> Result<bool, ServerFnError> {
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

/// Get FAQ page versions (for history/rollback)
#[server(GetFaqPageVersions, "/api", endpoint = "get_faq_page_versions")]
pub async fn get_faq_page_versions(id: String) -> Result<Vec<ContentItemVersion>, ServerFnError> {
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

/// Revert FAQ page to a specific version
#[server(RevertFaqPage, "/api", endpoint = "revert_faq_page")]
pub async fn revert_faq_page(id: String, version: i32) -> Result<String, ServerFnError> {
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

/// Create preview URL for FAQ page content
/// Generates a signed preview token and returns the full preview URL
#[server(CreateFaqPreview, "/api", endpoint = "create_faq_preview")]
pub async fn create_faq_preview(content_id: String) -> Result<String, ServerFnError> {
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
        let payload = PreviewTokenPayload::new(&content_id, FAQ_PAGE_SCHEMA_ID);
        let preview_token = generate_preview_token(&payload, preview_secret.as_bytes());

        // Return preview URL pointing to FAQ page with preview_token
        Ok(format!("{}/faq?preview_token={}", uno_app_url, preview_token))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

// ========================================
// Response Types
// ========================================

/// Response from get_faq_page
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FaqPageResponse {
    pub id: Option<String>,
    pub data: FaqPageData,
    pub status: ContentItemStatus,
    pub version: i32,
    pub has_published_version: bool,
}

// ========================================
// Validation Helpers
// ========================================

#[cfg(feature = "ssr")]
fn validate_faq_question(question: &serde_json::Value, index: usize) -> Result<(), ServerFnError> {
    // Validate question text
    let q_text = question.get("question")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if q_text.trim().is_empty() {
        return Err(ServerFnError::new(format!(
            "Question {} must have question text",
            index + 1
        )));
    }

    // Validate answer text
    let answer = question.get("answer")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if answer.trim().is_empty() {
        return Err(ServerFnError::new(format!(
            "Question {} must have an answer",
            index + 1
        )));
    }

    // Validate category
    let category = question.get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    let valid_categories = ["general", "earnings", "setup", "security"];
    if !valid_categories.contains(&category) {
        return Err(ServerFnError::new(format!(
            "Question {} has invalid category: {}. Valid categories are: {}",
            index + 1,
            category,
            valid_categories.join(", ")
        )));
    }

    Ok(())
}

// ========================================
// Server Function Registration
// ========================================

/// Force FAQ server functions to be registered
/// Call this from main.rs to ensure inventory picks up the server functions
#[cfg(feature = "ssr")]
pub fn register_faq_server_fns() {
    use server_fn::ServerFn;

    // Reference the URL to ensure the server function is linked
    println!("Registering FAQ page server functions:");
    println!("  GetFaqPage: {}", GetFaqPage::url());
    println!("  SaveFaqPage: {}", SaveFaqPage::url());
    println!("  PublishFaqPage: {}", PublishFaqPage::url());
    println!("  GetFaqPageVersions: {}", GetFaqPageVersions::url());
    println!("  RevertFaqPage: {}", RevertFaqPage::url());
    println!("  CreateFaqPreview: {}", CreateFaqPreview::url());
}
