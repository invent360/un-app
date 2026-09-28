//! Slider component for selecting a value from a range.
//!
//! Provides both a native HTML range input variant and a custom div-based
//! slider for advanced styling. The custom variant supports marks, tooltips,
//! keyboard navigation, and full accessibility.

use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use leptos::tachys::view::any_view::AnyView;
use wasm_bindgen::JsCast;
use super::types::{
    SliderSize, SliderOrientation, SliderMark,
    TooltipConfig, TooltipVisibility, TooltipPlacement,
    SliderClassNames, SliderStyles,
};
use crate::try_use_theme;

/// Slider variant type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SliderVariant {
    /// Native HTML range input (recommended for basic use).
    Native,
    /// Custom div-based slider for advanced styling.
    #[default]
    Custom,
}

/// Slider component for selecting a value from a range.
///
/// # Features
/// - Single value selection
/// - Step increment support
/// - Marks with labels
/// - Dots at step intervals
/// - Horizontal and vertical orientation
/// - Reverse direction
/// - Customizable tooltip
/// - Keyboard navigation (Arrow, PageUp/Down, Home/End)
/// - Touch support
/// - ARIA accessibility
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::{Slider, SliderMark};
///
/// let value = RwSignal::new(30.0);
///
/// // Basic slider
/// view! { <Slider value=value /> }
///
/// // With marks
/// let marks = vec![
///     SliderMark::new(0.0, "0%"),
///     SliderMark::new(50.0, "50%"),
///     SliderMark::new(100.0, "100%"),
/// ];
/// view! { <Slider value=value marks=marks /> }
/// ```
#[component]
pub fn Slider(
    /// Current value (two-way binding via RwSignal).
    #[prop(into)]
    value: RwSignal<f64>,
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
    /// Fill track from min to value (default: true).
    #[prop(optional)]
    included: Option<bool>,
    /// Mark points with labels.
    #[prop(optional, into)]
    marks: Option<Vec<SliderMark>>,
    /// Show dots at step intervals.
    #[prop(optional)]
    dots: bool,
    /// Tooltip configuration.
    #[prop(optional, into)]
    tooltip: Option<TooltipConfig>,
    /// Format function for value display.
    #[prop(optional, into)]
    format: Option<Callback<f64, String>>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Slider rendering variant (Native or Custom).
    #[prop(optional)]
    variant: Option<SliderVariant>,
    /// Show value text next to slider.
    #[prop(optional)]
    show_value: bool,
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
    on_change: Option<Callback<f64>>,
    /// Change complete callback (fires on mouse/touch up).
    #[prop(optional, into)]
    on_change_complete: Option<Callback<f64>>,
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
    let included = included.unwrap_or(true);
    let variant = variant.unwrap_or_default();
    let tooltip_config = tooltip.unwrap_or_default();
    let class_names = class_names.unwrap_or_default();
    let styles = styles.unwrap_or_default();

    let is_vertical = orientation.is_vertical();

    // Build CSS prefix
    let slider_prefix = format!("fx-slider-{}", design_system);
    let size_class = size.class(&slider_prefix);
    let orientation_class = orientation.class(&slider_prefix);

    // Calculate percentage for CSS (memoized)
    let percentage = Memo::new(move |_| {
        let val = value.get();
        let pct = ((val - min) / (max - min) * 100.0).clamp(0.0, 100.0);
        if reverse { 100.0 - pct } else { pct }
    });

    // Generate step dots if enabled
    let step_dots = if dots && step > 0.0 {
        let mut dots_vec = Vec::new();
        let mut v = min;
        while v <= max {
            dots_vec.push(v);
            v += step;
        }
        Some(dots_vec)
    } else {
        None
    };

    match variant {
        SliderVariant::Native => {
            render_native_slider(
                value,
                min, max, step,
                disabled, reverse, is_vertical,
                show_value,
                slider_prefix, size_class,
                class, format, percentage,
                on_change, on_change_complete,
            )
        }
        SliderVariant::Custom => {
            render_custom_slider(
                value,
                min, max, step,
                disabled, reverse, is_vertical, included,
                show_value,
                marks, step_dots,
                tooltip_config,
                slider_prefix, size_class, orientation_class,
                class, class_names, styles,
                format, percentage,
                on_change, on_change_complete,
            )
        }
    }
}

