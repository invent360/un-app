//! Media field component for image/file uploads
//!
//! Supports both file upload (to cloud storage) and URL input.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::api::schema_types::{MediaFieldConfig, MediaType};

const LABEL_STYLE: &str = "display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;";
const DROPZONE_STYLE: &str = "border: 2px dashed #334155; border-radius: 0.375rem; padding: 1.5rem; text-align: center; cursor: pointer; transition: border-color 0.2s, background 0.2s;";
const DROPZONE_ACTIVE_STYLE: &str = "border: 2px dashed #3b82f6; border-radius: 0.375rem; padding: 1.5rem; text-align: center; cursor: pointer; background: rgba(59, 130, 246, 0.1);";
const PREVIEW_STYLE: &str = "width: 100%; max-width: 200px; border-radius: 0.375rem; margin-top: 0.5rem;";
const INPUT_STYLE: &str = "width: 100%; background: #1e293b; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;";
const HELP_TEXT_STYLE: &str = "color: #64748b; font-size: 0.75rem; margin-top: 0.5rem;";
const REQUIRED_STYLE: &str = "color: #ef4444; margin-left: 0.25rem;";
const REMOVE_BTN_STYLE: &str = "background: #ef4444; border: none; border-radius: 0.25rem; padding: 0.25rem 0.5rem; color: white; font-size: 0.75rem; cursor: pointer; margin-top: 0.5rem;";
const TAB_STYLE: &str = "padding: 0.5rem 1rem; background: transparent; border: none; color: #64748b; cursor: pointer; font-size: 0.875rem; border-bottom: 2px solid transparent;";
const TAB_ACTIVE_STYLE: &str = "padding: 0.5rem 1rem; background: transparent; border: none; color: #3b82f6; cursor: pointer; font-size: 0.875rem; border-bottom: 2px solid #3b82f6;";
const PROGRESS_STYLE: &str = "width: 100%; height: 4px; background: #334155; border-radius: 2px; margin-top: 0.5rem; overflow: hidden;";
const PROGRESS_BAR_STYLE: &str = "height: 100%; background: #3b82f6; transition: width 0.3s ease;";
const ERROR_STYLE: &str = "color: #ef4444; font-size: 0.75rem; margin-top: 0.5rem;";
const SUCCESS_STYLE: &str = "color: #22c55e; font-size: 0.75rem; margin-top: 0.5rem;";

/// Upload state
#[derive(Clone, Default)]
pub struct UploadState {
    pub is_uploading: bool,
    pub progress: f32,
    pub error: Option<String>,
}

