//! InputGroup Leptos component.

use leptos::prelude::*;
use super::types::InputSize;
use crate::try_use_theme;

/// InputGroup component.
///
/// A wrapper for combining inputs with addons (prefix/suffix buttons or text).
/// Useful for search inputs, URL inputs, or inputs with action buttons.
///
/// # Props
///
/// - `size` - Size variant (Sm, Md, Lg) - applies to all children
/// - `compact` - Whether to use compact spacing
/// - `class` - Additional CSS classes
/// - `children` - Input and addon elements
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::{InputGroup, InputAddon, TextInput};
///
/// view! {
///     <InputGroup>
///         <InputAddon>"https://"</InputAddon>
///         <TextInput placeholder="example.com" />
///         <InputAddon>".com"</InputAddon>
///     </InputGroup>
/// }
/// ```
#[component]
pub fn InputGroup(
    /// Size variant for all children.
    #[prop(optional, into)]
    size: Option<InputSize>,
    /// Use compact spacing.
    #[prop(optional)]
    compact: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Child elements (inputs and addons).
    children: Children,
) -> impl IntoView {
    // Get theme context for design system prefix
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let size = size.unwrap_or_default();

    // Build CSS classes - pattern: fx-{component}-{design_system}
    let group_prefix = format!("fx-input-group-{}", design_system);
    let base_class = group_prefix.clone();
    let size_class = if size != InputSize::Md {
        size.class(&group_prefix)
    } else {
        String::new()
    };

    // State classes
    let state_classes = {
        let mut classes = Vec::new();
        if compact {
            classes.push(format!("{}-compact", group_prefix));
        }
        classes.join(" ")
    };

    // Combine all classes
    let combined_class = {
        let mut parts = vec![base_class];
        if !size_class.is_empty() {
            parts.push(size_class);
        }
        if !state_classes.is_empty() {
            parts.push(state_classes);
        }
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}

/// InputAddon component.
///
/// A visual addon that attaches to the side of an input within an InputGroup.
/// Can contain text, icons, or buttons.
///
/// # Props
///
/// - `class` - Additional CSS classes
/// - `children` - Addon content
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::InputAddon;
///
/// view! {
///     <InputAddon>"$"</InputAddon>
///     <InputAddon>
///         <Icon name="search" />
///     </InputAddon>
/// }
/// ```
#[component]
pub fn InputAddon(
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Addon content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system prefix
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let addon_class = format!("fx-input-addon-{}", design_system);
    let combined_class = if let Some(custom) = class {
        format!("{} {}", addon_class, custom)
    } else {
        addon_class
    };

    view! {
        <span class=combined_class>
            {children()}
        </span>
    }
}

/// InputButton component.
///
/// A button addon that attaches to an input within an InputGroup.
/// Styled to visually connect with adjacent inputs.
///
/// # Props
///
/// - `disabled` - Whether the button is disabled
/// - `loading` - Whether the button is in loading state
/// - `class` - Additional CSS classes
/// - `on_click` - Click handler
/// - `children` - Button content
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::input::InputButton;
///
/// view! {
///     <InputButton on_click=move |_| search()>
///         "Search"
///     </InputButton>
/// }
/// ```
#[component]
pub fn InputButton(
    /// Whether the button is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Whether the button is in loading state.
    #[prop(optional)]
    loading: bool,
    /// Button type attribute.
    #[prop(optional, into)]
    button_type: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    /// Button content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system prefix
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let button_class = format!("fx-input-button-{}", design_system);
    let button_type = button_type.unwrap_or_else(|| "button".to_string());

    // State classes
    let state_classes = {
        let mut classes = Vec::new();
        if disabled || loading {
            classes.push(format!("{}-disabled", button_class));
        }
        if loading {
            classes.push(format!("{}-loading", button_class));
        }
        classes.join(" ")
    };

    let combined_class = {
        let mut parts = vec![button_class];
        if !state_classes.is_empty() {
            parts.push(state_classes);
        }
        if let Some(custom) = class {
            parts.push(custom);
        }
        parts.join(" ")
    };

    // Click handler that respects disabled/loading state
    let handle_click = move |ev: leptos::ev::MouseEvent| {
        if disabled || loading {
            ev.prevent_default();
            return;
        }
        if let Some(ref cb) = on_click {
            cb.run(ev);
        }
    };

    view! {
        <button
            type=button_type
            class=combined_class
            disabled=disabled || loading
            on:click=handle_click
        >
            {children()}
        </button>
    }
}
