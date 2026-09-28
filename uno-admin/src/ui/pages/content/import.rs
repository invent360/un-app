//! Bulk content import page
//!
//! Provides a UI for importing content from JSON files with preview and validation.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use server_fn::codec::PostUrl;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::api::content_client::{ContentClient, UpsertContentRequest};

/// Server function to import raw JSON content directly
#[server(ImportRawJson, "/api", endpoint = "import_raw_json", input = PostUrl)]
pub async fn import_raw_json(
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

/// Panel styling constants (matching editor.rs dark navy theme)
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 24px;";

/// Import item from JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub content_type: String,
    pub slug: String,
    pub content: serde_json::Value,
    #[serde(default)]
    pub translations: Option<serde_json::Value>,
}

/// Validation result for an import item
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub item: ImportItem,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Import result for a single item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub slug: String,
    pub success: bool,
    pub message: String,
}

/// Bulk import response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkImportResponse {
    pub imported: i32,
    pub skipped: i32,
    pub errors: Vec<ImportResult>,
}

/// Server function to bulk import content
#[server(BulkImportContent, "/api")]
pub async fn bulk_import_content(
    items: Vec<ImportItem>,
) -> Result<BulkImportResponse, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    let mut imported = 0;
    let mut skipped = 0;
    let mut errors = Vec::new();

    for item in items {
        let request = UpsertContentRequest {
            content_type: item.content_type,
            slug: item.slug.clone(),
            content: item.content,
            translations: item.translations,
            display_order: None,
            is_featured: None,
            change_summary: Some("Bulk import".to_string()),
        };

        match client.create_content(request).await {
            Ok(_) => {
                imported += 1;
            }
            Err(e) => {
                let err_str = e.to_string();
                // Check if it's a duplicate slug error
                if err_str.contains("already exists") || err_str.contains("duplicate") {
                    skipped += 1;
                    errors.push(ImportResult {
                        slug: item.slug,
                        success: false,
                        message: "Content with this slug already exists".to_string(),
                    });
                } else {
                    errors.push(ImportResult {
                        slug: item.slug,
                        success: false,
                        message: err_str,
                    });
                }
            }
        }
    }

    Ok(BulkImportResponse {
        imported,
        skipped,
        errors,
    })
}

/// Validate an import item
fn validate_item(item: &ImportItem) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check content type
    let valid_types = ["task", "guide", "faq", "home", "error"];
    if !valid_types.contains(&item.content_type.as_str()) {
        errors.push(format!("Invalid content type: {}. Must be one of: task, guide, faq, home, error", item.content_type));
    }

    // Check slug
    if item.slug.is_empty() {
        errors.push("Slug is required".to_string());
    } else if item.slug.contains(' ') {
        errors.push("Slug cannot contain spaces".to_string());
    }

    // Check content has title
    if item.content.get("title").is_none() {
        errors.push("Content must have a 'title' field".to_string());
    }

    // Warnings for missing optional fields
    if item.content.get("description").is_none() {
        warnings.push("Missing 'description' field".to_string());
    }

    if item.translations.is_none() {
        warnings.push("No translations provided".to_string());
    }

    ValidationResult {
        item: item.clone(),
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}

/// Parse JSON content from string
fn parse_import_json(content: &str) -> Result<Vec<ImportItem>, String> {
    // Try parsing as array first
    if let Ok(items) = serde_json::from_str::<Vec<ImportItem>>(content) {
        return Ok(items);
    }

    // Try parsing as single object
    if let Ok(item) = serde_json::from_str::<ImportItem>(content) {
        return Ok(vec![item]);
    }

    Err("Invalid JSON format. Expected an array of content items or a single content item.".to_string())
}

