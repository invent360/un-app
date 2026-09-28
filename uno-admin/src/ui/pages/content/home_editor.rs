//! Home Page Content Editor
//!
//! Full-page editor for the home page content, following the same
//! pattern as the task and guide editors.

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use crate::api::section_types::*;
use crate::api::home_client::*;
use crate::api::schema_types::ContentItemStatus;
use crate::components::common::icon::{Icon, IconName};
use crate::components::forms::HomeSectionEditor;

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;

/// Home Page Content Editor
#[component]
pub fn HomePageEditor() -> impl IntoView {
    let navigate = use_navigate();

    // State
    let item_id = RwSignal::new(None::<String>);
    let title = RwSignal::new("Home Page".to_string());
    let sections = RwSignal::new(Vec::<serde_json::Value>::new());
    let status = RwSignal::new(ContentItemStatus::Draft);
    let version = RwSignal::new(0);
    let has_published_version = RwSignal::new(false);

    let is_loading = RwSignal::new(true);
    let is_saving = RwSignal::new(false);
    let is_publishing = RwSignal::new(false);
    let is_previewing = RwSignal::new(false);
    let save_error = RwSignal::new(None::<String>);
    let save_success = RwSignal::new(false);
    let has_changes = RwSignal::new(false);
    let preview_url = RwSignal::new(None::<String>);

    // Load existing home page data on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match get_home_page().await {
                Ok(response) => {
                    item_id.set(response.id);
                    title.set(response.data.title);
                    sections.set(
                        response.data.sections.iter()
                            .map(|s| s.to_json())
                            .collect()
                    );
                    status.set(response.status);
                    version.set(response.version);
                    has_published_version.set(response.has_published_version);
                    is_loading.set(false);
                }
                Err(e) => {
                    // Log error but allow editing (will create new on save)
                    leptos::logging::log!("Error loading home page: {}", e);
                    is_loading.set(false);
                }
            }
        });
    });

    // Track changes
    let on_sections_change = Callback::new(move |new_sections: Vec<serde_json::Value>| {
        sections.set(new_sections);
        has_changes.set(true);
        save_success.set(false);
    });

    // Save handler
    let on_save = move |_| {
        is_saving.set(true);
        save_error.set(None);
        save_success.set(false);

        let current_id = item_id.get();
        let current_title = title.get();
        let current_sections = sections.get();

        spawn_local(async move {
            // Build HomePageData
            let home_data = serde_json::json!({
                "title": current_title,
                "sections": current_sections
            });

            let data_json = serde_json::to_string(&home_data).unwrap_or_default();

            match save_home_page(
                current_id,
                data_json,
                None,
                Some("Updated home page".to_string()),
            ).await {
                Ok(new_id) => {
                    item_id.set(Some(new_id));
                    status.set(ContentItemStatus::Draft);
                    version.update(|v| *v += 1);
                    has_changes.set(false);
                    save_success.set(true);
                    is_saving.set(false);

                    // Clear success message after 3 seconds
                    set_timeout(move || {
                        save_success.set(false);
                    }, std::time::Duration::from_secs(3));
                }
                Err(e) => {
                    save_error.set(Some(e.to_string()));
                    is_saving.set(false);
                }
            }
        });
    };

    // Publish handler
    let on_publish = move |_| {
        if let Some(id) = item_id.get() {
            // First save any pending changes
            if has_changes.get() {
                save_error.set(Some("Please save your changes before publishing".to_string()));
                return;
            }

            is_publishing.set(true);
            save_error.set(None);

            spawn_local(async move {
                match publish_home_page(id).await {
                    Ok(_) => {
                        status.set(ContentItemStatus::Published);
                        has_published_version.set(true);
                        is_publishing.set(false);
                    }
                    Err(e) => {
                        save_error.set(Some(format!("Publish failed: {}", e)));
                        is_publishing.set(false);
                    }
                }
            });
        } else {
            save_error.set(Some("Please save the home page first before publishing".to_string()));
        }
    };

    // Preview handler
    let on_preview = move |_| {
        if let Some(id) = item_id.get() {
            // Save any pending changes first
            if has_changes.get() {
                save_error.set(Some("Please save your changes before previewing".to_string()));
                return;
            }

            is_previewing.set(true);
            save_error.set(None);

            spawn_local(async move {
                match create_home_preview(id).await {
                    Ok(url) => {
                        preview_url.set(Some(url));
                        is_previewing.set(false);
                    }
                    Err(e) => {
                        save_error.set(Some(format!("Preview failed: {}", e)));
                        is_previewing.set(false);
                    }
                }
            });
        } else {
            save_error.set(Some("Please save the home page first before previewing".to_string()));
        }
    };

    // Status badge color
    let status_color = move || {
        match status.get() {
            ContentItemStatus::Draft => "#f59e0b",
            ContentItemStatus::PendingReview => "#3b82f6",
            ContentItemStatus::Approved => "#8b5cf6",
            ContentItemStatus::Published => "#22c55e",
            ContentItemStatus::Scheduled => "#06b6d4",
            ContentItemStatus::Archived => "#6b7280",
        }
    };

    let status_text = move || {
        match status.get() {
            ContentItemStatus::Draft => "Draft",
            ContentItemStatus::PendingReview => "Pending Review",
            ContentItemStatus::Approved => "Approved",
            ContentItemStatus::Published => "Published",
            ContentItemStatus::Scheduled => "Scheduled",
            ContentItemStatus::Archived => "Archived",
        }
    };

    view! {
        <div class="home-page-editor" style="min-height: 100vh; background: #0f172a;">
            // Header
            <div style="position: sticky; top: 0; z-index: 100; background: #1e293b; border-bottom: 1px solid #334155; padding: 1rem 1.5rem;">
                <div style="display: flex; justify-content: space-between; align-items: center; max-width: 1200px; margin: 0 auto;">
                    <div style="display: flex; align-items: center; gap: 1rem;">
                        <button
                            type="button"
                            style="background: transparent; border: none; color: #94a3b8; cursor: pointer; padding: 0.5rem; display: flex; align-items: center;"
                            title="Back to content list"
                            on:click=move |_| {
                                let nav = navigate.clone();
                                nav("/content", Default::default());
                            }
                        >
                            <Icon name=IconName::ArrowLeft size=24 />
                        </button>
                        <div>
                            <h1 style="color: #e2e8f0; font-size: 1.25rem; font-weight: 600; margin: 0; display: flex; align-items: center; gap: 0.5rem;">
                                <Icon name=IconName::Home size=24 />
                                "Home Page Editor"
                            </h1>
                            <div style="display: flex; align-items: center; gap: 0.5rem; margin-top: 0.25rem;">
                                <span style={format!(
                                    "background: {}; color: white; font-size: 0.65rem; padding: 0.125rem 0.5rem; border-radius: 9999px; text-transform: uppercase; font-weight: 600;",
                                    status_color()
                                )}>
                                    {status_text}
                                </span>
                                <span style="color: #64748b; font-size: 0.75rem;">
                                    {move || format!("v{}", version.get())}
                                </span>
                                {move || if has_changes.get() {
                                    view! {
                                        <span style="color: #f59e0b; font-size: 0.75rem;">
                                            "(unsaved changes)"
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        </div>
                    </div>

                    <div style="display: flex; align-items: center; gap: 0.75rem;">
                        // Save Success Indicator
                        {move || if save_success.get() {
                            view! {
                                <span style="display: flex; align-items: center; gap: 0.25rem; color: #22c55e; font-size: 0.875rem;">
                                    <Icon name=IconName::CheckCircle size=16 />
                                    "Saved"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Save Draft Button
                        <button
                            type="button"
                            style="background: #334155; color: #e2e8f0; border: none; border-radius: 0.375rem; padding: 0.625rem 1rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; gap: 0.5rem;"
                            disabled=move || is_saving.get() || is_loading.get()
                            on:click=on_save
                        >
                            {move || if is_saving.get() {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <span class="spinner" style="width: 16px; height: 16px; border: 2px solid #64748b; border-top-color: white; border-radius: 50%; animation: spin 1s linear infinite;"></span>
                                        "Saving..."
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <Icon name=IconName::Document size=16 />
                                        "Save Draft"
                                    </span>
                                }.into_any()
                            }}
                        </button>

                        // Preview Button
                        <button
                            type="button"
                            style="background: #0ea5e9; color: white; border: none; border-radius: 0.375rem; padding: 0.625rem 1rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; gap: 0.5rem;"
                            disabled=move || is_previewing.get() || is_loading.get() || item_id.get().is_none()
                            on:click=on_preview
                        >
                            {move || if is_previewing.get() {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <span class="spinner" style="width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 1s linear infinite;"></span>
                                        "Generating..."
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <Icon name=IconName::Eye size=16 />
                                        "Preview"
                                    </span>
                                }.into_any()
                            }}
                        </button>

                        // Publish Button
                        <button
                            type="button"
                            style="background: #22c55e; color: white; border: none; border-radius: 0.375rem; padding: 0.625rem 1rem; cursor: pointer; font-size: 0.875rem; display: flex; align-items: center; gap: 0.5rem;"
                            disabled=move || is_publishing.get() || is_loading.get() || item_id.get().is_none()
                            on:click=on_publish
                        >
                            {move || if is_publishing.get() {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <span class="spinner" style="width: 16px; height: 16px; border: 2px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%; animation: spin 1s linear infinite;"></span>
                                        "Publishing..."
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span style="display: flex; align-items: center; gap: 0.5rem;">
                                        <Icon name=IconName::Rocket size=16 />
                                        "Publish"
                                    </span>
                                }.into_any()
                            }}
                        </button>
                    </div>
                </div>
            </div>

            // Error Message
            {move || if let Some(error) = save_error.get() {
                view! {
                    <div style="max-width: 1200px; margin: 1rem auto 0; padding: 0 1.5rem;">
                        <div style="background: rgba(239, 68, 68, 0.1); border: 1px solid #ef4444; border-radius: 0.375rem; padding: 0.75rem 1rem; display: flex; align-items: center; gap: 0.5rem;">
                            <Icon name=IconName::Close size=16 />
                            <span style="color: #ef4444; font-size: 0.875rem;">{error}</span>
                            <button
                                type="button"
                                style="margin-left: auto; background: transparent; border: none; color: #ef4444; cursor: pointer; padding: 0.25rem;"
                                on:click=move |_| save_error.set(None)
                            >
                                <Icon name=IconName::Close size=16 />
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Preview URL Popup
            {move || {
                if let Some(url) = preview_url.get() {
                    let url_for_link = url.clone();
                    let url_display = url.clone();
                    view! {
                        <div style="position: fixed; top: 80px; right: 20px; z-index: 1000; max-width: 500px; background: linear-gradient(135deg, #1e3a5f 0%, #0f172a 100%); border: 1px solid #06b6d4; border-radius: 12px; padding: 16px; box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);">
                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
                                <span style="font-weight: 600; color: #22d3ee; font-size: 14px;">"Preview Link Generated"</span>
                                <button
                                    type="button"
                                    style="background: none; border: none; color: #64748b; cursor: pointer; padding: 4px;"
                                    on:click=move |_| preview_url.set(None)
                                >
                                    <Icon name=IconName::Close size=18 />
                                </button>
                            </div>
                            <div style="background: #0f172a; border-radius: 8px; padding: 12px; margin-bottom: 12px;">
                                <code style="color: #94a3b8; font-size: 12px; word-break: break-all;">
                                    {url_display}
                                </code>
                            </div>
                            <div style="display: flex; gap: 8px; justify-content: flex-end;">
                                <a
                                    href=url_for_link
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    style="padding: 10px 16px; background: #0ea5e9; color: white; border-radius: 6px; font-weight: 500; font-size: 13px; text-decoration: none; display: flex; align-items: center; gap: 6px;"
                                >
                                    <Icon name=IconName::Eye size=16 />
                                    "Open Preview"
                                </a>
                                <button
                                    type="button"
                                    style="padding: 10px 16px; background: #334155; color: #e2e8f0; border: none; border-radius: 6px; font-weight: 500; font-size: 13px; cursor: pointer;"
                                    on:click=move |_| preview_url.set(None)
                                >
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Main Content
            <div style="max-width: 1200px; margin: 0 auto; padding: 1.5rem;">
                {move || if is_loading.get() {
                    view! {
                        <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 4rem; color: #64748b;">
                            <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #334155; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 1rem;"></div>
                            "Loading home page content..."
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                            // Page Title Card
                            <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem;">
                                <h2 style="color: #e2e8f0; font-size: 1rem; font-weight: 600; margin: 0 0 1rem 0; display: flex; align-items: center; gap: 0.5rem;">
                                    <Icon name=IconName::Document size=18 />
                                    "Page Settings"
                                </h2>
                                <div>
                                    <label style="display: block; color: #94a3b8; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 500;">
                                        "Page Title (Internal)"
                                    </label>
                                    <input
                                        type="text"
                                        style="width: 100%; max-width: 400px; background: #0f172a; border: 1px solid #334155; border-radius: 0.375rem; padding: 0.625rem 0.75rem; color: #e2e8f0; font-size: 0.875rem; outline: none;"
                                        placeholder="Home Page"
                                        prop:value=move || title.get()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            title.set(val);
                                            has_changes.set(true);
                                            save_success.set(false);
                                        }
                                    />
                                    <div style="font-size: 0.75rem; color: #64748b; margin-top: 0.25rem;">
                                        "This is for internal reference only and won't be shown on the page"
                                    </div>
                                </div>
                            </div>

                            // Sections Editor Card
                            <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem;">
                                <h2 style="color: #e2e8f0; font-size: 1rem; font-weight: 600; margin: 0 0 1rem 0; display: flex; align-items: center; gap: 0.5rem;">
                                    <Icon name=IconName::Folder size=18 />
                                    "Page Sections"
                                </h2>
                                <p style="color: #64748b; font-size: 0.875rem; margin-bottom: 1rem;">
                                    "Add and arrange sections to build your home page. Drag to reorder, click to edit."
                                </p>
                                <HomeSectionEditor
                                    value=sections
                                    on_change=on_sections_change
                                    read_only=false
                                />
                            </div>

                            // Quick Stats Card
                            <div style="background: #1e293b; border: 1px solid #334155; border-radius: 0.5rem; padding: 1.5rem;">
                                <h2 style="color: #e2e8f0; font-size: 1rem; font-weight: 600; margin: 0 0 1rem 0; display: flex; align-items: center; gap: 0.5rem;">
                                    <Icon name=IconName::Analytics size=18 />
                                    "Quick Stats"
                                </h2>
                                <div style="display: flex; gap: 2rem; flex-wrap: wrap;">
                                    <div>
                                        <div style="color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">Total Sections</div>
                                        <div style="color: #e2e8f0; font-size: 1.5rem; font-weight: 600;">
                                            {move || sections.get().len()}
                                        </div>
                                    </div>
                                    <div>
                                        <div style="color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">Visible Sections</div>
                                        <div style="color: #22c55e; font-size: 1.5rem; font-weight: 600;">
                                            {move || {
                                                sections.get().iter()
                                                    .filter(|s| s.get("is_visible").and_then(|v| v.as_bool()).unwrap_or(true))
                                                    .count()
                                            }}
                                        </div>
                                    </div>
                                    <div>
                                        <div style="color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">Version</div>
                                        <div style="color: #3b82f6; font-size: 1.5rem; font-weight: 600;">
                                            {move || version.get()}
                                        </div>
                                    </div>
                                    <div>
                                        <div style="color: #64748b; font-size: 0.75rem; margin-bottom: 0.25rem;">Published</div>
                                        <div style={move || if has_published_version.get() {
                                            "color: #22c55e; font-size: 1.5rem; font-weight: 600;"
                                        } else {
                                            "color: #f59e0b; font-size: 1.5rem; font-weight: 600;"
                                        }}>
                                            {move || if has_published_version.get() { "Yes" } else { "No" }}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>

            // CSS for spinner animation
            <style>
                {"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}
            </style>
        </div>
    }
}
