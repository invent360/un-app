//! DatePicker Leptos component.

use leptos::prelude::*;
use super::types::{DatePickerMode, DatePickerSize};
use crate::try_use_theme;

/// DatePicker component.
///
/// Date selection input with calendar dropdown.
///
/// # Props
///
/// - `value` - Selected date (YYYY-MM-DD format)
/// - `placeholder` - Placeholder text
/// - `mode` - Picker mode (Date, Week, Month, Quarter, Year)
/// - `size` - Size variant
/// - `disabled` - Disabled state
/// - `allow_clear` - Show clear button
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::calendar::DatePicker;
///
/// let date = RwSignal::new(None::<String>);
///
/// view! {
///     <DatePicker value=date placeholder="Select date" />
/// }
/// ```
#[component]
pub fn DatePicker(
    /// Selected date value (YYYY-MM-DD format).
    #[prop(into)]
    value: RwSignal<Option<String>>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Picker mode.
    #[prop(optional, into)]
    mode: Option<DatePickerMode>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<DatePickerSize>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Show clear button.
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
    let mode = mode.unwrap_or_default();
    let size = size.unwrap_or_default();
    let allow_clear = allow_clear.unwrap_or(true);
    let placeholder = placeholder.unwrap_or_else(|| "Select date".to_string());

    // Build CSS classes
    let picker_prefix = format!("fx-date-picker-{}", design_system);
    let mode_class = mode.class(&picker_prefix);
    let size_class = size.class(&picker_prefix);

    let picker_prefix_for_class = picker_prefix.clone();
    let picker_prefix_for_input = picker_prefix.clone();
    let picker_prefix_for_icon = picker_prefix.clone();
    let picker_prefix_for_clear = picker_prefix.clone();
    let picker_prefix_for_dropdown = picker_prefix.clone();

    // State
    let is_open = RwSignal::new(false);
    let current_year = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_full_year() as i32
    });
    let current_month = RwSignal::new({
        let now = js_sys::Date::new_0();
        now.get_month() as u32
    });

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
    let handle_select = move |day: u32| {
        let year = current_year.get_untracked();
        let month = current_month.get_untracked() + 1;
        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
        value.set(Some(date_str.clone()));
        is_open.set(false);
        if let Some(ref cb) = on_change {
            cb.run(Some(date_str));
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
                    value=move || value.get().unwrap_or_default()
                />
                <span class=format!("{}-suffix", picker_prefix_for_icon)>
                    {if allow_clear && value.get().is_some() {
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
                                "📅"
                            </span>
                        }.into_any()
                    }}
                </span>
            </div>

            // Dropdown calendar
            {move || {
                if is_open.get() {
                    let picker_prefix = picker_prefix_for_dropdown.clone();
                    let year = current_year.get();
                    let month = current_month.get();
                    let days = days_in_month(year, month);
                    let first_day = first_day_of_month(year, month);
                    let month_name = month_names[month as usize];

                    // Selected date parts
                    let selected_parts: Option<(i32, u32, u32)> = value.get().and_then(|s| {
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
                                    let is_selected = selected_parts.map(|(sy, sm, sd)| {
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
                                            on:click=move |_| handle_select(day)
                                        >
                                            {day}
                                        </span>
                                    }
                                }).collect_view()}
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
                                        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
                                        value.set(Some(date_str.clone()));
                                        is_open.set(false);
                                        if let Some(ref cb) = on_change {
                                            cb.run(Some(date_str));
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
