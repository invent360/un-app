//! Axis components for charts.

use leptos::prelude::*;
use crate::chart::utils::{generate_ticks, format_tick_label, TickFormat, format_time_label, TimeFormat};

/// Axis orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AxisOrientation {
    /// X-axis (horizontal).
    #[default]
    Bottom,
    /// X-axis at top.
    Top,
    /// Y-axis (vertical).
    Left,
    /// Y-axis at right.
    Right,
}

/// Axis props shared between X and Y axis.
#[derive(Debug, Clone)]
pub struct AxisProps {
    /// Axis position (x or y coordinate).
    pub position: f64,
    /// Start coordinate.
    pub start: f64,
    /// End coordinate.
    pub end: f64,
    /// Domain min value.
    pub domain_min: f64,
    /// Domain max value.
    pub domain_max: f64,
    /// Number of ticks.
    pub tick_count: usize,
    /// Tick format.
    pub format: TickFormat,
    /// Show axis line.
    pub show_line: bool,
    /// Show tick marks.
    pub show_ticks: bool,
    /// Show labels.
    pub show_labels: bool,
    /// Axis label.
    pub label: Option<String>,
}

impl Default for AxisProps {
    fn default() -> Self {
        Self {
            position: 0.0,
            start: 0.0,
            end: 100.0,
            domain_min: 0.0,
            domain_max: 100.0,
            tick_count: 5,
            format: TickFormat::Number,
            show_line: true,
            show_ticks: true,
            show_labels: true,
            label: None,
        }
    }
}

impl AxisProps {
    /// Create new axis props.
    pub fn new(position: f64, start: f64, end: f64, domain_min: f64, domain_max: f64) -> Self {
        Self {
            position,
            start,
            end,
            domain_min,
            domain_max,
            ..Default::default()
        }
    }

    /// Set tick count.
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self
    }

    /// Set format.
    pub fn format(mut self, format: TickFormat) -> Self {
        self.format = format;
        self
    }

    /// Set label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// X-Axis component (horizontal).
#[component]
pub fn XAxis(
    /// Axis properties.
    props: AxisProps,
    /// Is this a time axis.
    #[prop(optional)]
    time_axis: bool,
    /// Time format for labels.
    #[prop(optional)]
    time_format: Option<TimeFormat>,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let ticks = generate_ticks(props.domain_min, props.domain_max, props.tick_count);
    let range = props.end - props.start;
    let domain_range = props.domain_max - props.domain_min;

    let tick_length = 6.0;
    let label_offset = 20.0;

    let axis_class = class.unwrap_or_else(|| "fx-chart-x-axis".to_string());

    view! {
        <g class=axis_class>
            // Axis line
            {props.show_line.then(|| view! {
                <line
                    x1=props.start
                    y1=props.position
                    x2=props.end
                    y2=props.position
                    stroke="var(--fx-color-text-tertiary, #595959)"
                    stroke-width="1"
                />
            })}

            // Ticks and labels
            {ticks.into_iter().map(|tick| {
                let normalized = if domain_range.abs() > f64::EPSILON {
                    (tick - props.domain_min) / domain_range
                } else {
                    0.5
                };
                let x = props.start + normalized * range;

                let label = if time_axis {
                    format_time_label(tick as i64, time_format.unwrap_or_default())
                } else {
                    format_tick_label(tick, props.format)
                };

                view! {
                    <g class="fx-chart-tick">
                        // Tick mark
                        {props.show_ticks.then(|| view! {
                            <line
                                x1=x
                                y1=props.position
                                x2=x
                                y2=props.position + tick_length
                                stroke="var(--fx-color-text-tertiary, #595959)"
                                stroke-width="1"
                            />
                        })}

                        // Label
                        {props.show_labels.then(|| view! {
                            <text
                                x=x
                                y=props.position + label_offset
                                fill="var(--fx-color-text-secondary, #8c8c8c)"
                                font-size="11"
                                text-anchor="middle"
                            >
                                {label}
                            </text>
                        })}
                    </g>
                }
            }).collect::<Vec<_>>()}

            // Axis label
            {props.label.map(|label| {
                let center_x = props.start + range / 2.0;
                view! {
                    <text
                        x=center_x
                        y=props.position + 35.0
                        fill="var(--fx-color-text-secondary, #8c8c8c)"
                        font-size="12"
                        text-anchor="middle"
                    >
                        {label}
                    </text>
                }
            })}
        </g>
    }
}

