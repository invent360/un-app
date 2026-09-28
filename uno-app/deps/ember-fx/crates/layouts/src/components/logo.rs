//! Logo component for layout headers.

use leptos::prelude::*;
use crate::types::LogoConfig;

/// Logo component that displays in the sidebar header.
///
/// Supports both text-based logos (with icon + full text) and image logos.
/// The full text is hidden when the sidebar is collapsed.
#[component]
pub fn Logo(
    /// Logo configuration.
    #[prop(into)]
    config: LogoConfig,
    /// Whether the sidebar is collapsed (hides full text).
    #[prop(into)]
    collapsed: Signal<bool>,
    /// Only show on mobile (hidden on desktop).
    #[prop(optional)]
    mobile_only: bool,
) -> impl IntoView {
    let class = move || {
        let mut class = "fx-logo".to_string();
        if collapsed.get() {
            class.push_str(" fx-logo-collapsed");
        }
        if mobile_only {
            class.push_str(" fx-logo-mobile-only");
        }
        class
    };

    view! {
        <a href=config.href.clone() class=class>
            {if let Some(ref url) = config.image_url {
                // Image logo
                view! {
                    <img
                        src=url.clone()
                        alt="Logo"
                        class="fx-logo-image"
                    />
                }.into_any()
            } else {
                // Text logo
                view! {
                    <div class="fx-logo-icon">
                        {config.icon_text.clone().unwrap_or_default()}
                    </div>
                    <span class="fx-logo-text">
                        {config.full_text.clone().unwrap_or_default()}
                    </span>
                }.into_any()
            }}
        </a>
    }
}
