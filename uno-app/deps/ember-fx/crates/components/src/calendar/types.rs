//! Calendar component type definitions.
//!
//! Types for date/time picker components.

use std::fmt;

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
    /// Get the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Week => "week",
            Self::Month => "month",
            Self::Quarter => "quarter",
            Self::Year => "year",
        }
    }

    /// Get the full CSS class.
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl fmt::Display for DatePickerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
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
    /// Get the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Middle => "middle",
            Self::Large => "large",
        }
    }

    /// Get the full CSS class.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            Self::Middle => String::new(),
            _ => format!("{}-{}", prefix, self.as_suffix()),
        }
    }
}

impl fmt::Display for DatePickerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
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
    /// Get the CSS class suffix.
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Middle => "middle",
            Self::Large => "large",
        }
    }

    /// Get the full CSS class.
    pub fn class(&self, prefix: &str) -> String {
        match self {
            Self::Middle => String::new(),
            _ => format!("{}-{}", prefix, self.as_suffix()),
        }
    }
}

impl fmt::Display for TimePickerSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}

/// Time format for time pickers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimeFormat {
    /// 24-hour format (HH:mm)
    #[default]
    H24,
    /// 12-hour format with AM/PM (hh:mm A)
    H12,
    /// 24-hour with seconds (HH:mm:ss)
    H24WithSeconds,
    /// 12-hour with seconds (hh:mm:ss A)
    H12WithSeconds,
}

impl TimeFormat {
    /// Get the format string.
    pub fn as_format(&self) -> &'static str {
        match self {
            Self::H24 => "HH:mm",
            Self::H12 => "hh:mm A",
            Self::H24WithSeconds => "HH:mm:ss",
            Self::H12WithSeconds => "hh:mm:ss A",
        }
    }

    /// Check if this format includes seconds.
    pub fn has_seconds(&self) -> bool {
        matches!(self, Self::H24WithSeconds | Self::H12WithSeconds)
    }

    /// Check if this is 12-hour format.
    pub fn is_12_hour(&self) -> bool {
        matches!(self, Self::H12 | Self::H12WithSeconds)
    }
}

/// Date range value.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateRangeValue {
    /// Start date (YYYY-MM-DD format).
    pub start: Option<String>,
    /// End date (YYYY-MM-DD format).
    pub end: Option<String>,
}

impl DateRangeValue {
    /// Create a new date range.
    pub fn new(start: Option<String>, end: Option<String>) -> Self {
        Self { start, end }
    }

    /// Create from string tuple.
    pub fn from_pair(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start: Some(start.into()),
            end: Some(end.into()),
        }
    }

    /// Check if range is empty.
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Check if range is complete.
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Get display string.
    pub fn display(&self, separator: &str) -> String {
        match (&self.start, &self.end) {
            (Some(s), Some(e)) => format!("{} {} {}", s, separator, e),
            (Some(s), None) => format!("{} {}", s, separator),
            (None, Some(e)) => format!("{} {}", separator, e),
            (None, None) => String::new(),
        }
    }
}

/// Time range value.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeRangeValue {
    /// Start time (HH:mm or HH:mm:ss format).
    pub start: Option<String>,
    /// End time (HH:mm or HH:mm:ss format).
    pub end: Option<String>,
}

impl TimeRangeValue {
    /// Create a new time range.
    pub fn new(start: Option<String>, end: Option<String>) -> Self {
        Self { start, end }
    }

    /// Create from string tuple.
    pub fn from_pair(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start: Some(start.into()),
            end: Some(end.into()),
        }
    }

    /// Check if range is empty.
    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Check if range is complete.
    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Get display string.
    pub fn display(&self, separator: &str) -> String {
        match (&self.start, &self.end) {
            (Some(s), Some(e)) => format!("{} {} {}", s, separator, e),
            (Some(s), None) => format!("{} {}", s, separator),
            (None, Some(e)) => format!("{} {}", separator, e),
            (None, None) => String::new(),
        }
    }
}

