//! CMS-driven Hero section component

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::{HomeSection, check_license_availability};
use crate::components::wizard::use_wizard_state;

/// CMS-driven Hero section that renders from API data
#[component]
pub fn CmsHeroSection(
    /// Section data from CMS
    section: HomeSection,
) -> impl IntoView {
    // Get wizard state from app-level context
    let wizard_state = use_wizard_state();

    // Handler to open wizard and check availability
    let open_wizard = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        if let Some(state) = wizard_state.clone() {
            state.open();
            // Check availability asynchronously
            let s = state.clone();
            spawn_local(async move {
                match check_license_availability().await {
                    Ok(available) => s.set_availability(available),
                    Err(_) => s.set_availability(false),
                }
            });
        }
    };

    // Helper to get nested data (handles both flat and nested "data.data" structure)
    let nested_data = section.data.get("data");

    // Extract hero-specific fields from section.data or section level
    let headline = section.data.get("headline")
        .and_then(|v| v.as_str())
        .or_else(|| section.data.get("title").and_then(|v| v.as_str()))
        .unwrap_or(&section.title)
        .to_string();

    let headline_highlight = section.data.get("headline_highlight")
        .and_then(|v| v.as_str())
        .unwrap_or("For Free")
        .to_string();

    // Use section.description if subheadline not in data
    let subheadline = section.data.get("subheadline")
        .and_then(|v| v.as_str())
        .or_else(|| section.data.get("description").and_then(|v| v.as_str()))
        .map(|s| s.to_string())
        .unwrap_or_else(|| section.description.clone());

    // Extract CTA button
    let cta_text = section.data.get("cta_button")
        .and_then(|v| v.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("Get Started")
        .to_string();

    let cta_link = section.data.get("cta_button")
        .and_then(|v| v.get("link"))
        .and_then(|v| v.as_str())
        .unwrap_or("#")
        .to_string();

    // Extract highlights/benefits (check both nested data.data.highlights and flat data.highlights)
    let highlights: Vec<String> = nested_data
        .and_then(|d| d.get("highlights"))
        .or_else(|| section.data.get("highlights"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|h| {
                    // Handle both object format { text: "..." } and string format
                    h.get("text")
                        .and_then(|t| t.as_str())
                        .or_else(|| h.as_str())
                        .map(String::from)
                })
                .collect()
        })
        .unwrap_or_default();

    // Extract media/images for carousel (check nested and flat, both "media" and "images" keys)
    let media: Vec<String> = nested_data
        .and_then(|d| d.get("media").or_else(|| d.get("images")))
        .or_else(|| section.data.get("media"))
        .or_else(|| section.data.get("images"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    view! {
        <section class="hero">
            <div class="container hero-content">
                <div class="hero-text">
                    <h1 class="hero-title">
                        {headline}
                        <span class="highlight">{headline_highlight}</span>
                    </h1>
                    <p class="hero-subtitle">
                        {subheadline}
                    </p>
                    <div class="hero-cta">
                        <a href={cta_link} class="btn btn-primary btn-large" on:click=open_wizard>
                            {cta_text}
                        </a>
                    </div>
                    <div class="hero-benefits">
                        {highlights.into_iter().map(|text| {
                            view! {
                                <div class="benefit">
                                    <CheckIcon/>
                                    <span>{text}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
                <div class="hero-visual">
                    <CmsPhoneCarousel media=media />
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

/// Phone carousel component that displays CMS media
#[component]
fn CmsPhoneCarousel(
    /// Media URLs from CMS
    media: Vec<String>,
) -> impl IntoView {
    use ember_fx_components::{Carousel, CarouselSlide, CarouselSize};

    let active_index = RwSignal::new(0usize);

    view! {
        <div class="phone-mockup phone-carousel-container">
            {move || {
                if media.is_empty() {
                    // Placeholder when no media
                    view! {
                        <div class="phone-screen phone-carousel-screen phone-carousel-loading">
                            <div class="carousel-placeholder">
                                <div class="placeholder-shimmer"></div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // Build carousel slides from CMS media
                    let slides: Vec<CarouselSlide> = media.iter().enumerate().map(|(idx, url)| {
                        CarouselSlide::image(&format!("slide-{}", idx), url)
                            .alt(&format!("Slide {}", idx + 1))
                    }).collect();

                    view! {
                        <div class="phone-screen phone-carousel-screen">
                            <Carousel
                                slides=slides
                                active_index=active_index
                                size=CarouselSize::Large
                                show_arrows=false
                                show_pagination=true
                                show_thumbnails=false
                                autoplay=5000
                                loop_slides=true
                                class="hero-phone-carousel"
                            />
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Check icon component
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg class="icon icon-check" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
            <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
    }
}
