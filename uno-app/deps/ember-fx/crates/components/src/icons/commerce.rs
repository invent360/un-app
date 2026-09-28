//! Commerce icons page.

use leptos::prelude::*;
use ember_fx_icons::{CommerceIcon, get_commerce_svg};

/// Commerce icons page.
#[component]
pub fn CommerceIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Commerce Icons"</h1>
                <p class="page-subtitle">"Shopping, products, shipping, and e-commerce icons"</p>
            </header>

            <section class="demo-section">
                <h2>"All Commerce Icons"</h2>
                <div class="icon-grid">
                    {CommerceIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_commerce_svg(icon)></span>
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
                    <pre><code>{r#"use ember_fx_icons::{Commerce, CommerceIcon, get_commerce_svg};

// Using the component
view! {
    <Commerce icon=CommerceIcon::Cart class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_commerce_svg(CommerceIcon::Package);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
