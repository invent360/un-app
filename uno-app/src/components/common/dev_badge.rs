//! Development mode badge component
//!
//! Shows a visible badge when displaying mock/demo data to prevent user confusion.

use leptos::prelude::*;

/// Development mode badge - shows when mock data is being displayed
///
/// This badge should be shown near any data that is hardcoded or mocked,
/// to clearly indicate to users that the values shown are not real.
#[component]
pub fn DevBadge(
    /// Optional custom text (defaults to "Demo Data")
    #[prop(optional)]
    text: Option<&'static str>,
) -> impl IntoView {
    let label = text.unwrap_or("Demo Data");

    view! {
        <span class="dev-badge" title="This section displays demonstration data, not live values">
            {label}
        </span>
    }
}

/// Development mode banner - larger, more prominent notice
#[component]
pub fn DevBanner(
    /// Title of the banner
    #[prop(optional)]
    title: Option<&'static str>,
    /// Description text
    #[prop(optional)]
    description: Option<&'static str>,
) -> impl IntoView {
    let title_text = title.unwrap_or("Development Mode");
    let desc_text = description.unwrap_or("Data shown below is for demonstration purposes only.");

    view! {
        <div class="dev-banner">
            <span class="dev-banner-icon">"⚠️"</span>
            <div class="dev-banner-content">
                <span class="dev-banner-title">{title_text}</span>
                <span class="dev-banner-desc">{desc_text}</span>
            </div>
        </div>
    }
}
