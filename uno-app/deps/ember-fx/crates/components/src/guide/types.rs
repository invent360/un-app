//! Type definitions for Guide components.

use serde::{Deserialize, Serialize};

/// Guide display variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideVariant {
    /// Full-screen/modal carousel (immersive)
    #[default]
    Carousel,
    /// Side panel drawer (non-blocking)
    Drawer,
    /// Inline accordion (embedded)
    Accordion,
    /// Spotlight tour (contextual)
    Tour,
}

impl GuideVariant {
    /// Get the CSS class suffix for this variant.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            GuideVariant::Carousel => "carousel",
            GuideVariant::Drawer => "drawer",
            GuideVariant::Accordion => "accordion",
            GuideVariant::Tour => "tour",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Guide component size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideSize {
    /// Compact size
    Small,
    /// Default size
    #[default]
    Medium,
    /// Large/expanded size
    Large,
    /// Full screen (mobile/immersive)
    FullScreen,
}

impl GuideSize {
    /// Get the CSS class suffix for this size.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            GuideSize::Small => "sm",
            GuideSize::Medium => "md",
            GuideSize::Large => "lg",
            GuideSize::FullScreen => "fullscreen",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Drawer position (for GuideDrawer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPosition {
    /// Slide in from right
    #[default]
    Right,
    /// Slide in from left
    Left,
    /// Slide in from bottom (mobile)
    Bottom,
}

impl DrawerPosition {
    /// Get the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            DrawerPosition::Right => "right",
            DrawerPosition::Left => "left",
            DrawerPosition::Bottom => "bottom",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Progress indicator style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressStyle {
    /// Dot indicators
    #[default]
    Dots,
    /// Progress bar
    Bar,
    /// Step numbers
    Numbers,
    /// Step list (vertical)
    Steps,
    /// Hidden
    None,
}

impl ProgressStyle {
    /// Get the CSS class suffix for this style.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            ProgressStyle::Dots => "dots",
            ProgressStyle::Bar => "bar",
            ProgressStyle::Numbers => "numbers",
            ProgressStyle::Steps => "steps",
            ProgressStyle::None => "none",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-progress-{}", prefix, self.as_suffix())
    }
}

/// Transition effect between steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideTransition {
    /// Slide transition
    #[default]
    Slide,
    /// Fade transition
    Fade,
    /// No transition
    None,
}

impl GuideTransition {
    /// Get the CSS class suffix for this transition.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            GuideTransition::Slide => "slide",
            GuideTransition::Fade => "fade",
            GuideTransition::None => "none",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-transition-{}", prefix, self.as_suffix())
    }
}

/// A single step in a guide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideStep {
    /// Unique identifier for this step.
    pub id: String,
    /// Step order (1-based).
    pub order: i32,
    /// Step title.
    pub title: String,
    /// Step description (supports markdown).
    pub description: String,
    /// Optional image URL.
    #[serde(default)]
    pub image: Option<String>,
    /// Optional video URL.
    #[serde(default)]
    pub video: Option<String>,
    /// Whether this step is completed.
    #[serde(default)]
    pub completed: bool,
    /// Target element selector (for tour mode).
    #[serde(default)]
    pub target: Option<String>,
    /// Tooltip placement (for tour mode).
    #[serde(default)]
    pub placement: Option<String>,
}

impl GuideStep {
    /// Create a new guide step.
    pub fn new(id: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            order: 0,
            title: title.into(),
            description: description.into(),
            image: None,
            video: None,
            completed: false,
            target: None,
            placement: None,
        }
    }

    /// Set the step order.
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    /// Set an image for this step.
    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.image = Some(url.into());
        self
    }

    /// Set a video for this step.
    pub fn video(mut self, url: impl Into<String>) -> Self {
        self.video = Some(url.into());
        self
    }

    /// Mark this step as completed.
    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = completed;
        self
    }

    /// Set target element selector (for tour mode).
    pub fn target(mut self, selector: impl Into<String>) -> Self {
        self.target = Some(selector.into());
        self
    }

    /// Set tooltip placement (for tour mode).
    pub fn placement(mut self, placement: impl Into<String>) -> Self {
        self.placement = Some(placement.into());
        self
    }
}

