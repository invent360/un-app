//! Nature icons page.

use leptos::prelude::*;
use ember_fx_icons::{NatureIcon, get_nature_svg};

/// Nature icons page.
#[component]
pub fn NatureIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Nature Icons"</h1>
                <p class="page-subtitle">"Plants, farming, weather, and landscape icons"</p>
            </header>

            <section class="demo-section">
                <h2>"All Nature Icons"</h2>
                <div class="icon-grid">
                    {NatureIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_nature_svg(icon)></span>
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
                    <pre><code>{r#"use ember_fx_icons::{Nature, NatureIcon, get_nature_svg};

// Using the component
view! {
    <Nature icon=NatureIcon::Tree class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_nature_svg(NatureIcon::Flower);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
