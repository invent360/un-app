//! Animal icons page.

use leptos::prelude::*;
use ember_fx_icons::{AnimalIcon, get_animal_svg};

/// Animal icons page.
#[component]
pub fn AnimalIconsPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components/icons" class="back-link">"<- Back to Icons"</a>
                <h1 class="page-title">"Animal Icons"</h1>
                <p class="page-subtitle">"Pet, farm, wildlife, sea creatures, and insect icons"</p>
            </header>

            <section class="demo-section">
                <h2>"All Animal Icons"</h2>
                <div class="icon-grid">
                    {AnimalIcon::all()
                        .iter()
                        .map(|icon| {
                            let icon = *icon;
                            view! {
                                <div class="icon-grid-item">
                                    <span class="icon-grid-svg" inner_html=get_animal_svg(icon)></span>
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
                    <pre><code>{r#"use ember_fx_icons::{Animal, AnimalIcon, get_animal_svg};

// Using the component
view! {
    <Animal icon=AnimalIcon::Dog class="w-6 h-6" />
}

// Using the SVG directly
let svg = get_animal_svg(AnimalIcon::Cat);
view! {
    <span inner_html=svg></span>
}"#}</code></pre>
                </div>
            </section>
        </div>
    }
}
