//! Finance icons page.

use leptos::prelude::*;
use ember_fx_icons::{FinanceIcon, get_finance_svg};

/// Finance icons page.
#[component]
pub fn FinanceIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Finance Icons"</h1>
                <p class="page-subtitle">"Money, banking, trading, and business icons"</p>
            </header>

            <section class="demo-section">
                <h2>"All Finance Icons"</h2>
                <div class="icon-grid">
                    {FinanceIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_finance_svg(icon)></span>
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
                    <pre><code>{r#"use ember_fx_icons::{Finance, FinanceIcon, get_finance_svg};

// Using the component
view! {
    <Finance icon=FinanceIcon::Wallet class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_finance_svg(FinanceIcon::Bank);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
