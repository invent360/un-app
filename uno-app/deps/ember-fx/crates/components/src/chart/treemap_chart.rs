use leptos::prelude::*;

use super::types::{ColorPalette, TreemapChartConfig, TreemapNode};
use super::utils::{chart_prefix, try_use_theme};

/// Hierarchical treemap chart component with full interactivity
///
/// Features hover effects, tooltips, and click handling for each node.
///
/// # Props
///
/// - `data` - Treemap node data (hierarchical)
/// - `config` - Chart configuration
/// - `colors` - Optional color palette
/// - `on_node_click` - Click handler with node index
/// - `loading` - Loading state
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::chart::{TreemapChart, TreemapNode};
///
/// let data = vec![
///     TreemapNode::new("Category A", 100.0),
///     TreemapNode::new("Category B", 75.0),
///     TreemapNode::new("Category C", 50.0),
/// ];
///
/// view! {
///     <TreemapChart
///         data=Signal::derive(move || data.clone())
///         on_node_click=Callback::new(|idx| log::info!("Clicked node {}", idx))
///     />
/// }
/// ```
#[component]
pub fn TreemapChart(
    /// Treemap node data (hierarchical)
    #[prop(into)]
    data: Signal<Vec<TreemapNode>>,
    /// Configuration for the treemap chart
    #[prop(optional)]
    config: Option<TreemapChartConfig>,
    /// Optional color palette
    #[prop(optional, into)]
    colors: Option<Signal<ColorPalette>>,
    /// Click handler with node index
    #[prop(optional, into)]
    on_node_click: Option<Callback<usize>>,
    /// Whether the chart is in a loading state
    #[prop(optional)]
    loading: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = try_use_theme();
    let prefix = chart_prefix("treemap");

    // Interactive state
    let hovered_node = RwSignal::new(None::<usize>); // Index of hovered node
    let tooltip_data = RwSignal::new(None::<(String, f64, f64, f64, f64, String)>); // (name, value, percentage, x, y, color)
    // Track mouse position relative to container for tooltip
    let mouse_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // Container ref for calculating relative mouse position
    let container_ref = NodeRef::<leptos::html::Div>::new();

    let config = config.unwrap_or_default();

    let default_colors = ColorPalette::default();
    let colors = colors.unwrap_or_else(|| Signal::stored(default_colors));

    // Chart dimensions
    let width = 600.0;
    let height = 400.0;
    let padding = 2.0;

    // Store config value for closures
    let show_labels = config.show_labels;

    // Calculate total value for percentage calculations
    let total_value = Memo::new(move |_| {
        let nodes = data.get();
        nodes.iter().map(|n| n.value).sum::<f64>()
    });

    // Calculate treemap layout using squarified algorithm (simplified)
    let treemap_rects = Memo::new(move |_| {
        let nodes = data.get();
        let palette = colors.get();

        if nodes.is_empty() {
            return Vec::new();
        }

        // Calculate total value
        let total: f64 = nodes.iter().map(|n| n.value).sum();
        if total == 0.0 {
            return Vec::new();
        }

        // Simple row-based layout
        // Tuple: (x, y, width, height, name, color, value, original_index)
        let mut rects: Vec<(f64, f64, f64, f64, String, String, f64, usize)> = Vec::new();
        let mut current_y = 0.0;
        let mut remaining_height = height;
        let mut remaining_value = total;

        // Sort nodes by value (descending) for better layout
        let mut sorted_nodes: Vec<_> = nodes.iter().enumerate().collect();
        sorted_nodes.sort_by(|a, b| b.1.value.partial_cmp(&a.1.value).unwrap_or(std::cmp::Ordering::Equal));

        // Simple strip-based treemap
        let mut row_nodes: Vec<(usize, &TreemapNode)> = Vec::new();
        let mut row_value = 0.0;
        let target_row_height = (height / (nodes.len() as f64).sqrt()).max(40.0);

        for (idx, node) in sorted_nodes {
            row_nodes.push((idx, node));
            row_value += node.value;

            let row_height = (row_value / remaining_value) * remaining_height;

            // Check if we should finalize this row
            if row_height >= target_row_height || remaining_value - row_value < 0.001 {
                // Layout this row
                let mut current_x = 0.0;
                for (node_idx, row_node) in &row_nodes {
                    let node_width = (row_node.value / row_value) * width;

                    let color = row_node.color.as_ref()
                        .map(|c| c.as_css().to_string())
                        .unwrap_or_else(|| palette.color_at(*node_idx).to_string());

                    rects.push((
                        current_x + padding,
                        current_y + padding,
                        (node_width - padding * 2.0).max(0.0),
                        (row_height - padding * 2.0).max(0.0),
                        row_node.name.clone(),
                        color,
                        row_node.value,
                        *node_idx, // original index for click callback
                    ));

                    current_x += node_width;
                }

                current_y += row_height;
                remaining_height -= row_height;
                remaining_value -= row_value;
                row_nodes.clear();
                row_value = 0.0;
            }
        }

        // Handle any remaining nodes
        if !row_nodes.is_empty() {
            let row_height = remaining_height;
            let mut current_x = 0.0;
            for (node_idx, row_node) in &row_nodes {
                let node_width = (row_node.value / row_value) * width;

                let color = row_node.color.as_ref()
                    .map(|c| c.as_css().to_string())
                    .unwrap_or_else(|| palette.color_at(*node_idx).to_string());

                rects.push((
                    current_x + padding,
                    current_y + padding,
                    (node_width - padding * 2.0).max(0.0),
                    (row_height - padding * 2.0).max(0.0),
                    row_node.name.clone(),
                    color,
                    row_node.value,
                    *node_idx, // original index for click callback
                ));

                current_x += node_width;
            }
        }

        rects
    });

    // Pre-clone prefix for use in closures
    let prefix_svg = prefix.clone();
    let prefix_node = prefix.clone();
    let prefix_rect = prefix.clone();
    let prefix_label = prefix.clone();
    let prefix_value = prefix.clone();
    let prefix_loading = prefix.clone();
    let prefix_spinner = prefix.clone();

    let combined_class = {
        let mut parts = vec![prefix.clone()];
        if loading {
            parts.push(format!("{}-loading", prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class style="position: relative;" node_ref=container_ref>
            <svg viewBox=format!("0 0 {} {}", width, height) class=format!("{}-svg", prefix_svg)>
                // Treemap rectangles with interactivity
                {
                    let prefix_node = prefix_node.clone();
                    let prefix_rect = prefix_rect.clone();
                    let prefix_label = prefix_label.clone();
                    let prefix_value = prefix_value.clone();
                    let on_click = on_node_click.clone();

                    move || {
                        let prefix_node = prefix_node.clone();
                        let prefix_rect = prefix_rect.clone();
                        let prefix_label = prefix_label.clone();
                        let prefix_value = prefix_value.clone();
                        let on_click = on_click.clone();
                        let total = total_value.get();

                        treemap_rects.get().into_iter().map({
                            let prefix_node = prefix_node.clone();
                            let prefix_rect = prefix_rect.clone();
                            let prefix_label = prefix_label.clone();
                            let prefix_value = prefix_value.clone();
                            let on_click = on_click.clone();

                            move |(x, y, w, h, label, color, value, node_idx)| {
                                let min_label_size = 40.0;
                                let can_show_label = w > min_label_size && h > 20.0;
                                let prefix_node = prefix_node.clone();
                                let prefix_rect = prefix_rect.clone();
                                let prefix_label = prefix_label.clone();
                                let prefix_value = prefix_value.clone();

                                // Calculate percentage
                                let percentage = if total > 0.0 { (value / total) * 100.0 } else { 0.0 };

                                // Clone values for event handlers
                                let label_enter = label.clone();
                                let color_enter = color.clone();
                                let on_click = on_click.clone();

                                // Check if this node is hovered
                                let is_hovered = move || {
                                    hovered_node.get() == Some(node_idx)
                                };

                                // Opacity dims non-hovered nodes when any node is hovered
                                let opacity = move || {
                                    match hovered_node.get() {
                                        Some(idx) if idx != node_idx => 0.7,
                                        _ => 1.0,
                                    }
                                };

                                // Stroke attributes for hover highlight
                                let stroke = move || {
                                    if is_hovered() { "white" } else { "none" }
                                };
                                let stroke_width_hover = move || {
                                    if is_hovered() { "2" } else { "0" }
                                };

                                view! {
                                    <g class=format!("{}-node", prefix_node)>
                                        // Rectangle with interactive styling
                                        <rect
                                            x=x
                                            y=y
                                            width=w
                                            height=h
                                            fill=color.clone()
                                            stroke=stroke
                                            stroke-width=stroke_width_hover
                                            rx="2"
                                            class=prefix_rect.clone()
                                            style=move || format!(
                                                "opacity: {}; transition: all 0.2s ease; cursor: pointer;",
                                                opacity()
                                            )
                                            on:mouseenter=move |ev| {
                                                hovered_node.set(Some(node_idx));
                                                tooltip_data.set(Some((
                                                    label_enter.clone(),
                                                    value,
                                                    percentage,
                                                    x + w / 2.0,
                                                    y + h / 2.0,
                                                    color_enter.clone(),
                                                )));
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
                                                hovered_node.set(None);
                                                tooltip_data.set(None);
                                            }
                                            on:click=move |_| {
                                                if let Some(ref callback) = on_click {
                                                    callback.run(node_idx);
                                                }
                                            }
                                        />

                                        // Label
                                        {if show_labels && can_show_label {
                                            Some(view! {
                                                <text
                                                    x=x + w / 2.0
                                                    y=y + h / 2.0
                                                    text-anchor="middle"
                                                    dominant-baseline="middle"
                                                    fill="white"
                                                    font-size="12"
                                                    font-weight="600"
                                                    class=prefix_label.clone()
                                                    style="pointer-events: none;"
                                                >
                                                    {label.clone()}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}

                                        // Value (shown below label when there's enough space)
                                        {if show_labels && can_show_label && h > 40.0 {
                                            Some(view! {
                                                <text
                                                    x=x + w / 2.0
                                                    y=y + h / 2.0 + 14.0
                                                    text-anchor="middle"
                                                    dominant-baseline="middle"
                                                    fill="white"
                                                    opacity="0.9"
                                                    font-size="11"
                                                    class=prefix_value.clone()
                                                    style="pointer-events: none;"
                                                >
                                                    {format!("{:.1}", value)}
                                                </text>
                                            })
                                        } else {
                                            None
                                        }}
                                    </g>
                                }
                            }
                        }).collect_view()
                    }
                }
            </svg>

            // Tooltip showing node name, value, and percentage
            {move || {
                let prefix = prefix.clone();
                tooltip_data.get().map(|(name, value, percentage, _data_tip_x, _data_tip_y, color)| {
                    // Use actual mouse position for tooltip placement
                    let (mx, my) = mouse_pos.get();
                    let tooltip_x = mx + 15.0;
                    let tooltip_y = my;
                    view! {
                        <div
                            class=format!("{}-tooltip", prefix)
                            style=format!(
                                "position: absolute; left: {}px; top: {}px; background: var(--fx-color-bg-elevated, #1f1f1f); border: 1px solid var(--fx-color-border, #303030); border-radius: 6px; padding: 8px 12px; font-size: 12px; pointer-events: none; z-index: 10; box-shadow: 0 2px 8px rgba(0,0,0,0.3); transform: translateY(-50%);",
                                tooltip_x, tooltip_y
                            )
                        >
                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                <span style=format!("width: 10px; height: 10px; border-radius: 2px; background: {};", color) />
                                <span style="color: var(--fx-color-text, #fff); font-weight: 500;">
                                    {name}
                                </span>
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Value: {:.1}", value)}
                            </div>
                            <div style="color: var(--fx-color-text-secondary, #8c8c8c);">
                                {format!("Percentage: {:.1}%", percentage)}
                            </div>
                        </div>
                    }
                })
            }}

            // Loading overlay
            {move || {
                let prefix_loading = prefix_loading.clone();
                let prefix_spinner = prefix_spinner.clone();
                if loading {
                    Some(view! {
                        <div class=format!("{}-loading", prefix_loading)>
                            <div class=format!("{}-spinner", prefix_spinner)></div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
