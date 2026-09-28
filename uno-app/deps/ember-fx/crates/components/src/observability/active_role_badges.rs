//! ActiveRoleBadges Leptos component.

use leptos::prelude::*;
use super::types::NodeRole;
use crate::try_use_theme;

/// ActiveRoleBadges component.
///
/// Displays a row of colored chips showing active node roles.
///
/// # Props
///
/// - `roles` - List of active roles
/// - `compact` - Use compact layout
/// - `show_inactive` - Show inactive roles (grayed out)
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{ActiveRoleBadges, NodeRole};
///
/// view! {
///     <ActiveRoleBadges
///         roles=Signal::derive(move || vec![NodeRole::Storage, NodeRole::Relay])
///     />
/// }
/// ```
#[component]
pub fn ActiveRoleBadges(
    /// Active node roles.
    #[prop(into)]
    roles: Signal<Vec<NodeRole>>,
    /// Compact mode for smaller displays.
    #[prop(optional)]
    compact: Option<bool>,
    /// Show inactive roles (grayed out).
    #[prop(optional)]
    show_inactive: Option<bool>,
    /// On role click handler.
    #[prop(optional, into)]
    on_role_click: Option<Callback<NodeRole>>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let compact = compact.unwrap_or(false);
    let show_inactive = show_inactive.unwrap_or(false);

    // Build CSS classes
    let prefix = format!("fx-role-badges-{}", design_system);
    let badge_prefix = format!("fx-role-badge-{}", design_system);

    // Pre-compute class names
    let base_class = prefix.clone();
    let compact_class = format!("{}-compact", prefix);
    let badge_class = badge_prefix.clone();
    let badge_icon_class = format!("{}-icon", badge_prefix);
    let badge_label_class = format!("{}-label", badge_prefix);
    let badge_active_class = format!("{}-active", badge_prefix);
    let badge_inactive_class = format!("{}-inactive", badge_prefix);

    let combined_class = {
        let base_class = base_class.clone();
        let compact_class = compact_class.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![base_class.clone()];
            if compact {
                parts.push(compact_class.clone());
            }
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Get all roles for showing inactive
    let all_roles = NodeRole::all();

    // Single role badge component
    let render_badge = move |role: NodeRole, is_active: bool| {
        let role_suffix = role.as_suffix();
        let badge_class = badge_class.clone();
        let badge_icon_class = badge_icon_class.clone();
        let badge_label_class = badge_label_class.clone();
        let badge_active_class = badge_active_class.clone();
        let badge_inactive_class = badge_inactive_class.clone();

        let badge_classes = {
            let mut classes = vec![
                badge_class.clone(),
                format!("{}-{}", badge_class, role_suffix),
            ];
            if is_active {
                classes.push(badge_active_class);
            } else {
                classes.push(badge_inactive_class);
            }
            classes.join(" ")
        };

        let style_str = if is_active {
            format!("--fx-role-color: {};", role.as_color())
        } else {
            String::new()
        };

        let on_click = on_role_click.clone();
        let click_handler = move |_| {
            if let Some(ref cb) = on_click {
                cb.run(role);
            }
        };

        view! {
            <span
                class=badge_classes
                style=style_str
                on:click=click_handler
            >
                <span class=badge_icon_class>{role.as_icon()}</span>
                <span class=badge_label_class>{role.as_label()}</span>
            </span>
        }
    };

    view! {
        <div class=combined_class>
            {move || {
                let active_roles = roles.get();

                if show_inactive {
                    // Show all roles, active ones highlighted
                    all_roles.iter().map(|role| {
                        let is_active = active_roles.contains(role);
                        render_badge(*role, is_active)
                    }).collect_view()
                } else {
                    // Show only active roles
                    active_roles.iter().map(|role| {
                        render_badge(*role, true)
                    }).collect_view()
                }
            }}
        </div>
    }
}
