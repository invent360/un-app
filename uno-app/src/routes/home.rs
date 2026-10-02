//! Landing page route - Djed Nodes style
//!
//! Supports CMS preview mode via ?preview_token query parameter.
//! Content is loaded from the CMS database.
//!
//! R5-13 Enhancements:
//! - Device suitability detection
//! - Task availability indicator
//! - Single "Check Eligibility" CTA
//! - Support identity display

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::common::{PreviewBanner, DevBadge};
use crate::components::suitability::use_suitability_state;
use crate::hooks::t;

#[cfg(feature = "hydrate")]
use crate::hooks::use_locale;
#[cfg(feature = "hydrate")]
use crate::api::{get_home_with_preview, HomeSection, HomeFaqItem};
#[cfg(feature = "hydrate")]
use crate::components::sections::CmsTestimonialsSection;

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

            // R5-13: Task availability banner
            <TaskAvailabilityBanner />

            // Static homepage content (no CMS dependency)
            <StaticHeroSection />
            <StaticHowItWorksSection />
            <StaticEarningsSection />

            // CMS-driven testimonials section (loads after hydration)
            <HomeContent preview_token=preview_token.clone() set_is_preview=set_is_preview />

            // R5-13: Support identity section
            <SupportIdentitySection />
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
/// NOTE: Only renders testimonials here since hero/how_it_works/earnings
/// are handled by static components with t() translations above.
#[cfg(feature = "hydrate")]
#[component]
fn CmsSections(sections: Vec<HomeSection>) -> impl IntoView {
    // Sort sections by display_order
    let mut sorted_sections = sections.clone();
    sorted_sections.sort_by_key(|s| s.display_order);

    view! {
        <>
            {sorted_sections.into_iter().filter(|s| {
                // Only render testimonials from CMS
                // Static sections handle hero, how_it_works, and earnings
                s.is_visible && s.section_type == "testimonials"
            }).map(|section| {
                view! { <CmsTestimonialsSection section=section /> }.into_any()
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

// ==============================================
// STATIC HOMEPAGE SECTIONS (No CMS dependency)
// ==============================================

/// Static hero section - always works without database
#[component]
fn StaticHeroSection() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="container hero-content">
                <div class="hero-text">
                    <h1 class="hero-title">
                        {move || t("home.hero.title")} " "
                        <span class="highlight">{move || t("home.hero.highlight")}</span>
                    </h1>
                    <p class="hero-subtitle">
                        {move || t("home.hero.subtitle")}
                    </p>
                    <div class="hero-cta">
                        <a href="/eligibility" class="btn btn-primary btn-large">
                            {move || t("home.hero.cta")}
                        </a>
                    </div>
                    <div class="hero-benefits">
                        <div class="benefit">
                            <span class="benefit-icon">"✓"</span>
                            <span>{move || t("home.hero.benefit_1")}</span>
                        </div>
                        <div class="benefit">
                            <span class="benefit-icon">"✓"</span>
                            <span>{move || t("home.hero.benefit_2")}</span>
                        </div>
                        <div class="benefit">
                            <span class="benefit-icon">"✓"</span>
                            <span>{move || t("home.hero.benefit_3")}</span>
                        </div>
                    </div>
                </div>
                <div class="hero-visual">
                    <div class="phone-mockup">
                        <div class="phone-screen">
                            <div class="carousel-placeholder">
                                <div class="app-preview-placeholder"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="hero-wave">
                <svg viewBox="0 0 1440 120" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M0 120L48 105C96 90 192 60 288 45C384 30 480 30 576 37.5C672 45 768 60 864 67.5C960 75 1056 75 1152 67.5C1248 60 1344 45 1392 37.5L1440 30V120H1392C1344 120 1248 120 1152 120C1056 120 960 120 864 120C768 120 672 120 576 120C480 120 384 120 288 120C192 120 96 120 48 120H0Z" fill="var(--color-bg-secondary)"/>
                </svg>
            </div>
        </section>
    }
}

/// Static how it works section
#[component]
fn StaticHowItWorksSection() -> impl IntoView {
    view! {
        <section id="how-it-works" class="section how-it-works">
            <div class="container">
                <h2 class="section-title">{move || t("home.how_it_works.title")}</h2>
                <p class="section-subtitle">{move || t("home.how_it_works.subtitle")}</p>

                <div class="steps">
                    <div class="step">
                        <div class="step-number">"1"</div>
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="5" y="2" width="14" height="20" rx="2" ry="2"></rect>
                                <line x1="12" y1="18" x2="12.01" y2="18"></line>
                            </svg>
                        </div>
                        <h3 class="step-title">{move || t("home.how_it_works.step1_title")}</h3>
                        <p class="step-description">{move || t("home.how_it_works.step1_desc")}</p>
                    </div>
                    <div class="step-arrow">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </div>
                    <div class="step">
                        <div class="step-number">"2"</div>
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
                            </svg>
                        </div>
                        <h3 class="step-title">{move || t("home.how_it_works.step2_title")}</h3>
                        <p class="step-description">{move || t("home.how_it_works.step2_desc")}</p>
                    </div>
                    <div class="step-arrow">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </div>
                    <div class="step">
                        <div class="step-number">"3"</div>
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect>
                                <line x1="1" y1="10" x2="23" y2="10"></line>
                            </svg>
                        </div>
                        <h3 class="step-title">{move || t("home.how_it_works.step3_title")}</h3>
                        <p class="step-description">{move || t("home.how_it_works.step3_desc")}</p>
                    </div>
                </div>

                <div class="app-buttons how-it-works-buttons">
                    <a href="https://apps.apple.com/gb/app/unity-network-app/id6755482738" target="_blank" rel="noopener noreferrer" class="app-download-btn">
                        <span class="app-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
                            </svg>
                        </span>
                        <span class="app-text">
                            <span class="app-sublabel">"Download on the"</span>
                            <span class="app-label">"App Store"</span>
                        </span>
                    </a>
                    <a href="https://play.google.com/store/apps/details?id=io.unetwork.app" target="_blank" rel="noopener noreferrer" class="app-download-btn">
                        <span class="app-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M3 20.5v-17c0-.59.34-1.11.84-1.35L13.69 12l-9.85 9.85c-.5-.25-.84-.76-.84-1.35zm13.81-5.38L6.05 21.34l8.49-8.49 2.27 2.27zm3.35-4.31c.34.27.59.69.59 1.19s-.22.9-.57 1.18l-2.29 1.32-2.5-2.5 2.5-2.5 2.27 1.31zM6.05 2.66l10.76 6.22-2.27 2.27L6.05 2.66z"/>
                            </svg>
                        </span>
                        <span class="app-text">
                            <span class="app-sublabel">"Get it on"</span>
                            <span class="app-label">"Google Play"</span>
                        </span>
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Static earnings potential section
#[component]
fn StaticEarningsSection() -> impl IntoView {
    view! {
        <section id="earnings" class="section earnings">
            <div class="container">
                <h2 class="section-title">{move || t("home.earnings.title")}</h2>
                <p class="section-subtitle">{move || t("home.earnings.subtitle")}</p>

                <div class="earnings-grid">
                    <div class="earnings-card">
                        <div class="earnings-card-header">
                            <span class="device-count">{move || t("home.earnings.tier_casual")}</span>
                        </div>
                        <div class="earnings-card-amount">"$5-15"</div>
                        <div class="earnings-card-period">{move || t("home.earnings.per_month")}</div>
                        <ul class="earnings-card-features">
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.devices_1")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.uptime_4h")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.connection_standard")}
                            </li>
                        </ul>
                    </div>
                    <div class="earnings-card featured">
                        <div class="earnings-card-badge">{move || t("home.earnings.most_popular")}</div>
                        <div class="earnings-card-header">
                            <span class="device-count">{move || t("home.earnings.tier_active")}</span>
                        </div>
                        <div class="earnings-card-amount">"$15-35"</div>
                        <div class="earnings-card-period">{move || t("home.earnings.per_month")}</div>
                        <ul class="earnings-card-features">
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.devices_2_3")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.uptime_8h")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.connection_good")}
                            </li>
                        </ul>
                    </div>
                    <div class="earnings-card">
                        <div class="earnings-card-header">
                            <span class="device-count">{move || t("home.earnings.tier_power")}</span>
                        </div>
                        <div class="earnings-card-amount">"$35-75"</div>
                        <div class="earnings-card-period">{move || t("home.earnings.per_month")}</div>
                        <ul class="earnings-card-features">
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.devices_4_plus")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.uptime_24_7")}
                            </li>
                            <li>
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
                                {move || t("home.earnings.connection_fast")}
                            </li>
                        </ul>
                    </div>
                </div>

                <p class="earnings-disclaimer">
                    {move || t("home.earnings.disclaimer")}
                </p>
            </div>
        </section>
    }
}

// ==============================================
// R5-13 LANDING PAGE ENHANCEMENTS
// ==============================================

/// Task availability banner showing current status
#[component]
fn TaskAvailabilityBanner() -> impl IntoView {
    // Mock task availability data - would come from API
    let tasks_available = true;
    let last_updated = "September 30, 2026";

    if !tasks_available {
        return view! {
            <div class="task-availability-banner unavailable">
                <div class="banner-content">
                    <span class="banner-icon">"⏳"</span>
                    <div class="banner-text">
                        <span class="banner-title">{move || t("home.tasks.unavailable_title")}</span>
                        <span class="banner-meta">{move || t("home.tasks.unavailable_subtitle")} " • " {move || t("home.tasks.last_checked")} " " {last_updated}</span>
                    </div>
                </div>
            </div>
        }.into_any();
    }

    view! {
        <div class="task-availability-banner available">
            <div class="banner-content">
                <span class="banner-icon">"✓"</span>
                <div class="banner-text">
                    <span class="banner-title">{move || t("home.tasks.available_title")} " " <DevBadge /></span>
                    <span class="banner-meta">{move || t("home.tasks.available_subtitle")} " • " {move || t("home.tasks.data_as_of")} " " {last_updated}</span>
                </div>
            </div>
        </div>
    }.into_any()
}

/// Check eligibility CTA section
#[component]
fn EligibilityCTASection() -> impl IntoView {
    // Get suitability state to trigger the modal
    let suitability_state = use_suitability_state();

    let open_eligibility_check = move |_| {
        if let Some(state) = suitability_state.as_ref() {
            state.open();
        }
    };

    view! {
        <section class="eligibility-cta-section">
            <div class="container">
                <div class="cta-card">
                    <div class="cta-content">
                        <h2>{move || t("home.eligibility.title")}</h2>
                        <p>{move || t("home.eligibility.subtitle")}</p>

                        <div class="device-compatibility">
                            <DeviceCompatibilityIndicator />
                        </div>

                        <button
                            class="btn-primary btn-lg cta-button"
                            on:click=open_eligibility_check
                        >
                            {move || t("home.eligibility.cta")}
                        </button>

                        <p class="cta-note">
                            {move || t("home.eligibility.no_payment")} " • " {move || t("home.eligibility.no_documents")}
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Device info for compatibility indicator
#[derive(Clone, Default)]
struct DeviceInfo {
    device: String,
    is_compatible: bool,
    message: String,
}

/// Device compatibility indicator
#[component]
fn DeviceCompatibilityIndicator() -> impl IntoView {
    // Use a signal so SSR and initial hydration both render the same "checking" state
    let device_info = RwSignal::new(None::<DeviceInfo>);

    // Detect device type only after hydration (client-side Effect)
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::prelude::Effect;
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(navigator)) = window.navigator().user_agent().map(Some) {
                    let ua = navigator.to_lowercase();
                    let info = if ua.contains("android") {
                        DeviceInfo {
                            device: "Android".to_string(),
                            is_compatible: true,
                            message: "Your device is supported".to_string(),
                        }
                    } else if ua.contains("iphone") || ua.contains("ipad") {
                        DeviceInfo {
                            device: "iOS".to_string(),
                            is_compatible: false,
                            message: "iOS support coming soon".to_string(),
                        }
                    } else {
                        DeviceInfo {
                            device: "Desktop/Other".to_string(),
                            is_compatible: false,
                            message: "Mobile app required".to_string(),
                        }
                    };
                    device_info.set(Some(info));
                }
            }
        });
    }

    view! {
        {move || {
            match device_info.get() {
                Some(info) => {
                    let class = if info.is_compatible {
                        "device-indicator compatible"
                    } else {
                        "device-indicator incompatible"
                    };
                    let icon = if info.is_compatible { "check" } else { "info" };

                    view! {
                        <div class=class>
                            <span class="device-icon">{icon}</span>
                            <span class="device-type">{info.device.clone()}</span>
                            <span class="device-status">{info.message.clone()}</span>
                        </div>
                    }.into_any()
                }
                None => {
                    view! {
                        <div class="device-indicator unknown">
                            <span class="device-icon">"..."</span>
                            <span class="device-status">"Checking device compatibility..."</span>
                        </div>
                    }.into_any()
                }
            }
        }}
    }
}

