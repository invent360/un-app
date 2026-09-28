//! Ant Design main layout component.

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::context::{LayoutContext, provide_layout};
use crate::types::{NavItem, LogoConfig, UserConfig, MobileNavItem, NavPosition, nav_to_mobile};
use super::{AntHeader, AntSidebar, AntMobileNav, AntTopMobileNav};

/// Mobile breakpoint in pixels.
#[cfg(target_arch = "wasm32")]
const MOBILE_BREAKPOINT: i32 = 768;

/// Main layout component (Ant Design).
///
/// Provides a complete app shell with sidebar, header, and mobile navigation.
/// Automatically handles responsive behavior and layout state.
///
/// # Props
///
/// - `nav_items` - Sidebar navigation items
/// - `mobile_nav_items` - Mobile bottom nav items (optional, defaults to first 5 nav_items)
/// - `logo` - Logo configuration
/// - `user` - User menu configuration
/// - `auth_paths` - Paths that should use auth layout (no sidebar/header)
/// - `show_theme_toggle` - Whether to show theme toggle in header
///
/// # Example
///
/// ```ignore
/// use ember_fx_layouts::{MainLayout, NavItem, LogoConfig, UserConfig};
///
/// <MainLayout
///     nav_items=vec![
///         NavItem::new("dashboard", "Dashboard", "/dashboard")
///             .icon(DASHBOARD_ICON),
///     ]
///     logo=LogoConfig::text("S", "Stax Board").href("/")
///     user=UserConfig::new("John Doe").avatar("/avatar.jpg")
///     auth_paths=vec!["/login".to_string()]
/// >
///     <Router>
///         <Routes />
///     </Router>
/// </MainLayout>
/// ```
#[component]
pub fn AntMainLayout(
    /// Navigation items for sidebar (accepts Signal for reactive updates).
    #[prop(into)]
    nav_items: Signal<Vec<NavItem>>,
    /// Mobile navigation items (optional).
    #[prop(optional, into)]
    mobile_nav_items: Option<Vec<MobileNavItem>>,
    /// Logo configuration.
    #[prop(optional, into)]
    logo: Option<LogoConfig>,
    /// User configuration.
    #[prop(optional, into)]
    user: Option<UserConfig>,
    /// Paths that should use auth layout (no sidebar/header).
    #[prop(optional, into)]
    auth_paths: Option<Vec<String>>,
    /// Whether to show theme toggle.
    #[prop(optional)]
    show_theme_toggle: Option<bool>,
    /// Callback when nav item is selected.
    #[prop(optional, into)]
    on_nav_select: Option<Callback<String>>,
    /// Child content.
    children: Children,
) -> impl IntoView {
    // Create and provide layout context
    let ctx = LayoutContext::new();
    provide_layout(ctx);

    // Initialize nav position based on screen size (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        // Initial detection
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                let width = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0) as i32;
                let is_mobile = width < MOBILE_BREAKPOINT;
                ctx.init_nav_position(is_mobile);
            }
        });

        // Listen for resize events to update mobile state
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                let closure = Closure::<dyn Fn()>::new(move || {
                    if let Some(window) = web_sys::window() {
                        let width = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0) as i32;
                        let is_mobile = width < MOBILE_BREAKPOINT;
                        ctx.update_mobile_state(is_mobile);
                    }
                });

                let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                closure.forget(); // Keep the closure alive
            }
        });
    }

    // Defaults
    let logo = logo.unwrap_or_else(|| LogoConfig::text("A", "App"));
    let user = user.unwrap_or_default();
    let show_theme_toggle = show_theme_toggle.unwrap_or(true);
    let auth_paths = auth_paths.unwrap_or_else(|| vec!["/login".to_string(), "/register".to_string()]);

    // Derive mobile nav items from nav_items if not provided (use initial value)
    let mobile_nav_items = mobile_nav_items.unwrap_or_else(|| nav_to_mobile(&nav_items.get_untracked()));

    // Clone logo and user for use in multiple closures
    let logo_for_sidebar = logo.clone();
    let logo_for_header = logo.clone();
    let user_for_header = user.clone();

    // Store the callback so we can pass it down
    let on_select_stored = StoredValue::new(on_nav_select);

    // Get location for path checks
    let location = use_location();

    // Clone auth_paths for closures
    let auth_paths_for_check = auth_paths.clone();
    let auth_paths_for_overlay = auth_paths.clone();
    let auth_paths_for_sidebar = auth_paths.clone();
    let auth_paths_for_header = auth_paths.clone();
    let auth_paths_for_main = auth_paths.clone();
    let auth_paths_for_mobile = auth_paths.clone();

    // Create Memos for derived values to consolidate reactive access
    let current_path = Memo::new(move |_| location.pathname.get());
    let is_collapsed = Memo::new(move |_| ctx.collapsed.get());
    let is_mobile_open = Memo::new(move |_| ctx.mobile_menu_open.get());

    // Check if current path is an auth path
    let is_auth_path = Memo::new(move |_| {
        let path = current_path.get();
        auth_paths_for_check.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)))
    });

    // Check if using horizontal nav (top or bottom) - hides sidebar on desktop
    let is_horizontal_nav = Memo::new(move |_| {
        ctx.nav_position.get().is_horizontal()
    });

    // Build class based on layout state using Memos
    let layout_class = Memo::new(move |_| {
        if is_auth_path.get() {
            "fx-layout-ant fx-layout-auth".to_string()
        } else {
            let mut class = "fx-layout-ant fx-layout-app".to_string();
            if is_collapsed.get() {
                class.push_str(" fx-layout-sidebar-collapsed");
            }
            if is_mobile_open.get() {
                class.push_str(" fx-layout-mobile-menu-open");
            }
            // Add nav position classes
            match ctx.nav_position.get() {
                NavPosition::Left => {
                    class.push_str(" fx-layout-nav-left");
                }
                NavPosition::Right => {
                    class.push_str(" fx-layout-nav-right");
                }
                NavPosition::Top => {
                    class.push_str(" fx-layout-nav-horizontal fx-layout-nav-top");
                }
                NavPosition::Bottom => {
                    class.push_str(" fx-layout-nav-horizontal fx-layout-nav-bottom");
                }
            }
            class
        }
    });

    // Create Memo for overlay visibility
    let show_overlay = Memo::new(move |_| {
        let path = current_path.get();
        let is_auth = auth_paths_for_overlay.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)));
        !is_auth && is_mobile_open.get()
    });

    // Create Memo for sidebar visibility (hidden when using horizontal nav)
    let show_sidebar = Memo::new(move |_| {
        let path = current_path.get();
        let is_auth = auth_paths_for_sidebar.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)));
        // Hide sidebar on auth pages and when horizontal nav is active
        !is_auth && !is_horizontal_nav.get()
    });

    // Create Memo for header visibility
    let show_header = Memo::new(move |_| {
        let path = current_path.get();
        !auth_paths_for_header.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)))
    });

    // Create Memo for main content class
    let main_class = Memo::new(move |_| {
        let path = current_path.get();
        let is_auth = auth_paths_for_main.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)));
        if is_auth {
            "fx-layout-main fx-layout-main-auth"
        } else {
            "fx-layout-main fx-layout-main-app"
        }
    });

    // Create Memo for mobile nav visibility
    let show_mobile_nav = Memo::new(move |_| {
        let path = current_path.get();
        !auth_paths_for_mobile.iter().any(|p| path == *p || path.starts_with(&format!("{}/", p)))
    });

    // Create Memo for nav position
    let is_nav_top = Memo::new(move |_| ctx.nav_position.get() == NavPosition::Top);
    let is_nav_bottom = Memo::new(move |_| ctx.nav_position.get() == NavPosition::Bottom);

    // Clone mobile nav items for both top and bottom nav
    let mobile_nav_items_for_top = mobile_nav_items.clone();
    let mobile_nav_items_for_bottom = mobile_nav_items.clone();

    view! {
        <div class=layout_class>
            // Mobile overlay backdrop
            <Show when=move || show_overlay.get()>
                <div
                    class="fx-layout-overlay"
                    on:click=move |_| ctx.close_mobile_menu()
                />
            </Show>

            // Sidebar (app mode only) - receives signal for reactive nav updates
            <Show when=move || show_sidebar.get()>
                <AntSidebar
                    nav_items=nav_items
                    logo=logo_for_sidebar.clone()
                    collapsed=ctx.collapsed
                    on_select=on_select_stored
                />
            </Show>

            // Header (app mode only)
            <Show when=move || show_header.get()>
                <AntHeader
                    logo=logo_for_header.clone()
                    user=user_for_header.clone()
                    collapsed=Memo::new(move |_| ctx.collapsed.get())
                    show_theme_toggle=show_theme_toggle
                    on_menu_click=move || ctx.toggle_mobile_menu()
                />
            </Show>

            // Top mobile navigation (when nav position is top)
            <Show when=move || show_mobile_nav.get() && is_nav_top.get()>
                <AntTopMobileNav
                    items=mobile_nav_items_for_top.clone()
                    on_select=on_select_stored
                />
            </Show>

            // Main content area
            <main class=main_class>
                <div class="fx-layout-content">
                    {children()}
                </div>
            </main>

            // Bottom mobile navigation (when nav position is bottom)
            <Show when=move || show_mobile_nav.get() && is_nav_bottom.get()>
                <AntMobileNav
                    items=mobile_nav_items_for_bottom.clone()
                    on_select=on_select_stored
                />
            </Show>

        </div>
    }
}
