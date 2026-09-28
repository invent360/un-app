//! Slider Leptos component.
//!
//! Provides both a native HTML range input variant and a custom div-based
//! slider for advanced styling. The native variant is recommended for
//! better accessibility and mobile support.

use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use wasm_bindgen::JsCast;
use super::types::{SliderSize, SliderMark};
use crate::try_use_theme;

/// Slider variant type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SliderVariant {
    /// Native HTML range input (recommended for accessibility).
    Native,
    /// Custom div-based slider for advanced styling.
    #[default]
    Custom,
}

/// Slider component.
///
/// A slider input for selecting a value from a range. Supports both native
/// HTML range input (for better accessibility) and custom div-based rendering.
///
/// # Props
///
/// - `value` - Current value (RwSignal for two-way binding)
/// - `min` - Minimum value (default: 0.0)
/// - `max` - Maximum value (default: 100.0)
/// - `step` - Step increment (default: 1.0)
/// - `disabled` - Disabled state
/// - `vertical` - Vertical orientation
/// - `marks` - Mark points on the slider
/// - `tooltip_visible` - Whether to show value tooltip
/// - `variant` - Slider rendering variant (Native or Custom)
/// - `show_value` - Show value text next to slider
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::Slider;
///
/// let value = RwSignal::new(30.0);
///
/// // Native slider (recommended)
/// view! {
///     <Slider value=value min=0.0 max=100.0 variant=SliderVariant::Native />
/// }
///
/// // Custom slider with marks
/// view! {
///     <Slider value=value min=0.0 max=100.0 marks=vec![...] />
/// }
/// ```
#[component]
pub fn Slider(
    /// Current value (single value mode).
    #[prop(into)]
    value: RwSignal<f64>,
    /// Minimum value.
    #[prop(optional, into)]
    min: Option<f64>,
    /// Maximum value.
    #[prop(optional, into)]
    max: Option<f64>,
    /// Step increment.
    #[prop(optional, into)]
    step: Option<f64>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Vertical orientation (only for Custom variant).
    #[prop(optional)]
    vertical: bool,
    /// Mark points.
    #[prop(optional, into)]
    marks: Option<Vec<SliderMark>>,
    /// Show tooltip on hover/drag.
    #[prop(optional)]
    tooltip_visible: Option<bool>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SliderSize>,
    /// Slider variant (Native or Custom).
    #[prop(optional)]
    variant: Option<SliderVariant>,
    /// Show value text next to slider.
    #[prop(optional)]
    show_value: bool,
    /// Format function for value display.
    #[prop(optional, into)]
    format: Option<Callback<f64, String>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,
    /// After change callback (fires on mouse/touch up).
    #[prop(optional, into)]
    on_after_change: Option<Callback<f64>>,
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
    let tooltip_visible = tooltip_visible.unwrap_or(true);
    let variant = variant.unwrap_or_default();

    // Build CSS classes
    let slider_prefix = format!("fx-slider-{}", design_system);
    let size_class = size.class(&slider_prefix);

    // Calculate percentage for CSS variable (memoized for performance)
    let percentage = Memo::new(move |_| {
        let val = value.get();
        ((val - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    });

    // Format value display
    let format_value = move |v: f64| {
        if let Some(ref fmt) = format {
            fmt.run(v)
        } else {
            format!("{:.0}", v)
        }
    };

    match variant {
        SliderVariant::Native => {
            // Native HTML range input with CSS variable for styling
            let slider_prefix_for_native = slider_prefix.clone();
            let class_clone = class.clone();

            let native_class = move || {
                let mut parts = vec![
                    slider_prefix_for_native.clone(),
                    format!("{}-native", slider_prefix_for_native),
                ];
                if !size_class.is_empty() {
                    parts.push(size_class.clone());
                }
                if disabled {
                    parts.push(format!("{}-disabled", slider_prefix_for_native));
                }
                if let Some(ref custom) = class_clone {
                    parts.push(custom.clone());
                }
                parts.join(" ")
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
                            if let Some(ref cb) = on_after_change {
                                cb.run(value.get());
                            }
                        }
                    />
                    {if show_value {
                        Some(view! {
                            <span class=format!("{}-value", slider_prefix)>
                                {move || format_value(value.get())}
                            </span>
                        })
                    } else {
                        None
                    }}
                </div>
            }.into_any()
        }
        SliderVariant::Custom => {
            // Custom div-based slider
            let slider_prefix_for_class = slider_prefix.clone();
            let slider_prefix_for_rail = slider_prefix.clone();
            let slider_prefix_for_track = slider_prefix.clone();
            let slider_prefix_for_handle = slider_prefix.clone();
            let slider_prefix_for_marks = slider_prefix.clone();
            let slider_prefix_for_tooltip = slider_prefix.clone();
            let slider_prefix_for_value = slider_prefix.clone();
            let class_clone = class.clone();

            let combined_class = move || {
                let mut parts = vec![slider_prefix_for_class.clone()];
                if !size_class.is_empty() {
                    parts.push(size_class.clone());
                }
                if disabled {
                    parts.push(format!("{}-disabled", slider_prefix_for_class));
                }
                if vertical {
                    parts.push(format!("{}-vertical", slider_prefix_for_class));
                }
                if let Some(ref custom) = class_clone {
                    parts.push(custom.clone());
                }
                parts.join(" ")
            };

            // Dragging state
            let is_dragging = RwSignal::new(false);
            let is_focused = RwSignal::new(false);

            // Handle mouse/touch events
            let handle_move = move |client_x: i32, client_y: i32, element: web_sys::Element| {
                if disabled {
                    return;
                }

                let rect = element.get_bounding_client_rect();
                let ratio = if vertical {
                    let height = rect.height();
                    let y = client_y as f64 - rect.top();
                    1.0 - (y / height).clamp(0.0, 1.0)
                } else {
                    let width = rect.width();
                    let x = client_x as f64 - rect.left();
                    (x / width).clamp(0.0, 1.0)
                };

                let new_value = min + ratio * (max - min);
                let stepped_value = (new_value / step).round() * step;
                let clamped_value = stepped_value.clamp(min, max);

                value.set(clamped_value);
                if let Some(ref cb) = on_change {
                    cb.run(clamped_value);
                }
            };

            view! {
                <div
                    class=combined_class
                    style=move || format!("--slider-value: {}%", percentage.get())
                >
                    <div
                        class=format!("{}-rail", slider_prefix_for_rail)
                        // Mouse events
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
                                if let Some(ref cb) = on_after_change {
                                    cb.run(value.get());
                                }
                            }
                        }
                        on:mouseleave=move |_| {
                            if is_dragging.get_untracked() {
                                is_dragging.set(false);
                                if let Some(ref cb) = on_after_change {
                                    cb.run(value.get());
                                }
                            }
                        }
                        // Touch events for mobile support
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
                                if let Some(ref cb) = on_after_change {
                                    cb.run(value.get());
                                }
                            }
                        }
                        on:touchcancel=move |_: web_sys::TouchEvent| {
                            if is_dragging.get_untracked() {
                                is_dragging.set(false);
                            }
                        }
                    >
                        // Track (filled portion)
                        <div
                            class=format!("{}-track", slider_prefix_for_track)
                            style=move || {
                                if vertical {
                                    format!("height: {}%;", percentage.get())
                                } else {
                                    format!("width: {}%;", percentage.get())
                                }
                            }
                        />
                        // Handle (draggable thumb)
                        <div
                            class=move || {
                                let mut cls = format!("{}-handle", slider_prefix_for_handle);
                                if is_dragging.get_untracked() {
                                    cls.push_str(&format!(" {}-handle-dragging", slider_prefix_for_handle));
                                }
                                if is_focused.get() {
                                    cls.push_str(&format!(" {}-handle-focused", slider_prefix_for_handle));
                                }
                                cls
                            }
                            style=move || {
                                if vertical {
                                    format!("bottom: {}%;", percentage.get())
                                } else {
                                    format!("left: {}%; top: 0%; transform: translate(-50%, -50%);", percentage.get())
                                }
                            }
                            tabindex=if disabled { "-1" } else { "0" }
                            role="slider"
                            aria-valuemin=min
                            aria-valuemax=max
                            aria-valuenow=move || value.get()
                            aria-orientation=if vertical { "vertical" } else { "horizontal" }
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
                                        Some((value.get() + step).min(max))
                                    }
                                    "ArrowLeft" | "ArrowDown" => {
                                        ev.prevent_default();
                                        Some((value.get() - step).max(min))
                                    }
                                    "PageUp" => {
                                        ev.prevent_default();
                                        Some((value.get() + large_step).min(max))
                                    }
                                    "PageDown" => {
                                        ev.prevent_default();
                                        Some((value.get() - large_step).max(min))
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
                                    let stepped_value = (new_val / step).round() * step;
                                    let clamped_value = stepped_value.clamp(min, max);
                                    value.set(clamped_value);
                                    if let Some(ref cb) = on_change {
                                        cb.run(clamped_value);
                                    }
                                }
                            }
                        >
                            // Tooltip
                            {if tooltip_visible {
                                Some(view! {
                                    <div
                                        class=move || {
                                            let base = format!("{}-tooltip", slider_prefix_for_tooltip);
                                            if is_dragging.get() || is_focused.get() {
                                                format!("{} {}-tooltip-visible", base, slider_prefix_for_tooltip)
                                            } else {
                                                base
                                            }
                                        }
                                    >
                                        {move || format_value(value.get())}
                                    </div>
                                })
                            } else {
                                None
                            }}
                        </div>
                    </div>

                    // Marks
                    {marks.clone().map(|marks| {
                        let slider_prefix = slider_prefix_for_marks.clone();
                        view! {
                            <div class=format!("{}-marks", slider_prefix)>
                                {marks.iter().map(|mark| {
                                    let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                                    let slider_prefix = slider_prefix.clone();
                                    let mark_value = mark.value;
                                    let is_active = move || value.get() >= mark_value;
                                    view! {
                                        <span
                                            class=move || {
                                                let base = format!("{}-mark", slider_prefix);
                                                if is_active() {
                                                    format!("{} {}-mark-active", base, slider_prefix)
                                                } else {
                                                    base
                                                }
                                            }
                                            style=if vertical {
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
                                {marks.iter().map(|mark| {
                                    let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                                    let slider_prefix = slider_prefix.clone();
                                    let mark_value = mark.value;
                                    let is_active = move || value.get() >= mark_value;
                                    view! {
                                        <span
                                            class=move || {
                                                let base = format!("{}-dot", slider_prefix);
                                                if is_active() {
                                                    format!("{} {}-dot-active", base, slider_prefix)
                                                } else {
                                                    base
                                                }
                                            }
                                            style=if vertical {
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
                        Some(view! {
                            <span class=format!("{}-value", slider_prefix_for_value)>
                                {move || format_value(value.get())}
                            </span>
                        })
                    } else {
                        None
                    }}
                </div>
            }.into_any()
        }
    }
}

/// Range slider for selecting a range of values.
#[component]
pub fn RangeSlider(
    /// Range values [min_selected, max_selected].
    #[prop(into)]
    value: RwSignal<(f64, f64)>,
    /// Minimum value.
    #[prop(optional, into)]
    min: Option<f64>,
    /// Maximum value.
    #[prop(optional, into)]
    max: Option<f64>,
    /// Step increment.
    #[prop(optional, into)]
    step: Option<f64>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Mark points.
    #[prop(optional, into)]
    marks: Option<Vec<SliderMark>>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SliderSize>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<(f64, f64)>>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(100.0);
    let step = step.unwrap_or(1.0);
    let size = size.unwrap_or_default();

    let slider_prefix = format!("fx-slider-{}", design_system);
    let size_class = size.class(&slider_prefix);

    let percentage_low = move || {
        let (low, _) = value.get();
        ((low - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    };

    let percentage_high = move || {
        let (_, high) = value.get();
        ((high - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    };

    let combined_class = {
        let mut parts = vec![slider_prefix.clone(), format!("{}-range", slider_prefix)];
        if !size_class.is_empty() {
            parts.push(size_class);
        }
        if disabled {
            parts.push(format!("{}-disabled", slider_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Pre-clone for use in view
    let rail_class = format!("{}-rail", slider_prefix);
    let track_class = format!("{}-track", slider_prefix);
    let handle_class = format!("{}-handle", slider_prefix);
    let tooltip_class = format!("{}-tooltip", slider_prefix);
    let slider_prefix_for_marks = slider_prefix.clone();

    view! {
        <div class=combined_class>
            <div class=rail_class>
                // Track between the two handles
                <div
                    class=track_class
                    style=move || {
                        format!(
                            "left: {}%; width: {}%;",
                            percentage_low(),
                            percentage_high() - percentage_low()
                        )
                    }
                />
                // Low handle
                <div
                    class=handle_class.clone()
                    style=move || format!("left: {}%;", percentage_low())
                    tabindex=if disabled { "-1" } else { "0" }
                    role="slider"
                    aria-valuemin=min
                    aria-valuemax=move || value.get().1
                    aria-valuenow=move || value.get().0
                >
                    <div class=tooltip_class.clone()>
                        {move || format!("{:.0}", value.get().0)}
                    </div>
                </div>
                // High handle
                <div
                    class=handle_class.clone()
                    style=move || format!("left: {}%;", percentage_high())
                    tabindex=if disabled { "-1" } else { "0" }
                    role="slider"
                    aria-valuemin=move || value.get().0
                    aria-valuemax=max
                    aria-valuenow=move || value.get().1
                >
                    <div class=tooltip_class.clone()>
                        {move || format!("{:.0}", value.get().1)}
                    </div>
                </div>
            </div>

            // Marks
            {marks.map(|marks| {
                let slider_prefix = slider_prefix_for_marks.clone();
                view! {
                    <div class=format!("{}-marks", slider_prefix)>
                        {marks.into_iter().map(|mark| {
                            let pos = ((mark.value - min) / (max - min) * 100.0).clamp(0.0, 100.0);
                            let slider_prefix = slider_prefix.clone();
                            view! {
                                <span
                                    class=format!("{}-mark", slider_prefix)
                                    style=format!("left: {}%;", pos)
                                >
                                    {mark.label.clone()}
                                </span>
                            }
                        }).collect_view()}
                    </div>
                }
            })}
        </div>
    }
}
