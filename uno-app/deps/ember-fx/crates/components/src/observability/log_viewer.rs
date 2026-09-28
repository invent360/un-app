//! LogViewer Leptos component.

use leptos::prelude::*;
use super::types::{LogEntry, LogFilter, LogLevel};
use crate::try_use_theme;

/// LogViewer component.
///
/// Streaming log viewer with filtering and virtual scrolling.
///
/// # Props
///
/// - `logs` - Log entries (reactive)
/// - `filter` - Filter configuration
/// - `streaming` - Enable streaming mode
/// - `max_entries` - Maximum visible entries
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{LogViewer, LogEntry, LogLevel};
///
/// let logs = vec![
///     LogEntry::new(1716000000000, LogLevel::Info, "Connected", "p2p"),
///     LogEntry::new(1716000001000, LogLevel::Error, "Failed", "exchange"),
/// ];
///
/// view! {
///     <LogViewer logs=Signal::derive(move || logs.clone()) />
/// }
/// ```
#[component]
pub fn LogViewer(
    /// Log entries.
    logs: Signal<Vec<LogEntry>>,
    /// Filter configuration.
    #[prop(optional)]
    filter: Option<RwSignal<LogFilter>>,
    /// Enable streaming mode.
    #[prop(optional)]
    streaming: Option<bool>,
    /// Maximum visible entries (for virtualization).
    #[prop(optional)]
    max_entries: Option<usize>,
    /// Click handler for log entry.
    #[prop(optional, into)]
    on_entry_click: Option<Callback<LogEntry>>,
    /// Click handler for trace ID.
    #[prop(optional, into)]
    on_trace_click: Option<Callback<String>>,
    /// Show timestamps.
    #[prop(optional)]
    show_timestamps: Option<bool>,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve props
    let _streaming = streaming.unwrap_or(false);
    let max_entries = max_entries.unwrap_or(1000);
    let show_timestamps = show_timestamps.unwrap_or(true);
    let _on_entry_click = on_entry_click;
    let _on_trace_click = on_trace_click;

    // Internal filter state if not provided
    let internal_filter = RwSignal::new(LogFilter::default());
    let filter = filter.unwrap_or(internal_filter);

    // Expanded entry state
    let expanded_ids = RwSignal::new(std::collections::HashSet::<String>::new());

    // Build CSS classes
    let log_prefix = format!("fx-log-viewer-{}", design_system);

    let combined_class = {
        let mut parts = vec![log_prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", log_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Pre-compute all class names to avoid ownership issues in closures
    let filter_bar_class = format!("{}-filter-bar", log_prefix);
    let search_class = format!("{}-search", log_prefix);
    let level_filter_class = format!("{}-level-filter", log_prefix);
    let entries_class = format!("{}-entries", log_prefix);
    let entry_class = log_prefix.clone();
    let entry_row_class = format!("{}-entry-row", log_prefix);
    let timestamp_class = format!("{}-timestamp", log_prefix);
    let level_badge_class = format!("{}-level-badge", log_prefix);
    let source_class = format!("{}-source", log_prefix);
    let message_class = format!("{}-message", log_prefix);
    let expand_icon_class = format!("{}-expand-icon", log_prefix);
    let entry_details_class = format!("{}-entry-details", log_prefix);
    let detail_row_class = format!("{}-detail-row", log_prefix);
    let detail_label_class = format!("{}-detail-label", log_prefix);
    let detail_value_class = format!("{}-detail-value", log_prefix);
    let trace_link_class = format!("{}-trace-link", log_prefix);
    let footer_class = format!("{}-footer", log_prefix);
    let count_class = format!("{}-count", log_prefix);
    let loading_overlay_class = format!("{}-loading-overlay", log_prefix);
    let loading_spinner_class = format!("{}-loading-spinner", log_prefix);

    // Filter and limit logs
    let filtered_logs = move || {
        let current_filter = filter.get();
        let all_logs = logs.get();

        all_logs
            .into_iter()
            .filter(|log| current_filter.matches(log))
            .take(max_entries)
            .collect::<Vec<_>>()
    };

    // Format timestamp
    let format_time = |ts: i64| {
        let secs = (ts / 1000) % 86400;
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;
        let millis = ts % 1000;
        format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
    };

    view! {
        <div class=combined_class>
            // Filter bar
            <div class=filter_bar_class>
                // Search input
                <input
                    type="text"
                    placeholder="Search logs..."
                    class=search_class
                    on:input=move |ev| {
                        filter.update(|f| f.search = event_target_value(&ev));
                    }
                />

                // Level filter
                <select
                    class=level_filter_class
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        filter.update(|f| {
                            f.levels = if value == "all" {
                                vec![]
                            } else {
                                match value.as_str() {
                                    "error" => vec![LogLevel::Error],
                                    "warn" => vec![LogLevel::Warn, LogLevel::Error],
                                    "info" => vec![LogLevel::Info, LogLevel::Warn, LogLevel::Error],
                                    "debug" => vec![LogLevel::Debug, LogLevel::Info, LogLevel::Warn, LogLevel::Error],
                                    _ => vec![],
                                }
                            };
                        });
                    }
                >
                    <option value="all">"All Levels"</option>
                    <option value="error">"Error"</option>
                    <option value="warn">"Warn+"</option>
                    <option value="info">"Info+"</option>
                    <option value="debug">"Debug+"</option>
                </select>
            </div>

            // Log entries
            <div class=entries_class>
                {move || {
                    let entry_class = entry_class.clone();
                    let entry_row_class = entry_row_class.clone();
                    let timestamp_class = timestamp_class.clone();
                    let level_badge_class = level_badge_class.clone();
                    let source_class = source_class.clone();
                    let message_class = message_class.clone();
                    let expand_icon_class = expand_icon_class.clone();
                    let entry_details_class = entry_details_class.clone();
                    let detail_row_class = detail_row_class.clone();
                    let detail_label_class = detail_label_class.clone();
                    let detail_value_class = detail_value_class.clone();
                    let trace_link_class = trace_link_class.clone();

                    filtered_logs().into_iter().map(move |entry| {
                        let entry_id = entry.id.clone();
                        let entry_id_clone = entry_id.clone();
                        let entry_id_for_icon = entry_id.clone();
                        let entry_id_for_details = entry_id.clone();
                        let level_class = format!("{}-level-{}", entry_class, entry.level.as_suffix());
                        let has_metadata = !entry.metadata.is_empty() || entry.trace_id.is_some();
                        let full_entry_class = format!("{}-entry {}", entry_class, level_class);

                        let entry_row_class = entry_row_class.clone();
                        let timestamp_class = timestamp_class.clone();
                        let level_badge_class = level_badge_class.clone();
                        let source_class = source_class.clone();
                        let message_class = message_class.clone();
                        let expand_icon_class = expand_icon_class.clone();
                        let entry_details_class = entry_details_class.clone();
                        let detail_row_class = detail_row_class.clone();
                        let detail_label_class = detail_label_class.clone();
                        let detail_value_class = detail_value_class.clone();
                        let trace_link_class = trace_link_class.clone();

                        view! {
                            <div
                                class=full_entry_class
                                on:click=move |_| {
                                    if has_metadata {
                                        expanded_ids.update(|ids| {
                                            if ids.contains(&entry_id_clone) {
                                                ids.remove(&entry_id_clone);
                                            } else {
                                                ids.insert(entry_id_clone.clone());
                                            }
                                        });
                                    }
                                }
                            >
                                // Main row
                                <div class=entry_row_class>
                                    // Timestamp
                                    {show_timestamps.then(|| view! {
                                        <span class=timestamp_class.clone()>
                                            {format_time(entry.timestamp)}
                                        </span>
                                    })}

                                    // Level badge
                                    <span
                                        class=level_badge_class.clone()
                                        style=format!("color: {}", entry.level.as_color())
                                    >
                                        {entry.level.as_label()}
                                    </span>

                                    // Source
                                    <span class=source_class.clone()>
                                        {entry.source.clone()}
                                    </span>

                                    // Message
                                    <span class=message_class.clone()>
                                        {entry.message.clone()}
                                    </span>

                                    // Expand indicator
                                    {has_metadata.then(|| view! {
                                        <span class=expand_icon_class.clone()>
                                            {move || if expanded_ids.get().contains(&entry_id_for_icon) { "▼" } else { "▶" }}
                                        </span>
                                    })}
                                </div>

                                // Expanded details
                                {move || {
                                    let entry_details_class = entry_details_class.clone();
                                    let detail_row_class = detail_row_class.clone();
                                    let detail_label_class = detail_label_class.clone();
                                    let detail_value_class = detail_value_class.clone();
                                    let trace_link_class = trace_link_class.clone();

                                    expanded_ids.get().contains(&entry_id_for_details).then(|| {
                                        let entry_clone = entry.clone();
                                        let detail_row_class2 = detail_row_class.clone();
                                        let detail_label_class2 = detail_label_class.clone();
                                        let detail_value_class2 = detail_value_class.clone();

                                        view! {
                                            <div class=entry_details_class>
                                                // Trace ID
                                                {entry_clone.trace_id.clone().map(|tid| {
                                                    let drc = detail_row_class.clone();
                                                    let dlc = detail_label_class.clone();
                                                    let dvc = detail_value_class.clone();
                                                    let tlc = trace_link_class.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>
                                                                "trace_id:"
                                                            </span>
                                                            <span class=format!("{} {}", dvc, tlc)>
                                                                {tid}
                                                            </span>
                                                        </div>
                                                    }
                                                })}

                                                // Metadata
                                                {entry_clone.metadata.iter().map(|(k, v)| {
                                                    let drc = detail_row_class2.clone();
                                                    let dlc = detail_label_class2.clone();
                                                    let dvc = detail_value_class2.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>
                                                                {format!("{}:", k)}
                                                            </span>
                                                            <span class=dvc>
                                                                {v.clone()}
                                                            </span>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }
                                    })
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            // Footer
            <div class=footer_class>
                <span class=count_class>
                    {move || format!("Showing {} entries", filtered_logs().len())}
                </span>
            </div>

            // Loading overlay
            {loading.then(|| view! {
                <div class=loading_overlay_class>
                    <span class=loading_spinner_class></span>
                </div>
            })}
        </div>
    }
}
