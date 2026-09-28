//! ArcFlowLines component for connection visualization.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{ArcConnection, ArcFlowConfig, ArcStyle, arc_connection_path};

/// ArcFlowLines component.
///
/// Displays curved connection lines between points with optional animation.
///
/// # Example
///
/// ```ignore
/// let connections = Signal::derive(|| vec![
///     ArcConnection::new(
///         "conn-1",
///         ArcPoint::new(100.0, 200.0),
///         ArcPoint::new(400.0, 150.0),
///     ).style(ArcStyle::Animated),
/// ]);
///
/// view! {
///     <ArcFlowLines connections=connections />
/// }
/// ```
#[component]
pub fn ArcFlowLines(
    /// Arc connections to render.
    connections: Signal<Vec<ArcConnection>>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<ArcFlowConfig>,
    /// Callback when connection is clicked.
    #[prop(optional, into)]
    on_connection_click: Option<Callback<ArcConnection>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-arcflow-{}", design_system);

    // Extract config
    let width = config.width;
    let height = config.height;
    let stroke_width = config.stroke_width;
    let curvature = config.curvature;
    let show_endpoints = config.show_endpoints;
    let animate_duration = config.animate_duration;
    let show_labels = config.show_labels;

    // Hover state
    let hovered_id = RwSignal::new(None::<String>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style=format!("position: relative; width: {}px; height: {}px;", width, height)
        >
            <svg
                viewBox=format!("0 0 {} {}", width, height)
                style="width: 100%; height: 100%;"
            >
                // CSS for animations
                <style>
                    {format!(r#"
                        .{prefix}-arc-animated {{
                            stroke-dasharray: 8 4;
                            animation: {prefix}-dash-flow {animate_duration}ms linear infinite;
                        }}
                        @keyframes {prefix}-dash-flow {{
                            to {{ stroke-dashoffset: -12; }}
                        }}
                        .{prefix}-arc-pulse {{
                            animation: {prefix}-pulse 2s ease-in-out infinite;
                        }}
                        @keyframes {prefix}-pulse {{
                            0%, 100% {{ opacity: 0.4; }}
                            50% {{ opacity: 1; }}
                        }}
                    "#, prefix = prefix, animate_duration = animate_duration)}
                </style>

                // Render connections
                {move || {
                    connections.get().into_iter().map(|conn| {
                        let conn_id = conn.id.clone();
                        let conn_id_hover = conn_id.clone();
                        let conn_id_unhover = conn_id.clone();
                        let conn_clone = conn.clone();
                        let conn_for_click = conn.clone();
                        let callback = on_connection_click.clone();

                        // Generate path
                        let path = arc_connection_path(
                            conn.source.x,
                            conn.source.y,
                            conn.target.x,
                            conn.target.y,
                            curvature,
                        );

                        // Get color
                        let stroke_color = conn.color
                            .map(|c| c.as_css().to_string())
                            .unwrap_or_else(|| "var(--fx-color-primary, #eb2f96)".to_string());

                        // Style classes
                        let style_class = match conn.style {
                            ArcStyle::Animated => format!("{}-arc-animated", prefix),
                            ArcStyle::Pulse => format!("{}-arc-pulse", prefix),
                            _ => String::new(),
                        };

                        let dash_array = conn.style.dash_array();

                        // Line thickness based on value
                        let line_width = conn.value
                            .map(|v| stroke_width + v.min(5.0))
                            .unwrap_or(stroke_width);

                        // Hover opacity
                        let is_hovered = move || hovered_id.get().as_ref() == Some(&conn_id);
                        let opacity = move || if is_hovered() { 1.0 } else { 0.7 };

                        // Midpoint for label
                        let mx = (conn.source.x + conn.target.x) / 2.0;
                        let my = (conn.source.y + conn.target.y) / 2.0 - curvature * 20.0;

                        let has_click = callback.is_some();
                        let cursor = if has_click { "pointer" } else { "default" };

                        view! {
                            <g>
                                // Connection path
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke=stroke_color.clone()
                                    stroke-width=line_width
                                    stroke-linecap="round"
                                    stroke-dasharray=dash_array
                                    class=style_class
                                    opacity=opacity
                                    style=format!(
                                        "cursor: {}; transition: opacity 0.2s ease, stroke-width 0.2s ease;",
                                        cursor
                                    )
                                    on:mouseenter=move |_| hovered_id.set(Some(conn_id_hover.clone()))
                                    on:mouseleave=move |_| hovered_id.set(None)
                                    on:click=move |_| {
                                        if let Some(ref cb) = callback {
                                            cb.run(conn_for_click.clone());
                                        }
                                    }
                                />

                                // Endpoints
                                {show_endpoints.then(|| view! {
                                    <>
                                        // Source point
                                        <circle
                                            cx=conn_clone.source.x
                                            cy=conn_clone.source.y
                                            r=4.0
                                            fill=stroke_color.clone()
                                        />
                                        // Target point
                                        <circle
                                            cx=conn_clone.target.x
                                            cy=conn_clone.target.y
                                            r=4.0
                                            fill=stroke_color.clone()
                                        />
                                    </>
                                })}

                                // Label
                                {(show_labels && conn.label.is_some()).then(|| {
                                    let label = conn.label.clone().unwrap_or_default();
                                    view! {
                                        <text
                                            x=mx
                                            y=my
                                            text-anchor="middle"
                                            style="font-size: 11px; fill: var(--fx-color-text-secondary, #888); pointer-events: none;"
                                        >
                                            {label}
                                        </text>
                                    }
                                })}

                                // Source label
                                {conn.source.label.as_ref().map(|label| view! {
                                    <text
                                        x=conn.source.x
                                        y=conn.source.y - 10.0
                                        text-anchor="middle"
                                        style="font-size: 10px; fill: var(--fx-color-text-secondary, #888); pointer-events: none;"
                                    >
                                        {label.clone()}
                                    </text>
                                })}

                                // Target label
                                {conn.target.label.as_ref().map(|label| view! {
                                    <text
                                        x=conn.target.x
                                        y=conn.target.y - 10.0
                                        text-anchor="middle"
                                        style="font-size: 10px; fill: var(--fx-color-text-secondary, #888); pointer-events: none;"
                                    >
                                        {label.clone()}
                                    </text>
                                })}
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>
        </div>
    }
}
