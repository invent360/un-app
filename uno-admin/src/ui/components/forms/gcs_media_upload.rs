//! GCS Media Upload Component
//!
//! A reusable component for uploading media files to Google Cloud Storage.
//! Used by all section editors (Task, Guide, Home, etc.) to upload images
//! to GCS instead of storing base64 in the database.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

/// Maximum file size: 5MB
const MAX_FILE_SIZE: f64 = 5.0 * 1024.0 * 1024.0;

/// Upload state
#[derive(Clone, Default)]
pub struct GcsUploadState {
    pub is_uploading: bool,
    pub progress: f32,
    pub error: Option<String>,
}

/// Response from file upload API
#[derive(Clone, Debug, serde::Deserialize)]
pub struct UploadResponse {
    pub storage_urls: Vec<String>,
    pub display_urls: Vec<String>,
    pub uploaded_count: usize,
    pub errors: Vec<String>,
}

/// GCS Media Upload List Component
///
/// A reusable component that uploads files to GCS and stores storage URLs.
/// This replaces the base64 data URL approach used in section editors.
///
/// # Props
/// - `value`: Current list of storage URLs (gcs://rwa-app/...)
/// - `on_change`: Callback when URLs change
/// - `content_type`: Content type used as folder prefix (e.g., "task", "guide", "home")
/// - `accept`: File types to accept (default: "image/*")
/// - `label`: Label for the upload button (default: "images")
#[component]
pub fn GcsMediaUploadList(
    /// Current list of storage URLs
    value: Signal<Vec<String>>,
    /// Callback when URLs change
    on_change: Callback<Vec<String>>,
    /// Content type for GCS path (e.g., "task", "guide", "home")
    #[prop(into)]
    content_type: String,
    /// File types to accept
    #[prop(default = "image/*".to_string())]
    accept: String,
    /// Label for the upload button
    #[prop(default = "images".to_string())]
    label: String,
) -> impl IntoView {
    let file_input_ref = NodeRef::<leptos::html::Input>::new();
    let upload_state = RwSignal::new(GcsUploadState::default());
    let display_urls = RwSignal::new(Vec::<String>::new());
    let content_type_for_upload = content_type.clone();

    // Fetch display URLs when value changes
    Effect::new({
        let value = value.clone();
        move |_| {
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
        }
    });

    let trigger_file_input = move |_| {
        upload_state.update(|s| s.error = None);
        if let Some(input) = file_input_ref.get() {
            input.click();
        }
    };

    let handle_file_select = {
        let content_type = content_type_for_upload.clone();
        move |ev: web_sys::Event| {
            let input: HtmlInputElement = event_target(&ev);
            upload_state.update(|s| s.error = None);

            if let Some(files) = input.files() {
                if files.length() == 0 {
                    return;
                }

                // Validate file sizes
                let mut valid_files = Vec::new();
                for i in 0..files.length() {
                    if let Some(file) = files.get(i) {
                        let file_size = file.size();
                        if file_size > MAX_FILE_SIZE {
                            let size_mb = file_size / (1024.0 * 1024.0);
                            let max_mb = MAX_FILE_SIZE / (1024.0 * 1024.0);
                            upload_state.update(|s| {
                                s.error = Some(format!(
                                    "File '{}' is too large ({:.1}MB). Maximum size is {:.1}MB.",
                                    file.name(), size_mb, max_mb
                                ));
                            });
                            continue;
                        }
                        valid_files.push(file);
                    }
                }

                if valid_files.is_empty() {
                    input.set_value("");
                    return;
                }

                // Start upload
                let content_type = content_type.clone();
                let value = value.clone();
                let on_change = on_change.clone();

                upload_state.set(GcsUploadState {
                    is_uploading: true,
                    progress: 0.0,
                    error: None,
                });

                wasm_bindgen_futures::spawn_local(async move {
                    upload_state.update(|s| s.progress = 25.0);

                    match upload_files_to_gcs(&content_type, valid_files).await {
                        Ok(response) => {
                            upload_state.set(GcsUploadState {
                                is_uploading: false,
                                progress: 100.0,
                                error: None,
                            });

                            // Add new URLs to existing list
                            let mut current = value.get();
                            current.extend(response.storage_urls);
                            on_change.run(current);

                            if !response.errors.is_empty() {
                                upload_state.update(|s| {
                                    s.error = Some(response.errors.join(", "));
                                });
                            }
                        }
                        Err(e) => {
                            upload_state.set(GcsUploadState {
                                is_uploading: false,
                                progress: 0.0,
                                error: Some(e),
                            });
                        }
                    }
                });
            }
            input.set_value("");
        }
    };

    let remove_item = move |index: usize| {
        let mut current = value.get();
        if index < current.len() {
            current.remove(index);
            on_change.run(current);
        }
    };

    let label_clone = label.clone();

    view! {
        <div>
            // Hidden file input
            <input
                type="file"
                accept=accept.clone()
                multiple=true
                style="display: none;"
                node_ref=file_input_ref
                on:change=handle_file_select
            />

            // Media previews grid
            {move || {
                let urls = display_urls.get();
                if !urls.is_empty() {
                    view! {
                        <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.75rem;">
                            {urls.iter().enumerate().map(|(index, url)| {
                                let url = url.clone();
                                let is_image = url.contains("image") ||
                                    url.contains(".png") || url.contains(".jpg") ||
                                    url.contains(".jpeg") || url.contains(".gif") ||
                                    url.contains(".webp") || url.contains("googleusercontent");
                                view! {
                                    <div style="position: relative; width: 64px; height: 64px; border-radius: 0.25rem; overflow: hidden; border: 1px solid #334155;">
                                        {if is_image {
                                            view! {
                                                <img src=url.clone() style="width: 100%; height: 100%; object-fit: cover;" />
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div style="width: 100%; height: 100%; background: #334155; display: flex; align-items: center; justify-content: center; color: #94a3b8; font-size: 0.75rem;">
                                                    "File"
                                                </div>
                                            }.into_any()
                                        }}
                                        <button
                                            type="button"
                                            style="position: absolute; top: 2px; right: 2px; width: 18px; height: 18px; background: rgba(239, 68, 68, 0.9); border: none; border-radius: 50%; color: white; font-size: 10px; cursor: pointer; display: flex; align-items: center; justify-content: center;"
                                            on:click=move |_| remove_item(index)
                                        >
                                            "×"
                                        </button>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Upload button or progress
            {move || {
                let state = upload_state.get();
                if state.is_uploading {
                    view! {
                        <div style="width: 100%; padding: 1rem; background: #0f172a; border: 2px dashed #3b82f6; border-radius: 0.375rem;">
                            <p style="color: #3b82f6; font-size: 0.875rem; margin: 0 0 0.5rem 0; text-align: center;">
                                "Uploading to GCS..."
                            </p>
                            <div style="width: 100%; height: 4px; background: #334155; border-radius: 2px; overflow: hidden;">
                                <div style=format!("height: 100%; background: #3b82f6; transition: width 0.3s ease; width: {}%;", state.progress)></div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            style="width: 100%; padding: 1rem; background: #0f172a; border: 2px dashed #334155; border-radius: 0.375rem; color: #64748b; font-size: 0.875rem; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 0.5rem;"
                            on:click=trigger_file_input
                        >
                            <span style="font-size: 1.25rem;">"+"</span>
                            {format!(" Upload {}", label_clone)}
                        </button>
                    }.into_any()
                }
            }}

            // Helper text
            <div style="font-size: 0.75rem; color: #64748b; margin-top: 0.5rem;">
                {move || {
                    let count = value.get().len();
                    format!("{} uploaded (max 5MB each)", count)
                }}
            </div>

            // Error message
            {move || {
                if let Some(err) = upload_state.get().error {
                    view! {
                        <div style="padding: 0.5rem; background: rgba(239, 68, 68, 0.1); border: 1px solid #ef4444; border-radius: 0.25rem; color: #ef4444; font-size: 0.75rem; margin-top: 0.5rem;">
                            {err}
                        </div>
                    }.into_any()
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

/// Upload files to GCS via the API
async fn upload_files_to_gcs(content_type: &str, files: Vec<web_sys::File>) -> Result<UploadResponse, String> {
    use gloo_net::http::Request;

    // Create form data
    let form_data = web_sys::FormData::new()
        .map_err(|e| format!("Failed to create form: {:?}", e))?;

    // Add content_type as resource_id (becomes the folder path)
    form_data.append_with_str("resource_id", content_type)
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
