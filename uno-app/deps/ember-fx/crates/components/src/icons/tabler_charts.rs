//! Tabler Charts icons page.

use leptos::prelude::*;
use ember_fx_icons::tabler::{TablerIconData, ChartsIcon};

/// Charts icons page.
#[component]
pub fn ChartsIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Charts Icons"</h1>
                <p class="page-subtitle">"Data visualization and chart icons from Tabler Icons"</p>
                <p class="page-count">{format!("{} icons", ChartsIcon::count())}</p>
            </header>

            <section class="demo-section">
                <h2>"All Charts Icons"</h2>
                <div class="icon-grid">
                    {ChartsIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            let name = icon.name();
                            let svg = icon.outline_svg();
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg fx-icon fx-icon-tabler fx-icon-lg" inner_html=svg></span>
                                    <span class="icon-grid-name">{name}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </section>

            <section class="demo-section">
                <h2>"Usage"</h2>
                <div class="code-block">
                    <pre><code>{r#"use ember_fx_icons::tabler::{TablerIcon, ChartsIcon};
use ember_fx_icons::types::IconSize;

view! {
    <TablerIcon icon=ChartsIcon::Example size=IconSize::Lg />
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
