//! Timeline Leptos component.

use leptos::prelude::*;
use super::types::{TimelineMode, TimelineItem, TimelineItemColor};
use crate::try_use_theme;

/// Timeline component.
///
/// Vertical timeline display.
///
/// # Props
///
/// - `items` - Timeline items
/// - `mode` - Display mode (Left, Right, Alternate)
/// - `pending` - Show pending indicator
/// - `pending_dot` - Custom pending dot
/// - `reverse` - Reverse item order
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::{Timeline, TimelineItem};
///
/// let items = vec![
///     TimelineItem::new("Create project"),
///     TimelineItem::new("Solve initial bugs"),
///     TimelineItem::new("Technical review"),
/// ];
///
/// view! {
///     <Timeline items=items />
/// }
/// ```
#[component]
pub fn Timeline(
    /// Timeline items.
    #[prop(into)]
    items: Vec<TimelineItem>,
    /// Display mode.
    #[prop(optional, into)]
    mode: Option<TimelineMode>,
    /// Show pending indicator.
    #[prop(optional)]
    pending: bool,
    /// Custom pending dot.
    #[prop(optional, into)]
    pending_dot: Option<String>,
    /// Reverse order.
    #[prop(optional)]
    reverse: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let mode = mode.unwrap_or_default();

    // Build CSS classes
    let timeline_prefix = format!("fx-timeline-{}", design_system);
    let mode_class = mode.class(&timeline_prefix);

    let combined_class = {
        let mut parts = vec![timeline_prefix.clone(), mode_class];
        if pending {
            parts.push(format!("{}-pending", timeline_prefix));
        }
        if reverse {
            parts.push(format!("{}-reverse", timeline_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Process items
    let display_items: Vec<TimelineItem> = if reverse {
        items.into_iter().rev().collect()
    } else {
        items
    };

    view! {
        <ul class=combined_class>
            {display_items.into_iter().enumerate().map(|(idx, item)| {
                let timeline_prefix = timeline_prefix.clone();
                let _is_last = pending && idx == 0 && reverse || pending && !reverse;
                let mode = mode;

                view! {
                    <li class=move || {
                        let mut cls = vec![format!("{}-item", timeline_prefix)];
                        if mode == TimelineMode::Alternate {
                            if idx % 2 == 0 {
                                cls.push(format!("{}-item-left", timeline_prefix));
                            } else {
                                cls.push(format!("{}-item-right", timeline_prefix));
                            }
                        }
                        cls.join(" ")
                    }>
                        <div class=format!("{}-item-tail", timeline_prefix)></div>
                        <div
                            class=format!("{}-item-head", timeline_prefix)
                            style=format!("border-color: {}; color: {};",
                                item.color.as_css(), item.color.as_css())
                        >
                            {item.dot.clone().unwrap_or_else(|| "".to_string())}
                        </div>
                        <div class=format!("{}-item-content", timeline_prefix)>
                            {item.content.clone()}
                        </div>
                        {item.label.clone().map(|l| view! {
                            <div class=format!("{}-item-label", timeline_prefix)>
                                {l}
                            </div>
                        })}
                    </li>
                }
            }).collect_view()}
            {if pending {
                let timeline_prefix = timeline_prefix.clone();
                Some(view! {
                    <li class=format!("{}-item {}-item-pending", timeline_prefix, timeline_prefix)>
                        <div class=format!("{}-item-tail", timeline_prefix)></div>
                        <div class=format!("{}-item-head {}-item-head-pending", timeline_prefix, timeline_prefix)>
                            {pending_dot.clone().unwrap_or_else(|| "⏳".to_string())}
                        </div>
                        <div class=format!("{}-item-content", timeline_prefix)>
                            "..."
                        </div>
                    </li>
                })
            } else {
                None
            }}
        </ul>
    }
}

/// Timeline Item Component for composition pattern.
#[component]
pub fn TimelineItemComponent(
    /// Item content.
    #[prop(into)]
    content: String,
    /// Item label.
    #[prop(optional, into)]
    label: Option<String>,
    /// Dot color.
    #[prop(optional, into)]
    color: Option<TimelineItemColor>,
    /// Custom dot content.
    #[prop(optional, into)]
    dot: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let color = color.unwrap_or_default();
    let timeline_prefix = format!("fx-timeline-{}", design_system);

    let combined_class = {
        let mut parts = vec![format!("{}-item", timeline_prefix)];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <li class=combined_class>
            <div class=format!("{}-item-tail", timeline_prefix)></div>
            <div
                class=format!("{}-item-head", timeline_prefix)
                style=format!("border-color: {}; color: {};", color.as_css(), color.as_css())
            >
                {dot.unwrap_or_default()}
            </div>
            <div class=format!("{}-item-content", timeline_prefix)>
                {content}
            </div>
            {label.map(|l| view! {
                <div class=format!("{}-item-label", timeline_prefix)>
                    {l}
                </div>
            })}
        </li>
    }
}
