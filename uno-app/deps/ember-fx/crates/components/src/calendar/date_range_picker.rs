//! DateRangePicker Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{DatePickerMode, DatePickerSize, DateRangeValue, DateRangePreset};
use crate::try_use_theme;

/// DateRangePicker component.
///
/// Dual date selection with start and end date.
///
/// # Props
///
/// - `value` - Selected date range
/// - `placeholder` - Placeholder texts (start, end)
/// - `mode` - Picker mode (Date, Week, Month, Year)
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `presets` - Quick selection presets
/// - `separator` - Text between dates
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::calendar::{DateRangePicker, DateRangeValue};
///
/// let range = RwSignal::new(DateRangeValue::default());
///
/// view! {
///     <DateRangePicker value=range placeholder=("Start date", "End date") />
/// }
/// ```
#[component]
pub fn DateRangePicker(
    /// Selected date range value.
    #[prop(into)]
    value: RwSignal<DateRangeValue>,
    /// Placeholder texts (start, end).
    #[prop(optional, into)]
    placeholder: Option<(String, String)>,
    /// Picker mode.
    #[prop(optional, into)]
    mode: Option<DatePickerMode>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<DatePickerSize>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Quick selection presets.
    #[prop(optional, into)]
    presets: Option<Vec<DateRangePreset>>,
    /// Separator between dates.
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
    on_change: Option<Callback<DateRangeValue>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let mode = mode.unwrap_or_default();
    let size = size.unwrap_or_default();
    let allow_clear = allow_clear.unwrap_or(true);
    let separator = separator.unwrap_or_else(|| "~".to_string());
    let (start_placeholder, end_placeholder) = placeholder
        .unwrap_or_else(|| ("Start date".to_string(), "End date".to_string()));

    // Build CSS classes
    let picker_prefix = format!("fx-date-range-picker-{}", design_system);
    let mode_class = mode.class(&picker_prefix);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_inputs = picker_prefix.clone();
    let picker_prefix_for_inputs_suffix = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();
    let picker_prefix_for_dropdown_view = picker_prefix.clone();
    let picker_prefix_for_presets = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let active_panel = RwSignal::new("start".to_string()); // "start" or "end"
    let start_year = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_full_year() as i32
    });
    let start_month = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_month() as u32
    });
    let end_year = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_full_year() as i32
    });
    let end_month = RwSignal::new({
        let now = js_sys::Date::new_0();
        (now.get_month() + 1) as u32 % 12
    });

    // Store presets
    let presets_stored = StoredValue::new(presets.clone());

    let combined_class = {
        let class = class.clone();
        move || {
            let mut parts = vec![picker_prefix_for_class.clone(), mode_class.clone()];
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

    // Handle date selection
    let handle_select = move |day: u32, is_start: bool| {
        let (year, month) = if is_start {
            (start_year.get_untracked(), start_month.get_untracked())
        } else {
            (end_year.get_untracked(), end_month.get_untracked())
        };
        let date_str = format!("{:04}-{:02}-{:02}", year, month + 1, day);

        let current = value.get_untracked();
        let new_value = if is_start {
            DateRangeValue::new(Some(date_str), current.end)
        } else {
            DateRangeValue::new(current.start, Some(date_str))
        };

        value.set(new_value.clone());

        // Auto-switch to end panel after selecting start
        if is_start {
            active_panel.set("end".to_string());
        } else {
            is_open.set(false);
        }

        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Handle preset selection
    let handle_preset = move |preset: DateRangePreset| {
        let new_value = DateRangeValue::new(Some(preset.start), Some(preset.end));
        value.set(new_value.clone());
        is_open.set(false);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Handle clear
    let handle_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        let new_value = DateRangeValue::default();
        value.set(new_value.clone());
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Month names
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    // Render calendar panel
    let render_calendar = move |is_start: bool| {
        let picker_prefix = picker_prefix_for_dropdown.clone();
        let year = if is_start { start_year.get() } else { end_year.get() };
        let month = if is_start { start_month.get() } else { end_month.get() };
        let days = days_in_month(year, month);
        let first_day = first_day_of_month(year, month);
        let month_name = month_names[month as usize];

        // Selected date parts
        let val = value.get();
        let selected_date = if is_start { &val.start } else { &val.end };
        let selected_parts: Option<(i32, u32, u32)> = selected_date.as_ref().and_then(|s| {
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

        let handle_prev = {
            move |_: ev::MouseEvent| {
                if is_start {
                    let month = start_month.get_untracked();
                    if month == 0 {
                        start_month.set(11);
                        start_year.update(|y| *y -= 1);
                    } else {
                        start_month.set(month - 1);
                    }
                } else {
                    let month = end_month.get_untracked();
                    if month == 0 {
                        end_month.set(11);
                        end_year.update(|y| *y -= 1);
                    } else {
                        end_month.set(month - 1);
                    }
                }
            }
        };

        let handle_next = {
            move |_: ev::MouseEvent| {
                if is_start {
                    let month = start_month.get_untracked();
                    if month == 11 {
                        start_month.set(0);
                        start_year.update(|y| *y += 1);
                    } else {
                        start_month.set(month + 1);
                    }
                } else {
                    let month = end_month.get_untracked();
                    if month == 11 {
                        end_month.set(0);
                        end_year.update(|y| *y += 1);
                    } else {
                        end_month.set(month + 1);
                    }
                }
            }
        };

        view! {
            <div class=format!("{}-calendar", picker_prefix)>
                // Header
                <div class=format!("{}-header", picker_prefix)>
                    <button
                        class=format!("{}-nav-btn", picker_prefix)
                        on:click=handle_prev
                    >
                        "‹"
                    </button>
                    <span class=format!("{}-header-text", picker_prefix)>
                        {format!("{} {}", month_name, year)}
                    </span>
                    <button
                        class=format!("{}-nav-btn", picker_prefix)
                        on:click=handle_next
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
                        let is_selected = selected_parts.map(|(sy, sm, sd)| {
                            sy == year && sm == (month + 1) && sd == day
                        }).unwrap_or(false);

                        // Check if in range
                        let val = value.get();
                        let current_date = format!("{:04}-{:02}-{:02}", year, month + 1, day);
                        let in_range = match (&val.start, &val.end) {
                            (Some(s), Some(e)) => current_date > *s && current_date < *e,
                            _ => false,
                        };

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
                                    if in_range {
                                        cls.push(format!("{}-cell-in-range", picker_prefix));
                                    }
                                    if is_today {
                                        cls.push(format!("{}-cell-today", picker_prefix));
                                    }
                                    cls.join(" ")
                                }
                                on:click=move |_| handle_select(day, is_start)
                            >
                                {day}
                            </span>
                        }
                    }).collect_view()}
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
                                    "📅"
                                </span>
                            }.into_any()
                        }
                    }
                </span>
            </div>

            // Dropdown with dual calendars
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown_view.clone();
                    let presets_prefix = picker_prefix_for_presets.clone();
                    let presets_list = presets_stored.get_value();

                    Some(view! {
                        <div class=format!("{}-dropdown", picker_prefix)>
                            // Presets panel (if any)
                            {if presets_list.is_some() {
                                let presets = presets_list.clone().unwrap();
                                Some(view! {
                                    <div class=format!("{}-presets", presets_prefix)>
                                        {presets.into_iter().map(|preset| {
                                            let preset_clone = preset.clone();
                                            view! {
                                                <div
                                                    class=format!("{}-preset-item", presets_prefix)
                                                    on:click=move |_| handle_preset(preset_clone.clone())
                                                >
                                                    {preset.label.clone()}
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                })
                            } else {
                                None
                            }}

                            // Dual calendar panels
                            <div class=format!("{}-panels", picker_prefix)>
                                {render_calendar(true)}
                                {render_calendar(false)}
                            </div>

                            // Footer
                            <div class=format!("{}-footer", picker_prefix)>
                                <button
                                    class=format!("{}-today-btn", picker_prefix)
                                    on:click=move |_| {
                                        let now = js_sys::Date::new_0();
                                        let year = now.get_full_year();
                                        let month = now.get_month() + 1;
                                        let day = now.get_date();
                                        let today = format!("{:04}-{:02}-{:02}", year, month, day);
                                        let new_value = DateRangeValue::new(Some(today.clone()), Some(today));
                                        value.set(new_value.clone());
                                        is_open.set(false);
                                        if let Some(ref cb) = on_change {
                                            cb.run(new_value);
                                        }
                                    }
                                >
                                    "Today"
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
