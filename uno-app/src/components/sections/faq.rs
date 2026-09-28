//! CMS-driven FAQ section component for the home page

use leptos::prelude::*;
use crate::api::HomeSection;
use crate::api::faq::get_faqs;

/// Get color for FAQ category
fn get_category_color(category: &str) -> &'static str {
    match category {
        "general" => "#3b82f6",
        "earnings" => "#22c55e",
        "setup" => "#8b5cf6",
        "security" => "#ef4444",
        _ => "#64748b",
    }
}

/// CMS-driven FAQ section for the home page
/// Displays FAQ items in an accordion style
#[component]
pub fn CmsFaqSection(
    /// Section data from CMS
    section: HomeSection,
) -> impl IntoView {
    let title = section.title.clone();
    let subtitle = section.description.clone();

    // Fetch FAQ data from API (all FAQs, not just featured)
    let faq_resource = Resource::new(
        || (),
        |_| async move {
            leptos::logging::log!("Fetching FAQs for home page...");
            match get_faqs(None, None, None, None).await {
                Ok(response) => {
                    leptos::logging::log!("FAQ API success: {} items", response.items.len());
                    Ok(response)
                }
                Err(e) => {
                    leptos::logging::error!("FAQ API error: {:?}", e);
                    Err(e)
                }
            }
        }
    );

    view! {
        <section class="section faq-section">
            <div class="container">
                <div class="faq-section-wrapper">
                    <h2 class="section-title">{title}</h2>
                    <p class="section-subtitle">{subtitle}</p>

                    <Suspense fallback=move || view! {
                        <div class="loading-state" style="text-align: center; padding: 2rem;">
                            <div class="spinner" style="width: 32px; height: 32px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                        </div>
                        <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                    }>
                        {move || {
                            match faq_resource.get() {
                                Some(Ok(response)) if !response.items.is_empty() => {
                                    view! {
                                        <div class="faq-items home-faq-items">
                                            {response.items.into_iter().map(|item| {
                                                let category_color = get_category_color(&item.category);
                                                let border_style = format!("border-left-color: {}", category_color);
                                                view! {
                                                    <details class="faq-item">
                                                        <summary class="faq-summary" style=border_style>
                                                            <span style="flex: 1; font-weight: 500;">{item.question.clone()}</span>
                                                        </summary>
                                                        <div class="faq-answer">
                                                            {item.answer.clone()}
                                                        </div>
                                                    </details>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                                Some(Ok(_)) => {
                                    // No FAQs - show message
                                    view! {
                                        <p style="text-align: center; color: #6b7280; padding: 1rem;">"No FAQs available yet."</p>
                                    }.into_any()
                                }
                                Some(Err(_)) => {
                                    // Error - show nothing (fail silently)
                                    view! { <div></div> }.into_any()
                                }
                                None => {
                                    // Still loading
                                    view! { <div></div> }.into_any()
                                }
                            }
                        }}
                    </Suspense>
                </div>
            </div>
        </section>
    }.into_any()
}
