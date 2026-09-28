//! Chart tooltip component.

use leptos::prelude::*;

/// Tooltip data row.
#[derive(Debug, Clone)]
pub struct TooltipRow {
    /// Series/item name.
    pub name: String,
    /// Value to display.
    pub value: String,
    /// Item color.
    pub color: Option<String>,
}

impl TooltipRow {
    /// Create a new tooltip row.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            color: None,
        }
    }

    /// Set color.
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Tooltip props.
#[derive(Debug, Clone)]
pub struct TooltipProps {
    /// Tooltip title (e.g., timestamp).
    pub title: Option<String>,
    /// Data rows.
    pub rows: Vec<TooltipRow>,
    /// X position.
    pub x: f64,
    /// Y position.
    pub y: f64,
    /// Is visible.
    pub visible: bool,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            title: None,
            rows: vec![],
            x: 0.0,
            y: 0.0,
            visible: false,
        }
    }
}

impl TooltipProps {
    /// Create new tooltip props.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            visible: true,
            ..Default::default()
        }
    }

    /// Set title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a row.
    pub fn row(mut self, row: TooltipRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Add rows.
    pub fn rows(mut self, rows: Vec<TooltipRow>) -> Self {
        self.rows = rows;
        self
    }

    /// Set visibility.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

/// Chart tooltip component.
#[component]
pub fn ChartTooltip(
    /// Tooltip data signal.
    data: Signal<TooltipProps>,
    /// Offset from cursor X.
    #[prop(optional)]
    offset_x: Option<f64>,
    /// Offset from cursor Y.
    #[prop(optional)]
    offset_y: Option<f64>,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let offset_x = offset_x.unwrap_or(10.0);
    let offset_y = offset_y.unwrap_or(10.0);
    let base_class = class.unwrap_or_else(|| "fx-chart-tooltip".to_string());

    view! {
        {move || {
            let props = data.get();
            if !props.visible {
                return view! {}.into_any();
            }

            let left = props.x + offset_x;
            let top = props.y + offset_y;

            view! {
                <div
                    class=base_class.clone()
                    style=format!(
                        "position: absolute; left: {}px; top: {}px; z-index: 1000; \
                        background: var(--fx-color-bg-elevated, #1f1f1f); \
                        border: 1px solid var(--fx-color-border, #303030); \
                        border-radius: 6px; padding: 8px 12px; \
                        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3); \
                        font-size: 12px; pointer-events: none; \
                        min-width: 120px;",
                        left, top
                    )
                >
                    // Title
                    {props.title.map(|title| view! {
                        <div
                            style="color: var(--fx-color-text-secondary, #8c8c8c); \
                                   margin-bottom: 6px; font-size: 11px; \
                                   border-bottom: 1px solid var(--fx-color-border, #303030); \
                                   padding-bottom: 4px;"
                        >
                            {title}
                        </div>
                    })}

                    // Rows
                    {props.rows.iter().map(|row| {
                        view! {
                            <div
                                style="display: flex; justify-content: space-between; \
                                       align-items: center; padding: 2px 0; gap: 16px;"
                            >
                                <span style="display: flex; align-items: center; gap: 6px; \
                                            color: var(--fx-color-text-secondary, #8c8c8c);">
                                    {row.color.as_ref().map(|c| view! {
                                        <span style=format!(
                                            "width: 8px; height: 8px; border-radius: 50%; background: {};",
                                            c
                                        ) />
                                    })}
                                    {row.name.clone()}
                                </span>
                                <span style="color: var(--fx-color-text, #ffffff); font-weight: 500;">
                                    {row.value.clone()}
                                </span>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }.into_any()
        }}
    }
}

/// Simple tooltip for single value display.
#[component]
pub fn SimpleTooltip(
    /// X position.
    x: f64,
    /// Y position.
    y: f64,
    /// Label text.
    label: String,
    /// Value text.
    value: String,
    /// Is visible.
    #[prop(optional)]
    visible: bool,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_class = class.unwrap_or_else(|| "fx-chart-simple-tooltip".to_string());

    let display = if visible { "block" } else { "none" };

    view! {
        <div
            class=base_class
            style=format!(
                "display: {}; position: absolute; left: {}px; top: {}px; \
                transform: translate(-50%, -100%) translateY(-8px); \
                background: var(--fx-color-bg-elevated, #1f1f1f); \
                border: 1px solid var(--fx-color-border, #303030); \
                border-radius: 4px; padding: 4px 8px; \
                font-size: 11px; white-space: nowrap; \
                pointer-events: none; z-index: 1000;",
                display, x, y
            )
        >
            <span style="color: var(--fx-color-text-secondary, #8c8c8c); margin-right: 4px;">
                {label}
            </span>
            <span style="color: var(--fx-color-text, #ffffff); font-weight: 500;">
                {value}
            </span>
        </div>
    }
}

/// Crosshair component for hover tracking.
#[component]
pub fn Crosshair(
    /// X position (for vertical line).
    #[prop(optional)]
    x: Option<f64>,
    /// Y position (for horizontal line).
    #[prop(optional)]
    y: Option<f64>,
    /// Chart area bounds.
    bounds: (f64, f64, f64, f64), // (x, y, width, height)
    /// Show vertical line.
    #[prop(optional)]
    show_vertical: bool,
    /// Show horizontal line.
    #[prop(optional)]
    show_horizontal: bool,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let (bx, by, bw, bh) = bounds;
    let base_class = class.unwrap_or_else(|| "fx-chart-crosshair".to_string());

    view! {
        <g class=base_class>
            // Vertical line
            {(show_vertical && x.is_some()).then(|| {
                let x_pos = x.unwrap();
                view! {
                    <line
                        x1=x_pos
                        y1=by
                        x2=x_pos
                        y2=by + bh
                        stroke="var(--fx-color-text-tertiary, #595959)"
                        stroke-width="1"
                        stroke-dasharray="4,4"
                        opacity="0.5"
                    />
                }
            })}

            // Horizontal line
            {(show_horizontal && y.is_some()).then(|| {
                let y_pos = y.unwrap();
                view! {
                    <line
                        x1=bx
                        y1=y_pos
                        x2=bx + bw
                        y2=y_pos
                        stroke="var(--fx-color-text-tertiary, #595959)"
                        stroke-width="1"
                        stroke-dasharray="4,4"
                        opacity="0.5"
                    />
                }
            })}
        </g>
    }
}