/// Guide content metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GuideMetadata {
    /// Guide title.
    pub title: String,
    /// Guide description/subtitle.
    #[serde(default)]
    pub description: Option<String>,
    /// Difficulty level (easy, medium, hard).
    #[serde(default)]
    pub difficulty: Option<String>,
    /// Estimated duration in minutes.
    #[serde(default)]
    pub duration_minutes: Option<i32>,
    /// Thumbnail image URL.
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Category/tag.
    #[serde(default)]
    pub category: Option<String>,
}

impl GuideMetadata {
    /// Create new guide metadata.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set difficulty.
    pub fn difficulty(mut self, diff: impl Into<String>) -> Self {
        self.difficulty = Some(diff.into());
        self
    }

    /// Set duration in minutes.
    pub fn duration(mut self, minutes: i32) -> Self {
        self.duration_minutes = Some(minutes);
        self
    }

    /// Set thumbnail.
    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        self.thumbnail = Some(url.into());
        self
    }
}

/// Spotlight/tour step placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TourPlacement {
    /// Above the target
    Top,
    /// Below the target
    #[default]
    Bottom,
    /// Left of the target
    Left,
    /// Right of the target
    Right,
    /// Centered in viewport
    Center,
}

impl TourPlacement {
    /// Get the CSS class suffix for this placement.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            TourPlacement::Top => "top",
            TourPlacement::Bottom => "bottom",
            TourPlacement::Left => "left",
            TourPlacement::Right => "right",
            TourPlacement::Center => "center",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-placement-{}", prefix, self.as_suffix())
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top" => TourPlacement::Top,
            "bottom" => TourPlacement::Bottom,
            "left" => TourPlacement::Left,
            "right" => TourPlacement::Right,
            "center" => TourPlacement::Center,
            _ => TourPlacement::Bottom,
        }
    }
}

/// Stepper orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperOrientation {
    /// Horizontal layout (steps in a row)
    #[default]
    Horizontal,
    /// Vertical layout (steps stacked)
    Vertical,
}

impl StepperOrientation {
    /// Get the CSS class suffix for this orientation.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            StepperOrientation::Horizontal => "horizontal",
            StepperOrientation::Vertical => "vertical",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-{}", prefix, self.as_suffix())
    }
}

/// Position of step header/title relative to the step number (for horizontal stepper).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperHeaderPosition {
    /// Title above the number
    Top,
    /// Title to the right of the number (default)
    #[default]
    Right,
    /// Title below the number
    Bottom,
    /// Title to the left of the number
    Left,
}

impl StepperHeaderPosition {
    /// Get the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            StepperHeaderPosition::Top => "top",
            StepperHeaderPosition::Right => "right",
            StepperHeaderPosition::Bottom => "bottom",
            StepperHeaderPosition::Left => "left",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-header-{}", prefix, self.as_suffix())
    }
}

/// Step status for visual state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    /// Step not yet reached
    #[default]
    Pending,
    /// Currently active step
    Active,
    /// Step completed
    Completed,
    /// Step has an error
    Error,
}

/// Step layout mode controlling image and content arrangement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepLayout {
    /// Side by side layout: smaller image on left/right, content beside it.
    /// Best for mobile app screenshots (portrait orientation).
    #[default]
    SideBySide,
    /// Stacked layout: larger image on top, content below.
    /// Best for desktop screenshots or landscape images.
    Stacked,
}

impl StepLayout {
    /// Get the CSS class suffix for this layout.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            StepLayout::SideBySide => "side-by-side",
            StepLayout::Stacked => "stacked",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-layout-{}", prefix, self.as_suffix())
    }
}

/// Position of navigation controls (Back/Next buttons).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlsPosition {
    /// Controls at the bottom center (default)
    #[default]
    Bottom,
    /// Controls at the top right
    TopRight,
}

impl ControlsPosition {
    /// Get the CSS class suffix for this position.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            ControlsPosition::Bottom => "bottom",
            ControlsPosition::TopRight => "top-right",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-controls-{}", prefix, self.as_suffix())
    }
}

/// Header layout mode for horizontal stepper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperHeaderLayout {
    /// Standard layout: steps above content, controls in content or top-right
    #[default]
    Standard,
    /// Split layout: steps left with pagination, controls right in unified header row
    Split,
}

impl StepperHeaderLayout {
    /// Get the CSS class suffix for this layout.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            StepperHeaderLayout::Standard => "standard",
            StepperHeaderLayout::Split => "split",
        }
    }

    /// Build the full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-stepper-header-{}", prefix, self.as_suffix())
    }
}
