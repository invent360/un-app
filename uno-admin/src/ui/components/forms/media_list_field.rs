//! Media list field component for multiple image/file uploads
//!
//! Supports uploading multiple files to cloud storage (GCS).

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::api::schema_types::{MediaListFieldConfig, MediaType};
use super::media_field::{UploadResponse, UploadState};

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const DROPZONE_STYLE: &str = "border: 2px dashed #334155; border-radius: 0.375rem; padding: 1.5rem; text-align: center; cursor: pointer; transition: border-color 0.2s, background 0.2s;";
const DROPZONE_ACTIVE_STYLE: &str = "border: 2px dashed #3b82f6; border-radius: 0.375rem; padding: 1.5rem; text-align: center; cursor: pointer; background: rgba(59, 130, 246, 0.1);";
const PREVIEW_CONTAINER_STYLE: &str = "display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;";
const PREVIEW_ITEM_STYLE: &str = "position: relative; width: 100px; height: 100px; border-radius: 0.375rem; overflow: hidden; border: 1px solid #334155;";
const PREVIEW_IMG_STYLE: &str = "width: 100%; height: 100%; object-fit: cover;";
const REMOVE_BTN_STYLE: &str = "position: absolute; top: 4px; right: 4px; background: rgba(239, 68, 68, 0.9); border: none; border-radius: 50%; width: 20px; height: 20px; color: white; font-size: 12px; cursor: pointer; display: flex; align-items: center; justify-content: center;";
const PROGRESS_STYLE: &str = "width: 100%; height: 4px; background: #334155; border-radius: 2px; margin-top: 0.5rem; overflow: hidden;";
const PROGRESS_BAR_STYLE: &str = "height: 100%; background: #3b82f6; transition: width 0.3s ease;";
const ERROR_STYLE: &str = "color: #ef4444; font-size: 0.75rem; margin-top: 0.5rem;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.5rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";

