//! Server functions for guides
//!
//! Fetches guides from the schema-driven CMS (content_items table).

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use crate::server::utils::url_converter::storage_to_display_urls;

/// A single stage within a section's content
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stage {
    pub order: f64,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub videos: Vec<String>,
}

/// Content within a guide section
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionContent {
    #[serde(default)]
    pub stages: Vec<Stage>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub videos: Vec<String>,
}

/// Individual guide section in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideSection {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub cover_images: Vec<String>,
    #[serde(default)]
    pub cover_videos: Vec<String>,
    pub content: SectionContent,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    pub direction: String,
    pub is_featured: bool,
    pub is_fallback: bool,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Complexity level: "easy" or "medium"
    #[serde(default)]
    pub complexity: String,
    /// Estimated time in minutes (as string)
    #[serde(default)]
    pub estimated_time: String,
    /// Pre-requisite step indices
    #[serde(default)]
    pub pre_steps: Vec<usize>,
}

/// Response structure for guides page API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidesPageResponse {
    /// Page title (from CMS)
    pub name: String,
    /// Page description (from CMS)
    pub description: String,
    /// Content type identifier
    pub content_type: String,
    /// List of guide sections
    pub sections: Vec<GuideSection>,
    /// Whether in preview mode
    pub is_preview: bool,
}

/// Extract page info and sections from a ContentItem
#[cfg(feature = "ssr")]
fn content_item_to_page_response(
    item: &crate::types::ContentItem,
    locale: &str,
    is_preview: bool,
) -> GuidesPageResponse {
    // Get localized data (falls back to English if no translation)
    let content = item.localize(locale);

    // Determine text direction for locale
    let direction = crate::types::get_text_direction(locale).to_string();

    // Check if we're using fallback
    let is_fallback = locale != "en" && !item.has_translation(locale);

    // Extract page-level info
    let name = content.get("name")
        .or_else(|| content.get("title"))
        .and_then(|v| v.as_str())
        .unwrap_or("Guides")
        .to_string();

    let description = content.get("description")
        .or_else(|| content.get("summary"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Extract sections from the page content
    let raw_sections = content.get("sections")
        .and_then(|s| s.as_array())
        .cloned()
        .unwrap_or_default();

    // Convert each raw section to a GuideSection
    let base_slug = item.slug.clone().unwrap_or_default();
    let sections: Vec<GuideSection> = raw_sections.into_iter().enumerate().map(|(i, section)| {
        // Extract section-level fields
        let section_slug = section.get("slug")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if i == 0 { base_slug.clone() } else { format!("{}-{}", base_slug, i) }
            });

        let section_title = section.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let section_description = section.get("description")
            .or_else(|| section.get("summary"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let cover_images: Vec<String> = section.get("cover_images")
            .or_else(|| section.get("images"))  // Fallback to "images" field
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let cover_images = storage_to_display_urls(&cover_images);

        let cover_videos: Vec<String> = section.get("cover_videos")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let cover_videos = storage_to_display_urls(&cover_videos);

        let metadata = section.get("metadata")
            .and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();

        // Extract content (stages, title, body, images, videos)
        // Stages can be directly on section or inside a "content" sub-object
        let content_obj = section.get("content").cloned().unwrap_or_default();

        let stages: Vec<Stage> = section.get("stages")  // First check directly on section
            .or_else(|| section.get("steps"))  // Fallback for old "steps" field on section
            .or_else(|| content_obj.get("stages"))  // Then check inside content
            .or_else(|| content_obj.get("steps"))  // Fallback for old "steps" in content
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().map(|stage| {
                    let images: Vec<String> = stage.get("images")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default();
                    let videos: Vec<String> = stage.get("videos")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default();
                    Stage {
                        order: stage.get("order").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        title: stage.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        description: stage.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        images: storage_to_display_urls(&images),
                        videos: storage_to_display_urls(&videos),
                    }
                }).collect()
            })
            .unwrap_or_default();

        let content_title = content_obj.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let content_body = content_obj.get("body")
            .or_else(|| content_obj.get("summary"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let content_images: Vec<String> = content_obj.get("images")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let content_images = storage_to_display_urls(&content_images);

        let content_videos: Vec<String> = content_obj.get("videos")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let content_videos = storage_to_display_urls(&content_videos);

        // Extract new section-level fields
        let complexity = section.get("complexity")
            .and_then(|v| v.as_str())
            .unwrap_or("easy")
            .to_string();

        let estimated_time = section.get("estimated_time")
            .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_i64().map(|n| n.to_string())))
            .unwrap_or_default();

        let pre_steps: Vec<usize> = section.get("pre_steps")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
            .unwrap_or_default();

        GuideSection {
            id: i as i32,
            slug: section_slug,
            title: section_title,
            description: section_description,
            cover_images,
            cover_videos,
            content: SectionContent {
                stages,
                title: content_title,
                body: content_body,
                images: content_images,
                videos: content_videos,
            },
            metadata,
            direction: direction.clone(),
            is_featured: item.is_featured && i == 0,
            is_fallback,
            published_at: item.published_at,
            complexity,
            estimated_time,
            pre_steps,
        }
    }).collect();

    GuidesPageResponse {
        name,
        description,
        content_type: "guide".to_string(),
        sections,
        is_preview,
    }
}

