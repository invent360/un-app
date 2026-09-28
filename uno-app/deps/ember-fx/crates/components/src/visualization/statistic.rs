//! Statistic Leptos component.

use leptos::prelude::*;
use super::types::StatisticValueStyle;
use crate::try_use_theme;

/// Statistic component.
///
/// Display statistic numbers.
///
/// # Props
///
/// - `title` - Statistic title
/// - `value` - Statistic value
/// - `prefix` - Value prefix
/// - `suffix` - Value suffix
/// - `precision` - Decimal precision
/// - `value_style` - Value style (Positive, Negative)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::Statistic;
///
/// view! {
///     <Statistic title="Active Users" value="112,893" />
/// }
/// ```
#[component]
pub fn Statistic(
    /// Statistic title.
    #[prop(into)]
    title: String,
    /// Statistic value.
    #[prop(into)]
    value: String,
    /// Value prefix.
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Value suffix.
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Value style.
    #[prop(optional, into)]
    value_style: Option<StatisticValueStyle>,
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

    // Resolve defaults
    let value_style = value_style.unwrap_or_default();

    // Build CSS classes
    let statistic_prefix = format!("fx-statistic-{}", design_system);
    let style_class = value_style.class(&statistic_prefix);

    let combined_class = {
        let mut parts = vec![statistic_prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", statistic_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            <div class=format!("{}-title", statistic_prefix)>
                {title}
            </div>
            <div class=format!("{}-content", statistic_prefix)>
                <span class=format!("{}-value {}", statistic_prefix, style_class)>
                    {prefix.clone().map(|p| view! {
                        <span class=format!("{}-prefix", statistic_prefix)>{p}</span>
                    })}
                    <span class=format!("{}-value-int", statistic_prefix)>
                        {if loading { "---".to_string() } else { value.clone() }}
                    </span>
                    {suffix.clone().map(|s| view! {
                        <span class=format!("{}-suffix", statistic_prefix)>{s}</span>
                    })}
                </span>
            </div>
        </div>
    }
}

/// Statistic Group component.
///
/// Group multiple statistics together.
#[component]
pub fn StatisticGroup(
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Statistic children.
    children: Children,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let statistic_prefix = format!("fx-statistic-{}", design_system);
    let group_class = format!("{}-group", statistic_prefix);

    let combined_class = {
        let mut parts = vec![group_class];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}

/// Countdown component.
///
/// Countdown timer statistic.
///
/// # Props
///
/// - `title` - Countdown title
/// - `value` - Target timestamp (milliseconds)
/// - `format` - Display format (default: "HH:mm:ss")
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::visualization::Countdown;
///
/// view! {
///     <Countdown title="Countdown" value=deadline />
/// }
/// ```
#[component]
pub fn Countdown(
    /// Countdown title.
    #[prop(into)]
    title: String,
    /// Target timestamp in milliseconds.
    #[prop(into)]
    value: Signal<i64>,
    /// Display format.
    #[prop(optional, into)]
    format: Option<String>,
    /// On finish callback.
    #[prop(optional, into)]
    on_finish: Option<Callback<()>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let _format = format.unwrap_or_else(|| "HH:mm:ss".to_string());
    let _on_finish = on_finish;

    // Build CSS classes
    let statistic_prefix = format!("fx-statistic-{}", design_system);
    let countdown_class = format!("{}-countdown", statistic_prefix);

    let combined_class = {
        let mut parts = vec![statistic_prefix.clone(), countdown_class];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Calculate remaining time
    let remaining = move || {
        let target = value.get();
        let now = js_sys::Date::now() as i64;
        let diff = (target - now).max(0);

        let hours = diff / (1000 * 60 * 60);
        let minutes = (diff % (1000 * 60 * 60)) / (1000 * 60);
        let seconds = (diff % (1000 * 60)) / 1000;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    };

    view! {
        <div class=combined_class>
            <div class=format!("{}-title", statistic_prefix)>
                {title}
            </div>
            <div class=format!("{}-content", statistic_prefix)>
                <span class=format!("{}-value", statistic_prefix)>
                    {remaining}
                </span>
            </div>
        </div>
    }
}
