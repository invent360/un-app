//! Landing page route - Djed Nodes style
//!
//! Supports CMS preview mode via ?preview_token query parameter.
//! Content is loaded from the CMS database.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::common::PreviewBanner;

#[cfg(feature = "hydrate")]
use crate::hooks::{t, use_locale};
#[cfg(feature = "hydrate")]
use crate::api::{get_home_with_preview, HomeSection, HomeFaqItem};
#[cfg(feature = "hydrate")]
use crate::components::sections::{CmsHeroSection, CmsHowItWorksSection, CmsEarningsSection, CmsTestimonialsSection};

/// Landing page component
#[component]
pub fn HomePage() -> impl IntoView {
    let query = use_query_map();

    // Extract preview_token from query string - use get_untracked for initial value
    let preview_token = query.get_untracked().get("preview_token").map(|s| s.to_string());

    // Track if we're in preview mode
    let (is_preview, set_is_preview) = signal(preview_token.is_some());

    view! {
        <div class="landing-page">
            // Show preview banner if in preview mode
            {move || is_preview.get().then(|| view! { <PreviewBanner /> })}

            // Dynamic CMS content (client-side only)
            <HomeContent preview_token=preview_token.clone() set_is_preview=set_is_preview />
        </div>
    }
}

/// Dynamic home content that fetches from CMS
#[component]
fn HomeContent(
    preview_token: Option<String>,
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
    // Signal to track if we're mounted on client
    let is_mounted = RwSignal::new(false);

    // Effect runs only on client after hydration
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move || {
            is_mounted.set(true);
        });
    }

    view! {
        <div class="home-dynamic-container">
            {move || {
                if is_mounted.get() {
                    // Client-only: render the dynamic CMS content
                    #[cfg(feature = "hydrate")]
                    {
                        view! { <CmsHomeContent preview_token=preview_token.clone() set_is_preview=set_is_preview /> }.into_any()
                    }
                    #[cfg(not(feature = "hydrate"))]
                    {
                        view! {}.into_any()
                    }
                } else {
                    // SSR and initial client render: show loading state with spinner
                    view! {
                        <div class="loading-state" style="text-align: center; padding: 3rem;">
                            <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                        </div>
                        <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// CMS-driven home content (client-only)
#[cfg(feature = "hydrate")]
#[component]
fn CmsHomeContent(
    preview_token: Option<String>,
    set_is_preview: WriteSignal<bool>,
) -> impl IntoView {
    let ctx = use_locale();
    let token = preview_token.clone();

    let home_resource = Resource::new(
        move || (ctx.locale.get().code().to_string(), token.clone()),
        |(locale, token)| async move {
            match get_home_with_preview(locale, token).await {
                Ok(response) => {
                    leptos::logging::log!("Home API success: sections={}", response.sections.len());
                    Some(response)
                }
                Err(e) => {
                    leptos::logging::error!("Home API error: {:?}", e);
                    None
                }
            }
        }
    );

    // Update preview state when resource loads
    Effect::new(move || {
        if let Some(Some(response)) = home_resource.get() {
            set_is_preview.set(response.is_preview);
        }
    });

    view! {
        <Suspense fallback=move || view! {
            // Loading state with spinner
            <div class="loading-state" style="text-align: center; padding: 3rem;">
                <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
            </div>
            <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
        }>
            {move || {
                match home_resource.get() {
                    Some(Some(response)) if !response.sections.is_empty() => {
                        let faqs = response.faqs.clone();
                        // Render CMS sections + FAQ section
                        view! {
                            <CmsSections sections=response.sections />
                            <HomeFaqSection faqs=faqs />
                        }.into_any()
                    }
                    Some(_) => {
                        // API returned but no content - show "coming soon" message only
                        view! {
                            <div class="empty-state" style="text-align: center; padding: 3rem;">
                                <p class="placeholder-text">{move || t("home.coming_soon")}</p>
                            </div>
                        }.into_any()
                    }
                    None => {
                        // Still loading - show spinner (Suspense should handle this, but just in case)
                        view! {
                            <div class="loading-state" style="text-align: center; padding: 3rem;">
                                <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                            </div>
                            <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                        }.into_any()
                    }
                }
            }}
        </Suspense>
    }
}

/// Render CMS sections dynamically
#[cfg(feature = "hydrate")]
#[component]
fn CmsSections(sections: Vec<HomeSection>) -> impl IntoView {
    // Sort sections by display_order
    let mut sorted_sections = sections.clone();
    sorted_sections.sort_by_key(|s| s.display_order);

    view! {
        <>
            {sorted_sections.into_iter().filter(|s| s.is_visible && s.section_type != "faq").map(|section| {
                match section.section_type.as_str() {
                    "hero" => view! { <CmsHeroSection section=section /> }.into_any(),
                    "how_it_works" => view! { <CmsHowItWorksSection section=section /> }.into_any(),
                    "earnings" => view! { <CmsEarningsSection section=section /> }.into_any(),
                    "testimonials" => view! { <CmsTestimonialsSection section=section /> }.into_any(),
                    _ => view! { <div></div> }.into_any(),
                }
            }).collect_view()}
        </>
    }
}

/// FAQ section for home page - receives FAQs from home API response
#[cfg(feature = "hydrate")]
#[component]
fn HomeFaqSection(faqs: Vec<HomeFaqItem>) -> impl IntoView {
    let title = t("faq.title");
    let subtitle = t("faq.subtitle");
    let search_placeholder = t("faq.search_placeholder");

    const ITEMS_PER_PAGE: usize = 5;

    // Debug: log how many FAQs we received
    leptos::logging::log!("HomeFaqSection: received {} FAQs", faqs.len());

    // Don't render if no FAQs
    if faqs.is_empty() {
        return view! {
            <section class="section faq-section">
                <div class="container">
                    <p style="color: #888;">"Debug: No FAQs received from API"</p>
                </div>
            </section>
        }.into_any();
    }

    // Search, filter, and pagination state
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal("all".to_string());
    let (current_page, set_current_page) = signal(1usize);

    // Categories for filter buttons
    let categories = vec![
        ("all", "All"),
        ("general", "General"),
        ("earnings", "Earnings"),
        ("setup", "Setup"),
        ("security", "Security"),
    ];

    // Store FAQs for reactive access
    let faqs_store = StoredValue::new(faqs);

    view! {
        <section class="section faq-section">
            <div class="container">
                <div class="faq-section-wrapper">
                    <h2 class="section-title">{title}</h2>
                    <p class="section-subtitle">{subtitle}</p>

                    // Search and filters
                    <div class="faq-filters">
                        <div class="search-bar">
                            <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="11" cy="11" r="8"></circle>
                                <path d="m21 21-4.3-4.3"></path>
                            </svg>
                            <input
                                type="text"
                                class="search-input"
                                placeholder=search_placeholder
                                on:input=move |ev| {
                                    set_search_query.set(event_target_value(&ev));
                                    set_current_page.set(1);
                                }
                            />
                        </div>

                        <div class="faq-categories">
                            {categories.into_iter().map(|(value, label)| {
                                let value_clone = value.to_string();
                                let is_active = move || selected_category.get() == value;
                                view! {
                                    <button
                                        class="category-btn"
                                        class:active=is_active
                                        on:click=move |_| {
                                            set_selected_category.set(value_clone.clone());
                                            set_current_page.set(1);
                                        }
                                    >
                                        {label}
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // FAQ items and pagination - computed together to avoid multiple reactive passes
                    {move || {
                        let query = search_query.get().to_lowercase();
                        let category = selected_category.get();
                        let page = current_page.get();

                        // Filter FAQs
                        let filtered: Vec<HomeFaqItem> = faqs_store.with_value(|faqs| {
                            faqs.iter()
                                .filter(|item| {
                                    let category_match = category == "all" || item.category == category;
                                    let search_match = query.is_empty()
                                        || item.question.to_lowercase().contains(&query)
                                        || item.answer.to_lowercase().contains(&query);
                                    category_match && search_match
                                })
                                .cloned()
                                .collect()
                        });

                        let total = filtered.len();
                        let total_pages = if total == 0 { 1 } else { (total + ITEMS_PER_PAGE - 1) / ITEMS_PER_PAGE };

                        // Paginate
                        let start = (page - 1) * ITEMS_PER_PAGE;
                        let end = (start + ITEMS_PER_PAGE).min(total);
                        let paginated: Vec<HomeFaqItem> = filtered.into_iter().skip(start).take(end - start).collect();

                        view! {
                            <div class="faq-items home-faq-items">
                                {if paginated.is_empty() {
                                    view! {
                                        <p class="no-results">"No questions found matching your search."</p>
                                    }.into_any()
                                } else {
                                    paginated.into_iter().map(|item| {
                                        view! {
                                            <details class="faq-item">
                                                <summary>{item.question.clone()}</summary>
                                                <p>{item.answer.clone()}</p>
                                            </details>
                                        }
                                    }).collect_view().into_any()
                                }}
                            </div>

                            // Pagination controls
                            {if total_pages > 1 {
                                let prev_disabled = page <= 1;
                                let next_disabled = page >= total_pages;

                                view! {
                                    <div class="faq-pagination">
                                        <button
                                            class="pagination-btn"
                                            disabled=prev_disabled
                                            on:click=move |_| {
                                                let p = current_page.get();
                                                if p > 1 { set_current_page.set(p - 1); }
                                            }
                                        >
                                            "Previous"
                                        </button>

                                        <div class="pagination-pages">
                                            {(1..=total_pages).map(|p| {
                                                let is_current = page == p;
                                                view! {
                                                    <button
                                                        class="pagination-page"
                                                        class:active=is_current
                                                        on:click=move |_| set_current_page.set(p)
                                                    >
                                                        {p}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>

                                        <button
                                            class="pagination-btn"
                                            disabled=next_disabled
                                            on:click=move |_| {
                                                let p = current_page.get();
                                                set_current_page.set(p + 1);
                                            }
                                        >
                                            "Next"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        }
                    }}
                </div>
            </div>
        </section>
    }.into_any()
}

