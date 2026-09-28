//! GeoActivityMap Leptos component.
//!
//! A world map with pulsing activity markers.

use leptos::prelude::*;
use crate::try_use_theme;

/// An activity marker on the map.
#[derive(Debug, Clone)]
pub struct ActivityMarker {
    /// X coordinate in SVG viewbox space (0-128).
    pub x: f64,
    /// Y coordinate in SVG viewbox space (0-72).
    pub y: f64,
    /// Activity level (affects marker size).
    pub value: f64,
    /// Optional label.
    pub label: Option<String>,
}

impl ActivityMarker {
    /// Create a new marker.
    pub fn new(x: f64, y: f64, value: f64) -> Self {
        Self {
            x,
            y,
            value,
            label: None,
        }
    }

    /// Create a marker with a label.
    pub fn with_label(x: f64, y: f64, value: f64, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            value,
            label: Some(label.into()),
        }
    }
}

/// Predefined city locations on the map.
pub mod locations {
    use super::ActivityMarker;

    pub fn new_york(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(24.0, 26.0, value, "New York")
    }

    pub fn los_angeles(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(14.0, 28.0, value, "Los Angeles")
    }

    pub fn london(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(49.0, 19.0, value, "London")
    }

    pub fn paris(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(51.0, 21.0, value, "Paris")
    }

    pub fn tokyo(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(108.0, 26.0, value, "Tokyo")
    }

    pub fn sydney(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(114.0, 54.0, value, "Sydney")
    }

    pub fn singapore(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(96.0, 42.0, value, "Singapore")
    }

    pub fn dubai(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(70.0, 32.0, value, "Dubai")
    }

    pub fn sao_paulo(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(32.0, 50.0, value, "São Paulo")
    }

    pub fn mumbai(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(80.0, 34.0, value, "Mumbai")
    }

    pub fn shanghai(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(98.0, 26.0, value, "Shanghai")
    }

    pub fn frankfurt(value: f64) -> ActivityMarker {
        ActivityMarker::with_label(52.0, 20.0, value, "Frankfurt")
    }
}

/// Configuration for the map.
#[derive(Debug, Clone)]
pub struct GeoActivityMapConfig {
    /// Map width.
    pub width: u32,
    /// Map height.
    pub height: u32,
    /// Show zoom controls.
    pub show_zoom_controls: bool,
    /// Enable pulsing animation.
    pub animate: bool,
    /// Base marker size.
    pub base_marker_size: f64,
    /// Maximum marker size.
    pub max_marker_size: f64,
    /// Marker color (CSS).
    pub marker_color: String,
    /// Glow color (CSS).
    pub glow_color: String,
}

impl Default for GeoActivityMapConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 400,
            show_zoom_controls: true,
            animate: true,
            base_marker_size: 3.0,
            max_marker_size: 12.0,
            marker_color: "#52c41a".to_string(),
            glow_color: "rgba(82, 196, 26, 0.4)".to_string(),
        }
    }
}

