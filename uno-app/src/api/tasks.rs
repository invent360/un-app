//! Server functions for tasks

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::types::LocalizedContent;

#[cfg(feature = "ssr")]
use crate::server::utils::url_converter::storage_to_display_urls;

/// Earnings tier data for a task
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskEarningsTier {
    pub name: String,
    pub min_earnings: f64,
    pub max_earnings: f64,
    pub period: String,
    #[serde(default)]
    pub features: Vec<String>,
    pub is_popular: bool,
}

/// Individual task section in the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSection {
    pub id: usize,
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub cover_images: Vec<String>,
    #[serde(default)]
    pub cover_videos: Vec<String>,
    pub task_status: String,
    /// Task type: "active" or "passive"
    #[serde(default)]
    pub task_type: String,
    pub difficulty: String,
    pub duration: String,
    #[serde(default)]
    pub earnings_tiers: Vec<TaskEarningsTier>,
    #[serde(default)]
    pub requirements: Vec<String>,
}

/// Response structure for tasks page API (mirrors GuidesPageResponse)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksPageResponse {
    /// Page title (from CMS)
    pub name: String,
    /// Page description (from CMS)
    pub description: String,
    /// Content type identifier
    pub content_type: String,
    /// List of task sections
    pub tasks: Vec<TaskSection>,
    /// Whether in preview mode
    pub is_preview: bool,
}

/// Extract page info and tasks from a ContentItem
#[cfg(feature = "ssr")]
fn content_item_to_tasks_response(
    item: &crate::types::ContentItem,
    locale: &str,
    is_preview: bool,
) -> TasksPageResponse {
    // Get localized data (falls back to English if no translation)
    let content = item.localize(locale);

    // Extract page-level info
    let name = content.get("name")
        .or_else(|| content.get("title"))
        .and_then(|v| v.as_str())
        .unwrap_or("Tasks")
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

    // Convert each raw section to a TaskSection
    let base_slug = item.slug.clone().unwrap_or_default();
    let tasks: Vec<TaskSection> = raw_sections.into_iter().enumerate().map(|(i, section)| {
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
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let cover_images: Vec<String> = section.get("cover_images")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let cover_images = storage_to_display_urls(&cover_images);

        let cover_videos: Vec<String> = section.get("cover_videos")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let cover_videos = storage_to_display_urls(&cover_videos);

        let task_status = section.get("task_status")
            .or_else(|| section.get("status"))
            .and_then(|v| v.as_str())
            .unwrap_or("active")
            .to_string();

        let task_type = section.get("task_type")
            .and_then(|v| v.as_str())
            .unwrap_or("active")
            .to_string();

        let difficulty = section.get("difficulty")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let duration = section.get("duration")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Parse earnings tiers
        let earnings_tiers = section.get("earnings_tiers")
            .or_else(|| section.get("earnings"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().map(|tier| {
                    TaskEarningsTier {
                        name: tier.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        min_earnings: tier.get("min_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        max_earnings: tier.get("max_earnings").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        period: tier.get("period").and_then(|v| v.as_str()).unwrap_or("month").to_string(),
                        features: tier.get("features")
                            .and_then(|v| v.as_array())
                            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                            .unwrap_or_default(),
                        is_popular: tier.get("is_popular").and_then(|v| v.as_bool()).unwrap_or(false),
                    }
                }).collect()
            })
            .unwrap_or_default();

        let requirements = section.get("requirements")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        TaskSection {
            id: i,
            slug: section_slug,
            title: section_title,
            description: section_description,
            cover_images,
            cover_videos,
            task_status,
            task_type,
            difficulty,
            duration,
            earnings_tiers,
            requirements,
        }
    }).collect();

    TasksPageResponse {
        name,
        description,
        content_type: "task".to_string(),
        tasks,
        is_preview,
    }
}

/// Server function to get tasks
#[server(GetTasks, "/api")]
pub async fn get_tasks(locale: String) -> Result<Vec<LocalizedContent>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;
    let tasks = factory.content_service
        .get_contents_by_type("task", &locale)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(tasks)
}

/// Server function to get tasks page with optional preview token
/// Returns TasksPageResponse with page title, description, and task sections
#[server(GetTasksWithPreview, "/api")]
pub async fn get_tasks_with_preview(
    locale: String,
    preview_token: Option<String>,
) -> Result<TasksPageResponse, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::server::app::ServiceFactory;

    let factory: Data<ServiceFactory> = extract().await?;

    // If preview token provided, validate and fetch draft content
    if let Some(token) = preview_token {
        // Get the preview secret key (shared between admin and app)
        let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
            .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

        // Validate the preview token using signed token validation
        match uno_api::auth::validate_preview_token(&token, preview_secret.as_bytes()) {
            Ok(payload) => {
                // Verify schema matches
                if payload.schema_id != "task" {
                    tracing::warn!("Preview token schema mismatch: expected 'task', got '{}'", payload.schema_id);
                    return Err(ServerFnError::new("Invalid preview token: schema mismatch"));
                }

                // Parse content_id as UUID
                let content_id = uuid::Uuid::parse_str(&payload.content_id)
                    .map_err(|e| ServerFnError::new(format!("Invalid content ID: {}", e)))?;

                tracing::info!("Preview mode: fetching draft task content_id={}", content_id);

                // Fetch the specific content item by ID (includes draft/unpublished)
                let detail = factory.content_item_service
                    .get_by_id(content_id)
                    .await
                    .map_err(|e| ServerFnError::new(e.to_string()))?;

                if let Some(detail) = detail {
                    let response = content_item_to_tasks_response(&detail.item, &locale, true);
                    tracing::info!("Preview mode: returning name='{}', tasks={}", response.name, response.tasks.len());
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

    // Normal fetch - get all published task content items
    let items = factory.content_item_service
        .get_published_items_by_schema("task")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // If we have published task content, use it
    if let Some(first_item) = items.first() {
        let response = content_item_to_tasks_response(first_item, &locale, false);
        return Ok(response);
    }

    // No published tasks - return empty response
    Ok(TasksPageResponse {
        name: "Tasks".to_string(),
        description: "".to_string(),
        content_type: "task".to_string(),
        tasks: vec![],
        is_preview: false,
    })
}

/// Register tasks server functions explicitly
#[cfg(feature = "ssr")]
pub fn register_tasks_server_fns() {
    use server_fn::ServerFn;
    println!("Registering tasks server functions:");
    println!("  GetTasksWithPreview: {}", GetTasksWithPreview::url());
}