/// Support identity section with accountable contact info
#[component]
fn SupportIdentitySection() -> impl IntoView {
    view! {
        <section class="support-identity-section">
            <div class="container">
                <div class="support-card">
                    <h3>{move || t("home.support.title")}</h3>

                    <div class="support-contacts">
                        <div class="contact-item">
                            <span class="contact-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                    <polyline points="22,6 12,13 2,6"></polyline>
                                </svg>
                            </span>
                            <div class="contact-details">
                                <span class="contact-label">{move || t("home.support.email_label")}</span>
                                <a href="mailto:support@unetwork.io" class="contact-value">
                                    "support@unetwork.io"
                                </a>
                            </div>
                        </div>

                        <div class="contact-item">
                            <span class="contact-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                                    <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                                </svg>
                            </span>
                            <div class="contact-details">
                                <span class="contact-label">{move || t("home.support.help_center")}</span>
                                <a href="/guides" class="contact-value">
                                    {move || t("home.support.guides_link")}
                                </a>
                            </div>
                        </div>

                        <div class="contact-item">
                            <span class="contact-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
                                    <line x1="12" y1="17" x2="12.01" y2="17"></line>
                                </svg>
                            </span>
                            <div class="contact-details">
                                <span class="contact-label">{move || t("home.support.faq_label")}</span>
                                <a href="/faq" class="contact-value">
                                    {move || t("home.support.faq_link")}
                                </a>
                            </div>
                        </div>
                    </div>

                    <div class="support-identity">
                        <p class="identity-text">
                            {move || t("home.support.identity")} " • "
                            <a href="/terms">{move || t("home.support.terms")}</a>
                            " • "
                            <a href="/privacy">{move || t("home.support.privacy")}</a>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

