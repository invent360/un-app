//! Navigation item component for sidebar.

use leptos::prelude::*;
use crate::context::try_use_layout;
use crate::types::NavItem;

const CHEVRON_RIGHT: &str = r#"<path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>"#;

/// Navigation item component.
///
/// Renders a single navigation item, which can be either a link or a
/// collapsible group with children.
#[component]
pub fn NavItemComponent(
    /// The navigation item to render.
    #[prop(into)]
    item: NavItem,
    /// Whether the sidebar is collapsed.
    #[prop(into)]
    collapsed: Signal<bool>,
    /// Current path for active state detection.
    #[prop(into)]
    current_path: Signal<String>,
    /// Callback when item is selected (wrapped in StoredValue for cloning).
    #[prop(optional, into)]
    on_select: StoredValue<Option<Callback<String>>>,
) -> impl IntoView {
    let ctx = try_use_layout();
    let key = item.key.clone();
    let key_for_toggle = key.clone();
    let key_for_open = key.clone();
    let has_children = item.is_group();

    // Clone item for use in closures
    let item_for_active = item.clone();

    if has_children {
        // Render as collapsible group
        let children = item.children.clone();
        let item_for_class = item_for_active.clone();
        let key_for_class = key_for_open.clone();

        view! {
            <li class="fx-nav-group">
                <button
                    class=move || {
                        let mut class = "fx-nav-group-header".to_string();
                        if item_for_class.matches_path(&current_path.get()) {
                            class.push_str(" fx-nav-group-active");
                        }
                        if ctx.map(|c| c.is_submenu_open(&key_for_class)).unwrap_or(false) {
                            class.push_str(" fx-nav-group-open");
                        }
                        class
                    }
                    on:click=move |_| {
                        if let Some(ctx) = ctx {
                            ctx.toggle_submenu(&key_for_toggle);
                        }
                    }
                    disabled=item.disabled
                >
                    {item.icon.clone().map(|icon| {
                        let icon_class = move || {
                            if ctx.map(|c| c.icon_box_enabled.get()).unwrap_or(false) {
                                "fx-nav-item-icon fx-icon-boxed"
                            } else {
                                "fx-nav-item-icon"
                            }
                        };
                        view! {
                            <span class=icon_class>
                                <svg
                                    width="18"
                                    height="18"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    inner_html=icon
                                />
                            </span>
                        }
                    })}
                    <span class="fx-nav-item-label">{item.label.clone()}</span>
                    <span class="fx-nav-group-arrow">
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            inner_html=CHEVRON_RIGHT
                        />
                    </span>
                </button>
                <ul class=move || {
                    let is_open = ctx.map(|c| c.is_submenu_open(&key_for_open)).unwrap_or(false);
                    if is_open && !collapsed.get() {
                        "fx-nav-submenu"
                    } else {
                        "fx-nav-submenu fx-nav-submenu-hidden"
                    }
                }>
                    {children.into_iter().map(|child| {
                        view! {
                            <NavItemComponent
                                item=child
                                collapsed=collapsed
                                current_path=current_path
                                on_select=on_select
                            />
                        }
                    }).collect_view()}
                </ul>
            </li>
        }.into_any()
    } else {
        // Render as link
        let path = item.path.clone().unwrap_or_default();
        let key_for_click = key.clone();
        let icon = item.icon.clone();
        let active_icon = item.active_icon.clone();
        let item_for_class = item_for_active.clone();
        let item_for_icon = item_for_active.clone();
        let is_disabled = item.disabled;
        let is_danger = item.danger;

        view! {
            <li>
                <a
                    href=path
                    class=move || {
                        let mut class = "fx-nav-item".to_string();
                        if item_for_class.matches_path(&current_path.get()) {
                            class.push_str(" fx-nav-item-active");
                        }
                        if is_disabled {
                            class.push_str(" fx-nav-item-disabled");
                        }
                        if is_danger {
                            class.push_str(" fx-nav-item-danger");
                        }
                        class
                    }
                    on:click=move |_| {
                        if let Some(ref cb) = on_select.get_value() {
                            cb.run(key_for_click.clone());
                        }
                        // Close mobile menu on navigation
                        if let Some(ctx) = ctx {
                            ctx.close_mobile_menu();
                        }
                    }
                >
                    {icon.map(|default_icon| {
                        let active_icon = active_icon.clone();
                        let default_icon_clone = default_icon.clone();
                        let icon_class = move || {
                            if ctx.map(|c| c.icon_box_enabled.get()).unwrap_or(false) {
                                "fx-nav-item-icon fx-icon-boxed"
                            } else {
                                "fx-nav-item-icon"
                            }
                        };
                        view! {
                            <span class=icon_class>
                                <svg
                                    width="18"
                                    height="18"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    inner_html=move || {
                                        if item_for_icon.matches_path(&current_path.get()) {
                                            active_icon.clone().unwrap_or_else(|| default_icon_clone.clone())
                                        } else {
                                            default_icon.clone()
                                        }
                                    }
                                />
                            </span>
                        }
                    })}
                    <span class="fx-nav-item-label">{item.label.clone()}</span>
                    {item.badge.map(|count| {
                        if count > 0 {
                            view! {
                                <span class="fx-nav-item-badge">
                                    {if count > 99 { "99+".to_string() } else { count.to_string() }}
                                </span>
                            }.into_any()
                        } else {
                            view! { <></> }.into_any()
                        }
                    })}
                </a>
            </li>
        }.into_any()
    }
}
