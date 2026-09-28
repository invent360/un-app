//! TriaState Leptos component.
//!
//! A segmented control component optimized for 3-state selection (e.g., Daily | Weekly | Monthly).

use leptos::prelude::*;
use super::types::SelectionSize;
use crate::try_use_theme;

/// Option for TriaState component.
#[derive(Debug, Clone, PartialEq)]
pub struct TriaStateOption {
    /// The value of the option.
    pub value: String,
    /// Display label.
    pub label: String,
}

impl TriaStateOption {
    /// Create a new option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }

    /// Create an option where value equals label.
    pub fn simple(label: impl Into<String>) -> Self {
        let l = label.into();
        Self::new(l.clone(), l)
    }
}

/// TriaState component.
///
/// A segmented control for selecting one of three options.
/// Provides a compact, visually appealing way to switch between states.
///
/// # Props
///
/// - `options` - Exactly 3 options to display
/// - `value` - Signal for the selected value
/// - `size` - Size variant (Sm, Md, Lg)
/// - `disabled` - Whether the control is disabled
/// - `block` - Whether the control should take full width
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::selection::{TriaState, TriaStateOption};
///
/// let selected = RwSignal::new("weekly".to_string());
///
/// view! {
///     <TriaState
///         options=vec![
///             TriaStateOption::new("daily", "Daily"),
///             TriaStateOption::new("weekly", "Weekly"),
///             TriaStateOption::new("monthly", "Monthly"),
///         ]
///         value=selected
///     />
/// }
/// ```
#[component]
pub fn TriaState(
    /// The three options to display.
    #[prop(into)]
    options: Vec<TriaStateOption>,
    /// Selected value signal.
    #[prop(into)]
    value: RwSignal<String>,
    /// Size variant.
    #[prop(optional, into)]
    size: Option<SelectionSize>,
    /// Whether the control is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the control should take full width.
    #[prop(optional)]
    block: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Change handler.
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let size = size.unwrap_or_default();

    // Build CSS classes
    let tria_prefix = format!("fx-tria-state-{}", design_system);

    let size_class = match size {
        SelectionSize::Sm => format!("{}-sm", tria_prefix),
        SelectionSize::Lg => format!("{}-lg", tria_prefix),
        SelectionSize::Md => String::new(),
    };

    // Combined container class
    let combined_class = {
        let mut parts = vec![tria_prefix.clone()];
        if !size_class.is_empty() {
            parts.push(size_class);
        }
        if disabled {
            parts.push(format!("{}-disabled", tria_prefix));
        }
        if block {
            parts.push(format!("{}-block", tria_prefix));
        }
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    let option_base_class = format!("{}-option", tria_prefix);
    let option_selected_class = format!("{}-option-selected", tria_prefix);

    view! {
        <div class=combined_class role="radiogroup">
            {options.into_iter().map(|opt| {
                let opt_value_for_click = opt.value.clone();
                let opt_value_for_class = opt.value.clone();
                let option_base = option_base_class.clone();
                let option_selected = option_selected_class.clone();

                let handle_click = move |_: leptos::ev::MouseEvent| {
                    if disabled {
                        return;
                    }
                    value.set(opt_value_for_click.clone());
                    if let Some(ref cb) = on_change {
                        cb.run(opt_value_for_click.clone());
                    }
                };

                view! {
                    <button
                        type="button"
                        class=move || {
                            let mut cls = vec![option_base.clone()];
                            if value.get() == opt_value_for_class {
                                cls.push(option_selected.clone());
                            }
                            cls.join(" ")
                        }
                        disabled=disabled
                        on:click=handle_click
                    >
                        {opt.label}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tria_state_option() {
        let opt = TriaStateOption::new("value1", "Label 1");
        assert_eq!(opt.value, "value1");
        assert_eq!(opt.label, "Label 1");

        let simple = TriaStateOption::simple("Option");
        assert_eq!(simple.value, "Option");
        assert_eq!(simple.label, "Option");
    }
}
