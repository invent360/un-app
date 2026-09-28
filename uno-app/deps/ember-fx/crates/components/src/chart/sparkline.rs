//! SparklineChart Leptos component.

use leptos::prelude::*;
use super::types::{SparklineConfig, ChartColor};
use crate::try_use_theme;

/// SparklineChart component.
///
/// Compact inline chart for embedding in tables or cards.
///
/// # Props
///
/// - `data` - Numeric values (no timestamps, just values in order)
/// - `config` - Chart configuration
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::SparklineChart;
///
/// let values = vec![10.0, 15.0, 12.0, 18.0, 22.0, 19.0, 25.0];
///
/// view! {
///     <SparklineChart data=Signal::derive(move || values.clone()) />
/// }
/// ```
#[component]
pub fn SparklineChart(
    /// Numeric values in order.
    data: Signal<Vec<f64>>,
    /// Chart configuration.
    #[prop(optional)]
    config: Option<SparklineConfig>,
    /// Color override.
    #[prop(optional)]
    color: Option<ChartColor>,
    /// Show trend indicator.
    #[prop(optional)]
    show_trend: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve config
    let config = config.unwrap_or_default();
    let chart_width = config.width;
    let chart_height = config.height;
    let line_width = config.line_width;
    let show_markers = config.show_markers;
    let fill = config.fill;
    let color = color.unwrap_or(config.color);
    let show_trend = show_trend.unwrap_or(false);

    // Build CSS classes
    let chart_prefix = format!("fx-sparkline-{}", design_system);

    let combined_class = {
        let mut parts = vec![chart_prefix.clone()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Padding for markers
    let padding = if show_markers { 3.0 } else { 1.0 };
    let inner_width = chart_width as f64 - padding * 2.0;
    let inner_height = chart_height as f64 - padding * 2.0;

    // Calculate path
    let path = move || {
        let values = data.get();
        if values.is_empty() {
            return String::new();
        }

        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let val_range = (max_val - min_val).max(0.001);

        let step = inner_width / (values.len() - 1).max(1) as f64;

        let mut path = String::new();
        for (i, &val) in values.iter().enumerate() {
            let x = padding + step * i as f64;
            let y = padding + inner_height - ((val - min_val) / val_range) * inner_height;

            if i == 0 {
                path.push_str(&format!("M {} {}", x, y));
            } else {
                path.push_str(&format!(" L {} {}", x, y));
            }
        }
        path
    };

    // Calculate fill path
    let fill_path = move || {
        if !fill {
            return String::new();
        }

        let values = data.get();
        if values.is_empty() {
            return String::new();
        }

        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let val_range = (max_val - min_val).max(0.001);

        let step = inner_width / (values.len() - 1).max(1) as f64;
        let baseline_y = padding + inner_height;

        let mut path = format!("M {} {}", padding, baseline_y);

        for (i, &val) in values.iter().enumerate() {
            let x = padding + step * i as f64;
            let y = padding + inner_height - ((val - min_val) / val_range) * inner_height;
            path.push_str(&format!(" L {} {}", x, y));
        }

        let last_x = padding + step * (values.len() - 1).max(0) as f64;
        path.push_str(&format!(" L {} {} Z", last_x, baseline_y));
        path
    };

    // Calculate min/max markers
    let markers = move || {
        if !show_markers {
            return vec![];
        }

        let values = data.get();
        if values.is_empty() {
            return vec![];
        }

        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let val_range = (max_val - min_val).max(0.001);

        let step = inner_width / (values.len() - 1).max(1) as f64;

        let mut markers = vec![];

        // Find min and max indices
        let min_idx = values.iter().enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i);
        let max_idx = values.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i);

        if let Some(idx) = min_idx {
            let x = padding + step * idx as f64;
            let y = padding + inner_height - ((values[idx] - min_val) / val_range) * inner_height;
            markers.push((x, y, ChartColor::Error)); // Red for min
        }

        if let Some(idx) = max_idx {
            let x = padding + step * idx as f64;
            let y = padding + inner_height - ((values[idx] - min_val) / val_range) * inner_height;
            markers.push((x, y, ChartColor::Success)); // Green for max
        }

        markers
    };

    // Calculate trend
    let trend = move || {
        if !show_trend {
            return None;
        }

        let values = data.get();
        if values.len() < 2 {
            return None;
        }

        let first = values.first().copied()?;
        let last = values.last().copied()?;
        let diff = last - first;
        let percent = if first != 0.0 { (diff / first) * 100.0 } else { 0.0 };

        Some((diff > 0.0, percent))
    };

    let stroke_color = color.as_css();
    let fill_color = color.as_fill_css();

    // Pre-compute class names
    let svg_class = format!("{}-svg", chart_prefix);
    let fill_class = format!("{}-fill", chart_prefix);
    let line_class = format!("{}-line", chart_prefix);
    let marker_class = format!("{}-marker", chart_prefix);
    let trend_up_class = format!("{}-trend-up", chart_prefix);
    let trend_down_class = format!("{}-trend-down", chart_prefix);

    view! {
        <span class=combined_class style="display: inline-flex; align-items: center; gap: 4px;">
            <svg
                width=chart_width
                height=chart_height
                viewBox=format!("0 0 {} {}", chart_width, chart_height)
                class=svg_class
            >
                // Fill area
                {fill.then(|| view! {
                    <path
                        d=fill_path
                        fill=fill_color
                        class=fill_class.clone()
                    />
                })}

                // Line
                <path
                    d=path
                    fill="none"
                    stroke=stroke_color
                    stroke-width=line_width
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class=line_class
                />

                // Min/max markers
                {move || {
                    let marker_class = marker_class.clone();
                    markers().into_iter().map(move |(x, y, marker_color)| {
                        view! {
                            <circle
                                cx=x
                                cy=y
                                r="2.5"
                                fill=marker_color.as_css()
                                class=marker_class.clone()
                            />
                        }
                    }).collect::<Vec<_>>()
                }}
            </svg>

            // Trend indicator
            {move || {
                let trend_up_class = trend_up_class.clone();
                let trend_down_class = trend_down_class.clone();
                trend().map(move |(is_up, percent)| {
                    let trend_class = if is_up {
                        trend_up_class.clone()
                    } else {
                        trend_down_class.clone()
                    };
                    let arrow = if is_up { "↑" } else { "↓" };
                    let color = if is_up { ChartColor::Success } else { ChartColor::Error };

                    view! {
                        <span
                            class=trend_class
                            style=format!("color: {}; font-size: 10px;", color.as_css())
                        >
                            {arrow}{format!("{:.1}%", percent.abs())}
                        </span>
                    }
                })
            }}
        </span>
    }
}
