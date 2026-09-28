//! Content list page for CMS management
//!
//! Uses schema-driven content items API.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use web_sys;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
use crate::components::common::progress_spinner::{ProgressSpinner, LoadingOverlay, SpinnerSize};
use chrono::{DateTime, Utc};
use crate::api::schema_types::{ContentItemSummary, ContentItemListResponse, ContentItemStatus};
use crate::api::schema_client::{get_schemas, list_content_items, publish_content_item, delete_content_item};
use crate::pages::cms::CmsMenuBar;
use crate::context::use_user_role;
use crate::pages::content::create_schema_content_preview;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusFilter {
    #[default]
    All,
    Draft,
    PendingReview,
    Approved,
    Published,
    Scheduled,
    Archived,
}

impl StatusFilter {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            StatusFilter::All => None,
            StatusFilter::Draft => Some("draft"),
            StatusFilter::PendingReview => Some("pending_review"),
            StatusFilter::Approved => Some("approved"),
            StatusFilter::Published => Some("published"),
            StatusFilter::Scheduled => Some("scheduled"),
            StatusFilter::Archived => Some("archived"),
        }
    }
}

/// Content list page
#[component]
pub fn ContentListPage() -> impl IntoView {
    let navigate = use_navigate();
    let role_ctx = use_user_role();
    let can_bulk_import = move || role_ctx.user.get().role.can_bulk_import();

    // Fetch available schemas for the type filter
    let schemas_resource = Resource::new(
        || (),
        |_| async move {
            get_schemas().await.ok().unwrap_or_default()
        }
    );

    // Filter state - type filter is now a String (schema_id) instead of enum
    let (type_filter, set_type_filter) = signal(String::new()); // Empty = All
    let (status_filter, set_status_filter) = signal(StatusFilter::All);
    let (search_query, set_search_query) = signal(String::new());
    let (current_page, set_current_page) = signal(1i32);
    let (per_page, set_per_page) = signal(5i32);

    // Create reactive resource that depends on all filter signals
    let content_resource = Resource::new(
        move || (
            type_filter.get(),
            status_filter.get(),
            search_query.get(),
            current_page.get(),
            per_page.get(),
        ),
        |(type_f, status_f, search, page, pp)| async move {
            list_content_items(
                if type_f.is_empty() { None } else { Some(type_f) },
                status_f.as_str().map(String::from),
                if search.is_empty() { None } else { Some(search) },
                Some(page),
                Some(pp),
            ).await
        }
    );

    // Navigation handlers
    let nav = navigate.clone();
    let on_edit_click = move |id: String| {
        nav(&format!("/content/{}", id), Default::default());
    };

    let nav_view = navigate.clone();
    let on_view_click = move |id: String| {
        nav_view(&format!("/content/{}?mode=view", id), Default::default());
    };

    let nav_new = navigate.clone();
    let on_new_click = move |_| {
        nav_new("/content/new", Default::default());
    };

    view! {
        <div>
            <Header title="CMS".to_string() show_search=false />
            <CmsMenuBar />

            <div class="px-4 py-4 space-y-4">
                // Action bar
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold text-slate-900 dark:text-white">
                        "All Content"
                    </h2>
                    <div class="flex items-center gap-3">
                        // Import button - only for publishers/admins
                        {move || can_bulk_import().then(|| view! {
                            <a
                                href="/content/import"
                                class="flex items-center justify-center gap-2 px-4 py-2 bg-slate-600 text-white rounded-lg hover:bg-slate-500 transition font-medium"
                            >
                                <Icon name=IconName::Document size=18 />
                                "Import"
                            </a>
                        })}
                        <button
                            class="flex items-center justify-center gap-2 px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition font-medium"
                            on:click=on_new_click
                        >
                            <Icon name=IconName::Plus size=18 />
                            "New Content"
                        </button>
                    </div>
                </div>

                // Filters
                <div class="rounded-2xl" style="background: #0f172a; border: 1px solid #1e293b; padding: 16px;">
                    <div class="flex flex-wrap items-center gap-4">
                        // Content type filter - dynamically populated from schemas
                        <div class="flex items-center gap-2">
                            <label class="text-sm text-slate-400">"Type:"</label>
                            <Suspense fallback=move || view! {
                                <select
                                    class="text-sm text-white/90"
                                    style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px;"
                                    disabled=true
                                >
                                    <option>"Loading..."</option>
                                </select>
                            }>
                                {move || {
                                    let schemas = schemas_resource.get().unwrap_or_default();
                                    view! {
                                        <select
                                            class="text-sm text-white/90 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                            style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                set_type_filter.set(if value == "all" { String::new() } else { value });
                                                set_current_page.set(1);
                                            }
                                        >
                                            <option value="all">"All Types"</option>
                                            {schemas.into_iter().map(|schema| {
                                                let schema_id = schema.id.clone();
                                                let schema_name = schema.name.clone();
                                                view! {
                                                    <option value=schema_id>{schema_name}</option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    }
                                }}
                            </Suspense>
                        </div>

                        // Status filter
                        <div class="flex items-center gap-2">
                            <label class="text-sm text-slate-400">"Status:"</label>
                            <select
                                class="text-sm text-white/90 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_status_filter.set(match value.as_str() {
                                        "draft" => StatusFilter::Draft,
                                        "pending_review" => StatusFilter::PendingReview,
                                        "approved" => StatusFilter::Approved,
                                        "published" => StatusFilter::Published,
                                        "scheduled" => StatusFilter::Scheduled,
                                        "archived" => StatusFilter::Archived,
                                        _ => StatusFilter::All,
                                    });
                                    set_current_page.set(1);
                                }
                            >
                                <option value="all">"All Status"</option>
                                <option value="draft">"Draft"</option>
                                <option value="pending_review">"Pending Review"</option>
                                <option value="approved">"Approved"</option>
                                <option value="published">"Published"</option>
                                <option value="scheduled">"Scheduled"</option>
                                <option value="archived">"Archived"</option>
                            </select>
                        </div>

                        // Search input
                        <div class="flex-1 min-w-[200px]">
                            <input
                                type="text"
                                placeholder="Search content..."
                                class="w-full text-sm text-white/90 placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 12px;"
                                on:input=move |ev| {
                                    set_search_query.set(event_target_value(&ev));
                                    set_current_page.set(1);
                                }
                            />
                        </div>

                        // Per page selector
                        <div class="flex items-center gap-2">
                            <label class="text-sm text-slate-400">"Show:"</label>
                            <select
                                class="text-sm text-white/90 focus:outline-none focus:ring-2 focus:ring-primary-500 transition-all"
                                style="background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 8px 32px 8px 12px; appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%2212%22 viewBox=%220 0 12 12%22%3E%3Cpath fill=%22%2394a3b8%22 d=%22M2 4l4 4 4-4%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 12px center;"
                                on:change=move |ev| {
                                    let value: i32 = event_target_value(&ev).parse().unwrap_or(5);
                                    set_per_page.set(value);
                                    set_current_page.set(1);
                                }
                            >
                                <option value="5" selected>"5"</option>
                                <option value="10">"10"</option>
                                <option value="20">"20"</option>
                                <option value="50">"50"</option>
                            </select>
                        </div>
                    </div>
                </div>

                // Content table
                <Suspense fallback=move || view! {
                    <div class="fx-card-prime" style="padding: 24px; display: flex; align-items: center; justify-content: center; min-height: 300px;">
                        <ProgressSpinner size=SpinnerSize::Default />
                    </div>
                }>
                    {move || {
                        match content_resource.get() {
                            None => view! {
                                <div style="display: flex; align-items: center; justify-content: center; padding: 48px;">
                                    <ProgressSpinner size=SpinnerSize::Default />
                                </div>
                            }.into_any(),
                            Some(Err(e)) => view! {
                                <div class="fx-alert-prime fx-alert-prime-error">
                                    <strong>"Error: "</strong>{e.to_string()}
                                </div>
                            }.into_any(),
                            Some(Ok(response)) => {
                                let items = response.items.clone();
                                let total = response.total;
                                let page = response.page;
                                let pp = response.per_page;
                                let total_pages = ((total as f64) / (pp as f64)).ceil() as i32;

                                if items.is_empty() {
                                    view! {
                                        <div class="fx-card-prime" style="padding: 48px; text-align: center;">
                                            <div style="opacity: 0.4; margin-bottom: 16px;">
                                                <Icon name=IconName::Document size=48 class="mx-auto".to_string() />
                                            </div>
                                            <p style="opacity: 0.65;">
                                                "No content found"
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    let edit_handler = on_edit_click.clone();
                                    let view_handler = on_view_click.clone();

                                    view! {
                                        <div class="space-y-4">
                                            <ContentTable
                                                items=items
                                                on_edit=edit_handler
                                                on_view=view_handler
                                            />

                                            // Pagination
                                            <div style="background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 12px 16px; display: flex; align-items: center; justify-content: space-between;">
                                                <span class="fx-pagination-prime-total-text">
                                                    {format!("Showing {} items (Page {} of {})", response.items.len(), page, total_pages.max(1))}
                                                </span>
                                                <nav class="fx-pagination-prime">
                                                    <button
                                                        class="fx-pagination-prime-prev fx-pagination-prime-item"
                                                        class:fx-pagination-prime-item-disabled=move || current_page.get() <= 1
                                                        disabled=move || current_page.get() <= 1
                                                        on:click=move |_| set_current_page.update(|p| *p = (*p - 1).max(1))
                                                    >
                                                        "Previous"
                                                    </button>
                                                    <span class="fx-pagination-prime-simple-pager">
                                                        {move || format!("{} / {}", current_page.get(), total_pages.max(1))}
                                                    </span>
                                                    <button
                                                        class="fx-pagination-prime-next fx-pagination-prime-item"
                                                        class:fx-pagination-prime-item-disabled={
                                                            let max = total_pages;
                                                            move || current_page.get() >= max
                                                        }
                                                        disabled={
                                                            let max = total_pages;
                                                            move || current_page.get() >= max
                                                        }
                                                        on:click=move |_| set_current_page.update(|p| *p += 1)
                                                    >
                                                        "Next"
                                                    </button>
                                                </nav>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

/// Content table component
#[component]
fn ContentTable(
    items: Vec<ContentItemSummary>,
    on_edit: impl Fn(String) + Clone + 'static,
    on_view: impl Fn(String) + Clone + 'static,
) -> impl IntoView {
    view! {
        <div class="fx-table-prime-wrapper" style="background: #0f172a; border-radius: 12px; overflow: hidden;">
            <table class="fx-table-prime fx-table-prime-hoverable" style="background: transparent;">
                <thead class="fx-table-prime-thead" style="background: #0f172a;">
                    <tr>
                        <th style="min-width: 200px; padding: 12px 16px; color: #94a3b8;">"Title"</th>
                        <th style="width: 80px; text-align: center; padding: 12px 8px; color: #94a3b8;">"Type"</th>
                        <th style="width: 100px; text-align: center; padding: 12px 8px; color: #94a3b8;">"Status"</th>
                        <th style="width: 80px; text-align: center; padding: 12px 8px; color: #94a3b8;">"Version"</th>
                        <th style="width: 100px; text-align: center; padding: 12px 8px; color: #94a3b8;">"Updated"</th>
                        <th style="width: 200px; text-align: center; padding: 12px 16px; color: #94a3b8;">"Actions"</th>
                    </tr>
                </thead>
                <tbody class="fx-table-prime-tbody" style="background: #0f172a;">
                    {items.into_iter().enumerate().map(|(index, item)| {
                        let id = item.id.clone();
                        let id_view = item.id.clone();
                        let edit_click = on_edit.clone();
                        let view_click = on_view.clone();
                        view! {
                            <ContentTableRow
                                item=item
                                index=index
                                on_edit=move || edit_click(id.clone())
                                on_view=move || view_click(id_view.clone())
                            />
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}

/// Content table row component
#[component]
fn ContentTableRow(
    item: ContentItemSummary,
    index: usize,
    on_view: impl Fn() + 'static,
    on_edit: impl Fn() + 'static,
) -> impl IntoView {
    // Zebra striping - alternate row backgrounds
    let row_bg = if index % 2 == 0 { "#0f172a" } else { "#1e293b" };
    let status_tag = match item.status {
        ContentItemStatus::Published => "fx-tag-prime fx-tag-prime-green",
        ContentItemStatus::Approved => "fx-tag-prime fx-tag-prime-blue",
        ContentItemStatus::PendingReview => "fx-tag-prime fx-tag-prime-orange",
        ContentItemStatus::Draft => "fx-tag-prime",
        ContentItemStatus::Scheduled => "fx-tag-prime fx-tag-prime-cyan",
        ContentItemStatus::Archived => "fx-tag-prime fx-tag-prime-red",
    };

    // Dynamic type tag coloring based on schema_id
    let type_tag = get_type_tag_class(&item.schema_id);
    let slug_display = item.slug.clone().unwrap_or_else(|| "-".to_string());
    let item_title = item.title.clone();

    // Store item data for actions
    let content_id = item.id.clone();
    let schema_id = item.schema_id.clone();
    let content_id_for_publish = item.id.clone();
    let content_id_for_delete = item.id.clone();
    let is_published = item.status == ContentItemStatus::Published;

    // Delete confirmation modal state
    let (show_delete_modal, set_show_delete_modal) = signal(false);
    let (is_deleting, set_is_deleting) = signal(false);

    // Preview action
    let preview_action = Action::new(move |_: &()| {
        let cid = content_id.clone();
        let sid = schema_id.clone();
        async move {
            match create_schema_content_preview(cid, sid).await {
                Ok(url) => {
                    // Open in new tab
                    if let Some(window) = web_sys::window() {
                        let _ = window.open_with_url_and_target(&url, "_blank");
                    }
                }
                Err(e) => {
                    tracing::error!("Preview failed: {}", e);
                }
            }
        }
    });

    // Publish action
    let (is_publishing, set_is_publishing) = signal(false);
    let publish_action = Action::new(move |_: &()| {
        let cid = content_id_for_publish.clone();
        set_is_publishing.set(true);
        async move {
            match publish_content_item(cid).await {
                Ok(_) => {
                    // Refresh will happen via resource
                }
                Err(e) => {
                    tracing::error!("Publish failed: {}", e);
                }
            }
            set_is_publishing.set(false);
        }
    });

    // Delete action
    let delete_action = Action::new(move |_: &()| {
        let cid = content_id_for_delete.clone();
        set_is_deleting.set(true);
        async move {
            match delete_content_item(cid).await {
                Ok(_) => {
                    // Refresh will happen via resource
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                }
                Err(e) => {
                    tracing::error!("Delete failed: {}", e);
                }
            }
            set_is_deleting.set(false);
            set_show_delete_modal.set(false);
        }
    });

    view! {
        <tr style=format!("background: {}; border-bottom: 1px solid #334155; transition: background 0.15s;", row_bg) class="content-table-row">
            // Title & Slug
            <td style="padding: 14px 16px;">
                <div style="font-weight: 500; color: #e2e8f0;">
                    {item.title.clone()}
                </div>
                <div style="font-size: 12px; color: #64748b; margin-top: 2px; font-family: monospace;">
                    {slug_display}
                </div>
            </td>

            // Type (schema_id)
            <td style="text-align: center; padding: 14px 8px;">
                <span class={type_tag}>
                    {item.schema_id.clone()}
                </span>
            </td>

            // Status
            <td style="text-align: center; padding: 14px 8px;">
                <span class={status_tag}>
                    {item.status.display_name()}
                </span>
            </td>

            // Version
            <td style="text-align: center; padding: 14px 8px; color: #cbd5e1;">
                <span>{"v"}{item.version}</span>
                {item.published_version.map(|pv| view! {
                    <span style="font-size: 12px; color: #64748b; margin-left: 4px;">
                        {"(pub: v"}{pv}{")" }
                    </span>
                })}
            </td>

            // Updated
            <td style="text-align: center; padding: 14px 8px;">
                <span style="font-size: 13px; color: #94a3b8;">
                    {format_date(&item.updated_at)}
                </span>
            </td>

            // Actions - grouped: [View Edit] [Preview Publish] [Delete]
            <td style="text-align: center; padding: 14px 16px;">
                <div style="display: flex; gap: 12px; justify-content: center; align-items: center;">
                    // Group 1: View + Edit
                    <div style="display: flex; gap: 1px;">
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #475569; color: white; border: none; border-radius: 4px; cursor: pointer;"
                            on:click=move |_| on_view()
                            title="View"
                        >
                            <Icon name=IconName::Eye size=14 />
                        </button>
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer;"
                            on:click=move |_| on_edit()
                            title="Edit"
                        >
                            <Icon name=IconName::Edit size=14 />
                        </button>
                    </div>

                    // Group 2: Preview + Publish
                    <div style="display: flex; gap: 1px;">
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #0ea5e9; color: white; border: none; border-radius: 4px; cursor: pointer;"
                            on:click=move |_| { let _ = preview_action.dispatch(()); }
                            title="Preview"
                        >
                            <Icon name=IconName::Eye size=14 />
                        </button>
                        <button
                            style={move || format!(
                                "display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: {}; color: white; border: none; border-radius: 4px; cursor: {};",
                                if is_published { "#475569" } else { "#22c55e" },
                                if is_published { "not-allowed" } else { "pointer" }
                            )}
                            on:click=move |_| { if !is_published { let _ = publish_action.dispatch(()); } }
                            title={if is_published { "Already Published" } else { "Publish" }}
                            disabled=move || is_published || is_publishing.get()
                        >
                            <Icon name=IconName::Rocket size=14 />
                        </button>
                    </div>

                    // Group 3: Delete
                    <div style="display: flex;">
                        <button
                            style="display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: #ef4444; color: white; border: none; border-radius: 4px; cursor: pointer;"
                            on:click=move |_| set_show_delete_modal.set(true)
                            title="Delete"
                        >
                            <Icon name=IconName::Trash size=14 />
                        </button>
                    </div>
                </div>

                // Delete confirmation modal
                <Show when=move || show_delete_modal.get()>
                    <div
                        style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.6); display: flex; align-items: center; justify-content: center; z-index: 9999;"
                        on:click=move |_| set_show_delete_modal.set(false)
                    >
                        <div
                            style="background: #1e293b; border: 1px solid #334155; border-radius: 12px; padding: 24px; max-width: 400px; width: 90%;"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 16px;">
                                <div style="width: 40px; height: 40px; background: rgba(239, 68, 68, 0.2); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                                    <Icon name=IconName::Trash size=20 class="text-red-500".to_string() />
                                </div>
                                <h3 style="color: #f1f5f9; font-size: 18px; font-weight: 600; margin: 0;">
                                    "Delete Content"
                                </h3>
                            </div>
                            <p style="color: #94a3b8; font-size: 14px; line-height: 1.5; margin-bottom: 24px;">
                                "Are you sure you want to delete \""
                                <span style="color: #e2e8f0; font-weight: 500;">{item_title.clone()}</span>
                                "\"? This action cannot be undone."
                            </p>
                            <div style="display: flex; gap: 12px; justify-content: flex-end;">
                                <button
                                    class="fx-btn-prime fx-btn-prime-ghost"
                                    style="padding: 8px 16px; color: #94a3b8;"
                                    on:click=move |_| set_show_delete_modal.set(false)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    class="fx-btn-prime"
                                    style="padding: 8px 16px; background: #ef4444; color: white; border: none; border-radius: 8px;"
                                    on:click=move |_| { let _ = delete_action.dispatch(()); }
                                    disabled=move || is_deleting.get()
                                >
                                    {move || if is_deleting.get() { "Deleting..." } else { "Delete" }}
                                </button>
                            </div>
                        </div>
                    </div>
                </Show>
            </td>
        </tr>
    }
}

/// Format date for display
fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Get tag class for content type (supports both known and dynamic types)
fn get_type_tag_class(content_type: &str) -> &'static str {
    match content_type {
        "task" => "fx-tag-prime fx-tag-prime-purple",
        "guide" => "fx-tag-prime fx-tag-prime-cyan",
        "faq" => "fx-tag-prime fx-tag-prime-green",
        "error" => "fx-tag-prime fx-tag-prime-red",
        "front" => "fx-tag-prime fx-tag-prime-blue",
        // For dynamic/custom types, use a neutral style
        _ => "fx-tag-prime fx-tag-prime-orange",
    }
}
