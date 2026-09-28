//! Ant Design mobile bottom navigation.

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::types::MobileNavItem;

/// Helper to check if a path is active.
fn is_path_active(current: &str, check_path: &str) -> bool {
    if check_path == "/" {
        current == "/"
    } else {
        current.starts_with(check_path)
    }
}

/// Single mobile nav item component.
#[component]
fn MobileNavItemView(
    item: MobileNavItem,
    on_select: StoredValue<Option<Callback<String>>>,
) -> impl IntoView {
    let location = use_location();
    let path = item.path.clone();
    let path_for_class = path.clone();
    let _path_for_icon = path.clone();
    let key_for_click = item.key.clone();
    let icon = item.icon.clone();
    let icon_for_active = icon.clone();
    let active_icon = item.active_icon.clone();
    let badge = item.badge;

    // Compute active state as a Memo derived from location
    let is_active = Memo::new(move |_| {
        let current = location.pathname.get();
        is_path_active(&current, &path_for_class)
    });

    // Derive class string from active state
    let class_string = Memo::new(move |_| {
        let mut class = "fx-mobile-nav-item".to_string();
        if is_active.get() {
            class.push_str(" fx-mobile-nav-item-active");
        }
        class
    });

    // Derive icon from active state
    let active_icon_clone = active_icon.clone();
    let icon_for_active_clone = icon_for_active.clone();
    let icon_clone = icon.clone();
    let icon_html = Memo::new(move |_| {
        if is_active.get() {
            active_icon_clone.clone().unwrap_or_else(|| icon_for_active_clone.clone())
        } else {
            icon_clone.clone()
        }
    });

    view! {
        <a
            href=path
            class=class_string
            on:click=move |_| {
                if let Some(ref cb) = on_select.get_value() {
                    cb.run(key_for_click.clone());
                }
            }
        >
            <div class="fx-mobile-nav-icon">
                <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    inner_html=icon_html
                />
                {badge.map(|count| {
                    if count > 0 {
                        view! {
                            <span class="fx-mobile-nav-badge">
                                {if count > 99 { "99+".to_string() } else { count.to_string() }}
                            </span>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                })}
            </div>
            <span class="fx-mobile-nav-label">{item.label}</span>
        </a>
    }
}

/// Ant Design mobile bottom navigation.
///
/// A fixed bottom navigation bar for mobile devices. Shows up to 5 navigation
/// items with icons and labels. Hidden on desktop (768px+).
///
/// # Example
///
/// ```ignore
/// <AntMobileNav
///     items=vec![
///         MobileNavItem::new("home", "Home", "/", HOME_ICON),
///         MobileNavItem::new("wallet", "Wallet", "/wallet", WALLET_ICON),
///     ]
/// />
/// ```
#[component]
pub fn AntMobileNav(
    /// Navigation items.
    #[prop(into)]
    items: Vec<MobileNavItem>,
    /// Selection callback (wrapped for internal use).
    #[prop(optional, into)]
    on_select: StoredValue<Option<Callback<String>>>,
) -> impl IntoView {
    view! {
        <nav class="fx-mobile-nav-ant">
            {items.into_iter().map(|item| {
                view! {
                    <MobileNavItemView item=item on_select=on_select />
                }
            }).collect_view()}
        </nav>
    }
}
