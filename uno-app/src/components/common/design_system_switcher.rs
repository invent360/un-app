//! Design System Switcher Component
//!
//! A dropdown/toggle component for switching between Ant and Material design systems.
//! Uses ember-fx's design system functionality.

use leptos::prelude::*;
use crate::hooks::{
    current_design_system,
    set_design_system,
    toggle_design_system,
    DesignSystem,
};

/// Design system switcher component
///
/// Displays the current design system with an option to switch.
/// Can be rendered as a simple toggle or a full dropdown.
///
/// # Example
///
/// ```ignore
/// view! { <DesignSystemSwitcher /> }
/// ```
#[component]
pub fn DesignSystemSwitcher(
    /// Whether to show as a compact toggle (default: false, shows dropdown)
    #[prop(optional, default = false)]
    compact: bool,
) -> impl IntoView {
    let current_system = move || current_design_system();

    let handle_toggle = move |_| {
        toggle_design_system();
    };

    let handle_select = move |system: DesignSystem| {
        set_design_system(system);
    };

    if compact {
        // Compact toggle button
        view! {
            <button
                class="design-system-toggle"
                on:click=handle_toggle
                title=move || format!("Current: {} - Click to switch", system_name(current_system()))
            >
                <span class="design-system-icon">
                    {move || system_icon(current_system())}
                </span>
            </button>
        }.into_any()
    } else {
        // Full dropdown selector
        view! {
            <div class="design-system-switcher">
                <label class="switcher-label">"Design System"</label>
                <div class="switcher-options">
                    <button
                        class="switcher-option"
                        class:active=move || current_system() == DesignSystem::Ant
                        on:click=move |_| handle_select(DesignSystem::Ant)
                    >
                        <span class="option-icon">"A"</span>
                        <span class="option-name">"Ant Design"</span>
                    </button>
                    <button
                        class="switcher-option"
                        class:active=move || current_system() == DesignSystem::Material
                        on:click=move |_| handle_select(DesignSystem::Material)
                    >
                        <span class="option-icon">"M"</span>
                        <span class="option-name">"Material"</span>
                    </button>
                </div>
            </div>
        }.into_any()
    }
}

/// Get the display name for a design system
fn system_name(system: DesignSystem) -> &'static str {
    match system {
        DesignSystem::Ant => "Ant Design",
        DesignSystem::Material => "Material Design",
        _ => "Other",
    }
}

/// Get an icon/emoji for a design system
fn system_icon(system: DesignSystem) -> &'static str {
    match system {
        DesignSystem::Ant => "A",
        DesignSystem::Material => "M",
        _ => "?",
    }
}
