//! InputMask Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{InputSize, InputVariant, ValidationState, MaskType};
use crate::try_use_theme;

/// InputMask component.
///
/// A text input with formatting mask for structured input like
/// phone numbers, credit cards, dates, etc.
///
/// # Mask Pattern Characters
///
/// - `#` - Any digit (0-9)
/// - `A` - Any letter (a-z, A-Z)
/// - `*` - Any alphanumeric character
/// - Other characters are treated as literals
///
/// # Props
///
/// - `value` - Controlled value signal (raw, unformatted value)
/// - `mask` - Mask type or custom pattern
/// - `placeholder_char` - Character to show for unfilled positions (default: '_')
/// - `show_mask` - Whether to show mask placeholder in empty input
/// - `size` - Input size (Sm, Md, Lg)
/// - `variant` - Visual style variant
/// - `disabled` - Whether the input is disabled
/// - `readonly` - Whether the input is readonly
/// - `label` - Label text
/// - `placeholder` - Placeholder text (overrides mask placeholder)
/// - `validation` - Validation state
/// - `error_message` - Error message to display
/// - `class` - Additional CSS classes
/// - `on_change` - Change handler (receives raw value)
/// - `on_complete` - Handler when mask is fully filled
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{InputMask, MaskType};
///
/// let phone = RwSignal::new(String::new());
///
/// view! {
///     // Phone number mask
///     <InputMask
///         value=phone
///         mask=MaskType::Phone
///         label="Phone Number"
///     />
///
///     // Credit card mask
///     <InputMask
///         value=card
///         mask=MaskType::CreditCard
///         placeholder_char=' '
///     />
///
///     // Custom mask
///     <InputMask
///         value=custom
///         mask=MaskType::Custom("AA-####".to_string())
///     />
/// }
/// ```
#[component]
pub fn InputMask(
    /// Controlled value signal (raw value without mask characters).
    #[prop(into)]
    value: RwSignal<String>,
    /// Mask type or pattern.
    #[prop(into)]
    mask: MaskType,
    /// Placeholder character for unfilled positions.
    #[prop(optional)]
    #[prop(default = '_')]
    placeholder_char: char,
    /// Show mask placeholder in input.
    #[prop(optional)]
    #[prop(default = true)]
    show_mask: bool,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Visual variant.
    #[prop(optional, into)]
    variant: Option<InputVariant>,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the input is readonly.
    #[prop(optional)]
    readonly: bool,
    /// Label text.
    #[prop(optional, into)]
    label: Option<String>,
    /// Placeholder text.
    #[prop(optional, into)]
    placeholder: Option<String>,
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
    on_change: Option<Callback<String>>,
    /// Complete handler.
    #[prop(optional, into)]
    on_complete: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let validation = validation.unwrap_or_default();

    // Get the mask pattern
    let pattern = mask.pattern().to_string();
    let pattern_stored = StoredValue::new(pattern.clone());

    // Calculate the number of input positions
    let input_positions: Vec<(usize, char)> = pattern
        .chars()
        .enumerate()
        .filter(|(_, c)| matches!(c, '#' | 'A' | '*'))
        .collect();
    let max_raw_length = input_positions.len();

    // Format raw value to display value
    let format_display = move |raw: &str| -> String {
        let pattern = pattern_stored.get_value();
        let mut result = String::new();
        let mut raw_chars = raw.chars();

        for c in pattern.chars() {
            match c {
                '#' | 'A' | '*' => {
                    if let Some(rc) = raw_chars.next() {
                        result.push(rc);
                    } else if show_mask {
                        result.push(placeholder_char);
                    } else {
                        break;
                    }
                }
                _ => result.push(c),
            }
        }

        result
    };

    // Parse display value to raw value
    let parse_raw = move |display: &str| -> String {
        let pattern = pattern_stored.get_value();
        let mut result = String::new();
        let display_chars: Vec<char> = display.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        for (i, pc) in pattern_chars.iter().enumerate() {
            if i >= display_chars.len() {
                break;
            }

            let dc = display_chars[i];

            match pc {
                '#' => {
                    if dc.is_ascii_digit() {
                        result.push(dc);
                    }
                }
                'A' => {
                    if dc.is_ascii_alphabetic() {
                        result.push(dc);
                    }
                }
                '*' => {
                    if dc.is_ascii_alphanumeric() {
                        result.push(dc);
                    }
                }
                _ => {
                    // Skip literal characters
                }
            }
        }

        result
    };

    // Validate character at position
    let validate_char = move |c: char, pos: usize| -> bool {
        let pattern = pattern_stored.get_value();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        // Find the pattern position for this raw position
        let mut raw_pos = 0;
        for pc in pattern_chars.iter() {
            match pc {
                '#' => {
                    if raw_pos == pos {
                        return c.is_ascii_digit();
                    }
                    raw_pos += 1;
                }
                'A' => {
                    if raw_pos == pos {
                        return c.is_ascii_alphabetic();
                    }
                    raw_pos += 1;
                }
                '*' => {
                    if raw_pos == pos {
                        return c.is_ascii_alphanumeric();
                    }
                    raw_pos += 1;
                }
                _ => {}
            }
        }

        false
    };

    // Build CSS classes
    let mask_prefix = format!("fx-input-mask-{}", design_system);

    let wrapper_class = {
        let mut parts = vec![
            mask_prefix.clone(),
            size.class(&mask_prefix),
            variant.class(&mask_prefix),
            mask.class(&mask_prefix),
        ];

        if disabled {
            parts.push(format!("{}-disabled", mask_prefix));
        }
        if readonly {
            parts.push(format!("{}-readonly", mask_prefix));
        }

        let val_class = validation.class(&mask_prefix);
        if !val_class.is_empty() {
            parts.push(val_class);
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Handle input
    let on_change_clone = on_change.clone();
    let on_complete_clone = on_complete.clone();
    let handle_input = move |ev: ev::Event| {
        if disabled || readonly {
            return;
        }

        let input_value = event_target_value(&ev);

        // Extract only valid characters based on the mask
        let raw = parse_raw(&input_value);

        // Validate and filter characters
        let validated: String = raw
            .chars()
            .enumerate()
            .filter(|(i, c)| validate_char(*c, *i))
            .map(|(_, c)| c)
            .take(max_raw_length)
            .collect();

        value.set(validated.clone());

        if let Some(ref cb) = on_change_clone {
            cb.run(validated.clone());
        }

        // Check if complete
        if validated.len() == max_raw_length {
            if let Some(ref cb) = on_complete_clone {
                cb.run(validated);
            }
        }
    };

    // Handle keydown for special keys
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        // Allow navigation and deletion keys
        match ev.key().as_str() {
            "Backspace" | "Delete" | "ArrowLeft" | "ArrowRight" |
            "Home" | "End" | "Tab" => {
                // Allow these keys
            }
            _ => {
                // For other keys, we handle validation in input event
            }
        }
    };

    // CSS class names
    let label_class = format!("{}-label", mask_prefix);
    let input_wrapper_class = format!("{}-input-wrapper", mask_prefix);
    let input_class = format!("{}-input", mask_prefix);
    let error_class = format!("{}-error", mask_prefix);

    // Generate placeholder from mask
    let mask_placeholder = if show_mask {
        format_display("")
    } else {
        placeholder.clone().unwrap_or_default()
    };

    view! {
        <div class=wrapper_class>
            {label.map(|l| view! {
                <label class=label_class.clone()>{l}</label>
            })}

            <div class=input_wrapper_class>
                <input
                    type="text"
                    class=input_class
                    value=move || format_display(&value.get())
                    placeholder=placeholder.clone().unwrap_or(mask_placeholder.clone())
                    disabled=disabled
                    readonly=readonly
                    aria-invalid=validation.is_error().to_string()
                    on:input=handle_input
                    on:keydown=handle_keydown
                />
            </div>

            {error_message.clone().filter(|_| validation.is_error()).map(|msg| view! {
                <span class=error_class.clone()>{msg}</span>
            })}
        </div>
    }
}

/// Get the inputmode attribute based on mask type.
impl MaskType {
    /// Get the recommended inputmode for this mask type.
    pub fn inputmode(&self) -> &'static str {
        match self {
            MaskType::Phone | MaskType::CreditCard | MaskType::Date |
            MaskType::Time | MaskType::Currency => "numeric",
            MaskType::Custom(_) => "text",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_type_pattern() {
        assert_eq!(MaskType::Phone.pattern(), "(###) ###-####");
        assert_eq!(MaskType::CreditCard.pattern(), "#### #### #### ####");
        assert_eq!(MaskType::Date.pattern(), "##/##/####");
    }

    #[test]
    fn test_mask_type_inputmode() {
        assert_eq!(MaskType::Phone.inputmode(), "numeric");
        assert_eq!(MaskType::Custom("AA-##".to_string()).inputmode(), "text");
    }
}
