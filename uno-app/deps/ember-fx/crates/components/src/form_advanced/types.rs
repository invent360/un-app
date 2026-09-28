//! Types for form-advanced components.

/// Slider size variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SliderSize {
    Small,
    #[default]
    Default,
}

impl SliderSize {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            SliderSize::Small => format!("{}-small", prefix),
            SliderSize::Default => String::new(),
        }
    }
}

/// Rate character type.
#[derive(Debug, Clone, Default)]
pub enum RateCharacter {
    #[default]
    Star,
    Heart,
    Custom(String),
}

impl RateCharacter {
    pub fn as_str(&self) -> &str {
        match self {
            RateCharacter::Star => "★",
            RateCharacter::Heart => "♥",
            RateCharacter::Custom(s) => s.as_str(),
        }
    }

    pub fn empty_str(&self) -> &str {
        match self {
            RateCharacter::Star => "☆",
            RateCharacter::Heart => "♡",
            RateCharacter::Custom(_) => "☆",
        }
    }
}

/// Date picker mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DatePickerMode {
    #[default]
    Date,
    Week,
    Month,
    Quarter,
    Year,
}

impl DatePickerMode {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            DatePickerMode::Date => format!("{}-date", prefix),
            DatePickerMode::Week => format!("{}-week", prefix),
            DatePickerMode::Month => format!("{}-month", prefix),
            DatePickerMode::Quarter => format!("{}-quarter", prefix),
            DatePickerMode::Year => format!("{}-year", prefix),
        }
    }
}

/// Date picker size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DatePickerSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl DatePickerSize {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            DatePickerSize::Small => format!("{}-small", prefix),
            DatePickerSize::Middle => String::new(),
            DatePickerSize::Large => format!("{}-large", prefix),
        }
    }
}

/// Time picker size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimePickerSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl TimePickerSize {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            TimePickerSize::Small => format!("{}-small", prefix),
            TimePickerSize::Middle => String::new(),
            TimePickerSize::Large => format!("{}-large", prefix),
        }
    }
}

/// Transfer item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferItem {
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl TransferItem {
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Transfer direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    Left,
    Right,
}

/// Upload list type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UploadListType {
    #[default]
    Text,
    Picture,
    PictureCard,
}

impl UploadListType {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            UploadListType::Text => format!("{}-text", prefix),
            UploadListType::Picture => format!("{}-picture", prefix),
            UploadListType::PictureCard => format!("{}-picture-card", prefix),
        }
    }
}

/// Upload file status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UploadFileStatus {
    #[default]
    Ready,
    Uploading,
    Done,
    Error,
    Removed,
}

impl UploadFileStatus {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            UploadFileStatus::Ready => String::new(),
            UploadFileStatus::Uploading => format!("{}-uploading", prefix),
            UploadFileStatus::Done => format!("{}-done", prefix),
            UploadFileStatus::Error => format!("{}-error", prefix),
            UploadFileStatus::Removed => format!("{}-removed", prefix),
        }
    }
}

/// Upload file item.
#[derive(Debug, Clone)]
pub struct UploadFile {
    pub uid: String,
    pub name: String,
    pub status: UploadFileStatus,
    pub percent: f64,
    pub url: Option<String>,
    pub thumb_url: Option<String>,
    pub size: Option<u64>,
    pub file_type: Option<String>,
    pub error: Option<String>,
}

impl UploadFile {
    pub fn new(uid: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uid: uid.into(),
            name: name.into(),
            status: UploadFileStatus::Ready,
            percent: 0.0,
            url: None,
            thumb_url: None,
            size: None,
            file_type: None,
            error: None,
        }
    }

    pub fn status(mut self, status: UploadFileStatus) -> Self {
        self.status = status;
        self
    }

    pub fn percent(mut self, percent: f64) -> Self {
        self.percent = percent;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn thumb_url(mut self, url: impl Into<String>) -> Self {
        self.thumb_url = Some(url.into());
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

/// Slider marks.
#[derive(Debug, Clone)]
pub struct SliderMark {
    pub value: f64,
    pub label: String,
}

impl SliderMark {
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

/// Color format for ColorPicker.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorFormat {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Rgba,
    Hsla,
}

impl ColorFormat {
    /// Get format label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Hex => "HEX",
            Self::Rgb => "RGB",
            Self::Hsl => "HSL",
            Self::Rgba => "RGBA",
            Self::Hsla => "HSLA",
        }
    }

    /// Check if format supports alpha.
    pub fn has_alpha(&self) -> bool {
        matches!(self, Self::Rgba | Self::Hsla)
    }
}

