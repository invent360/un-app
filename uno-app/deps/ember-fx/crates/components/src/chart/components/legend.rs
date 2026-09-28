//! Chart legend component.

use leptos::prelude::*;
use crate::chart::utils::LegendPosition;

/// Legend item data.
#[derive(Debug, Clone)]
pub struct LegendItemData {
    /// Item label.
    pub label: String,
    /// Item color (CSS).
    pub color: String,
    /// Is item active/visible.
    pub active: bool,
    /// Item shape.
    pub shape: LegendShape,
}

impl LegendItemData {
    /// Create a new legend item.
    pub fn new(label: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: color.into(),
            active: true,
            shape: LegendShape::Circle,
        }
    }

    /// Set active state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set shape.
    pub fn shape(mut self, shape: LegendShape) -> Self {
        self.shape = shape;
        self
    }
}

/// Legend marker shape.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegendShape {
    /// Circle marker.
    #[default]
    Circle,
    /// Square marker.
    Square,
    /// Line marker (for line charts).
    Line,
    /// Diamond marker.
    Diamond,
}

/// Legend props.
#[derive(Debug, Clone)]
pub struct LegendProps {
    /// Legend items.
    pub items: Vec<LegendItemData>,
    /// Legend position.
    pub position: LegendPosition,
    /// Interactive (clickable).
    pub interactive: bool,
    /// Item spacing.
    pub spacing: f64,
    /// Marker size.
    pub marker_size: f64,
}

impl Default for LegendProps {
    fn default() -> Self {
        Self {
            items: vec![],
            position: LegendPosition::Bottom,
            interactive: true,
            spacing: 20.0,
            marker_size: 12.0,
        }
    }
}

impl LegendProps {
    /// Create legend props with items.
    pub fn new(items: Vec<LegendItemData>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    /// Set position.
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set interactive mode.
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

/// Chart legend component.
#[component]
pub fn ChartLegend(
    /// Legend items.
    items: Signal<Vec<LegendItemData>>,
    /// Legend position.
    #[prop(optional)]
    position: LegendPosition,
    /// Interactive (clickable to toggle visibility).
    #[prop(optional)]
    interactive: bool,
    /// Toggle callback (receives item index).
    #[prop(optional, into)]
    on_toggle: Option<Callback<usize>>,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_class = "fx-chart-legend";
    let position_class = match position {
        LegendPosition::Top => "fx-chart-legend-top",
        LegendPosition::Right => "fx-chart-legend-right",
        LegendPosition::Bottom => "fx-chart-legend-bottom",
        LegendPosition::Left => "fx-chart-legend-left",
    };

    let combined_class = {
        let mut parts = vec![base_class.to_string(), position_class.to_string()];
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    let is_vertical = matches!(position, LegendPosition::Left | LegendPosition::Right);
    let flex_direction = if is_vertical { "column" } else { "row" };

    view! {
        <div
            class=combined_class
            style=format!(
                "display: flex; flex-direction: {}; flex-wrap: wrap; gap: 12px; padding: 8px; align-items: center; justify-content: center;",
                flex_direction
            )
        >
            {move || items.get().into_iter().enumerate().map(|(idx, item)| {
                let on_toggle = on_toggle.clone();
                let is_interactive = interactive;
                let opacity = if item.active { "1" } else { "0.4" };
                let cursor = if is_interactive { "pointer" } else { "default" };

                let marker_view = match item.shape {
                    LegendShape::Circle => view! {
                        <span
                            style=format!(
                                "display: inline-block; width: 10px; height: 10px; border-radius: 50%; background: {}; margin-right: 6px;",
                                item.color
                            )
                        />
                    }.into_any(),
                    LegendShape::Square => view! {
                        <span
                            style=format!(
                                "display: inline-block; width: 10px; height: 10px; border-radius: 2px; background: {}; margin-right: 6px;",
                                item.color
                            )
                        />
                    }.into_any(),
                    LegendShape::Line => view! {
                        <span
                            style=format!(
                                "display: inline-block; width: 16px; height: 3px; border-radius: 1px; background: {}; margin-right: 6px;",
                                item.color
                            )
                        />
                    }.into_any(),
                    LegendShape::Diamond => view! {
                        <span
                            style=format!(
                                "display: inline-block; width: 10px; height: 10px; background: {}; margin-right: 6px; transform: rotate(45deg);",
                                item.color
                            )
                        />
                    }.into_any(),
                };

                view! {
                    <div
                        class="fx-chart-legend-item"
                        style=format!(
                            "display: flex; align-items: center; opacity: {}; cursor: {}; font-size: 12px; color: var(--fx-color-text-secondary, #8c8c8c); user-select: none;",
                            opacity, cursor
                        )
                        on:click=move |_| {
                            if is_interactive {
                                if let Some(ref callback) = on_toggle {
                                    callback.run(idx);
                                }
                            }
                        }
                    >
                        {marker_view}
                        <span>{item.label.clone()}</span>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Inline legend for compact charts.
#[component]
pub fn InlineLegend(
    /// Legend items (name, color pairs).
    items: Vec<(String, String)>,
    /// Additional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let base_class = class.unwrap_or_else(|| "fx-chart-inline-legend".to_string());

    view! {
        <div
            class=base_class
            style="display: flex; gap: 16px; font-size: 11px; color: var(--fx-color-text-secondary, #8c8c8c);"
        >
            {items.into_iter().map(|(label, color)| {
                view! {
                    <span style="display: flex; align-items: center; gap: 4px;">
                        <span style=format!(
                            "width: 8px; height: 8px; border-radius: 50%; background: {};",
                            color
                        ) />
                        {label}
                    </span>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
