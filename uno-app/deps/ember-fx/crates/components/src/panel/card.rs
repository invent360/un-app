//! StatCard component for dashboard metrics.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{StatCardSize, StatIcon};

/// StatCard component for displaying key metrics.
///
/// A card component inspired by Ant Design Pro's ChartCard and ORION dashboard,
/// designed for dashboard metric displays with icons, trends and charts.
///
/// # Props
///
/// - `title` - Card title
/// - `value` - Main metric value
/// - `tooltip` - Optional tooltip for the title
/// - `icon` - Optional icon badge
/// - `icon_color` - Custom icon color (CSS color string)
/// - `subtitle` - Optional subtitle below value
/// - `size` - Card size variant
/// - `loading` - Loading state
/// - `bordered` - Show border
/// - `progress` - Optional progress value (0.0 to 1.0)
///
/// # Slots
///
/// - `children` - Main content area (trend indicators)
/// - `chart` - Optional chart/sparkline area
/// - `footer` - Footer area for secondary metrics
///
/// # Example
///
/// ```ignore
/// <StatCard
///     title="Total Sales"
///     value="$126,560"
///     icon=StatIcon::Dollar
///     icon_color="#52c41a"
///     tooltip="Total sales this month"
///     progress=0.75
/// >
///     <Trend flag=TrendFlag::Up value="12%" label="WoW" />
///     <div slot:footer>
///         <Field label="Daily Sales" value="$12,423" />
///     </div>
/// </StatCard>
/// ```
#[component]
pub fn StatCard(
    /// Card title.
    #[prop(into)]
    title: String,
    /// Main metric value.
    #[prop(into)]
    value: String,
    /// Tooltip text for the title.
    #[prop(optional, into)]
    tooltip: Option<String>,
    /// Icon badge.
    #[prop(optional)]
    icon: Option<StatIcon>,
    /// Custom icon color (overrides default icon color).
    #[prop(optional, into)]
    icon_color: Option<String>,
    /// Subtitle text below value.
    #[prop(optional, into)]
    subtitle: Option<String>,
    /// Card size.
    #[prop(optional)]
    size: StatCardSize,
    /// Loading state.
    #[prop(optional)]
    loading: bool,
    /// Show border.
    #[prop(optional)]
    bordered: bool,
    /// Progress value (0.0 to 1.0) for mini progress indicator.
    #[prop(optional)]
    progress: Option<f64>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Trend/content area.
    #[prop(optional)]
    children: Option<Children>,
    /// Chart/sparkline slot.
    #[prop(optional)]
    chart: Option<Children>,
    /// Footer slot.
    #[prop(optional)]
    footer: Option<Children>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-statcard-{}", design_system);
    let prefix_for_class = prefix.clone();

    let card_class = move || {
        let mut classes = vec![
            prefix_for_class.clone(),
            format!("{}-{}", prefix_for_class, size.as_str()),
        ];

        if loading {
            classes.push(format!("{}-loading", prefix_for_class));
        }

        if bordered {
            classes.push(format!("{}-bordered", prefix_for_class));
        }

        if icon.is_some() {
            classes.push(format!("{}-with-icon", prefix_for_class));
        }

        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }

        classes.join(" ")
    };

    let value_class = format!("{}-value-{}", prefix, size.as_str());

    // Icon sizing based on card size
    let icon_size = match size {
        StatCardSize::Small => 32,
        StatCardSize::Default => 48,
        StatCardSize::Large => 64,
    };

    view! {
        <div
            class=card_class
            style="background: var(--fx-color-bg-container, #1f1f1f); \
                   border-radius: var(--fx-border-radius-lg, 8px); \
                   padding: 20px; \
                   position: relative; \
                   transition: box-shadow 0.2s ease;"
        >
            // Loading overlay
            {loading.then(|| view! {
                <div style="position: absolute; inset: 0; display: flex; align-items: center; \
                            justify-content: center; background: rgba(0,0,0,0.4); \
                            border-radius: var(--fx-border-radius-lg, 8px); z-index: 10;">
                    <span style="width: 24px; height: 24px; border: 2px solid transparent; \
                                 border-top-color: var(--fx-color-primary, #1890ff); \
                                 border-radius: 50%; animation: spin 1s linear infinite;"></span>
                </div>
            })}

            <div style="display: flex; align-items: flex-start; gap: 16px;">
                // Icon badge (optional)
                {icon.map(|i| {
                    let color = icon_color.clone()
                        .unwrap_or_else(|| i.default_color().to_string());
                    view! {
                        <div style=format!(
                            "width: {}px; height: {}px; border-radius: 12px; \
                             background: {}; display: flex; align-items: center; \
                             justify-content: center; flex-shrink: 0; \
                             box-shadow: 0 4px 12px {}40;",
                            icon_size, icon_size, color, color
                        )>
                            <svg
                                viewBox="0 0 24 24"
                                width=icon_size / 2
                                height=icon_size / 2
                                fill="white"
                            >
                                <path d=i.svg_path() />
                            </svg>
                        </div>
                    }
                })}

                <div style="flex: 1; min-width: 0;">
                    // Header with title
                    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 8px;">
                        <span
                            style="font-size: 14px; color: var(--fx-color-text-secondary, #888); \
                                   font-weight: 500;"
                            title=tooltip.clone()
                        >
                            {title}
                        </span>
                        {tooltip.is_some().then(|| view! {
                            <span
                                style="color: var(--fx-color-text-tertiary, #666); cursor: help;"
                                aria-hidden="true"
                            >
                                "ⓘ"
                            </span>
                        })}
                    </div>

                    // Value display
                    <div
                        class=value_class
                        style="font-size: 28px; font-weight: 600; \
                               color: var(--fx-color-text, #fff); \
                               line-height: 1.2; margin-bottom: 4px;"
                    >
                        {value}
                    </div>

                    // Subtitle
                    {subtitle.map(|s| view! {
                        <div style="font-size: 12px; color: var(--fx-color-text-tertiary, #666); \
                                    margin-bottom: 8px;">
                            {s}
                        </div>
                    })}

                    // Trend indicators
                    {children.map(|c| view! {
                        <div style="display: flex; align-items: center; gap: 8px; margin-top: 8px;">
                            {c()}
                        </div>
                    })}
                </div>
            </div>

            // Progress bar (optional)
            {progress.map(|p| {
                let pct = (p * 100.0).clamp(0.0, 100.0);
                view! {
                    <div style="margin-top: 16px;">
                        <div style="height: 4px; background: var(--fx-color-bg-elevated, #2a2a2a); \
                                    border-radius: 2px; overflow: hidden;">
                            <div style=format!(
                                "height: 100%; width: {:.1}%; \
                                 background: linear-gradient(90deg, var(--fx-color-primary), var(--fx-color-success)); \
                                 border-radius: 2px; transition: width 0.5s ease;",
                                pct
                            ) />
                        </div>
                    </div>
                }
            })}

            // Chart area (optional)
            {chart.map(|c| view! {
                <div style="margin-top: 16px;">
                    {c()}
                </div>
            })}

            // Footer
            {footer.map(|f| view! {
                <div style="margin-top: 16px; padding-top: 12px; \
                            border-top: 1px solid var(--fx-color-border, #303030);">
                    {f()}
                </div>
            })}
        </div>
    }
}

