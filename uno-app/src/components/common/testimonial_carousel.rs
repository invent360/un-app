//! Testimonial Carousel Component
//!
//! A localized testimonial carousel using ember-fx Carousel in centered mode.
//! Supports both static testimonials and dynamic loading from JSON.
//!
//! # Example
//!
//! ```ignore
//! // Using locale keys for dynamic localization
//! let testimonials = vec![
//!     TestimonialData {
//!         key: "t1".to_string(),
//!         quote_key: "landing.testimonials.quote1".to_string(),
//!         author: "Maria".to_string(),
//!         location: "Philippines".to_string(),
//!         avatar: "M".to_string(),
//!         rating: 5,
//!     },
//! ];
//!
//! view! {
//!     <TestimonialCarousel
//!         testimonials=testimonials
//!         title_key="landing.testimonials.title"
//!         subtitle_key="landing.testimonials.subtitle"
//!     />
//! }
//! ```

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::t;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_components::{Carousel, CarouselSlide, SlidesQty, ResponsiveSlidesQty};

/// Data for a single testimonial.
///
/// Can use either direct values or locale keys for dynamic localization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestimonialData {
    /// Unique key for this testimonial.
    pub key: String,
    /// The quote/testimonial text OR a locale key (e.g., "landing.testimonials.quote1").
    pub quote_key: String,
    /// Author name.
    pub author: String,
    /// Author location/subtitle.
    pub location: String,
    /// Avatar (single letter or URL).
    #[serde(default)]
    pub avatar: String,
    /// Rating (1-5 stars).
    #[serde(default = "default_rating")]
    pub rating: u8,
}

fn default_rating() -> u8 {
    5
}

impl TestimonialData {
    /// Create a new testimonial with a locale key for the quote.
    pub fn new(key: impl Into<String>, quote_key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            quote_key: quote_key.into(),
            author: String::new(),
            location: String::new(),
            avatar: String::new(),
            rating: 5,
        }
    }

    /// Set the author name.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        // Auto-set avatar from first letter if not set
        if self.avatar.is_empty() {
            self.avatar = self.author.chars().next().unwrap_or('?').to_string();
        }
        self
    }

    /// Set the author location.
    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.location = location.into();
        self
    }

    /// Set the avatar (single letter or URL).
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = avatar.into();
        self
    }

    /// Set the rating (1-5 stars).
    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = rating.min(5);
        self
    }

    /// Parse testimonials from a JSON string.
    pub fn from_json(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert to ember-fx CarouselSlide.
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    pub fn to_carousel_slide(&self, quote_text: String) -> CarouselSlide {
        CarouselSlide::testimonial(&self.key)
            .description(quote_text)
            .author(&self.author)
            .author_location(&self.location)
            .author_avatar(&self.avatar)
            .rating(self.rating)
    }
}

/// Testimonial Carousel component with localization support.
///
/// Uses ember-fx Carousel in centered mode to display testimonials.
/// Quote text is resolved via the t() translation function.
///
/// # Props
///
/// - `testimonials` - Vector of testimonial data
/// - `title_key` - Locale key for the section title
/// - `subtitle_key` - Locale key for the section subtitle
/// - `class` - Additional CSS classes
#[component]
pub fn TestimonialCarousel(
    /// Testimonial data (can use locale keys for quotes).
    #[prop(into)]
    testimonials: Vec<TestimonialData>,
    /// Locale key for the title (e.g., "landing.testimonials.title").
    #[prop(optional, into)]
    title_key: Option<String>,
    /// Locale key for the subtitle.
    #[prop(optional, into)]
    subtitle_key: Option<String>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        let active_index = RwSignal::new(0usize);

        // Store testimonials for reactive access
        let testimonials_store = StoredValue::new(testimonials);

        // Build CSS classes
        let wrapper_class = format!(
            "testimonial-carousel {}",
            class.unwrap_or_default()
        );

        view! {
            <section class=wrapper_class>
                // Header with localized title/subtitle
                {title_key.clone().map(|key| {
                    view! {
                        <h2 class="testimonial-carousel-title">
                            {move || t(&key)}
                        </h2>
                    }
                })}

                {subtitle_key.clone().map(|key| {
                    view! {
                        <p class="testimonial-carousel-subtitle">
                            {move || t(&key)}
                        </p>
                    }
                })}

                // Dynamic carousel that reads localized quotes
                <div class="testimonial-carousel-wrapper">
                    {move || {
                        // Build slides with translated quotes
                        let slides: Vec<CarouselSlide> = testimonials_store.with_value(|testimonials| {
                            testimonials.iter().map(|data| {
                                // Resolve quote using translation function
                                let quote = t(&data.quote_key);
                                data.to_carousel_slide(quote)
                            }).collect()
                        });

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
                                    xxl: None,
                                })
                                is_centered=true
                                is_draggable=true
                                is_auto_height=true
                                show_pagination=true
                                show_arrows=true
                                show_thumbnails=false
                                loop_slides=true
                                autoplay=5000
                                class="testimonials-carousel-inner"
                            />
                        }
                    }}
                </div>
            </section>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <section class="testimonial-carousel">
                <p>"Testimonial carousel requires csr, hydrate, or ssr feature"</p>
            </section>
        }
    }
}

/// Star rating component for testimonials.
#[component]
pub fn StarRating(
    /// Rating value (1-5).
    #[prop(default = 5)]
    rating: u8,
) -> impl IntoView {
    let stars = (0..5).map(|i| {
        if i < rating {
            view! { <span class="star star-filled">"★"</span> }
        } else {
            view! { <span class="star">"☆"</span> }
        }
    }).collect_view();

    view! {
        <div class="star-rating">
            {stars}
        </div>
    }
}
