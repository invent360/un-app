//! Header component with responsive hamburger menu

use leptos::prelude::*;
use leptos::task::spawn_local;
use super::Nav;
use crate::components::common::{LanguageSelector, ThemeSwitcher};
use crate::components::wizard::use_wizard_state;
use crate::hooks::t;
use crate::api::check_license_availability;

#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };

    let close_menu = move |_| {
        set_menu_open.set(false);
    };

    // Get wizard state from app-level context
    let wizard_state = use_wizard_state();

    // Handler to open wizard and check availability
    let open_wizard = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        if let Some(state) = wizard_state.clone() {
            state.open();
            // Check availability asynchronously
            let s = state.clone();
            spawn_local(async move {
                match check_license_availability().await {
                    Ok(available) => s.set_availability(available),
                    Err(_) => s.set_availability(false),
                }
            });
        }
    };

    view! {
        <header class="site-header">
            <div class="header-content">
                // Logo - always visible, left side
                <a href="/" class="logo">
                    <img src="/assets/djed.png" alt="DJED Nodes" class="logo-img" />
                </a>

                // Start Earning CTA button - opens wizard directly
                <a href="#" class="header-cta-btn" on:click=open_wizard>
                    {move || t("nav.start_earning")}
                </a>

                // Desktop navigation and actions
                <div class="header-desktop">
                    <Nav />
                    <div class="header-actions">
                        <ThemeSwitcher />
                        <LanguageSelector />
                    </div>
                </div>

                // Hamburger button - mobile only
                <button
                    class="hamburger-btn"
                    class:open=move || menu_open.get()
                    on:click=toggle_menu
                    aria-label="Toggle menu"
                >
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                    <span class="hamburger-line"></span>
                </button>
            </div>

            // Mobile menu overlay
            <div class="mobile-menu" class:open=move || menu_open.get()>
                <ul class="mobile-nav-links">
                    <li><a href="/" on:click=close_menu>{move || t("nav.home")}</a></li>
                    <li><a href="/tasks" on:click=close_menu>{move || t("nav.tasks")}</a></li>
                    <li><a href="/faq" on:click=close_menu>{move || t("nav.faq")}</a></li>
                </ul>
                <div class="mobile-actions">
                    <ThemeSwitcher />
                    <LanguageSelector />
                </div>
            </div>
        </header>
    }
}
