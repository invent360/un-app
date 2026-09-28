//! Menu Leptos component.

use leptos::prelude::*;
use super::types::{MenuMode, MenuTheme, MenuItem};
use crate::try_use_theme;

/// Menu component.
///
/// Navigation menu with support for submenus.
///
/// # Props
///
/// - `items` - Menu items configuration
/// - `mode` - Menu mode (Vertical, Horizontal, Inline)
/// - `theme` - Menu theme (Light, Dark)
/// - `selected_keys` - Currently selected item keys
/// - `open_keys` - Currently open submenu keys
/// - `collapsed` - Whether the menu is collapsed (for inline mode)
/// - `on_select` - Selection callback
///
/// # Example
///
/// ```ignore
/// use ember_fx::components::navigation::{Menu, MenuItem};
///
/// let items = vec![
///     MenuItem::new("home", "Home").icon("🏠"),
///     MenuItem::new("settings", "Settings").icon("⚙️"),
/// ];
///
/// view! {
///     <Menu items=items />
/// }
/// ```
#[component]
pub fn Menu(
    /// Menu items.
    #[prop(into)]
    items: Vec<MenuItem>,
    /// Menu mode.
    #[prop(optional, into)]
    mode: Option<MenuMode>,
    /// Menu theme.
    #[prop(optional, into)]
    theme: Option<MenuTheme>,
    /// Selected item keys.
    #[prop(optional, into)]
    selected_keys: Option<RwSignal<Vec<String>>>,
    /// Open submenu keys.
    #[prop(optional, into)]
    open_keys: Option<RwSignal<Vec<String>>>,
    /// Whether menu is collapsed.
    #[prop(optional)]
    collapsed: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
    /// Selection callback.
    #[prop(optional, into)]
    on_select: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    // Resolve defaults
    let mode = mode.unwrap_or_default();
    let theme = theme.unwrap_or_default();
    let selected_keys = selected_keys.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let open_keys = open_keys.unwrap_or_else(|| RwSignal::new(Vec::new()));

    // Build CSS classes
    let menu_prefix = format!("fx-menu-{}", design_system);
    let mode_class = mode.class(&menu_prefix);
    let theme_class = theme.class(&menu_prefix);

    let combined_class = {
        let mut parts = vec![menu_prefix.clone(), mode_class, theme_class];
        if collapsed {
            parts.push(format!("{}-collapsed", menu_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // Class names for menu items
    let item_class = format!("{}-item", menu_prefix);
    let _item_group_class = format!("{}-item-group", menu_prefix);
    let submenu_class = format!("{}-submenu", menu_prefix);
    let submenu_title_class = format!("{}-submenu-title", menu_prefix);
    let submenu_list_class = format!("{}-submenu-list", menu_prefix);
    let icon_class = format!("{}-item-icon", menu_prefix);
    let title_class = format!("{}-title-content", menu_prefix);

    view! {
        <ul class=combined_class role="menu">
            {items.into_iter().map(|item| {
                let key = item.key.clone();
                let key_for_select = key.clone();
                let key_for_active = key.clone();
                let key_for_submenu = key.clone();
                let has_children = !item.children.is_empty();
                let on_select = on_select.clone();
                let item_class = item_class.clone();
                let submenu_class = submenu_class.clone();
                let submenu_title_class = submenu_title_class.clone();
                let submenu_list_class = submenu_list_class.clone();
                let icon_class = icon_class.clone();
                let title_class = title_class.clone();
                let menu_prefix = menu_prefix.clone();

                if has_children {
                    // Submenu item
                    let children = item.children.clone();
                    let key_for_class = key_for_submenu.clone();
                    let key_for_style = key_for_submenu.clone();

                    view! {
                        <li
                            class=move || {
                                let mut cls = vec![submenu_class.clone()];
                                if open_keys.get().contains(&key_for_class) {
                                    cls.push(format!("{}-submenu-open", menu_prefix));
                                }
                                cls.join(" ")
                            }
                        >
                            <div
                                class=submenu_title_class.clone()
                                on:click=move |_| {
                                    let mut keys = open_keys.get();
                                    if keys.contains(&key) {
                                        keys.retain(|k| k != &key);
                                    } else {
                                        keys.push(key.clone());
                                    }
                                    open_keys.set(keys);
                                }
                            >
                                {item.icon.clone().map(|i| view! {
                                    <span
                                        class=icon_class.clone()
                                        style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #2a2a2a; border: 1px solid #3a3a3a; border-radius: 8px; flex-shrink: 0;"
                                    >{i}</span>
                                })}
                                <span class=title_class.clone()>{item.label.clone()}</span>
                                <span class=format!("{}-submenu-arrow", menu_prefix)>"▼"</span>
                            </div>
                            <ul
                                class=submenu_list_class.clone()
                                style=move || {
                                    if open_keys.get().contains(&key_for_style) {
                                        None
                                    } else {
                                        Some("display: none;")
                                    }
                                }
                            >
                                {children.into_iter().map(|child| {
                                    let child_key = child.key.clone();
                                    let child_key_for_click = child_key.clone();
                                    let child_key_for_active = child_key.clone();
                                    let on_select = on_select.clone();
                                    let item_class = item_class.clone();
                                    let icon_class = icon_class.clone();
                                    let title_class = title_class.clone();
                                    let menu_prefix = menu_prefix.clone();

                                    view! {
                                        <li
                                            class=move || {
                                                let mut cls = vec![item_class.clone()];
                                                if selected_keys.get().contains(&child_key_for_active) {
                                                    cls.push(format!("{}-item-selected", menu_prefix));
                                                }
                                                if child.disabled {
                                                    cls.push(format!("{}-item-disabled", menu_prefix));
                                                }
                                                if child.danger {
                                                    cls.push(format!("{}-item-danger", menu_prefix));
                                                }
                                                cls.join(" ")
                                            }
                                            on:click=move |_| {
                                                if !child.disabled {
                                                    selected_keys.set(vec![child_key_for_click.clone()]);
                                                    if let Some(ref cb) = on_select {
                                                        cb.run(child_key_for_click.clone());
                                                    }
                                                }
                                            }
                                            role="menuitem"
                                        >
                                            {child.icon.clone().map(|i| view! {
                                                <span
                                                    class=icon_class.clone()
                                                    style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #2a2a2a; border: 1px solid #3a3a3a; border-radius: 8px; flex-shrink: 0;"
                                                >{i}</span>
                                            })}
                                            <span class=title_class.clone()>{child.label.clone()}</span>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        </li>
                    }.into_any()
                } else {
                    // Regular item
                    view! {
                        <li
                            class=move || {
                                let mut cls = vec![item_class.clone()];
                                if selected_keys.get().contains(&key_for_active) {
                                    cls.push(format!("{}-item-selected", menu_prefix));
                                }
                                if item.disabled {
                                    cls.push(format!("{}-item-disabled", menu_prefix));
                                }
                                if item.danger {
                                    cls.push(format!("{}-item-danger", menu_prefix));
                                }
                                cls.join(" ")
                            }
                            on:click=move |_| {
                                if !item.disabled {
                                    selected_keys.set(vec![key_for_select.clone()]);
                                    if let Some(ref cb) = on_select {
                                        cb.run(key_for_select.clone());
                                    }
                                }
                            }
                            role="menuitem"
                        >
                            {item.icon.clone().map(|i| view! {
                                <span
                                    class=icon_class.clone()
                                    style="display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; background: #2a2a2a; border: 1px solid #3a3a3a; border-radius: 8px; flex-shrink: 0;"
                                >{i}</span>
                            })}
                            <span class=title_class.clone()>{item.label.clone()}</span>
                        </li>
                    }.into_any()
                }
            }).collect_view()}
        </ul>
    }
}
