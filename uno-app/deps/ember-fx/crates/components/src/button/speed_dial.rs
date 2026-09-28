//! SpeedDial Leptos component.

use leptos::prelude::*;
use leptos::ev;
use super::types::{ButtonVariant, ButtonShape, FloatButtonPlacement, SpeedDialDirection, SpeedDialAction};
use crate::try_use_theme;

/// SpeedDial component.
///
/// An expanding floating action button that reveals multiple actions.
/// Similar to Material Design's Speed Dial pattern.
///
/// # Props
///
/// - `actions` - List of actions to display when expanded
/// - `direction` - Direction to expand (Up, Down, Left, Right)
/// - `placement` - Screen position
/// - `default_open` - Whether to start in open state
/// - `open` - Controlled open state signal
/// - `open_on_hover` - Whether to open on hover vs click
/// - `close_on_action` - Whether to close after clicking an action
/// - `icon` - Main button icon (default state)
/// - `open_icon` - Main button icon when open
/// - `aria_label` - Accessible label
/// - `class` - Additional CSS classes
/// - `on_action` - Action click handler (receives action key)
/// - `on_open_change` - Open state change handler
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::{SpeedDial, SpeedDialAction, SpeedDialDirection};
///
/// let actions = vec![
///     SpeedDialAction::new("copy", "📋").with_tooltip("Copy"),
///     SpeedDialAction::new("save", "💾").with_tooltip("Save"),
///     SpeedDialAction::new("print", "🖨️").with_tooltip("Print"),
/// ];
///
/// view! {
///     <SpeedDial
///         actions=actions
///         direction=SpeedDialDirection::Up
///         aria_label="Actions"
///         on_action=move |key| log::info!("Action: {}", key)
///     >
///         "+"
///     </SpeedDial>
/// }
/// ```
#[component]
pub fn SpeedDial(
    /// Actions to display.
    #[prop(into)]
    actions: Vec<SpeedDialAction>,
    /// Expand direction.
    #[prop(optional, into)]
    direction: Option<SpeedDialDirection>,
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
    /// Whether open by default.
    #[prop(optional)]
    default_open: bool,
    /// Controlled open state.
    #[prop(optional, into)]
    open: Option<RwSignal<bool>>,
    /// Open on hover vs click.
    #[prop(optional)]
    open_on_hover: bool,
    /// Close after action click.
    #[prop(optional)]
    #[prop(default = true)]
    close_on_action: bool,
    /// Main button icon.
    #[prop(optional)]
    icon: Option<Children>,
    /// Main button icon when open.
    #[prop(optional)]
    open_icon: Option<Children>,
    /// Accessible label.
    #[prop(into)]
    aria_label: String,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Action click handler.
    #[prop(optional, into)]
    on_action: Option<Callback<String>>,
    /// Open state change handler.
    #[prop(optional, into)]
    on_open_change: Option<Callback<bool>>,
) -> impl IntoView {
    // Get theme context for design system
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.design_system().as_str())
        .unwrap_or("ant");

    // Resolve defaults
    let direction = direction.unwrap_or_default();
    let placement = placement.unwrap_or_default();
    let offset = offset.unwrap_or((24, 24));

    // Open state - use provided signal or create internal one
    let is_open = open.unwrap_or_else(|| RwSignal::new(default_open));

    // Build CSS classes
    let speed_dial_prefix = format!("fx-speed-dial-{}", design_system);
    let btn_prefix = format!("fx-btn-{}", design_system);

    // Pre-clone for use after closures
    let trigger_class_prefix = format!("{}-trigger", speed_dial_prefix);
    let action_class = format!("{}-action", speed_dial_prefix);
    let actions_class = format!("{}-actions", speed_dial_prefix);
    let speed_dial_prefix_for_view = speed_dial_prefix.clone();

    let combined_class = {
        let prefix = speed_dial_prefix.clone();
        move || {
            let mut parts = vec![
                prefix.clone(),
                direction.class(&prefix),
                placement.class(&prefix),
            ];

            if is_open.get() {
                parts.push(format!("{}-open", prefix));
            }

            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }

            parts.join(" ")
        }
    };

    let main_btn_class = {
        let parts = vec![
            trigger_class_prefix,
            btn_prefix.clone(),
            ButtonVariant::Primary.class(&btn_prefix),
            ButtonShape::Circle.class(&btn_prefix),
        ];
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

    // Toggle handler
    let on_open_change_clone = on_open_change.clone();
    let handle_toggle = move |_: ev::MouseEvent| {
        let new_state = !is_open.get();
        is_open.set(new_state);
        if let Some(ref cb) = on_open_change_clone {
            cb.run(new_state);
        }
    };

    // Hover handlers (if open_on_hover is true)
    let on_open_change_hover = on_open_change.clone();
    let handle_mouse_enter = move |_: ev::MouseEvent| {
        if open_on_hover && !is_open.get() {
            is_open.set(true);
            if let Some(ref cb) = on_open_change_hover {
                cb.run(true);
            }
        }
    };

    let on_open_change_leave = on_open_change.clone();
    let handle_mouse_leave = move |_: ev::MouseEvent| {
        if open_on_hover && is_open.get() {
            is_open.set(false);
            if let Some(ref cb) = on_open_change_leave {
                cb.run(false);
            }
        }
    };

    // Action click handler
    let on_action_clone = on_action.clone();
    let on_open_change_action = on_open_change.clone();
    let handle_action_click = move |key: String, disabled: bool| {
        if disabled {
            return;
        }

        if let Some(ref cb) = on_action_clone {
            cb.run(key);
        }

        if close_on_action {
            is_open.set(false);
            if let Some(ref cb) = on_open_change_action {
                cb.run(false);
            }
        }
    };

    // Action button classes
    let action_btn_class = {
        let mut parts = vec![
            format!("{}-btn", action_class),
            btn_prefix.clone(),
            ButtonVariant::Secondary.class(&btn_prefix),
            ButtonShape::Circle.class(&btn_prefix),
        ];
        parts.join(" ")
    };

    view! {
        <div
            class=combined_class
            style=position_style
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            // Actions container
            <div class=actions_class role="menu">
                {actions.into_iter().enumerate().map(|(index, action)| {
                    let key = action.key.clone();
                    let disabled = action.disabled;
                    let handler = handle_action_click.clone();

                    let action_item_class = if disabled {
                        format!("{} {}-disabled", action_class.clone(), action_class.clone())
                    } else {
                        action_class.clone()
                    };

                    view! {
                        <div
                            class=action_item_class
                            style=format!("--action-index: {};", index)
                            role="menuitem"
                        >
                            {action.tooltip.clone().map(|tooltip| view! {
                                <span class=format!("{}-tooltip", action_class.clone())>{tooltip}</span>
                            })}
                            {action.label.clone().map(|label| view! {
                                <span class=format!("{}-label", action_class.clone())>{label}</span>
                            })}
                            <button
                                type="button"
                                class=action_btn_class.clone()
                                disabled=disabled
                                aria-label=action.tooltip.clone().or(action.label.clone()).unwrap_or(action.key.clone())
                                on:click=move |_| handler(key.clone(), disabled)
                            >
                                <span class=format!("{}-icon", action_class.clone())>
                                    {action.icon.clone()}
                                </span>
                            </button>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Main trigger button
            <button
                type="button"
                class=main_btn_class
                aria-label=aria_label.clone()
                aria-expanded=move || is_open.get().to_string()
                aria-haspopup="menu"
                on:click=handle_toggle
            >
                {
                let prefix = speed_dial_prefix_for_view.clone();
                move || {
                    if is_open.get() {
                        match &open_icon {
                            Some(_) => view! { <span class=format!("{}-icon-open", prefix)>"×"</span> }.into_any(),
                            None => view! { <span class=format!("{}-icon-open", prefix)>"×"</span> }.into_any(),
                        }
                    } else {
                        match &icon {
                            Some(_) => view! { <span class=format!("{}-icon", prefix)>"+"</span> }.into_any(),
                            None => view! { <span class=format!("{}-icon", prefix)>"+"</span> }.into_any(),
                        }
                    }
                }
            }
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_dial_direction_class() {
        let direction = SpeedDialDirection::Up;
        assert_eq!(direction.class("fx-speed-dial"), "fx-speed-dial-direction-up");
    }

    #[test]
    fn test_speed_dial_action_builder() {
        let action = SpeedDialAction::new("edit", "✏️")
            .with_tooltip("Edit")
            .with_label("Edit Item");

        assert_eq!(action.key, "edit");
        assert_eq!(action.icon, "✏️");
        assert_eq!(action.tooltip, Some("Edit".to_string()));
        assert_eq!(action.label, Some("Edit Item".to_string()));
    }
}