/// Render native HTML range input slider.
#[allow(clippy::too_many_arguments)]
fn render_native_slider(
    value: RwSignal<f64>,
    min: f64, max: f64, step: f64,
    disabled: bool, reverse: bool, is_vertical: bool,
    show_value: bool,
    slider_prefix: String, size_class: String,
    class: Option<String>,
    format: Option<Callback<f64, String>>,
    percentage: Memo<f64>,
    on_change: Option<Callback<f64>>,
    on_change_complete: Option<Callback<f64>>,
) -> AnyView {
    let slider_prefix_clone = slider_prefix.clone();
    let slider_prefix_for_value = slider_prefix.clone();
    let class_clone = class.clone();

    let native_class = move || {
        let mut parts = vec![
            slider_prefix_clone.clone(),
            format!("{}-native", slider_prefix_clone),
        ];
        if !size_class.is_empty() {
            parts.push(size_class.clone());
        }
        if disabled {
            parts.push(format!("{}-disabled", slider_prefix_clone));
        }
        if reverse {
            parts.push(format!("{}-reverse", slider_prefix_clone));
        }
        if is_vertical {
            parts.push(format!("{}-vertical", slider_prefix_clone));
        }
        if let Some(ref custom) = class_clone {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Create format closure
    let format_value = move |v: f64| {
        if let Some(ref fmt) = format {
            fmt.run(v)
        } else {
            format!("{:.0}", v)
        }
    };

    let value_display = if show_value {
        view! {
            <span class=format!("{}-value", slider_prefix_for_value)>
                {move || format_value(value.get())}
            </span>
        }.into_any()
    } else {
        view! {}.into_any()
    };

    view! {
        <div class=move || format!("{}-wrapper", slider_prefix)>
            <input
                type="range"
                class=native_class
                min=min
                max=max
                step=step
                disabled=disabled
                prop:value=move || value.get()
                style=move || format!("--slider-value: {}%", percentage.get())
                on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        let stepped = (val / step).round() * step;
                        let clamped = stepped.clamp(min, max);
                        value.set(clamped);
                        if let Some(ref cb) = on_change {
                            cb.run(clamped);
                        }
                    }
                }
                on:change=move |_| {
                    if let Some(ref cb) = on_change_complete {
                        cb.run(value.get());
                    }
                }
            />
            {value_display}
        </div>
    }.into_any()
}

