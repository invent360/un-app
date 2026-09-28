//! FAQ-related server functions

use leptos::prelude::*;
use crate::types::{FaqItemResponse, FaqCategory, FaqListResponse};

/// Get all FAQ items with optional filtering
#[server(GetFaqs, "/api")]
pub async fn get_faqs(
    query: Option<String>,
    category: Option<String>,
    locale: Option<String>,
    featured_only: Option<bool>,
) -> Result<FaqListResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::FaqSearchParams;

    let factory: Data<ServiceFactory> = extract().await?;

    let params = FaqSearchParams {
        query,
        category,
        locale,
        featured_only: featured_only.unwrap_or(false),
    };

    factory.faq_service.get_faqs(params)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get FAQ content for preview mode
/// Validates the preview token and fetches draft content from CMS
#[server(GetFaqPreview, "/api")]
pub async fn get_faq_preview(
    preview_token: String,
    locale: Option<String>,
) -> Result<FaqListResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;
    use crate::types::FaqSearchParams;
    use uno_api::auth::validate_preview_token;
    use uuid::Uuid;

    // Get preview secret from environment
    let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
        .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

    // Validate token
    let payload = validate_preview_token(&preview_token, preview_secret.as_bytes())
        .map_err(|e| ServerFnError::new(format!("Invalid preview token: {:?}", e)))?;

    // Verify this is an FAQ preview
    if payload.schema_id != "faq" {
        return Err(ServerFnError::new("Preview token is not for FAQ content"));
    }

    let factory: Data<ServiceFactory> = extract().await?;

    // Get existing FAQs from database
    let params = FaqSearchParams {
        query: None,
        category: None,
        locale: locale.clone(),
        featured_only: false,
    };
    let mut response = factory.faq_service.get_faqs(params)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Try to fetch the preview content item and add it to the list
    if let Ok(content_id) = Uuid::parse_str(&payload.content_id) {
        if let Ok(Some(detail)) = factory.content_item_service.get_by_id(content_id).await {
            // Extract FAQ data from content item - new structure with questions array
            if let Some(questions) = detail.item.data.get("questions").and_then(|v| v.as_array()) {
                // Clear existing items and replace with preview content
                response.items.clear();
                response.total = 0;

                for (idx, q) in questions.iter().enumerate() {
                    let category = q.get("category")
                        .and_then(|v| v.as_str())
                        .unwrap_or("general")
                        .to_string();
                    let question = q.get("question")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let answer = q.get("answer")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let is_featured = q.get("is_featured")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let is_visible = q.get("is_visible")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);

                    // Only include visible questions
                    if is_visible && !question.is_empty() {
                        let preview_faq = FaqItemResponse {
                            id: idx as i32,
                            category,
                            question,
                            answer,
                            is_featured,
                        };
                        response.items.push(preview_faq);
                        response.total += 1;
                    }
                }
            }
        }
    }

    Ok(response)
}

/// Search FAQ items by query string
#[server(SearchFaqs, "/api")]
pub async fn search_faqs(
    query: String,
    locale: Option<String>,
) -> Result<Vec<FaqItemResponse>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.faq_service.search_faqs(&query, locale.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get all FAQ categories with counts
#[server(GetFaqCategories, "/api")]
pub async fn get_faq_categories() -> Result<Vec<FaqCategory>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.faq_service.get_categories()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get featured FAQ items
#[server(GetFeaturedFaqs, "/api")]
pub async fn get_featured_faqs(
    locale: Option<String>,
) -> Result<Vec<FaqItemResponse>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.faq_service.get_featured(locale.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Get a single FAQ item by ID
#[server(GetFaqById, "/api")]
pub async fn get_faq_by_id(
    id: i32,
    locale: Option<String>,
) -> Result<Option<FaqItemResponse>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    factory.faq_service.get_faq_by_id(id, locale.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Register FAQ server functions explicitly
#[cfg(feature = "ssr")]
pub fn register_faq_server_fns() {
    use server_fn::ServerFn;
    println!("Registering FAQ server functions:");
    println!("  GetFaqs: {}", GetFaqs::url());
    println!("  GetFaqPreview: {}", GetFaqPreview::url());
    println!("  SearchFaqs: {}", SearchFaqs::url());
    println!("  GetFaqCategories: {}", GetFaqCategories::url());
    println!("  GetFeaturedFaqs: {}", GetFeaturedFaqs::url());
    println!("  GetFaqById: {}", GetFaqById::url());
}