/// Server function to get guides
#[server(GetGuides, "/api")]
pub async fn get_guides(locale: String) -> Result<GuidesPageResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // Fetch published guides from schema-driven CMS
    let items = factory.content_item_service
        .get_published_items_by_schema("guide")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Use the first guide page item (there should typically be one)
    if let Some(item) = items.first() {
        Ok(content_item_to_page_response(item, &locale, false))
    } else {
        // No guides found - return empty response
        Ok(GuidesPageResponse {
            name: "Guides".to_string(),
            description: String::new(),
            content_type: "guide".to_string(),
            sections: vec![],
            is_preview: false,
        })
    }
}

/// Server function to get guides with optional preview token
#[server(GetGuidesWithPreview, "/api")]
pub async fn get_guides_with_preview(
    locale: String,
    preview_token: Option<String>,
) -> Result<GuidesPageResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // If preview token provided, validate and fetch draft content
    if let Some(token) = preview_token {
        // Get the preview secret key (shared between admin and app)
        let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
            .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

        // Validate the preview token
        match uno_api::auth::validate_preview_token(&token, preview_secret.as_bytes()) {
            Ok(payload) => {
                // Verify schema matches
                if payload.schema_id != "guide" {
                    tracing::warn!("Preview token schema mismatch: expected 'guide', got '{}'", payload.schema_id);
                    return Err(ServerFnError::new("Invalid preview token: schema mismatch"));
                }

                // Parse content_id as UUID
                let content_id = uuid::Uuid::parse_str(&payload.content_id)
                    .map_err(|e| ServerFnError::new(format!("Invalid content ID: {}", e)))?;

                tracing::info!("Preview mode: fetching draft content_id={}", content_id);

                // Fetch the specific content item by ID (includes draft/unpublished)
                let detail = factory.content_item_service
                    .get_by_id(content_id)
                    .await
                    .map_err(|e| ServerFnError::new(e.to_string()))?;

                if let Some(detail) = detail {
                    let response = content_item_to_page_response(&detail.item, &locale, true);
                    tracing::info!("Preview mode: returning name='{}', sections={}", response.name, response.sections.len());
                    return Ok(response);
                } else {
                    return Err(ServerFnError::new("Content not found for preview"));
                }
            }
            Err(e) => {
                tracing::warn!("Invalid preview token: {:?}", e);
                // Fall through to normal published content fetch
            }
        }
    }

    // Fetch published guides from schema-driven CMS
    let items = factory.content_item_service
        .get_published_items_by_schema("guide")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    tracing::info!("get_guides_with_preview: Found {} guide items for locale={}", items.len(), locale);

    // Use the first guide page item (there should typically be one)
    if let Some(item) = items.first() {
        let response = content_item_to_page_response(item, &locale, false);
        tracing::info!("get_guides_with_preview: Returning name='{}', sections={}", response.name, response.sections.len());
        Ok(response)
    } else {
        // No guides found - return empty response
        tracing::warn!("get_guides_with_preview: No guide items found, returning defaults");
        Ok(GuidesPageResponse {
            name: "Guides".to_string(),
            description: String::new(),
            content_type: "guide".to_string(),
            sections: vec![],
            is_preview: false,
        })
    }
}

/// Register guides server functions explicitly
#[cfg(feature = "ssr")]
pub fn register_guides_server_fns() {
    use server_fn::ServerFn;
    println!("Registering guides server functions:");
    println!("  GetGuidesWithPreview: {}", GetGuidesWithPreview::url());
}
