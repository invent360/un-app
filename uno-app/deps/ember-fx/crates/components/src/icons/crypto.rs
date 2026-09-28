//! Crypto token icons page.

use leptos::prelude::*;
use ember_fx_icons::{TokenIcon, get_token_svg};

/// Crypto token icons page.
#[component]
pub fn CryptoIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Crypto Token Icons"</h1>
                <p class="page-subtitle">"Cryptocurrency and token icons for blockchain applications"</p>
            </header>

            <section class="demo-section">
                <h2>"All Crypto Icons"</h2>
                <div class="icon-grid">
                    {TokenIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_token_svg(icon)></span>
                                    <span class="icon-grid-symbol">{icon.symbol()}</span>
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
                    <pre><code>{r#"use ember_fx_icons::{Token, TokenIcon, get_token_svg};

// Using the component
view! {
    <Token icon=TokenIcon::BTC class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_token_svg(TokenIcon::ETH);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