/// Media list field for multiple file/image uploads
#[component]
pub fn MediaListField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current values (storage URLs)
    value: RwSignal<Vec<String>>,
    /// Field configuration
    config: MediaListFieldConfig,
    /// Description/help text
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback (receives storage URLs)
    #[prop(optional)]
    on_change: Option<Callback<Vec<String>>>,
    /// Resource ID for grouping uploads (e.g., content ID)
    #[prop(optional, into)]
    resource_id: Option<String>,
) -> impl IntoView {
    let allowed_types = config.allowed_types.clone();
    let field_key_clone = field_key.clone();

    // Drag state
    let is_dragging = RwSignal::new(false);

    // Upload state
    let upload_state = RwSignal::new(UploadState::default());

    // Display URLs for previews (fetched from storage URLs)
    let display_urls = RwSignal::new(Vec::<String>::new());

    // Fetch display URLs when value changes
    Effect::new(move |_| {
        let storage_urls = value.get();
        if storage_urls.is_empty() {
            display_urls.set(Vec::new());
            return;
        }

        // Convert storage URLs to display URLs
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_display_urls(&storage_urls).await {
                Ok(urls) => display_urls.set(urls),
                Err(e) => tracing::error!("Failed to fetch display URLs: {}", e),
            }
        });
    });

    // Remove item
    let remove_item = move |index: usize| {
        let mut urls = value.get();
        if index < urls.len() {
            urls.remove(index);
            value.set(urls.clone());
            if let Some(cb) = on_change {
                cb.run(urls);
            }
        }
    };

    // File upload handler
    let resource_id_for_upload = resource_id.clone().unwrap_or_else(generate_random_id);
    let on_files_selected: Callback<web_sys::FileList> = Callback::new({
        let resource_id = resource_id_for_upload.clone();
        move |files: web_sys::FileList| {
            if files.length() == 0 {
                return;
            }

            let resource_id = resource_id.clone();

            // Start upload
            upload_state.set(UploadState {
                is_uploading: true,
                progress: 0.0,
                error: None,
            });

            // Collect files
            let mut file_list = Vec::new();
            for i in 0..files.length() {
                if let Some(file) = files.get(i) {
                    file_list.push(file);
                }
            }

            // Upload files
            wasm_bindgen_futures::spawn_local(async move {
                upload_state.set(UploadState {
                    is_uploading: true,
                    progress: 25.0,
                    error: None,
                });

                match upload_files_to_server(&resource_id, file_list).await {
                    Ok(response) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 100.0,
                            error: None,
                        });

                        // Add new URLs to existing list
                        let mut current = value.get();
                        current.extend(response.storage_urls.clone());
                        value.set(current.clone());

                        if let Some(cb) = on_change {
                            cb.run(current);
                        }
                    }
                    Err(e) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 0.0,
                            error: Some(e),
                        });
                    }
                }
            });
        }
    });

    // Handle file input change
    let on_file_change: Callback<web_sys::Event> = Callback::new({
        let on_files_selected = on_files_selected.clone();
        move |ev: web_sys::Event| {
            let input: web_sys::HtmlInputElement = event_target(&ev);
            if let Some(files) = input.files() {
                on_files_selected.run(files);
            }
            // Reset input so same file can be selected again
            input.set_value("");
        }
    });

    // Handle drag events
    let on_dragover: Callback<web_sys::DragEvent> = Callback::new(move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        is_dragging.set(true);
    });

    let on_dragleave: Callback<web_sys::DragEvent> = Callback::new(move |_ev: web_sys::DragEvent| {
        is_dragging.set(false);
    });

    let on_drop: Callback<web_sys::DragEvent> = Callback::new({
        let on_files_selected = on_files_selected.clone();
        move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            is_dragging.set(false);

            if let Some(data_transfer) = ev.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    on_files_selected.run(files);
                }
            }
        }
    });

    // Generate accept attribute for file input
    let accept_types = allowed_types.iter().map(|t| match t {
        MediaType::Image => "image/*",
        MediaType::Video => "video/*",
        MediaType::Audio => "audio/*",
        MediaType::Document => "application/pdf,.doc,.docx,.txt",
    }).collect::<Vec<_>>().join(",");
    let accept_types = if accept_types.is_empty() { "image/*".to_string() } else { accept_types };

    let allowed_types_str = if allowed_types.is_empty() {
        "Images".to_string()
    } else {
        allowed_types.iter().map(|t| match t {
            MediaType::Image => "Images",
            MediaType::Video => "Videos",
            MediaType::Audio => "Audio",
            MediaType::Document => "Documents",
        }).collect::<Vec<_>>().join(", ")
    };

    view! {
        <div class="form-field" style="margin-bottom: 1rem;">
            <label style=LABEL_STYLE>
                {label.clone()}
                {move || if required {
                    view! { <span style=REQUIRED_STYLE>"*"</span> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>

            // Image previews
            {move || {
                let urls = display_urls.get();
                if urls.is_empty() {
                    view! { <span></span> }.into_any()
                } else {
                    view! {
                        <div style=PREVIEW_CONTAINER_STYLE>
                            {urls.into_iter().enumerate().map(|(idx, url)| {
                                view! {
                                    <div style=PREVIEW_ITEM_STYLE>
                                        <img src=url style=PREVIEW_IMG_STYLE alt="Preview" />
                                        <button
                                            type="button"
                                            style=REMOVE_BTN_STYLE
                                            on:click=move |_| remove_item(idx)
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }}

            // Upload dropzone
            {
                let on_dragover = on_dragover.clone();
                let on_dragleave = on_dragleave.clone();
                let on_drop = on_drop.clone();
                let on_file_change = on_file_change.clone();
                move || {
                    let upload_state_val = upload_state.get();
                    let field_id = format!("file-input-{}", field_key_clone);
                    let field_id_clone = field_id.clone();
                    let accept = accept_types.clone();
                    let on_dragover = on_dragover.clone();
                    let on_dragleave = on_dragleave.clone();
                    let on_drop = on_drop.clone();
                    let on_file_change = on_file_change.clone();

                    view! {
                        <div>
                            <div
                                style=move || if is_dragging.get() { DROPZONE_ACTIVE_STYLE } else { DROPZONE_STYLE }
                                on:dragover=move |ev| on_dragover.run(ev)
                                on:dragleave=move |ev| on_dragleave.run(ev)
                                on:drop=move |ev| on_drop.run(ev)
                                on:click=move |_| {
                                    if let Some(window) = web_sys::window() {
                                        if let Some(document) = window.document() {
                                            if let Some(input) = document.get_element_by_id(&field_id_clone) {
                                                let _ = input.dyn_ref::<web_sys::HtmlInputElement>()
                                                    .map(|i| i.click());
                                            }
                                        }
                                    }
                                }
                            >
                                <input
                                    type="file"
                                    id=field_id
                                    accept=accept
                                    multiple=true
                                    style="display: none;"
                                    on:change=move |ev| on_file_change.run(ev)
                                />
                                {if upload_state_val.is_uploading {
                                    view! {
                                        <div>
                                            <p style="color: #94a3b8; margin: 0;">
                                                "Uploading..."
                                            </p>
                                            <div style=PROGRESS_STYLE>
                                                <div style=format!("{}; width: {}%;", PROGRESS_BAR_STYLE, upload_state_val.progress)></div>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div>
                                            <p style="color: #94a3b8; margin: 0 0 0.5rem 0;">
                                                "Drop files here or click to browse"
                                            </p>
                                            <p style="color: #64748b; font-size: 0.75rem; margin: 0;">
                                                {format!("Allowed: {} (multiple)", allowed_types_str)}
                                            </p>
                                        </div>
                                    }.into_any()
                                }}
                            </div>

                            // Error message
                            {move || {
                                if let Some(ref error) = upload_state.get().error {
                                    view! { <p style=ERROR_STYLE>{error.clone()}</p> }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                }
            }

            // Help text
            {move || {
                if let Some(ref desc) = description {
                    view! { <p style=HELP_TEXT_STYLE>{desc.clone()}</p> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Fetch display URLs for storage URLs
async fn fetch_display_urls(storage_urls: &[String]) -> Result<Vec<String>, String> {
    use gloo_net::http::Request;

    let response = Request::post("/api/files/display-urls")
        .header("Content-Type", "application/json")
        .body(serde_json::json!({ "storage_urls": storage_urls }).to_string())
        .map_err(|e| format!("Failed to create request: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !response.ok() {
        return Err(format!("Failed to get display URLs: {}", response.status()));
    }

    #[derive(serde::Deserialize)]
    struct Response {
        display_urls: Vec<String>,
    }

    let result: Response = response.json().await
        .map_err(|e| format!("Failed to parse response: {:?}", e))?;

    Ok(result.display_urls)
}

/// Upload multiple files to server
async fn upload_files_to_server(resource_id: &str, files: Vec<web_sys::File>) -> Result<UploadResponse, String> {
    use gloo_net::http::Request;
    use futures_util::StreamExt;

    // Create form data
    let form_data = web_sys::FormData::new()
        .map_err(|e| format!("Failed to create form: {:?}", e))?;

    // Add resource_id
    form_data.append_with_str("resource_id", resource_id)
        .map_err(|e| format!("Failed to append resource_id: {:?}", e))?;

    // Add files
    for file in files {
        form_data.append_with_blob_and_filename("file", &file, &file.name())
            .map_err(|e| format!("Failed to append file: {:?}", e))?;
    }

    let response = Request::post("/api/files/upload")
        .body(form_data)
        .map_err(|e| format!("Failed to create request: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {:?}", e))?;

    if !response.ok() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("Upload failed ({}): {}", status, text));
    }

    response
        .json::<UploadResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {:?}", e))
}

/// Generate a random ID
fn generate_random_id() -> String {
    use js_sys::Math;

    let mut id = String::with_capacity(32);
    for _ in 0..32 {
        let digit = (Math::random() * 16.0) as u8;
        id.push(char::from_digit(digit as u32, 16).unwrap_or('0'));
    }

    format!(
        "{}-{}-{}-{}",
        &id[0..8],
        &id[8..12],
        &id[12..16],
        &id[16..32]
    )
}
