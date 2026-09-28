//! Tabler System icons page.

use leptos::prelude::*;
use ember_fx_icons::tabler::{TablerIconData, SystemIcon};

/// System icons page.
#[component]
pub fn SystemIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"System Icons"</h1>
                <p class="page-subtitle">"System and UI action icons from Tabler Icons"</p>
                <p class="page-count">{format!("{} icons", SystemIcon::count())}</p>
            </header>

            <section class="demo-section">
                <h2>"All System Icons"</h2>
                <div class="icon-grid">
                    {SystemIcon::all()
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
                    <pre><code>{r#"use ember_fx_icons::tabler::{TablerIcon, SystemIcon};
use ember_fx_icons::types::IconSize;

view! {
    <TablerIcon icon=SystemIcon::Example size=IconSize::Lg />
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