/// Media field for single file/image with upload support
#[component]
pub fn MediaField(
    /// Field key for identification
    #[prop(into)]
    field_key: String,
    /// Field label
    #[prop(into)]
    label: String,
    /// Current value (storage URL)
    value: RwSignal<Option<String>>,
    /// Display URL (for preview, converted from storage URL)
    #[prop(optional)]
    display_value: Option<RwSignal<Option<String>>>,
    /// Field configuration
    #[prop(optional)]
    config: Option<MediaFieldConfig>,
    /// Description/help text
    description: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// On change callback (receives storage URL)
    #[prop(optional)]
    on_change: Option<Callback<Option<String>>>,
    /// Resource ID for grouping uploads (e.g., content ID)
    #[prop(optional, into)]
    resource_id: Option<String>,
    /// Enable file upload (default: true)
    #[prop(default = true)]
    enable_upload: bool,
    /// Enable URL input (default: true)
    #[prop(default = true)]
    enable_url_input: bool,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let allowed_types = config.allowed_types.clone();
    let field_key_clone = field_key.clone();

    // Input mode: "upload" or "url"
    let input_mode = RwSignal::new(if enable_upload { "upload" } else { "url" });

    // Drag state
    let is_dragging = RwSignal::new(false);

    // Upload state
    let upload_state = RwSignal::new(UploadState::default());

    // URL input handler
    let on_url_input = move |ev: web_sys::Event| {
        let url = event_target_value(&ev);
        let new_value = if url.is_empty() { None } else { Some(url) };
        value.set(new_value.clone());
        // For URL input, display value is same as storage value
        if let Some(dv) = display_value {
            dv.set(new_value.clone());
        }
        if let Some(cb) = on_change {
            cb.run(new_value);
        }
    };

    // Clear value
    let clear_value = move |_| {
        value.set(None);
        if let Some(dv) = display_value {
            dv.set(None);
        }
        if let Some(cb) = on_change {
            cb.run(None);
        }
        upload_state.set(UploadState::default());
    };

    // File upload handler - generate resource ID if not provided
    let resource_id_for_upload = resource_id.clone().unwrap_or_else(|| generate_random_id());
    let on_file_selected: Callback<web_sys::FileList> = Callback::new({
        let resource_id = resource_id_for_upload.clone();
        move |files: web_sys::FileList| {
            if files.length() == 0 {
                return;
            }

            let file = match files.get(0) {
                Some(f) => f,
                None => return,
            };

            let file_name = file.name();
            let file_type = file.type_();
            let resource_id = resource_id.clone();

            // Start upload
            upload_state.set(UploadState {
                is_uploading: true,
                progress: 0.0,
                error: None,
            });

            // Read file and upload
            wasm_bindgen_futures::spawn_local(async move {
                // Read file as ArrayBuffer
                let array_buffer = match read_file_as_array_buffer(&file).await {
                    Ok(ab) => ab,
                    Err(e) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 0.0,
                            error: Some(format!("Failed to read file: {:?}", e)),
                        });
                        return;
                    }
                };

                // Convert to bytes
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let bytes = uint8_array.to_vec();

                // Create form data
                let form_data = match web_sys::FormData::new() {
                    Ok(fd) => fd,
                    Err(e) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 0.0,
                            error: Some(format!("Failed to create form: {:?}", e)),
                        });
                        return;
                    }
                };

                // Add resource_id
                if let Err(e) = form_data.append_with_str("resource_id", &resource_id) {
                    tracing::error!("Failed to append resource_id: {:?}", e);
                }

                // Create blob from bytes
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&uint8_array);

                let blob_options = web_sys::BlobPropertyBag::new();
                blob_options.set_type(&file_type);

                let blob = match web_sys::Blob::new_with_u8_array_sequence_and_options(
                    &blob_parts,
                    &blob_options,
                ) {
                    Ok(b) => b,
                    Err(e) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 0.0,
                            error: Some(format!("Failed to create blob: {:?}", e)),
                        });
                        return;
                    }
                };

                // Add file to form
                if let Err(e) = form_data.append_with_blob_and_filename("file", &blob, &file_name) {
                    upload_state.set(UploadState {
                        is_uploading: false,
                        progress: 0.0,
                        error: Some(format!("Failed to append file: {:?}", e)),
                    });
                    return;
                }

                // Upload via fetch
                upload_state.set(UploadState {
                    is_uploading: true,
                    progress: 50.0, // Simulated progress
                    error: None,
                });

                match upload_file_to_server(form_data).await {
                    Ok(response) => {
                        upload_state.set(UploadState {
                            is_uploading: false,
                            progress: 100.0,
                            error: None,
                        });

                        // Set the storage URL
                        if let Some(storage_url) = response.storage_urls.first() {
                            value.set(Some(storage_url.clone()));

                            // Set display URL if provided
                            if let Some(dv) = display_value {
                                if let Some(display_url) = response.display_urls.first() {
                                    dv.set(Some(display_url.clone()));
                                }
                            }

                            if let Some(cb) = on_change {
                                cb.run(Some(storage_url.clone()));
                            }
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

    // Handle file input change - wrap in Callback for use in reactive contexts
    let on_file_change: Callback<web_sys::Event> = Callback::new({
        let on_file_selected = on_file_selected.clone();
        move |ev: web_sys::Event| {
            let input: web_sys::HtmlInputElement = event_target(&ev);
            if let Some(files) = input.files() {
                on_file_selected.run(files);
            }
        }
    });

    // Handle drag events - wrap in Callback for use in reactive contexts
    let on_dragover: Callback<web_sys::DragEvent> = Callback::new(move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        is_dragging.set(true);
    });

    let on_dragleave: Callback<web_sys::DragEvent> = Callback::new(move |_ev: web_sys::DragEvent| {
        is_dragging.set(false);
    });

    let on_drop: Callback<web_sys::DragEvent> = Callback::new({
        let on_file_selected = on_file_selected.clone();
        move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            is_dragging.set(false);

            if let Some(data_transfer) = ev.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    on_file_selected.run(files);
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
    let accept_types = if accept_types.is_empty() { "*/*".to_string() } else { accept_types };

    let allowed_types_str = if allowed_types.is_empty() {
        "All types".to_string()
    } else {
        allowed_types.iter().map(|t| match t {
            MediaType::Image => "Images",
            MediaType::Video => "Videos",
            MediaType::Audio => "Audio",
            MediaType::Document => "Documents",
        }).collect::<Vec<_>>().join(", ")
    };

    // Check if current value is an image
    let is_image = move || {
        let url = display_value.map(|dv| dv.get()).flatten()
            .or_else(|| value.get());

        url.as_ref().map(|v| {
            v.ends_with(".jpg") || v.ends_with(".jpeg") ||
            v.ends_with(".png") || v.ends_with(".gif") ||
            v.ends_with(".webp") || v.ends_with(".svg") ||
            v.starts_with("data:image/") ||
            v.contains("X-Goog-Signature") // Signed GCS URLs
        }).unwrap_or(false)
    };

    // Get preview URL
    let preview_url = move || {
        display_value.map(|dv| dv.get()).flatten()
            .or_else(|| value.get())
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

            // Mode tabs (only show if both modes enabled)
            {move || {
                if enable_upload && enable_url_input {
                    view! {
                        <div style="display: flex; margin-bottom: 0.5rem; border-bottom: 1px solid #334155;">
                            <button
                                type="button"
                                style=move || if input_mode.get() == "upload" { TAB_ACTIVE_STYLE } else { TAB_STYLE }
                                on:click=move |_| input_mode.set("upload")
                            >
                                "Upload"
                            </button>
                            <button
                                type="button"
                                style=move || if input_mode.get() == "url" { TAB_ACTIVE_STYLE } else { TAB_STYLE }
                                on:click=move |_| input_mode.set("url")
                            >
                                "URL"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Upload dropzone
            {
                let on_dragover = on_dragover.clone();
                let on_dragleave = on_dragleave.clone();
                let on_drop = on_drop.clone();
                let on_file_change = on_file_change.clone();
                move || {
                    if input_mode.get() == "upload" && enable_upload {
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
                                        // Trigger file input click
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
                                                "Drop file here or click to browse"
                                            </p>
                                            <p style="color: #64748b; font-size: 0.75rem; margin: 0;">
                                                {format!("Allowed: {}", allowed_types_str)}
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
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // URL input
            {move || {
                if input_mode.get() == "url" && enable_url_input {
                    view! {
                        <input
                            type="text"
                            style=INPUT_STYLE
                            prop:value=move || value.get().unwrap_or_default()
                            on:input=on_url_input
                            placeholder="Enter media URL..."
                        />
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Preview
            {move || {
                if let Some(ref url) = preview_url() {
                    if is_image() {
                        view! {
                            <div style="margin-top: 0.5rem;">
                                <img src=url.clone() style=PREVIEW_STYLE alt="Preview" />
                                <br />
                                <button type="button" style=REMOVE_BTN_STYLE on:click=clear_value>
                                    "Remove"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div style="margin-top: 0.5rem;">
                                <span style="color: #e2e8f0; font-size: 0.875rem;">
                                    "File uploaded"
                                </span>
                                <br />
                                <button type="button" style=REMOVE_BTN_STYLE on:click=clear_value>
                                    "Remove"
                                </button>
                            </div>
                        }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

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

/// Upload response from server
#[derive(Clone, Debug, serde::Deserialize)]
pub struct UploadResponse {
    pub storage_urls: Vec<String>,
    pub display_urls: Vec<String>,
    pub uploaded_count: usize,
    pub total_bytes: u64,
    pub errors: Vec<String>,
}

/// Read file as ArrayBuffer
async fn read_file_as_array_buffer(file: &web_sys::File) -> Result<js_sys::ArrayBuffer, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let promise = file.array_buffer();
    let result = JsFuture::from(promise).await?;
    result.dyn_into::<js_sys::ArrayBuffer>()
}

/// Upload file to server
async fn upload_file_to_server(form_data: web_sys::FormData) -> Result<UploadResponse, String> {
    use gloo_net::http::Request;

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

/// Generate a random ID using crypto API (for resource grouping)
fn generate_random_id() -> String {
    use js_sys::Math;

    // Generate a simple random ID using Math.random
    // Format: 8 hex chars - 4 hex chars - 4 hex chars - 12 hex chars
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
