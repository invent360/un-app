//! Chart grid component.

use leptos::prelude::*;
use crate::chart::utils::generate_ticks;

/// Grid line style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GridStyle {
    /// Solid lines.
    Solid,
    /// Dashed lines.
    #[default]
    Dashed,
    /// Dotted lines.
    Dotted,
}

impl GridStyle {
    /// Get the SVG stroke-dasharray value.
    pub fn dash_array(&self) -> &'static str {
        match self {
            Self::Solid => "none",
            Self::Dashed => "4,4",
            Self::Dotted => "1,3",
        }
    }
}

/// Grid properties.
#[derive(Debug, Clone)]
pub struct GridProps {
    /// X start coordinate.
    pub x: f64,
    /// Y start coordinate.
    pub y: f64,
    /// Grid width.
    pub width: f64,
    /// Grid height.
    pub height: f64,
    /// X-axis domain.
    pub x_domain: Option<(f64, f64)>,
    /// Y-axis domain.
    pub y_domain: Option<(f64, f64)>,
    /// Number of horizontal lines.
    pub horizontal_lines: usize,
    /// Number of vertical lines.
    pub vertical_lines: usize,
    /// Show horizontal lines.
    pub show_horizontal: bool,
    /// Show vertical lines.
    pub show_vertical: bool,
    /// Grid line style.
    pub style: GridStyle,
    /// Grid line color.
    pub color: String,
    /// Grid line opacity.
    pub opacity: f32,
}

impl Default for GridProps {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            x_domain: None,
            y_domain: None,
            horizontal_lines: 5,
            vertical_lines: 6,
            show_horizontal: true,
            show_vertical: true,
            style: GridStyle::Dashed,
            color: "var(--fx-color-border, #303030)".to_string(),
            opacity: 0.5,
        }
    }
}

impl GridProps {
    /// Create new grid props for a chart area.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            ..Default::default()
        }
    }

    /// Set domains for value-aligned grid lines.
    pub fn domains(mut self, x_domain: (f64, f64), y_domain: (f64, f64)) -> Self {
        self.x_domain = Some(x_domain);
        self.y_domain = Some(y_domain);
        self
    }

    /// Set grid style.
    pub fn style(mut self, style: GridStyle) -> Self {
        self.style = style;
        self
    }

    /// Set horizontal line count.
    pub fn horizontal_lines(mut self, count: usize) -> Self {
        self.horizontal_lines = count;
        self
    }

    /// Set vertical line count.
    pub fn vertical_lines(mut self, count: usize) -> Self {
        self.vertical_lines = count;
        self
    }

    /// Only show horizontal lines.
    pub fn horizontal_only(mut self) -> Self {
        self.show_horizontal = true;
        self.show_vertical = false;
        self
    }

    /// Only show vertical lines.
    pub fn vertical_only(mut self) -> Self {
        self.show_horizontal = false;
        self.show_vertical = true;
        self
    }
}

