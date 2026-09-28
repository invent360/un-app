//! Content editor page with version history and translations
//! Provides a user-friendly form interface for creating and editing CMS content
//!
//! Now supports schema-driven forms via the SchemaForm component for dynamic content types.

use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map, use_query_map};
use server_fn::codec::PostUrl;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, LoadingOverlay, SpinnerSize};
use crate::components::forms::{SchemaForm, ValidationError, validate_form_data, TaskContentEditor, HomeContentEditor, FaqContentEditor, GuideContentEditor};
use crate::api::content_client::{ContentDetailResponse, ContentVersion, VersionDiff};
// ContentItemVersion is returned by get_content_item_versions
use crate::api::schema_client::{get_schema, get_schemas, get_content_item, save_content_item, get_content_item_versions, revert_content_item, publish_content_item};
use crate::context::use_user_role;
use super::{DiffViewer, TranslationEditor};

#[cfg(feature = "ssr")]
use crate::api::content_client::{ContentClient, UpsertContentRequest};

#[cfg(feature = "ssr")]
use uno_api::auth::{generate_preview_token, PreviewTokenPayload};

/// Panel styling constants (matching list.rs dark navy theme)
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 24px;";
const INPUT_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px;";
const TEXTAREA_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 100px; resize: vertical;";
const LABEL_STYLE: &str = "display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;";

/// Server function to get content detail
#[server(GetContentDetail, "/api", endpoint = "get_content_detail")]
pub async fn get_content_detail(id: i32) -> Result<ContentDetailResponse, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.get_content(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Server function to save content
#[server(SaveContent, "/api", endpoint = "save_content", input = PostUrl)]
pub async fn save_content(
    id: Option<i32>,
    content_type: String,
    slug: String,
    content: String,
    translations: Option<String>,
    change_summary: Option<String>,
) -> Result<i32, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let content_json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| ServerFnError::new(format!("Invalid content JSON: {}", e)))?;

    let translations_json: Option<serde_json::Value> = translations
        .map(|t| serde_json::from_str(&t))
        .transpose()
        .map_err(|e| ServerFnError::new(format!("Invalid translations JSON: {}", e)))?;

    let request = UpsertContentRequest {
        content_type,
        slug,
        content: content_json,
        translations: translations_json,
        display_order: None,
        is_featured: None,
        change_summary,
    };

    match id {
        Some(content_id) => {
            let response = client.update_content(content_id, request)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
            Ok(response.id)
        }
        None => {
            let response = client.create_content(request)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
            Ok(response.id)
        }
    }
}

/// Server function to import content from raw JSON
/// This is useful for importing backup JSON files directly into the CMS
#[server(ImportContentJson, "/api", endpoint = "import_content_json", input = PostUrl)]
pub async fn import_content_json(
    schema_id: String,
    slug: String,
    json_content: String,
) -> Result<i32, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let content: serde_json::Value = serde_json::from_str(&json_content)
        .map_err(|e| ServerFnError::new(format!("Invalid JSON: {}", e)))?;

    let request = UpsertContentRequest {
        content_type: schema_id,
        slug,
        content,
        translations: None,
        display_order: Some(0),
        is_featured: Some(false),
        change_summary: Some("Imported from JSON backup".to_string()),
    };

    let response = client.create_content(request)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(response.id)
}

/// Server function to submit content for review
#[server(SubmitContentForReview, "/api", endpoint = "submit_content_for_review")]
pub async fn submit_content_for_review(
    version_id: i32,
    submitted_by: String,
    notes: Option<String>,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.submit_for_review(version_id, &submitted_by, notes.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to publish content directly
#[server(PublishContentDirect, "/api", endpoint = "publish_content_direct")]
pub async fn publish_content_direct(
    content_id: i32,
    published_by: String,
    commit_message: Option<String>,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.publish_direct(content_id, &published_by, commit_message.as_deref())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to update content schedule
#[server(UpdateContentSchedule, "/api", endpoint = "update_content_schedule")]
pub async fn update_content_schedule(
    content_id: i32,
    publish_at: Option<String>,
    unpublish_at: Option<String>,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    // Parse datetime strings to chrono DateTime if provided
    let publish_dt = publish_at
        .filter(|s| !s.is_empty())
        .map(|s| chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)))
        .transpose()
        .map_err(|e| ServerFnError::new(format!("Invalid publish_at datetime: {}", e)))?;

    let unpublish_dt = unpublish_at
        .filter(|s| !s.is_empty())
        .map(|s| chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")
            .map(|dt| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)))
        .transpose()
        .map_err(|e| ServerFnError::new(format!("Invalid unpublish_at datetime: {}", e)))?;

    tracing::info!(
        "Updating schedule for content {}: publish_at={:?}, unpublish_at={:?}",
        content_id, publish_dt, unpublish_dt
    );

    client.update_schedule(content_id, publish_dt, unpublish_dt)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update schedule: {}", e)))?;

    Ok(())
}

/// Server function to create preview token
#[server(CreateContentPreview, "/api", endpoint = "create_content_preview")]
pub async fn create_content_preview(
    version_id: i32,
    created_by: String,
) -> Result<String, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let token = client.create_preview_token(version_id, &created_by, Some(24))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Return the preview URL
    let uno_app_url = std::env::var("UNO_APP_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    Ok(format!("{}/preview?token={}", uno_app_url, token.token))
}

