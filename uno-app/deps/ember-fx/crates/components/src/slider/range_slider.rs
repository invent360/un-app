//! Range Slider component for selecting a range of values.
//!
//! Provides a dual-handle slider for selecting a range (min, max) from a
//! continuous scale. Supports all features of the single Slider plus
//! range-specific features like draggable track and handle collision handling.

use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use wasm_bindgen::JsCast;
use super::types::{
    SliderSize, SliderOrientation, SliderMark,
    TooltipConfig, TooltipVisibility, TooltipPlacement,
    RangeConfig, SliderClassNames, SliderStyles,
};
use crate::try_use_theme;

/// Range Slider component for selecting a range of values.
///
/// # Features
/// - Dual handles for range selection
/// - Draggable track between handles
/// - Handle collision handling (pushable)
/// - Min/max range constraints
/// - All features from single Slider
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::{RangeSlider, RangeConfig};
///
/// let range = RwSignal::new((20.0, 80.0));
///
/// // Basic range slider
/// view! { <RangeSlider value=range /> }
///
/// // With draggable track
/// view! {
///     <RangeSlider
///         value=range
///         range_config=RangeConfig::new().draggable_track(true)
///     />
/// }
/// ```
#[component]
pub fn RangeSlider(
    /// Range values (low, high) via RwSignal.
    #[prop(into)]
    value: RwSignal<(f64, f64)>,
    /// Minimum value (default: 0.0).
    #[prop(optional, into)]
    min: Option<f64>,
    /// Maximum value (default: 100.0).
    #[prop(optional, into)]
    max: Option<f64>,
    /// Step increment (default: 1.0).
    #[prop(optional, into)]
    step: Option<f64>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SliderSize>,
    /// Orientation (horizontal or vertical).
    #[prop(optional, into)]
    orientation: Option<SliderOrientation>,
    /// Reverse direction.
    #[prop(optional)]
    reverse: bool,
    /// Mark points with labels.
    #[prop(optional, into)]
    marks: Option<Vec<SliderMark>>,
    /// Show dots at step intervals.
    #[prop(optional)]
    dots: bool,
    /// Range configuration (draggable track, constraints).
    #[prop(optional, into)]
    range_config: Option<RangeConfig>,
    /// Tooltip configuration.
    #[prop(optional, into)]
    tooltip: Option<TooltipConfig>,
    /// Format function for value display.
    #[prop(optional, into)]
    format: Option<Callback<f64, String>>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Semantic class name overrides.
    #[prop(optional, into)]
    class_names: Option<SliderClassNames>,
    /// Semantic style overrides.
    #[prop(optional, into)]
    styles: Option<SliderStyles>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback (fires during drag).
    #[prop(optional, into)]
    on_change: Option<Callback<(f64, f64)>>,
    /// Change complete callback (fires on mouse/touch up).
    #[prop(optional, into)]
    on_change_complete: Option<Callback<(f64, f64)>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(100.0);
    let step = step.unwrap_or(1.0);
    let size = size.unwrap_or_default();
    let orientation = orientation.unwrap_or_default();
    let range_config = range_config.unwrap_or_default();
    let tooltip_config = tooltip.unwrap_or_default();
    let class_names = class_names.unwrap_or_default();
    let styles = styles.unwrap_or_default();

    let is_vertical = orientation.is_vertical();

    // CSS prefix
    let slider_prefix = format!("fx-slider-{}", design_system);
    let size_class = size.class(&slider_prefix);
    let orientation_class = orientation.class(&slider_prefix);

    // Clone prefixes for closures
    let prefix_for_root = slider_prefix.clone();
    let prefix_for_rail = slider_prefix.clone();
    let prefix_for_track = slider_prefix.clone();
    let prefix_for_handle_low = slider_prefix.clone();
    let prefix_for_handle_high = slider_prefix.clone();
    let prefix_for_tooltip_low = slider_prefix.clone();
    let prefix_for_tooltip_high = slider_prefix.clone();
    let prefix_for_marks = slider_prefix.clone();

    // Calculate percentages
    let percentage_low = Memo::new(move |_| {
        let (low, _) = value.get();
        let pct = ((low - min) / (max - min) * 100.0).clamp(0.0, 100.0);
        if reverse { 100.0 - pct } else { pct }
    });

    let percentage_high = Memo::new(move |_| {
        let (_, high) = value.get();
        let pct = ((high - min) / (max - min) * 100.0).clamp(0.0, 100.0);
        if reverse { 100.0 - pct } else { pct }
    });

    // Format value display
    let format_value = move |v: f64| {
        if let Some(ref fmt) = format {
            fmt.run(v)
        } else {
            format!("{:.0}", v)
        }
    };

    // State for which handle is being dragged (0 = low, 1 = high, 2 = track)
    let active_handle = RwSignal::new(Option::<usize>::None);
    let is_focused_low = RwSignal::new(false);
    let is_focused_high = RwSignal::new(false);
    let is_hovering = RwSignal::new(false);
    let track_drag_start = RwSignal::new(Option::<(f64, f64, f64)>::None); // (start_ratio, low, high)

    // Tooltip visibility
    let tooltip_visible_low = move || {
        match tooltip_config.visible {
            TooltipVisibility::Always => true,
            TooltipVisibility::Never => false,
            TooltipVisibility::Hover => {
                active_handle.get() == Some(0) || is_focused_low.get() || is_hovering.get()
            }
        }
    };

    let tooltip_visible_high = move || {
        match tooltip_config.visible {
            TooltipVisibility::Always => true,
            TooltipVisibility::Never => false,
            TooltipVisibility::Hover => {
                active_handle.get() == Some(1) || is_focused_high.get() || is_hovering.get()
            }
        }
    };

    // Tooltip placement
    let tooltip_placement = tooltip_config.placement.unwrap_or_else(|| {
        TooltipPlacement::default_for_orientation(
            if is_vertical { SliderOrientation::Vertical } else { SliderOrientation::Horizontal }
        )
    });

    // Update value with constraints
    let update_value = move |new_low: f64, new_high: f64, trigger_change: bool| {
        let mut low = (new_low / step).round() * step;
        let mut high = (new_high / step).round() * step;

        // Clamp to min/max
        low = low.clamp(min, max);
        high = high.clamp(min, max);

        // Apply range constraints
        if let Some(min_range) = range_config.min_range {
            if high - low < min_range {
                // Don't allow values closer than min_range
                return;
            }
        }
        if let Some(max_range) = range_config.max_range {
            if high - low > max_range {
                // Don't allow values further than max_range
                return;
            }
        }

        // Ensure low <= high
        if low > high {
            if range_config.pushable {
                // Swap if pushable
                std::mem::swap(&mut low, &mut high);
            } else {
                // Block the change
                return;
            }
        }

        value.set((low, high));
        if trigger_change {
            if let Some(ref cb) = on_change {
                cb.run((low, high));
            }
        }
    };

    // Handle move calculation
    let calculate_ratio = move |client_x: i32, client_y: i32, rect: &web_sys::DomRect| -> f64 {
        if is_vertical {
            let height = rect.height();
            let y = client_y as f64 - rect.top();
            let r = 1.0 - (y / height).clamp(0.0, 1.0);
            if reverse { 1.0 - r } else { r }
        } else {
            let width = rect.width();
            let x = client_x as f64 - rect.left();
            let r = (x / width).clamp(0.0, 1.0);
            if reverse { 1.0 - r } else { r }
        }
    };

    let handle_move = move |client_x: i32, client_y: i32, element: web_sys::Element, handle_idx: Option<usize>| {
        if disabled {
            return;
        }

        let rect = element.get_bounding_client_rect();
        let ratio = calculate_ratio(client_x, client_y, &rect);
        let new_value = min + ratio * (max - min);

        let (low, high) = value.get_untracked();

        match handle_idx {
            Some(0) => {
                // Low handle
                let clamped = new_value.clamp(min, if range_config.pushable { max } else { high });
                if range_config.pushable && clamped > high {
                    update_value(high, clamped, true);
                } else {
                    update_value(clamped, high, true);
                }
            }
            Some(1) => {
                // High handle
                let clamped = new_value.clamp(if range_config.pushable { min } else { low }, max);
                if range_config.pushable && clamped < low {
                    update_value(clamped, low, true);
                } else {
                    update_value(low, clamped, true);
                }
            }
            Some(2) => {
                // Track drag
                if let Some((start_ratio, start_low, start_high)) = track_drag_start.get_untracked() {
                    let delta = (ratio - start_ratio) * (max - min);
                    let new_low = (start_low + delta).clamp(min, max - (start_high - start_low));
                    let new_high = (start_high + delta).clamp(min + (start_high - start_low), max);
                    update_value(new_low, new_high, true);
                }
            }
            None => {
                // Click on rail - move closest handle
                let dist_to_low = (new_value - low).abs();
                let dist_to_high = (new_value - high).abs();
                if dist_to_low <= dist_to_high {
                    update_value(new_value, high, true);
                    active_handle.set(Some(0));
                } else {
                    update_value(low, new_value, true);
                    active_handle.set(Some(1));
                }
            }
            _ => {} // Ignore other values
        }
    };

    // Root class
    let combined_class = {
        let prefix = prefix_for_root.clone();
        let class_clone = class.clone();
        let root_class = class_names.root.clone();
        move || {
            let mut parts = vec![prefix.clone(), format!("{}-range", prefix)];
            if !size_class.is_empty() {
                parts.push(size_class.clone());
            }
            if !orientation_class.is_empty() {
                parts.push(orientation_class.clone());
            }
            if disabled {
                parts.push(format!("{}-disabled", prefix));
            }
            if reverse {
                parts.push(format!("{}-reverse", prefix));
            }
            if let Some(ref custom) = class_clone {
                parts.push(custom.clone());
            }
            if let Some(ref root) = root_class {
                parts.push(root.clone());
            }
            parts.join(" ")
        }
    };

    // Rail class
    let rail_class = {
        let prefix = prefix_for_rail.clone();
        let custom = class_names.rail.clone();
        move || {
            let mut c = format!("{}-rail", prefix);
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Track class
    let track_class = {
        let prefix = prefix_for_track.clone();
        let draggable = range_config.draggable_track;
        let custom = class_names.track.clone();
        move || {
            let mut c = format!("{}-track", prefix);
            if draggable {
                c.push_str(&format!(" {}-track-draggable", prefix));
            }
            if active_handle.get() == Some(2) {
                c.push_str(&format!(" {}-track-dragging", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Track style
    let track_style = {
        let custom = styles.track.clone();
        move || {
            let low_pct = percentage_low.get();
            let high_pct = percentage_high.get();
            let start = low_pct.min(high_pct);
            let width = (high_pct - low_pct).abs();

            let mut s = if is_vertical {
                format!("bottom: {}%; height: {}%;", start, width)
            } else {
                format!("left: {}%; width: {}%;", start, width)
            };
            if let Some(ref cs) = custom {
                s.push(' ');
                s.push_str(cs);
            }
            s
        }
    };

    // Handle classes
    let handle_class_low = {
        let prefix = prefix_for_handle_low.clone();
        let custom = class_names.handle.clone();
        move || {
            let mut c = format!("{}-handle {}-handle-start", prefix, prefix);
            if active_handle.get() == Some(0) {
                c.push_str(&format!(" {}-handle-active", prefix));
            }
            if is_focused_low.get() {
                c.push_str(&format!(" {}-handle-focused", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    let handle_class_high = {
        let prefix = prefix_for_handle_high.clone();
        let custom = class_names.handle.clone();
        move || {
            let mut c = format!("{}-handle {}-handle-end", prefix, prefix);
            if active_handle.get() == Some(1) {
                c.push_str(&format!(" {}-handle-active", prefix));
            }
            if is_focused_high.get() {
                c.push_str(&format!(" {}-handle-focused", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Handle styles
    let handle_style_low = {
        let custom = styles.handle.clone();
        move || {
            let pct = percentage_low.get();
            let mut s = if is_vertical {
                format!("bottom: {}%;", pct)
            } else {
                format!("left: {}%;", pct)
            };
            if let Some(ref cs) = custom {
                s.push(' ');
                s.push_str(cs);
            }
            s
        }
    };

    let handle_style_high = {
        let custom = styles.handle.clone();
        move || {
            let pct = percentage_high.get();
            let mut s = if is_vertical {
                format!("bottom: {}%;", pct)
            } else {
                format!("left: {}%;", pct)
            };
            if let Some(ref cs) = custom {
                s.push(' ');
                s.push_str(cs);
            }
            s
        }
    };

    // Tooltip classes
    let tooltip_class_low = {
        let prefix = prefix_for_tooltip_low.clone();
        let placement = tooltip_placement;
        let custom = class_names.tooltip.clone();
        move || {
            let mut c = format!("{}-tooltip {}", prefix, placement.class(&format!("{}-tooltip", prefix)));
            if tooltip_visible_low() {
                c.push_str(&format!(" {}-tooltip-visible", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    let tooltip_class_high = {
        let prefix = prefix_for_tooltip_high.clone();
        let placement = tooltip_placement;
        let custom = class_names.tooltip.clone();
        move || {
            let mut c = format!("{}-tooltip {}", prefix, placement.class(&format!("{}-tooltip", prefix)));
            if tooltip_visible_high() {
                c.push_str(&format!(" {}-tooltip-visible", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Keyboard handler for handles
    let handle_keydown = move |ev: KeyboardEvent, handle_idx: usize| {
        if disabled {
            return;
        }

        let key = ev.key();
        let large_step = step * 10.0;
        let (low, high) = value.get();

        let (current, other) = if handle_idx == 0 {
            (low, high)
        } else {
            (high, low)
        };

        let delta = match key.as_str() {
            "ArrowRight" | "ArrowUp" => {
                ev.prevent_default();
                if reverse { -step } else { step }
            }
            "ArrowLeft" | "ArrowDown" => {
                ev.prevent_default();
                if reverse { step } else { -step }
            }
            "PageUp" => {
                ev.prevent_default();
                if reverse { -large_step } else { large_step }
            }
            "PageDown" => {
                ev.prevent_default();
                if reverse { large_step } else { -large_step }
            }
            "Home" => {
                ev.prevent_default();
                min - current
            }
            "End" => {
                ev.prevent_default();
                max - current
            }
            _ => return,
        };

        let new_val = (current + delta).clamp(min, max);

        if handle_idx == 0 {
            if range_config.pushable && new_val > other {
                update_value(other, new_val, true);
            } else {
                update_value(new_val.min(other), high, true);
            }
        } else if range_config.pushable && new_val < other {
            update_value(new_val, other, true);
        } else {
            update_value(low, new_val.max(other), true);
        }
    };

    view! {
        <div class=combined_class style=styles.root.clone()>
            <div
                class=rail_class
                style=styles.rail.clone()
                on:mouseenter=move |_| is_hovering.set(true)
                on:mouseleave=move |_| {
                    is_hovering.set(false);
                    if active_handle.get_untracked().is_some() {
                        active_handle.set(None);
                        track_drag_start.set(None);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:mousedown=move |ev| {
                    if disabled { return; }
                    if let Some(target) = ev.current_target() {
                        if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                            handle_move(ev.client_x(), ev.client_y(), element, None);
                        }
                    }
                }
                on:mousemove=move |ev| {
                    let handle = active_handle.get_untracked();
                    if handle.is_none() { return; }
                    if let Some(target) = ev.current_target() {
                        if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                            handle_move(ev.client_x(), ev.client_y(), element, handle);
                        }
                    }
                }
                on:mouseup=move |_| {
                    if active_handle.get_untracked().is_some() {
                        active_handle.set(None);
                        track_drag_start.set(None);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:touchstart=move |ev: web_sys::TouchEvent| {
                    if disabled { return; }
                    ev.prevent_default();
                    if let Some(touch) = ev.touches().get(0) {
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                handle_move(touch.client_x(), touch.client_y(), element, None);
                            }
                        }
                    }
                }
                on:touchmove=move |ev: web_sys::TouchEvent| {
                    let handle = active_handle.get_untracked();
                    if handle.is_none() { return; }
                    ev.prevent_default();
                    if let Some(touch) = ev.touches().get(0) {
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                handle_move(touch.client_x(), touch.client_y(), element, handle);
                            }
                        }
                    }
                }
                on:touchend=move |_: web_sys::TouchEvent| {
                    if active_handle.get_untracked().is_some() {
                        active_handle.set(None);
                        track_drag_start.set(None);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:touchcancel=move |_: web_sys::TouchEvent| {
                    active_handle.set(None);
                    track_drag_start.set(None);
                }
            >
                // Track between handles
                <div
                    class=track_class
                    style=track_style
                    on:mousedown=move |ev| {
                        if disabled || !range_config.draggable_track { return; }
                        ev.stop_propagation();
                        active_handle.set(Some(2));
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                let rect = element.get_bounding_client_rect();
                                let ratio = calculate_ratio(ev.client_x(), ev.client_y(), &rect);
                                let (low, high) = value.get_untracked();
                                track_drag_start.set(Some((ratio, low, high)));
                            }
                        }
                    }
                />

                // Low handle
                <div
                    class=handle_class_low
                    style=handle_style_low
                    tabindex=if disabled { "-1" } else { "0" }
                    role="slider"
                    aria-valuemin=min
                    aria-valuemax=move || value.get().1
                    aria-valuenow=move || value.get().0
                    aria-orientation=if is_vertical { "vertical" } else { "horizontal" }
                    aria-disabled=disabled
                    aria-label="Range minimum"
                    on:mousedown=move |ev| {
                        if disabled { return; }
                        ev.stop_propagation();
                        active_handle.set(Some(0));
                    }
                    on:touchstart=move |ev: web_sys::TouchEvent| {
                        if disabled { return; }
                        ev.stop_propagation();
                        active_handle.set(Some(0));
                    }
                    on:focus=move |_| is_focused_low.set(true)
                    on:blur=move |_| is_focused_low.set(false)
                    on:keydown=move |ev| handle_keydown(ev, 0)
                >
                    {if tooltip_config.visible != TooltipVisibility::Never {
                        let format_value = format_value.clone();
                        let tooltip_style = styles.tooltip.clone();
                        Some(view! {
                            <div class=tooltip_class_low style=tooltip_style>
                                {move || format_value(value.get().0)}
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>

                // High handle
                <div
                    class=handle_class_high
                    style=handle_style_high
                    tabindex=if disabled { "-1" } else { "0" }
                    role="slider"
                    aria-valuemin=move || value.get().0
                    aria-valuemax=max
                    aria-valuenow=move || value.get().1
                    aria-orientation=if is_vertical { "vertical" } else { "horizontal" }
                    aria-disabled=disabled
                    aria-label="Range maximum"
                    on:mousedown=move |ev| {
                        if disabled { return; }
                        ev.stop_propagation();
                        active_handle.set(Some(1));
                    }
                    on:touchstart=move |ev: web_sys::TouchEvent| {
                        if disabled { return; }
                        ev.stop_propagation();
                        active_handle.set(Some(1));
                    }
                    on:focus=move |_| is_focused_high.set(true)
                    on:blur=move |_| is_focused_high.set(false)
                    on:keydown=move |ev| handle_keydown(ev, 1)
                >
                    {if tooltip_config.visible != TooltipVisibility::Never {
                        let format_value = format_value.clone();
                        let tooltip_style = styles.tooltip.clone();
                        Some(view! {
                            <div class=tooltip_class_high style=tooltip_style>
                                {move || format_value(value.get().1)}
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>

            // Marks
            {marks.map(|marks_vec| {
                let prefix = prefix_for_marks.clone();
                let mark_class = class_names.mark.clone();
                let marks_container_style = styles.marks.clone();
                view! {
                    <div class=format!("{}-marks", prefix) style=marks_container_style.clone()>
                        {marks_vec.iter().map(|mark| {
                            let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                            let pos = if reverse { 100.0 - pos } else { pos };
                            let prefix = prefix.clone();
                            let mark_value = mark.value;
                            let is_in_range = move || {
                                let (low, high) = value.get();
                                mark_value >= low && mark_value <= high
                            };
                            let mark_class = mark_class.clone();
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-mark", prefix);
                                        if is_in_range() {
                                            c.push_str(&format!(" {}-mark-active", prefix));
                                        }
                                        if let Some(ref mc) = mark_class {
                                            c.push(' ');
                                            c.push_str(mc);
                                        }
                                        c
                                    }
                                    style=if is_vertical {
                                        format!("bottom: {}%;", pos)
                                    } else {
                                        format!("left: {}%;", pos)
                                    }
                                >
                                    {mark.label.clone()}
                                </span>
                            }
                        }).collect_view()}

                        // Mark dots
                        {marks_vec.iter().map(|mark| {
                            let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                            let pos = if reverse { 100.0 - pos } else { pos };
                            let prefix = prefix.clone();
                            let mark_value = mark.value;
                            let is_in_range = move || {
                                let (low, high) = value.get();
                                mark_value >= low && mark_value <= high
                            };
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-dot", prefix);
                                        if is_in_range() {
                                            c.push_str(&format!(" {}-dot-active", prefix));
                                        }
                                        c
                                    }
                                    style=if is_vertical {
                                        format!("bottom: {}%;", pos)
                                    } else {
                                        format!("left: {}%;", pos)
                                    }
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            })}
        </div>
    }
}
