//! Navigation component

use leptos::prelude::*;
use crate::hooks::t;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="main-nav">
            <ul class="nav-links">
                <li><a href="/">{move || t("nav.home")}</a></li>
                <li><a href="/tasks">{move || t("nav.tasks")}</a></li>
                <li><a href="/guides">{move || t("nav.guides")}</a></li>
                <li><a href="/faq">{move || t("nav.faq")}</a></li>
                <li><a href="/referrals">{move || t("nav.referrals")}</a></li>
            </ul>
        </nav>
    }
}
