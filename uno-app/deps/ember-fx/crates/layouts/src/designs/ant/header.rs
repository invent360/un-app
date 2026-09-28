//! Ant Design header component.

use leptos::prelude::*;
use crate::types::{LogoConfig, UserConfig};
use crate::components::{Logo, ThemeToggle, UserMenu};

const HAMBURGER_ICON: &str = r#"<path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>"#;

/// Ant Design header component.
///
/// A fixed header bar with hamburger menu (mobile), logo (mobile),
/// theme toggle, and user menu.
///
/// # Example
///
/// ```ignore
/// <AntHeader
///     logo=LogoConfig::text("S", "Stax Board")
///     user=UserConfig::new("John Doe")
///     collapsed=collapsed_signal
///     on_menu_click=move || ctx.toggle_mobile_menu()
/// />
/// ```
#[component]
pub fn AntHeader<F>(
    /// Logo configuration.
    #[prop(into)]
    logo: LogoConfig,
    /// User configuration.
    #[prop(into)]
    user: UserConfig,
    /// Whether sidebar is collapsed.
    #[prop(into)]
    collapsed: Signal<bool>,
    /// Whether to show theme toggle.
    #[prop(optional)]
    show_theme_toggle: bool,
    /// Hamburger menu click handler.
    on_menu_click: F,
) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let _ = collapsed; // Used via parent CSS class

    view! {
        <header class="fx-header-ant">
            // Left section
            <div class="fx-header-left">
                // Hamburger (mobile only)
                <button
                    class="fx-header-hamburger"
                    on:click={
                        let on_menu_click = on_menu_click.clone();
                        move |_| on_menu_click()
                    }
                    aria-label="Toggle menu"
                >
                    <svg
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        inner_html=HAMBURGER_ICON
                    />
                </button>

                // Logo (mobile only - desktop shows in sidebar)
                <Logo
                    config=logo
                    collapsed=Signal::derive(|| false)
                    mobile_only=true
                />
            </div>

            // Center section
            <div class="fx-header-center">
            </div>

            // Right section
            <div class="fx-header-right">
                // Theme toggle
                {show_theme_toggle.then(|| view! { <ThemeToggle /> })}

                // User menu
                <UserMenu config=user />
            </div>
        </header>
    }
}
