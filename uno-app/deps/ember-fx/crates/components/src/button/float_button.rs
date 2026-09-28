//! FloatButton and BackTop Leptos components.

use leptos::prelude::*;
use leptos::ev;
use super::types::{ButtonVariant, ButtonShape, FloatButtonPlacement};
use crate::try_use_theme;

/// FloatButton component.
///
/// A floating action button (FAB) that hovers over content in a fixed position.
/// Commonly used for primary actions like compose, add, or help.
///
/// # Props
///
/// - `variant` - Visual variant (Primary, Secondary, etc.)
/// - `shape` - Button shape (Circle by default)
/// - `placement` - Screen position (BottomRight by default)
/// - `offset` - Custom offset from edge [horizontal, vertical] in pixels
/// - `aria_label` - Accessible label (required)
/// - `tooltip` - Tooltip text
/// - `badge` - Badge content (e.g., notification count)
/// - `disabled` - Whether the button is disabled
/// - `class` - Additional CSS classes
/// - `on_click` - Click handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{FloatButton, FloatButtonPlacement, ButtonVariant};
///
/// view! {
///     <FloatButton
///         variant=ButtonVariant::Primary
///         aria_label="Create new item"
///         tooltip="Add new"
///     >
///         "+"
///     </FloatButton>
///
///     // Custom position
///     <FloatButton
///         placement=FloatButtonPlacement::BottomLeft
///         offset=(32, 32)
///         aria_label="Help"
///     >
///         "?"
///     </FloatButton>
/// }
/// ```
#[component]
pub fn FloatButton(
    /// Button variant (visual style).
    #[prop(optional, into)]
    variant: Option<ButtonVariant>,
    /// Button shape (defaults to Circle).
    #[prop(optional, into)]
    shape: Option<ButtonShape>,
    /// Placement position.
    #[prop(optional, into)]
    placement: Option<FloatButtonPlacement>,
    /// Offset from edge [horizontal, vertical] in pixels.
    #[prop(optional)]
    offset: Option<(i32, i32)>,
    /// Use fixed positioning (viewport-relative). Set to false for container-relative.
    #[prop(optional)]
    #[prop(default = true)]
    fixed: bool,
    /// Accessible label (required).
    #[prop(into)]
    aria_label: String,
    /// Tooltip text.
    #[prop(optional, into)]
    tooltip: Option<String>,
    /// Badge content.
    #[prop(optional, into)]
    badge: Option<String>,
    /// Whether the button is disabled.
    #[prop(optional)]
    disabled: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Icon/content.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Resolve defaults
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let shape = shape.unwrap_or(ButtonShape::Circle);
    let placement = placement.unwrap_or_default();
    let offset = offset.unwrap_or((24, 24));

    // Build CSS classes
    let float_prefix = format!("fx-float-btn-{}", design_system);
    let btn_prefix = format!("fx-btn-{}", design_system);

    let combined_class = {
        let mut parts = vec![
            float_prefix.clone(),
            placement.class(&float_prefix),
            variant.class(&btn_prefix),
        ];

        // Add shape class
        let shape_class = shape.class(&btn_prefix);
        if !shape_class.is_empty() {
            parts.push(shape_class);
        }

        if disabled {
            parts.push(format!("{}-disabled", float_prefix));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Custom positioning style
    let position_style = {
        let (h_offset, v_offset) = offset;
        let pos = if fixed { "position: fixed;" } else { "" };
        match placement {
            FloatButtonPlacement::BottomRight => format!("{} right: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::BottomLeft => format!("{} left: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopRight => format!("{} right: {}px; top: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopLeft => format!("{} left: {}px; top: {}px;", pos, h_offset, v_offset),
        }
    };

    // Click handler
    let handle_click = move |ev: ev::MouseEvent| {
        if disabled {
            ev.prevent_default();
            return;
        }
        if let Some(ref cb) = on_click {
            cb.run(ev);
        }
    };

    // Badge class
    let badge_class = format!("{}-badge", float_prefix);

    view! {
        <button
            type="button"
            class=combined_class
            style=position_style
            disabled=disabled
            aria-disabled=move || if disabled { Some("true") } else { None }
            aria-label=aria_label.clone()
            title=tooltip.clone()
            on:click=handle_click
        >
            <span class=format!("{}-content", float_prefix)>
                {children()}
            </span>
            {badge.map(|b| view! {
                <span class=badge_class.clone()>{b}</span>
            })}
        </button>
    }
}

/// FloatButton.Group component.
///
/// Groups multiple FloatButtons together in a stack.
///
/// # Props
///
/// - `placement` - Screen position for the group
/// - `offset` - Custom offset from edge
/// - `class` - Additional CSS classes
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{FloatButton, FloatButtonGroup};
///
/// view! {
///     <FloatButtonGroup>
///         <FloatButton aria_label="Edit">"✏️"</FloatButton>
///         <FloatButton aria_label="Share">"📤"</FloatButton>
///         <FloatButton aria_label="Delete">"🗑️"</FloatButton>
///     </FloatButtonGroup>
/// }
/// ```
#[component]
pub fn FloatButtonGroup(
    /// Placement position.
    #[prop(optional, into)]
    placement: Option<FloatButtonPlacement>,
    /// Offset from edge [horizontal, vertical] in pixels.
    #[prop(optional)]
    offset: Option<(i32, i32)>,
    /// Use fixed positioning (viewport-relative). Set to false for container-relative.
    #[prop(optional)]
    #[prop(default = true)]
    fixed: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Child FloatButtons.
    children: Children,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    let placement = placement.unwrap_or_default();
    let offset = offset.unwrap_or((24, 24));

    let group_prefix = format!("fx-float-btn-group-{}", design_system);

    let combined_class = {
        let mut parts = vec![
            group_prefix.clone(),
            placement.class(&group_prefix),
        ];

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Custom positioning style
    let position_style = {
        let (h_offset, v_offset) = offset;
        let pos = if fixed { "position: fixed;" } else { "" };
        match placement {
            FloatButtonPlacement::BottomRight => format!("{} right: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::BottomLeft => format!("{} left: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopRight => format!("{} right: {}px; top: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopLeft => format!("{} left: {}px; top: {}px;", pos, h_offset, v_offset),
        }
    };

    view! {
        <div class=combined_class style=position_style role="group">
            {children()}
        </div>
    }
}

/// BackTop component.
///
/// A specialized FloatButton that scrolls the page back to the top.
/// Automatically shows/hides based on scroll position.
///
/// # Props
///
/// - `visibility_height` - Scroll distance before showing (default: 400px)
/// - `target` - CSS selector for scroll container (defaults to window)
/// - `duration` - Scroll animation duration in ms (default: 450)
/// - `placement` - Screen position
/// - `class` - Additional CSS classes
/// - `on_click` - Click handler (in addition to scroll)
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::BackTop;
///
/// view! {
///     // Basic usage
///     <BackTop />
///
///     // Custom visibility threshold
///     <BackTop visibility_height=200 />
///
///     // Custom content
///     <BackTop>
///         "↑ Top"
///     </BackTop>
/// }
/// ```
#[component]
pub fn BackTop(
    /// Scroll distance (px) before showing.
    #[prop(optional)]
    visibility_height: Option<i32>,
    /// CSS selector for scroll target (defaults to window).
    #[prop(optional, into)]
    target: Option<String>,
    /// Scroll animation duration in ms.
    #[prop(optional)]
    duration: Option<u32>,
    /// Placement position.
    #[prop(optional, into)]
    placement: Option<FloatButtonPlacement>,
    /// Offset from edge [horizontal, vertical] in pixels.
    #[prop(optional)]
    offset: Option<(i32, i32)>,
    /// Use fixed positioning (viewport-relative). Set to false for container-relative.
    #[prop(optional)]
    #[prop(default = true)]
    fixed: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Custom content (defaults to up arrow).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Resolve defaults
    let visibility_height = visibility_height.unwrap_or(400);
    let _duration = duration.unwrap_or(450);
    let placement = placement.unwrap_or_default();
    let offset = offset.unwrap_or((24, 24));

    // Visibility state
    let is_visible = RwSignal::new(false);

    // Build CSS classes
    let back_top_prefix = format!("fx-back-top-{}", design_system);
    let icon_class = format!("{}-icon", back_top_prefix);

    let combined_class = move || {
        let mut parts = vec![
            back_top_prefix.clone(),
            placement.class(&back_top_prefix),
        ];

        if is_visible.get() {
            parts.push(format!("{}-visible", back_top_prefix));
        }

        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }

        parts.join(" ")
    };

    // Custom positioning style
    let position_style = {
        let (h_offset, v_offset) = offset;
        let pos = if fixed { "position: fixed;" } else { "" };
        match placement {
            FloatButtonPlacement::BottomRight => format!("{} right: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::BottomLeft => format!("{} left: {}px; bottom: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopRight => format!("{} right: {}px; top: {}px;", pos, h_offset, v_offset),
            FloatButtonPlacement::TopLeft => format!("{} left: {}px; top: {}px;", pos, h_offset, v_offset),
        }
    };

    // Setup scroll listener
    let _target_clone = target.clone();
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            if let Some(win) = window() {
                let visibility_height = visibility_height;
                let is_visible = is_visible;

                let closure = Closure::wrap(Box::new(move || {
                    if let Some(win) = window() {
                        let scroll_y = win.scroll_y().unwrap_or(0.0) as i32;
                        is_visible.set(scroll_y > visibility_height);
                    }
                }) as Box<dyn Fn()>);

                let _ = win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    });

    // Click handler
    let handle_click = move |_: ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(win) = window() {
                let _ = win.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }

        if let Some(ref cb) = on_click {
            cb.run(());
        }
    };

    view! {
        <button
            type="button"
            class=combined_class
            style=position_style
            aria-label="Back to top"
            on:click=handle_click
        >
            {match children {
                Some(c) => c().into_any(),
                None => view! {
                    <span class=icon_class>"↑"</span>
                }.into_any(),
            }}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_button_placement_class() {
        let placement = FloatButtonPlacement::BottomRight;
        assert_eq!(placement.class("fx-float-btn"), "fx-float-btn-bottom-right");
    }
}
