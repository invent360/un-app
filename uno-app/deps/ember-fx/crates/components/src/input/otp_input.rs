//! OtpInput Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{InputSize, ValidationState, OtpLength};
use crate::try_use_theme;
use ember_fx_utils::a11y::generate_id;

/// OtpInput component.
///
/// A one-time password input with separate digit boxes.
/// Supports auto-focus navigation, paste handling, and masking.
///
/// # Props
///
/// - `value` - Controlled value signal (the full OTP string)
/// - `length` - Number of digits (4, 6, or 8)
/// - `size` - Input size (Sm, Md, Lg)
/// - `masked` - Whether to mask digits (show dots)
/// - `disabled` - Whether the input is disabled
/// - `auto_focus` - Focus first input on mount
/// - `validation` - Validation state
/// - `error_message` - Error message to display
/// - `class` - Additional CSS classes
/// - `on_complete` - Handler when all digits entered
/// - `on_change` - Change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{OtpInput, OtpLength};
///
/// let otp = RwSignal::new(String::new());
///
/// view! {
///     // 6-digit OTP (default)
///     <OtpInput
///         value=otp
///         on_complete=move |code| verify_otp(code)
///     />
///
///     // 4-digit masked PIN
///     <OtpInput
///         value=pin
///         length=OtpLength::Four
///         masked=true
///     />
///
///     // With validation
///     <OtpInput
///         value=otp
///         validation=ValidationState::Error
///         error_message="Invalid code"
///     />
/// }
/// ```
#[component]
pub fn OtpInput(
    /// Controlled value signal.
    #[prop(into)]
    value: RwSignal<String>,
    /// Number of digits.
    #[prop(optional, into)]
    length: Option<OtpLength>,
    /// Input size.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Whether to mask digits.
    #[prop(optional)]
    masked: bool,
    /// Whether the input is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Auto-focus first input.
    #[prop(optional)]
    #[prop(default = true)]
    auto_focus: bool,
    /// Validation state.
    #[prop(optional, into)]
    validation: Option<ValidationState>,
    /// Error message.
    #[prop(optional, into)]
    error_message: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Complete handler.
    #[prop(optional, into)]
    on_complete: Option<Callback<String>>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let length = length.unwrap_or_default();
    let size = size.unwrap_or_default();
    let validation = validation.unwrap_or_default();
    let digit_count = length.count();

    // Build CSS classes
    let otp_prefix = format!("fx-otp-input-{}", design_system);

    let wrapper_class = {
        let mut parts = vec![
            otp_prefix.clone(),
            size.class(&otp_prefix),
            format!("{}-length-{}", otp_prefix, digit_count),
        ];

        if disabled {
            parts.push(format!("{}-disabled", otp_prefix));
        }
        if masked {
            parts.push(format!("{}-masked", otp_prefix));
        }

        let val_class = validation.class(&otp_prefix);
        if !val_class.is_empty() {
            parts.push(val_class);
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Track focus index
    let focused_index = RwSignal::new(0usize);

    // Get digit at index
    let get_digit = move |index: usize| -> String {
        value.get().chars().nth(index).map(|c| c.to_string()).unwrap_or_default()
    };

    // Update value at index
    let on_change_clone = on_change.clone();
    let on_complete_clone = on_complete.clone();
    let update_digit = move |index: usize, digit: char| {
        let mut chars: Vec<char> = value.get().chars().collect();

        // Pad with spaces if needed
        while chars.len() <= index {
            chars.push(' ');
        }

        chars[index] = digit;

        // Trim trailing spaces and join
        let new_value: String = chars.into_iter().collect();
        let trimmed = new_value.trim_end().to_string();
        value.set(trimmed.clone());

        if let Some(ref cb) = on_change_clone {
            cb.run(trimmed.clone());
        }

        // Check if complete
        let clean: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() == digit_count {
            if let Some(ref cb) = on_complete_clone {
                cb.run(clean);
            }
        }
    };

    // Clear digit at index
    let on_change_clear = on_change.clone();
    let clear_digit = move |index: usize| {
        let mut chars: Vec<char> = value.get().chars().collect();
        if index < chars.len() {
            chars[index] = ' ';
            let new_value: String = chars.into_iter().collect();
            let trimmed = new_value.trim_end().to_string();
            value.set(trimmed.clone());
            if let Some(ref cb) = on_change_clear {
                cb.run(trimmed);
            }
        }
    };

    // CSS class names
    let digits_class = format!("{}-digits", otp_prefix);
    let digit_class = format!("{}-digit", otp_prefix);
    let error_class = format!("{}-error", otp_prefix);

    // Generate unique ID for this OTP input group
    let otp_id = generate_id("otp");

    // Effect to handle focus changes
    let otp_id_for_effect = otp_id.clone();
    Effect::new(move |prev_index: Option<usize>| {
        let current_index = focused_index.get();

        // Only focus if the index actually changed
        if prev_index != Some(current_index) {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    let input_id = format!("{}-{}", otp_id_for_effect, current_index);
                    if let Ok(Some(element)) = document.query_selector(&format!("#{}", input_id)) {
                        if let Ok(input) = element.dyn_into::<web_sys::HtmlInputElement>() {
                            let _ = input.focus();
                            input.select();
                        }
                    }
                }
            }
        }

        current_index
    });

    view! {
        <div class=wrapper_class>
            <div class=digits_class role="group" aria-label="One-time password">
                {(0..digit_count).map(|index| {
                    let update = update_digit.clone();
                    let clear = clear_digit.clone();
                    let input_id = format!("{}-{}", otp_id, index);

                    // Input handler
                    let handle_input = move |ev: ev::Event| {
                        let input_value = event_target_value(&ev);

                        // Get the last character entered (in case of paste)
                        if let Some(c) = input_value.chars().last() {
                            if c.is_ascii_digit() {
                                update(index, c);
                                // Move to next input
                                if index < digit_count - 1 {
                                    focused_index.set(index + 1);
                                }
                            }
                        }
                    };

                    // Keydown handler
                    let handle_keydown = move |ev: ev::KeyboardEvent| {
                        match ev.key().as_str() {
                            "Backspace" => {
                                ev.prevent_default();
                                let current = get_digit(index);
                                if current.is_empty() && index > 0 {
                                    // Move to previous and clear
                                    focused_index.set(index - 1);
                                    clear(index - 1);
                                } else {
                                    clear(index);
                                    // Stay on current or move back
                                    if index > 0 {
                                        focused_index.set(index - 1);
                                    }
                                }
                            }
                            "Delete" => {
                                ev.prevent_default();
                                clear(index);
                            }
                            "ArrowLeft" if index > 0 => {
                                ev.prevent_default();
                                focused_index.set(index - 1);
                            }
                            "ArrowRight" if index < digit_count - 1 => {
                                ev.prevent_default();
                                focused_index.set(index + 1);
                            }
                            _ => {}
                        }
                    };

                    // Focus handler
                    let handle_focus = move |_: ev::FocusEvent| {
                        focused_index.set(index);
                    };

                    let is_focused = move || focused_index.get() == index;

                    view! {
                        <input
                            id=input_id
                            type={if masked { "password" } else { "text" }}
                            inputmode="numeric"
                            pattern="[0-9]"
                            maxlength=1
                            class=format!("{} {}", digit_class.clone(), if is_focused() { format!("{}-focused", digit_class.clone()) } else { String::new() })
                            class:focused=is_focused
                            value=move || get_digit(index)
                            disabled=disabled
                            aria-label=format!("Digit {}", index + 1)
                            autocomplete="one-time-code"
                            on:input=handle_input
                            on:keydown=handle_keydown
                            on:focus=handle_focus
                        />
                    }
                }).collect::<Vec<_>>()}
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
    fn test_otp_length_count() {
        assert_eq!(OtpLength::Four.count(), 4);
        assert_eq!(OtpLength::Six.count(), 6);
        assert_eq!(OtpLength::Eight.count(), 8);
    }

    #[test]
    fn test_otp_length_default() {
        assert_eq!(OtpLength::default(), OtpLength::Six);
    }
}