impl std::fmt::Display for ColorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_label())
    }
}

/// Color picker size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColorPickerSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl ColorPickerSize {
    pub fn class(&self, prefix: &str) -> String {
        match self {
            ColorPickerSize::Small => format!("{}-small", prefix),
            ColorPickerSize::Middle => String::new(),
            ColorPickerSize::Large => format!("{}-large", prefix),
        }
    }
}

/// Color preset group.
#[derive(Debug, Clone)]
pub struct ColorPreset {
    /// Group label.
    pub label: Option<String>,
    /// Preset colors.
    pub colors: Vec<String>,
}

impl ColorPreset {
    /// Create a new preset with colors.
    pub fn new(colors: Vec<impl Into<String>>) -> Self {
        Self {
            label: None,
            colors: colors.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Create a preset with a label.
    pub fn labeled(label: impl Into<String>, colors: Vec<impl Into<String>>) -> Self {
        Self {
            label: Some(label.into()),
            colors: colors.into_iter().map(|c| c.into()).collect(),
        }
    }

    /// Create default preset colors.
    pub fn default_colors() -> Self {
        Self::new(vec![
            "#F5222D", "#FA541C", "#FA8C16", "#FAAD14", "#FADB14",
            "#A0D911", "#52C41A", "#13C2C2", "#1890FF", "#2F54EB",
            "#722ED1", "#EB2F96", "#F5F5F5", "#D9D9D9", "#BFBFBF",
            "#8C8C8C", "#595959", "#262626", "#000000", "#FFFFFF",
        ])
    }

    /// Create material design preset.
    pub fn material_colors() -> Self {
        Self::labeled("Material", vec![
            "#F44336", "#E91E63", "#9C27B0", "#673AB7", "#3F51B5",
            "#2196F3", "#03A9F4", "#00BCD4", "#009688", "#4CAF50",
            "#8BC34A", "#CDDC39", "#FFEB3B", "#FFC107", "#FF9800",
            "#FF5722", "#795548", "#9E9E9E", "#607D8B",
        ])
    }
}

/// Color value with components.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorValue {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0.0-1.0).
    pub a: f32,
}

impl ColorValue {
    /// Create a new color from RGB.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new color from RGBA.
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parse from hex string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a as f32 / 255.0))
            }
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            _ => None,
        }
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to hex string with alpha.
    pub fn to_hex_alpha(&self) -> String {
        let a = (self.a * 255.0) as u8;
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, a)
    }

    /// Convert to rgb() string.
    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// Convert to rgba() string.
    pub fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
    }

    /// Convert to HSL components.
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            return (0.0, 0.0, l);
        }

        let d = max - min;
        let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

        let h = if max == r {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if max == g {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };

        (h * 360.0, s * 100.0, l * 100.0)
    }

    /// Convert to hsl() string.
    pub fn to_hsl_string(&self) -> String {
        let (h, s, l) = self.to_hsl();
        format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s, l)
    }

    /// Convert to hsla() string.
    pub fn to_hsla_string(&self) -> String {
        let (h, s, l) = self.to_hsl();
        format!("hsla({:.0}, {:.0}%, {:.0}%, {:.2})", h, s, l, self.a)
    }

    /// Format as specified format.
    pub fn format(&self, fmt: ColorFormat) -> String {
        match fmt {
            ColorFormat::Hex => self.to_hex(),
            ColorFormat::Rgb => self.to_rgb(),
            ColorFormat::Hsl => self.to_hsl_string(),
            ColorFormat::Rgba => self.to_rgba(),
            ColorFormat::Hsla => self.to_hsla_string(),
        }
    }
}
