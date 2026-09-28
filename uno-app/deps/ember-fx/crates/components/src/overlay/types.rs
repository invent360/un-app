//! Overlay component types.

/// Confirmation dialog type variants matching Ant Design.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConfirmType {
    /// Information dialog (blue icon).
    #[default]
    Info,
    /// Success dialog (green icon).
    Success,
    /// Warning dialog (orange icon).
    Warning,
    /// Error dialog (red icon).
    Error,
    /// Confirm dialog (orange icon, with cancel button).
    Confirm,
}

impl ConfirmType {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Confirm => "confirm",
        }
    }

    /// Returns the CSS class for this type.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-confirm-{}", prefix, self.as_suffix())
    }

    /// Returns the CSS class for the icon color.
    pub fn icon_class(&self, prefix: &str) -> String {
        format!("{}-confirm-icon-{}", prefix, self.as_suffix())
    }

    /// Whether this type shows a cancel button by default.
    pub fn has_cancel(&self) -> bool {
        matches!(self, Self::Confirm)
    }

    /// Returns the default OK button text for this type.
    pub fn default_ok_text(&self) -> &'static str {
        "OK"
    }

    /// Returns the default cancel button text for this type.
    pub fn default_cancel_text(&self) -> &'static str {
        "Cancel"
    }
}

/// Modal size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModalSize {
    /// Small modal (400px).
    Small,
    /// Default modal (520px).
    #[default]
    Default,
    /// Large modal (800px).
    Large,
    /// Extra large modal (1000px).
    ExtraLarge,
    /// Full screen modal.
    FullScreen,
}

impl ModalSize {
    /// Returns the width in pixels (None for fullscreen).
    pub fn width_px(&self) -> Option<u32> {
        match self {
            Self::Small => Some(400),
            Self::Default => Some(520),
            Self::Large => Some(800),
            Self::ExtraLarge => Some(1000),
            Self::FullScreen => None,
        }
    }

    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
            Self::ExtraLarge => "xl",
            Self::FullScreen => "fullscreen",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Animation type for modal transitions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModalAnimation {
    /// Zoom animation (default Ant Design).
    #[default]
    Zoom,
    /// Fade animation.
    Fade,
    /// Slide from top.
    SlideDown,
    /// No animation.
    None,
}

impl ModalAnimation {
    /// Returns the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Zoom => "zoom",
            Self::Fade => "fade",
            Self::SlideDown => "slide-down",
            Self::None => "none",
        }
    }

    /// Returns the full CSS class name.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

/// Configuration for confirm dialog buttons.
#[derive(Debug, Clone)]
pub struct ConfirmButtonConfig {
    /// OK button text.
    pub ok_text: String,
    /// Cancel button text.
    pub cancel_text: String,
    /// Whether the OK button is danger styled.
    pub ok_danger: bool,
    /// Whether the OK button shows loading state.
    pub ok_loading: bool,
}

impl Default for ConfirmButtonConfig {
    fn default() -> Self {
        Self {
            ok_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
            ok_danger: false,
            ok_loading: false,
        }
    }
}

/// Options for programmatic confirm dialogs.
#[derive(Debug, Clone, Default)]
pub struct ConfirmOptions {
    /// Dialog title.
    pub title: String,
    /// Dialog content/description.
    pub content: Option<String>,
    /// Custom icon (overrides type icon).
    pub icon: Option<String>,
    /// OK button text.
    pub ok_text: Option<String>,
    /// Cancel button text.
    pub cancel_text: Option<String>,
    /// Whether the OK button is danger styled.
    pub ok_danger: bool,
    /// Whether to center the modal vertically.
    pub centered: bool,
    /// Whether to show close button.
    pub closable: bool,
    /// Whether clicking mask closes the modal.
    pub mask_closable: bool,
    /// Custom width.
    pub width: Option<String>,
}

impl ConfirmOptions {
    /// Create new confirm options with a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the OK button text.
    pub fn ok_text(mut self, text: impl Into<String>) -> Self {
        self.ok_text = Some(text.into());
        self
    }

    /// Set the cancel button text.
    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = Some(text.into());
        self
    }

    /// Set whether the OK button is danger styled.
    pub fn ok_danger(mut self, danger: bool) -> Self {
        self.ok_danger = danger;
        self
    }

    /// Set whether the modal is centered.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set whether the modal is closable.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set custom width.
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
}
