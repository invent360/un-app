//! Ant Design settings drawer component.

use leptos::prelude::*;
use crate::context::use_layout;
use crate::types::NavPosition;

/// Settings drawer component for layout configuration.
///
/// Slides in from the left side and allows users to configure:
/// - Mobile navigation position (top/bottom)
/// - Other layout options
///
/// # Example
///
/// ```ignore
/// // The drawer is controlled via LayoutContext
/// let ctx = use_layout();
/// ctx.open_settings(); // Opens the drawer
/// ```
#[component]
pub fn AntSettingsDrawer() -> impl IntoView {
    let ctx = use_layout();

    // Get current nav position
    let nav_position = move || ctx.nav_position.get();
    let is_left = move || nav_position() == NavPosition::Left;
    let is_right = move || nav_position() == NavPosition::Right;
    let is_top = move || nav_position() == NavPosition::Top;
    let is_bottom = move || nav_position() == NavPosition::Bottom;

    // Drawer open state
    let is_open = move || ctx.settings_open.get();

    // Close drawer handler
    let close = move |_| ctx.close_settings();

    // Set nav position handlers
    let set_left = move |_| ctx.set_nav_position(NavPosition::Left);
    let set_right = move |_| ctx.set_nav_position(NavPosition::Right);
    let set_top = move |_| ctx.set_nav_position(NavPosition::Top);
    let set_bottom = move |_| ctx.set_nav_position(NavPosition::Bottom);

    // Reset to default handler
    let reset_to_default = move |_| ctx.reset_nav_position();
    let is_overridden = move || ctx.nav_position_override.get();
    let is_mobile = move || ctx.is_mobile.get();

    view! {
        <div class=move || format!(
            "fx-settings-drawer{}",
            if is_open() { " fx-settings-drawer-open" } else { "" }
        )>
            // Backdrop
            <div
                class="fx-settings-backdrop"
                on:click=close
            />

            // Drawer panel
            <div class="fx-settings-panel">
                // Header
                <div class="fx-settings-header">
                    <h2 class="fx-settings-title">"Settings"</h2>
                    <button
                        class="fx-settings-close"
                        on:click=close
                    >
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                        </svg>
                    </button>
                </div>

                // Content
                <div class="fx-settings-body">
                    // Navigation Position Section
                    <section class="fx-settings-section">
                        <h3 class="fx-settings-section-title">"Navigation Position"</h3>
                        <p class="fx-settings-section-desc">
                            {move || if is_mobile() {
                                "Default: Bottom (mobile)"
                            } else {
                                "Default: Left (desktop)"
                            }}
                        </p>

                        <div class="fx-settings-nav-options fx-settings-nav-grid">
                            // Left option (default)
                            <button
                                class=move || format!(
                                    "fx-settings-nav-option{}",
                                    if is_left() { " fx-settings-nav-option-active" } else { "" }
                                )
                                on:click=set_left
                            >
                                <div class="fx-settings-nav-preview fx-settings-nav-preview-left">
                                    <div class="fx-settings-nav-preview-bar fx-settings-nav-preview-bar-left"/>
                                    <div class="fx-settings-nav-preview-content"/>
                                </div>
                                <span class="fx-settings-nav-label">"Left"</span>
                            </button>

                            // Right option
                            <button
                                class=move || format!(
                                    "fx-settings-nav-option{}",
                                    if is_right() { " fx-settings-nav-option-active" } else { "" }
                                )
                                on:click=set_right
                            >
                                <div class="fx-settings-nav-preview fx-settings-nav-preview-right">
                                    <div class="fx-settings-nav-preview-content"/>
                                    <div class="fx-settings-nav-preview-bar fx-settings-nav-preview-bar-right"/>
                                </div>
                                <span class="fx-settings-nav-label">"Right"</span>
                            </button>

                            // Top option
                            <button
                                class=move || format!(
                                    "fx-settings-nav-option{}",
                                    if is_top() { " fx-settings-nav-option-active" } else { "" }
                                )
                                on:click=set_top
                            >
                                <div class="fx-settings-nav-preview fx-settings-nav-preview-top">
                                    <div class="fx-settings-nav-preview-bar fx-settings-nav-preview-bar-top"/>
                                    <div class="fx-settings-nav-preview-content"/>
                                </div>
                                <span class="fx-settings-nav-label">"Top"</span>
                            </button>

                            // Bottom option
                            <button
                                class=move || format!(
                                    "fx-settings-nav-option{}",
                                    if is_bottom() { " fx-settings-nav-option-active" } else { "" }
                                )
                                on:click=set_bottom
                            >
                                <div class="fx-settings-nav-preview fx-settings-nav-preview-bottom">
                                    <div class="fx-settings-nav-preview-content"/>
                                    <div class="fx-settings-nav-preview-bar fx-settings-nav-preview-bar-bottom"/>
                                </div>
                                <span class="fx-settings-nav-label">"Bottom"</span>
                            </button>
                        </div>

                        // Reset to default button (only shown when user has overridden)
                        <Show when=move || is_overridden()>
                            <button
                                class="fx-settings-reset-btn"
                                on:click=reset_to_default
                            >
                                "Reset to Default"
                            </button>
                        </Show>
                    </section>

                    // Layout Section
                    <section class="fx-settings-section">
                        <h3 class="fx-settings-section-title">"Layout"</h3>
                        <SettingToggle
                            label="Fixed Header"
                            checked=true
                            disabled=true
                        />
                        <SettingToggle
                            label="Fixed Sidebar"
                            checked=true
                            disabled=true
                        />
                        <p class="fx-settings-muted">"More layout options coming soon"</p>
                    </section>

                    // About Section
                    <section class="fx-settings-section fx-settings-section-border">
                        <h3 class="fx-settings-section-title">"About"</h3>
                        <div class="fx-settings-muted">
                            <p>"ember-fx-layouts"</p>
                            <p>"Ant Design Layout System"</p>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}

/// Toggle setting component.
#[component]
fn SettingToggle(
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> impl IntoView {
    let is_on = RwSignal::new(checked);

    view! {
        <div class="fx-settings-row">
            <span class=if disabled { "fx-settings-label fx-settings-label-disabled" } else { "fx-settings-label" }>
                {label}
            </span>
            <button
                class=move || format!(
                    "fx-settings-toggle{}{}",
                    if is_on.get() { " fx-settings-toggle-on" } else { "" },
                    if disabled { " fx-settings-toggle-disabled" } else { "" }
                )
                disabled=disabled
                on:click=move |_| if !disabled { is_on.update(|v| *v = !*v) }
            >
                <span class="fx-settings-toggle-handle" />
            </button>
        </div>
    }
}

/// Settings trigger button component.
///
/// A floating button that opens the settings drawer when clicked.
/// Typically placed in the bottom-right corner.
#[component]
pub fn AntSettingsTrigger() -> impl IntoView {
    let ctx = use_layout();

    view! {
        <button
            class="fx-settings-trigger"
            on:click=move |_| ctx.open_settings()
            title="Settings"
        >
            <svg class="fx-settings-trigger-icon" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
            </svg>
        </button>
    }
}
