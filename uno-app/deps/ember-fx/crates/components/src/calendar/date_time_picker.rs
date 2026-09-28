//! DateTimePicker Leptos component.

use leptos::prelude::*;
use super::types::{DatePickerSize, DateTimeValue, TimeFormat};
use crate::try_use_theme;

/// DateTimePicker component.
///
/// Combined date and time selection in a single picker.
///
/// # Props
///
/// - `value` - Selected date-time value
/// - `placeholder` - Placeholder text
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `time_format` - Time format (24h, 12h, with/without seconds)
/// - `show_time` - Whether to show time picker
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::calendar::{DateTimePicker, DateTimeValue};
///
/// let datetime = RwSignal::new(DateTimeValue::default());
///
/// view! {
///     <DateTimePicker value=datetime placeholder="Select date and time" />
/// }
/// ```
#[component]
pub fn DateTimePicker(
    /// Selected date-time value.
    #[prop(into)]
    value: RwSignal<DateTimeValue>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<DatePickerSize>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Time format.
    #[prop(optional, into)]
    time_format: Option<TimeFormat>,
    /// Show time picker.
    #[prop(optional)]
    show_time: Option<bool>,
    /// Allow clear.
    #[prop(optional)]
    allow_clear: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<DateTimeValue>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let time_format = time_format.unwrap_or_default();
    let show_time = show_time.unwrap_or(true);
    let allow_clear = allow_clear.unwrap_or(true);
    let placeholder = placeholder.unwrap_or_else(|| "Select date and time".to_string());

    let use_12_hours = time_format.is_12_hour();
    let show_second = time_format.has_seconds();

    // Build CSS classes
    let picker_prefix = format!("fx-datetime-picker-{}", design_system);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_input = picker_prefix.clone();
    let picker_prefix_for_input_suffix = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let active_tab = RwSignal::new("date".to_string()); // "date" or "time"

    // Date state
    let current_year = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_full_year() as i32
    });
    let current_month = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_month() as u32
    });

    // Time state
    let selected_hour = RwSignal::new(0u32);
    let selected_minute = RwSignal::new(0u32);
    let selected_second = RwSignal::new(0u32);
    let selected_period = RwSignal::new("AM".to_string());

    // Parse initial values
    Effect::new(move |_| {
        let val = value.get();

        // Parse date
        if let Some(date_str) = &val.date {
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() == 3 {
                if let Ok(y) = parts[0].parse::<i32>() {
                    current_year.set(y);
                }
                if let Ok(m) = parts[1].parse::<u32>() {
                    current_month.set(m - 1);
                }
            }
        }

        // Parse time
        if let Some(time_str) = &val.time {
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

    // Get days in month
    let days_in_month = move |year: i32, month: u32| -> u32 {
        let date = js_sys::Date::new_with_year_month_day(year as u32, (month + 1) as i32, 0);
        date.get_date()
    };

    // Get first day of month (0 = Sunday)
    let first_day_of_month = move |year: i32, month: u32| -> u32 {
        let date = js_sys::Date::new_with_year_month_day(year as u32, month as i32, 1);
        date.get_day()
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

    // Handle date selection
    let handle_date_select = move |day: u32| {
        let year = current_year.get_untracked();
        let month = current_month.get_untracked() + 1;
        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);

        let current = value.get_untracked();
        let new_value = DateTimeValue::new(Some(date_str), current.time);
        value.set(new_value.clone());

        // Switch to time tab if time is not set
        if show_time && new_value.time.is_none() {
            active_tab.set("time".to_string());
        }

        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Update time value
    let update_time = move || {
        let time_str = format_time(
            selected_hour.get_untracked(),
            selected_minute.get_untracked(),
            selected_second.get_untracked(),
            &selected_period.get_untracked()
        );

        let current = value.get_untracked();
        let new_value = DateTimeValue::new(current.date, Some(time_str));
        value.set(new_value.clone());

        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Handle clear
    let handle_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        let new_value = DateTimeValue::default();
        value.set(new_value.clone());
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Navigate months
    let prev_month = move |_| {
        let month = current_month.get_untracked();
        if month == 0 {
            current_month.set(11);
            current_year.update(|y| *y -= 1);
        } else {
            current_month.set(month - 1);
        }
    };

    let next_month = move |_| {
        let month = current_month.get_untracked();
        if month == 11 {
            current_month.set(0);
            current_year.update(|y| *y += 1);
        } else {
            current_month.set(month + 1);
        }
    };

    // Month names
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

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
                    class=format!("{}-input-field", picker_prefix_for_input)
                    placeholder=placeholder.clone()
                    readonly=true
                    disabled=disabled
                    value=move || value.get().display(" ")
                />
                <span class=format!("{}-suffix", picker_prefix_for_input)>
                    {
                        let prefix = picker_prefix_for_input_suffix.clone();
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
                                    "📅"
                                </span>
                            }.into_any()
                        }
                    }
                </span>
            </div>

            // Dropdown
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown.clone();
                    let year = current_year.get();
                    let month = current_month.get();
                    let days = days_in_month(year, month);
                    let first_day = first_day_of_month(year, month);
                    let month_name = month_names[month as usize];

                    // Selected date parts
                    let val = value.get();
                    let selected_date_parts: Option<(i32, u32, u32)> = val.date.as_ref().and_then(|s| {
                        let parts: Vec<&str> = s.split('-').collect();
                        if parts.len() == 3 {
                            Some((
                                parts[0].parse().ok()?,
                                parts[1].parse().ok()?,
                                parts[2].parse().ok()?,
                            ))
                        } else {
                            None
                        }
                    });

                    Some(view! {
                        <div class=format!("{}-dropdown", picker_prefix)>
                            // Tabs
                            {if show_time {
                                let tabs_prefix = picker_prefix.clone();
                                let date_tab_prefix = picker_prefix.clone();
                                let time_tab_prefix = picker_prefix.clone();
                                Some(view! {
                                    <div class=format!("{}-tabs", tabs_prefix)>
                                        <button
                                            class=move || {
                                                let mut cls = vec![format!("{}-tab", date_tab_prefix)];
                                                if active_tab.get() == "date" {
                                                    cls.push(format!("{}-tab-active", date_tab_prefix));
                                                }
                                                cls.join(" ")
                                            }
                                            on:click=move |_| active_tab.set("date".to_string())
                                        >
                                            "Date"
                                        </button>
                                        <button
                                            class=move || {
                                                let mut cls = vec![format!("{}-tab", time_tab_prefix)];
                                                if active_tab.get() == "time" {
                                                    cls.push(format!("{}-tab-active", time_tab_prefix));
                                                }
                                                cls.join(" ")
                                            }
                                            on:click=move |_| active_tab.set("time".to_string())
                                        >
                                            "Time"
                                        </button>
                                    </div>
                                })
                            } else {
                                None
                            }}

                            // Date panel
                            <div
                                class=format!("{}-date-panel", picker_prefix)
                                class:hidden=move || show_time && active_tab.get() != "date"
                            >
                                // Header
                                <div class=format!("{}-header", picker_prefix)>
                                    <button
                                        class=format!("{}-nav-btn", picker_prefix)
                                        on:click=prev_month
                                    >
                                        "‹"
                                    </button>
                                    <span class=format!("{}-header-text", picker_prefix)>
                                        {format!("{} {}", month_name, year)}
                                    </span>
                                    <button
                                        class=format!("{}-nav-btn", picker_prefix)
                                        on:click=next_month
                                    >
                                        "›"
                                    </button>
                                </div>

                                // Weekday headers
                                <div class=format!("{}-weekdays", picker_prefix)>
                                    {["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].into_iter().map(|day| {
                                        view! {
                                            <span class=format!("{}-weekday", picker_prefix)>{day}</span>
                                        }
                                    }).collect_view()}
                                </div>

                                // Calendar grid
                                <div class=format!("{}-body", picker_prefix)>
                                    // Empty cells before first day
                                    {(0..first_day).map(|_| {
                                        view! {
                                            <span class=format!("{}-cell {}-cell-empty", picker_prefix, picker_prefix)></span>
                                        }
                                    }).collect_view()}

                                    // Day cells
                                    {(1..=days).map(|day| {
                                        let picker_prefix = picker_prefix.clone();
                                        let is_selected = selected_date_parts.map(|(sy, sm, sd)| {
                                            sy == year && sm == (month + 1) && sd == day
                                        }).unwrap_or(false);

                                        // Check if today
                                        let now = js_sys::Date::new_0();
                                        let is_today = now.get_full_year() as i32 == year
                                            && now.get_month() == month
                                            && now.get_date() == day;

                                        view! {
                                            <span
                                                class=move || {
                                                    let mut cls = vec![format!("{}-cell", picker_prefix)];
                                                    if is_selected {
                                                        cls.push(format!("{}-cell-selected", picker_prefix));
                                                    }
                                                    if is_today {
                                                        cls.push(format!("{}-cell-today", picker_prefix));
                                                    }
                                                    cls.join(" ")
                                                }
                                                on:click=move |_| handle_date_select(day)
                                            >
                                                {day}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Time panel
                            {if show_time {
                                let picker_prefix = picker_prefix.clone();
                                Some(view! {
                                    <div
                                        class=format!("{}-time-panel", picker_prefix)
                                        class:hidden=move || active_tab.get() != "time"
                                    >
                                        <div class=format!("{}-time-columns", picker_prefix)>
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
                                                                    let mut cls = vec![format!("{}-time-cell", picker_prefix)];
                                                                    if selected_hour.get() == h_display {
                                                                        cls.push(format!("{}-time-cell-selected", picker_prefix));
                                                                    }
                                                                    cls.join(" ")
                                                                }
                                                                on:click=move |_| {
                                                                    selected_hour.set(h_display);
                                                                    update_time();
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
                                                                    let mut cls = vec![format!("{}-time-cell", picker_prefix)];
                                                                    if selected_minute.get() == m {
                                                                        cls.push(format!("{}-time-cell-selected", picker_prefix));
                                                                    }
                                                                    cls.join(" ")
                                                                }
                                                                on:click=move |_| {
                                                                    selected_minute.set(m);
                                                                    update_time();
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
                                                                            let mut cls = vec![format!("{}-time-cell", picker_prefix)];
                                                                            if selected_second.get() == s {
                                                                                cls.push(format!("{}-time-cell-selected", picker_prefix));
                                                                            }
                                                                            cls.join(" ")
                                                                        }
                                                                        on:click=move |_| {
                                                                            selected_second.set(s);
                                                                            update_time();
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
                                                                            let mut cls = vec![format!("{}-time-cell", picker_prefix)];
                                                                            if selected_period.get() == period_str {
                                                                                cls.push(format!("{}-time-cell-selected", picker_prefix));
                                                                            }
                                                                            cls.join(" ")
                                                                        }
                                                                        on:click=move |_| {
                                                                            selected_period.set(period_for_click.clone());
                                                                            update_time();
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
                                })
                            } else {
                                None
                            }}

                            // Footer
                            <div class=format!("{}-footer", picker_prefix)>
                                <button
                                    class=format!("{}-now-btn", picker_prefix)
                                    on:click=move |_| {
                                        let now = js_sys::Date::new_0();
                                        let year = now.get_full_year();
                                        let month = now.get_month() + 1;
                                        let day = now.get_date();
                                        let h = now.get_hours();
                                        let m = now.get_minutes();
                                        let s = now.get_seconds();

                                        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);

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

                                        let time_str = format_time(
                                            selected_hour.get_untracked(),
                                            selected_minute.get_untracked(),
                                            selected_second.get_untracked(),
                                            &selected_period.get_untracked()
                                        );

                                        let new_value = DateTimeValue::new(Some(date_str), Some(time_str));
                                        value.set(new_value.clone());
                                        is_open.set(false);

                                        if let Some(ref cb) = on_change {
                                            cb.run(new_value);
                                        }
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
