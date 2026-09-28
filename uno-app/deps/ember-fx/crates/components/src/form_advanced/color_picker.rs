//! ColorPicker Leptos component.

use leptos::prelude::*;
use super::types::{ColorFormat, ColorPickerSize, ColorPreset, ColorValue};
use crate::try_use_theme;

/// ColorPicker component.
///
/// Color selection with picker panel, presets, and format options.
///
/// # Props
///
/// - `value` - Selected color value
/// - `format` - Output format (Hex, Rgb, Hsl, etc.)
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `show_alpha` - Show alpha slider
/// - `presets` - Preset color palettes
/// - `allow_clear` - Allow clearing the value
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::{ColorPicker, ColorValue, ColorPreset};
///
/// let color = RwSignal::new(ColorValue::from_hex("#1890ff").unwrap());
///
/// view! {
///     <ColorPicker
///         value=color
///         presets=vec![ColorPreset::default_colors()]
///         show_alpha=true
///     />
/// }
/// ```
#[component]
pub fn ColorPicker(
    /// Selected color value.
    #[prop(into)]
    value: RwSignal<ColorValue>,
    /// Output format.
    #[prop(optional, into)]
    format: Option<ColorFormat>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<ColorPickerSize>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Show alpha slider.
    #[prop(optional)]
    show_alpha: bool,
    /// Preset color palettes.
    #[prop(optional, into)]
    presets: Option<Vec<ColorPreset>>,
    /// Allow clearing.
    #[prop(optional)]
    allow_clear: Option<bool>,
    /// Placeholder when no color selected.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<ColorValue>>,
    /// Format change callback.
    #[prop(optional, into)]
    on_format_change: Option<Callback<ColorFormat>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let format = format.unwrap_or_default();
    let size = size.unwrap_or_default();
    let allow_clear = allow_clear.unwrap_or(true);

    // Build CSS classes
    let picker_prefix = format!("fx-color-picker-{}", design_system);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_trigger = picker_prefix.clone();
    let picker_prefix_for_trigger_block = picker_prefix.clone();
    let picker_prefix_for_trigger_closure = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();
    let picker_prefix_for_panel = picker_prefix.clone();
    let picker_prefix_for_presets = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let current_format = RwSignal::new(format);

    // Store presets
    let presets_stored = StoredValue::new(presets.clone());

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![picker_prefix_for_class.clone()];
            if !size_class.is_empty() {
                parts.push(size_class.clone());
            }
            if disabled {
                parts.push(format!("{}-disabled", picker_prefix_for_class));
            }
            if is_open.get() {
                parts.push(format!("{}-open", picker_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Handle color change
    let handle_color_change = move |new_color: ColorValue| {
        value.set(new_color.clone());
        if let Some(ref cb) = on_change {
            cb.run(new_color);
        }
    };

    // Handle preset selection
    let handle_preset_click = move |hex: String| {
        if let Some(color) = ColorValue::from_hex(&hex) {
            let mut new_color = color;
            // Preserve alpha from current value
            let current = value.get_untracked();
            new_color.a = current.a;
            handle_color_change(new_color);
        }
    };

    // Handle format change
    let handle_format_change = move |new_format: ColorFormat| {
        current_format.set(new_format);
        if let Some(ref cb) = on_format_change {
            cb.run(new_format);
        }
    };

    // Handle clear
    let handle_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        handle_color_change(ColorValue::default());
    };

    // Handle RGB input changes
    let handle_r_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(r) = target.value().parse::<u8>() {
            let current = value.get_untracked();
            handle_color_change(ColorValue::rgba(r, current.g, current.b, current.a));
        }
    };

    let handle_g_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(g) = target.value().parse::<u8>() {
            let current = value.get_untracked();
            handle_color_change(ColorValue::rgba(current.r, g, current.b, current.a));
        }
    };

    let handle_b_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(b) = target.value().parse::<u8>() {
            let current = value.get_untracked();
            handle_color_change(ColorValue::rgba(current.r, current.g, b, current.a));
        }
    };

    let handle_a_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(a) = target.value().parse::<f32>() {
            let current = value.get_untracked();
            handle_color_change(ColorValue::rgba(current.r, current.g, current.b, a.clamp(0.0, 1.0)));
        }
    };

    // Handle hex input
    let handle_hex_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        let hex = target.value();
        if let Some(color) = ColorValue::from_hex(&hex) {
            handle_color_change(color);
        }
    };

    view! {
        <div class=combined_class>
            // Color trigger
            <div
                class=format!("{}-trigger", picker_prefix_for_trigger)
                on:click=move |_| {
                    if !disabled {
                        is_open.update(|o| *o = !*o);
                    }
                }
            >
                <div
                    class=format!("{}-color-block", picker_prefix_for_trigger_block)
                    style=move || {
                        let color = value.get();
                        format!("background-color: {};", color.to_rgba())
                    }
                />
                {
                    let prefix = picker_prefix_for_trigger_closure.clone();
                    move || if allow_clear && value.get() != ColorValue::default() {
                        view! {
                            <span
                                class=format!("{}-clear", prefix)
                                on:click=handle_clear
                            >
                                "×"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span class=format!("{}-arrow", prefix)>
                                "▼"
                            </span>
                        }.into_any()
                    }
                }
            </div>

            // Dropdown panel
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown.clone();
                    let panel_prefix = picker_prefix_for_panel.clone();
                    let presets_prefix = picker_prefix_for_presets.clone();
                    let presets_list = presets_stored.get_value();
                    let fmt = current_format.get();

                    Some(view! {
                        <div class=format!("{}-dropdown", picker_prefix)>
                            // Color panel
                            <div class=format!("{}-panel", panel_prefix)>
                                // Saturation/Brightness picker (simplified as a gradient)
                                <div class=format!("{}-saturation", panel_prefix)>
                                    <div
                                        class=format!("{}-saturation-white", panel_prefix)
                                        style=move || {
                                            let color = value.get();
                                            // Create hue-based background
                                            let (h, _, _) = color.to_hsl();
                                            format!("background: linear-gradient(to right, #fff, hsl({}, 100%, 50%));", h)
                                        }
                                    />
                                    <div class=format!("{}-saturation-black", panel_prefix) />
                                </div>

                                // Hue slider
                                <div class=format!("{}-hue-slider", panel_prefix)>
                                    <input
                                        type="range"
                                        min="0"
                                        max="360"
                                        class=format!("{}-hue-input", panel_prefix)
                                        value=move || {
                                            let (h, _, _) = value.get().to_hsl();
                                            h.to_string()
                                        }
                                        on:input=move |ev| {
                                            let target = event_target::<web_sys::HtmlInputElement>(&ev);
                                            if let Ok(h) = target.value().parse::<f32>() {
                                                // Convert HSL to RGB (simplified)
                                                let current = value.get();
                                                let (_, s, l) = current.to_hsl();
                                                // This is simplified - a real implementation would properly convert HSL to RGB
                                                let hue_color = hsl_to_rgb(h, s / 100.0, l / 100.0);
                                                handle_color_change(ColorValue::rgba(hue_color.0, hue_color.1, hue_color.2, current.a));
                                            }
                                        }
                                    />
                                </div>

                                // Alpha slider (if enabled)
                                {if show_alpha {
                                    Some(view! {
                                        <div class=format!("{}-alpha-slider", panel_prefix)>
                                            <input
                                                type="range"
                                                min="0"
                                                max="100"
                                                class=format!("{}-alpha-input", panel_prefix)
                                                value=move || (value.get().a * 100.0) as i32
                                                on:input=handle_a_change.clone()
                                            />
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>

                            // Input section
                            <div class=format!("{}-input-section", panel_prefix)>
                                // Format selector
                                <div class=format!("{}-format-selector", panel_prefix)>
                                    {[ColorFormat::Hex, ColorFormat::Rgb, ColorFormat::Hsl].into_iter().map(|f| {
                                        let panel_prefix = panel_prefix.clone();
                                        view! {
                                            <button
                                                class=move || {
                                                    let mut cls = vec![format!("{}-format-btn", panel_prefix)];
                                                    if current_format.get() == f {
                                                        cls.push(format!("{}-format-btn-active", panel_prefix));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| handle_format_change(f)
                                            >
                                                {f.as_label()}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>

                                // Value inputs based on format
                                <div class=format!("{}-inputs", panel_prefix)>
                                    {match fmt {
                                        ColorFormat::Hex | ColorFormat::Rgba | ColorFormat::Hsla => {
                                            view! {
                                                <input
                                                    type="text"
                                                    class=format!("{}-hex-input", panel_prefix)
                                                    value=move || value.get().to_hex()
                                                    on:change=handle_hex_change
                                                />
                                            }.into_any()
                                        }
                                        ColorFormat::Rgb => {
                                            view! {
                                                <div class=format!("{}-rgb-inputs", panel_prefix)>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"R"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="255"
                                                            value=move || value.get().r
                                                            on:change=handle_r_change
                                                        />
                                                    </div>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"G"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="255"
                                                            value=move || value.get().g
                                                            on:change=handle_g_change
                                                        />
                                                    </div>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"B"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="255"
                                                            value=move || value.get().b
                                                            on:change=handle_b_change
                                                        />
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                        ColorFormat::Hsl => {
                                            let (h, s, l) = value.get().to_hsl();
                                            view! {
                                                <div class=format!("{}-hsl-inputs", panel_prefix)>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"H"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="360"
                                                            value=h as i32
                                                            readonly=true
                                                        />
                                                    </div>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"S"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="100"
                                                            value=s as i32
                                                            readonly=true
                                                        />
                                                    </div>
                                                    <div class=format!("{}-input-group", panel_prefix)>
                                                        <label>"L"</label>
                                                        <input
                                                            type="number"
                                                            min="0"
                                                            max="100"
                                                            value=l as i32
                                                            readonly=true
                                                        />
                                                    </div>
                                                </div>
                                            }.into_any()
                                        }
                                    }}

                                    // Alpha input
                                    {if show_alpha {
                                        Some(view! {
                                            <div class=format!("{}-input-group", panel_prefix)>
                                                <label>"A"</label>
                                                <input
                                                    type="number"
                                                    min="0"
                                                    max="100"
                                                    step="1"
                                                    value=move || (value.get().a * 100.0) as i32
                                                    on:change=move |ev| {
                                                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                                                        if let Ok(a) = target.value().parse::<f32>() {
                                                            let current = value.get_untracked();
                                                            handle_color_change(ColorValue::rgba(
                                                                current.r, current.g, current.b,
                                                                (a / 100.0).clamp(0.0, 1.0)
                                                            ));
                                                        }
                                                    }
                                                />
                                                <span>"%"</span>
                                            </div>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                            </div>

                            // Presets
                            {if presets_list.is_some() {
                                let presets = presets_list.clone().unwrap();
                                Some(view! {
                                    <div class=format!("{}-presets", presets_prefix)>
                                        {presets.into_iter().map(|preset| {
                                            let presets_prefix = presets_prefix.clone();
                                            view! {
                                                <div class=format!("{}-preset-group", presets_prefix)>
                                                    {preset.label.clone().map(|label| view! {
                                                        <div class=format!("{}-preset-label", presets_prefix)>
                                                            {label}
                                                        </div>
                                                    })}
                                                    <div class=format!("{}-preset-colors", presets_prefix)>
                                                        {preset.colors.into_iter().map(|color| {
                                                            let color_clone = color.clone();
                                                            let presets_prefix = presets_prefix.clone();
                                                            view! {
                                                                <div
                                                                    class=format!("{}-preset-color", presets_prefix)
                                                                    style=format!("background-color: {};", color)
                                                                    on:click=move |_| handle_preset_click(color_clone.clone())
                                                                />
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                })
                            } else {
                                None
                            }}

                            // Footer
                            <div class=format!("{}-footer", panel_prefix)>
                                <button
                                    class=format!("{}-ok-btn", panel_prefix)
                                    on:click=move |_| is_open.set(false)
                                >
                                    "OK"
                                </button>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

/// Helper function to convert HSL to RGB.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        let v = (l * 255.0) as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h = h / 360.0;

    let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0/2.0 { return q; }
        if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
        p
    };

    let r = (hue_to_rgb(p, q, h + 1.0/3.0) * 255.0) as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0) as u8;
    let b = (hue_to_rgb(p, q, h - 1.0/3.0) * 255.0) as u8;

    (r, g, b)
}
