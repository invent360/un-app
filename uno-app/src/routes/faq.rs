//! FAQ page route
//!
//! Displays FAQ items fetched from the database/CMS.
//! Supports preview mode via preview_token query parameter.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::components::faq::{SearchBar, CategoryTabs, get_default_categories};
use crate::api::faq::get_faqs;
use crate::hooks::t;
use crate::types::FaqItemResponse;

const ITEMS_PER_PAGE: usize = 10;

/// FAQ page component
#[component]
pub fn FaqPage() -> impl IntoView {
    // Get query params for preview mode
    let query_params = use_query_map();
    let preview_token = move || -> Option<String> {
        query_params.get().get("preview_token").map(|s| s.to_string())
    };
    let is_preview = move || preview_token().is_some();

    // State for search, category filter, and pagination
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal::<Option<String>>(None);
    let (current_page, set_current_page) = signal(1usize);

    // Fetch all FAQ data from API
    let faq_resource = Resource::new(
        || (),
        |_| async move {
            get_faqs(None, None, None, None).await
        }
    );

    view! {
        <div class="page faq-page">
            // Preview banner
            {move || if is_preview() {
                view! {
                    <div class="preview-banner" style="background: linear-gradient(90deg, #f59e0b, #d97706); color: white; padding: 12px 20px; text-align: center; font-weight: 600;">
                        <span style="margin-right: 8px;">"Preview Mode"</span>
                        <span style="opacity: 0.9; font-weight: 400;">" - This content is not yet published"</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            <Suspense fallback=move || view! {
                <div class="loading-state" style="text-align: center; padding: 3rem;">
                    <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                </div>
                <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
            }>
                {move || {
                    match faq_resource.get() {
                        None => {
                            view! {
                                <div class="loading-state" style="text-align: center; padding: 3rem;">
                                    <div class="spinner" style="width: 40px; height: 40px; border: 3px solid #e2e8f0; border-top-color: #3b82f6; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto;"></div>
                                </div>
                                <style>{"@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"}</style>
                            }.into_any()
                        }
                        Some(Err(_)) => {
                            view! {
                                <div class="empty-state" style="text-align: center; padding: 3rem;">
                                    <p class="placeholder-text">{move || t("faq.coming_soon")}</p>
                                </div>
                            }.into_any()
                        }
                        Some(Ok(response)) if response.items.is_empty() => {
                            view! {
                                <div class="empty-state" style="text-align: center; padding: 3rem;">
                                    <p class="placeholder-text">{move || t("faq.coming_soon")}</p>
                                </div>
                            }.into_any()
                        }
                        Some(Ok(response)) => {
                            let items_store = StoredValue::new(response.items);

                            view! {
                                <header class="page-header">
                                    <h1>{move || t("faq.page_title")}</h1>
                                    <p>{move || t("faq.page_subtitle")}</p>
                                </header>

                                <section class="faq-section">
                                    <div class="container">
                                        <div class="faq-section-wrapper">
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
                                                        placeholder="Search questions..."
                                                        on:input=move |ev| {
                                                            set_search_query.set(event_target_value(&ev));
                                                            set_current_page.set(1);
                                                        }
                                                    />
                                                </div>

                                                <div class="faq-categories">
                                                    // "All" button
                                                    <button
                                                        class="category-btn"
                                                        class:active=move || selected_category.get().is_none()
                                                        on:click=move |_| {
                                                            set_selected_category.set(None);
                                                            set_current_page.set(1);
                                                        }
                                                    >
                                                        "All"
                                                    </button>
                                                    // Category buttons
                                                    {get_default_categories().into_iter().map(|cat| {
                                                        let cat_slug = cat.slug.clone();
                                                        let cat_slug_click = cat.slug.clone();
                                                        let cat_name = cat.name.clone();
                                                        let is_active = move || {
                                                            selected_category.get().as_ref() == Some(&cat_slug)
                                                        };
                                                        view! {
                                                            <button
                                                                class="category-btn"
                                                                class:active=is_active
                                                                on:click=move |_| {
                                                                    set_selected_category.set(Some(cat_slug_click.clone()));
                                                                    set_current_page.set(1);
                                                                }
                                                            >
                                                                {cat_name}
                                                            </button>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>

                                            // FAQ items with pagination
                                            {move || {
                                                let query = search_query.get().to_lowercase();
                                                let category = selected_category.get();
                                                let page = current_page.get();

                                                // Filter FAQs
                                                let filtered: Vec<FaqItemResponse> = items_store.with_value(|items| {
                                                    items.iter()
                                                        .filter(|item| {
                                                            let category_match = category.is_none() || category.as_ref() == Some(&item.category);
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
                                                let paginated: Vec<FaqItemResponse> = filtered.into_iter().skip(start).take(end - start).collect();

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
                    }
                }}
            </Suspense>
        </div>
    }
}

/// Capitalize category name for display
fn capitalize_category(cat: &str) -> String {
    let mut chars = cat.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Get color for category
fn get_category_color(category: &str) -> &'static str {
    match category {
        "general" => "#3b82f6",
        "earnings" => "#22c55e",
        "setup" => "#8b5cf6",
        "security" => "#ef4444",
        _ => "#64748b",
    }
}
