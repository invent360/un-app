//! Upload Leptos component.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::types::{UploadListType, UploadFile, UploadFileStatus};
use crate::try_use_theme;

/// Upload component.
///
/// File upload with drag and drop support.
///
/// # Props
///
/// - `file_list` - List of uploaded files
/// - `accept` - Accepted file types
/// - `multiple` - Allow multiple files
/// - `disabled` - Disabled state
/// - `list_type` - File list display type
/// - `max_count` - Maximum number of files
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::{Upload, UploadFile};
///
/// let files = RwSignal::new(Vec::<UploadFile>::new());
///
/// view! {
///     <Upload file_list=files>
///         <button>"Click to Upload"</button>
///     </Upload>
/// }
/// ```
#[component]
pub fn Upload(
    /// List of uploaded files.
    #[prop(into)]
    file_list: RwSignal<Vec<UploadFile>>,
    /// Accepted file types (e.g., "image/*,.pdf").
    #[prop(optional, into)]
    accept: Option<String>,
    /// Allow multiple files.
    #[prop(optional)]
    multiple: bool,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// File list display type.
    #[prop(optional, into)]
    list_type: Option<UploadListType>,
    /// Maximum number of files.
    #[prop(optional)]
    max_count: Option<usize>,
    /// Show upload list.
    #[prop(optional)]
    show_upload_list: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<UploadFile>>>,
    /// Remove callback.
    #[prop(optional, into)]
    on_remove: Option<Callback<UploadFile>>,
    /// Upload trigger content.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let list_type = list_type.unwrap_or_default();
    let show_upload_list = show_upload_list.unwrap_or(true);

    // Build CSS classes
    let upload_prefix = format!("fx-upload-{}", design_system);
    let list_type_class = list_type.class(&upload_prefix);

    let upload_prefix_for_class = upload_prefix.clone();
    let upload_prefix_for_input = upload_prefix.clone();
    let upload_prefix_for_list = upload_prefix.clone();

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![upload_prefix_for_class.clone(), list_type_class.clone()];
            if disabled {
                parts.push(format!("{}-disabled", upload_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Handle file selection
    let handle_files = move |files: web_sys::FileList| {
        let mut current_files = file_list.get();
        let max = max_count.unwrap_or(usize::MAX);

        for i in 0..files.length() {
            if current_files.len() >= max {
                break;
            }

            if let Some(file) = files.get(i) {
                let uid = format!("rc-upload-{}", js_sys::Date::now() as u64 + i as u64);
                let name = file.name();
                let size = file.size() as u64;
                let file_type = file.type_();

                let upload_file = UploadFile::new(uid, name)
                    .size(size)
                    .file_type(file_type)
                    .status(UploadFileStatus::Done);

                current_files.push(upload_file);
            }
        }

        file_list.set(current_files.clone());
        if let Some(ref cb) = on_change {
            cb.run(current_files);
        }
    };

    // Handle remove
    let handle_remove = move |file: UploadFile| {
        let mut files = file_list.get();
        files.retain(|f| f.uid != file.uid);
        file_list.set(files.clone());

        if let Some(ref cb) = on_remove {
            cb.run(file);
        }
        if let Some(ref cb) = on_change {
            cb.run(files);
        }
    };

    view! {
        <div class=combined_class>
            // Upload trigger
            <div class=format!("{}-select", upload_prefix_for_input)>
                <input
                    type="file"
                    class=format!("{}-input", upload_prefix_for_input)
                    accept=accept.clone().unwrap_or_default()
                    multiple=multiple
                    disabled=disabled
                    on:change=move |ev: web_sys::Event| {
                        if let Some(target) = ev.target() {
                            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                if let Some(files) = input.files() {
                                    handle_files(files);
                                }
                                // Reset input value to allow selecting same file again
                                input.set_value("");
                            }
                        }
                    }
                />
                {children()}
            </div>

            // File list
            {if show_upload_list {
                let upload_prefix = upload_prefix_for_list.clone();
                let upload_prefix_for_items = upload_prefix.clone();
                Some(view! {
                    <div class=format!("{}-list", upload_prefix)>
                        {move || {
                            file_list.get().into_iter().map(|file| {
                                let file_for_remove = file.clone();
                                let upload_prefix = upload_prefix_for_items.clone();
                                let status_class = file.status.class(&upload_prefix);

                                view! {
                                    <div class=format!("{}-list-item {}", upload_prefix, status_class)>
                                        <div class=format!("{}-list-item-info", upload_prefix)>
                                            <span class=format!("{}-list-item-icon", upload_prefix)>
                                                {match list_type {
                                                    UploadListType::Picture | UploadListType::PictureCard => {
                                                        if let Some(ref thumb) = file.thumb_url {
                                                            view! {
                                                                <img
                                                                    src=thumb.clone()
                                                                    alt=file.name.clone()
                                                                    class=format!("{}-list-item-thumbnail", upload_prefix)
                                                                />
                                                            }.into_any()
                                                        } else {
                                                            view! { <span>"📄"</span> }.into_any()
                                                        }
                                                    },
                                                    UploadListType::Text => {
                                                        view! { <span>"📎"</span> }.into_any()
                                                    }
                                                }}
                                            </span>
                                            <span class=format!("{}-list-item-name", upload_prefix)>
                                                {file.name.clone()}
                                            </span>
                                        </div>
                                        <div class=format!("{}-list-item-actions", upload_prefix)>
                                            {match file.status {
                                                UploadFileStatus::Uploading => {
                                                    view! {
                                                        <span class=format!("{}-list-item-progress", upload_prefix)>
                                                            {format!("{}%", file.percent as i32)}
                                                        </span>
                                                    }.into_any()
                                                },
                                                UploadFileStatus::Error => {
                                                    view! {
                                                        <span class=format!("{}-list-item-error", upload_prefix)>
                                                            {file.error.clone().unwrap_or_default()}
                                                        </span>
                                                    }.into_any()
                                                },
                                                _ => {
                                                    view! { <span></span> }.into_any()
                                                }
                                            }}
                                            <button
                                                class=format!("{}-list-item-remove", upload_prefix)
                                                on:click=move |_| handle_remove(file_for_remove.clone())
                                            >
                                                "×"
                                            </button>
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

/// Upload Dragger component.
///
/// Drag and drop upload area.
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::{UploadDragger, UploadFile};
///
/// let files = RwSignal::new(Vec::<UploadFile>::new());
///
/// view! {
///     <UploadDragger file_list=files />
/// }
/// ```
#[component]
pub fn UploadDragger(
    /// List of uploaded files.
    #[prop(into)]
    file_list: RwSignal<Vec<UploadFile>>,
    /// Accepted file types.
    #[prop(optional, into)]
    accept: Option<String>,
    /// Allow multiple files.
    #[prop(optional)]
    multiple: bool,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Maximum number of files.
    #[prop(optional)]
    max_count: Option<usize>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<UploadFile>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Build CSS classes
    let upload_prefix = format!("fx-upload-{}", design_system);

    let upload_prefix_for_class = upload_prefix.clone();
    let upload_prefix_for_dragger = upload_prefix.clone();

    // State
    let is_dragging = RwSignal::new(false);

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![
                upload_prefix_for_class.clone(),
                format!("{}-dragger", upload_prefix_for_class),
            ];
            if disabled {
                parts.push(format!("{}-disabled", upload_prefix_for_class));
            }
            if is_dragging.get() {
                parts.push(format!("{}-drag-over", upload_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Handle file drop
    let handle_files = move |files: web_sys::FileList| {
        let mut current_files = file_list.get();
        let max = max_count.unwrap_or(usize::MAX);

        for i in 0..files.length() {
            if current_files.len() >= max {
                break;
            }

            if let Some(file) = files.get(i) {
                let uid = format!("rc-upload-{}", js_sys::Date::now() as u64 + i as u64);
                let name = file.name();
                let size = file.size() as u64;
                let file_type = file.type_();

                let upload_file = UploadFile::new(uid, name)
                    .size(size)
                    .file_type(file_type)
                    .status(UploadFileStatus::Done);

                current_files.push(upload_file);
            }
        }

        file_list.set(current_files.clone());
        if let Some(ref cb) = on_change {
            cb.run(current_files);
        }
    };

    view! {
        <div
            class=combined_class
            on:dragenter=move |ev| {
                ev.prevent_default();
                if !disabled {
                    is_dragging.set(true);
                }
            }
            on:dragover=move |ev| {
                ev.prevent_default();
            }
            on:dragleave=move |ev| {
                ev.prevent_default();
                is_dragging.set(false);
            }
            on:drop=move |ev| {
                ev.prevent_default();
                is_dragging.set(false);
                if disabled {
                    return;
                }
                if let Some(data_transfer) = ev.data_transfer() {
                    if let Some(files) = data_transfer.files() {
                        handle_files(files);
                    }
                }
            }
        >
            <input
                type="file"
                class=format!("{}-input", upload_prefix_for_dragger)
                accept=accept.clone().unwrap_or_default()
                multiple=multiple
                disabled=disabled
                on:change=move |ev: web_sys::Event| {
                    if let Some(target) = ev.target() {
                        if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                            if let Some(files) = input.files() {
                                handle_files(files);
                            }
                            input.set_value("");
                        }
                    }
                }
            />
            <div class=format!("{}-dragger-content", upload_prefix_for_dragger)>
                <p class=format!("{}-drag-icon", upload_prefix_for_dragger)>
                    "📥"
                </p>
                <p class=format!("{}-text", upload_prefix_for_dragger)>
                    "Click or drag file to this area to upload"
                </p>
                <p class=format!("{}-hint", upload_prefix_for_dragger)>
                    "Support for a single or bulk upload."
                </p>
            </div>
        </div>
    }
}
