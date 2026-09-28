//! Common Errors page - displays error documentation from CMS

use leptos::prelude::*;
use crate::hooks::t;

/// Common Errors page component
#[component]
pub fn ErrorsPage() -> impl IntoView {
    view! {
        <div class="page errors-page">
            <header class="page-header">
                <h1>{move || t("errors.title")}</h1>
                <p>{move || t("errors.subtitle")}</p>
            </header>

            <section class="errors-content">
                <p class="placeholder-text">
                    {move || t("errors.coming_soon")}
                </p>
            </section>
        </div>
    }
}
