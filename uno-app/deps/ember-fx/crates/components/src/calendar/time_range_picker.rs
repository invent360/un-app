//! TimeRangePicker Leptos component.

use leptos::prelude::*;
use super::types::{TimePickerSize, TimeRangeValue};
use crate::try_use_theme;

/// TimeRangePicker component.
///
/// Dual time selection with start and end time.
///
/// # Props
///
/// - `value` - Selected time range
/// - `placeholder` - Placeholder texts (start, end)
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `use_12_hours` - Use 12-hour format
/// - `show_second` - Show seconds column
/// - `separator` - Text between times
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::calendar::{TimeRangePicker, TimeRangeValue};
///
/// let range = RwSignal::new(TimeRangeValue::default());
///
/// view! {
///     <TimeRangePicker value=range placeholder=("Start time", "End time") />
/// }
/// ```
#[component]
pub fn TimeRangePicker(
    /// Selected time range value.
    #[prop(into)]
    value: RwSignal<TimeRangeValue>,
    /// Placeholder texts (start, end).
    #[prop(optional, into)]
    placeholder: Option<(String, String)>,
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
    /// Separator between times.
    #[prop(optional, into)]
    separator: Option<String>,
    /// Allow clear.
    #[prop(optional)]
    allow_clear: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<TimeRangeValue>>,
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
    let separator = separator.unwrap_or_else(|| "~".to_string());
    let (start_placeholder, end_placeholder) = placeholder
        .unwrap_or_else(|| ("Start time".to_string(), "End time".to_string()));

    // Build CSS classes
    let picker_prefix = format!("fx-time-range-picker-{}", design_system);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_inputs = picker_prefix.clone();
    let picker_prefix_for_inputs_suffix = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();
    let picker_prefix_for_dropdown_view = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let active_panel = RwSignal::new("start".to_string());

    // Start time state
    let start_hour = RwSignal::new(0u32);
    let start_minute = RwSignal::new(0u32);
    let start_second = RwSignal::new(0u32);
    let start_period = RwSignal::new("AM".to_string());

    // End time state
    let end_hour = RwSignal::new(0u32);
    let end_minute = RwSignal::new(0u32);
    let end_second = RwSignal::new(0u32);
    let end_period = RwSignal::new("AM".to_string());

    // Parse initial values
    Effect::new(move |_| {
        let val = value.get();

        // Parse start time
        if let Some(time_str) = &val.start {
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() >= 2 {
                if let Ok(h) = parts[0].parse::<u32>() {
                    if use_12_hours {
                        if h == 0 {
                            start_hour.set(12);
                            start_period.set("AM".to_string());
                        } else if h < 12 {
                            start_hour.set(h);
                            start_period.set("AM".to_string());
                        } else if h == 12 {
                            start_hour.set(12);
                            start_period.set("PM".to_string());
                        } else {
                            start_hour.set(h - 12);
                            start_period.set("PM".to_string());
                        }
                    } else {
                        start_hour.set(h);
                    }
                }
                if let Ok(m) = parts[1].parse::<u32>() {
                    start_minute.set(m);
                }
                if parts.len() >= 3 {
                    if let Ok(s) = parts[2].parse::<u32>() {
                        start_second.set(s);
                    }
                }
            }
        }

        // Parse end time
        if let Some(time_str) = &val.end {
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() >= 2 {
                if let Ok(h) = parts[0].parse::<u32>() {
                    if use_12_hours {
                        if h == 0 {
                            end_hour.set(12);
                            end_period.set("AM".to_string());
                        } else if h < 12 {
                            end_hour.set(h);
                            end_period.set("AM".to_string());
                        } else if h == 12 {
                            end_hour.set(12);
                            end_period.set("PM".to_string());
                        } else {
                            end_hour.set(h - 12);
                            end_period.set("PM".to_string());
                        }
                    } else {
                        end_hour.set(h);
                    }
                }
                if let Ok(m) = parts[1].parse::<u32>() {
                    end_minute.set(m);
                }
                if parts.len() >= 3 {
                    if let Ok(s) = parts[2].parse::<u32>() {
                        end_second.set(s);
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

    // Format time string
    let format_time = move |hour: u32, minute: u32, second: u32, period: &str| -> String {
        let final_hour = if use_12_hours {
            if period == "AM" {
                if hour == 12 { 0 } else { hour }
            } else {
                if hour == 12 { 12 } else { hour + 12 }
            }
        } else {
            hour
        };

        if show_second {
            format!("{:02}:{:02}:{:02}", final_hour, minute, second)
        } else {
            format!("{:02}:{:02}", final_hour, minute)
        }
    };

    // Update value
    let update_value = move || {
        let start_time = format_time(
            start_hour.get_untracked(),
            start_minute.get_untracked(),
            start_second.get_untracked(),
            &start_period.get_untracked()
        );
        let end_time = format_time(
            end_hour.get_untracked(),
            end_minute.get_untracked(),
            end_second.get_untracked(),
            &end_period.get_untracked()
        );

        let new_value = TimeRangeValue::new(Some(start_time), Some(end_time));
        value.set(new_value.clone());

        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Handle clear
    let handle_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        let new_value = TimeRangeValue::default();
        value.set(new_value.clone());
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Generate hour options
    let max_hours = if use_12_hours { 12 } else { 24 };
    let start_hour_val = if use_12_hours { 1 } else { 0 };

    // Render time panel
    let render_time_panel = move |is_start: bool| {
        let picker_prefix = picker_prefix_for_dropdown.clone();
        let (hour_signal, minute_signal, second_signal, period_signal) = if is_start {
            (start_hour, start_minute, start_second, start_period)
        } else {
            (end_hour, end_minute, end_second, end_period)
        };

        view! {
            <div class=format!("{}-panel", picker_prefix)>
                <div class=format!("{}-panel-title", picker_prefix)>
                    {if is_start { "Start" } else { "End" }}
                </div>
                <div class=format!("{}-columns", picker_prefix)>
                    // Hours column
                    <div class=format!("{}-column", picker_prefix)>
                        <div class=format!("{}-column-title", picker_prefix)>"Hour"</div>
                        <ul class=format!("{}-column-list", picker_prefix)>
                            {(start_hour_val..=max_hours).map(|h| {
                                let h_display = if use_12_hours && h == 0 { 12 } else { h };
                                let picker_prefix = picker_prefix.clone();
                                view! {
                                    <li
                                        class=move || {
                                            let mut cls = vec![format!("{}-cell", picker_prefix)];
                                            if hour_signal.get() == h_display {
                                                cls.push(format!("{}-cell-selected", picker_prefix));
                                            }
                                            cls.join(" ")
                                        }
                                        on:click=move |_| {
                                            hour_signal.set(h_display);
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
                                            if minute_signal.get() == m {
                                                cls.push(format!("{}-cell-selected", picker_prefix));
                                            }
                                            cls.join(" ")
                                        }
                                        on:click=move |_| {
                                            minute_signal.set(m);
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
                                                    if second_signal.get() == s {
                                                        cls.push(format!("{}-cell-selected", picker_prefix));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| {
                                                    second_signal.set(s);
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
                                                    if period_signal.get() == period_str {
                                                        cls.push(format!("{}-cell-selected", picker_prefix));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| {
                                                    period_signal.set(period_for_click.clone());
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
            </div>
        }
    };

    view! {
        <div class=combined_class>
            // Input area
            <div
                class=format!("{}-input-wrapper", picker_prefix_for_inputs)
                on:click=move |_| {
                    if !disabled {
                        is_open.update(|o| *o = !*o);
                        active_panel.set("start".to_string());
                    }
                }
            >
                <input
                    type="text"
                    class=format!("{}-input {}-input-start", picker_prefix_for_inputs, picker_prefix_for_inputs)
                    placeholder=start_placeholder.clone()
                    readonly=true
                    disabled=disabled
                    value=move || value.get().start.unwrap_or_default()
                />
                <span class=format!("{}-separator", picker_prefix_for_inputs)>
                    {separator.clone()}
                </span>
                <input
                    type="text"
                    class=format!("{}-input {}-input-end", picker_prefix_for_inputs, picker_prefix_for_inputs)
                    placeholder=end_placeholder.clone()
                    readonly=true
                    disabled=disabled
                    value=move || value.get().end.unwrap_or_default()
                />
                <span class=format!("{}-suffix", picker_prefix_for_inputs)>
                    {
                        let prefix = picker_prefix_for_inputs_suffix.clone();
                        move || if allow_clear && !value.get().is_empty() {
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
                                <span class=format!("{}-icon", prefix)>
                                    "🕐"
                                </span>
                            }.into_any()
                        }
                    }
                </span>
            </div>

            // Dropdown with dual time panels
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown_view.clone();

                    Some(view! {
                        <div class=format!("{}-dropdown", picker_prefix)>
                            <div class=format!("{}-panels", picker_prefix)>
                                {render_time_panel(true)}
                                {render_time_panel(false)}
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

                                        // Set both start and end to now
                                        if use_12_hours {
                                            let (hour_12, period) = if h == 0 {
                                                (12, "AM")
                                            } else if h < 12 {
                                                (h, "AM")
                                            } else if h == 12 {
                                                (12, "PM")
                                            } else {
                                                (h - 12, "PM")
                                            };

                                            start_hour.set(hour_12);
                                            start_period.set(period.to_string());
                                            end_hour.set(hour_12);
                                            end_period.set(period.to_string());
                                        } else {
                                            start_hour.set(h);
                                            end_hour.set(h);
                                        }

                                        start_minute.set(m);
                                        start_second.set(s);
                                        end_minute.set(m);
                                        end_second.set(s);

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
