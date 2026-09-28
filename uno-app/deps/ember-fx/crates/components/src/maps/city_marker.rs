//! CityMarker component for map location callouts.

use leptos::prelude::*;
use crate::try_use_theme;
use super::types::{CityMarkerData, CityMarkerConfig, MarkerPosition, MarkerSize};

/// CityMarker component.
///
/// Displays a tooltip-style callout with icon, name, and value.
/// Used for marking cities/locations on maps.
///
/// # Example
///
/// ```ignore
/// let marker = CityMarkerData::new(
///     "chicago",
///     "Chicago",
///     "98,320,300",
///     MarkerPosition::absolute(400.0, 200.0),
/// );
///
/// view! {
///     <CityMarker data=marker />
/// }
/// ```
#[component]
pub fn CityMarker(
    /// Marker data.
    data: CityMarkerData,
    /// Configuration options.
    #[prop(optional)]
    config: Option<CityMarkerConfig>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<CityMarkerData>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let prefix = format!("fx-citymarker-{}", design_system);

    // Extract config
    let size = config.size;
    let show_connector = config.show_connector;
    let hover_expand = config.hover_expand;

    // Hover state
    let is_hovered = RwSignal::new(false);

    // Get position
    let (x, y) = match &data.position {
        MarkerPosition::Absolute { x, y } => (*x, *y),
        MarkerPosition::LatLng { lat: _, lng: _ } => {
            // Would need map context to convert - use absolute for now
            (0.0, 0.0)
        }
    };

    // Icon color
    let icon_color = data.color
        .map(|c| c.as_css().to_string())
        .unwrap_or_else(|| "var(--fx-color-primary, #1890ff)".to_string());

    // Size dimensions
    let (icon_size, font_size, padding) = match size {
        MarkerSize::Small => (24, 11, 4),
        MarkerSize::Medium => (32, 13, 6),
        MarkerSize::Large => (40, 15, 8),
    };

    let size_class = size.class_suffix();
    let combined_class = format!("{} {}-{} {}", prefix, prefix, size_class, class.clone().unwrap_or_default());

    let data_clone = data.clone();
    let data_for_click = data.clone();
    let has_click = on_click.is_some();
    let cursor = if has_click { "pointer" } else { "default" };

    // Scale on hover
    let scale = move || {
        if hover_expand && is_hovered.get() {
            "scale(1.05)"
        } else {
            "scale(1)"
        }
    };

    view! {
        <div
            class=combined_class
            style=move || format!(
                "position: absolute; left: {}px; top: {}px; \
                 display: flex; align-items: center; gap: {}px; \
                 transform: translate(-50%, -100%) {}; \
                 transition: transform 0.2s ease; cursor: {}; z-index: 10;",
                x, y, padding, scale(), cursor
            )
            on:mouseenter=move |_| is_hovered.set(true)
            on:mouseleave=move |_| is_hovered.set(false)
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb.run(data_for_click.clone());
                }
            }
        >
            // Icon badge
            <div style=format!(
                "width: {}px; height: {}px; border-radius: 8px; \
                 background: {}; display: flex; align-items: center; justify-content: center; \
                 box-shadow: var(--fx-marker-shadow, 0 2px 8px rgba(0,0,0,0.3));",
                icon_size, icon_size, icon_color
            )>
                <svg
                    viewBox="0 0 24 24"
                    width=icon_size / 2
                    height=icon_size / 2
                    fill="white"
                >
                    <path d=data.icon.svg_path() />
                </svg>
            </div>

            // Label box
            <div style=format!(
                "background: var(--fx-color-bg-elevated, #1f1f1f); \
                 border: 1px solid var(--fx-color-border, #434343); \
                 border-radius: 6px; padding: {}px {}px; \
                 box-shadow: var(--fx-marker-shadow, 0 2px 8px rgba(0,0,0,0.3));",
                padding, padding + 4
            )>
                <div style=format!(
                    "font-size: {}px; font-weight: 500; color: var(--fx-color-text-secondary, #888); \
                     white-space: nowrap;",
                    font_size - 2
                )>
                    {data_clone.name.clone()}
                </div>
                <div style=format!(
                    "font-size: {}px; font-weight: 600; color: var(--fx-color-text, #fff); \
                     white-space: nowrap;",
                    font_size
                )>
                    {data_clone.value.clone()}
                </div>
            </div>

            // Connector line (pointing down to exact location)
            {show_connector.then(|| view! {
                <div style=format!(
                    "position: absolute; left: 50%; bottom: -8px; \
                     width: 2px; height: 8px; background: {}; \
                     transform: translateX(-50%);",
                    icon_color
                ) />
            })}
        </div>
    }
}

/// CityMarkerGroup component for rendering multiple markers.
///
/// Handles positioning and z-index management for overlapping markers.
#[component]
pub fn CityMarkerGroup(
    /// Markers to display.
    markers: Signal<Vec<CityMarkerData>>,
    /// Configuration options (applies to all markers).
    #[prop(optional)]
    config: Option<CityMarkerConfig>,
    /// Click handler.
    #[prop(optional, into)]
    on_marker_click: Option<Callback<CityMarkerData>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let config = config.unwrap_or_default();

    view! {
        <div class=class.clone().unwrap_or_default() style="position: relative; width: 100%; height: 100%;">
            {move || {
                markers.get().into_iter().map(|marker| {
                    let config_clone = config.clone();
                    let callback = on_marker_click.clone();
                    if let Some(cb) = callback {
                        view! {
                            <CityMarker
                                data=marker
                                config=config_clone
                                on_click=cb
                            />
                        }.into_any()
                    } else {
                        view! {
                            <CityMarker
                                data=marker
                                config=config_clone
                            />
                        }.into_any()
                    }
                }).collect_view()
            }}
        </div>
    }
}
