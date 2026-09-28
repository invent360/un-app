//! SvgWorldMap component for geographic visualization.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::try_use_theme;
use super::types::{CountryState, SvgWorldMapConfig, ColorScale};
use super::svg_country_data::COUNTRIES;

/// SvgWorldMap component.
///
/// Displays an interactive world map with country-level granularity.
/// Supports hover highlighting, click callbacks, value-based coloring,
/// and optional zoom-to-country functionality.
///
/// # Example
///
/// ```ignore
/// let country_values = Signal::derive(|| vec![
///     CountryState::new("us", 85.0),
///     CountryState::new("cn", 72.0),
///     CountryState::new("de", 68.0),
/// ]);
///
/// view! {
///     <SvgWorldMap
///         data=country_values
///         config=SvgWorldMapConfig {
///             enable_zoom: true,
///             ..Default::default()
///         }
///         on_country_click=|state| log!("Clicked: {}", state.code)
///     />
/// }
/// ```
#[component]
pub fn SvgWorldMap(
    /// Country values for coloring.
    data: Signal<Vec<CountryState>>,
    /// Configuration options.
    #[prop(optional)]
    config: Option<SvgWorldMapConfig>,
    /// Callback when country is clicked.
    #[prop(optional, into)]
    on_country_click: Option<Callback<CountryState>>,
    /// Callback when country is hovered.
    #[prop(optional, into)]
    on_country_hover: Option<Callback<Option<String>>>,
    /// Callback when selection changes (multi-select mode).
    #[prop(optional, into)]
    on_selection_change: Option<Callback<Vec<String>>>,
    /// Callback when zoom state changes (country code or None for world view).
    #[prop(optional, into)]
    on_zoom_change: Option<Callback<Option<String>>>,
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
    let prefix = format!("fx-svgmap-{}", design_system);

    // Extract config values
    let width = config.width;
    let height = config.height;
    let viewbox_width = config.viewbox_width;
    let viewbox_height = config.viewbox_height;
    let color_scale = config.color_scale.clone();
    let default_fill = config.default_fill.clone();
    let stroke_color = config.stroke_color.clone();
    let stroke_width = config.stroke_width;
    let interactive = config.interactive;
    let show_tooltip = config.show_tooltip;
    let show_legend = config.show_legend;
    let min_value = config.min_value;
    let max_value = config.max_value;
    let multi_select = config.multi_select;
    let selection_color = config.selection_color.clone();
    let hover_color = config.hover_color.clone();
    let exclude_countries = config.exclude_countries.clone();
    let color_scale_for_legend = color_scale.clone();
    let enable_zoom = config.enable_zoom;
    let zoom_padding = config.zoom_padding;
    let show_zoom_controls = config.show_zoom_controls;

    // Reactive state
    let hovered_country = RwSignal::new(None::<String>);
    let selected_countries = RwSignal::new(Vec::<String>::new());
    let tooltip_pos = RwSignal::new((0.0_f64, 0.0_f64));
    let tooltip_content = RwSignal::new(None::<(String, String, String)>);

    // Zoom state: None = world view, Some(code) = zoomed to country
    let zoomed_country = RwSignal::new(None::<String>);
    // Store the zoomed country name for display
    let zoomed_country_name = RwSignal::new(None::<String>);

    let combined_class = format!("{} {}", prefix, class.clone().unwrap_or_default());

    // Compute viewBox based on zoom state
    let viewbox = move || {
        if let Some(code) = zoomed_country.get() {
            // Find the country's bounding box
            if let Some(country) = COUNTRIES.iter().find(|c| c.code == code) {
                let (min_x, min_y, bbox_width, bbox_height) = country.bbox;

                // Add padding
                let padded_x = (min_x - zoom_padding).max(0.0);
                let padded_y = (min_y - zoom_padding).max(0.0);
                let padded_w = bbox_width + zoom_padding * 2.0;
                let padded_h = bbox_height + zoom_padding * 2.0;

                // Maintain aspect ratio by expanding the smaller dimension
                let target_aspect = viewbox_width as f64 / viewbox_height as f64;
                let bbox_aspect = padded_w / padded_h;

                let (final_x, final_y, final_w, final_h) = if bbox_aspect > target_aspect {
                    // bbox is wider, expand height
                    let new_h = padded_w / target_aspect;
                    let y_offset = (new_h - padded_h) / 2.0;
                    (padded_x, padded_y - y_offset, padded_w, new_h)
                } else {
                    // bbox is taller, expand width
                    let new_w = padded_h * target_aspect;
                    let x_offset = (new_w - padded_w) / 2.0;
                    (padded_x - x_offset, padded_y, new_w, padded_h)
                };

                return format!("{:.1} {:.1} {:.1} {:.1}", final_x, final_y, final_w, final_h);
            }
        }
        // Default: world view
        format!("0 0 {} {}", viewbox_width, viewbox_height)
    };

    // Handler to zoom out
    let zoom_out = move |_| {
        zoomed_country.set(None);
        zoomed_country_name.set(None);
        if let Some(ref cb) = on_zoom_change {
            cb.run(None);
        }
    };

    view! {
        <div
            class=combined_class
            style=format!(
                "position: relative; width: {}px; height: {}px; background: var(--fx-color-bg, #141414);",
                width, height
            )
        >
            // Loading overlay
            {move || {
                loading.map(|l| l.get()).unwrap_or(false).then(|| view! {
                    <div style="position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.5); z-index: 100;">
                        <div style="color: var(--fx-color-text); font-size: 14px;">{"Loading..."}</div>
                    </div>
                })
            }}

            <svg
                viewBox=viewbox
                style="width: 100%; height: 100%; transition: viewBox 0.5s ease-in-out;"
                preserveAspectRatio="xMidYMid meet"
            >
                // Background - clickable to zoom out
                <rect
                    width=viewbox_width
                    height=viewbox_height
                    fill="var(--fx-color-bg, #141414)"
                    style=move || if enable_zoom && zoomed_country.get().is_some() {
                        "cursor: pointer;"
                    } else {
                        ""
                    }
                    on:click=move |_| {
                        if enable_zoom && zoomed_country.get().is_some() {
                            zoomed_country.set(None);
                            zoomed_country_name.set(None);
                            if let Some(ref cb) = on_zoom_change {
                                cb.run(None);
                            }
                        }
                    }
                />

                // Countries group
                <g class="countries">
                    {COUNTRIES.iter()
                        .filter(|country| !exclude_countries.contains(&country.code.to_string()))
                        .map(|country| {
                            let code = country.code;
                            let name = country.name;
                            let region = country.region;
                            let color_scale = color_scale.clone();
                            let default_fill = default_fill.clone();
                            let stroke_color = stroke_color.clone();
                            let selection_color = selection_color.clone();
                            let hover_color = hover_color.clone();
                            let on_country_click = on_country_click.clone();
                            let on_country_hover = on_country_hover.clone();
                            let on_selection_change = on_selection_change.clone();
                            let on_zoom_change = on_zoom_change.clone();

                            // Render all paths for this country
                            country.paths.iter().map(|path_d| {
                                let path_d = *path_d;
                                let code = code.to_string();
                                let code_for_hover = code.clone();
                                let code_for_click = code.clone();
                                let code_for_selection = code.clone();
                                let code_for_zoom = code.clone();
                                let name = name.to_string();
                                let name_for_zoom = name.clone();
                                let region = region.to_string();
                                let color_scale = color_scale.clone();
                                let default_fill = default_fill.clone();
                                let stroke_color = stroke_color.clone();
                                let selection_color = selection_color.clone();
                                let hover_color = hover_color.clone();
                                let on_country_click = on_country_click.clone();
                                let on_country_hover = on_country_hover.clone();
                                let on_selection_change = on_selection_change.clone();
                                let on_zoom_change = on_zoom_change.clone();

                                // Calculate fill color based on value
                                let fill_color = {
                                    let code = code.clone();
                                    let color_scale = color_scale.clone();
                                    let default_fill = default_fill.clone();
                                    move || {
                                        let country_data = data.get();
                                        if let Some(state) = country_data.iter().find(|s| s.code == code) {
                                            let t = if max_value > min_value {
                                                ((state.value - min_value) / (max_value - min_value)).clamp(0.0, 1.0)
                                            } else {
                                                0.5
                                            };
                                            color_scale.color_at(t)
                                        } else {
                                            default_fill.clone()
                                        }
                                    }
                                };

                                // Reactive fill, opacity, stroke based on hover/selection/zoom
                                let code_for_fill = code_for_hover.clone();
                                let code_for_opacity = code_for_hover.clone();
                                let code_for_stroke = code_for_hover.clone();
                                let code_for_sel_fill = code_for_selection.clone();
                                let code_for_sel_opacity = code_for_selection.clone();
                                let code_for_zoom_opacity = code_for_zoom.clone();

                                let effective_fill = {
                                    let selection_color = selection_color.clone();
                                    let hover_color = hover_color.clone();
                                    let fill_color = fill_color.clone();
                                    move || {
                                        let is_selected = selected_countries.get().contains(&code_for_sel_fill);
                                        let is_hovered = hovered_country.get().as_ref() == Some(&code_for_fill);
                                        if is_selected {
                                            selection_color.clone().unwrap_or_else(|| "var(--fx-color-primary, #1890ff)".to_string())
                                        } else if is_hovered {
                                            hover_color.clone().unwrap_or_else(|| fill_color())
                                        } else {
                                            fill_color()
                                        }
                                    }
                                };

                                let opacity = move || {
                                    let is_selected = selected_countries.get().contains(&code_for_sel_opacity);
                                    let is_hovered = hovered_country.get().as_ref() == Some(&code_for_opacity);
                                    let is_zoomed = zoomed_country.get().as_ref() == Some(&code_for_zoom_opacity);
                                    if is_hovered || is_selected || is_zoomed { 1.0 } else { 0.85 }
                                };
                                let stroke_w = move || {
                                    let is_hovered = hovered_country.get().as_ref() == Some(&code_for_stroke);
                                    if is_hovered { stroke_width * 2.0 } else { stroke_width }
                                };

                                view! {
                                    <path
                                        d=path_d
                                        fill=effective_fill
                                        stroke=stroke_color.clone()
                                        stroke-width=stroke_w
                                        opacity=opacity
                                        style=format!(
                                            "cursor: {}; transition: fill 0.2s, opacity 0.2s, stroke-width 0.2s;",
                                            if interactive { "pointer" } else { "default" }
                                        )
                                        on:mouseenter={
                                            let code = code.clone();
                                            let name = name.clone();
                                            let region = region.clone();
                                            let on_country_hover = on_country_hover.clone();
                                            move |e| {
                                                if interactive {
                                                    hovered_country.set(Some(code.clone()));
                                                    if let Some(ref cb) = on_country_hover {
                                                        cb.run(Some(code.clone()));
                                                    }
                                                    if show_tooltip {
                                                        let country_data = data.get();
                                                        let value_str = country_data.iter()
                                                            .find(|s| s.code == code)
                                                            .map(|s| format!("{:.1}", s.value))
                                                            .unwrap_or_else(|| "N/A".to_string());
                                                        tooltip_content.set(Some((
                                                            name.clone(),
                                                            value_str,
                                                            region.clone(),
                                                        )));
                                                        if let Some(rect) = e.target()
                                                            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                                            .and_then(|el| el.closest("svg").ok().flatten())
                                                            .map(|svg| svg.get_bounding_client_rect())
                                                        {
                                                            tooltip_pos.set((
                                                                e.client_x() as f64 - rect.left() + 10.0,
                                                                e.client_y() as f64 - rect.top() - 50.0,
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        on:mouseleave={
                                            let on_country_hover = on_country_hover.clone();
                                            move |_| {
                                                hovered_country.set(None);
                                                tooltip_content.set(None);
                                                if let Some(ref cb) = on_country_hover {
                                                    cb.run(None);
                                                }
                                            }
                                        }
                                        on:click={
                                            let code = code_for_click.clone();
                                            let name_for_zoom = name_for_zoom.clone();
                                            let on_country_click = on_country_click.clone();
                                            let on_selection_change = on_selection_change.clone();
                                            let on_zoom_change = on_zoom_change.clone();
                                            move |e| {
                                                if interactive {
                                                    // Handle zoom
                                                    if enable_zoom {
                                                        // If already zoomed to this country, zoom out
                                                        if zoomed_country.get().as_ref() == Some(&code) {
                                                            zoomed_country.set(None);
                                                            zoomed_country_name.set(None);
                                                            if let Some(ref cb) = on_zoom_change {
                                                                cb.run(None);
                                                            }
                                                        } else {
                                                            // Zoom to this country
                                                            zoomed_country.set(Some(code.clone()));
                                                            zoomed_country_name.set(Some(name_for_zoom.clone()));
                                                            if let Some(ref cb) = on_zoom_change {
                                                                cb.run(Some(code.clone()));
                                                            }
                                                        }
                                                        // Stop propagation so background click doesn't fire
                                                        e.stop_propagation();
                                                    }

                                                    // Handle selection
                                                    if multi_select {
                                                        selected_countries.update(|selected| {
                                                            if selected.contains(&code) {
                                                                selected.retain(|c| c != &code);
                                                            } else {
                                                                selected.push(code.clone());
                                                            }
                                                        });
                                                        if let Some(ref cb) = on_selection_change {
                                                            cb.run(selected_countries.get());
                                                        }
                                                    }
                                                    // Fire click callback
                                                    if let Some(ref cb) = on_country_click {
                                                        let country_data = data.get();
                                                        if let Some(state) = country_data.iter()
                                                            .find(|s| s.code == code)
                                                        {
                                                            cb.run(state.clone());
                                                        } else {
                                                            cb.run(CountryState::new(code.clone(), 0.0));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    />
                                }
                            }).collect_view()
                        }).collect_view()}
                </g>
            </svg>

            // Zoom controls (back button)
            {move || {
                let is_zoomed = zoomed_country.get().is_some();
                (enable_zoom && show_zoom_controls && is_zoomed).then(|| {
                    let country_name = zoomed_country_name.get().unwrap_or_default();
                    view! {
                        <div style="position: absolute; top: 16px; left: 16px; \
                                    display: flex; flex-direction: column; gap: 8px;">
                            // Back button
                            <button
                                style="background: var(--fx-color-bg-elevated, #1f1f1f); \
                                       border: 1px solid var(--fx-color-border, #434343); \
                                       border-radius: 6px; padding: 8px 16px; \
                                       color: var(--fx-color-text, #fff); \
                                       font-size: 13px; cursor: pointer; \
                                       display: flex; align-items: center; gap: 6px; \
                                       transition: background 0.2s;"
                                on:click=zoom_out
                            >
                                // Arrow icon
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M19 12H5M12 19l-7-7 7-7"/>
                                </svg>
                                {"Back to World"}
                            </button>
                            // Country name badge
                            <div style="background: var(--fx-color-primary, #1890ff); \
                                        border-radius: 6px; padding: 6px 12px; \
                                        color: #fff; font-size: 12px; font-weight: 600;">
                                {country_name}
                            </div>
                        </div>
                    }
                })
            }}

            // Tooltip
            {move || {
                tooltip_content.get().map(|(name, value, region)| {
                    let (tx, ty) = tooltip_pos.get();
                    view! {
                        <div style=format!(
                            "position: absolute; left: {}px; top: {}px; \
                             background: var(--fx-color-bg-elevated, #1f1f1f); \
                             border: 1px solid var(--fx-color-border, #434343); \
                             border-radius: 6px; padding: 10px 14px; \
                             pointer-events: none; z-index: 50; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.4); \
                             min-width: 120px;",
                            tx, ty
                        )>
                            <div style="font-size: 14px; font-weight: 600; color: var(--fx-color-text, #fff); margin-bottom: 4px;">
                                {name}
                            </div>
                            <div style="font-size: 12px; color: var(--fx-color-text-secondary, #888); margin-bottom: 4px;">
                                {region}
                            </div>
                            <div style="font-size: 16px; font-weight: 700; color: var(--fx-color-primary, #1890ff);">
                                {value}
                            </div>
                        </div>
                    }
                })
            }}

            // Legend
            {move || {
                // Hide legend when zoomed in to avoid clutter
                let is_zoomed = zoomed_country.get().is_some();
                (show_legend && !is_zoomed).then(|| {
                    let gradient_stops = color_scale_for_legend.gradient_stops();
                    let gradient_css = gradient_stops.iter()
                        .map(|(pos, color)| format!("{} {:.0}%", color, pos * 100.0))
                        .collect::<Vec<_>>()
                        .join(", ");

                    view! {
                        <div style="position: absolute; bottom: 16px; right: 16px; \
                                    background: var(--fx-color-bg-elevated, #1f1f1f); \
                                    border: 1px solid var(--fx-color-border, #434343); \
                                    border-radius: 6px; padding: 12px;">
                            <div style="font-size: 11px; color: var(--fx-color-text-secondary, #888); margin-bottom: 8px;">
                                {"Value Scale"}
                            </div>
                            <div style=format!(
                                "width: 120px; height: 8px; border-radius: 4px; \
                                 background: linear-gradient(90deg, {});",
                                gradient_css
                            ) />
                            <div style="display: flex; justify-content: space-between; margin-top: 4px;">
                                <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                    {format!("{:.0}", min_value)}
                                </span>
                                <span style="font-size: 10px; color: var(--fx-color-text-secondary, #888);">
                                    {format!("{:.0}", max_value)}
                                </span>
                            </div>
                        </div>
                    }
                })
            }}

            // Zoom hint (shown when zoom is enabled but not zoomed)
            {move || {
                let is_zoomed = zoomed_country.get().is_some();
                (enable_zoom && show_zoom_controls && !is_zoomed).then(|| {
                    view! {
                        <div style="position: absolute; top: 16px; left: 16px; \
                                    background: var(--fx-color-bg-elevated, #1f1f1f); \
                                    border: 1px solid var(--fx-color-border, #434343); \
                                    border-radius: 6px; padding: 6px 12px; \
                                    font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                            {"Click a country to zoom"}
                        </div>
                    }
                })
            }}
        </div>
    }
}

/// Standalone legend component for SvgWorldMap.
#[component]
pub fn SvgWorldMapLegend(
    /// Color scale to display.
    color_scale: ColorScale,
    /// Minimum value.
    #[prop(optional, default = 0.0)]
    min_value: f64,
    /// Maximum value.
    #[prop(optional, default = 100.0)]
    max_value: f64,
    /// Legend title.
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

    let prefix = format!("fx-svgmap-legend-{}", design_system);

    let gradient_stops = color_scale.gradient_stops();
    let gradient_css = gradient_stops.iter()
        .map(|(pos, color)| format!("{} {:.0}%", color, pos * 100.0))
        .collect::<Vec<_>>()
        .join(", ");

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

            <div style=format!(
                "width: 150px; height: 10px; border-radius: 4px; \
                 background: linear-gradient(90deg, {});",
                gradient_css
            ) />

            <div style="display: flex; justify-content: space-between; margin-top: 6px;">
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {format!("{:.0}", min_value)}
                </span>
                <span style="font-size: 11px; color: var(--fx-color-text-secondary, #888);">
                    {format!("{:.0}", max_value)}
                </span>
            </div>
        </div>
    }
}
