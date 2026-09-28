//! Tabler Devices icons page.

use leptos::prelude::*;
use ember_fx_icons::tabler::{TablerIconData, DevicesIcon};

/// Devices icons page.
#[component]
pub fn DevicesIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Devices Icons"</h1>
                <p class="page-subtitle">"Device and hardware icons from Tabler Icons"</p>
                <p class="page-count">{format!("{} icons", DevicesIcon::count())}</p>
            </header>

            <section class="demo-section">
                <h2>"All Devices Icons"</h2>
                <div class="icon-grid">
                    {DevicesIcon::all()
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
                    <pre><code>{r#"use ember_fx_icons::tabler::{TablerIcon, DevicesIcon};
use ember_fx_icons::types::IconSize;

view! {
    <TablerIcon icon=DevicesIcon::Example size=IconSize::Lg />
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
