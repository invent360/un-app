//! Media component types.
//!
//! This module contains type definitions for media components including
//! the Carousel component.
//!
//! # Dynamic Content Loading
//!
//! Slides can be loaded from JSON for dynamic content:
//!
//! ```ignore
//! let json = r#"[
//!     {"key": "1", "type": "image", "src": "/img/1.jpg", "title": "Slide 1"},
//!     {"key": "2", "type": "image", "src": "/img/2.jpg", "title": "Slide 2"}
//! ]"#;
//!
//! let slides = CarouselSlide::from_json(json).unwrap();
//! ```

use std::fmt::{self, Display};
use serde::{Deserialize, Serialize};

/// Carousel size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CarouselSize {
    /// Small carousel (300px height).
    Small,
    /// Medium carousel (400px height, default).
    #[default]
    Medium,
    /// Large carousel (500px height).
    Large,
}

impl CarouselSize {
    /// Returns the CSS class suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }

    /// Returns the full CSS class for this size.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Returns the height in pixels for this size.
    pub fn height_px(&self) -> u32 {
        match self {
            Self::Small => 300,
            Self::Medium => 400,
            Self::Large => 500,
        }
    }
}

impl Display for CarouselSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Carousel transition effect.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CarouselEffect {
    /// Slide transition using CSS transforms (default).
    #[default]
    Slide,
    /// Fade transition using opacity.
    Fade,
}

impl CarouselEffect {
    /// Returns the CSS class suffix for this effect.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Slide => "effect-slide",
            Self::Fade => "effect-fade",
        }
    }

    /// Returns the full CSS class for this effect.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl Display for CarouselEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Thumbnail navigation position.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ThumbnailPosition {
    /// Thumbnails at the bottom (default).
    #[default]
    Bottom,
    /// Thumbnails at the top.
    Top,
    /// Thumbnails on the left side.
    Left,
    /// Thumbnails on the right side.
    Right,
}

impl ThumbnailPosition {
    /// Returns the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Bottom => "thumbs-bottom",
            Self::Top => "thumbs-top",
            Self::Left => "thumbs-left",
            Self::Right => "thumbs-right",
        }
    }

    /// Returns the full CSS class for this position.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Returns true if thumbnails are positioned horizontally.
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Bottom | Self::Top)
    }

    /// Returns true if thumbnails are positioned vertically.
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

impl Display for ThumbnailPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

// ============================================
// New Preline-style carousel types
// ============================================

/// Carousel display mode.
///
/// Determines how the carousel handles slide transitions and interactions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CarouselMode {
    /// Transform-based sliding with CSS transforms (default).
    /// Provides smooth transitions and supports drag/swipe.
    #[default]
    Default,
    /// CSS scroll-snap mode using native browser scrolling.
    /// Better performance on mobile but limited animation control.
    ScrollSnap,
}

impl CarouselMode {
    /// Returns the CSS class suffix for this mode.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "mode-default",
            Self::ScrollSnap => "mode-snap",
        }
    }

    /// Returns true if this mode uses scroll-snap.
    pub fn is_snap(&self) -> bool {
        matches!(self, Self::ScrollSnap)
    }
}

impl Display for CarouselMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Pagination dot position.
///
/// Controls where the pagination dots are displayed relative to the carousel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum PaginationPosition {
    /// Dots at the bottom inside the viewport (default).
    #[default]
    Bottom,
    /// Dots at the top inside the viewport.
    Top,
    /// Dots at the bottom outside the viewport.
    BottomOutside,
    /// Dots at the top outside the viewport.
    TopOutside,
}

impl PaginationPosition {
    /// Returns the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Bottom => "pagination-bottom",
            Self::Top => "pagination-top",
            Self::BottomOutside => "pagination-bottom-outside",
            Self::TopOutside => "pagination-top-outside",
        }
    }

    /// Returns true if pagination is outside the viewport.
    pub fn is_outside(&self) -> bool {
        matches!(self, Self::BottomOutside | Self::TopOutside)
    }
}