/// Content import page
#[component]
pub fn ContentImportPage() -> impl IntoView {
    let navigate = use_navigate();

    // Quick import state
    let (quick_schema, set_quick_schema) = signal(String::from("task"));
    let (quick_slug, set_quick_slug) = signal(String::new());
    let (quick_json, set_quick_json) = signal(String::new());
    let (quick_importing, set_quick_importing) = signal(false);
    let (quick_result, set_quick_result) = signal(Option::<Result<i32, String>>::None);

    // Quick import action
    let quick_import_action = Action::new(move |_: &()| {
        let schema = quick_schema.get();
        let slug = quick_slug.get();
        let json = quick_json.get();

        async move {
            set_quick_importing.set(true);
            set_quick_result.set(None);

            let result = import_raw_json(schema, slug, json).await;

            set_quick_importing.set(false);

            match result {
                Ok(id) => {
                    set_quick_result.set(Some(Ok(id)));
                    // Clear the form on success
                    set_quick_slug.set(String::new());
                    set_quick_json.set(String::new());
                }
                Err(e) => {
                    set_quick_result.set(Some(Err(e.to_string())));
                }
            }
        }
    });

    // State
    let (_file_content, set_file_content) = signal(Option::<String>::None);
    let (parsed_items, set_parsed_items) = signal(Vec::<ValidationResult>::new());
    let (parse_error, set_parse_error) = signal(Option::<String>::None);
    let (is_importing, set_is_importing) = signal(false);
    let (import_result, set_import_result) = signal(Option::<BulkImportResponse>::None);
    let (error_message, set_error_message) = signal(Option::<String>::None);

    // Handle file selection
    let on_file_change = move |ev: web_sys::Event| {
        use wasm_bindgen::JsCast;

        let input = ev.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();

                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Ok(result) = reader_clone.result() {
                        if let Some(text) = result.as_string() {
                            set_file_content.set(Some(text.clone()));

                            // Parse and validate
                            match parse_import_json(&text) {
                                Ok(items) => {
                                    let validated: Vec<ValidationResult> = items.iter()
                                        .map(|item| validate_item(item))
                                        .collect();
                                    set_parsed_items.set(validated);
                                    set_parse_error.set(None);
                                }
                                Err(e) => {
                                    set_parse_error.set(Some(e));
                                    set_parsed_items.set(Vec::new());
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

                let _ = reader.read_as_text(&file);
            }
        }
    };

    // Import action
    let import_action = Action::new(move |items: &Vec<ImportItem>| {
        let items_clone = items.clone();
        async move {
            set_is_importing.set(true);
            set_error_message.set(None);

            let result = bulk_import_content(items_clone).await;

            set_is_importing.set(false);

            match result {
                Ok(response) => {
                    set_import_result.set(Some(response));
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
        }
    });

    // Navigation
    let nav = navigate.clone();
    let on_back = move |_| {
        nav("/content", Default::default());
    };

    // Computed values
    let valid_items = move || {
        parsed_items.get().iter()
            .filter(|r| r.is_valid)
            .map(|r| r.item.clone())
            .collect::<Vec<_>>()
    };

    let valid_count = move || valid_items().len();
    let total_count = move || parsed_items.get().len();
    let invalid_count = move || total_count() - valid_count();

    view! {
        <div>
            <Header
                title="Import Content".to_string()
                show_search=false
            />

            <div class="px-4 py-4 space-y-4">
                // Back button
                <button
                    class="flex items-center gap-2 text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition"
                    on:click=on_back
                >
                    <Icon name=IconName::ArrowLeft size=20 />
                    "Back to Content"
                </button>

                // Error message
                {move || error_message.get().map(|msg| view! {
                    <div class="p-4 rounded-lg bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 text-sm">
                        {msg}
                    </div>
                })}

                // Quick Import Section
                <div style=PANEL_STYLE>
                    <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">
                        "Quick Import (Raw JSON)"
                    </h3>
                    <p style="font-size: 14px; color: #94a3b8; margin-bottom: 20px;">
                        "Paste raw JSON content directly from backup files (task.json, guide.json, etc.)"
                    </p>

                    // Quick import result message
                    {move || quick_result.get().map(|result| match result {
                        Ok(id) => view! {
                            <div style="background: rgba(34, 197, 94, 0.1); border: 1px solid #22c55e; border-radius: 8px; padding: 12px; margin-bottom: 16px; color: #22c55e;">
                                "Successfully imported content with ID: " {id}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <div style="background: rgba(220, 38, 38, 0.1); border: 1px solid #dc2626; border-radius: 8px; padding: 12px; margin-bottom: 16px; color: #f87171;">
                                "Error: " {e}
                            </div>
                        }.into_any(),
                    })}

                    <div style="display: grid; grid-template-columns: 1fr 2fr; gap: 16px; margin-bottom: 16px;">
                        // Schema type dropdown
                        <div>
                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                "Content Type"
                            </label>
                            <select
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; color: #e2e8f0; font-size: 14px;"
                                on:change=move |ev| {
                                    use wasm_bindgen::JsCast;
                                    let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                                    set_quick_schema.set(target.value());
                                }
                                prop:value=move || quick_schema.get()
                            >
                                <option value="task">"task"</option>
                                <option value="guide">"guide"</option>
                                <option value="home">"home"</option>
                                <option value="faq">"faq"</option>
                            </select>
                        </div>

                        // Slug input
                        <div>
                            <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                                "Slug"
                            </label>
                            <input
                                type="text"
                                placeholder="e.g., task-main, guide-main"
                                style="width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; color: #e2e8f0; font-size: 14px;"
                                on:input=move |ev| {
                                    use wasm_bindgen::JsCast;
                                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                                    set_quick_slug.set(target.value());
                                }
                                prop:value=move || quick_slug.get()
                            />
                        </div>
                    </div>

                    // JSON content textarea
                    <div style="margin-bottom: 16px;">
                        <label style="display: block; font-size: 13px; font-weight: 500; color: #94a3b8; margin-bottom: 6px;">
                            "JSON Content"
                        </label>
                        <textarea
                            placeholder="Paste JSON content here..."
                            style="width: 100%; min-height: 200px; background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; color: #e2e8f0; font-size: 13px; font-family: monospace; resize: vertical;"
                            on:input=move |ev| {
                                use wasm_bindgen::JsCast;
                                let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
                                set_quick_json.set(target.value());
                            }
                            prop:value=move || quick_json.get()
                        />
                    </div>

                    // Import button
                    <button
                        style="padding: 10px 20px; background: #22c55e; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500;"
                        on:click=move |_| {
                            let _ = quick_import_action.dispatch(());
                        }
                        disabled=move || quick_importing.get() || quick_slug.get().is_empty() || quick_json.get().is_empty()
                    >
                        {move || if quick_importing.get() { "Importing..." } else { "Import Content" }}
                    </button>
                </div>

                <div style="text-align: center; color: #64748b; font-size: 14px; margin: 16px 0;">
                    "— OR —"
                </div>

                // Import result
                {move || import_result.get().map(|result| view! {
                    <div style=PANEL_STYLE>
                        <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">
                            "Import Complete"
                        </h3>
                        <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; margin-bottom: 16px;">
                            <div style="background: rgba(34, 197, 94, 0.1); border: 1px solid #22c55e; border-radius: 8px; padding: 16px; text-align: center;">
                                <div style="font-size: 32px; font-weight: 700; color: #22c55e;">
                                    {result.imported}
                                </div>
                                <div style="font-size: 14px; color: #94a3b8;">"Imported"</div>
                            </div>
                            <div style="background: rgba(234, 179, 8, 0.1); border: 1px solid #eab308; border-radius: 8px; padding: 16px; text-align: center;">
                                <div style="font-size: 32px; font-weight: 700; color: #eab308;">
                                    {result.skipped}
                                </div>
                                <div style="font-size: 14px; color: #94a3b8;">"Skipped"</div>
                            </div>
                            <div style="background: rgba(220, 38, 38, 0.1); border: 1px solid #dc2626; border-radius: 8px; padding: 16px; text-align: center;">
                                <div style="font-size: 32px; font-weight: 700; color: #dc2626;">
                                    {result.errors.len()}
                                </div>
                                <div style="font-size: 14px; color: #94a3b8;">"Errors"</div>
                            </div>
                        </div>

                        // Show errors if any
                        {(!result.errors.is_empty()).then(|| view! {
                            <div style="margin-top: 16px;">
                                <h4 style="font-size: 14px; font-weight: 500; color: #94a3b8; margin-bottom: 8px;">
                                    "Error Details"
                                </h4>
                                <div style="background: #1e293b; border-radius: 8px; padding: 12px; max-height: 200px; overflow-y: auto;">
                                    {result.errors.iter().map(|err| view! {
                                        <div style="padding: 8px 0; border-bottom: 1px solid #334155;">
                                            <span style="font-weight: 500; color: #f87171;">{err.slug.clone()}</span>
                                            <span style="color: #64748b; margin-left: 8px;">{err.message.clone()}</span>
                                        </div>
                                    }).collect_view()}
                                </div>
                            </div>
                        })}

                        <div style="margin-top: 20px; display: flex; gap: 12px;">
                            <button
                                style="padding: 10px 20px; background: #06b6d4; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500;"
                                on:click=move |_| {
                                    set_import_result.set(None);
                                    set_file_content.set(None);
                                    set_parsed_items.set(Vec::new());
                                }
                            >
                                "Import More"
                            </button>
                            <a
                                href="/content"
                                style="padding: 10px 20px; background: #334155; color: #e2e8f0; border: none; border-radius: 8px; text-decoration: none; font-weight: 500;"
                            >
                                "View Content List"
                            </a>
                        </div>
                    </div>
                })}

                // File upload area (show when no import result)
                {move || import_result.get().is_none().then(|| view! {
                    <div style=PANEL_STYLE>
                        <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0; margin-bottom: 16px;">
                            "Upload JSON File"
                        </h3>

                        // File input
                        <div style="border: 2px dashed #334155; border-radius: 12px; padding: 40px; text-align: center; cursor: pointer; transition: border-color 0.2s;"
                            class="hover:border-cyan-500"
                        >
                            <input
                                type="file"
                                accept=".json"
                                style="display: none;"
                                id="file-input"
                                on:change=on_file_change.clone()
                            />
                            <label for="file-input" style="cursor: pointer;">
                                <div style="margin-bottom: 12px;">
                                    <Icon name=IconName::Document size=48 />
                                </div>
                                <p style="font-size: 16px; color: #e2e8f0; margin-bottom: 8px;">
                                    "Drop a JSON file here or click to browse"
                                </p>
                                <p style="font-size: 13px; color: #64748b;">
                                    "Supports single content object or array of content items"
                                </p>
                            </label>
                        </div>

                        // Format help
                        <details style="margin-top: 20px;">
                            <summary style="font-size: 14px; color: #94a3b8; cursor: pointer;">
                                "View JSON format example"
                            </summary>
                            <pre style="background: #1e293b; border-radius: 8px; padding: 16px; margin-top: 12px; font-size: 13px; color: #e2e8f0; overflow-x: auto;">
{r#"[
  {
    "content_type": "task",
    "slug": "my-task",
    "content": {
      "title": "My Task Title",
      "description": "Task description here",
      "steps": ["Step 1", "Step 2"]
    },
    "translations": {
      "es": {
        "title": "Mi Tarea",
        "description": "Descripción de la tarea"
      }
    }
  }
]"#}
                            </pre>
                        </details>
                    </div>
                })}

                // Parse error
                {move || parse_error.get().map(|err| view! {
                    <div style="background: rgba(220, 38, 38, 0.1); border: 1px solid #dc2626; border-radius: 8px; padding: 16px; color: #f87171;">
                        <strong>"Parse Error: "</strong>{err}
                    </div>
                })}

                // Preview table (show when items parsed and no import result)
                {move || {
                    let items = parsed_items.get();
                    let has_items = !items.is_empty();
                    let no_result = import_result.get().is_none();

                    (has_items && no_result).then(|| view! {
                        <div style=PANEL_STYLE>
                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px;">
                                <h3 style="font-size: 18px; font-weight: 600; color: #e2e8f0;">
                                    "Preview ({total_count()} items)"
                                </h3>
                                <div style="display: flex; gap: 12px; align-items: center;">
                                    <span style="font-size: 14px; color: #22c55e;">
                                        {valid_count()}" valid"
                                    </span>
                                    {(invalid_count() > 0).then(|| view! {
                                        <span style="font-size: 14px; color: #dc2626;">
                                            {invalid_count()}" invalid"
                                        </span>
                                    })}
                                </div>
                            </div>

                            // Preview table
                            <div style="overflow-x: auto;">
                                <table style="width: 100%; border-collapse: collapse;">
                                    <thead>
                                        <tr style="border-bottom: 1px solid #334155;">
                                            <th style="text-align: left; padding: 12px 8px; font-size: 13px; font-weight: 500; color: #94a3b8;">"Status"</th>
                                            <th style="text-align: left; padding: 12px 8px; font-size: 13px; font-weight: 500; color: #94a3b8;">"Type"</th>
                                            <th style="text-align: left; padding: 12px 8px; font-size: 13px; font-weight: 500; color: #94a3b8;">"Slug"</th>
                                            <th style="text-align: left; padding: 12px 8px; font-size: 13px; font-weight: 500; color: #94a3b8;">"Title"</th>
                                            <th style="text-align: left; padding: 12px 8px; font-size: 13px; font-weight: 500; color: #94a3b8;">"Issues"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {items.into_iter().map(|result| {
                                            let title = result.item.content.get("title")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("—")
                                                .to_string();
                                            let status_color = if result.is_valid { "#22c55e" } else { "#dc2626" };
                                            let status_icon = if result.is_valid { "✓" } else { "✗" };
                                            let issues = if !result.errors.is_empty() {
                                                result.errors.join(", ")
                                            } else if !result.warnings.is_empty() {
                                                result.warnings.join(", ")
                                            } else {
                                                "—".to_string()
                                            };
                                            let issue_color = if !result.errors.is_empty() { "#f87171" } else { "#eab308" };

                                            view! {
                                                <tr style="border-bottom: 1px solid #1e293b;">
                                                    <td style="padding: 12px 8px;">
                                                        <span style=format!("color: {};", status_color)>
                                                            {status_icon}
                                                        </span>
                                                    </td>
                                                    <td style="padding: 12px 8px;">
                                                        <span style="padding: 4px 8px; background: #334155; border-radius: 4px; font-size: 12px; color: #e2e8f0;">
                                                            {result.item.content_type.clone()}
                                                        </span>
                                                    </td>
                                                    <td style="padding: 12px 8px; font-family: monospace; font-size: 13px; color: #e2e8f0;">
                                                        {result.item.slug.clone()}
                                                    </td>
                                                    <td style="padding: 12px 8px; font-size: 14px; color: #e2e8f0;">
                                                        {title}
                                                    </td>
                                                    <td style=format!("padding: 12px 8px; font-size: 13px; color: {};", issue_color)>
                                                        {issues}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>

                            // Import button
                            <div style="margin-top: 20px; display: flex; gap: 12px; justify-content: flex-end;">
                                <button
                                    style="padding: 10px 20px; background: #334155; color: #e2e8f0; border: none; border-radius: 8px; cursor: pointer; font-weight: 500;"
                                    on:click=move |_| {
                                        set_file_content.set(None);
                                        set_parsed_items.set(Vec::new());
                                    }
                                >
                                    "Clear"
                                </button>
                                <button
                                    style="padding: 10px 20px; background: #22c55e; color: white; border: none; border-radius: 8px; cursor: pointer; font-weight: 500; display: flex; align-items: center; gap: 8px;"
                                    on:click=move |_| {
                                        let items = valid_items();
                                        if !items.is_empty() {
                                            let _ = import_action.dispatch(items);
                                        }
                                    }
                                    disabled=move || is_importing.get() || valid_count() == 0
                                >
                                    {move || if is_importing.get() {
                                        "Importing..."
                                    } else {
                                        "Import Valid Items"
                                    }}
                                    {move || format!(" ({})", valid_count())}
                                </button>
                            </div>
                        </div>
                    })
                }}
            </div>
        </div>
    }
}