/// Stat group for organizing multiple StatCards.
///
/// Provides responsive grid layout for StatCard components.
#[component]
pub fn StatGroup(
    /// Number of columns (1-4).
    #[prop(optional)]
    columns: Option<u8>,
    /// Gap between cards.
    #[prop(optional, into)]
    gap: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// StatCard children.
    children: Children,
) -> impl IntoView {
    let cols = columns.unwrap_or(4).min(4).max(1);
    let gap_value = gap.unwrap_or_else(|| "16px".to_string());

    let group_class = {
        let mut classes = vec![
            "fx-statgroup".to_string(),
            format!("fx-statgroup-cols-{}", cols),
        ];
        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }
        classes.join(" ")
    };

    let style = format!("gap: {};", gap_value);

    view! {
        <div class=group_class style=style>
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statcard_size() {
        assert_eq!(StatCardSize::Small.as_str(), "sm");
        assert_eq!(StatCardSize::Default.as_str(), "md");
        assert_eq!(StatCardSize::Large.as_str(), "lg");
    }

    #[test]
    fn test_statcard_value_class() {
        assert_eq!(StatCardSize::Small.value_class(), "fx-statcard-value-sm");
        assert_eq!(StatCardSize::Default.value_class(), "fx-statcard-value-md");
        assert_eq!(StatCardSize::Large.value_class(), "fx-statcard-value-lg");
    }
}