/// GeoActivityMap component.
///
/// A world map with pulsing activity markers.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{GeoActivityMap, ActivityMarker, locations};
///
/// let markers = vec![
///     locations::new_york(100.0),
///     locations::london(80.0),
///     locations::tokyo(60.0),
/// ];
///
/// view! {
///     <GeoActivityMap
///         markers=Signal::derive(move || markers.clone())
///         title="Customer Activity".to_string()
///     />
/// }
/// ```
#[component]
pub fn GeoActivityMap(
    /// Activity markers.
    #[prop(into)]
    markers: Signal<Vec<ActivityMarker>>,
    /// Map title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Configuration.
    #[prop(optional)]
    config: Option<GeoActivityMapConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let width = config.width;
    let height = config.height;
    let show_zoom_controls = config.show_zoom_controls;
    let animate = config.animate;
    let base_marker_size = config.base_marker_size;
    let max_marker_size = config.max_marker_size;
    let marker_color = config.marker_color.clone();
    let glow_color = config.glow_color.clone();

    let prefix = format!("fx-geo-activity-{}", design_system);

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

    // Zoom state
    let zoom = RwSignal::new(1.0f64);

    // Calculate marker sizes based on values
    let marker_sizes = move || {
        let all_markers = markers.get();
        let max_value = all_markers
            .iter()
            .map(|m| m.value)
            .fold(0.0f64, f64::max)
            .max(1.0);

        all_markers
            .iter()
            .map(|m| {
                let normalized = m.value / max_value;
                let size = base_marker_size + (normalized * (max_marker_size - base_marker_size));
                (m.x, m.y, size, m.label.clone())
            })
            .collect::<Vec<_>>()
    };

    // Viewbox calculation with zoom
    let viewbox = move || {
        let z = zoom.get();
        let vb_width = 128.0 / z;
        let vb_height = 72.0 / z;
        let offset_x = (128.0 - vb_width) / 2.0;
        let offset_y = (72.0 - vb_height) / 2.0;
        format!("{} {} {} {}", offset_x - 1.0, offset_y - 1.0, vb_width + 2.0, vb_height + 2.0)
    };

    let marker_color_clone = marker_color.clone();
    let glow_color_clone = glow_color.clone();

    view! {
        <div
            class=combined_class
            style=format!("width: {}px; height: {}px;", width, height)
        >
            // Title bar
            <div class=format!("{}-header", prefix)>
                {title.clone().map(|t| {
                    let prefix = prefix.clone();
                    view! {
                        <span class=format!("{}-title", prefix)>
                            {t}
                        </span>
                    }
                })}

                // Zoom controls
                {show_zoom_controls.then(|| {
                    let prefix = prefix.clone();
                    view! {
                        <div class=format!("{}-zoom-controls", prefix)>
                            <button
                                class=format!("{}-zoom-btn", prefix)
                                on:click=move |_| zoom.update(|z| *z = (*z * 1.5).min(4.0))
                            >
                                "+"
                            </button>
                            <button
                                class=format!("{}-zoom-btn", prefix)
                                on:click=move |_| zoom.update(|z| *z = (*z / 1.5).max(0.5))
                            >
                                "-"
                            </button>
                        </div>
                    }
                })}
            </div>

            // Map SVG
            <svg
                class=format!("{}-map", prefix)
                viewBox=viewbox
                preserveAspectRatio="xMidYMid meet"
            >
                // Map background
                <rect
                    x="-1"
                    y="-1"
                    width="130"
                    height="74"
                    fill="var(--fx-color-bg-container, #1a1a1a)"
                />

                // World map dots (using path from SVG file or simplified)
                <g class=format!("{}-countries", prefix)>
                    <path
                        fill="#3a3a3a"
                        d="M24.5,15.5h1M26.5,15.5h4M32.5,15.5h1M34.5,15.5h1M36.5,15.5h1M98.5,15.5h2M101.5,15.5h2M104.5,15.5h1M17.5,16.5h1M24.5,16.5h10M35.5,16.5h3M93.5,16.5h1M95.5,16.5h2M98.5,16.5h2M101.5,16.5h1M103.5,16.5h3M107.5,16.5h2M16.5,17.5h2M23.5,17.5h9M33.5,17.5h2M36.5,17.5h2M49.5,17.5h2M52.5,17.5h2M55.5,17.5h1M57.5,17.5h1M59.5,17.5h1M91.5,17.5h1M93.5,17.5h7M101.5,17.5h1M103.5,17.5h5M109.5,17.5h2M15.5,18.5h4M22.5,18.5h9M32.5,18.5h7M46.5,18.5h3M50.5,18.5h8M59.5,18.5h2M62.5,18.5h2M91.5,18.5h1M93.5,18.5h9M103.5,18.5h7M111.5,18.5h1M13.5,19.5h1M15.5,19.5h5M22.5,19.5h6M29.5,19.5h11M46.5,19.5h1M48.5,19.5h11M60.5,19.5h1M62.5,19.5h4M67.5,19.5h2M70.5,19.5h2M93.5,19.5h1M95.5,19.5h7M103.5,19.5h8M112.5,19.5h1M12.5,20.5h9M23.5,20.5h4M28.5,20.5h13M46.5,20.5h1M48.5,20.5h19M68.5,20.5h5M95.5,20.5h7M103.5,20.5h8M112.5,20.5h1"
                    />
                </g>

                // Glow/pulse definitions
                <defs>
                    <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
                        <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>
                </defs>

                // Activity markers
                {move || {
                    marker_sizes().into_iter().map(|(x, y, size, _label)| {
                        let glow_size = size * 2.5;
                        let marker_color = marker_color_clone.clone();
                        let glow_color = glow_color_clone.clone();

                        view! {
                            <g class="activity-marker">
                                // Outer glow (pulsing)
                                <circle
                                    cx=format!("{:.1}", x)
                                    cy=format!("{:.1}", y)
                                    r=format!("{:.1}", glow_size)
                                    fill=glow_color.clone()
                                    filter="url(#glow)"
                                >
                                    {animate.then(|| view! {
                                        <animate
                                            attributeName="r"
                                            values=format!("{:.1};{:.1};{:.1}", glow_size * 0.8, glow_size * 1.2, glow_size * 0.8)
                                            dur="2s"
                                            repeatCount="indefinite"
                                        />
                                        <animate
                                            attributeName="opacity"
                                            values="0.6;0.3;0.6"
                                            dur="2s"
                                            repeatCount="indefinite"
                                        />
                                    })}
                                </circle>

                                // Inner solid circle
                                <circle
                                    cx=format!("{:.1}", x)
                                    cy=format!("{:.1}", y)
                                    r=format!("{:.1}", size)
                                    fill=marker_color.clone()
                                    opacity="0.9"
                                />

                                // Center highlight
                                <circle
                                    cx=format!("{:.1}", x)
                                    cy=format!("{:.1}", y)
                                    r=format!("{:.1}", size * 0.3)
                                    fill="white"
                                    opacity="0.5"
                                />
                            </g>
                        }
                    }).collect_view()
                }}
            </svg>

            // Legend
            <div class=format!("{}-legend", prefix)>
                <span class=format!("{}-legend-item", prefix)>
                    <span
                        class=format!("{}-legend-dot", prefix)
                        style=format!("background-color: {};", marker_color)
                    />
                    "Activity"
                </span>
            </div>
        </div>
    }
}