/// Server function to get version history
#[server(GetVersionHistory, "/api", endpoint = "get_version_history")]
pub async fn get_version_history(content_id: i32) -> Result<Vec<ContentVersion>, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.get_version_history(content_id, Some(20))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Server function to revert to a version
#[server(RevertToVersion, "/api", endpoint = "revert_to_version")]
pub async fn revert_to_version(
    content_id: i32,
    version: i32,
    reverted_by: String,
) -> Result<(), ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.revert_to_version(content_id, version, &reverted_by)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// Server function to compare two versions
#[server(CompareVersions, "/api", endpoint = "compare_versions")]
pub async fn compare_versions(
    content_id: i32,
    version_a: i32,
    version_b: i32,
) -> Result<VersionDiff, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    client.compare_versions(content_id, version_a, version_b)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// Server function to generate preview URL for schema-driven content
/// This creates a preview URL that opens the content in uno-app using the actual page
/// with a preview_token parameter - no duplicate preview pages needed.
#[server(CreateSchemaContentPreview, "/api", endpoint = "create_schema_content_preview")]
pub async fn create_schema_content_preview(
    content_id: String,
    schema_id: String,
) -> Result<String, ServerFnError> {
    // Get the uno-app URL from environment
    let uno_app_url = std::env::var("UNO_APP_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    // Get the preview secret key (shared between admin and app)
    let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
        .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

    // Determine the actual page route based on schema type
    let page_route = match schema_id.as_str() {
        "guide" => "/guides",
        "task" => "/tasks",
        "faq" => "/faq",
        "home" => "/", // Home page preview goes to root
        _ => "/guides", // Default to guides
    };

    // Generate a proper signed preview token with content_id, schema_id, and expiry
    let payload = PreviewTokenPayload::new(&content_id, &schema_id);
    let preview_token = generate_preview_token(&payload, preview_secret.as_bytes());

    // Generate preview URL pointing to the ACTUAL page with preview_token
    // The page will use the same components but fetch draft content when token is valid
    Ok(format!("{}{}?preview_token={}", uno_app_url, page_route, preview_token))
}

/// Content editor page
#[component]
pub fn ContentEditorPage() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();
    let query = use_query_map();
    let role_ctx = use_user_role();

    // Check if mode=view query parameter is set
    let initial_view_mode = query.get_untracked().get("mode").map(|m| m == "view").unwrap_or(false);

    // Permission checks
    let can_publish = move || role_ctx.user.get().role.can_publish();
    let can_revert = move || role_ctx.user.get().role.can_revert();

    // Get content ID from URL (None for new content)
    // Now uses String IDs (UUIDs) for schema-driven content items
    let content_id_str = move || {
        params.get_untracked().get("id").and_then(|id| {
            if id == "new" { None } else { Some(id.clone()) }
        })
    };
    // Legacy i32 ID for backward compatibility (deprecated)
    let content_id = move || {
        params.get_untracked().get("id").and_then(|id| {
            if id == "new" { None } else { id.parse::<i32>().ok() }
        })
    };

    // State
    let (is_loading, set_is_loading) = signal(false);
    let loading_message = RwSignal::new(Option::<String>::None);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (success_message, set_success_message) = signal(Option::<String>::None);

    // Form state - Basic fields
    let (content_type, set_content_type) = signal("task".to_string());
    let (slug, set_slug) = signal(String::new());
    let (change_summary, set_change_summary) = signal(String::new());
    let (current_status, set_current_status) = signal("draft".to_string());
    let (current_version, set_current_version) = signal(1i32);
    let (version_id, set_version_id) = signal(Option::<i32>::None);

    // User-friendly content fields (converted to JSON on save)
    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (body_content, set_body_content) = signal(String::new());

    // Type-specific fields
    let (steps_text, set_steps_text) = signal(String::new()); // For tasks/guides - one step per line
    let (question, set_question) = signal(String::new()); // For FAQs
    let (answer, set_answer) = signal(String::new()); // For FAQs
    let (error_code, set_error_code) = signal(String::new()); // For errors
    let (solution, set_solution) = signal(String::new()); // For errors

    // Raw JSON mode (for advanced users)
    let (use_raw_json, set_use_raw_json) = signal(false);
    let (content_json, set_content_json) = signal("{}".to_string());
    let translations_json = RwSignal::new(String::new());

    // Schema-driven form data
    let form_data = RwSignal::new(serde_json::json!({}));
    let validation_errors = RwSignal::new(Vec::<ValidationError>::new());
    let (use_schema_form, set_use_schema_form) = signal(true); // Default to schema-driven form

    // Scheduling fields
    let (publish_at, set_publish_at) = signal(String::new()); // ISO datetime string
    let (unpublish_at, set_unpublish_at) = signal(String::new()); // ISO datetime string
    let (scheduling_enabled, set_scheduling_enabled) = signal(false);

    // View mode (read-only display) - initialized from query param ?mode=view
    let (view_mode, set_view_mode) = signal(initial_view_mode);

    // Fetch all available schemas for the content type selector
    let schemas_resource = Resource::new(
        || (),
        |_| async move {
            get_schemas().await.ok().unwrap_or_default()
        }
    );

    // Fetch schema based on content_type (for schema-driven forms)
    let schema_resource = Resource::new(
        move || content_type.get(),
        |ct| async move {
            // Try to fetch schema - if it fails, schema-driven form won't be available
            get_schema(ct).await.ok()
        }
    );

    // Build JSON from form fields
    let build_content_json = move || {
        if use_raw_json.get() {
            return content_json.get();
        }

        // If using schema-driven form, return the form_data directly
        if use_schema_form.get() {
            return serde_json::to_string(&form_data.get()).unwrap_or_else(|_| "{}".to_string());
        }

        let ct = content_type.get();
        match ct.as_str() {
            "task" => {
                let steps_str = steps_text.get();
                let steps: Vec<String> = steps_str.lines().filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect();
                serde_json::json!({
                    "title": title.get(),
                    "description": description.get(),
                    "steps": steps,
                    "body": body_content.get()
                }).to_string()
            }
            "guide" => {
                let stages_str = steps_text.get();
                let stages: Vec<String> = stages_str.lines().filter(|s| !s.trim().is_empty()).map(|s| s.to_string()).collect();
                serde_json::json!({
                    "title": title.get(),
                    "description": description.get(),
                    "sections": [{
                        "title": "Stages",
                        "stages": stages
                    }],
                    "body": body_content.get()
                }).to_string()
            }
            "faq" => {
                serde_json::json!({
                    "title": title.get(),
                    "question": question.get(),
                    "answer": answer.get()
                }).to_string()
            }
            "error" => {
                serde_json::json!({
                    "title": title.get(),
                    "error_code": error_code.get(),
                    "description": description.get(),
                    "solution": solution.get()
                }).to_string()
            }
            _ => {
                serde_json::json!({
                    "title": title.get(),
                    "description": description.get(),
                    "body": body_content.get()
                }).to_string()
            }
        }
    };

    // Parse JSON into form fields
    let parse_content_json = move |json_str: &str, _ct: &str| {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
            // Title
            if let Some(t) = val.get("title").and_then(|v| v.as_str()) {
                set_title.set(t.to_string());
            }
            // Description
            if let Some(d) = val.get("description").and_then(|v| v.as_str()) {
                set_description.set(d.to_string());
            }
            // Body
            if let Some(b) = val.get("body").and_then(|v| v.as_str()) {
                set_body_content.set(b.to_string());
            }
            // Steps (for tasks)
            if let Some(steps) = val.get("steps").and_then(|v| v.as_array()) {
                let steps_str: Vec<String> = steps.iter()
                    .filter_map(|s| s.as_str().map(|x| x.to_string()))
                    .collect();
                set_steps_text.set(steps_str.join("\n"));
            }
            // Sections with stages (for guides) - also check "steps" for backward compatibility
            if let Some(sections) = val.get("sections").and_then(|v| v.as_array()) {
                if let Some(first_section) = sections.first() {
                    if let Some(stages) = first_section.get("stages").or_else(|| first_section.get("steps")).and_then(|v| v.as_array()) {
                        let stages_str: Vec<String> = stages.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect();
                        set_steps_text.set(stages_str.join("\n"));
                    }
                }
            }
            // FAQ fields
            if let Some(q) = val.get("question").and_then(|v| v.as_str()) {
                set_question.set(q.to_string());
            }
            if let Some(a) = val.get("answer").and_then(|v| v.as_str()) {
                set_answer.set(a.to_string());
            }
            // Error fields
            if let Some(ec) = val.get("error_code").and_then(|v| v.as_str()) {
                set_error_code.set(ec.to_string());
            }
            if let Some(sol) = val.get("solution").and_then(|v| v.as_str()) {
                set_solution.set(sol.to_string());
            }

            // Also populate form_data for schema-driven mode
            form_data.set(val);
        }
    };

    // Fetch content detail if editing existing (using schema-driven API)
    let content_resource = Resource::new(
        move || content_id_str(),
        |id| async move {
            match id {
                Some(content_id) => Some(get_content_item(content_id).await),
                None => None,
            }
        }
    );

    // Update form state when content is loaded
    Effect::new(move |_| {
        if let Some(Some(Ok(detail))) = content_resource.get() {
            // Set schema/content type
            set_content_type.set(detail.item.schema_id.clone());

            // Set slug
            if let Some(s) = &detail.item.slug {
                set_slug.set(s.clone());
            }

            // Store raw JSON for advanced mode
            let json_str = serde_json::to_string_pretty(&detail.item.data).unwrap_or_default();
            set_content_json.set(json_str.clone());

            if let Some(translations) = &detail.item.translations {
                translations_json.set(serde_json::to_string_pretty(translations).unwrap_or_default());
            }

            // Set form_data for schema-driven mode
            form_data.set(detail.item.data.clone());

            // Parse JSON into user-friendly fields (for non-schema mode)
            parse_content_json(&json_str, &detail.item.schema_id);

            set_current_status.set(detail.item.status.as_str().to_string());
            set_current_version.set(detail.item.version);
        }
    });

    // Fetch version history for existing content
    let version_history_resource = Resource::new(
        move || content_id_str(),
        |id| async move {
            match id {
                Some(content_id) => {
                    match get_content_item_versions(content_id).await {
                        Ok(versions) => Some(versions),
                        Err(e) => {
                            tracing::error!("Failed to fetch version history: {}", e);
                            None
                        }
                    }
                }
                None => None,
            }
        }
    );

    // Version history panel state
    let (show_version_history, set_show_version_history) = signal(false);
    let (selected_compare_version, set_selected_compare_version) = signal(Option::<i32>::None);

    // Navigation
    let nav = navigate.clone();
    let on_back = move |_| {
        nav("/content", Default::default());
    };

    // Save handler
    let save_action = Action::new(move |_: &()| {
        let id = content_id_str();  // Use string ID for schema-driven API
        let ct = content_type.get();
        let s = slug.get();

        // Validate schema-driven form before save
        let schema_opt = schema_resource.get().flatten();
        let using_schema = use_schema_form.get();
        let current_form_data = form_data.get();

        // Build JSON from form fields (or use raw JSON if in advanced mode)
        let c = build_content_json();
        let t = translations_json.get();
        let cs = change_summary.get();

        async move {
            // Set loading state first
            set_is_loading.set(true);
            loading_message.set(Some("Validating content...".to_string()));
            set_error_message.set(None);
            set_success_message.set(None);

            // Validate slug is provided
            if s.trim().is_empty() {
                set_error_message.set(Some("Slug is required".to_string()));
                set_is_loading.set(false);
                loading_message.set(None);
                return;
            }

            // Validate if using schema-driven form
            // Skip validation for task/home/faq/guide content types since they use custom editors with different data structures
            let skip_validation = ct == "task" || ct == "home" || ct == "faq" || ct == "guide";
            if using_schema && !skip_validation {
                if let Some(schema) = schema_opt {
                    let errors = validate_form_data(&schema, &current_form_data);
                    if !errors.is_empty() {
                        validation_errors.set(errors.clone());
                        set_error_message.set(Some(format!(
                            "Validation failed: {} error(s). Please fix the highlighted fields.",
                            errors.len()
                        )));
                        set_is_loading.set(false);
                        loading_message.set(None);
                        return;
                    }
                    // Clear previous validation errors
                    validation_errors.set(Vec::new());
                }
            } else if skip_validation {
                // Clear any previous validation errors for custom editor content types
                validation_errors.set(Vec::new());
            }

            loading_message.set(Some("Saving content...".to_string()));

            // Extract task_status from the JSON data if present
            // It can be at top level, or nested in sections[0].task_status
            let task_status: Option<String> = serde_json::from_str::<serde_json::Value>(&c)
                .ok()
                .and_then(|v| {
                    // Try top-level task_status or status
                    v.get("task_status")
                        .or_else(|| v.get("status"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                        // If not found, try sections[0].task_status
                        .or_else(|| {
                            v.get("sections")
                                .and_then(|s| s.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|section| {
                                    section.get("task_status")
                                        .or_else(|| section.get("status"))
                                        .and_then(|s| s.as_str())
                                        .map(|s| s.to_string())
                                })
                        })
                })
                // Default to "active" for task content type if not found
                .or_else(|| if ct == "task" { Some("active".to_string()) } else { None });

            // Use schema-driven API (save_content_item)
            let result = save_content_item(
                id,                          // Already a String ID
                ct,                          // schema_id
                if s.is_empty() { None } else { Some(s) },  // slug as Option
                c,                           // data JSON
                if t.is_empty() { None } else { Some(t) },  // translations
                None,                        // is_featured
                None,                        // display_order
                if cs.is_empty() { None } else { Some(cs) },  // change_summary
                task_status,                 // task_status extracted from data
            ).await;

            set_is_loading.set(false);
            loading_message.set(None);

            match result {
                Ok(_new_id) => {
                    set_success_message.set(Some("Content saved successfully".to_string()));
                    set_change_summary.set(String::new());
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
        }
    });

    // Submit for review handler
    let submit_review_action = Action::new(move |_: &()| {
        let vid = version_id.get();
        let username = role_ctx.user.get().name;
        async move {
            if let Some(v_id) = vid {
                set_is_loading.set(true);
                loading_message.set(Some("Submitting for review...".to_string()));
                set_error_message.set(None);

                let result = submit_content_for_review(
                    v_id,
                    username,
                    None,
                ).await;

                set_is_loading.set(false);
                loading_message.set(None);

                match result {
                    Ok(_) => {
                        set_success_message.set(Some("Submitted for review".to_string()));
                        set_current_status.set("pending_review".to_string());
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Publish handler
    let publish_action = Action::new(move |_: &()| {
        let id = content_id();
        let username = role_ctx.user.get().name;
        async move {
            if let Some(c_id) = id {
                set_is_loading.set(true);
                loading_message.set(Some("Publishing content...".to_string()));
                set_error_message.set(None);

                let result = publish_content_direct(
                    c_id,
                    username,
                    None,
                ).await;

                set_is_loading.set(false);
                loading_message.set(None);

                match result {
                    Ok(_) => {
                        set_success_message.set(Some("Content published".to_string()));
                        set_current_status.set("published".to_string());
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Revert confirmation state
    let (revert_confirm_version, set_revert_confirm_version) = signal(Option::<i32>::None);
    let (is_reverting, set_is_reverting) = signal(false);

    // Revert action handler
    let revert_action = Action::new(move |version: &i32| {
        let ver = *version;
        let id = content_id();
        let username = role_ctx.user.get().name;
        async move {
            if let Some(c_id) = id {
                set_is_reverting.set(true);
                set_error_message.set(None);

                let result = revert_to_version(
                    c_id,
                    ver,
                    username,
                ).await;

                set_is_reverting.set(false);
                set_revert_confirm_version.set(None);

                match result {
                    Ok(_) => {
                        set_success_message.set(Some(format!("Reverted to version {}", ver)));
                        // Trigger a reload by incrementing version (will cause resource refetch)
                        // In practice, we'd want to invalidate the resource
                        #[cfg(feature = "hydrate")]
                        {
                            // Reload the page to get fresh data
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Diff modal state
    let (diff_compare_version, set_diff_compare_version) = signal(Option::<i32>::None);
    let (diff_result, set_diff_result) = signal(Option::<VersionDiff>::None);
    let (is_loading_diff, set_is_loading_diff) = signal(false);
    // Flag to indicate if diff modal is showing as revert preview
    let (is_revert_preview, set_is_revert_preview) = signal(false);

    // Diff action handler - compares selected version with current version
    let diff_action = Action::new(move |version: &i32| {
        let ver = *version;
        let id = content_id();
        let curr_ver = current_version.get();
        async move {
            if let Some(c_id) = id {
                set_is_loading_diff.set(true);
                set_error_message.set(None);

                // Compare the selected version with current version
                let result = compare_versions(c_id, ver, curr_ver).await;

                set_is_loading_diff.set(false);

                match result {
                    Ok(diff) => {
                        set_diff_result.set(Some(diff));
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Preview handler - works with both legacy (version_id) and schema-driven (content_id_str) content
    let (preview_url, set_preview_url) = signal(Option::<String>::None);
    let preview_action = Action::new(move |_: &()| {
        let vid = version_id.get();
        let content_id_string = content_id_str();
        let schema = content_type.get();
        let username = role_ctx.user.get().name;
        async move {
            // Try schema-driven preview first (for content items with string IDs)
            if let Some(c_id) = content_id_string {
                match create_schema_content_preview(c_id, schema).await {
                    Ok(url) => {
                        set_preview_url.set(Some(url));
                        return;
                    }
                    Err(e) => {
                        tracing::warn!("Schema preview failed, trying legacy: {}", e);
                    }
                }
            }
            // Fallback to legacy preview (for content with integer version IDs)
            if let Some(v_id) = vid {
                match create_content_preview(v_id, username).await {
                    Ok(url) => set_preview_url.set(Some(url)),
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            } else {
                set_error_message.set(Some("No content ID available for preview".to_string()));
            }
        }
    });

    // Schema-driven revert action
    let schema_revert_action = Action::new(move |version: &i32| {
        let ver = *version;
        let c_id = content_id_str();
        async move {
            if let Some(id) = c_id {
                set_is_reverting.set(true);
                set_is_loading.set(true);
                loading_message.set(Some(format!("Reverting to version {}...", ver)));
                set_error_message.set(None);

                let result = revert_content_item(id, ver).await;

                set_is_reverting.set(false);
                set_is_loading.set(false);
                loading_message.set(None);
                set_revert_confirm_version.set(None);

                match result {
                    Ok(_) => {
                        set_success_message.set(Some(format!("Reverted to version {}", ver)));
                        // Reload the page to get fresh data
                        #[cfg(feature = "hydrate")]
                        {
                            if let Some(window) = web_sys::window() {
                                let _ = window.location().reload();
                            }
                        }
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Schema-driven publish action
    let schema_publish_action = Action::new(move |_: &()| {
        let c_id = content_id_str();
        async move {
            if let Some(id) = c_id {
                set_is_loading.set(true);
                loading_message.set(Some("Publishing content...".to_string()));
                set_error_message.set(None);

                let result = publish_content_item(id).await;

                set_is_loading.set(false);
                loading_message.set(None);

                match result {
                    Ok(_) => {
                        set_success_message.set(Some("Content published successfully".to_string()));
                        set_current_status.set("published".to_string());
                    }
                    Err(e) => set_error_message.set(Some(e.to_string())),
                }
            }
        }
    });

    // Use content_id_str for schema-driven content (UUID strings)
    let is_new = move || content_id_str().is_none();

    // Compute title based on mode
    let header_title = move || {
        if content_id().is_none() {
            "New Content".to_string()
        } else if view_mode.get() {
            "View Content".to_string()
        } else {
            "Edit Content".to_string()
        }
    };

    view! {
        <div>
            // Full-page loading overlay for save/publish operations
            <LoadingOverlay
                visible=Signal::derive(move || is_loading.get())
                message=Signal::derive(move || loading_message.get())
            />

            <Header
                title=header_title()
                show_search=false
            />

            <div class="px-4 py-4 space-y-4">
                // Back button and status
                <div class="flex items-center justify-between">
                    <button
                        class="flex items-center justify-center w-10 h-10 rounded-lg transition"
                        style="color: #06b6d4; background: rgba(6, 182, 212, 0.1);"
                        on:click=on_back
                        title="Back to Content"
                    >
                        <Icon name=IconName::ArrowBigLeftLine size=24 />
                    </button>

                    {move || (!is_new()).then(|| {
                        let status = current_status.get();
                        let status_class = match status.as_str() {
                            "published" => "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400",
                            "approved" => "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400",
                            "pending_review" => "bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400",
                            _ => "bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400",
                        };
                        view! {
                            <div class="flex items-center gap-3">
                                <span class="text-sm text-slate-500 dark:text-slate-400">
                                    {"Version "}{current_version.get()}
                                </span>
                                <span class={format!("px-3 py-1 rounded-full text-sm font-medium {}", status_class)}>
                                    {format_status(&status)}
                                </span>
                            </div>
                        }
                    })}
                </div>

                // Toast Messages
                {move || error_message.get().map(|msg| view! {
                    <div style="
                        padding: 16px 20px;
                        border-radius: 8px;
                        background: linear-gradient(135deg, #991b1b 0%, #7f1d1d 100%);
                        border: 1px solid #dc2626;
                        color: #fecaca;
                        font-size: 14px;
                        font-weight: 500;
                        display: flex;
                        align-items: center;
                        gap: 12px;
                        box-shadow: 0 4px 12px rgba(220, 38, 38, 0.3);
                    ">
                        <span style="font-size: 18px;">"✕"</span>
                        {msg}
                    </div>
                })}

                {move || success_message.get().map(|msg| view! {
                    <div style="
                        padding: 16px 20px;
                        border-radius: 8px;
                        background: linear-gradient(135deg, #166534 0%, #14532d 100%);
                        border: 1px solid #22c55e;
                        color: #bbf7d0;
                        font-size: 14px;
                        font-weight: 500;
                        display: flex;
                        align-items: center;
                        gap: 12px;
                        box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
                    ">
                        <span style="font-size: 18px;">"✓"</span>
                        {msg}
                    </div>
                })}

                // Preview URL popup - fixed position for visibility
                <Show when=move || preview_url.get().is_some()>
                    {move || {
                        let url = preview_url.get().unwrap_or_default();
                        let url_for_link = url.clone();
                        view! {
                            <div style="position: fixed; top: 80px; right: 20px; z-index: 1000; max-width: 500px; background: linear-gradient(135deg, #1e3a5f 0%, #0f172a 100%); border: 1px solid #06b6d4; border-radius: 12px; padding: 16px; box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);">
                                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
                                    <span style="font-weight: 600; color: #22d3ee; font-size: 14px;">"Preview Link Generated"</span>
                                    <button
                                        style="background: none; border: none; color: #64748b; cursor: pointer; padding: 4px;"
                                        on:click=move |_| set_preview_url.set(None)
                                    >
                                        <Icon name=IconName::Close size=18 />
                                    </button>
                                </div>
                                <div style="background: #0f172a; border-radius: 8px; padding: 12px; margin-bottom: 12px;">
                                    <code style="color: #94a3b8; font-size: 12px; word-break: break-all;">
                                        {url}
                                    </code>
                                </div>
                                <div style="display: flex; gap: 8px;">
                                    <a
                                        href={url_for_link}
                                        target="_blank"
                                        style="flex: 1; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 10px 16px; background: #06b6d4; color: white; border-radius: 6px; text-decoration: none; font-weight: 500; font-size: 13px;"
                                    >
                                        <Icon name=IconName::Eye size=16 />
                                        "Open Preview"
                                    </a>
                                    <button
                                        style="padding: 10px 16px; background: #334155; color: #e2e8f0; border: none; border-radius: 6px; font-weight: 500; font-size: 13px; cursor: pointer;"
                                        on:click=move |_| set_preview_url.set(None)
                                    >
                                        "Close"
                                    </button>
                                </div>
                            </div>
                        }
                    }}
                </Show>

                // View mode banner
                {move || view_mode.get().then(|| view! {
                    <div style="
                        padding: 12px 20px;
                        border-radius: 8px;
                        background: linear-gradient(135deg, #1e3a5f 0%, #1e293b 100%);
                        border: 1px solid #06b6d4;
                        color: #06b6d4;
                        font-size: 14px;
                        font-weight: 500;
                        display: flex;
                        align-items: center;
                        gap: 12px;
                    ">
                        <Icon name=IconName::Eye size=18 />
                        "View Mode - Content is read-only. Click Edit to make changes."
                    </div>
                })}

                // Loading state
                <Suspense fallback=move || view! {
                    <div style="display: flex; align-items: center; justify-content: center; padding: 48px; background: #0f172a; border: 1px solid #1e293b; border-radius: 12px;">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {move || {
                        // For new content, show the form immediately
                        // For existing content, wait for the resource
                        let show_form = is_new() || content_resource.get().map(|r| r.is_some()).unwrap_or(false);

                        if !show_form {
                            view! {
                                <div style="display: flex; align-items: center; justify-content: center; padding: 48px;">
                                    <ProgressSpinner size=SpinnerSize::Default />
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-4">
                                    // Main editor (full width)
                                    <div class="space-y-4">
                                        // Basic info - Content Type, Slug, Title
                                        <div style=PANEL_STYLE>
                                            <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 20px;">
                                                "Basic Information"
                                            </h3>

                                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 16px;">
                                                <div>
                                                    <label style=LABEL_STYLE>"Content Type"</label>
                                                    <select
                                                        style=INPUT_STYLE
                                                        prop:value=move || content_type.get()
                                                        on:change=move |ev| {
                                                            let new_type = event_target_value(&ev);
                                                            set_content_type.set(new_type);
                                                            // Reset form data when type changes
                                                            form_data.set(serde_json::json!({}));
                                                            // Keep schema-driven form enabled (it's the default)
                                                        }
                                                        disabled=move || !is_new() || view_mode.get()
                                                    >
                                                        // Dynamic schema options
                                                        {move || {
                                                            schemas_resource.get().map(|schemas| {
                                                                if schemas.is_empty() {
                                                                    // Fallback if no schemas loaded yet
                                                                    view! {
                                                                        <option value="task">"Task"</option>
                                                                        <option value="guide">"Guide"</option>
                                                                        <option value="faq">"FAQ"</option>
                                                                        <option value="error">"Error"</option>
                                                                    }.into_any()
                                                                } else {
                                                                    schemas.into_iter().map(|schema| {
                                                                        let schema_id = schema.id.clone();
                                                                        let schema_name = schema.name.clone();
                                                                        view! {
                                                                            <option value=schema_id>{schema_name}</option>
                                                                        }
                                                                    }).collect_view().into_any()
                                                                }
                                                            })
                                                        }}
                                                    </select>
                                                </div>

                                                <div>
                                                    <label style=LABEL_STYLE>"Slug " <span style="color: #ef4444;">"*"</span></label>
                                                    <input
                                                        type="text"
                                                        style=INPUT_STYLE
                                                        placeholder="my-content-slug"
                                                        prop:value=move || slug.get()
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            set_slug.set(val);
                                                        }
                                                        disabled=move || view_mode.get()
                                                        required=true
                                                    />
                                                </div>
                                            </div>

                                            // Title field - hidden when using schema-driven form (schema includes title)
                                            {move || {
                                                if !use_schema_form.get() {
                                                    view! {
                                                        <div>
                                                            <label style=LABEL_STYLE>"Title *"</label>
                                                            <input
                                                                type="text"
                                                                style=INPUT_STYLE
                                                                placeholder="Enter a descriptive title..."
                                                                prop:value=move || title.get()
                                                                on:input=move |ev| set_title.set(event_target_value(&ev))
                                                                disabled=move || view_mode.get()
                                                            />
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            }}
                                        </div>

                                        // Content fields - Dynamic based on content type or schema
                                        <div style=PANEL_STYLE>
                                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px;">
                                                <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                                                    "Content"
                                                </h3>
                                                <div style="display: flex; align-items: center; gap: 16px;">
                                                    // Schema-driven toggle (only if schema available and not in view mode)
                                                    {move || {
                                                        let has_schema = schema_resource.get().flatten().is_some();
                                                        let in_view = view_mode.get();
                                                        (has_schema && !in_view).then(|| view! {
                                                            <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; color: #06b6d4; cursor: pointer;">
                                                                <input
                                                                    type="checkbox"
                                                                    prop:checked=move || use_schema_form.get()
                                                                    on:change=move |ev| set_use_schema_form.set(event_target_checked(&ev))
                                                                    style="width: 16px; height: 16px; accent-color: #06b6d4;"
                                                                />
                                                                "Schema-driven form"
                                                            </label>
                                                        })
                                                    }}
                                                    {move || (!view_mode.get()).then(|| view! {
                                                        <label style="display: flex; align-items: center; gap: 8px; font-size: 13px; color: #94a3b8; cursor: pointer;">
                                                            <input
                                                                type="checkbox"
                                                                prop:checked=move || use_raw_json.get()
                                                                on:change=move |ev| {
                                                                    set_use_raw_json.set(event_target_checked(&ev));
                                                                    // When switching to raw JSON, sync from form_data
                                                                    if event_target_checked(&ev) && use_schema_form.get() {
                                                                        set_content_json.set(
                                                                            serde_json::to_string_pretty(&form_data.get()).unwrap_or_else(|_| "{}".to_string())
                                                                        );
                                                                    }
                                                                }
                                                                style="width: 16px; height: 16px;"
                                                            />
                                                            "Advanced (Raw JSON)"
                                                        </label>
                                                    })}
                                                </div>
                                            </div>

                                            // Show appropriate form based on mode
                                            {move || if use_raw_json.get() {
                                                // Raw JSON mode
                                                view! {
                                                    <div>
                                                        <textarea
                                                            style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 12px; width: 100%; color: #e2e8f0; font-family: monospace; font-size: 13px; min-height: 300px; resize: vertical;"
                                                            placeholder="{}"
                                                            prop:value=move || content_json.get()
                                                            on:input=move |ev| set_content_json.set(event_target_value(&ev))
                                                            disabled=move || view_mode.get()
                                                        />
                                                        <p style="font-size: 12px; color: #64748b; margin-top: 8px;">
                                                            {move || if view_mode.get() {
                                                                "Read-only mode. Click Edit to make changes."
                                                            } else {
                                                                "Enter content as valid JSON. Required fields depend on content type."
                                                            }}
                                                        </p>
                                                    </div>
                                                }.into_any()
                                            } else if use_schema_form.get() {
                                                // Schema-driven form mode
                                                let ct = content_type.get();

                                                // Use TaskContentEditor for task content type (matches guide form layout)
                                                if ct == "task" {
                                                    let is_readonly = view_mode.get();
                                                    let errors_signal = Signal::derive(move || validation_errors.get());
                                                    view! {
                                                        <TaskContentEditor
                                                            data=form_data
                                                            errors=errors_signal
                                                            read_only=is_readonly
                                                        />
                                                    }.into_any()
                                                } else if ct == "home" {
                                                    // Use HomeContentEditor for home content type (uses modal for sections)
                                                    let is_readonly = view_mode.get();
                                                    let errors_signal = Signal::derive(move || validation_errors.get());
                                                    view! {
                                                        <HomeContentEditor
                                                            data=form_data
                                                            errors=errors_signal
                                                            read_only=is_readonly
                                                        />
                                                    }.into_any()
                                                } else if ct == "faq" {
                                                    // Use FaqContentEditor for FAQ content type (uses modal for questions)
                                                    let is_readonly = view_mode.get();
                                                    view! {
                                                        <FaqContentEditor
                                                            data=form_data
                                                            read_only=is_readonly
                                                        />
                                                    }.into_any()
                                                } else if ct == "guide" {
                                                    // Use GuideContentEditor for guide content type (uses modal for sections)
                                                    let is_readonly = view_mode.get();
                                                    let errors_signal = Signal::derive(move || validation_errors.get());
                                                    view! {
                                                        <GuideContentEditor
                                                            data=form_data
                                                            errors=errors_signal
                                                            read_only=is_readonly
                                                        />
                                                    }.into_any()
                                                } else if let Some(Some(schema)) = schema_resource.get() {
                                                    let schema_clone = schema.clone();
                                                    let errors_signal = Signal::derive(move || validation_errors.get());
                                                    let is_readonly = view_mode.get();
                                                    view! {
                                                        <SchemaForm
                                                            schema=schema_clone
                                                            data=form_data
                                                            errors=errors_signal
                                                            read_only=is_readonly
                                                        />
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div style="display: flex; align-items: center; justify-content: center; padding: 40px;">
                                                            <ProgressSpinner size=SpinnerSize::Small />
                                                        </div>
                                                    }.into_any()
                                                }
                                            } else {
                                                // Legacy user-friendly form mode
                                                let ct = content_type.get();
                                                let readonly_wrapper_style = if view_mode.get() {
                                                    "pointer-events: none; opacity: 0.8;"
                                                } else {
                                                    ""
                                                };
                                                match ct.as_str() {
                                                    "task" | "guide" => view! {
                                                        <div style=format!("display: flex; flex-direction: column; gap: 16px; {}", readonly_wrapper_style)>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Description"</label>
                                                                <textarea
                                                                    style=TEXTAREA_STYLE
                                                                    placeholder="Briefly describe what this content is about..."
                                                                    prop:value=move || description.get()
                                                                    on:input=move |ev| set_description.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Steps (one per line)"</label>
                                                                <textarea
                                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 150px; resize: vertical;"
                                                                    placeholder="Step 1: Do something\nStep 2: Do another thing\nStep 3: Complete the task"
                                                                    prop:value=move || steps_text.get()
                                                                    on:input=move |ev| set_steps_text.set(event_target_value(&ev))
                                                                />
                                                                <p style="font-size: 12px; color: #64748b; margin-top: 6px;">
                                                                    "Enter each step on a new line"
                                                                </p>
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Additional Content (optional)"</label>
                                                                <textarea
                                                                    style=TEXTAREA_STYLE
                                                                    placeholder="Any additional notes, tips, or content..."
                                                                    prop:value=move || body_content.get()
                                                                    on:input=move |ev| set_body_content.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    }.into_any(),
                                                    "faq" => view! {
                                                        <div style=format!("display: flex; flex-direction: column; gap: 16px; {}", readonly_wrapper_style)>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Question *"</label>
                                                                <textarea
                                                                    style=TEXTAREA_STYLE
                                                                    placeholder="What is the frequently asked question?"
                                                                    prop:value=move || question.get()
                                                                    on:input=move |ev| set_question.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Answer *"</label>
                                                                <textarea
                                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 200px; resize: vertical;"
                                                                    placeholder="Provide a clear, helpful answer to the question..."
                                                                    prop:value=move || answer.get()
                                                                    on:input=move |ev| set_answer.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    }.into_any(),
                                                    "error" => view! {
                                                        <div style=format!("display: flex; flex-direction: column; gap: 16px; {}", readonly_wrapper_style)>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Error Code"</label>
                                                                <input
                                                                    type="text"
                                                                    style=INPUT_STYLE
                                                                    placeholder="e.g., E001, CONNECTION_FAILED"
                                                                    prop:value=move || error_code.get()
                                                                    on:input=move |ev| set_error_code.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Description"</label>
                                                                <textarea
                                                                    style=TEXTAREA_STYLE
                                                                    placeholder="Explain what this error means..."
                                                                    prop:value=move || description.get()
                                                                    on:input=move |ev| set_description.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Solution"</label>
                                                                <textarea
                                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 150px; resize: vertical;"
                                                                    placeholder="How to fix or resolve this error..."
                                                                    prop:value=move || solution.get()
                                                                    on:input=move |ev| set_solution.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    }.into_any(),
                                                    _ => view! {
                                                        <div style=format!("display: flex; flex-direction: column; gap: 16px; {}", readonly_wrapper_style)>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Description"</label>
                                                                <textarea
                                                                    style=TEXTAREA_STYLE
                                                                    prop:value=move || description.get()
                                                                    on:input=move |ev| set_description.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                            <div>
                                                                <label style=LABEL_STYLE>"Body"</label>
                                                                <textarea
                                                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; width: 100%; color: #e2e8f0; font-size: 14px; min-height: 200px; resize: vertical;"
                                                                    prop:value=move || body_content.get()
                                                                    on:input=move |ev| set_body_content.set(event_target_value(&ev))
                                                                />
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }}
                                        </div>

                                        // Translations - Per-locale editor (hidden in view mode)
                                        {move || (!view_mode.get()).then(|| view! {
                                            <TranslationEditor
                                                base_content=Signal::derive(move || build_content_json())
                                                translations=translations_json
                                                content_type=Signal::derive(move || content_type.get())
                                            />
                                        })}

                                        // Change summary (hidden in view mode)
                                        {move || (!view_mode.get()).then(|| view! {
                                            <div style=PANEL_STYLE>
                                                <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">
                                                    "Change Summary"
                                                </h3>
                                                <input
                                                    type="text"
                                                    style=INPUT_STYLE
                                                    placeholder="Briefly describe what changed (for version history)..."
                                                    prop:value=move || change_summary.get()
                                                    on:input=move |ev| set_change_summary.set(event_target_value(&ev))
                                                />
                                            </div>
                                        })}

                                        // Scheduling section (hidden in view mode)
                                        {move || (!view_mode.get()).then(|| view! {
                                        <div style=PANEL_STYLE>
                                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                                                <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                                                    "Scheduling"
                                                </h3>
                                                <label style="display: flex; align-items: center; gap: 8px; cursor: pointer;">
                                                    <input
                                                        type="checkbox"
                                                        style="width: 16px; height: 16px; accent-color: #06b6d4;"
                                                        prop:checked=move || scheduling_enabled.get()
                                                        on:change=move |ev| set_scheduling_enabled.set(event_target_checked(&ev))
                                                    />
                                                    <span style="font-size: 13px; color: #94a3b8;">"Enable scheduling"</span>
                                                </label>
                                            </div>

                                            {move || scheduling_enabled.get().then(|| view! {
                                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                                                    <div>
                                                        <label style=LABEL_STYLE>"Publish At"</label>
                                                        <input
                                                            type="datetime-local"
                                                            style=INPUT_STYLE
                                                            prop:value=move || publish_at.get()
                                                            on:input=move |ev| set_publish_at.set(event_target_value(&ev))
                                                        />
                                                        <p style="font-size: 12px; color: #64748b; margin-top: 4px;">
                                                            "Content will be automatically published at this time"
                                                        </p>
                                                    </div>
                                                    <div>
                                                        <label style=LABEL_STYLE>"Unpublish At"</label>
                                                        <input
                                                            type="datetime-local"
                                                            style=INPUT_STYLE
                                                            prop:value=move || unpublish_at.get()
                                                            on:input=move |ev| set_unpublish_at.set(event_target_value(&ev))
                                                        />
                                                        <p style="font-size: 12px; color: #64748b; margin-top: 4px;">
                                                            "Content will be automatically unpublished at this time"
                                                        </p>
                                                    </div>
                                                </div>

                                                {move || {
                                                    let pub_at = publish_at.get();
                                                    let unpub_at = unpublish_at.get();
                                                    (!pub_at.is_empty() || !unpub_at.is_empty()).then(|| {
                                                        view! {
                                                            <div style="margin-top: 12px; padding: 12px; background: #1e3a5f; border-radius: 8px; border-left: 4px solid #06b6d4;">
                                                                <p style="font-size: 13px; color: #94a3b8;">
                                                                    <Icon name=IconName::Clock size=14 />
                                                                    " Schedule summary: "
                                                                    {if !pub_at.is_empty() {
                                                                        format!("Publish at {}", pub_at.replace("T", " "))
                                                                    } else {
                                                                        "No publish scheduled".to_string()
                                                                    }}
                                                                    {if !unpub_at.is_empty() {
                                                                        format!(" • Unpublish at {}", unpub_at.replace("T", " "))
                                                                    } else {
                                                                        String::new()
                                                                    }}
                                                                </p>
                                                            </div>
                                                        }
                                                    })
                                                }}
                                            })}

                                            {move || (!scheduling_enabled.get()).then(|| view! {
                                                <p style="font-size: 13px; color: #64748b;">
                                                    "Enable scheduling to set automatic publish and unpublish times for this content."
                                                </p>
                                            })}
                                        </div>
                                        })}

                                        // Actions bar at bottom
                                        <div style=PANEL_STYLE>
                                            <div style="display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 10px;">
                                                // Left side - View/Edit toggle
                                                <div>
                                                    {move || if !is_new() {
                                                        if view_mode.get() {
                                                            view! {
                                                                <button
                                                                    style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #1e293b; color: #06b6d4; border-radius: 8px; border: 1px solid #06b6d4; font-weight: 500; cursor: pointer;"
                                                                    on:click=move |_| set_view_mode.set(false)
                                                                >
                                                                    <Icon name=IconName::Edit size=18 />
                                                                    "Edit"
                                                                </button>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <button
                                                                    style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #1e293b; color: #94a3b8; border-radius: 8px; border: 1px solid #334155; font-weight: 500; cursor: pointer;"
                                                                    on:click=move |_| set_view_mode.set(true)
                                                                >
                                                                    <Icon name=IconName::Eye size=18 />
                                                                    "View"
                                                                </button>
                                                            }.into_any()
                                                        }
                                                    } else {
                                                        view! { <div></div> }.into_any()
                                                    }}
                                                </div>

                                                // Right side - Action buttons
                                                <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 10px;">
                                                <button
                                                    style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #06b6d4; color: white; border-radius: 8px; font-weight: 500; cursor: pointer; border: none;"
                                                    on:click=move |_| { let _ = save_action.dispatch(()); }
                                                    disabled=move || is_loading.get() || view_mode.get()
                                                >
                                                    <Icon name=IconName::Document size=18 />
                                                    {move || if is_loading.get() { "Saving..." } else { "Save Draft" }}
                                                </button>

                                                <Show when=move || !is_new() && !view_mode.get()>
                                                    <button
                                                        style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #f59e0b; color: white; border-radius: 8px; font-weight: 500; cursor: pointer; border: none;"
                                                        on:click=move |_| { let _ = submit_review_action.dispatch(()); }
                                                        disabled=move || is_loading.get() || current_status.get() != "draft"
                                                    >
                                                        <Icon name=IconName::CheckCircle size=18 />
                                                        "Submit for Review"
                                                    </button>

                                                    // Publish button - only visible to publishers/admins (uses schema-driven publish)
                                                    <Show when=move || can_publish()>
                                                        <button
                                                            style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #22c55e; color: white; border-radius: 8px; font-weight: 500; cursor: pointer; border: none;"
                                                            on:click=move |_| { let _ = schema_publish_action.dispatch(()); }
                                                            disabled=move || is_loading.get()
                                                        >
                                                            <Icon name=IconName::Rocket size=18 />
                                                            "Publish"
                                                        </button>
                                                    </Show>

                                                    <button
                                                        style="display: flex; align-items: center; justify-content: center; gap: 8px; padding: 10px 20px; background: #0ea5e9; color: white; border-radius: 8px; border: none; font-weight: 500; cursor: pointer;"
                                                        on:click=move |_| { let _ = preview_action.dispatch(()); }
                                                    >
                                                        <Icon name=IconName::Eye size=18 />
                                                        "Preview"
                                                    </button>
                                                </Show>
                                                </div>
                                            </div>
                                        </div>

                                        // Version Info (only for existing content) - simplified for schema-driven API
                                        {move || (!is_new()).then(|| {
                                            content_resource.get().and_then(|r| r).and_then(|r| r.ok()).map(|detail| {
                                                let current_version = detail.item.version;
                                                let published_version = detail.item.published_version;
                                                let status = detail.item.status.display_name();
                                                view! {
                                                    <div style=PANEL_STYLE>
                                                        <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">
                                                            "Version Info"
                                                        </h3>
                                                        <div style="display: flex; gap: 24px; flex-wrap: wrap;">
                                                            <div>
                                                                <span style="font-size: 13px; color: #64748b;">"Current Version: "</span>
                                                                <span style="font-weight: 600; color: #22d3ee;">{"v"}{current_version}</span>
                                                            </div>
                                                            {published_version.map(|pv| view! {
                                                                <div>
                                                                    <span style="font-size: 13px; color: #64748b;">"Published: "</span>
                                                                    <span style="font-weight: 600; color: #22c55e;">{"v"}{pv}</span>
                                                                </div>
                                                            })}
                                                            <div>
                                                                <span style="font-size: 13px; color: #64748b;">"Status: "</span>
                                                                <span style="font-weight: 500; color: #e2e8f0;">{status}</span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                        })}

                                        // Version History Panel (only for existing content)
                                        {move || (!is_new()).then(|| {
                                            let curr_ver = current_version.get();
                                            view! {
                                                <div style=PANEL_STYLE>
                                                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                                                        <h3 style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                                                            "Version History"
                                                        </h3>
                                                        <button
                                                            style="display: flex; align-items: center; gap: 6px; padding: 6px 12px; background: transparent; border: 1px solid #334155; border-radius: 6px; color: #94a3b8; font-size: 13px; cursor: pointer;"
                                                            on:click=move |_| set_show_version_history.update(|v| *v = !*v)
                                                        >
                                                            {move || if show_version_history.get() {
                                                                view! { <><Icon name=IconName::ChevronUp size=14 />" Hide"</> }.into_any()
                                                            } else {
                                                                view! { <><Icon name=IconName::ChevronDown size=14 />" Show"</> }.into_any()
                                                            }}
                                                        </button>
                                                    </div>

                                                    {move || show_version_history.get().then(|| {
                                                        view! {
                                                            <Suspense fallback=move || view! {
                                                                <div style="display: flex; align-items: center; justify-content: center; padding: 30px;">
                                                                    <ProgressSpinner size=SpinnerSize::Small />
                                                                </div>
                                                            }>
                                                                {move || {
                                                                    match version_history_resource.get() {
                                                                        Some(Some(versions)) if !versions.is_empty() => {
                                                                            let versions_clone = versions.clone();
                                                                            view! {
                                                                                <div style="display: flex; flex-direction: column; gap: 8px; max-height: 400px; overflow-y: auto;">
                                                                                    {versions_clone.into_iter().map(|version| {
                                                                                        let ver_num = version.version_number;
                                                                                        let is_current = ver_num == curr_ver;
                                                                                        let change_summary = version.change_summary.clone().unwrap_or_else(|| "No description".to_string());
                                                                                        let created_at = version.created_at.format("%Y-%m-%d %H:%M").to_string();
                                                                                        let created_by = version.created_by.clone().unwrap_or_else(|| "Unknown".to_string());

                                                                                        view! {
                                                                                            <div style={format!(
                                                                                                "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; background: {}; border-radius: 8px; border: 1px solid {};",
                                                                                                if is_current { "#1e3a5f" } else { "#1e293b" },
                                                                                                if is_current { "#06b6d4" } else { "#334155" }
                                                                                            )}>
                                                                                                <div style="flex: 1;">
                                                                                                    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                                                                                        <span style={format!(
                                                                                                            "font-weight: 600; color: {};",
                                                                                                            if is_current { "#22d3ee" } else { "#e2e8f0" }
                                                                                                        )}>
                                                                                                            {"v"}{ver_num}
                                                                                                        </span>
                                                                                                        {is_current.then(|| view! {
                                                                                                            <span style="padding: 2px 8px; background: #06b6d4; color: #0f172a; border-radius: 4px; font-size: 11px; font-weight: 600;">
                                                                                                                "CURRENT"
                                                                                                            </span>
                                                                                                        })}
                                                                                                    </div>
                                                                                                    <p style="font-size: 13px; color: #94a3b8; margin-bottom: 4px; max-width: 400px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                                                                                                        {change_summary}
                                                                                                    </p>
                                                                                                    <p style="font-size: 12px; color: #64748b;">
                                                                                                        {created_at}" by "{created_by}
                                                                                                    </p>
                                                                                                </div>

                                                                                                // Action buttons (not shown for current version)
                                                                                                {(!is_current).then(|| view! {
                                                                                                    <div style="display: flex; gap: 8px;">
                                                                                                        // Revert button
                                                                                                        {can_revert().then(|| view! {
                                                                                                            <button
                                                                                                                style="padding: 6px 12px; background: #dc2626; color: white; border: none; border-radius: 6px; font-size: 12px; font-weight: 500; cursor: pointer; display: flex; align-items: center; gap: 4px;"
                                                                                                                on:click=move |_| set_revert_confirm_version.set(Some(ver_num))
                                                                                                                title="Revert to this version"
                                                                                                            >
                                                                                                                <Icon name=IconName::Refresh size=14 />
                                                                                                                "Revert"
                                                                                                            </button>
                                                                                                        })}
                                                                                                    </div>
                                                                                                })}
                                                                                            </div>
                                                                                        }
                                                                                    }).collect_view()}
                                                                                </div>
                                                                            }.into_any()
                                                                        }
                                                                        Some(Some(_)) => {
                                                                            view! {
                                                                                <div style="padding: 20px; text-align: center; color: #64748b;">
                                                                                    "No version history available"
                                                                                </div>
                                                                            }.into_any()
                                                                        }
                                                                        Some(None) => {
                                                                            view! {
                                                                                <div style="padding: 20px; text-align: center; color: #f87171;">
                                                                                    "Failed to load version history"
                                                                                </div>
                                                                            }.into_any()
                                                                        }
                                                                        None => {
                                                                            view! {
                                                                                <div style="display: flex; align-items: center; justify-content: center; padding: 30px;">
                                                                                    <ProgressSpinner size=SpinnerSize::Small />
                                                                                </div>
                                                                            }.into_any()
                                                                        }
                                                                    }
                                                                }}
                                                            </Suspense>
                                                        }
                                                    })}
                                                </div>
                                            }
                                        })}
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </Suspense>

                // Revert confirmation modal
                {move || revert_confirm_version.get().map(|version| {
                    view! {
                        <div
                            style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 50;"
                            on:click=move |_| set_revert_confirm_version.set(None)
                        >
                            <div
                                style="background: #1e293b; border-radius: 12px; padding: 24px; max-width: 400px; width: 90%; border: 1px solid #334155;"
                                on:click=move |e| e.stop_propagation()
                            >
                                <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 12px;">
                                    {"Revert to Version "}{version}{"?"}
                                </h3>
                                <p style="font-size: 14px; color: #94a3b8; margin-bottom: 20px;">
                                    "This will create a new version with the content from version "{version}". The current version will be preserved in history."
                                </p>
                                <div style="display: flex; gap: 12px; justify-content: flex-end;">
                                    <button
                                        style="padding: 10px 16px; background: #334155; color: #e2e8f0; border: none; border-radius: 6px; cursor: pointer; font-weight: 500;"
                                        on:click=move |_| set_revert_confirm_version.set(None)
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        style="padding: 10px 16px; background: #dc2626; color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: 500; display: flex; align-items: center; gap: 6px;"
                                        on:click=move |_| { let _ = schema_revert_action.dispatch(version); }
                                        disabled=move || is_reverting.get()
                                    >
                                        {move || if is_reverting.get() {
                                            "Reverting..."
                                        } else {
                                            "Revert"
                                        }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                })}

                // Diff viewer modal (also used for revert preview)
                {move || {
                    // Show modal if we have diff result or are loading
                    let show_modal = diff_result.get().is_some() || is_loading_diff.get();
                    let compare_ver = diff_compare_version.get();
                    let is_revert = is_revert_preview.get();

                    show_modal.then(|| {
                        view! {
                            <div
                                style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 50; padding: 20px;"
                                on:click=move |_| {
                                    set_diff_result.set(None);
                                    set_diff_compare_version.set(None);
                                    set_is_revert_preview.set(false);
                                }
                            >
                                <div
                                    style="background: #0f172a; border-radius: 12px; max-width: 900px; width: 100%; max-height: 90vh; display: flex; flex-direction: column; border: 1px solid #334155;"
                                    on:click=move |e| e.stop_propagation()
                                >
                                    // Modal header
                                    <div style="display: flex; align-items: center; justify-content: space-between; padding: 20px 24px; border-bottom: 1px solid #334155; flex-shrink: 0;">
                                        <div>
                                            <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0;">
                                                {move || if is_revert_preview.get() {
                                                    format!("Revert Preview - Version {}", compare_ver.unwrap_or(0))
                                                } else {
                                                    "Version Comparison".to_string()
                                                }}
                                            </h3>
                                            {move || is_revert_preview.get().then(|| view! {
                                                <p style="font-size: 13px; color: #94a3b8; margin-top: 4px;">
                                                    "Review the changes before reverting"
                                                </p>
                                            })}
                                        </div>
                                        <button
                                            style="padding: 8px; background: transparent; border: none; color: #94a3b8; cursor: pointer; border-radius: 6px;"
                                            on:click=move |_| {
                                                set_diff_result.set(None);
                                                set_diff_compare_version.set(None);
                                                set_is_revert_preview.set(false);
                                            }
                                        >
                                            <Icon name=IconName::Close size=20 />
                                        </button>
                                    </div>

                                    // Modal body (scrollable)
                                    <div style="padding: 24px; overflow-y: auto; flex: 1;">
                                        {move || {
                                            if is_loading_diff.get() {
                                                view! {
                                                    <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; gap: 16px;">
                                                        <ProgressSpinner size=SpinnerSize::Default />
                                                        <p style="font-size: 14px; color: #94a3b8;">"Loading preview..."</p>
                                                    </div>
                                                }.into_any()
                                            } else if let Some(diff) = diff_result.get() {
                                                view! {
                                                    <DiffViewer diff=diff />
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div style="text-align: center; padding: 40px; color: #94a3b8;">
                                                        <p>"No diff data available"</p>
                                                    </div>
                                                }.into_any()
                                            }
                                        }}
                                    </div>

                                    // Modal footer (only in revert preview mode)
                                    {move || is_revert_preview.get().then(|| {
                                        let ver = diff_compare_version.get();
                                        view! {
                                            <div style="padding: 16px 24px; border-top: 1px solid #334155; display: flex; gap: 12px; justify-content: flex-end; flex-shrink: 0; background: #0f172a;">
                                                <button
                                                    style="padding: 10px 20px; background: #334155; color: #e2e8f0; border: none; border-radius: 6px; cursor: pointer; font-weight: 500;"
                                                    on:click=move |_| {
                                                        set_diff_result.set(None);
                                                        set_diff_compare_version.set(None);
                                                        set_is_revert_preview.set(false);
                                                    }
                                                >
                                                    "Cancel"
                                                </button>
                                                <button
                                                    style="padding: 10px 20px; background: #dc2626; color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: 500; display: flex; align-items: center; gap: 8px;"
                                                    on:click=move |_| {
                                                        if let Some(version) = ver {
                                                            // Close the preview modal
                                                            set_diff_result.set(None);
                                                            set_diff_compare_version.set(None);
                                                            set_is_revert_preview.set(false);
                                                            // Trigger the revert (using schema-driven revert)
                                                            let _ = schema_revert_action.dispatch(version);
                                                        }
                                                    }
                                                    disabled=move || is_reverting.get()
                                                >
                                                    <Icon name=IconName::Refresh size=16 />
                                                    {move || if is_reverting.get() {
                                                        "Reverting..."
                                                    } else {
                                                        "Confirm Revert"
                                                    }}
                                                </button>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })
                }}
            </div>
        </div>
    }
}

/// Format status for display
fn format_status(status: &str) -> String {
    match status {
        "pending_review" => "Pending Review".to_string(),
        s => {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        }
    }
}

/// Format date for display
fn format_date(date_str: &str) -> String {
    if date_str.len() >= 10 {
        date_str[..10].to_string()
    } else {
        date_str.to_string()
    }
}

/// Force editor server functions to be registered
/// Call this from main.rs to ensure inventory picks up the server functions
#[cfg(feature = "ssr")]
pub fn register_editor_server_fns() {
    use server_fn::ServerFn;
    println!("Registering editor server functions:");
    println!("  GetContentDetail: {}", GetContentDetail::url());
    println!("  SaveContent: {}", SaveContent::url());
    println!("  CreateContentPreview: {}", CreateContentPreview::url());
    println!("  GetVersionHistory: {}", GetVersionHistory::url());
    println!("  RevertToVersion: {}", RevertToVersion::url());
    println!("  CompareVersions: {}", CompareVersions::url());
    println!("  CreateSchemaContentPreview: {}", CreateSchemaContentPreview::url());
}
