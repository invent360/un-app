//! BatteryGauge Leptos component.
//!
//! A battery-style gauge with stacked horizontal bars and voltage display.

use leptos::prelude::*;
use crate::try_use_theme;

/// Configuration for the battery gauge.
#[derive(Debug, Clone)]
pub struct BatteryGaugeConfig {
    /// Number of bars in the gauge.
    pub bar_count: usize,
    /// Bar height in pixels.
    pub bar_height: u32,
    /// Gap between bars in pixels.
    pub bar_gap: u32,
    /// Width of the gauge.
    pub width: u32,
    /// Voltage unit label.
    pub unit: String,
    /// Minimum voltage (empty).
    pub min_voltage: f64,
    /// Maximum voltage (full).
    pub max_voltage: f64,
    /// Decimal precision for voltage display.
    pub precision: usize,
}

impl Default for BatteryGaugeConfig {
    fn default() -> Self {
        Self {
            bar_count: 14,
            bar_height: 16,
            bar_gap: 4,
            width: 200,
            unit: "V".to_string(),
            min_voltage: 3.0,
            max_voltage: 4.2,
            precision: 4,
        }
    }
}

/// Get color for a bar based on its position (0 = top/green, 1 = bottom/red).
fn bar_color(position: f64) -> String {
    // Gradient from green (top) -> yellow -> orange -> red (bottom)
    let (r, g, b) = if position < 0.4 {
        // Green to yellow-green
        let t = position / 0.4;
        (
            (80.0 + t * 100.0) as u8,
            (200.0 - t * 30.0) as u8,
            (80.0 - t * 40.0) as u8,
        )
    } else if position < 0.6 {
        // Yellow-green to yellow
        let t = (position - 0.4) / 0.2;
        (
            (180.0 + t * 50.0) as u8,
            (170.0 - t * 20.0) as u8,
            (40.0) as u8,
        )
    } else if position < 0.8 {
        // Yellow to orange
        let t = (position - 0.6) / 0.2;
        (
            (230.0 + t * 20.0) as u8,
            (150.0 - t * 70.0) as u8,
            (40.0 + t * 20.0) as u8,
        )
    } else {
        // Orange to red
        let t = (position - 0.8) / 0.2;
        (
            (250.0) as u8,
            (80.0 - t * 40.0) as u8,
            (60.0 + t * 20.0) as u8,
        )
    };

    format!("rgb({}, {}, {})", r, g, b)
}

/// Get text color based on voltage level.
fn voltage_color(level: f64) -> &'static str {
    if level > 0.6 {
        "#4ade80" // Green
    } else if level > 0.3 {
        "#fbbf24" // Yellow
    } else {
        "#f87171" // Red
    }
}

/// BatteryGauge component.
///
/// A battery-style gauge showing voltage level with stacked colored bars.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{BatteryGauge, BatteryGaugeConfig};
///
/// view! {
///     <BatteryGauge
///         title="Entrance Battery".to_string()
///         voltage=Signal::derive(|| 3.642)
///     />
/// }
/// ```
#[component]
pub fn BatteryGauge(
    /// Gauge title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Current voltage value.
    #[prop(into)]
    voltage: Signal<f64>,
    /// Configuration.
    #[prop(optional)]
    config: Option<BatteryGaugeConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let bar_count = config.bar_count;
    let bar_height = config.bar_height;
    let bar_gap = config.bar_gap;
    let width = config.width;
    let unit = config.unit.clone();
    let min_voltage = config.min_voltage;
    let max_voltage = config.max_voltage;
    let precision = config.precision;

    let prefix = format!("fx-battery-gauge-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Calculate level (0-1)
    let level = move || {
        let v = voltage.get();
        ((v - min_voltage) / (max_voltage - min_voltage)).clamp(0.0, 1.0)
    };

    // Calculate filled bar count
    let filled_bars = move || {
        let l = level();
        (l * bar_count as f64).ceil() as usize
    };

    // Total height calculation
    let total_height = bar_count as u32 * bar_height + (bar_count as u32 - 1) * bar_gap;

    view! {
        <div
            class=combined_class
            style=format!(
                "background: var(--fx-color-bg-container, #1a1a1a); border-radius: 8px; padding: 16px; width: {}px; border: 1px solid #333;",
                width + 32
            )
        >
            // Title
            {title.clone().map(|t| {
                view! {
                    <div style="color: var(--fx-color-text-secondary, #8c8c8c); font-size: 14px; font-weight: 500; margin-bottom: 12px; text-align: center;">
                        {t}
                    </div>
                }
            })}

            // Voltage display
            <div style=move || format!(
                "font-size: 28px; font-weight: 300; color: {}; text-align: center; margin-bottom: 16px; font-family: monospace;",
                voltage_color(level())
            )>
                {move || format!("{:.prec$} {}", voltage.get(), unit, prec = precision)}
            </div>

            // Battery bars
            <div style=format!(
                "display: flex; flex-direction: column; gap: {}px; height: {}px;",
                bar_gap, total_height
            )>
                {(0..bar_count).map(|i| {
                    // Position from top (0) to bottom (1)
                    let position = i as f64 / (bar_count - 1) as f64;
                    let color = bar_color(position);
                    let bar_index = i;

                    view! {
                        <div
                            style=move || {
                                let is_filled = bar_index < filled_bars();
                                let opacity = if is_filled { 1.0 } else { 0.15 };
                                format!(
                                    "height: {}px; background: {}; border-radius: 2px; opacity: {}; transition: opacity 0.3s ease;",
                                    bar_height, color, opacity
                                )
                            }
                        />
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Compact battery indicator (smaller, inline-friendly).
#[component]
pub fn CompactBatteryIndicator(
    /// Current voltage value.
    #[prop(into)]
    voltage: Signal<f64>,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<BatteryGaugeConfig>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    let min_voltage = config.min_voltage;
    let max_voltage = config.max_voltage;
    let unit = config.unit.clone();
    let precision = config.precision;

    // Calculate level (0-1)
    let level = move || {
        let v = voltage.get();
        ((v - min_voltage) / (max_voltage - min_voltage)).clamp(0.0, 1.0)
    };

    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            // Battery icon
            <div style="position: relative; width: 32px; height: 16px; border: 2px solid #666; border-radius: 3px;">
                // Battery tip
                <div style="position: absolute; right: -5px; top: 3px; width: 3px; height: 8px; background: #666; border-radius: 0 2px 2px 0;"></div>
                // Fill level
                <div style=move || {
                    let l = level();
                    let color = if l > 0.6 { "#4ade80" } else if l > 0.3 { "#fbbf24" } else { "#f87171" };
                    format!(
                        "position: absolute; left: 1px; top: 1px; bottom: 1px; width: calc({:.0}% - 2px); background: {}; border-radius: 1px; transition: width 0.3s ease;",
                        l * 100.0, color
                    )
                }/>
            </div>

            // Voltage text
            <span style=move || format!(
                "font-size: 14px; font-family: monospace; color: {};",
                voltage_color(level())
            )>
                {move || format!("{:.prec$} {}", voltage.get(), unit, prec = precision)}
            </span>

            // Label
            {label.map(|l| {
                view! {
                    <span style="font-size: 12px; color: #8c8c8c;">{l}</span>
                }
            })}
        </div>
    }
}
