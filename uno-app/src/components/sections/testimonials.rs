//! CMS-driven Testimonials section component

use leptos::prelude::*;
use crate::api::HomeSection;
use crate::components::common::TestimonialData;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_components::{Carousel, CarouselSlide, SlidesQty, ResponsiveSlidesQty};

/// Parse testimonials from a JSON array
fn parse_testimonials(arr: &[serde_json::Value]) -> Vec<TestimonialData> {
    arr.iter()
        .enumerate()
        .filter_map(|(idx, t)| {
            let quote = t.get("quote").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let author_name = t.get("author_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let author_location = t.get("author_location").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let rating = t.get("rating").and_then(|v| v.as_i64()).unwrap_or(5) as u8;

            // Skip empty testimonials
            if quote.is_empty() && author_name.is_empty() {
                return None;
            }

            // Generate avatar from first letter of name
            let avatar = author_name.chars().next().unwrap_or('?').to_string();

            Some(TestimonialData {
                key: format!("cms-t{}", idx),
                quote_key: quote,
                author: author_name,
                location: author_location,
                avatar,
                rating,
            })
        })
        .collect()
}

/// CMS-driven Testimonials section
/// Testimonials are fetched from the database and injected into section.data by the API
#[component]
pub fn CmsTestimonialsSection(
    /// Section data from CMS (with testimonials injected from database)
    section: HomeSection,
) -> impl IntoView {
    let title = section.title.clone();
    let subtitle = section.description.clone();

    // Helper to get nested data (handles both flat and nested "data.data" structure)
    let nested_data = section.data.get("data");

    // Extract testimonials from section.data (check both nested and flat)
    // Testimonials are injected by the API from the database
    let testimonials: Vec<TestimonialData> = nested_data
        .and_then(|d| d.get("testimonials"))
        .or_else(|| section.data.get("testimonials"))
        .and_then(|v| v.as_array())
        .map(|arr| parse_testimonials(arr))
        .unwrap_or_default();

    // If no testimonials, don't render the section
    if testimonials.is_empty() {
        return view! { <div></div> }.into_any();
    }

    view! {
        <section class="section testimonials">
            <div class="container">
                <CmsTestimonialCarousel
                    testimonials=testimonials
                    title=title
                    subtitle=subtitle
                />
            </div>
        </section>
    }.into_any()
}

/// Custom testimonial carousel that uses direct text instead of locale keys
#[component]
fn CmsTestimonialCarousel(
    testimonials: Vec<TestimonialData>,
    title: String,
    subtitle: String,
) -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        let active_index = RwSignal::new(0usize);
        let testimonials_store = StoredValue::new(testimonials);

        view! {
            <div class="testimonial-carousel-wrapper">
                <h2 class="section-title">{title}</h2>
                <p class="section-subtitle">{subtitle}</p>

                <div class="testimonial-carousel">
                    {move || {
                        // Build slides with direct quote text
                        let slides: Vec<CarouselSlide> = testimonials_store.with_value(|testimonials| {
                            testimonials.iter().map(|data| {
                                // Use quote_key directly as the quote text (not a locale key)
                                CarouselSlide::testimonial(&data.key)
                                    .description(data.quote_key.clone())
                                    .author(&data.author)
                                    .author_location(&data.location)
                                    .author_avatar(&data.avatar)
                                    .rating(data.rating)
                            }).collect()
                        });

                        if slides.is_empty() {
                            view! { <p class="empty-state">"No testimonials available"</p> }.into_any()
                        } else {
                            view! {
                                <Carousel
                                    slides=slides
                                    active_index=active_index
                                    slides_qty=SlidesQty::Responsive(ResponsiveSlidesQty {
                                        xs: Some(1),
                                        sm: Some(1),
                                        md: Some(2),
                                        lg: Some(3),
                                        xl: Some(3),
                                        xxl: Some(3),
                                    })
                                    show_arrows=true
                                    show_pagination=true
                                    show_thumbnails=false
                                    autoplay=6000
                                    loop_slides=true
                                    class="testimonials-carousel"
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! { <p>"Loading testimonials..."</p> }
    }
}
