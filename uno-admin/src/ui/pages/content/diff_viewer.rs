//! Version diff viewer component
//!
//! Displays a side-by-side comparison of changes between two content versions.

use leptos::prelude::*;
use crate::api::content_client::{VersionDiff, ContentChange};

/// Panel styling constants (matching editor.rs dark navy theme)
const PANEL_STYLE: &str = "background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 24px;";

/// Diff viewer component for comparing two content versions
#[component]
pub fn DiffViewer(
    /// The version diff data to display
    diff: VersionDiff,
) -> impl IntoView {
    let changes = diff.changes.clone();
    let has_changes = !changes.is_empty();

    view! {
        <div style=PANEL_STYLE>
            // Header with version comparison info
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px; padding-bottom: 16px; border-bottom: 1px solid #334155;">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <span style="font-size: 16px; font-weight: 600; color: #e2e8f0;">
                        "Comparing "
                    </span>
                    <span style="padding: 4px 12px; background: #dc2626; color: white; border-radius: 6px; font-weight: 500; font-size: 14px;">
                        {"v"}{diff.version_a}
                    </span>
                    <span style="color: #64748b;">"→"</span>
                    <span style="padding: 4px 12px; background: #22c55e; color: white; border-radius: 6px; font-weight: 500; font-size: 14px;">
                        {"v"}{diff.version_b}
                    </span>
                </div>
                <span style="font-size: 14px; color: #94a3b8;">
                    {diff.total_changes}{" change(s)"}
                </span>
            </div>

            // Changes list
            {if has_changes {
                view! {
                    <div style="display: flex; flex-direction: column; gap: 12px;">
                        {changes.into_iter().map(|change| {
                            view! { <DiffRow change=change /> }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! {
                    <div style="text-align: center; padding: 40px; color: #64748b;">
                        <p style="font-size: 16px;">"No changes detected between these versions"</p>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

/// Individual diff row component
#[component]
fn DiffRow(
    /// The content change to display
    change: ContentChange,
) -> impl IntoView {
    let (bg_color, border_color, icon, label) = match change.change_type.as_str() {
        "added" => ("rgba(34, 197, 94, 0.1)", "#22c55e", "+", "Added"),
        "removed" => ("rgba(220, 38, 38, 0.1)", "#dc2626", "-", "Removed"),
        _ => ("rgba(59, 130, 246, 0.1)", "#3b82f6", "~", "Modified"),
    };

    let field_display = if change.locale.is_empty() || change.locale == "en" {
        change.field.clone()
    } else {
        format!("{} ({})", change.field, change.locale)
    };

    let old_val = change.old_value.clone().unwrap_or_else(|| "—".to_string());
    let new_val = change.new_value.clone().unwrap_or_else(|| "—".to_string());

    // Truncate long values for display
    let truncate = |s: String, max_len: usize| -> String {
        if s.len() > max_len {
            format!("{}...", &s[..max_len])
        } else {
            s
        }
    };

    let old_display = truncate(old_val.clone(), 200);
    let new_display = truncate(new_val.clone(), 200);

    view! {
        <div style=format!("background: {}; border: 1px solid {}; border-radius: 8px; padding: 16px;", bg_color, border_color)>
            // Header row with field name and change type
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
                <div style="display: flex; align-items: center; gap: 8px;">
                    <span style=format!("width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; background: {}; color: white; border-radius: 4px; font-weight: 600; font-size: 14px;", border_color)>
                        {icon}
                    </span>
                    <span style="font-weight: 500; color: #e2e8f0; font-size: 14px;">
                        {field_display}
                    </span>
                </div>
                <span style=format!("font-size: 12px; color: {}; font-weight: 500;", border_color)>
                    {label}
                </span>
            </div>

            // Side-by-side comparison
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                // Old value
                <div>
                    <div style="font-size: 11px; font-weight: 500; color: #94a3b8; margin-bottom: 6px; text-transform: uppercase;">
                        "Before"
                    </div>
                    <div style="background: rgba(220, 38, 38, 0.15); border: 1px solid rgba(220, 38, 38, 0.3); border-radius: 6px; padding: 12px; font-family: monospace; font-size: 13px; color: #f87171; word-break: break-word; white-space: pre-wrap; max-height: 200px; overflow-y: auto;">
                        {old_display}
                    </div>
                </div>

                // New value
                <div>
                    <div style="font-size: 11px; font-weight: 500; color: #94a3b8; margin-bottom: 6px; text-transform: uppercase;">
                        "After"
                    </div>
                    <div style="background: rgba(34, 197, 94, 0.15); border: 1px solid rgba(34, 197, 94, 0.3); border-radius: 6px; padding: 12px; font-family: monospace; font-size: 13px; color: #4ade80; word-break: break-word; white-space: pre-wrap; max-height: 200px; overflow-y: auto;">
                        {new_display}
                    </div>
                </div>
            </div>
        </div>
    }
}
