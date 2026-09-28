//! ActivityViewer Leptos component.

use leptos::prelude::*;
use super::types::{Activity, ActivityFilter, ActivityCategory};
use crate::try_use_theme;

/// ActivityViewer component.
///
/// Device activity log viewer with filtering and grouping.
///
/// # Props
///
/// - `activities` - Activity entries (reactive)
/// - `filter` - Filter configuration
/// - `max_entries` - Maximum visible entries
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{ActivityViewer, Activity, ActivityCategory};
///
/// let activities = vec![
///     Activity::new(
///         ActivityCategory::System,
///         "AppStarted",
///         "Application started successfully",
///         1716000000000,
///     ),
/// ];
///
/// view! {
///     <ActivityViewer activities=Signal::derive(move || activities.clone()) />
/// }
/// ```
#[component]
pub fn ActivityViewer(
    /// Activity entries.
    activities: Signal<Vec<Activity>>,
    /// Filter configuration.
    #[prop(optional)]
    filter: Option<RwSignal<ActivityFilter>>,
    /// Maximum visible entries (for virtualization).
    #[prop(optional)]
    max_entries: Option<usize>,
    /// Click handler for activity entry.
    #[prop(optional, into)]
    on_activity_click: Option<Callback<Activity>>,
    /// Show timestamps.
    #[prop(optional)]
    show_timestamps: Option<bool>,
    /// Group by category.
    #[prop(optional)]
    group_by_category: Option<bool>,
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
    let max_entries = max_entries.unwrap_or(500);
    let show_timestamps = show_timestamps.unwrap_or(true);
    let _group_by_category = group_by_category.unwrap_or(false);
    let _on_activity_click = on_activity_click;

    // Internal filter state if not provided
    let internal_filter = RwSignal::new(ActivityFilter::default());
    let filter = filter.unwrap_or(internal_filter);

    // Expanded entry state
    let expanded_ids = RwSignal::new(std::collections::HashSet::<String>::new());

    // Build CSS classes
    let activity_prefix = format!("fx-activity-viewer-{}", design_system);

    let combined_class = {
        let mut parts = vec![activity_prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", activity_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Pre-compute all class names
    let filter_bar_class = format!("{}-filter-bar", activity_prefix);
    let search_class = format!("{}-search", activity_prefix);
    let category_filter_class = format!("{}-category-filter", activity_prefix);
    let error_toggle_class = format!("{}-error-toggle", activity_prefix);
    let entries_class = format!("{}-entries", activity_prefix);
    let entry_class = format!("{}-entry", activity_prefix);
    let entry_row_class = format!("{}-entry-row", activity_prefix);
    let timestamp_class = format!("{}-timestamp", activity_prefix);
    let category_badge_class = format!("{}-category-badge", activity_prefix);
    let name_class = format!("{}-name", activity_prefix);
    let description_class = format!("{}-description", activity_prefix);
    let error_indicator_class = format!("{}-error-indicator", activity_prefix);
    let expand_icon_class = format!("{}-expand-icon", activity_prefix);
    let entry_details_class = format!("{}-entry-details", activity_prefix);
    let detail_row_class = format!("{}-detail-row", activity_prefix);
    let detail_label_class = format!("{}-detail-label", activity_prefix);
    let detail_value_class = format!("{}-detail-value", activity_prefix);
    let footer_class = format!("{}-footer", activity_prefix);
    let count_class = format!("{}-count", activity_prefix);
    let loading_overlay_class = format!("{}-loading-overlay", activity_prefix);
    let loading_spinner_class = format!("{}-loading-spinner", activity_prefix);

    // Filter and limit activities
    let filtered_activities = move || {
        let current_filter = filter.get();
        let all_activities = activities.get();

        all_activities
            .into_iter()
            .filter(|a| current_filter.matches(a))
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

    // Format date and time
    let format_datetime = |ts: i64| {
        let total_secs = ts / 1000;
        let days_since_epoch = total_secs / 86400;
        let secs_today = total_secs % 86400;
        let hours = secs_today / 3600;
        let mins = (secs_today % 3600) / 60;
        let secs = secs_today % 60;

        // Simple date calculation (approximate, doesn't account for leap years etc.)
        let year = 1970 + (days_since_epoch / 365);
        let day_of_year = days_since_epoch % 365;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month.min(12), day.min(31), hours, mins, secs
        )
    };

    view! {
        <div class=combined_class>
            // Filter bar
            <div class=filter_bar_class>
                // Search input
                <input
                    type="text"
                    placeholder="Search activities..."
                    class=search_class
                    on:input=move |ev| {
                        filter.update(|f| f.search = event_target_value(&ev));
                    }
                />

                // Category filter
                <select
                    class=category_filter_class
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        filter.update(|f| {
                            f.categories = if value == "all" {
                                vec![]
                            } else {
                                match value.as_str() {
                                    "system" => vec![ActivityCategory::System],
                                    "user" => vec![ActivityCategory::User],
                                    "plugin" => vec![ActivityCategory::Plugin],
                                    "network" => vec![ActivityCategory::Network],
                                    "transaction" => vec![ActivityCategory::Transaction],
                                    "storage" => vec![ActivityCategory::Storage],
                                    "wallet" => vec![ActivityCategory::Wallet],
                                    _ => vec![],
                                }
                            };
                        });
                    }
                >
                    <option value="all">"All Categories"</option>
                    <option value="system">"System"</option>
                    <option value="user">"User"</option>
                    <option value="plugin">"Plugin"</option>
                    <option value="network">"Network"</option>
                    <option value="transaction">"Transaction"</option>
                    <option value="storage">"Storage"</option>
                    <option value="wallet">"Wallet"</option>
                </select>

                // Error toggle
                <label class=error_toggle_class>
                    <input
                        type="checkbox"
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            filter.update(|f| f.errors_only = checked);
                        }
                    />
                    " Errors only"
                </label>
            </div>

            // Activity entries
            <div class=entries_class>
                {move || {
                    let entry_class = entry_class.clone();
                    let entry_row_class = entry_row_class.clone();
                    let timestamp_class = timestamp_class.clone();
                    let category_badge_class = category_badge_class.clone();
                    let name_class = name_class.clone();
                    let description_class = description_class.clone();
                    let error_indicator_class = error_indicator_class.clone();
                    let expand_icon_class = expand_icon_class.clone();
                    let entry_details_class = entry_details_class.clone();
                    let detail_row_class = detail_row_class.clone();
                    let detail_label_class = detail_label_class.clone();
                    let detail_value_class = detail_value_class.clone();

                    filtered_activities().into_iter().map(move |activity| {
                        let activity_id = activity.id.clone();
                        let activity_id_clone = activity_id.clone();
                        let activity_id_for_icon = activity_id.clone();
                        let activity_id_for_details = activity_id.clone();
                        let has_details = !activity.metadata.is_empty()
                            || activity.device_id.is_some()
                            || activity.session_id.is_some()
                            || activity.user_id.is_some();

                        let category_class = format!("{}-category-{}", entry_class, activity.category.as_suffix());
                        let full_entry_class = if activity.is_error {
                            format!("{} {} {}-error", entry_class, category_class, entry_class)
                        } else {
                            format!("{} {}", entry_class, category_class)
                        };

                        let entry_row_class = entry_row_class.clone();
                        let timestamp_class = timestamp_class.clone();
                        let category_badge_class = category_badge_class.clone();
                        let name_class = name_class.clone();
                        let description_class = description_class.clone();
                        let error_indicator_class = error_indicator_class.clone();
                        let expand_icon_class = expand_icon_class.clone();
                        let entry_details_class = entry_details_class.clone();
                        let detail_row_class = detail_row_class.clone();
                        let detail_label_class = detail_label_class.clone();
                        let detail_value_class = detail_value_class.clone();

                        view! {
                            <div
                                class=full_entry_class
                                on:click=move |_| {
                                    if has_details {
                                        expanded_ids.update(|ids| {
                                            if ids.contains(&activity_id_clone) {
                                                ids.remove(&activity_id_clone);
                                            } else {
                                                ids.insert(activity_id_clone.clone());
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
                                            {format_time(activity.timestamp)}
                                        </span>
                                    })}

                                    // Category badge
                                    <span
                                        class=category_badge_class.clone()
                                        style=format!("color: {}; border-color: {}", activity.category.as_color(), activity.category.as_color())
                                    >
                                        <span>{activity.category.as_icon()}</span>
                                        " "
                                        {activity.category.as_label()}
                                    </span>

                                    // Activity name
                                    <span class=name_class.clone()>
                                        {activity.name.clone()}
                                    </span>

                                    // Error indicator
                                    {activity.is_error.then(|| view! {
                                        <span class=error_indicator_class.clone()>"!"</span>
                                    })}

                                    // Description
                                    <span class=description_class.clone()>
                                        {activity.description.clone()}
                                    </span>

                                    // Expand indicator
                                    {has_details.then(|| view! {
                                        <span class=expand_icon_class.clone()>
                                            {move || if expanded_ids.get().contains(&activity_id_for_icon) { "v" } else { ">" }}
                                        </span>
                                    })}
                                </div>

                                // Expanded details
                                {move || {
                                    let entry_details_class = entry_details_class.clone();
                                    let detail_row_class = detail_row_class.clone();
                                    let detail_label_class = detail_label_class.clone();
                                    let detail_value_class = detail_value_class.clone();

                                    expanded_ids.get().contains(&activity_id_for_details).then(|| {
                                        let activity_clone = activity.clone();
                                        let detail_row_class2 = detail_row_class.clone();
                                        let detail_label_class2 = detail_label_class.clone();
                                        let detail_value_class2 = detail_value_class.clone();

                                        view! {
                                            <div class=entry_details_class>
                                                // Full timestamp
                                                <div class=detail_row_class.clone()>
                                                    <span class=detail_label_class.clone()>"Timestamp:"</span>
                                                    <span class=detail_value_class.clone()>
                                                        {format_datetime(activity_clone.timestamp)}
                                                    </span>
                                                </div>

                                                // Device ID
                                                {activity_clone.device_id.clone().map(|did| {
                                                    let drc = detail_row_class.clone();
                                                    let dlc = detail_label_class.clone();
                                                    let dvc = detail_value_class.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>"Device:"</span>
                                                            <span class=dvc>{did}</span>
                                                        </div>
                                                    }
                                                })}

                                                // Session ID
                                                {activity_clone.session_id.clone().map(|sid| {
                                                    let drc = detail_row_class.clone();
                                                    let dlc = detail_label_class.clone();
                                                    let dvc = detail_value_class.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>"Session:"</span>
                                                            <span class=dvc>{sid}</span>
                                                        </div>
                                                    }
                                                })}

                                                // User ID
                                                {activity_clone.user_id.clone().map(|uid| {
                                                    let drc = detail_row_class.clone();
                                                    let dlc = detail_label_class.clone();
                                                    let dvc = detail_value_class.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>"User:"</span>
                                                            <span class=dvc>{uid}</span>
                                                        </div>
                                                    }
                                                })}

                                                // Metadata
                                                {activity_clone.metadata.iter().map(|(k, v)| {
                                                    let drc = detail_row_class2.clone();
                                                    let dlc = detail_label_class2.clone();
                                                    let dvc = detail_value_class2.clone();
                                                    view! {
                                                        <div class=drc>
                                                            <span class=dlc>{format!("{}:", k)}</span>
                                                            <span class=dvc>{v.clone()}</span>
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
                    {move || format!("Showing {} activities", filtered_activities().len())}
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
