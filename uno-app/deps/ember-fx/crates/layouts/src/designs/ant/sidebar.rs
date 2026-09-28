//! Ant Design sidebar component.

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::types::{NavItem, LogoConfig};
use crate::components::{Logo, NavItemComponent, CollapseToggle};
use crate::context::try_use_layout;

/// Ant Design sidebar component.
///
/// A fixed sidebar navigation panel with logo, navigation items, and optional
/// footer content. Supports collapsing on desktop and slide-in on mobile.
///
/// The nav_items prop accepts a signal, allowing the sidebar to reactively
/// update when navigation items change without re-rendering the entire layout.
///
/// # Example
///
/// ```ignore
/// <AntSidebar
///     nav_items=Signal::derive(move || nav_items_signal.get())
///     logo=LogoConfig::text("S", "Stax Board")
///     collapsed=collapsed_signal
/// />
/// ```
#[component]
pub fn AntSidebar(
    /// Navigation items (reactive signal for dynamic updates).
    #[prop(into)]
    nav_items: Signal<Vec<NavItem>>,
    /// Logo configuration.
    #[prop(into)]
    logo: LogoConfig,
    /// Whether sidebar is collapsed.
    #[prop(into)]
    collapsed: RwSignal<bool>,
    /// Custom footer content.
    #[prop(optional)]
    footer: Option<Children>,
    /// Selection callback (wrapped for internal use).
    #[prop(optional, into)]
    on_select: StoredValue<Option<Callback<String>>>,
) -> impl IntoView {
    let location = use_location();

    // Create signals for passing to child components
    // Use Memo to avoid creating derived signals that evaluate outside reactive context
    let current_path = Memo::new(move |_| location.pathname.get());
    let collapsed_signal = Memo::new(move |_| collapsed.get());

    view! {
        <aside class="fx-sidebar-ant">
            // Header (logo)
            <div class="fx-sidebar-header">
                <Logo
                    config=logo
                    collapsed=collapsed_signal
                />
            </div>

            // Navigation - reactive closure re-renders only this part when nav_items changes
            <nav class="fx-sidebar-nav">
                <ul class="fx-nav-menu">
                    {move || {
                        nav_items.get().into_iter().map(|item| {
                            view! {
                                <NavItemComponent
                                    item=item
                                    collapsed=collapsed_signal
                                    current_path=current_path
                                    on_select=on_select
                                />
                            }
                        }).collect_view()
                    }}
                </ul>
            </nav>

            // Footer slot
            {footer.map(|children| {
                view! {
                    <div class="fx-sidebar-footer">
                        {children()}
                    </div>
                }
            })}

            // Icon box toggle
            <IconBoxToggle collapsed=collapsed_signal />

            // Collapse toggle (desktop only)
            <CollapseToggle collapsed=collapsed />
        </aside>
    }
}

/// Icon box toggle component.
#[component]
fn IconBoxToggle(
    #[prop(into)]
    collapsed: Signal<bool>,
) -> impl IntoView {
    let ctx = try_use_layout();

    view! {
        <div class="fx-icon-box-toggle">
            <button
                class=move || {
                    let enabled = ctx.map(|c| c.icon_box_enabled.get()).unwrap_or(true);
                    if enabled {
                        "fx-toggle-switch fx-toggle-on"
                    } else {
                        "fx-toggle-switch"
                    }
                }
                on:click=move |_| {
                    if let Some(ctx) = ctx {
                        ctx.toggle_icon_box();
                    }
                }
            >
                <span class="fx-toggle-switch-handle"></span>
            </button>
            <span class="fx-nav-item-label">
                {move || if collapsed.get() { "" } else { "Icon boxes" }}
            </span>
        </div>
    }
}