impl Display for PaginationPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Responsive slides quantity configuration.
///
/// Allows specifying different number of visible slides at different breakpoints.
/// Breakpoints follow Tailwind CSS conventions.
///
/// # Example
///
/// ```ignore
/// let responsive = ResponsiveSlidesQty {
///     xs: Some(1),   // 1 slide on mobile
///     sm: Some(2),   // 2 slides at 640px+
///     md: Some(3),   // 3 slides at 768px+
///     lg: Some(4),   // 4 slides at 1024px+
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponsiveSlidesQty {
    /// Extra small screens (0px+).
    pub xs: Option<u32>,
    /// Small screens (640px+).
    pub sm: Option<u32>,
    /// Medium screens (768px+).
    pub md: Option<u32>,
    /// Large screens (1024px+).
    pub lg: Option<u32>,
    /// Extra large screens (1280px+).
    pub xl: Option<u32>,
    /// 2X Extra large screens (1536px+).
    pub xxl: Option<u32>,
}

impl Default for ResponsiveSlidesQty {
    fn default() -> Self {
        Self {
            xs: Some(1),
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }
}

impl ResponsiveSlidesQty {
    /// Create a responsive config starting with the given number of slides.
    pub fn new(xs: u32) -> Self {
        Self {
            xs: Some(xs),
            ..Default::default()
        }
    }

    /// Set slides for small screens (640px+).
    pub fn sm(mut self, qty: u32) -> Self {
        self.sm = Some(qty);
        self
    }

    /// Set slides for medium screens (768px+).
    pub fn md(mut self, qty: u32) -> Self {
        self.md = Some(qty);
        self
    }

    /// Set slides for large screens (1024px+).
    pub fn lg(mut self, qty: u32) -> Self {
        self.lg = Some(qty);
        self
    }

    /// Set slides for extra large screens (1280px+).
    pub fn xl(mut self, qty: u32) -> Self {
        self.xl = Some(qty);
        self
    }

    /// Set slides for 2x extra large screens (1536px+).
    pub fn xxl(mut self, qty: u32) -> Self {
        self.xxl = Some(qty);
        self
    }

    /// Get the number of slides for a given viewport width.
    pub fn get_for_width(&self, width: u32) -> u32 {
        if width >= 1536 {
            self.xxl.or(self.xl).or(self.lg).or(self.md).or(self.sm).or(self.xs).unwrap_or(1)
        } else if width >= 1280 {
            self.xl.or(self.lg).or(self.md).or(self.sm).or(self.xs).unwrap_or(1)
        } else if width >= 1024 {
            self.lg.or(self.md).or(self.sm).or(self.xs).unwrap_or(1)
        } else if width >= 768 {
            self.md.or(self.sm).or(self.xs).unwrap_or(1)
        } else if width >= 640 {
            self.sm.or(self.xs).unwrap_or(1)
        } else {
            self.xs.unwrap_or(1)
        }
    }
}

/// Number of visible slides configuration.
///
/// Can be a fixed number or responsive based on viewport width.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SlidesQty {
    /// Fixed number of visible slides.
    Fixed(u32),
    /// Responsive number based on viewport breakpoints.
    Responsive(ResponsiveSlidesQty),
}

impl Default for SlidesQty {
    fn default() -> Self {
        Self::Fixed(1)
    }
}

impl SlidesQty {
    /// Create a fixed slides quantity.
    pub fn fixed(qty: u32) -> Self {
        Self::Fixed(qty)
    }

    /// Create a responsive slides quantity.
    pub fn responsive(config: ResponsiveSlidesQty) -> Self {
        Self::Responsive(config)
    }

    /// Get the number of slides for a given viewport width.
    pub fn get_for_width(&self, width: u32) -> u32 {
        match self {
            Self::Fixed(qty) => *qty,
            Self::Responsive(config) => config.get_for_width(width),
        }
    }

    /// Returns true if this is responsive.
    pub fn is_responsive(&self) -> bool {
        matches!(self, Self::Responsive(_))
    }
}

impl From<u32> for SlidesQty {
    fn from(qty: u32) -> Self {
        Self::Fixed(qty)
    }
}

impl From<ResponsiveSlidesQty> for SlidesQty {
    fn from(config: ResponsiveSlidesQty) -> Self {
        Self::Responsive(config)
    }
}

/// Loading state class configuration.
///
/// Controls CSS classes applied during carousel loading animation.
/// Based on Preline's loading state system.
///
/// # Example
///
/// ```ignore
/// let loading = LoadingClasses {
///     remove: Some("opacity-0".to_string()),
///     add: Some("opacity-100 transition-opacity".to_string()),
///     add_after: Some("duration-300".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoadingClasses {
    /// Classes to remove when carousel initializes.
    pub remove: Option<String>,
    /// Classes to add immediately on init.
    pub add: Option<String>,
    /// Classes to add after a brief delay (creates stagger effect).
    pub add_after: Option<String>,
}

