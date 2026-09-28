//! WindMap component for dot-based map visualization.
//!
//! This component renders pre-computed dot positions from the wind_map SVG,
//! creating a dot-matrix style map visualization.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;
use super::wind_map_data::{WIND_MAP_DOTS, WindMapDot, DOT_COUNT};

/// Configuration for WindMap component.
#[derive(Debug, Clone)]
pub struct WindMapConfig {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Dot size multiplier.
    pub dot_size: f64,
    /// Dot color.
    pub dot_color: String,
    /// Enable interactive hover.
    pub interactive: bool,
    /// Show tooltip on hover.
    pub show_tooltip: bool,
    /// Background color.
    pub background_color: String,
    /// Dot opacity.
    pub dot_opacity: f64,
    /// Show glow effect on dots.
    pub show_glow: bool,
    /// Animate dots.
    pub animated: bool,
}

impl Default for WindMapConfig {
    fn default() -> Self {
        Self {
            width: 900,
            height: 450,
            dot_size: 1.0,
            dot_color: "#ffffff".to_string(),
            interactive: true,
            show_tooltip: true,
            background_color: "#181413".to_string(),
            dot_opacity: 1.0,
            show_glow: false,
            animated: false,
        }
    }
}

/// WindMap component.
///
/// Displays wind measurement points as dots forming wind patterns.
///
/// # Example
///
/// ```ignore
/// view! {
///     <WindMap />
/// }
/// ```
#[component]
pub fn WindMap(
    /// Configuration options.
    #[prop(optional)]
    config: Option<WindMapConfig>,
    /// Callback when dot is clicked.
    #[prop(optional, into)]
    on_dot_click: Option<Callback<WindMapDot>>,
    /// Callback when dot is hovered.
    #[prop(optional, into)]
    on_dot_hover: Option<Callback<Option<WindMapDot>>>,
    /// Loading state.
    #[prop(optional)]
    loading: Option<Signal<bool>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-wind-map-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let dot_size = config.dot_size;
    let dot_color = config.dot_color.clone();
    let interactive = config.interactive;
    let show_tooltip = config.show_tooltip;
    let background_color = config.background_color.clone();
    let dot_opacity = config.dot_opacity;
    let show_glow = config.show_glow;
    let animated = config.animated;

    // Reactive state
    let hovered_dot = RwSignal::new(None::<usize>);
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(f64, f64)>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Animation CSS
    let animation_css = if animated {
        r#"
        @keyframes wind-pulse {
            0%, 100% { opacity: 0.6; }
            50% { opacity: 1.0; }
        }
        .wind-dot { animation: wind-pulse 2s ease-in-out infinite; }
        "#
    } else {
        ""
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; background: {};",
                width, height, background_color
            )
        >
            // Animation styles
            {(!animation_css.is_empty()).then(|| view! {
                <style>{animation_css}</style>
            })}

            // Loading overlay
            {move || {
                loading.map(|l| l.get()).unwrap_or(false).then(|| view! {
                    <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5); z-index: 100;">
                        <div style="color: var(--fx-color-text); font-size: 14px;">{"Loading..."}</div>
                    </div>
                })
            }}

            <svg
                viewBox=format!("0 0 {} {}", width, height)
                style="width: 100%; height: 100%;"
                preserveAspectRatio="xMidYMid meet"
            >
                // Defs for glow effect
                {show_glow.then(|| view! {
                    <defs>
                        <filter id="wind-glow" x="-50%" y="-50%" width="200%" height="200%">
                            <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
                            <feMerge>
                                <feMergeNode in="coloredBlur"/>
                                <feMergeNode in="SourceGraphic"/>
                            </feMerge>
                        </filter>
                    </defs>
                })}

                // Background
                <rect
                    width=width
                    height=height
                    fill=background_color.clone()
                />

                // Wind dots
                <g class="wind-dots" filter={if show_glow { "url(#wind-glow)" } else { "" }}>
                    {WIND_MAP_DOTS.iter().enumerate().map(|(idx, dot)| {
                        let dot_copy = *dot;
                        let dot_for_click = *dot;
                        let dot_for_hover = *dot;
                        let callback_click = on_dot_click.clone();
                        let callback_hover = on_dot_hover.clone();
                        let dot_color = dot_color.clone();

                        // Hover state
                        let opacity = move || {
                            let is_hovered = hovered_dot.get() == Some(idx);
                            if is_hovered { 1.0 } else { dot_opacity }
                        };
                        let radius = move || {
                            let is_hovered = hovered_dot.get() == Some(idx);
                            let base_r = dot.radius * dot_size;
                            if is_hovered { base_r * 2.0 } else { base_r }
                        };

                        view! {
                            <circle
                                class={if animated { "wind-dot" } else { "" }}
                                cx=dot.x
                                cy=dot.y
                                r=radius
                                fill=dot_color.clone()
                                opacity=opacity
                                style=format!(
                                    "cursor: {}; transition: r 0.15s, opacity 0.15s;{}",
                                    if interactive { "pointer" } else { "default" },
                                    if animated { format!("animation-delay: {}ms;", (idx % 100) * 20) } else { "".to_string() }
                                )
                                on:mouseenter={
                                    let callback_hover = callback_hover.clone();
                                    move |e| {
                                        if interactive {
                                            hovered_dot.set(Some(idx));
                                            if let Some(ref cb) = callback_hover {
                                                cb.run(Some(dot_for_hover));
                                            }
                                            if show_tooltip {
                                                tooltip_content.set(Some((
                                                    dot_copy.x,
                                                    dot_copy.y,
                                                )));
                                                if let Some(rect) = e.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                                    .and_then(|el| el.closest("svg").ok().flatten())
                                                    .map(|svg| svg.get_bounding_client_rect())
                                                {
                                                    tooltip_pos.set((
                                                        e.client_x() as f64 - rect.left() + 10.0,
                                                        e.client_y() as f64 - rect.top() - 40.0,
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                                on:mouseleave={
                                    let callback_hover = callback_hover.clone();
                                    move |_| {
                                        hovered_dot.set(None);
                                        tooltip_content.set(None);
                                        if let Some(ref cb) = callback_hover {
                                            cb.run(None);
                                        }
                                    }
                                }
                                on:click={
                                    let callback_click = callback_click.clone();
                                    move |_| {
                                        if interactive {
                                            if let Some(ref cb) = callback_click {
                                                cb.run(dot_for_click);
                                            }
                                        }
                                    }
                                }
                            />
                        }
                    }).collect_view()}
                </g>
            </svg>

            // Tooltip
            {move || {
                tooltip_content.get().map(|(x, y)| {
                    let (tx, ty) = tooltip_pos.get();
                    view! {
                        <div style=format!(
                            "position: absolute; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 6px; padding: 8px 12px; \
                             pointer-events: none; z-index: 50; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.4);",
                            tx, ty
                        )>
                            <div style="font-size: 12px; color: var(--fx-color-text-secondary, #888);">
                                {"Point"}
                            </div>
                            <div style="font-size: 11px; color: var(--fx-color-text, #fff);">
                                {format!("({:.1}, {:.1})", x, y)}
                            </div>
                        </div>
                    }
                })
            }}

            // Info badge
            <div style="position: absolute; bottom: 16px; right: 16px; \
                        background: var(--fx-color-bg-elevated, #1f1f1f); \
                        border: 1px solid var(--fx-color-border, #434343); \
                        border-radius: 6px; padding: 8px 12px;">
                <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {"Dot Matrix"}
                </div>
                <div style="font-size: 12px; font-weight: 500; color: var(--fx-color-text, #fff);">
                    {format!("{} points", DOT_COUNT)}
                </div>
            </div>
        </div>
    }
}

/// Standalone info component for WindMap.
#[component]
pub fn WindMapInfo(
    /// Title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let prefix = format!("fx-wind-map-info-{}", design_system);
    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    view! {
        <div
            class=combined_class
            style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                   border: 1px solid var(--fx-color-border, #434343); \
                   border-radius: 6px; padding: 12px; display: inline-block;"
        >
            {title.map(|t| view! {
                <div style="font-size: 12px; font-weight: 500; color: var(--fx-color-text, #fff); margin-bottom: 8px;">
                    {t}
                </div>
            })}

            <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                {format!("{} points", DOT_COUNT)}
            </div>
        </div>
    }
}
