//! Navigation component types.

/// Menu mode variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MenuMode {
    /// Vertical menu (default).
    #[default]
    Vertical,
    /// Horizontal menu.
    Horizontal,
    /// Inline menu (collapsible).
    Inline,
}

impl MenuMode {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Inline => "inline",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Menu theme variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MenuTheme {
    /// Light theme.
    Light,
    /// Dark theme (default).
    #[default]
    Dark,
}

impl MenuTheme {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Menu item configuration.
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Unique key for the item.
    pub key: String,
    /// Item label.
    pub label: String,
    /// Icon for the item.
    pub icon: Option<String>,
    /// Whether the item is disabled.
    pub disabled: bool,
    /// Sub-menu items.
    pub children: Vec<MenuItem>,
    /// Danger/destructive style.
    pub danger: bool,
}

impl MenuItem {
    /// Create a new menu item.
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            children: Vec::new(),
            danger: false,
        }
    }

    /// Set the icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set whether the item is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set danger style.
    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    /// Add child items (creates submenu).
    pub fn children(mut self, children: Vec<MenuItem>) -> Self {
        self.children = children;
        self
    }
}

/// Breadcrumb separator style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    /// Slash separator (default).
    #[default]
    Slash,
    /// Arrow separator.
    Arrow,
    /// Custom separator (use separator prop).
    Custom,
}

impl BreadcrumbSeparator {
    /// Returns the separator character.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Slash => "/",
            Self::Arrow => ">",
            Self::Custom => "",
        }
    }
}

/// Breadcrumb item configuration.
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    /// Item label.
    pub label: String,
    /// Link href.
    pub href: Option<String>,
    /// Icon for the item.
    pub icon: Option<String>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
        }
    }

    /// Set the href link.
    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Set the icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Pagination size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PaginationSize {
    /// Small pagination.
    Small,
    /// Default size.
    #[default]
    Default,
}

impl PaginationSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Steps size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsSize {
    /// Default size.
    #[default]
    Default,
    /// Small size.
    Small,
}

impl StepsSize {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Small => "small",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Steps direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsDirection {
    /// Horizontal steps (default).
    #[default]
    Horizontal,
    /// Vertical steps.
    Vertical,
}

impl StepsDirection {
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

/// Steps type/style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsType {
    /// Default style.
    #[default]
    Default,
    /// Navigation style.
    Navigation,
    /// Inline style.
    Inline,
}

impl StepsType {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Navigation => "navigation",
            Self::Inline => "inline",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Steps icon style variant.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsIconType {
    /// Default filled style - circle filled with color.
    #[default]
    Default,
    /// Outlined style - ring outline for current step.
    Outlined,
    /// Dot style - ring outline with small dot indicator.
    Dot,
}

impl StepsIconType {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Default => "icon-default",
            Self::Outlined => "icon-outlined",
            Self::Dot => "icon-dot",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Steps connector line weight.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsLineWeight {
    /// Thin line (1px).
    Thin,
    /// Default line weight (1px).
    #[default]
    Default,
    /// Medium line (2px).
    Medium,
    /// Thick line (3px).
    Thick,
}

impl StepsLineWeight {
    /// Returns the line thickness in pixels.
    pub fn as_px(&self) -> u8 {
        match self {
            Self::Thin => 1,
            Self::Default => 1,
            Self::Medium => 2,
            Self::Thick => 3,
        }
    }
}

/// Steps label placement.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsLabelPlacement {
    /// Labels placed horizontally next to icon (default).
    #[default]
    Horizontal,
    /// Labels placed vertically below icon.
    Vertical,
}

impl StepsLabelPlacement {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Horizontal => "label-horizontal",
            Self::Vertical => "label-vertical",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Step status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepStatus {
    /// Waiting status.
    #[default]
    Wait,
    /// In progress status.
    Process,
    /// Finished status.
    Finish,
    /// Error status.
    Error,
}

impl StepStatus {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Wait => "wait",
            Self::Process => "process",
            Self::Finish => "finish",
            Self::Error => "error",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Steps responsive behavior on mobile.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum StepsResponsive {
    /// Compress steps to fit screen (default).
    #[default]
    Compress,
    /// Enable horizontal scrolling.
    Scroll,
    /// Hide on mobile (show only current step indicator).
    Hide,
}

impl StepsResponsive {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Compress => "responsive-compress",
            Self::Scroll => "responsive-scroll",
            Self::Hide => "responsive-hide",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Step item configuration.
#[derive(Debug, Clone)]
pub struct StepItem {
    /// Step title.
    pub title: String,
    /// Step description.
    pub description: Option<String>,
    /// Step subtitle.
    pub subtitle: Option<String>,
    /// Step icon.
    pub icon: Option<String>,
    /// Step status (overrides auto-calculation).
    pub status: Option<StepStatus>,
    /// Whether the step is disabled.
    pub disabled: bool,
}

impl StepItem {
    /// Create a new step item.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            subtitle: None,
            icon: None,
            status: None,
            disabled: false,
        }
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the subtitle.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set the icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the status.
    pub fn status(mut self, status: StepStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Set whether the step is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
