//! Trend indicator component.

use leptos::prelude::*;
use super::types::{TrendFlag, TrendColorMode};

/// Trend indicator showing direction and value.
///
/// Used within StatCard to display metric changes.
///
/// # Props
///
/// - `flag` - Trend direction (Up/Down/Flat)
/// - `value` - Trend value (e.g., "12%")
/// - `label` - Optional label (e.g., "Week over week")
/// - `color_mode` - How to interpret the trend color
///
/// # Example
///
/// ```ignore
/// <Trend
///     flag=TrendFlag::Up
///     value="12%"
///     label="Week over week"
///     color_mode=TrendColorMode::Standard
/// />
/// ```
#[component]
pub fn Trend(
    /// Trend direction.
    #[prop(optional)]
    flag: TrendFlag,
    /// Trend value (e.g., "12%", "+5").
    #[prop(into)]
    value: String,
    /// Optional label describing the trend.
    #[prop(optional, into)]
    label: Option<String>,
    /// Color interpretation mode.
    #[prop(optional)]
    color_mode: TrendColorMode,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let trend_class = {
        let mut classes = vec![
            "fx-trend".to_string(),
            format!("fx-trend-{}", flag.as_str()),
            color_mode.color_class(flag).to_string(),
        ];

        if let Some(ref custom) = class {
            classes.push(custom.clone());
        }

        classes.join(" ")
    };

    view! {
        <span class=trend_class>
            <span class="fx-trend-icon" aria-hidden="true">
                {flag.icon()}
            </span>
            <span class="fx-trend-value">
                {value}
            </span>
            {label.map(|l| view! {
                <span class="fx-trend-label">{l}</span>
            })}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_flag_display() {
        assert_eq!(TrendFlag::Up.icon(), "▲");
        assert_eq!(TrendFlag::Down.icon(), "▼");
        assert_eq!(TrendFlag::Flat.icon(), "―");
    }

    #[test]
    fn test_color_mode_classes() {
        assert_eq!(
            TrendColorMode::Standard.color_class(TrendFlag::Up),
            "fx-trend-color-success"
        );
        assert_eq!(
            TrendColorMode::Reversed.color_class(TrendFlag::Up),
            "fx-trend-color-error"
        );
    }
}
