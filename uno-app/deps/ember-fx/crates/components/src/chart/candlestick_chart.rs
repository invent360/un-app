use leptos::prelude::*;

use super::types::{ColorPalette, CandlestickChartConfig, OHLCPoint};
use super::utils::{chart_prefix, try_use_theme, format_time_label, TimeFormat};

/// Rendered candle data for display
#[derive(Clone, Debug, PartialEq)]
struct CandleRenderData {
    index: usize,
    x: f64,
    y_high: f64,
    y_low: f64,
    y_body_top: f64,
    y_body_bottom: f64,
    color: String,
    is_bullish: bool,
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: Option<f64>,
}

/// Chart layout data
#[derive(Clone, Debug, PartialEq)]
struct ChartLayoutData {
    candlesticks: Vec<CandleRenderData>,
    candle_width: f64,
    y_labels: Vec<(f64, String)>,
    x_labels: Vec<(f64, String)>,
}

impl Default for ChartLayoutData {
    fn default() -> Self {
        Self {
            candlesticks: Vec::new(),
            candle_width: 0.0,
            y_labels: Vec::new(),
            x_labels: Vec::new(),
        }
    }
}

/// Tooltip data for candlestick hover
#[derive(Clone, Debug)]
struct CandleTooltipData {
    /// Index of the candle
    index: usize,
    /// Unix timestamp
    timestamp: i64,
    /// Open price
    open: f64,
    /// High price
    high: f64,
    /// Low price
    low: f64,
    /// Close price
    close: f64,
    /// Optional volume
    volume: Option<f64>,
    /// Whether bullish (green) or bearish (red)
    is_bullish: bool,
    /// Candle color
    color: String,
    /// Mouse X position for crosshair
    mouse_x: f64,
    /// Mouse Y position for crosshair
    mouse_y: f64,
}

/// Format a price value as currency
fn format_currency(value: f64) -> String {
    if value >= 1000.0 {
        format!("${:.2}", value)
    } else if value >= 1.0 {
        format!("${:.4}", value)
    } else {
        format!("${:.6}", value)
    }
}

/// Format volume with compact notation
fn format_volume(value: f64) -> String {
    if value >= 1_000_000_000.0 {
        format!("{:.2}B", value / 1_000_000_000.0)
    } else if value >= 1_000_000.0 {
        format!("{:.2}M", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.2}K", value / 1_000.0)
    } else {
        format!("{:.0}", value)
    }
}

