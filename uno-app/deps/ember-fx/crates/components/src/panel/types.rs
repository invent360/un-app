//! Type definitions for StatCard components.

use std::fmt;

// Re-export ChartColor for convenience
#[cfg(feature = "chart")]
pub use crate::chart::ChartColor;

/// Icon types for StatCard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatIcon {
    /// Heart icon - for health/wellness metrics
    Heart,
    /// Chart/trending icon - for analytics
    #[default]
    Chart,
    /// Bar chart icon - for comparison metrics
    BarChart,
    /// Users icon - for user-related metrics
    Users,
    /// Dollar/money icon - for financial metrics
    Dollar,
    /// Globe icon - for global/geographic metrics
    Globe,
    /// Shopping cart icon - for e-commerce
    Cart,
    /// Clock icon - for time-based metrics
    Clock,
    /// Star icon - for ratings/favorites
    Star,
    /// Lightning bolt - for performance metrics
    Bolt,
}

impl StatIcon {
    /// Get SVG path for the icon.
    pub fn svg_path(&self) -> &'static str {
        match self {
            Self::Heart => "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z",
            Self::Chart => "M3.5 18.49l6-6.01 4 4L22 6.92l-1.41-1.41-7.09 7.97-4-4L2 16.99z",
            Self::BarChart => "M4 9h4v11H4zm6-5h4v16h-4zm6 8h4v8h-4z",
            Self::Users => "M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z",
            Self::Dollar => "M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z",
            Self::Globe => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z",
            Self::Cart => "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H5.21l-.94-2H1zm16 16c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
            Self::Clock => "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm.5-13H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            Self::Star => "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            Self::Bolt => "M7 2v11h3v9l7-12h-4l4-8z",
        }
    }

    /// Get default background color for the icon.
    pub fn default_color(&self) -> &'static str {
        match self {
            Self::Heart => "#eb2f96",       // Magenta
            Self::Chart => "#1890ff",       // Blue
            Self::BarChart => "#722ed1",    // Purple
            Self::Users => "#13c2c2",       // Cyan
            Self::Dollar => "#52c41a",      // Green
            Self::Globe => "#fa8c16",       // Orange
            Self::Cart => "#fa541c",        // Red-Orange
            Self::Clock => "#2f54eb",       // Geek Blue
            Self::Star => "#faad14",        // Gold
            Self::Bolt => "#a0d911",        // Lime
        }
    }
}

/// Trend direction flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrendFlag {
    /// Upward trend (typically shown in red for costs, green for revenue)
    Up,
    /// Downward trend
    Down,
    /// No change / flat
    #[default]
    Flat,
}

impl TrendFlag {
    /// Get CSS class suffix.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Flat => "flat",
        }
    }

    /// Get the arrow/icon character.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Up => "▲",
            Self::Down => "▼",
            Self::Flat => "―",
        }
    }
}

impl fmt::Display for TrendFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trend color mode.
///
/// Different contexts interpret up/down trends differently:
/// - Revenue: Up is good (green), Down is bad (red)
/// - Costs: Up is bad (red), Down is good (green)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrendColorMode {
    /// Standard: Up = green, Down = red (for revenue, users, etc.)
    #[default]
    Standard,
    /// Reversed: Up = red, Down = green (for costs, errors, etc.)
    Reversed,
    /// Neutral: Both use neutral color
    Neutral,
}

impl TrendColorMode {
    /// Get the color class for a trend flag.
    pub fn color_class(&self, flag: TrendFlag) -> &'static str {
        match (self, flag) {
            (Self::Standard, TrendFlag::Up) => "fx-trend-color-success",
            (Self::Standard, TrendFlag::Down) => "fx-trend-color-error",
            (Self::Reversed, TrendFlag::Up) => "fx-trend-color-error",
            (Self::Reversed, TrendFlag::Down) => "fx-trend-color-success",
            (_, TrendFlag::Flat) | (Self::Neutral, _) => "fx-trend-color-neutral",
        }
    }
}

/// StatCard size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatCardSize {
    /// Small card
    Small,
    /// Default size
    #[default]
    Default,
    /// Large card
    Large,
}

impl StatCardSize {
    /// Get CSS class suffix.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "md",
            Self::Large => "lg",
        }
    }

    /// Get value font size class.
    pub fn value_class(&self) -> &'static str {
        match self {
            Self::Small => "fx-statcard-value-sm",
            Self::Default => "fx-statcard-value-md",
            Self::Large => "fx-statcard-value-lg",
        }
    }
}

// ============================================
// Accordion Types
// ============================================

/// Accordion item data.
#[derive(Clone, Debug, PartialEq)]
pub struct AccordionItem {
    /// Unique identifier for this item.
    pub id: String,
    /// Title displayed in the header.
    pub title: String,
    /// Content displayed when expanded.
    pub content: String,
}

impl AccordionItem {
    /// Create a new accordion item.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        }
    }
}

/// Accordion size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionSize {
    /// Small accordion with compact padding
    Small,
    /// Default size
    #[default]
    Default,
    /// Large accordion with more padding
    Large,
}

impl AccordionSize {
    /// Get CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "sm",
            Self::Default => "default",
            Self::Large => "lg",
        }
    }

    /// Get full CSS class with prefix.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for AccordionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_flag() {
        assert_eq!(TrendFlag::Up.icon(), "▲");
        assert_eq!(TrendFlag::Down.icon(), "▼");
        assert_eq!(TrendFlag::Up.as_str(), "up");
    }

    #[test]
    fn test_trend_color_mode() {
        // Standard mode: up is good
        assert_eq!(
            TrendColorMode::Standard.color_class(TrendFlag::Up),
            "fx-trend-color-success"
        );
        assert_eq!(
            TrendColorMode::Standard.color_class(TrendFlag::Down),
            "fx-trend-color-error"
        );

        // Reversed mode: up is bad (for costs)
        assert_eq!(
            TrendColorMode::Reversed.color_class(TrendFlag::Up),
            "fx-trend-color-error"
        );
        assert_eq!(
            TrendColorMode::Reversed.color_class(TrendFlag::Down),
            "fx-trend-color-success"
        );
    }

    #[test]
    fn test_statcard_size() {
        assert_eq!(StatCardSize::default(), StatCardSize::Default);
        assert_eq!(StatCardSize::Large.as_str(), "lg");
    }

    #[test]
    fn test_accordion_item() {
        let item = AccordionItem::new("1", "Title", "Content");
        assert_eq!(item.id, "1");
        assert_eq!(item.title, "Title");
        assert_eq!(item.content, "Content");
    }

    #[test]
    fn test_accordion_size() {
        assert_eq!(AccordionSize::default(), AccordionSize::Default);
        assert_eq!(AccordionSize::Small.as_suffix(), "sm");
        assert_eq!(AccordionSize::Large.class("fx-accordion-ant"), "fx-accordion-ant-lg");
    }
}
