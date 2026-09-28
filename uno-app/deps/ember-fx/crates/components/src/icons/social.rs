//! Social icons page.

use leptos::prelude::*;
use ember_fx_icons::{SocialIcon, get_social_svg};

/// Social icons page.
#[component]
pub fn SocialIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Social Icons"</h1>
                <p class="page-subtitle">"Communication, social actions, media, and profile icons"</p>
            </header>

            <section class="demo-section">
                <h2>"All Social Icons"</h2>
                <div class="icon-grid">
                    {SocialIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_social_svg(icon)></span>
                                    <span class="icon-grid-name">{icon.name()}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </section>

            <section class="demo-section">
                <h2>"Usage"</h2>
                <div class="code-block">
                    <pre><code>{r#"use ember_fx_icons::{Social, SocialIcon, get_social_svg};

// Using the component
view! {
    <Social icon=SocialIcon::Message class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_social_svg(SocialIcon::Like);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