/// Render custom div-based slider.
#[allow(clippy::too_many_arguments)]
fn render_custom_slider(
    value: RwSignal<f64>,
    min: f64, max: f64, step: f64,
    disabled: bool, reverse: bool, is_vertical: bool, included: bool,
    show_value: bool,
    marks: Option<Vec<SliderMark>>,
    step_dots: Option<Vec<f64>>,
    tooltip_config: TooltipConfig,
    slider_prefix: String, size_class: String, orientation_class: String,
    class: Option<String>,
    class_names: SliderClassNames,
    styles: SliderStyles,
    format: Option<Callback<f64, String>>,
    percentage: Memo<f64>,
    on_change: Option<Callback<f64>>,
    on_change_complete: Option<Callback<f64>>,
) -> AnyView {
    // Create format closure
    let format_value = {
        let format = format.clone();
        move |v: f64| {
            if let Some(ref fmt) = format {
                fmt.run(v)
            } else {
                format!("{:.0}", v)
            }
        }
    };
    // Clone prefixes for closures
    let prefix_for_root = slider_prefix.clone();
    let prefix_for_rail = slider_prefix.clone();
    let prefix_for_track = slider_prefix.clone();
    let prefix_for_handle = slider_prefix.clone();
    let prefix_for_tooltip = slider_prefix.clone();
    let prefix_for_marks = slider_prefix.clone();
    let prefix_for_dots = slider_prefix.clone();
    let prefix_for_value = slider_prefix.clone();

    // Dragging state
    let is_dragging = RwSignal::new(false);
    let is_focused = RwSignal::new(false);
    let is_hovering = RwSignal::new(false);

    // Tooltip visibility
    let tooltip_visible = move || {
        match tooltip_config.visible {
            TooltipVisibility::Always => true,
            TooltipVisibility::Never => false,
            TooltipVisibility::Hover => is_dragging.get() || is_focused.get() || is_hovering.get(),
        }
    };

    // Tooltip placement
    let tooltip_placement = tooltip_config.placement.unwrap_or_else(|| {
        TooltipPlacement::default_for_orientation(
            if is_vertical { SliderOrientation::Vertical } else { SliderOrientation::Horizontal }
        )
    });

    // Handle mouse/touch move calculation
    let handle_move = move |client_x: i32, client_y: i32, element: web_sys::Element| {
        if disabled {
            return;
        }

        let rect = element.get_bounding_client_rect();
        let ratio = if is_vertical {
            let height = rect.height();
            let y = client_y as f64 - rect.top();
            let r = 1.0 - (y / height).clamp(0.0, 1.0);
            if reverse { 1.0 - r } else { r }
        } else {
            let width = rect.width();
            let x = client_x as f64 - rect.left();
            let r = (x / width).clamp(0.0, 1.0);
            if reverse { 1.0 - r } else { r }
        };

        let new_value = min + ratio * (max - min);
        let stepped_value = (new_value / step).round() * step;
        let clamped_value = stepped_value.clamp(min, max);

        value.set(clamped_value);
        if let Some(ref cb) = on_change {
            cb.run(clamped_value);
        }
    };

    // Root class builder
    let combined_class = {
        let prefix = prefix_for_root.clone();
        let class_clone = class.clone();
        let root_class = class_names.root.clone();
        move || {
            let mut parts = vec![prefix.clone()];
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

    // Root style
    let root_style = {
        let custom_style = styles.root.clone();
        move || {
            let mut s = format!("--slider-value: {}%;", percentage.get());
            if let Some(ref cs) = custom_style {
                s.push(' ');
                s.push_str(cs);
            }
            s
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
        let custom = class_names.track.clone();
        move || {
            let mut c = format!("{}-track", prefix);
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Handle class
    let handle_class = {
        let prefix = prefix_for_handle.clone();
        let custom = class_names.handle.clone();
        move || {
            let mut c = format!("{}-handle", prefix);
            if is_dragging.get_untracked() {
                c.push_str(&format!(" {}-handle-dragging", prefix));
            }
            if is_focused.get() {
                c.push_str(&format!(" {}-handle-focused", prefix));
            }
            if let Some(ref cc) = custom {
                c.push(' ');
                c.push_str(cc);
            }
            c
        }
    };

    // Tooltip class
    let tooltip_class = {
        let prefix = prefix_for_tooltip.clone();
        let placement = tooltip_placement;
        let custom = class_names.tooltip.clone();
        move || {
            let mut c = format!("{}-tooltip", prefix);
            c.push_str(&format!(" {}", placement.class(&format!("{}-tooltip", prefix))));
            if tooltip_visible() {
                c.push_str(&format!(" {}-tooltip-visible", prefix));
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
            let pct = percentage.get();
            let mut s = if is_vertical {
                if included {
                    format!("height: {}%;", pct)
                } else {
                    "height: 0%;".to_string()
                }
            } else if included {
                format!("width: {}%;", pct)
            } else {
                "width: 0%;".to_string()
            };
            if let Some(ref cs) = custom {
                s.push(' ');
                s.push_str(cs);
            }
            s
        }
    };

    // Handle style - only set position, let CSS handle centering
    let handle_style = {
        let custom = styles.handle.clone();
        move || {
            let pct = percentage.get();
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

    view! {
        <div
            class=combined_class
            style=root_style
        >
            <div
                class=rail_class
                style=styles.rail.clone()
                on:mouseenter=move |_| is_hovering.set(true)
                on:mouseleave=move |_| {
                    is_hovering.set(false);
                    if is_dragging.get_untracked() {
                        is_dragging.set(false);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:mousedown=move |ev| {
                    if disabled { return; }
                    is_dragging.set(true);
                    if let Some(target) = ev.current_target() {
                        if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                            handle_move(ev.client_x(), ev.client_y(), element);
                        }
                    }
                }
                on:mousemove=move |ev| {
                    if !is_dragging.get_untracked() { return; }
                    if let Some(target) = ev.current_target() {
                        if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                            handle_move(ev.client_x(), ev.client_y(), element);
                        }
                    }
                }
                on:mouseup=move |_| {
                    if is_dragging.get_untracked() {
                        is_dragging.set(false);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:touchstart=move |ev: web_sys::TouchEvent| {
                    if disabled { return; }
                    ev.prevent_default();
                    is_dragging.set(true);
                    if let Some(touch) = ev.touches().get(0) {
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                handle_move(touch.client_x(), touch.client_y(), element);
                            }
                        }
                    }
                }
                on:touchmove=move |ev: web_sys::TouchEvent| {
                    if !is_dragging.get_untracked() { return; }
                    ev.prevent_default();
                    if let Some(touch) = ev.touches().get(0) {
                        if let Some(target) = ev.current_target() {
                            if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                handle_move(touch.client_x(), touch.client_y(), element);
                            }
                        }
                    }
                }
                on:touchend=move |_: web_sys::TouchEvent| {
                    if is_dragging.get_untracked() {
                        is_dragging.set(false);
                        if let Some(ref cb) = on_change_complete {
                            cb.run(value.get());
                        }
                    }
                }
                on:touchcancel=move |_: web_sys::TouchEvent| {
                    is_dragging.set(false);
                }
            >
                // Track (filled portion)
                <div
                    class=track_class
                    style=track_style
                />

                // Handle (draggable thumb)
                <div
                    class=handle_class
                    style=handle_style
                    tabindex=if disabled { "-1" } else { "0" }
                    role="slider"
                    aria-valuemin=min
                    aria-valuemax=max
                    aria-valuenow=move || value.get()
                    aria-orientation=if is_vertical { "vertical" } else { "horizontal" }
                    aria-disabled=disabled
                    on:focus=move |_| is_focused.set(true)
                    on:blur=move |_| is_focused.set(false)
                    on:keydown=move |ev: KeyboardEvent| {
                        if disabled { return; }

                        let key = ev.key();
                        let large_step = step * 10.0;

                        let new_value = match key.as_str() {
                            "ArrowRight" | "ArrowUp" => {
                                ev.prevent_default();
                                let delta = if reverse { -step } else { step };
                                Some((value.get() + delta).clamp(min, max))
                            }
                            "ArrowLeft" | "ArrowDown" => {
                                ev.prevent_default();
                                let delta = if reverse { step } else { -step };
                                Some((value.get() + delta).clamp(min, max))
                            }
                            "PageUp" => {
                                ev.prevent_default();
                                let delta = if reverse { -large_step } else { large_step };
                                Some((value.get() + delta).clamp(min, max))
                            }
                            "PageDown" => {
                                ev.prevent_default();
                                let delta = if reverse { large_step } else { -large_step };
                                Some((value.get() + delta).clamp(min, max))
                            }
                            "Home" => {
                                ev.prevent_default();
                                Some(min)
                            }
                            "End" => {
                                ev.prevent_default();
                                Some(max)
                            }
                            _ => None,
                        };

                        if let Some(new_val) = new_value {
                            let stepped = (new_val / step).round() * step;
                            let clamped = stepped.clamp(min, max);
                            value.set(clamped);
                            if let Some(ref cb) = on_change {
                                cb.run(clamped);
                            }
                        }
                    }
                >
                    // Tooltip
                    {if tooltip_config.visible != TooltipVisibility::Never {
                        let format_value = format_value.clone();
                        let tooltip_style = styles.tooltip.clone();
                        Some(view! {
                            <div
                                class=tooltip_class
                                style=tooltip_style
                            >
                                {move || format_value(value.get())}
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>
            </div>

            // Step dots
            {step_dots.map(|dots| {
                let prefix = prefix_for_dots.clone();
                let dot_class = class_names.dot.clone();
                let dot_style = styles.dot.clone();
                view! {
                    <div class=format!("{}-dots", prefix)>
                        {dots.iter().map(|&dot_val| {
                            let pos = ((dot_val - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                            let pos = if reverse { 100.0 - pos } else { pos };
                            let prefix = prefix.clone();
                            let is_active = move || value.get() >= dot_val;
                            let dot_class = dot_class.clone();
                            let dot_style = dot_style.clone();
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-dot-step", prefix);
                                        if is_active() {
                                            c.push_str(&format!(" {}-dot-step-active", prefix));
                                        }
                                        if let Some(ref dc) = dot_class {
                                            c.push(' ');
                                            c.push_str(dc);
                                        }
                                        c
                                    }
                                    style=move || {
                                        let mut s = if is_vertical {
                                            format!("bottom: {}%;", pos)
                                        } else {
                                            format!("left: {}%;", pos)
                                        };
                                        if let Some(ref ds) = dot_style {
                                            s.push(' ');
                                            s.push_str(ds);
                                        }
                                        s
                                    }
                                />
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            // Marks
            {marks.map(|marks_vec| {
                let prefix = prefix_for_marks.clone();
                let mark_class = class_names.mark.clone();
                let marks_container_class = class_names.marks.clone();
                let marks_container_style = styles.marks.clone();
                let mark_style_override = styles.mark.clone();
                view! {
                    <div
                        class=move || {
                            let mut c = format!("{}-marks", prefix);
                            if let Some(ref mc) = marks_container_class {
                                c.push(' ');
                                c.push_str(mc);
                            }
                            c
                        }
                        style=marks_container_style.clone()
                    >
                        // Mark labels
                        {marks_vec.iter().map(|mark| {
                            let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                            let pos = if reverse { 100.0 - pos } else { pos };
                            let prefix = prefix.clone();
                            let mark_value = mark.value;
                            let is_active = move || value.get() >= mark_value;
                            let mark_class = mark_class.clone();
                            let mark_custom_class = mark.class.clone();
                            let mark_custom_style = mark.style.clone();
                            let mark_style_override = mark_style_override.clone();
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-mark", prefix);
                                        if is_active() {
                                            c.push_str(&format!(" {}-mark-active", prefix));
                                        }
                                        if let Some(ref mc) = mark_class {
                                            c.push(' ');
                                            c.push_str(mc);
                                        }
                                        if let Some(ref mcc) = mark_custom_class {
                                            c.push(' ');
                                            c.push_str(mcc);
                                        }
                                        c
                                    }
                                    style=move || {
                                        let mut s = if is_vertical {
                                            format!("bottom: {}%;", pos)
                                        } else {
                                            format!("left: {}%;", pos)
                                        };
                                        if let Some(ref ms) = mark_style_override {
                                            s.push(' ');
                                            s.push_str(ms);
                                        }
                                        if let Some(ref mcs) = mark_custom_style {
                                            s.push(' ');
                                            s.push_str(mcs);
                                        }
                                        s
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
                            let is_active = move || value.get() >= mark_value;
                            view! {
                                <span
                                    class=move || {
                                        let mut c = format!("{}-dot", prefix);
                                        if is_active() {
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

            // Value display
            {if show_value {
                let format_value = format_value.clone();
                Some(view! {
                    <span class=format!("{}-value", prefix_for_value)>
                        {move || format_value(value.get())}
                    </span>
                })
            } else {
                None
            }}
        </div>
    }.into_any()
}
