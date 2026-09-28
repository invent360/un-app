//! Type definitions for Slider components.

/// Slider size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SliderSize {
    /// Small slider (2px rail, 10px handle)
    Small,
    /// Default slider (4px rail, 14px handle)
    #[default]
    Default,
    /// Large slider (6px rail, 18px handle)
    Large,
}

impl SliderSize {
    /// Get CSS class suffix for this size.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            SliderSize::Small => format!("{}-small", prefix),
            SliderSize::Default => String::new(),
            SliderSize::Large => format!("{}-large", prefix),
        }
    }
}

/// Slider orientation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Horizontal slider (default)
    #[default]
    Horizontal,
    /// Vertical slider
    Vertical,
}

impl SliderOrientation {
    /// Check if orientation is vertical.
    pub fn is_vertical(&self) -> bool {
        matches!(self, SliderOrientation::Vertical)
    }

    /// Get CSS class suffix.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            SliderOrientation::Horizontal => String::new(),
            SliderOrientation::Vertical => format!("{}-vertical", prefix),
        }
    }
}

/// Tooltip placement relative to handle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    /// Above the handle (default for horizontal)
    #[default]
    Top,
    /// Below the handle
    Bottom,
    /// Left of the handle (default for vertical RTL)
    Left,
    /// Right of the handle (default for vertical LTR)
    Right,
}

impl TooltipPlacement {
    /// Get CSS class suffix.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            TooltipPlacement::Top => format!("{}-top", prefix),
            TooltipPlacement::Bottom => format!("{}-bottom", prefix),
            TooltipPlacement::Left => format!("{}-left", prefix),
            TooltipPlacement::Right => format!("{}-right", prefix),
        }
    }

    /// Get default placement for orientation.
    pub fn default_for_orientation(orientation: SliderOrientation) -> Self {
        match orientation {
            SliderOrientation::Horizontal => TooltipPlacement::Top,
            SliderOrientation::Vertical => TooltipPlacement::Right,
        }
    }
}

/// Tooltip visibility mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipVisibility {
    /// Always show tooltip
    Always,
    /// Show on hover/focus/drag (default)
    #[default]
    Hover,
    /// Never show tooltip
    Never,
}

/// Tooltip configuration.
#[derive(Debug, Clone, Default)]
pub struct TooltipConfig {
    /// Visibility mode
    pub visible: TooltipVisibility,
    /// Placement relative to handle
    pub placement: Option<TooltipPlacement>,
    /// Offset from handle in pixels
    pub offset: Option<i32>,
}

impl TooltipConfig {
    /// Create a new tooltip config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set visibility mode.
    pub fn visible(mut self, visible: TooltipVisibility) -> Self {
        self.visible = visible;
        self
    }

    /// Set placement.
    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = Some(placement);
        self
    }

    /// Set offset.
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Always visible tooltip.
    pub fn always() -> Self {
        Self {
            visible: TooltipVisibility::Always,
            ..Default::default()
        }
    }

    /// Never show tooltip.
    pub fn never() -> Self {
        Self {
            visible: TooltipVisibility::Never,
            ..Default::default()
        }
    }
}

/// Range slider configuration.
#[derive(Debug, Clone, Default)]
pub struct RangeConfig {
    /// Allow dragging the track between handles to move both.
    pub draggable_track: bool,
    /// Minimum gap between handles.
    pub min_range: Option<f64>,
    /// Maximum gap between handles.
    pub max_range: Option<f64>,
    /// Push handles when they collide instead of blocking.
    pub pushable: bool,
}

impl RangeConfig {
    /// Create a new range config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable draggable track.
    pub fn draggable_track(mut self, enabled: bool) -> Self {
        self.draggable_track = enabled;
        self
    }

    /// Set minimum range between handles.
    pub fn min_range(mut self, min: f64) -> Self {
        self.min_range = Some(min);
        self
    }

    /// Set maximum range between handles.
    pub fn max_range(mut self, max: f64) -> Self {
        self.max_range = Some(max);
        self
    }

    /// Enable pushable handles.
    pub fn pushable(mut self, enabled: bool) -> Self {
        self.pushable = enabled;
        self
    }
}

