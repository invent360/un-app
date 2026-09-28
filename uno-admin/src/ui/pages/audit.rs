//! Audit Log Viewer page
//!
//! Displays a searchable, filterable list of all CMS actions for compliance and debugging.

use leptos::prelude::*;
use crate::components::layout::Header;
use crate::components::common::icon::{Icon, IconName};
#[cfg(feature = "ssr")]
use crate::api::{ContentClient, AuditLogFilter};
use serde::{Deserialize, Serialize};

/// Panel styling constants (matching list.rs dark navy theme)
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 24px;";
const INPUT_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; color: #e2e8f0; font-size: 14px;";
const SELECT_STYLE: &str = "background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 10px 14px; color: #e2e8f0; font-size: 14px; min-width: 150px;";

/// Audit log entry for display (mapped from API response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntryDisplay {
    pub id: i32,
    pub entity_type: String,
    pub entity_id: Option<i32>,
    pub action: String,
    pub actor: String,
    pub details: Option<String>,
    pub created_at: String,
}

/// Response from the audit log API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogResponseDisplay {
    pub entries: Vec<AuditLogEntryDisplay>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// Action types for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionFilter {
    All,
    Create,
    Update,
    Publish,
    Revert,
    Delete,
    Review,
}

impl ActionFilter {
    fn as_str(&self) -> Option<&'static str> {
        match self {
            ActionFilter::All => None,
            ActionFilter::Create => Some("create"),
            ActionFilter::Update => Some("update"),
            ActionFilter::Publish => Some("publish"),
            ActionFilter::Revert => Some("revert"),
            ActionFilter::Delete => Some("delete"),
            ActionFilter::Review => Some("review"),
        }
    }

    fn display(&self) -> &'static str {
        match self {
            ActionFilter::All => "All Actions",
            ActionFilter::Create => "Create",
            ActionFilter::Update => "Update",
            ActionFilter::Publish => "Publish",
            ActionFilter::Revert => "Revert",
            ActionFilter::Delete => "Delete",
            ActionFilter::Review => "Review",
        }
    }
}

/// Server function to fetch audit logs
#[server(GetAuditLogs, "/api")]
pub async fn get_audit_logs(
    action: Option<String>,
    actor: Option<String>,
    entity_id: Option<i32>,
    from_date: Option<String>,
    to_date: Option<String>,
    page: i32,
    per_page: i32,
) -> Result<AuditLogResponseDisplay, ServerFnError> {
    let client = ContentClient::from_env()
        .map_err(|e| ServerFnError::new(format!("Failed to create client: {}", e)))?;

    // Build the filter
    let filter = AuditLogFilter {
        entity_type: Some("content".to_string()), // Filter to content-related actions
        entity_id,
        action,
        actor_id: actor,
        from_date,
        to_date,
        page,
        per_page,
    };

    let response = client.list_audit_logs(filter).await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch audit logs: {}", e)))?;

    // Map API response to display types
    let entries = response.entries.into_iter()
        .map(|e| AuditLogEntryDisplay {
            id: e.id,
            entity_type: e.entity_type,
            entity_id: e.entity_id,
            action: e.action,
            actor: e.actor_id,
            details: e.metadata.map(|m| m.to_string()),
            created_at: e.created_at,
        })
        .collect();

    Ok(AuditLogResponseDisplay {
        entries,
        total: response.total,
        page: response.page,
        per_page: response.per_page,
    })
}