impl LoadingClasses {
    /// Create a new loading classes configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set classes to remove on init.
    pub fn remove(mut self, classes: impl Into<String>) -> Self {
        self.remove = Some(classes.into());
        self
    }

    /// Set classes to add immediately.
    pub fn add(mut self, classes: impl Into<String>) -> Self {
        self.add = Some(classes.into());
        self
    }

    /// Set classes to add after delay.
    pub fn add_after(mut self, classes: impl Into<String>) -> Self {
        self.add_after = Some(classes.into());
        self
    }

    /// Check if any loading classes are configured.
    pub fn is_configured(&self) -> bool {
        self.remove.is_some() || self.add.is_some() || self.add_after.is_some()
    }
}

/// Slide content type - either an image or a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SlideContent {
    /// Image slide with URL and optional alt text.
    Image {
        /// Image source URL.
        src: String,
        /// Alt text for accessibility.
        #[serde(default)]
        alt: Option<String>,
    },
    /// Video slide with URL and playback options.
    Video {
        /// Video source URL.
        src: String,
        /// Poster image URL.
        #[serde(default)]
        poster: Option<String>,
        /// Auto-play the video when slide is active.
        #[serde(default)]
        autoplay: bool,
        /// Mute the video.
        #[serde(default = "default_true")]
        muted: bool,
        /// Show video controls.
        #[serde(default = "default_true")]
        controls: bool,
    },
    /// Custom HTML content (for testimonials, cards, etc.)
    Custom {
        /// Custom HTML or component identifier.
        #[serde(default)]
        html: Option<String>,
    },
}

fn default_true() -> bool {
    true
}

impl SlideContent {
    /// Returns true if this is an image slide.
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image { .. })
    }

    /// Returns true if this is a video slide.
    pub fn is_video(&self) -> bool {
        matches!(self, Self::Video { .. })
    }

    /// Returns true if this is a custom content slide.
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }

    /// Returns the source URL if available.
    pub fn src(&self) -> Option<&str> {
        match self {
            Self::Image { src, .. } => Some(src),
            Self::Video { src, .. } => Some(src),
            Self::Custom { .. } => None,
        }
    }
}

/// Configuration for a carousel slide.
///
/// Use the builder pattern to create slides:
///
/// ```ignore
/// let slide = CarouselSlide::image("slide-1", "/images/photo.jpg")
///     .title("Beautiful Sunset")
///     .description("A stunning view over the ocean")
///     .alt("Sunset over the ocean");
/// ```
///
/// Or load from JSON:
///
/// ```ignore
/// let json = r#"[
///     {"key": "1", "type": "image", "src": "/img/1.jpg", "title": "Slide 1"},
///     {"key": "2", "type": "image", "src": "/img/2.jpg", "title": "Slide 2"}
/// ]"#;
/// let slides = CarouselSlide::from_json(json).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarouselSlide {
    /// Unique key for the slide (used for rendering).
    pub key: String,
    /// The slide content (image or video).
    #[serde(flatten)]
    pub content: SlideContent,
    /// Optional title displayed over the slide.
    #[serde(default)]
    pub title: Option<String>,
    /// Optional description displayed over the slide.
    #[serde(default)]
    pub description: Option<String>,
    /// Custom thumbnail URL (defaults to image src or video poster).
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Optional author name (for testimonials).
    #[serde(default)]
    pub author: Option<String>,
    /// Optional author location/subtitle.
    #[serde(default)]
    pub author_location: Option<String>,
    /// Optional author avatar URL or initial.
    #[serde(default)]
    pub author_avatar: Option<String>,
    /// Optional rating (1-5 stars).
    #[serde(default)]
    pub rating: Option<u8>,
}