/// DateTime value (combined date and time).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTimeValue {
    /// Date part (YYYY-MM-DD format).
    pub date: Option<String>,
    /// Time part (HH:mm or HH:mm:ss format).
    pub time: Option<String>,
}

impl DateTimeValue {
    /// Create a new date-time value.
    pub fn new(date: Option<String>, time: Option<String>) -> Self {
        Self { date, time }
    }

    /// Create from date and time strings.
    pub fn from_parts(date: impl Into<String>, time: impl Into<String>) -> Self {
        Self {
            date: Some(date.into()),
            time: Some(time.into()),
        }
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.date.is_none() && self.time.is_none()
    }

    /// Check if complete (has both date and time).
    pub fn is_complete(&self) -> bool {
        self.date.is_some() && self.time.is_some()
    }

    /// Get ISO format string (YYYY-MM-DDTHH:mm:ss).
    pub fn to_iso(&self) -> Option<String> {
        match (&self.date, &self.time) {
            (Some(d), Some(t)) => Some(format!("{}T{}", d, t)),
            _ => None,
        }
    }

    /// Get display string.
    pub fn display(&self, separator: &str) -> String {
        match (&self.date, &self.time) {
            (Some(d), Some(t)) => format!("{}{}{}", d, separator, t),
            (Some(d), None) => d.clone(),
            (None, Some(t)) => t.clone(),
            (None, None) => String::new(),
        }
    }
}

/// Preset range for quick selection.
#[derive(Debug, Clone, PartialEq)]
pub struct DateRangePreset {
    /// Display label.
    pub label: String,
    /// Start date value.
    pub start: String,
    /// End date value.
    pub end: String,
}

impl DateRangePreset {
    /// Create a new preset.
    pub fn new(label: impl Into<String>, start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            start: start.into(),
            end: end.into(),
        }
    }

    /// Create "Today" preset.
    pub fn today(today: &str) -> Self {
        Self::new("Today", today, today)
    }

    /// Create "Yesterday" preset.
    pub fn yesterday(yesterday: &str) -> Self {
        Self::new("Yesterday", yesterday, yesterday)
    }

    /// Create "Last 7 days" preset.
    pub fn last_7_days(start: &str, end: &str) -> Self {
        Self::new("Last 7 days", start, end)
    }

    /// Create "Last 30 days" preset.
    pub fn last_30_days(start: &str, end: &str) -> Self {
        Self::new("Last 30 days", start, end)
    }

    /// Create "This month" preset.
    pub fn this_month(start: &str, end: &str) -> Self {
        Self::new("This month", start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_mode() {
        assert_eq!(DatePickerMode::default(), DatePickerMode::Date);
        assert_eq!(DatePickerMode::Week.class("fx-picker"), "fx-picker-week");
    }

    #[test]
    fn test_date_picker_size() {
        assert_eq!(DatePickerSize::default(), DatePickerSize::Middle);
        assert_eq!(DatePickerSize::Middle.class("fx-picker"), "");
        assert_eq!(DatePickerSize::Small.class("fx-picker"), "fx-picker-small");
    }

    #[test]
    fn test_time_format() {
        assert_eq!(TimeFormat::default(), TimeFormat::H24);
        assert!(TimeFormat::H24WithSeconds.has_seconds());
        assert!(TimeFormat::H12.is_12_hour());
    }

    #[test]
    fn test_date_range_value() {
        let empty = DateRangeValue::default();
        assert!(empty.is_empty());

        let range = DateRangeValue::from_pair("2024-01-01", "2024-01-31");
        assert!(range.is_complete());
        assert_eq!(range.display("~"), "2024-01-01 ~ 2024-01-31");
    }

    #[test]
    fn test_date_time_value() {
        let dt = DateTimeValue::from_parts("2024-01-15", "14:30");
        assert!(dt.is_complete());
        assert_eq!(dt.to_iso(), Some("2024-01-15T14:30".to_string()));
    }

    #[test]
    fn test_date_range_preset() {
        let preset = DateRangePreset::new("Custom", "2024-01-01", "2024-12-31");
        assert_eq!(preset.label, "Custom");
    }
}