/// Slider mark point with label.
#[derive(Debug, Clone)]
pub struct SliderMark {
    /// Value position for the mark.
    pub value: f64,
    /// Label text to display.
    pub label: String,
    /// Optional custom CSS style.
    pub style: Option<String>,
    /// Optional custom CSS class.
    pub class: Option<String>,
}

impl SliderMark {
    /// Create a new mark at the given value with a label.
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            style: None,
            class: None,
        }
    }

    /// Add custom style to the mark.
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Add custom class to the mark.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

/// Semantic CSS class name overrides for slider parts.
#[derive(Debug, Clone, Default)]
pub struct SliderClassNames {
    /// Root container class.
    pub root: Option<String>,
    /// Rail (background track) class.
    pub rail: Option<String>,
    /// Track (filled portion) class.
    pub track: Option<String>,
    /// Handle (thumb) class.
    pub handle: Option<String>,
    /// Marks container class.
    pub marks: Option<String>,
    /// Individual mark class.
    pub mark: Option<String>,
    /// Step dot class.
    pub dot: Option<String>,
    /// Tooltip class.
    pub tooltip: Option<String>,
}

impl SliderClassNames {
    /// Create empty class names.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set root class.
    pub fn root(mut self, class: impl Into<String>) -> Self {
        self.root = Some(class.into());
        self
    }

    /// Set rail class.
    pub fn rail(mut self, class: impl Into<String>) -> Self {
        self.rail = Some(class.into());
        self
    }

    /// Set track class.
    pub fn track(mut self, class: impl Into<String>) -> Self {
        self.track = Some(class.into());
        self
    }

    /// Set handle class.
    pub fn handle(mut self, class: impl Into<String>) -> Self {
        self.handle = Some(class.into());
        self
    }

    /// Set marks container class.
    pub fn marks(mut self, class: impl Into<String>) -> Self {
        self.marks = Some(class.into());
        self
    }

    /// Set individual mark class.
    pub fn mark(mut self, class: impl Into<String>) -> Self {
        self.mark = Some(class.into());
        self
    }

    /// Set dot class.
    pub fn dot(mut self, class: impl Into<String>) -> Self {
        self.dot = Some(class.into());
        self
    }

    /// Set tooltip class.
    pub fn tooltip(mut self, class: impl Into<String>) -> Self {
        self.tooltip = Some(class.into());
        self
    }
}

/// Semantic inline style overrides for slider parts.
#[derive(Debug, Clone, Default)]
pub struct SliderStyles {
    /// Root container style.
    pub root: Option<String>,
    /// Rail (background track) style.
    pub rail: Option<String>,
    /// Track (filled portion) style.
    pub track: Option<String>,
    /// Handle (thumb) style.
    pub handle: Option<String>,
    /// Marks container style.
    pub marks: Option<String>,
    /// Individual mark style.
    pub mark: Option<String>,
    /// Step dot style.
    pub dot: Option<String>,
    /// Tooltip style.
    pub tooltip: Option<String>,
}

impl SliderStyles {
    /// Create empty styles.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set root style.
    pub fn root(mut self, style: impl Into<String>) -> Self {
        self.root = Some(style.into());
        self
    }

    /// Set rail style.
    pub fn rail(mut self, style: impl Into<String>) -> Self {
        self.rail = Some(style.into());
        self
    }

    /// Set track style.
    pub fn track(mut self, style: impl Into<String>) -> Self {
        self.track = Some(style.into());
        self
    }

    /// Set handle style.
    pub fn handle(mut self, style: impl Into<String>) -> Self {
        self.handle = Some(style.into());
        self
    }

    /// Set marks container style.
    pub fn marks(mut self, style: impl Into<String>) -> Self {
        self.marks = Some(style.into());
        self
    }

    /// Set individual mark style.
    pub fn mark(mut self, style: impl Into<String>) -> Self {
        self.mark = Some(style.into());
        self
    }

    /// Set dot style.
    pub fn dot(mut self, style: impl Into<String>) -> Self {
        self.dot = Some(style.into());
        self
    }

    /// Set tooltip style.
    pub fn tooltip(mut self, style: impl Into<String>) -> Self {
        self.tooltip = Some(style.into());
        self
    }
}