/// Y-Axis component (vertical).
#[component]
pub fn YAxis(
    /// Axis properties.
    props: AxisProps,
    /// Position axis on the right side.
    #[prop(optional)]
    right: bool,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let ticks = generate_ticks(props.domain_min, props.domain_max, props.tick_count);
    let range = props.end - props.start;
    let domain_range = props.domain_max - props.domain_min;

    let tick_length = 6.0;
    let label_offset = if right { 12.0 } else { -12.0 };

    let axis_class = class.unwrap_or_else(|| "fx-chart-y-axis".to_string());

    view! {
        <g class=axis_class>
            // Axis line
            {props.show_line.then(|| view! {
                <line
                    x1=props.position
                    y1=props.start
                    x2=props.position
                    y2=props.end
                    stroke="var(--fx-color-text-tertiary, #595959)"
                    stroke-width="1"
                />
            })}

            // Ticks and labels
            {ticks.into_iter().map(|tick| {
                let normalized = if domain_range.abs() > f64::EPSILON {
                    (tick - props.domain_min) / domain_range
                } else {
                    0.5
                };
                // Y-axis is inverted (higher values at top)
                let y = props.end - normalized * range;

                let label = format_tick_label(tick, props.format);

                let tick_x1 = if right { props.position } else { props.position - tick_length };
                let tick_x2 = if right { props.position + tick_length } else { props.position };
                let text_anchor = if right { "start" } else { "end" };

                view! {
                    <g class="fx-chart-tick">
                        // Tick mark
                        {props.show_ticks.then(|| view! {
                            <line
                                x1=tick_x1
                                y1=y
                                x2=tick_x2
                                y2=y
                                stroke="var(--fx-color-text-tertiary, #595959)"
                                stroke-width="1"
                            />
                        })}

                        // Label
                        {props.show_labels.then(|| view! {
                            <text
                                x=props.position + label_offset
                                y=y
                                fill="var(--fx-color-text-secondary, #8c8c8c)"
                                font-size="11"
                                text-anchor=text_anchor
                                dominant-baseline="middle"
                            >
                                {label}
                            </text>
                        })}
                    </g>
                }
            }).collect::<Vec<_>>()}

            // Axis label (rotated)
            {props.label.map(|label| {
                let center_y = props.start + range / 2.0;
                let label_x = if right { props.position + 45.0 } else { props.position - 35.0 };
                let rotation = if right { 90.0 } else { -90.0 };
                view! {
                    <text
                        x=label_x
                        y=center_y
                        fill="var(--fx-color-text-secondary, #8c8c8c)"
                        font-size="12"
                        text-anchor="middle"
                        transform=format!("rotate({}, {}, {})", rotation, label_x, center_y)
                    >
                        {label}
                    </text>
                }
            })}
        </g>
    }
}

/// Categorical X-Axis for bar/column charts.
#[component]
pub fn CategoryAxis(
    /// Categories to display.
    categories: Vec<String>,
    /// Y position of axis.
    y: f64,
    /// Start X coordinate.
    start: f64,
    /// End X coordinate.
    end: f64,
    /// Show tick marks.
    #[prop(optional)]
    show_ticks: bool,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let range = end - start;
    let count = categories.len();
    let step = if count > 0 { range / count as f64 } else { range };

    let axis_class = class.unwrap_or_else(|| "fx-chart-category-axis".to_string());

    view! {
        <g class=axis_class>
            // Axis line
            <line
                x1=start
                y1=y
                x2=end
                y2=y
                stroke="var(--fx-color-text-tertiary, #595959)"
                stroke-width="1"
            />

            // Category labels
            {categories.into_iter().enumerate().map(|(i, label)| {
                let x = start + step * (i as f64 + 0.5);
                view! {
                    <g class="fx-chart-tick">
                        {show_ticks.then(|| view! {
                            <line
                                x1=x
                                y1=y
                                x2=x
                                y2=y + 6.0
                                stroke="var(--fx-color-text-tertiary, #595959)"
                                stroke-width="1"
                            />
                        })}
                        <text
                            x=x
                            y=y + 18.0
                            fill="var(--fx-color-text-secondary, #8c8c8c)"
                            font-size="11"
                            text-anchor="middle"
                        >
                            {label}
                        </text>
                    </g>
                }
            }).collect::<Vec<_>>()}
        </g>
    }
}
