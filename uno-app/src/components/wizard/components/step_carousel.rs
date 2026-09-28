//! Step Carousel Component
//!
//! A modular carousel component for wizard steps that supports dynamic content configuration.
//! Content can be passed via JSON-like configuration or directly as props.
//!
//! # Example
//!
//! ```ignore
//! use crate::components::wizard::components::{StepCarousel, SlideConfig};
//!
//! // Define slides dynamically
//! let slides = vec![
//!     SlideConfig::new("slide1", "/images/step1.png")
//!         .title("Step 1: Open the app")
//!         .description("Launch the app on your device"),
//!     SlideConfig::new("slide2", "/images/step2.png")
//!         .title("Step 2: Create account")
//!         .description("Enter your email and create a password"),
//! ];
//!
//! view! {
//!     <StepCarousel
//!         slides=slides
//!         carousel_title="Sign Up Process"
//!         carousel_description="Follow these steps to create your account"
//!     />
//! }
//! ```
//!
//! # JSON Configuration
//!
//! Slides can also be created from JSON:
//!
//! ```json
//! [
//!   {
//!     "key": "slide1",
//!     "media": "/images/step1.png",
//!     "media_type": "image",
//!     "title": "Step 1: Open the app",
//!     "description": "Launch the app on your device"
//!   },
//!   {
//!     "key": "slide2",
//!     "media": "/videos/step2.mp4",
//!     "media_type": "video",
//!     "title": "Step 2: Create account",
//!     "description": "Enter your email and create a password"
//!   }
//! ]
//! ```

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_components::{Carousel, CarouselSlide, CarouselSize as FxCarouselSize};

/// Carousel size variants
/// Mirrors ember_fx_components::CarouselSize for standalone use
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CarouselSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
impl CarouselSize {
    /// Convert to ember-fx CarouselSize
    fn to_fx_size(self) -> FxCarouselSize {
        match self {
            CarouselSize::Small => FxCarouselSize::Small,
            CarouselSize::Medium => FxCarouselSize::Medium,
            CarouselSize::Large => FxCarouselSize::Large,
        }
    }
}

/// Media type for carousel slides
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    #[default]
    Image,
    Video,
}

/// Configuration for a single carousel slide.
///
/// This struct provides a clean API for defining carousel content
/// that can be serialized/deserialized from JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlideConfig {
    /// Unique key for the slide
    pub key: String,
    /// Media source URL (image or video)
    pub media: String,
    /// Type of media (image or video)
    #[serde(default)]
    pub media_type: MediaType,
    /// Optional slide title
    #[serde(default)]
    pub title: Option<String>,
    /// Optional slide description
    #[serde(default)]
    pub description: Option<String>,
    /// Optional custom thumbnail URL
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Optional alt text for images
    #[serde(default)]
    pub alt: Option<String>,
    /// Optional video poster image
    #[serde(default)]
    pub poster: Option<String>,
}

impl SlideConfig {
    /// Create a new slide configuration with a key and media URL.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for this slide
    /// * `media` - URL to the image or video
    ///
    /// # Example
    ///
    /// ```ignore
    /// let slide = SlideConfig::new("step1", "/images/signup-step1.png")
    ///     .title("Open the App")
    ///     .description("Launch the UNetwork app on your device");
    /// ```
    pub fn new(key: impl Into<String>, media: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            media: media.into(),
            media_type: MediaType::Image,
            title: None,
            description: None,
            thumbnail: None,
            alt: None,
            poster: None,
        }
    }

    /// Create a new video slide configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for this slide
    /// * `media` - URL to the video
    pub fn video(key: impl Into<String>, media: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            media: media.into(),
            media_type: MediaType::Video,
            title: None,
            description: None,
            thumbnail: None,
            alt: None,
            poster: None,
        }
    }

    /// Set the slide title (builder pattern).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the slide description (builder pattern).
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set a custom thumbnail URL (builder pattern).
    pub fn thumbnail(mut self, thumbnail: impl Into<String>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    /// Set alt text for image slides (builder pattern).
    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    /// Set poster image for video slides (builder pattern).
    pub fn poster(mut self, poster: impl Into<String>) -> Self {
        self.poster = Some(poster.into());
        self
    }

    /// Convert to ember-fx-components CarouselSlide.
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    fn to_carousel_slide(&self) -> CarouselSlide {
        match self.media_type {
            MediaType::Image => {
                let mut slide = CarouselSlide::image(&self.key, &self.media);
                if let Some(ref title) = self.title {
                    slide = slide.title(title.clone());
                }
                if let Some(ref description) = self.description {
                    slide = slide.description(description.clone());
                }
                if let Some(ref thumbnail) = self.thumbnail {
                    slide = slide.thumbnail(thumbnail.clone());
                }
                if let Some(ref alt) = self.alt {
                    slide = slide.alt(alt.clone());
                }
                slide
            }
            MediaType::Video => {
                let mut slide = CarouselSlide::video(&self.key, &self.media);
                if let Some(ref title) = self.title {
                    slide = slide.title(title.clone());
                }
                if let Some(ref description) = self.description {
                    slide = slide.description(description.clone());
                }
                if let Some(ref thumbnail) = self.thumbnail {
                    slide = slide.thumbnail(thumbnail.clone());
                }
                if let Some(ref poster) = self.poster {
                    slide = slide.poster(poster.clone());
                }
                slide
            }
        }
    }
}

