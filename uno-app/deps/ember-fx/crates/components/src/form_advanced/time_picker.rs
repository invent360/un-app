//! TimePicker Leptos component.

use leptos::prelude::*;
use super::types::TimePickerSize;
use crate::try_use_theme;

/// TimePicker component.
///
/// Time selection input with dropdown.
///
/// # Props
///
/// - `value` - Selected time (HH:MM:SS format)
/// - `placeholder` - Placeholder text
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `use_12_hours` - Use 12-hour format
/// - `show_second` - Show seconds column
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::TimePicker;
///
/// let time = RwSignal::new(None::<String>);
///
/// view! {
///     <TimePicker value=time placeholder="Select time" />
/// }
/// ```
#[component]
pub fn TimePicker(
    /// Selected time value (HH:MM:SS format).
    #[prop(into)]
    value: RwSignal<Option<String>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<TimePickerSize>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Use 12-hour format.
    #[prop(optional)]
    use_12_hours: bool,
    /// Show seconds column.
    #[prop(optional)]
    show_second: Option<bool>,
    /// Allow clear.
    #[prop(optional)]
    allow_clear: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<Option<String>>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let show_second = show_second.unwrap_or(true);
    let allow_clear = allow_clear.unwrap_or(true);
    let placeholder = placeholder.unwrap_or_else(|| "Select time".to_string());

    // Build CSS classes
    let picker_prefix = format!("fx-time-picker-{}", design_system);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_input = picker_prefix.clone();
    let picker_prefix_for_input_field = picker_prefix.clone();
    let picker_prefix_for_suffix = picker_prefix.clone();
    let picker_prefix_for_clear = picker_prefix.clone();
    let picker_prefix_for_icon = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let selected_hour = RwSignal::new(0u32);
    let selected_minute = RwSignal::new(0u32);
    let selected_second = RwSignal::new(0u32);
    let selected_period = RwSignal::new("AM".to_string()); // For 12-hour format

    // Parse initial value
    Effect::new(move |_| {
        if let Some(time_str) = value.get() {
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() >= 2 {
                if let Ok(h) = parts[0].parse::<u32>() {
                    if use_12_hours {
                        if h == 0 {
                            selected_hour.set(12);
                            selected_period.set("AM".to_string());
                        } else if h < 12 {
                            selected_hour.set(h);
                            selected_period.set("AM".to_string());
                        } else if h == 12 {
                            selected_hour.set(12);
                            selected_period.set("PM".to_string());
                        } else {
                            selected_hour.set(h - 12);
                            selected_period.set("PM".to_string());
                        }
                    } else {
                        selected_hour.set(h);
                    }
                }
                if let Ok(m) = parts[1].parse::<u32>() {
                    selected_minute.set(m);
                }
                if parts.len() >= 3 {
                    if let Ok(s) = parts[2].parse::<u32>() {
                        selected_second.set(s);
                    }
                }
            }
        }
    });

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
                parts.push(format!("{}-focused", picker_prefix_for_class));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Update value when selection changes
    let update_value = move || {
        let hour = selected_hour.get_untracked();
        let minute = selected_minute.get_untracked();
        let second = selected_second.get_untracked();

        let final_hour = if use_12_hours {
            let period = selected_period.get_untracked();
            if period == "AM" {
                if hour == 12 { 0 } else { hour }
            } else {
                if hour == 12 { 12 } else { hour + 12 }
            }
        } else {
            hour
        };

        let time_str = if show_second {
            format!("{:02}:{:02}:{:02}", final_hour, minute, second)
        } else {
            format!("{:02}:{:02}", final_hour, minute)
        };

        value.set(Some(time_str.clone()));
        if let Some(ref cb) = on_change {
            cb.run(Some(time_str));
        }
    };

    // Handle clear
    let handle_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        value.set(None);
        if let Some(ref cb) = on_change {
            cb.run(None);
        }
    };

    // Generate hour options
    let max_hours = if use_12_hours { 12 } else { 24 };
    let start_hour = if use_12_hours { 1 } else { 0 };

    view! {
        <div class=combined_class>
            // Input area
            <div
                class=format!("{}-input", picker_prefix_for_input)
                on:click=move |_| {
                    if !disabled {
                        is_open.update(|o| *o = !*o);
                    }
                }
            >
                <input
                    type="text"
                    class=format!("{}-input-field", picker_prefix_for_input_field)
                    placeholder=placeholder.clone()
                    readonly=true
                    disabled=disabled
                    value=move || value.get().unwrap_or_default()
                />
                <span class=format!("{}-suffix", picker_prefix_for_suffix)>
                    {move || if allow_clear && value.get().is_some() {
                        view! {
                            <span
                                class=format!("{}-clear", picker_prefix_for_clear)
                                on:click=handle_clear
                            >
                                "×"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span class=format!("{}-icon", picker_prefix_for_icon)>
                                "🕐"
                            </span>
                        }.into_any()
                    }}
                </span>
            </div>

            // Dropdown
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown.clone();
                    Some(view! {
                        <div class=format!("{}-dropdown", picker_prefix)>
                            <div class=format!("{}-panel", picker_prefix)>
                                // Hours column
                                <div class=format!("{}-column", picker_prefix)>
                                    <div class=format!("{}-column-title", picker_prefix)>"Hour"</div>
                                    <ul class=format!("{}-column-list", picker_prefix)>
                                        {(start_hour..=max_hours).map(|h| {
                                            let h_display = if use_12_hours && h == 0 { 12 } else { h };
                                            let picker_prefix = picker_prefix.clone();
                                            view! {
                                                <li
                                                    class=move || {
                                                        let mut cls = vec![format!("{}-cell", picker_prefix)];
                                                        if selected_hour.get() == h_display {
                                                            cls.push(format!("{}-cell-selected", picker_prefix));
                                                        }
                                                        cls.join(" ")
                                                    }
                                                    on:click=move |_| {
                                                        selected_hour.set(h_display);
                                                        update_value();
                                                    }
                                                >
                                                    {format!("{:02}", h_display)}
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                </div>

                                // Minutes column
                                <div class=format!("{}-column", picker_prefix)>
                                    <div class=format!("{}-column-title", picker_prefix)>"Minute"</div>
                                    <ul class=format!("{}-column-list", picker_prefix)>
                                        {(0..60).map(|m| {
                                            let picker_prefix = picker_prefix.clone();
                                            view! {
                                                <li
                                                    class=move || {
                                                        let mut cls = vec![format!("{}-cell", picker_prefix)];
                                                        if selected_minute.get() == m {
                                                            cls.push(format!("{}-cell-selected", picker_prefix));
                                                        }
                                                        cls.join(" ")
                                                    }
                                                    on:click=move |_| {
                                                        selected_minute.set(m);
                                                        update_value();
                                                    }
                                                >
                                                    {format!("{:02}", m)}
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                </div>

                                // Seconds column (optional)
                                {if show_second {
                                    let picker_prefix = picker_prefix.clone();
                                    Some(view! {
                                        <div class=format!("{}-column", picker_prefix)>
                                            <div class=format!("{}-column-title", picker_prefix)>"Second"</div>
                                            <ul class=format!("{}-column-list", picker_prefix)>
                                                {(0..60).map(|s| {
                                                    let picker_prefix = picker_prefix.clone();
                                                    view! {
                                                        <li
                                                            class=move || {
                                                                let mut cls = vec![format!("{}-cell", picker_prefix)];
                                                                if selected_second.get() == s {
                                                                    cls.push(format!("{}-cell-selected", picker_prefix));
                                                                }
                                                                cls.join(" ")
                                                            }
                                                            on:click=move |_| {
                                                                selected_second.set(s);
                                                                update_value();
                                                            }
                                                        >
                                                            {format!("{:02}", s)}
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    })
                                } else {
                                    None
                                }}

                                // AM/PM column (for 12-hour format)
                                {if use_12_hours {
                                    let picker_prefix = picker_prefix.clone();
                                    Some(view! {
                                        <div class=format!("{}-column {}-column-period", picker_prefix, picker_prefix)>
                                            <div class=format!("{}-column-title", picker_prefix)></div>
                                            <ul class=format!("{}-column-list", picker_prefix)>
                                                {["AM", "PM"].into_iter().map(|period| {
                                                    let period_str = period.to_string();
                                                    let period_for_click = period_str.clone();
                                                    let picker_prefix = picker_prefix.clone();
                                                    view! {
                                                        <li
                                                            class=move || {
                                                                let mut cls = vec![format!("{}-cell", picker_prefix)];
                                                                if selected_period.get() == period_str {
                                                                    cls.push(format!("{}-cell-selected", picker_prefix));
                                                                }
                                                                cls.join(" ")
                                                            }
                                                            on:click=move |_| {
                                                                selected_period.set(period_for_click.clone());
                                                                update_value();
                                                            }
                                                        >
                                                            {period}
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>

                            // Footer
                            <div class=format!("{}-footer", picker_prefix)>
                                <button
                                    class=format!("{}-now-btn", picker_prefix)
                                    on:click=move |_| {
                                        let now = js_sys::Date::new_0();
                                        let h = now.get_hours();
                                        let m = now.get_minutes();
                                        let s = now.get_seconds();

                                        if use_12_hours {
                                            if h == 0 {
                                                selected_hour.set(12);
                                                selected_period.set("AM".to_string());
                                            } else if h < 12 {
                                                selected_hour.set(h);
                                                selected_period.set("AM".to_string());
                                            } else if h == 12 {
                                                selected_hour.set(12);
                                                selected_period.set("PM".to_string());
                                            } else {
                                                selected_hour.set(h - 12);
                                                selected_period.set("PM".to_string());
                                            }
                                        } else {
                                            selected_hour.set(h);
                                        }
                                        selected_minute.set(m);
                                        selected_second.set(s);
                                        update_value();
                                    }
                                >
                                    "Now"
                                </button>
                                <button
                                    class=format!("{}-ok-btn", picker_prefix)
                                    on:click=move |_| {
                                        is_open.set(false);
                                    }
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