/// Audit Log Viewer page
#[component]
pub fn AuditLogPage() -> impl IntoView {
    // Filter state
    let (action_filter, set_action_filter) = signal(ActionFilter::All);
    let (actor_search, set_actor_search) = signal(String::new());
    let (content_id_filter, set_content_id_filter) = signal(Option::<i32>::None);
    let (from_date, set_from_date) = signal(String::new());
    let (to_date, set_to_date) = signal(String::new());
    let (page, set_page) = signal(1i32);
    let per_page = 20i32;

    // Fetch audit logs
    let audit_logs = Resource::new(
        move || (
            action_filter.get(),
            actor_search.get(),
            content_id_filter.get(),
            from_date.get(),
            to_date.get(),
            page.get(),
        ),
        move |(action, actor, content_id, from, to, pg)| async move {
            get_audit_logs(
                action.as_str().map(String::from),
                if actor.is_empty() { None } else { Some(actor) },
                content_id,
                if from.is_empty() { None } else { Some(from) },
                if to.is_empty() { None } else { Some(to) },
                pg,
                per_page,
            ).await
        }
    );

    view! {
        <div>
            <Header title="Audit Log".to_string() show_search=false />

            <div style="padding: 24px; display: flex; flex-direction: column; gap: 24px;">
                // Filters panel
                <div style=PANEL_STYLE>
                    <div style="display: flex; flex-wrap: wrap; gap: 16px; align-items: flex-end;">
                        // Action filter
                        <div>
                            <label style="display: block; font-size: 12px; color: #94a3b8; margin-bottom: 6px;">
                                "Action"
                            </label>
                            <select
                                style=SELECT_STYLE
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_action_filter.set(match value.as_str() {
                                        "create" => ActionFilter::Create,
                                        "update" => ActionFilter::Update,
                                        "publish" => ActionFilter::Publish,
                                        "revert" => ActionFilter::Revert,
                                        "delete" => ActionFilter::Delete,
                                        "review" => ActionFilter::Review,
                                        _ => ActionFilter::All,
                                    });
                                    set_page.set(1);
                                }
                            >
                                <option value="all">"All Actions"</option>
                                <option value="create">"Create"</option>
                                <option value="update">"Update"</option>
                                <option value="publish">"Publish"</option>
                                <option value="revert">"Revert"</option>
                                <option value="delete">"Delete"</option>
                                <option value="review">"Review"</option>
                            </select>
                        </div>

                        // Actor search
                        <div>
                            <label style="display: block; font-size: 12px; color: #94a3b8; margin-bottom: 6px;">
                                "Actor"
                            </label>
                            <input
                                type="text"
                                style=INPUT_STYLE
                                placeholder="Search by user..."
                                prop:value=move || actor_search.get()
                                on:input=move |ev| {
                                    set_actor_search.set(event_target_value(&ev));
                                    set_page.set(1);
                                }
                            />
                        </div>

                        // Content ID filter
                        <div>
                            <label style="display: block; font-size: 12px; color: #94a3b8; margin-bottom: 6px;">
                                "Content ID"
                            </label>
                            <input
                                type="number"
                                style=INPUT_STYLE
                                placeholder="Filter by ID..."
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_content_id_filter.set(value.parse().ok());
                                    set_page.set(1);
                                }
                            />
                        </div>

                        // Date range
                        <div>
                            <label style="display: block; font-size: 12px; color: #94a3b8; margin-bottom: 6px;">
                                "From Date"
                            </label>
                            <input
                                type="date"
                                style=INPUT_STYLE
                                prop:value=move || from_date.get()
                                on:input=move |ev| {
                                    set_from_date.set(event_target_value(&ev));
                                    set_page.set(1);
                                }
                            />
                        </div>

                        <div>
                            <label style="display: block; font-size: 12px; color: #94a3b8; margin-bottom: 6px;">
                                "To Date"
                            </label>
                            <input
                                type="date"
                                style=INPUT_STYLE
                                prop:value=move || to_date.get()
                                on:input=move |ev| {
                                    set_to_date.set(event_target_value(&ev));
                                    set_page.set(1);
                                }
                            />
                        </div>

                        // Clear filters button
                        <button
                            style="padding: 10px 20px; background: #1e293b; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer;"
                            on:click=move |_| {
                                set_action_filter.set(ActionFilter::All);
                                set_actor_search.set(String::new());
                                set_content_id_filter.set(None);
                                set_from_date.set(String::new());
                                set_to_date.set(String::new());
                                set_page.set(1);
                            }
                        >
                            "Clear Filters"
                        </button>
                    </div>
                </div>

                // Audit log table
                <div style=PANEL_STYLE>
                    <Suspense fallback=move || view! {
                        <div style="text-align: center; padding: 40px; color: #94a3b8;">
                            "Loading audit logs..."
                        </div>
                    }>
                        {move || {
                            audit_logs.get().map(|result| {
                                match result {
                                    Ok(response) => {
                                        if response.entries.is_empty() {
                                            view! {
                                                <div style="text-align: center; padding: 40px; color: #64748b;">
                                                    <Icon name=IconName::Document size=48 />
                                                    <p style="margin-top: 16px;">"No audit log entries found"</p>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div>
                                                    // Summary
                                                    <div style="margin-bottom: 16px; color: #94a3b8; font-size: 14px;">
                                                        "Showing "{response.entries.len()}" of "{response.total}" entries"
                                                    </div>

                                                    // Table
                                                    <div style="overflow-x: auto;">
                                                        <table style="width: 100%; border-collapse: collapse;">
                                                            <thead>
                                                                <tr style="border-bottom: 1px solid #334155;">
                                                                    <th style="text-align: left; padding: 12px 16px; color: #94a3b8; font-weight: 500;">"Timestamp"</th>
                                                                    <th style="text-align: left; padding: 12px 16px; color: #94a3b8; font-weight: 500;">"Action"</th>
                                                                    <th style="text-align: left; padding: 12px 16px; color: #94a3b8; font-weight: 500;">"Entity"</th>
                                                                    <th style="text-align: left; padding: 12px 16px; color: #94a3b8; font-weight: 500;">"Actor"</th>
                                                                    <th style="text-align: left; padding: 12px 16px; color: #94a3b8; font-weight: 500;">"Details"</th>
                                                                </tr>
                                                            </thead>
                                                            <tbody>
                                                                {response.entries.into_iter().map(|entry| {
                                                                    let action_badge = get_action_badge(&entry.action);
                                                                    view! {
                                                                        <tr style="border-bottom: 1px solid #1e293b;">
                                                                            <td style="padding: 12px 16px; color: #cbd5e1; font-size: 13px;">
                                                                                {format_timestamp(&entry.created_at)}
                                                                            </td>
                                                                            <td style="padding: 12px 16px;">
                                                                                <span style={action_badge}>
                                                                                    {entry.action.to_uppercase()}
                                                                                </span>
                                                                            </td>
                                                                            <td style="padding: 12px 16px; color: #e2e8f0;">
                                                                                {entry.entity_id.map(|id| {
                                                                                    let entity_type = entry.entity_type.clone();
                                                                                    view! {
                                                                                        <a
                                                                                            href={format!("/content/{}", id)}
                                                                                            style="color: #06b6d4; text-decoration: none;"
                                                                                        >
                                                                                            {format!("{} #{}", entity_type, id)}
                                                                                        </a>
                                                                                    }
                                                                                })}
                                                                                {entry.entity_id.is_none().then(|| view! {
                                                                                    <span style="color: #64748b;">"-"</span>
                                                                                })}
                                                                            </td>
                                                                            <td style="padding: 12px 16px; color: #94a3b8;">
                                                                                {entry.actor}
                                                                            </td>
                                                                            <td style="padding: 12px 16px; color: #64748b; font-size: 13px;">
                                                                                {entry.details.unwrap_or_else(|| "-".to_string())}
                                                                            </td>
                                                                        </tr>
                                                                    }
                                                                }).collect_view()}
                                                            </tbody>
                                                        </table>
                                                    </div>

                                                    // Pagination (simple prev/next)
                                                    <div style="display: flex; justify-content: center; gap: 16px; margin-top: 24px;">
                                                        <button
                                                            style="padding: 8px 16px; background: #1e293b; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer;"
                                                            disabled=move || page.get() <= 1
                                                            on:click=move |_| set_page.update(|p| *p = (*p - 1).max(1))
                                                        >
                                                            "Previous"
                                                        </button>
                                                        <span style="padding: 8px 16px; color: #94a3b8;">
                                                            "Page "{move || page.get()}
                                                        </span>
                                                        <button
                                                            style="padding: 8px 16px; background: #1e293b; color: #94a3b8; border: 1px solid #334155; border-radius: 8px; cursor: pointer;"
                                                            on:click=move |_| set_page.update(|p| *p += 1)
                                                        >
                                                            "Next"
                                                        </button>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                    }
                                    Err(e) => view! {
                                        <div style="text-align: center; padding: 40px; color: #ef4444;">
                                            "Error loading audit logs: "{e.to_string()}
                                        </div>
                                    }.into_any(),
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

/// Get badge style for action type
fn get_action_badge(action: &str) -> &'static str {
    match action {
        "create" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #166534; color: #86efac;",
        "update" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #1e40af; color: #93c5fd;",
        "publish" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #059669; color: #6ee7b7;",
        "revert" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #b45309; color: #fcd34d;",
        "delete" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #be123c; color: #fda4af;",
        "review" => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #6d28d9; color: #c4b5fd;",
        _ => "display: inline-block; padding: 4px 8px; font-size: 11px; font-weight: 600; border-radius: 4px; background: #334155; color: #94a3b8;",
    }
}

/// Format ISO timestamp to readable format
fn format_timestamp(iso: &str) -> String {
    // Simple formatting - in production you'd use chrono
    iso.replace("T", " ").replace("Z", "").chars().take(16).collect()
}
