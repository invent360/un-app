//! Contact page route

use leptos::prelude::*;
use crate::hooks::t;

/// Contact page component
#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="page contact-page">
            <header class="page-header">
                <h1>{move || t("contact.title")}</h1>
                <p>{move || t("contact.subtitle")}</p>
            </header>

            <section class="contact-options">
                <div class="contact-card">
                    <h3>{move || t("contact.email_title")}</h3>
                    <p>{move || t("contact.email_desc")}</p>
                    <a href="mailto:support@unetwork.io" class="contact-link">
                        {move || t("contact.email_address")}
                    </a>
                </div>

                <div class="contact-card">
                    <h3>{move || t("contact.in_app_title")}</h3>
                    <p>{move || t("contact.in_app_desc")}</p>
                    <p class="contact-note">{move || t("contact.availability")}</p>
                </div>

                <div class="contact-card">
                    <h3>{move || t("contact.faq_title")}</h3>
                    <p>{move || t("contact.faq_desc")}</p>
                    <a href="/faq" class="contact-link">{move || t("contact.faq_link")}</a>
                </div>
            </section>

            <section class="response-time">
                <h2>{move || t("contact.response_times_title")}</h2>
                <p>{move || t("contact.response_times_desc")}</p>
            </section>
        </div>
    }
}
