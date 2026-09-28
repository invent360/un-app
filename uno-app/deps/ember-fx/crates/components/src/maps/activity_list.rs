//! ActivityList component for country/region breakdown tables.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{ActivityItem, ActivityListConfig, ActivitySortOrder, format_number};

/// ActivityList component.
///
/// Displays a list of items with values, percentages, and progress bars.
/// Commonly used for country/region activity breakdowns.
///
/// # Example
///
/// ```ignore
/// let items = Signal::derive(|| vec![
///     ActivityItem::new("us", "United States", 4504210.0, 53.0),
///     ActivityItem::new("fr", "France", 2100950.0, 15.0),
/// ]);
///
/// view! {
///     <ActivityList
///         items=items
///         title="Current Activity"
///     />
/// }
/// ```
#[component]
pub fn ActivityList(
    /// Activity items to display.
    items: Signal<Vec<ActivityItem>>,
    /// List title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<ActivityListConfig>,
    /// Item click handler.
    #[prop(optional, into)]
    on_item_click: Option<Callback<ActivityItem>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-activitylist-{}", design_system);

    // Extract config
    let show_icons = config.show_icons;
    let show_progress_bars = config.show_progress_bars;
    let show_percentages = config.show_percentages;
    let max_items = config.max_items;
    let sort_order = config.sort_order;
    let animated = config.animated;

    // Sort and limit items
    let sorted_items = move || {
        let mut list = items.get();

        // Sort based on order
        match sort_order {
            ActivitySortOrder::ValueDesc => list.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap()),
            ActivitySortOrder::ValueAsc => list.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap()),
            ActivitySortOrder::Alphabetical => list.sort_by(|a, b| a.name.cmp(&b.name)),
            ActivitySortOrder::PercentageDesc => list.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap()),
        }

        // Limit items
        if let Some(max) = max_items {
            list.truncate(max);
        }

        list
    };

    // Find max value for progress bar scaling
    let max_percentage = move || {
        sorted_items()
            .iter()
            .map(|i| i.percentage)
            .fold(0.0_f64, |a, b| a.max(b))
            .max(1.0)
    };

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div class=combined_class>
            // Header
            {title.clone().map(|t| view! {
                <div style="margin-bottom: 12px;">
                    <h4 style="margin: 0; font-size: 14px; font-weight: 600; color: var(--fx-color-text, #fff);">
                        {t}
                    </h4>
                    // Progress bar legend header
                    {show_progress_bars.then(|| view! {
                        <div style="margin-top: 8px; height: 4px; background: linear-gradient(90deg, var(--fx-color-success), var(--fx-color-primary), var(--fx-color-warning)); border-radius: 2px;" />
                    })}
                </div>
            })}

            // Items list
            <ul style="list-style: none; margin: 0; padding: 0;">
                {move || {
                    let max_pct = max_percentage();

                    sorted_items().into_iter().map(|item| {
                        let item_id = item.id.clone();
                        let item_clone = item.clone();
                        let item_for_click = item.clone();
                        let callback = on_item_click.clone();

                        let has_click = callback.is_some();
                        let cursor = if has_click { "pointer" } else { "default" };

                        // Calculate relative width for progress bar
                        let bar_width = if max_pct > 0.0 {
                            item.percentage / max_pct * 100.0
                        } else {
                            0.0
                        };

                        let transition = if animated { "width 0.5s ease" } else { "none" };

                        // Get item color or default
                        let bar_color = item.color
                            .map(|c| c.as_css().to_string())
                            .unwrap_or_else(|| "var(--fx-color-primary)".to_string());

                        // Trend indicator
                        let trend_view = item.trend.map(|t| {
                            let arrow = t.arrow();
                            let color = t.color();
                            view! {
                                <span style=format!("color: {}; margin-left: 4px;", color)>
                                    {arrow}
                                </span>
                            }
                        });

                        view! {
                            <li
                                data-key=item_id
                                style=format!(
                                    "display: flex; align-items: center; gap: 12px; padding: 8px 0; \
                                     border-bottom: 1px solid var(--fx-color-border, #303030); cursor: {};",
                                    cursor
                                )
                                on:click=move |_| {
                                    if let Some(ref cb) = callback {
                                        cb.run(item_for_click.clone());
                                    }
                                }
                            >
                                // Icon/flag
                                {show_icons.then(|| {
                                    let icon = item_clone.icon.clone();
                                    view! {
                                        <span style="font-size: 16px; width: 24px; text-align: center;">
                                            {icon.unwrap_or_default()}
                                        </span>
                                    }
                                })}

                                // Name
                                <span style="flex: 1; font-size: 14px; color: var(--fx-color-text, #fff);">
                                    {item.name.clone()}
                                </span>

                                // Value
                                <span style="font-size: 14px; font-weight: 500; color: var(--fx-color-text, #fff); min-width: 80px; text-align: right;">
                                    {format_number(item.value)}
                                </span>

                                // Percentage
                                {show_percentages.then(|| view! {
                                    <span style="font-size: 12px; color: var(--fx-color-text-secondary, #888); min-width: 40px; text-align: right;">
                                        {format!("{}%", item.percentage as i32)}
                                    </span>
                                })}

                                // Trend
                                {trend_view}

                                // Progress bar
                                {show_progress_bars.then(|| view! {
                                    <div style="width: 80px; height: 4px; background: var(--fx-color-bg-elevated, #1f1f1f); border-radius: 2px; overflow: hidden;">
                                        <div style=format!(
                                            "height: 100%; width: {}%; background: {}; border-radius: 2px; transition: {};",
                                            bar_width, bar_color, transition
                                        ) />
                                    </div>
                                })}
                            </li>
                        }
                    }).collect_view()
                }}
            </ul>
        </div>
    }
}
