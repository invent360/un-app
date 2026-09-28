//! Badge Leptos component.

use leptos::prelude::*;
use super::types::BadgeStatus;
use crate::try_use_theme;

/// Badge component.
///
/// Small status indicator or count display.
///
/// # Props
///
/// - `count` - Number to display in badge
/// - `dot` - Show as dot instead of count
/// - `status` - Badge status color
/// - `text` - Status text (when using status dot)
/// - `overflow_count` - Max count before showing "99+"
/// - `show_zero` - Whether to show badge when count is 0
/// - `offset` - Badge position offset [x, y]
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::notification::Badge;
///
/// view! {
///     <Badge count=5>
///         <div class="icon">Mail</div>
///     </Badge>
/// }
/// ```
#[component]
pub fn Badge(
    /// Number to display.
    #[prop(optional)]
    count: Option<u32>,
    /// Show as dot instead of count.
    #[prop(optional)]
    dot: bool,
    /// Badge status/color.
    #[prop(optional, into)]
    status: Option<BadgeStatus>,
    /// Status text (for status badges).
    #[prop(optional, into)]
    text: Option<String>,
    /// Max count before overflow.
    #[prop(optional)]
    overflow_count: Option<u32>,
    /// Show badge when count is 0.
    #[prop(optional)]
    show_zero: bool,
    /// Custom badge color.
    #[prop(optional, into)]
    color: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Child content to wrap.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let overflow_count = overflow_count.unwrap_or(99);

    // Build CSS classes
    let badge_prefix = format!("fx-badge-{}", design_system);

    // Determine if badge should show
    let should_show = count.map(|c| c > 0 || show_zero).unwrap_or(false) || dot || status.is_some();

    // Format count display
    let count_display = count.map(|c| {
        if c > overflow_count {
            format!("{}+", overflow_count)
        } else {
            c.to_string()
        }
    });

    // Build combined class
    let combined_class = {
        let mut parts = vec![badge_prefix.clone()];
        if status.is_some() {
            parts.push(format!("{}-status", badge_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Badge element classes
    let count_class = format!("{}-count", badge_prefix);
    let dot_class = format!("{}-dot", badge_prefix);
    let status_dot_class = format!("{}-status-dot", badge_prefix);
    let status_text_class = format!("{}-status-text", badge_prefix);

    // Status-specific class
    let status_class = status.map(|s| s.class(&badge_prefix));

    view! {
        <span class=combined_class>
            {children.map(|c| c())}

            {if status.is_some() {
                // Status badge (dot + text)
                let status_dot_combined = {
                    let mut cls = vec![status_dot_class.clone()];
                    if let Some(ref sc) = status_class {
                        cls.push(sc.clone());
                    }
                    cls.join(" ")
                };
                Some(view! {
                    <span
                        class=status_dot_combined
                        style=color.clone().map(|c| format!("background-color: {}", c))
                    ></span>
                    {text.clone().map(|t| view! {
                        <span class=status_text_class.clone()>{t}</span>
                    })}
                }.into_any())
            } else if dot && should_show {
                // Dot badge
                Some(view! {
                    <sup
                        class=dot_class.clone()
                        style=color.clone().map(|c| format!("background-color: {}", c))
                    ></sup>
                }.into_any())
            } else if let Some(ref display) = count_display {
                // Count badge
                if should_show {
                    Some(view! {
                        <sup
                            class=count_class.clone()
                            style=color.clone().map(|c| format!("background-color: {}", c))
                        >
                            {display.clone()}
                        </sup>
                    }.into_any())
                } else {
                    None
                }
            } else {
                None
            }}
        </span>
    }
}
