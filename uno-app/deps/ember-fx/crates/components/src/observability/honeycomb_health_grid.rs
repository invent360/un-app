//! HoneycombHealthGrid Leptos component.
//!
//! A hexagonal grid visualization for composite health status.

use leptos::prelude::*;
use crate::try_use_theme;

/// Health status for a hexagon cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HexHealthStatus {
    /// Healthy status (blue/green).
    #[default]
    Healthy,
    /// Warning status (yellow/orange).
    Warning,
    /// Critical status (red).
    Critical,
    /// Unknown/offline status (gray).
    Unknown,
}

impl HexHealthStatus {
    /// Get the fill color for this status.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Healthy => "#3b82f6",  // Blue
            Self::Warning => "#f59e0b",  // Amber
            Self::Critical => "#ef4444", // Red
            Self::Unknown => "#6b7280",  // Gray
        }
    }

    /// Get the stroke color for this status.
    pub fn stroke_color(&self) -> &'static str {
        match self {
            Self::Healthy => "#2563eb",
            Self::Warning => "#d97706",
            Self::Critical => "#dc2626",
            Self::Unknown => "#4b5563",
        }
    }
}

/// A single hexagon cell in the grid.
#[derive(Debug, Clone)]
pub struct HexCell {
    /// Unique identifier.
    pub id: String,
    /// Health status.
    pub status: HexHealthStatus,
    /// Optional label.
    pub label: Option<String>,
}

impl HexCell {
    /// Create a new hex cell.
    pub fn new(id: impl Into<String>, status: HexHealthStatus) -> Self {
        Self {
            id: id.into(),
            status,
            label: None,
        }
    }

    /// Create with label.
    pub fn with_label(id: impl Into<String>, status: HexHealthStatus, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status,
            label: Some(label.into()),
        }
    }
}

/// Configuration for the honeycomb grid.
#[derive(Debug, Clone)]
pub struct HoneycombConfig {
    /// Hexagon size (radius).
    pub hex_size: f64,
    /// Gap between hexagons.
    pub gap: f64,
    /// Number of columns.
    pub columns: usize,
    /// Show cell labels.
    pub show_labels: bool,
}

impl Default for HoneycombConfig {
    fn default() -> Self {
        Self {
            hex_size: 30.0,
            gap: 4.0,
            columns: 10,
            show_labels: false,
        }
    }
}

/// Generate hexagon path for a given center and size.
fn hexagon_path(cx: f64, cy: f64, size: f64) -> String {
    let mut points = Vec::new();
    for i in 0..6 {
        let angle = std::f64::consts::PI / 3.0 * i as f64 - std::f64::consts::PI / 6.0;
        let x = cx + size * angle.cos();
        let y = cy + size * angle.sin();
        points.push(format!("{:.1},{:.1}", x, y));
    }
    points.join(" ")
}

/// Calculate hexagon position in grid (offset coordinates).
fn hex_position(index: usize, columns: usize, hex_size: f64, gap: f64) -> (f64, f64) {
    let row = index / columns;
    let col = index % columns;

    let hex_width = hex_size * 2.0;
    let hex_height = hex_size * 1.732; // sqrt(3)
    let horiz_spacing = hex_width * 0.75 + gap;
    let vert_spacing = hex_height + gap;

    let x = col as f64 * horiz_spacing + hex_size + 10.0;
    let y = row as f64 * vert_spacing + hex_size + 10.0
        + if col % 2 == 1 { hex_height / 2.0 } else { 0.0 };

    (x, y)
}

/// HoneycombHealthGrid component.
///
/// Displays a grid of hexagonal cells representing health status.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{HoneycombHealthGrid, HexCell, HexHealthStatus};
///
/// let cells = vec![
///     HexCell::new("node-1", HexHealthStatus::Healthy),
///     HexCell::new("node-2", HexHealthStatus::Warning),
///     HexCell::new("node-3", HexHealthStatus::Critical),
/// ];
///
/// view! {
///     <HoneycombHealthGrid
///         title="Composite Health".to_string()
///         cells=Signal::derive(move || cells.clone())
///     />
/// }
/// ```
#[component]
pub fn HoneycombHealthGrid(
    /// Grid title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Hexagon cells.
    #[prop(into)]
    cells: Signal<Vec<HexCell>>,
    /// Click handler for cell.
    #[prop(optional, into)]
    on_cell_click: Option<Callback<String>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<HoneycombConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let hex_size = config.hex_size;
    let gap = config.gap;
    let columns = config.columns;
    let show_labels = config.show_labels;

    let prefix = format!("fx-honeycomb-{}", design_system);

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

    // Calculate SVG dimensions based on cells
    let svg_dimensions = move || {
        let cell_count = cells.get().len();
        let rows = (cell_count + columns - 1) / columns;

        let hex_width = hex_size * 2.0;
        let hex_height = hex_size * 1.732;
        let horiz_spacing = hex_width * 0.75 + gap;
        let vert_spacing = hex_height + gap;

        let width = columns as f64 * horiz_spacing + hex_size + 20.0;
        let height = rows as f64 * vert_spacing + hex_height + 20.0;

        (width, height)
    };

    view! {
        <div
            class=combined_class
            style="background: var(--fx-color-bg-container, #1f1f1f); border-radius: 8px; padding: 16px;"
        >
            // Title
            {title.clone().map(|t| {
                view! {
                    <div style="color: var(--fx-color-text, #fff); font-size: 16px; font-weight: 500; margin-bottom: 16px; text-align: center;">
                        {t}
                    </div>
                }
            })}

            // Honeycomb grid
            <svg
                width=move || format!("{:.0}", svg_dimensions().0)
                height=move || format!("{:.0}", svg_dimensions().1)
                viewBox=move || format!("0 0 {:.0} {:.0}", svg_dimensions().0, svg_dimensions().1)
            >
                {move || {
                    cells.get().into_iter().enumerate().map(|(i, cell)| {
                        let (cx, cy) = hex_position(i, columns, hex_size, gap);
                        let points = hexagon_path(cx, cy, hex_size);
                        let fill = cell.status.color();
                        let stroke = cell.status.stroke_color();
                        let cell_id = cell.id.clone();

                        let handle_click = {
                            let on_click = on_cell_click.clone();
                            let id = cell_id.clone();
                            move |_| {
                                if let Some(ref cb) = on_click {
                                    cb.run(id.clone());
                                }
                            }
                        };

                        view! {
                            <g
                                style="cursor: pointer;"
                                on:click=handle_click
                            >
                                <polygon
                                    points=points
                                    fill=fill
                                    stroke=stroke
                                    stroke-width="2"
                                />
                                {show_labels.then(|| {
                                    cell.label.clone().map(|label| {
                                        view! {
                                            <text
                                                x=format!("{:.1}", cx)
                                                y=format!("{:.1}", cy)
                                                fill="#fff"
                                                font-size="10"
                                                text-anchor="middle"
                                                dominant-baseline="middle"
                                            >
                                                {label}
                                            </text>
                                        }
                                    })
                                })}
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>
        </div>
    }
}