impl CarouselSlide {
    /// Create a new image slide.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for this slide
    /// * `src` - Image source URL
    ///
    /// # Example
    ///
    /// ```ignore
    /// let slide = CarouselSlide::image("1", "/images/photo.jpg")
    ///     .title("Photo Title")
    ///     .alt("Alt text for accessibility");
    /// ```
    pub fn image(key: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            content: SlideContent::Image {
                src: src.into(),
                alt: None,
            },
            title: None,
            description: None,
            thumbnail: None,
            author: None,
            author_location: None,
            author_avatar: None,
            rating: None,
        }
    }

    /// Create a new video slide.
    ///
    /// # Arguments
    ///
    /// * `key` - Unique identifier for this slide
    /// * `src` - Video source URL
    ///
    /// # Example
    ///
    /// ```ignore
    /// let slide = CarouselSlide::video("1", "/videos/demo.mp4")
    ///     .poster("/images/poster.jpg")
    ///     .title("Demo Video");
    /// ```
    pub fn video(key: impl Into<String>, src: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            content: SlideContent::Video {
                src: src.into(),
                poster: None,
                autoplay: false,
                muted: true,
                controls: true,
            },
            title: None,
            description: None,
            thumbnail: None,
            author: None,
            author_location: None,
            author_avatar: None,
            rating: None,
        }
    }

    /// Create a testimonial slide (custom content).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let slide = CarouselSlide::testimonial("t1")
    ///     .description("Great product!")
    ///     .author("John Doe")
    ///     .author_location("New York, USA")
    ///     .author_avatar("J")
    ///     .rating(5);
    /// ```
    pub fn testimonial(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            content: SlideContent::Custom { html: None },
            title: None,
            description: None,
            thumbnail: None,
            author: None,
            author_location: None,
            author_avatar: None,
            rating: None,
        }
    }

    /// Parse slides from a JSON string.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let json = r#"[
    ///     {"key": "1", "type": "image", "src": "/img/1.jpg", "title": "Slide 1"},
    ///     {"key": "2", "type": "image", "src": "/img/2.jpg", "title": "Slide 2"}
    /// ]"#;
    /// let slides = CarouselSlide::from_json(json).unwrap();
    /// ```
    pub fn from_json(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Parse a single slide from a JSON string.
    pub fn one_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert slides to a JSON string.
    pub fn to_json(slides: &[Self]) -> Result<String, serde_json::Error> {
        serde_json::to_string(slides)
    }

    /// Convert slides to a pretty-printed JSON string.
    pub fn to_json_pretty(slides: &[Self]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(slides)
    }

    /// Set the alt text for image slides.
    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        if let SlideContent::Image { alt: ref mut a, .. } = self.content {
            *a = Some(alt.into());
        }
        self
    }

    /// Set the poster image for video slides.
    pub fn poster(mut self, poster: impl Into<String>) -> Self {
        if let SlideContent::Video { poster: ref mut p, .. } = self.content {
            *p = Some(poster.into());
        }
        self
    }

    /// Set whether video should autoplay when slide is active.
    pub fn autoplay(mut self, autoplay: bool) -> Self {
        if let SlideContent::Video { autoplay: ref mut a, .. } = self.content {
            *a = autoplay;
        }
        self
    }

    /// Set whether video should be muted.
    pub fn muted(mut self, muted: bool) -> Self {
        if let SlideContent::Video { muted: ref mut m, .. } = self.content {
            *m = muted;
        }
        self
    }

    /// Set whether video controls should be shown.
    pub fn controls(mut self, controls: bool) -> Self {
        if let SlideContent::Video { controls: ref mut c, .. } = self.content {
            *c = controls;
        }
        self
    }

    /// Set the slide title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the slide description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set a custom thumbnail URL.
    pub fn thumbnail(mut self, thumbnail: impl Into<String>) -> Self {
        self.thumbnail = Some(thumbnail.into());
        self
    }

    /// Set the author name (for testimonials).
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the author location/subtitle (for testimonials).
    pub fn author_location(mut self, location: impl Into<String>) -> Self {
        self.author_location = Some(location.into());
        self
    }

    /// Set the author avatar (URL or single initial).
    pub fn author_avatar(mut self, avatar: impl Into<String>) -> Self {
        self.author_avatar = Some(avatar.into());
        self
    }

    /// Set the rating (1-5 stars).
    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = Some(rating.min(5));
        self
    }

    /// Check if this is a testimonial slide.
    pub fn is_testimonial(&self) -> bool {
        matches!(self.content, SlideContent::Custom { .. }) || self.author.is_some()
    }

    /// Get the thumbnail URL for this slide.
    ///
    /// Returns the custom thumbnail if set, otherwise falls back to:
    /// - Image source for image slides
    /// - Video poster for video slides
    /// - Author avatar for custom/testimonial slides
    /// - None if no fallback is available
    pub fn get_thumbnail(&self) -> Option<String> {
        self.thumbnail.clone().or_else(|| match &self.content {
            SlideContent::Image { src, .. } => Some(src.clone()),
            SlideContent::Video { poster, .. } => poster.clone(),
            SlideContent::Custom { .. } => self.author_avatar.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_size() {
        assert_eq!(CarouselSize::Small.as_suffix(), "sm");
        assert_eq!(CarouselSize::Medium.as_suffix(), "md");
        assert_eq!(CarouselSize::Large.as_suffix(), "lg");
        assert_eq!(CarouselSize::Small.height_px(), 300);
        assert_eq!(CarouselSize::Medium.height_px(), 400);
        assert_eq!(CarouselSize::Large.height_px(), 500);
    }

    #[test]
    fn test_thumbnail_position() {
        assert!(ThumbnailPosition::Bottom.is_horizontal());
        assert!(ThumbnailPosition::Top.is_horizontal());
        assert!(ThumbnailPosition::Left.is_vertical());
        assert!(ThumbnailPosition::Right.is_vertical());
    }

    #[test]
    fn test_carousel_slide_image() {
        let slide = CarouselSlide::image("1", "/test.jpg")
            .title("Test Title")
            .alt("Test Alt")
            .description("Test Description");

        assert_eq!(slide.key, "1");
        assert!(slide.content.is_image());
        assert_eq!(slide.title, Some("Test Title".to_string()));
        assert_eq!(slide.description, Some("Test Description".to_string()));
        assert_eq!(slide.get_thumbnail(), Some("/test.jpg".to_string()));
    }

    #[test]
    fn test_carousel_slide_video() {
        let slide = CarouselSlide::video("1", "/test.mp4")
            .poster("/poster.jpg")
            .autoplay(true)
            .muted(false);

        assert_eq!(slide.key, "1");
        assert!(slide.content.is_video());
        assert_eq!(slide.get_thumbnail(), Some("/poster.jpg".to_string()));
    }

    #[test]
    fn test_carousel_mode() {
        assert_eq!(CarouselMode::Default.as_suffix(), "mode-default");
        assert_eq!(CarouselMode::ScrollSnap.as_suffix(), "mode-snap");
        assert!(!CarouselMode::Default.is_snap());
        assert!(CarouselMode::ScrollSnap.is_snap());
    }

    #[test]
    fn test_pagination_position() {
        assert!(!PaginationPosition::Bottom.is_outside());
        assert!(!PaginationPosition::Top.is_outside());
        assert!(PaginationPosition::BottomOutside.is_outside());
        assert!(PaginationPosition::TopOutside.is_outside());
    }

    #[test]
    fn test_responsive_slides_qty() {
        let responsive = ResponsiveSlidesQty::new(1)
            .sm(2)
            .md(3)
            .lg(4);

        assert_eq!(responsive.get_for_width(400), 1);   // xs
        assert_eq!(responsive.get_for_width(640), 2);   // sm
        assert_eq!(responsive.get_for_width(768), 3);   // md
        assert_eq!(responsive.get_for_width(1024), 4);  // lg
        assert_eq!(responsive.get_for_width(1280), 4);  // xl (falls back to lg)
        assert_eq!(responsive.get_for_width(1536), 4);  // xxl (falls back to lg)
    }

    #[test]
    fn test_slides_qty() {
        let fixed = SlidesQty::Fixed(3);
        assert_eq!(fixed.get_for_width(500), 3);
        assert_eq!(fixed.get_for_width(1000), 3);
        assert!(!fixed.is_responsive());

        let responsive = SlidesQty::Responsive(ResponsiveSlidesQty::new(1).md(2));
        assert_eq!(responsive.get_for_width(400), 1);
        assert_eq!(responsive.get_for_width(800), 2);
        assert!(responsive.is_responsive());
    }

    #[test]
    fn test_loading_classes() {
        let loading = LoadingClasses::new()
            .remove("opacity-0")
            .add("opacity-100")
            .add_after("transition");

        assert!(loading.is_configured());
        assert_eq!(loading.remove, Some("opacity-0".to_string()));
        assert_eq!(loading.add, Some("opacity-100".to_string()));
        assert_eq!(loading.add_after, Some("transition".to_string()));

        let empty = LoadingClasses::default();
        assert!(!empty.is_configured());
    }
}
