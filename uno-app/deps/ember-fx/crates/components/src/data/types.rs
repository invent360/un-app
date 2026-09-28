//! Data display component types.

/// Avatar size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AvatarSize {
    /// Small avatar (24px).
    Small,
    /// Default size (32px).
    #[default]
    Default,
    /// Large avatar (40px).
    Large,
    /// Custom size (specify in pixels).
    Custom(u32),
}

impl AvatarSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }

    /// Returns the size in pixels.
    pub fn pixels(&self) -> u32 {
        match self {
            Self::Small => 24,
            Self::Default => 32,
            Self::Large => 40,
            Self::Custom(size) => *size,
        }
    }
}

/// Avatar shape variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AvatarShape {
    /// Circle shape (default).
    #[default]
    Circle,
    /// Square shape with rounded corners.
    Square,
}

impl AvatarShape {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Circle => "circle",
            Self::Square => "square",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Tooltip placement options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipPlacement {
    /// Top placement (default).
    #[default]
    Top,
    /// Top left placement.
    TopLeft,
    /// Top right placement.
    TopRight,
    /// Bottom placement.
    Bottom,
    /// Bottom left placement.
    BottomLeft,
    /// Bottom right placement.
    BottomRight,
    /// Left placement.
    Left,
    /// Left top placement.
    LeftTop,
    /// Left bottom placement.
    LeftBottom,
    /// Right placement.
    Right,
    /// Right top placement.
    RightTop,
    /// Right bottom placement.
    RightBottom,
}

impl TooltipPlacement {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::TopLeft => "top-left",
            Self::TopRight => "top-right",
            Self::Bottom => "bottom",
            Self::BottomLeft => "bottom-left",
            Self::BottomRight => "bottom-right",
            Self::Left => "left",
            Self::LeftTop => "left-top",
            Self::LeftBottom => "left-bottom",
            Self::Right => "right",
            Self::RightTop => "right-top",
            Self::RightBottom => "right-bottom",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Tooltip trigger options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipTrigger {
    /// Hover trigger (default).
    #[default]
    Hover,
    /// Focus trigger.
    Focus,
    /// Click trigger.
    Click,
}

/// Popover trigger options.
pub type PopoverTrigger = TooltipTrigger;

/// Popover placement options.
pub type PopoverPlacement = TooltipPlacement;

/// List size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ListSize {
    /// Small list.
    Small,
    /// Default size.
    #[default]
    Default,
    /// Large list.
    Large,
}

impl ListSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// List item layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ListLayout {
    /// Horizontal layout.
    Horizontal,
    /// Vertical layout (default).
    #[default]
    Vertical,
}

impl ListLayout {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Table size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TableSize {
    /// Small table.
    Small,
    /// Default size.
    #[default]
    Default,
    /// Large table.
    Large,
}

impl TableSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Table column alignment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColumnAlign {
    /// Left alignment (default).
    #[default]
    Left,
    /// Center alignment.
    Center,
    /// Right alignment.
    Right,
}

impl ColumnAlign {
    /// Returns the CSS text-align value.
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// Sort order for table columns.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortOrder {
    /// No sorting.
    #[default]
    None,
    /// Ascending order.
    Ascend,
    /// Descending order.
    Descend,
}

impl SortOrder {
    /// Returns the next sort order in the cycle.
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Ascend,
            Self::Ascend => Self::Descend,
            Self::Descend => Self::None,
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ascend => "ascend",
            Self::Descend => "descend",
        }
    }
}

/// Table column definition.
#[derive(Debug, Clone)]
pub struct TableColumn {
    /// Unique key for the column.
    pub key: String,
    /// Column title.
    pub title: String,
    /// Data index (field name in row data).
    pub data_index: String,
    /// Column width (CSS value).
    pub width: Option<String>,
    /// Column alignment.
    pub align: ColumnAlign,
    /// Whether column is sortable.
    pub sortable: bool,
    /// Whether column is filterable.
    pub filterable: bool,
    /// Fixed column position.
    pub fixed: Option<ColumnFixed>,
    /// Whether column is hidden.
    pub hidden: bool,
}

impl TableColumn {
    /// Create a new table column.
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        let key_str = key.into();
        Self {
            key: key_str.clone(),
            title: title.into(),
            data_index: key_str,
            width: None,
            align: ColumnAlign::default(),
            sortable: false,
            filterable: false,
            fixed: None,
            hidden: false,
        }
    }

    /// Set the data index.
    pub fn data_index(mut self, data_index: impl Into<String>) -> Self {
        self.data_index = data_index.into();
        self
    }

    /// Set the width.
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set the alignment.
    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }

    /// Set whether sortable.
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Set whether filterable.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Set fixed position.
    pub fn fixed(mut self, fixed: ColumnFixed) -> Self {
        self.fixed = Some(fixed);
        self
    }

    /// Set whether hidden.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}

/// Fixed column position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnFixed {
    /// Fixed to left.
    Left,
    /// Fixed to right.
    Right,
}

impl ColumnFixed {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Left => "fixed-left",
            Self::Right => "fixed-right",
        }
    }
}