/// Parse slide configurations from a JSON string.
///
/// # Arguments
///
/// * `json` - JSON string containing an array of slide configurations
///
/// # Example
///
/// ```ignore
/// let json = r#"[
///   {"key": "s1", "media": "/img/1.png", "title": "Step 1"},
///   {"key": "s2", "media": "/img/2.png", "title": "Step 2"}
/// ]"#;
///
/// let slides = parse_slides_from_json(json).unwrap();
/// ```
pub fn parse_slides_from_json(json: &str) -> Result<Vec<SlideConfig>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Step Carousel component for wizard steps.
///
/// A modular carousel that wraps ember-fx-components Carousel with
/// a clean configuration API suitable for wizard step content.
///
/// # Props
///
/// - `slides` - Vector of slide configurations
/// - `carousel_title` - Optional title for the entire carousel
/// - `carousel_description` - Optional description for the entire carousel
/// - `size` - Carousel size (Small, Medium, Large)
/// - `show_thumbnails` - Whether to show thumbnail navigation (default: true)
/// - `loop_slides` - Enable infinite loop navigation (default: true)
/// - `class` - Additional CSS classes
#[component]
pub fn StepCarousel(
    /// Slide configurations to display.
    #[prop(into)]
    slides: Vec<SlideConfig>,
    /// Optional carousel title (displayed in header).
    #[prop(optional, into)]
    carousel_title: Option<String>,
    /// Optional carousel description (displayed in header).
    #[prop(optional, into)]
    carousel_description: Option<String>,
    /// Carousel size variant.
    #[prop(optional)]
    size: Option<CarouselSize>,
    /// Show thumbnail navigation (default: true).
    #[prop(optional)]
    #[prop(default = true)]
    show_thumbnails: bool,
    /// Enable infinite loop (default: true).
    #[prop(optional)]
    #[prop(default = true)]
    loop_slides: bool,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // Convert SlideConfig to CarouselSlide
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    let carousel_slides: Vec<CarouselSlide> = slides
        .iter()
        .map(|s| s.to_carousel_slide())
        .collect();

    // Active slide index
    let active_index = RwSignal::new(0usize);

    // Build class string
    let class_str = class.unwrap_or_default();
    let wrapper_class = format!("step-carousel {}", class_str);

    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        // Extract title and description - Carousel expects String, not Option<String>
        let title_str = carousel_title.unwrap_or_default();
        let desc_str = carousel_description.unwrap_or_default();

        let fx_size = size.unwrap_or(CarouselSize::Medium).to_fx_size();
        view! {
            <div class=wrapper_class>
                <Carousel
                    slides=carousel_slides
                    active_index=active_index
                    title=title_str
                    description=desc_str
                    size=fx_size
                    show_thumbnails=show_thumbnails
                    loop_slides=loop_slides
                />
            </div>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <div class=wrapper_class>
                <p>"Carousel requires csr, hydrate, or ssr feature"</p>
            </div>
        }
    }
}

/// Helper macro for creating slide configurations inline.
///
/// # Example
///
/// ```ignore
/// let slides = slides![
///     ("step1", "/images/step1.png", "Open App", "Launch the app on your device"),
///     ("step2", "/images/step2.png", "Sign In", "Enter your credentials"),
/// ];
/// ```
#[macro_export]
macro_rules! slides {
    [$(($key:expr, $media:expr, $title:expr, $desc:expr)),* $(,)?] => {
        vec![
            $(
                SlideConfig::new($key, $media)
                    .title($title)
                    .description($desc),
            )*
        ]
    };
    [$(($key:expr, $media:expr, $title:expr)),* $(,)?] => {
        vec![
            $(
                SlideConfig::new($key, $media)
                    .title($title),
            )*
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_config_builder() {
        let slide = SlideConfig::new("test", "/image.png")
            .title("Test Title")
            .description("Test Description")
            .alt("Alt text");

        assert_eq!(slide.key, "test");
        assert_eq!(slide.media, "/image.png");
        assert_eq!(slide.title, Some("Test Title".to_string()));
        assert_eq!(slide.description, Some("Test Description".to_string()));
        assert_eq!(slide.alt, Some("Alt text".to_string()));
    }

    #[test]
    fn test_video_slide_config() {
        let slide = SlideConfig::video("vid1", "/video.mp4")
            .title("Video Title")
            .poster("/poster.png");

        assert_eq!(slide.media_type, MediaType::Video);
        assert_eq!(slide.poster, Some("/poster.png".to_string()));
    }

    #[test]
    fn test_parse_slides_from_json() {
        let json = r#"[
            {"key": "s1", "media": "/img/1.png", "title": "Step 1"},
            {"key": "s2", "media": "/img/2.png", "media_type": "image", "title": "Step 2", "description": "Do this"}
        ]"#;

        let slides = parse_slides_from_json(json).unwrap();
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].key, "s1");
        assert_eq!(slides[0].title, Some("Step 1".to_string()));
        assert_eq!(slides[1].description, Some("Do this".to_string()));
    }
}