/// Chart grid component.
#[component]
pub fn ChartGrid(
    /// Grid properties.
    props: GridProps,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let grid_class = class.unwrap_or_else(|| "fx-chart-grid".to_string());
    let dash_array = props.style.dash_array();

    // Calculate horizontal grid line positions
    let h_lines = if props.show_horizontal {
        if let Some((min, max)) = props.y_domain {
            // Use nice tick values
            generate_ticks(min, max, props.horizontal_lines)
                .into_iter()
                .map(|tick| {
                    let normalized = (tick - min) / (max - min);
                    // Y is inverted
                    props.y + props.height * (1.0 - normalized)
                })
                .collect::<Vec<_>>()
        } else {
            // Even spacing
            (0..=props.horizontal_lines)
                .map(|i| props.y + (props.height / props.horizontal_lines as f64) * i as f64)
                .collect::<Vec<_>>()
        }
    } else {
        vec![]
    };

    // Calculate vertical grid line positions
    let v_lines = if props.show_vertical {
        if let Some((min, max)) = props.x_domain {
            generate_ticks(min, max, props.vertical_lines)
                .into_iter()
                .map(|tick| {
                    let normalized = (tick - min) / (max - min);
                    props.x + props.width * normalized
                })
                .collect::<Vec<_>>()
        } else {
            (0..=props.vertical_lines)
                .map(|i| props.x + (props.width / props.vertical_lines as f64) * i as f64)
                .collect::<Vec<_>>()
        }
    } else {
        vec![]
    };

    let stroke_color = props.color.clone();
    let opacity = props.opacity;
    let x = props.x;
    let y = props.y;
    let width = props.width;
    let height = props.height;

    view! {
        <g class=grid_class opacity=opacity>
            // Horizontal lines
            {h_lines.into_iter().map(|line_y| {
                view! {
                    <line
                        x1=x
                        y1=line_y
                        x2=x + width
                        y2=line_y
                        stroke=stroke_color.clone()
                        stroke-width="1"
                        stroke-dasharray=dash_array
                    />
                }
            }).collect::<Vec<_>>()}

            // Vertical lines
            {v_lines.into_iter().map(|line_x| {
                view! {
                    <line
                        x1=line_x
                        y1=y
                        x2=line_x
                        y2=y + height
                        stroke=stroke_color.clone()
                        stroke-width="1"
                        stroke-dasharray=dash_array
                    />
                }
            }).collect::<Vec<_>>()}
        </g>
    }
}

/// Radar chart grid (circular or polygon).
#[component]
pub fn RadarGrid(
    /// Center X coordinate.
    cx: f64,
    /// Center Y coordinate.
    cy: f64,
    /// Outer radius.
    radius: f64,
    /// Number of axes (spokes).
    axes: usize,
    /// Number of concentric rings.
    rings: usize,
    /// Use polygon shape instead of circles.
    #[prop(optional)]
    polygon: bool,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let grid_class = class.unwrap_or_else(|| "fx-chart-radar-grid".to_string());
    let angle_step = 360.0 / axes as f64;

    // Generate concentric circles/polygons
    let rings_vec: Vec<f64> = (1..=rings)
        .map(|i| radius * (i as f64 / rings as f64))
        .collect();

    // Generate spoke angles
    let spokes: Vec<f64> = (0..axes)
        .map(|i| (i as f64 * angle_step - 90.0).to_radians())
        .collect();

    view! {
        <g class=grid_class>
            // Concentric rings
            {rings_vec.iter().map(|&r| {
                if polygon {
                    // Polygon path
                    let mut path = String::new();
                    for (i, &angle) in spokes.iter().enumerate() {
                        let x = cx + r * angle.cos();
                        let y = cy + r * angle.sin();
                        if i == 0 {
                            path.push_str(&format!("M{:.2},{:.2}", x, y));
                        } else {
                            path.push_str(&format!(" L{:.2},{:.2}", x, y));
                        }
                    }
                    path.push_str(" Z");
                    view! {
                        <path
                            d=path
                            fill="none"
                            stroke="var(--fx-color-border, #303030)"
                            stroke-width="1"
                            opacity="0.3"
                        />
                    }.into_any()
                } else {
                    view! {
                        <circle
                            cx=cx
                            cy=cy
                            r=r
                            fill="none"
                            stroke="var(--fx-color-border, #303030)"
                            stroke-width="1"
                            opacity="0.3"
                        />
                    }.into_any()
                }
            }).collect::<Vec<_>>()}

            // Spokes
            {spokes.iter().map(|&angle| {
                let x2 = cx + radius * angle.cos();
                let y2 = cy + radius * angle.sin();
                view! {
                    <line
                        x1=cx
                        y1=cy
                        x2=x2
                        y2=y2
                        stroke="var(--fx-color-border, #303030)"
                        stroke-width="1"
                        opacity="0.3"
                    />
                }
            }).collect::<Vec<_>>()}
        </g>
    }
}
