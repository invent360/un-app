//! Rate Leptos component.

use leptos::prelude::*;
use super::types::RateCharacter;
use crate::try_use_theme;

/// Rate component.
///
/// Star rating input component.
///
/// # Props
///
/// - `value` - Current rating value
/// - `count` - Total number of stars
/// - `allow_half` - Allow half-star ratings
/// - `allow_clear` - Allow clearing the rating
/// - `disabled` - Disabled state
/// - `character` - Custom character (star, heart, or custom)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::form_advanced::Rate;
///
/// let rating = RwSignal::new(3.0);
///
/// view! {
///     <Rate value=rating />
/// }
/// ```
#[component]
pub fn Rate(
    /// Current rating value.
    #[prop(into)]
    value: RwSignal<f64>,
    /// Total number of stars.
    #[prop(optional)]
    count: Option<usize>,
    /// Allow half-star ratings.
    #[prop(optional)]
    allow_half: bool,
    /// Allow clearing the rating.
    #[prop(optional)]
    allow_clear: Option<bool>,
    /// Disabled state.
    #[prop(optional)]
    disabled: bool,
    /// Custom character.
    #[prop(optional, into)]
    character: Option<RateCharacter>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change callback.
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let count = count.unwrap_or(5);
    let allow_clear = allow_clear.unwrap_or(true);
    let character = character.unwrap_or_default();

    // Build CSS classes
    let rate_prefix = format!("fx-rate-{}", design_system);

    let rate_prefix_for_class = rate_prefix.clone();

    let combined_class = {
        let rate_prefix = rate_prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![rate_prefix.clone()];
            if disabled {
                parts.push(format!("{}-disabled", rate_prefix));
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Hover state
    let hover_value = RwSignal::new(None::<f64>);

    // Handle click
    let handle_click = move |star_index: usize, is_half: bool| {
        if disabled {
            return;
        }

        let new_value = if allow_half && is_half {
            star_index as f64 + 0.5
        } else {
            (star_index + 1) as f64
        };

        // Allow clearing if clicking the same value
        let current = value.get();
        let final_value = if allow_clear && (new_value - current).abs() < 0.01 {
            0.0
        } else {
            new_value
        };

        value.set(final_value);
        if let Some(ref cb) = on_change {
            cb.run(final_value);
        }
    };

    // Handle hover
    let handle_hover = move |star_index: usize, is_half: bool| {
        if disabled {
            return;
        }

        let hover_val = if allow_half && is_half {
            star_index as f64 + 0.5
        } else {
            (star_index + 1) as f64
        };
        hover_value.set(Some(hover_val));
    };

    view! {
        <ul
            class=combined_class
            on:mouseleave=move |_| hover_value.set(None)
        >
            {(0..count).map(|i| {
                let rate_prefix = rate_prefix_for_class.clone();
                let character = character.clone();
                let filled_char = character.as_str().to_string();
                let _empty_char = character.empty_str().to_string();

                view! {
                    <li
                        class=move || {
                            let display_value = hover_value.get().unwrap_or_else(|| value.get());
                            let star_value = (i + 1) as f64;
                            let half_value = i as f64 + 0.5;

                            let mut cls = vec![format!("{}-star", rate_prefix)];

                            if display_value >= star_value {
                                cls.push(format!("{}-star-full", rate_prefix));
                            } else if allow_half && display_value >= half_value {
                                cls.push(format!("{}-star-half", rate_prefix));
                            } else {
                                cls.push(format!("{}-star-zero", rate_prefix));
                            }

                            cls.join(" ")
                        }
                    >
                        {if allow_half {
                            let rate_prefix_first = rate_prefix.clone();
                            let rate_prefix_second = rate_prefix.clone();
                            let filled_first = filled_char.clone();
                            let filled_second = filled_char.clone();
                            view! {
                                <div
                                    class=format!("{}-star-first", rate_prefix_first)
                                    on:click=move |_| handle_click(i, true)
                                    on:mouseenter=move |_| handle_hover(i, true)
                                >
                                    {filled_first}
                                </div>
                                <div
                                    class=format!("{}-star-second", rate_prefix_second)
                                    on:click=move |_| handle_click(i, false)
                                    on:mouseenter=move |_| handle_hover(i, false)
                                >
                                    {filled_second}
                                </div>
                            }.into_any()
                        } else {
                            let filled = filled_char.clone();
                            view! {
                                <div
                                    class=format!("{}-star-content", rate_prefix)
                                    on:click=move |_| handle_click(i, false)
                                    on:mouseenter=move |_| handle_hover(i, false)
                                >
                                    {filled}
                                </div>
                            }.into_any()
                        }}
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}