/// OHLC Candlestick chart component with full interactivity
///
/// Interactive financial candlestick chart with hover effects, crosshairs, and tooltips.
///
/// # Props
///
/// - `data` - OHLC data points
/// - `config` - Chart configuration
/// - `on_candle_click` - Click handler (candle index)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{CandlestickChart, OHLCPoint};
///
/// let data = vec![
///     OHLCPoint::new(1700000000000, 100.0, 105.0, 98.0, 103.0),
///     OHLCPoint::new(1700086400000, 103.0, 110.0, 102.0, 108.0),
/// ];
///
/// view! {
///     <CandlestickChart data=Signal::derive(move || data.clone()) />
/// }
/// ```
#[component]
pub fn CandlestickChart(
    /// OHLC data points
    #[prop(into)]
    data: Signal<Vec<OHLCPoint>>,
    /// Configuration for the candlestick chart
    #[prop(optional)]
    config: Option<CandlestickChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler (candle index)
    #[prop(optional, into)]
    on_candle_click: Option<Callback<usize>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let chart_prefix_str = chart_prefix("candlestick");
    let prefix_class = chart_prefix_str.clone();
    let prefix_svg = format!("{}-svg", chart_prefix_str);
    let prefix_candle = format!("{}-candle", chart_prefix_str);
    let prefix_loading = format!("{}-loading", chart_prefix_str);
    let prefix_spinner = format!("{}-spinner", chart_prefix_str);

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let _colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Interactive state
    let hovered_candle = RwSignal::new(None::<usize>);
    let tooltip_data = RwSignal::new(None::<CandleTooltipData>);
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    // Chart dimensions
    let width = 600.0;
    let height = 400.0;
    let padding = 60.0;
    let chart_width = width - padding * 2.0;
    let chart_height = height - padding * 2.0;

    // Store config values for closures
    let up_color = config.up_color.as_css().to_string();
    let down_color = config.down_color.as_css().to_string();
    let show_crosshair = config.show_crosshair;

    // Calculate scales and render candlesticks
    let chart_data = Memo::new(move |_| {
        let points = data.get();

        if points.is_empty() {
            return ChartLayoutData::default();
        }

        // Find min/max for y-axis
        let mut min_price = f64::MAX;
        let mut max_price = f64::MIN;
        for point in &points {
            min_price = min_price.min(point.low);
            max_price = max_price.max(point.high);
        }

        // Add padding to price range
        let price_range = max_price - min_price;
        let price_padding = price_range * 0.1;
        min_price -= price_padding;
        max_price += price_padding;

        let candle_width = (chart_width / points.len() as f64) * 0.8;
        let gap = (chart_width / points.len() as f64) * 0.2;

        let candlesticks: Vec<CandleRenderData> = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let x = padding + (i as f64 * (candle_width + gap)) + gap / 2.0;
                let is_bullish = point.close >= point.open;

                let body_top = if is_bullish { point.close } else { point.open };
                let body_bottom = if is_bullish { point.open } else { point.close };

                let y_high = padding + chart_height * (1.0 - (point.high - min_price) / (max_price - min_price));
                let y_low = padding + chart_height * (1.0 - (point.low - min_price) / (max_price - min_price));
                let y_body_top = padding + chart_height * (1.0 - (body_top - min_price) / (max_price - min_price));
                let y_body_bottom = padding + chart_height * (1.0 - (body_bottom - min_price) / (max_price - min_price));

                let color = if is_bullish { up_color.clone() } else { down_color.clone() };

                CandleRenderData {
                    index: i,
                    x,
                    y_high,
                    y_low,
                    y_body_top,
                    y_body_bottom,
                    color,
                    is_bullish,
                    timestamp: point.timestamp,
                    open: point.open,
                    high: point.high,
                    low: point.low,
                    close: point.close,
                    volume: point.volume,
                }
            })
            .collect();

        // Y-axis labels
        let y_labels: Vec<(f64, String)> = (0..=5)
            .map(|i| {
                let value = min_price + (max_price - min_price) * (i as f64 / 5.0);
                let y = padding + chart_height * (1.0 - i as f64 / 5.0);
                (y, format_currency(value))
            })
            .collect();

        // X-axis labels (show a subset of timestamps)
        let x_labels: Vec<(f64, String)> = points
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                let step = (points.len() / 5).max(1);
                i % step == 0 || *i == points.len() - 1
            })
            .map(|(i, point)| {
                let x = padding + (i as f64 * (candle_width + gap)) + gap / 2.0 + candle_width / 2.0;
                (x, format_time_label(point.timestamp, TimeFormat::Date))
            })
            .collect();

        ChartLayoutData {
            candlesticks,
            candle_width,
            y_labels,
            x_labels,
        }
    });

    let combined_class = {
        let mut parts = vec![prefix_class.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix_class));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg viewBox=format!("0 0 {} {}", width, height) class=prefix_svg>
                // Definitions for glow effect
                <defs>
                    <filter id="candleGlow" x="-50%" y="-50%" width="200%" height="200%">
                        <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>
                </defs>

                // Y-axis
                <line
                    x1=padding
                    y1=padding
                    x2=padding
                    y2=height - padding
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // X-axis
                <line
                    x1=padding
                    y1=height - padding
                    x2=width - padding
                    y2=height - padding
                    stroke=move || if theme.is_some() { "var(--chart-axis)" } else { "#9ca3af" }
                    stroke-width="1"
                />

                // Grid lines and Y-axis labels
                {move || {
                    let layout = chart_data.get();
                    layout.y_labels.into_iter().map(|(y, label)| {
                        view! {
                            <g>
                                <line
                                    x1=padding
                                    y1=y
                                    x2=width - padding
                                    y2=y
                                    stroke=move || if theme.is_some() { "var(--chart-grid)" } else { "#e5e7eb" }
                                    stroke-width="1"
                                    stroke-dasharray="4,4"
                                />
                                <text
                                    x=padding - 10.0
                                    y=y
                                    text-anchor="end"
                                    dominant-baseline="middle"
                                    fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                    font-size="11"
                                >
                                    {label}
                                </text>
                            </g>
                        }
                    }).collect_view()
                }}

                // X-axis labels
                {move || {
                    let layout = chart_data.get();
                    layout.x_labels.into_iter().map(|(x, label)| {
                        view! {
                            <text
                                x=x
                                y=height - padding + 20.0
                                text-anchor="middle"
                                fill=move || if theme.is_some() { "var(--chart-text)" } else { "#6b7280" }
                                font-size="10"
                            >
                                {label}
                            </text>
                        }
                    }).collect_view()
                }}

                // Crosshair lines (rendered behind candles but above grid)
                {
                    let show_crosshair = show_crosshair;
                    move || {
                        if !show_crosshair {
                            return None;
                        }
                        tooltip_data.get().map(|data| {
                            view! {
                                <g class="crosshair">
                                    // Vertical line
                                    <line
                                        x1=data.mouse_x
                                        y1=padding
                                        x2=data.mouse_x
                                        y2=height - padding
                                        stroke="var(--fx-color-text-tertiary, #595959)"
                                        stroke-width="1"
                                        stroke-dasharray="4,2"
                                        style="pointer-events: none;"
                                    />
                                    // Horizontal line
                                    <line
                                        x1=padding
                                        y1=data.mouse_y
                                        x2=width - padding
                                        y2=data.mouse_y
                                        stroke="var(--fx-color-text-tertiary, #595959)"
                                        stroke-width="1"
                                        stroke-dasharray="4,2"
                                        style="pointer-events: none;"
                                    />
                                </g>
                            }
                        })
                    }
                }

                // Candlesticks with interactivity
                {
                    let candle_class = prefix_candle.clone();
                    let on_click = on_candle_click.clone();

                    move || {
                        let layout = chart_data.get();
                        let candle_width = layout.candle_width;
                        let candle_class = candle_class.clone();
                        let on_click = on_click.clone();

                        layout.candlesticks.into_iter().map({
                            let candle_class = candle_class.clone();
                            let on_click = on_click.clone();

                            move |candle| {
                                let idx = candle.index;
                                let x = candle.x;
                                let y_high = candle.y_high;
                                let y_low = candle.y_low;
                                let y_body_top = candle.y_body_top;
                                let y_body_bottom = candle.y_body_bottom;
                                let color = candle.color.clone();
                                let is_bullish = candle.is_bullish;
                                let timestamp = candle.timestamp;
                                let open = candle.open;
                                let high = candle.high;
                                let low = candle.low;
                                let close = candle.close;
                                let volume = candle.volume;

                                let wick_x = x + candle_width / 2.0;
                                let body_height = (y_body_bottom - y_body_top).max(1.0);
                                let on_click = on_click.clone();
                                let color_for_tooltip = color.clone();

                                // Calculate center Y for crosshair positioning
                                let center_y = (y_body_top + y_body_bottom) / 2.0;

                                // Check if this candle is hovered
                                let is_hovered = move || hovered_candle.get() == Some(idx);

                                // Opacity based on hover state
                                let opacity = move || {
                                    match hovered_candle.get() {
                                        Some(h) if h != idx => 0.5,
                                        _ => 1.0,
                                    }
                                };

                                // Transform for hover effect
                                let transform = move || {
                                    if hovered_candle.get() == Some(idx) {
                                        "scale(1.05)".to_string()
                                    } else {
                                        String::new()
                                    }
                                };

                                // Filter for glow effect
                                let filter = move || {
                                    if hovered_candle.get() == Some(idx) {
                                        "url(#candleGlow)"
                                    } else {
                                        "none"
                                    }
                                };

                                // Stroke width based on hover
                                let stroke_width = move || {
                                    if hovered_candle.get() == Some(idx) { "2" } else { "1" }
                                };

                                view! {
                                    <g
                                        class=candle_class.clone()
                                        style=move || format!(
                                            "transform-origin: {}px {}px; transform: {}; opacity: {}; transition: all 0.15s ease; cursor: pointer;",
                                            wick_x, center_y, transform(), opacity()
                                        )
                                        filter=filter
                                        on:mouseenter=move |ev| {
                                            hovered_candle.set(Some(idx));
                                            tooltip_data.set(Some(CandleTooltipData {
                                                index: idx,
                                                timestamp,
                                                open,
                                                high,
                                                low,
                                                close,
                                                volume,
                                                is_bullish,
                                                color: color_for_tooltip.clone(),
                                                mouse_x: wick_x,
                                                mouse_y: center_y,
                                            }));
                                            // Get mouse position relative to container
                                            if let Some(container) = container_ref.get() {
                                                let rect = container.get_bounding_client_rect();
                                                let mx = ev.client_x() as f64 - rect.left();
                                                let my = ev.client_y() as f64 - rect.top();
                                                mouse_pos.set((mx, my));
                                            }
                                        }
                                        on:mousemove=move |ev| {
                                            // Update mouse position on move for smooth tracking
                                            if let Some(container) = container_ref.get() {
                                                let rect = container.get_bounding_client_rect();
                                                let mx = ev.client_x() as f64 - rect.left();
                                                let my = ev.client_y() as f64 - rect.top();
                                                mouse_pos.set((mx, my));
                                            }
                                        }
                                        on:mouseleave=move |_| {
                                            hovered_candle.set(None);
                                            tooltip_data.set(None);
                                        }
                                        on:click=move |_| {
                                            if let Some(ref callback) = on_click {
                                                callback.run(idx);
                                            }
                                        }
                                    >
                                        // Wick (high to low)
                                        <line
                                            x1=wick_x
                                            y1=y_high
                                            x2=wick_x
                                            y2=y_low
                                            stroke=color.clone()
                                            stroke-width=stroke_width
                                        />

                                        // Body
                                        <rect
                                            x=x
                                            y=y_body_top
                                            width=candle_width
                                            height=body_height
                                            fill=if is_bullish { "transparent".to_string() } else { color.clone() }
                                            stroke=color.clone()
                                            stroke-width=stroke_width
                                        />
                                    </g>
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Tooltip
            {
                let chart_prefix_tooltip = format!("{}-tooltip", chart_prefix_str);
                move || {
                    tooltip_data.get().map(|data| {
                        let change = data.close - data.open;
                        let change_pct = if data.open != 0.0 {
                            (change / data.open) * 100.0
                        } else {
                            0.0
                        };
                        let change_color = if change >= 0.0 {
                            "var(--fx-color-success, #52c41a)"
                        } else {
                            "var(--fx-color-error, #ff4d4f)"
                        };
                        let change_sign = if change >= 0.0 { "+" } else { "" };

                        // Use actual mouse position for tooltip placement
                        let (mx, my) = mouse_pos.get();
                        let tooltip_x = mx + 15.0;
                        let tooltip_y = my;

                        view! {
                            <div
                                class=chart_prefix_tooltip.clone()
                                style=format!(
                                    "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 8px; padding: 12px 16px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 4px 12px rgba(0,0,0,0.4); min-width: 180px; transform: translateY(-50%);",
                                    tooltip_x, tooltip_y
                                )
                            >
                                // Header with date and change
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; padding-bottom: 8px; border-bottom: 1px solid var(--fx-color-border, #303030);">
                                    <span style="color: var(--fx-color-text, #fff); font-weight: 600;">
                                        {format_time_label(data.timestamp, TimeFormat::DateTime)}
                                    </span>
                                    <span style=format!("color: {}; font-weight: 500;", change_color)>
                                        {format!("{}{:.2}%", change_sign, change_pct)}
                                    </span>
                                </div>

                                // OHLC Grid
                                <div style="display: grid; grid-template-columns: auto 1fr; gap: 4px 12px;">
                                    <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"Open"</span>
                                    <span style="color: var(--fx-color-text, #fff); text-align: right; font-family: monospace;">
                                        {format_currency(data.open)}
                                    </span>

                                    <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"High"</span>
                                    <span style="color: var(--fx-color-success, #52c41a); text-align: right; font-family: monospace;">
                                        {format_currency(data.high)}
                                    </span>

                                    <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"Low"</span>
                                    <span style="color: var(--fx-color-error, #ff4d4f); text-align: right; font-family: monospace;">
                                        {format_currency(data.low)}
                                    </span>

                                    <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"Close"</span>
                                    <span style=format!("color: {}; text-align: right; font-family: monospace;", change_color)>
                                        {format_currency(data.close)}
                                    </span>
                                </div>

                                // Volume (if available)
                                {data.volume.map(|vol| {
                                    view! {
                                        <div style="margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--fx-color-border, #303030); display: flex; justify-content: space-between;">
                                            <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"Volume"</span>
                                            <span style="color: var(--fx-color-text-secondary, #a6a6a6); font-family: monospace;">
                                                {format_volume(vol)}
                                            </span>
                                        </div>
                                    }
                                })}

                                // Change amount
                                <div style="margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--fx-color-border, #303030); display: flex; justify-content: space-between;">
                                    <span style="color: var(--fx-color-text-tertiary, #8c8c8c);">"Change"</span>
                                    <span style=format!("color: {}; font-family: monospace;", change_color)>
                                        {format!("{}{}", change_sign, format_currency(change.abs()))}
                                    </span>
                                </div>
                            </div>
                        }
                    })
                }
            }

            // Loading overlay
            {
                let loading_class = prefix_loading.clone();
                let spinner_class = prefix_spinner.clone();
                move || {
                    if loading {
                        Some(view! {
                            <div class=loading_class.clone()>
                                <div class=spinner_class.clone()></div>
                            </div>
                        })
                    } else {
                        None
                    }
                }
            }
        </div>
    }
}
