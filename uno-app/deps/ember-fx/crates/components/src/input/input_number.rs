//! InputNumber Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{InputSize, ValidationState, StepperPosition};
use crate::try_use_theme;

/// InputNumber component.
///
/// A number input with increment/decrement stepper buttons.
/// Supports min/max constraints, step values, and keyboard controls.
///
/// # Props
///
/// - `value` - Controlled value signal
/// - `min` - Minimum allowed value
/// - `max` - Maximum allowed value
/// - `step` - Step increment (default: 1.0)
/// - `precision` - Decimal places to display
/// - `size` - Input size (Sm, Md, Lg)
/// - `stepper_position` - Button placement (Right, Sides, None)
/// - `disabled` - Whether the input is disabled
/// - `readonly` - Whether the input is readonly
/// - `keyboard` - Enable keyboard up/down arrows (default: true)
/// - `label` - Label text
/// - `placeholder` - Placeholder text
/// - `prefix` - Prefix content (e.g., "$")
/// - `suffix` - Suffix content (e.g., "kg")
/// - `validation` - Validation state
/// - `error_message` - Error message to display
/// - `class` - Additional CSS classes
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{InputNumber, StepperPosition};
///
/// let quantity = RwSignal::new(1.0);
///
/// view! {
///     <InputNumber
///         value=quantity
///         min=0.0
///         max=100.0
///         step=1.0
///         label="Quantity"
///     />
///
///     // With currency prefix
///     <InputNumber
///         value=price
///         prefix="$"
///         precision=2
///         stepper_position=StepperPosition::None
///     />
/// }
/// ```
#[component]
pub fn InputNumber(
    /// Controlled value signal.
    #[prop(into)]
    value: RwSignal<f64>,
    /// Minimum allowed value.
    #[prop(optional)]
    min: Option<f64>,
    /// Maximum allowed value.
    #[prop(optional)]
    max: Option<f64>,
    /// Step increment.
    #[prop(optional)]
    step: Option<f64>,
    /// Decimal precision.
    #[prop(optional)]
    precision: Option<usize>,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Stepper button position.
    #[prop(optional, into)]
    stepper_position: Option<StepperPosition>,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the input is readonly.
    #[prop(optional)]
    readonly: bool,
    /// Enable keyboard controls.
    #[prop(optional)]
    #[prop(default = true)]
    keyboard: bool,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Prefix content.
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Suffix content.
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Validation state.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Error message.
    #[prop(optional, into)]
    error_message: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let stepper_position = stepper_position.unwrap_or_default();
    let step = step.unwrap_or(1.0);
    let validation = validation.unwrap_or_default();

    // Build CSS classes
    let input_number_prefix = format!("fx-input-number-{}", design_system);

    let wrapper_class = {
        let mut parts = vec![
            input_number_prefix.clone(),
            size.class(&input_number_prefix),
            stepper_position.class(&input_number_prefix),
        ];

        if disabled {
            parts.push(format!("{}-disabled", input_number_prefix));
        }
        if readonly {
            parts.push(format!("{}-readonly", input_number_prefix));
        }

        let val_class = validation.class(&input_number_prefix);
        if !val_class.is_empty() {
            parts.push(val_class);
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Format value for display
    let format_value = move |v: f64| -> String {
        match precision {
            Some(p) => format!("{:.prec$}", v, prec = p),
            None => {
                // Auto-detect: show decimals only if needed
                if v.fract() == 0.0 {
                    format!("{:.0}", v)
                } else {
                    format!("{}", v)
                }
            }
        }
    };

    // Clamp value to min/max
    let clamp_value = move |v: f64| -> f64 {
        let mut clamped = v;
        if let Some(min_val) = min {
            clamped = clamped.max(min_val);
        }
        if let Some(max_val) = max {
            clamped = clamped.min(max_val);
        }
        clamped
    };

    // Increment handler
    let handle_increment = move |_: ev::MouseEvent| {
        if disabled || readonly {
            return;
        }
        let new_value = clamp_value(value.get() + step);
        value.set(new_value);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Decrement handler
    let handle_decrement = move |_: ev::MouseEvent| {
        if disabled || readonly {
            return;
        }
        let new_value = clamp_value(value.get() - step);
        value.set(new_value);
        if let Some(ref cb) = on_change {
            cb.run(new_value);
        }
    };

    // Input change handler
    let on_change_clone = on_change.clone();
    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        if let Ok(parsed) = input_value.parse::<f64>() {
            let clamped = clamp_value(parsed);
            value.set(clamped);
            if let Some(ref cb) = on_change_clone {
                cb.run(clamped);
            }
        }
    };

    // Keyboard handler
    let on_change_kb = on_change.clone();
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if !keyboard || disabled || readonly {
            return;
        }

        match ev.key().as_str() {
            "ArrowUp" => {
                ev.prevent_default();
                let new_value = clamp_value(value.get() + step);
                value.set(new_value);
                if let Some(ref cb) = on_change_kb {
                    cb.run(new_value);
                }
            }
            "ArrowDown" => {
                ev.prevent_default();
                let new_value = clamp_value(value.get() - step);
                value.set(new_value);
                if let Some(ref cb) = on_change_kb {
                    cb.run(new_value);
                }
            }
            _ => {}
        }
    };

    // CSS class names
    let label_class = format!("{}-label", input_number_prefix);
    let input_wrapper_class = format!("{}-input-wrapper", input_number_prefix);
    let input_class = format!("{}-input", input_number_prefix);
    let prefix_class = format!("{}-prefix", input_number_prefix);
    let suffix_class = format!("{}-suffix", input_number_prefix);
    let stepper_class = format!("{}-stepper", input_number_prefix);
    let stepper_up_class = format!("{}-up", stepper_class);
    let stepper_down_class = format!("{}-down", stepper_class);
    let error_class = format!("{}-error", input_number_prefix);

    // Check if at limits
    let at_min = move || min.map(|m| value.get() <= m).unwrap_or(false);
    let at_max = move || max.map(|m| value.get() >= m).unwrap_or(false);

    view! {
        <div class=wrapper_class>
            {label.map(|l| view! {
                <label class=label_class.clone()>{l}</label>
            })}

            <div class=input_wrapper_class>
                // Left stepper (for Sides position)
                {(stepper_position == StepperPosition::Sides).then(|| view! {
                    <button
                        type="button"
                        class=format!("{} {}-left", stepper_class.clone(), stepper_class.clone())
                        disabled=disabled || readonly || at_min()
                        aria-label="Decrease"
                        tabindex=-1
                        on:click=handle_decrement.clone()
                    >
                        "−"
                    </button>
                })}

                {prefix.map(|p| view! {
                    <span class=prefix_class.clone()>{p}</span>
                })}

                <input
                    type="text"
                    inputmode="decimal"
                    class=input_class
                    value=move || format_value(value.get())
                    placeholder=placeholder.clone()
                    disabled=disabled
                    readonly=readonly
                    aria-invalid=validation.is_error().to_string()
                    aria-valuemin=min.map(|m| m.to_string())
                    aria-valuemax=max.map(|m| m.to_string())
                    aria-valuenow=move || value.get().to_string()
                    role="spinbutton"
                    on:input=handle_input
                    on:keydown=handle_keydown
                />

                {suffix.map(|s| view! {
                    <span class=suffix_class.clone()>{s}</span>
                })}

                // Right stepper (for Right position)
                {(stepper_position == StepperPosition::Right).then(|| view! {
                    <div class=stepper_class.clone()>
                        <button
                            type="button"
                            class=stepper_up_class.clone()
                            disabled=disabled || readonly || at_max()
                            aria-label="Increase"
                            tabindex=-1
                            on:click=handle_increment.clone()
                        >
                            "▲"
                        </button>
                        <button
                            type="button"
                            class=stepper_down_class.clone()
                            disabled=disabled || readonly || at_min()
                            aria-label="Decrease"
                            tabindex=-1
                            on:click=handle_decrement.clone()
                        >
                            "▼"
                        </button>
                    </div>
                })}

                // Right stepper (for Sides position)
                {(stepper_position == StepperPosition::Sides).then(|| view! {
                    <button
                        type="button"
                        class=format!("{} {}-right", stepper_class.clone(), stepper_class.clone())
                        disabled=disabled || readonly || at_max()
                        aria-label="Increase"
                        tabindex=-1
                        on:click=handle_increment.clone()
                    >
                        "+"
                    </button>
                })}
            </div>

            {error_message.clone().filter(|_| validation.is_error()).map(|msg| view! {
                <span class=error_class.clone()>{msg}</span>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stepper_position_class() {
        let pos = StepperPosition::Sides;
        assert_eq!(pos.class("fx-input-number"), "fx-input-number-stepper-sides");
    }
}
