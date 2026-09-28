//! ButtonGroup Leptos component.

use leptos::prelude::*;
use super::types::ButtonSize;
use crate::try_use_theme;

/// ButtonGroup component.
///
/// Groups multiple buttons together with consistent spacing and styling.
/// Can be horizontal (default) or vertical, and attached (no gaps) or spaced.
///
/// # Props
///
/// - `size` - Size for all child buttons (propagated via context)
/// - `vertical` - Whether to stack buttons vertically
/// - `attached` - Whether buttons are attached (no gaps, connected borders)
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{Button, ButtonGroup, ButtonVariant};
///
/// view! {
///     <ButtonGroup>
///         <Button variant=ButtonVariant::Primary>"Save"</Button>
///         <Button variant=ButtonVariant::Outline>"Cancel"</Button>
///     </ButtonGroup>
///
///     // Vertical attached group
///     <ButtonGroup vertical=true attached=true>
///         <Button>"Option 1"</Button>
///         <Button>"Option 2"</Button>
///         <Button>"Option 3"</Button>
///     </ButtonGroup>
/// }
/// ```
#[component]
pub fn ButtonGroup(
    /// Size for all child buttons.
    #[prop(optional, into)]
    size: Option<ButtonSize>,
    /// Whether to stack buttons vertically.
    #[prop(optional)]
    vertical: bool,
    /// Whether buttons are attached (no gaps).
    #[prop(optional)]
    attached: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Child buttons.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Build CSS classes
    let group_prefix = format!("fx-btn-group-{}", design_system);

    let combined_class = {
        let mut parts = vec![group_prefix.clone()];

        if vertical {
            parts.push(format!("{}-vertical", group_prefix));
        }

        if attached {
            parts.push(format!("{}-attached", group_prefix));
        }

        if let Some(ref s) = size {
            parts.push(s.class(&group_prefix));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Provide size context to children if specified
    // Note: This requires children to check for ButtonGroupContext
    if let Some(size) = size {
        provide_context(ButtonGroupContext { size });
    }

    view! {
        <div class=combined_class role="group">
            {children()}
        </div>
    }
}

/// Context provided by ButtonGroup to its children.
#[derive(Clone, Copy)]
pub struct ButtonGroupContext {
    /// Size from the group.
    pub size: ButtonSize,
}

/// Try to get the ButtonGroup context.
pub fn try_use_button_group() -> Option<ButtonGroupContext> {
    use_context::<ButtonGroupContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_group_context() {
        let ctx = ButtonGroupContext { size: ButtonSize::Lg };
        assert_eq!(ctx.size, ButtonSize::Lg);
    }
}
