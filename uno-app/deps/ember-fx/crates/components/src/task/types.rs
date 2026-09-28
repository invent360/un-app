//! Task card types for CMS-driven task display.

use serde::{Deserialize, Serialize};

/// Task status indicating availability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Active,
    ComingSoon,
    Maintenance,
}

impl TaskStatus {
    /// Get CSS class suffix for status
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskStatus::Active => "active",
            TaskStatus::ComingSoon => "coming-soon",
            TaskStatus::Maintenance => "maintenance",
        }
    }

    /// Get display label for status
    pub fn label(&self) -> &'static str {
        match self {
            TaskStatus::Active => "Active",
            TaskStatus::ComingSoon => "Coming Soon",
            TaskStatus::Maintenance => "Maintenance",
        }
    }
}

/// Difficulty level for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskDifficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

/// Task type indicating user participation level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    /// Active tasks require user participation
    #[default]
    Active,
    /// Passive tasks run in the background
    Passive,
}

impl TaskType {
    /// Get CSS class suffix for task type
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskType::Active => "active",
            TaskType::Passive => "passive",
        }
    }

    /// Get display label for task type
    pub fn label(&self) -> &'static str {
        match self {
            TaskType::Active => "Active",
            TaskType::Passive => "Passive",
        }
    }
}

/// Helper type for task_type builder method to accept both enum and string
pub struct TaskTypeInput(pub TaskType);

impl From<TaskType> for TaskTypeInput {
    fn from(t: TaskType) -> Self {
        TaskTypeInput(t)
    }
}

impl From<&str> for TaskTypeInput {
    fn from(s: &str) -> Self {
        TaskTypeInput(match s.to_lowercase().as_str() {
            "passive" => TaskType::Passive,
            _ => TaskType::Active,
        })
    }
}

impl From<String> for TaskTypeInput {
    fn from(s: String) -> Self {
        TaskTypeInput::from(s.as_str())
    }
}

impl TaskDifficulty {
    /// Get CSS class suffix for difficulty
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskDifficulty::Easy => "easy",
            TaskDifficulty::Medium => "medium",
            TaskDifficulty::Hard => "hard",
        }
    }

    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            TaskDifficulty::Easy => "Easy",
            TaskDifficulty::Medium => "Medium",
            TaskDifficulty::Hard => "Hard",
        }
    }
}

/// Earnings period
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EarningsPeriod {
    #[default]
    Month,
    Week,
    Day,
    Hour,
}

impl EarningsPeriod {
    /// Get display suffix (e.g., "/month")
    pub fn suffix(&self) -> &'static str {
        match self {
            EarningsPeriod::Month => "/mo",
            EarningsPeriod::Week => "/wk",
            EarningsPeriod::Day => "/day",
            EarningsPeriod::Hour => "/hr",
        }
    }
}

/// Earnings tier with features
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarningsTier {
    /// Tier name (e.g., "1 Device", "2-3 Devices")
    pub name: String,
    /// Minimum earnings
    pub min_earnings: f64,
    /// Maximum earnings
    pub max_earnings: f64,
    /// Earnings period
    #[serde(default)]
    pub period: EarningsPeriod,
    /// Features included in this tier
    #[serde(default)]
    pub features: Vec<String>,
    /// Whether this tier is the most popular/recommended
    #[serde(default)]
    pub is_popular: bool,
}

impl EarningsTier {
    /// Create a new earnings tier
    pub fn new(name: impl Into<String>, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            min_earnings: min,
            max_earnings: max,
            period: EarningsPeriod::Month,
            features: Vec::new(),
            is_popular: false,
        }
    }

    /// Set period
    pub fn period(mut self, period: EarningsPeriod) -> Self {
        self.period = period;
        self
    }

    /// Add a feature
    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    /// Set multiple features
    pub fn features(mut self, features: Vec<String>) -> Self {
        self.features = features;
        self
    }

    /// Mark as popular tier
    pub fn popular(mut self) -> Self {
        self.is_popular = true;
        self
    }

    /// Format earnings range for display (without locale)
    pub fn format_range(&self) -> String {
        format!(
            "${:.0} - ${:.0}{}",
            self.min_earnings,
            self.max_earnings,
            self.period.suffix()
        )
    }
}

/// Task data for display in TaskCard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskData {
    /// Unique identifier
    pub id: String,
    /// Task slug for URL
    pub slug: String,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Single image URL (for backwards compatibility)
    #[serde(default)]
    pub image: Option<String>,
    /// Multiple images for gallery display
    #[serde(default)]
    pub images: Vec<String>,
    /// Gallery auto-advance interval in milliseconds (default: 3000)
    #[serde(default = "default_gallery_interval")]
    pub gallery_interval: u32,
    /// Task status
    #[serde(default)]
    pub status: TaskStatus,
    /// Task type (active/passive)
    #[serde(default)]
    pub task_type: TaskType,
    /// Earnings tiers
    #[serde(default)]
    pub earnings_tiers: Vec<EarningsTier>,
    /// Estimated duration (e.g., "5-10 min/day")
    #[serde(default)]
    pub duration: Option<String>,
    /// Task difficulty
    #[serde(default)]
    pub difficulty: TaskDifficulty,
    /// Requirements list
    #[serde(default)]
    pub requirements: Vec<String>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: serde_json::Value,
    /// Text direction (ltr/rtl)
    #[serde(default = "default_direction")]
    pub direction: String,
}

fn default_gallery_interval() -> u32 {
    3000
}

fn default_direction() -> String {
    "ltr".to_string()
}

impl TaskData {
    /// Create a new task with required fields
    pub fn new(id: impl Into<String>, slug: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            image: None,
            images: Vec::new(),
            gallery_interval: 3000,
            status: TaskStatus::Active,
            task_type: TaskType::Active,
            earnings_tiers: Vec::new(),
            duration: None,
            difficulty: TaskDifficulty::Easy,
            requirements: Vec::new(),
            metadata: serde_json::Value::Null,
            direction: "ltr".to_string(),
        }
    }

    /// Set single image (for backwards compatibility)
    pub fn image(mut self, url: impl Into<String>) -> Self {
        self.image = Some(url.into());
        self
    }

    /// Add an image to the gallery
    pub fn add_image(mut self, url: impl Into<String>) -> Self {
        self.images.push(url.into());
        self
    }

    /// Set multiple images for gallery
    pub fn images(mut self, urls: Vec<String>) -> Self {
        self.images = urls;
        self
    }

    /// Set gallery auto-advance interval in milliseconds
    pub fn gallery_interval(mut self, ms: u32) -> Self {
        self.gallery_interval = ms;
        self
    }

    /// Get all images (combines single image and gallery images)
    pub fn all_images(&self) -> Vec<String> {
        let mut all = Vec::new();
        if let Some(ref img) = self.image {
            all.push(img.clone());
        }
        all.extend(self.images.clone());
        all
    }

    /// Check if task has multiple images (gallery mode)
    pub fn has_gallery(&self) -> bool {
        self.all_images().len() > 1
    }

    /// Set status
    pub fn status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    /// Set task type (active/passive)
    pub fn task_type(mut self, task_type: impl Into<TaskTypeInput>) -> Self {
        self.task_type = task_type.into().0;
        self
    }

    /// Add earnings tier
    pub fn tier(mut self, tier: EarningsTier) -> Self {
        self.earnings_tiers.push(tier);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set difficulty
    pub fn difficulty(mut self, difficulty: TaskDifficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    /// Add requirement
    pub fn requirement(mut self, req: impl Into<String>) -> Self {
        self.requirements.push(req.into());
        self
    }

    /// Set direction
    pub fn direction(mut self, dir: impl Into<String>) -> Self {
        self.direction = dir.into();
        self
    }

    /// Check if task is available (active)
    pub fn is_available(&self) -> bool {
        self.status == TaskStatus::Active
    }

    /// Get primary earnings display (first tier or combined range)
    pub fn primary_earnings(&self) -> Option<String> {
        if self.earnings_tiers.is_empty() {
            return None;
        }

        if self.earnings_tiers.len() == 1 {
            return Some(self.earnings_tiers[0].format_range());
        }

        // Combine all tiers for range
        let min = self.earnings_tiers.iter()
            .map(|t| t.min_earnings)
            .fold(f64::INFINITY, f64::min);
        let max = self.earnings_tiers.iter()
            .map(|t| t.max_earnings)
            .fold(f64::NEG_INFINITY, f64::max);
        let period = &self.earnings_tiers[0].period;

        Some(format!("${:.0} - ${:.0}{}", min, max, period.suffix()))
    }
}

/// Task card size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskCardSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TaskCardSize {
    /// Get CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskCardSize::Small => "sm",
            TaskCardSize::Medium => "md",
            TaskCardSize::Large => "lg",
        }
    }
}

/// Task card display variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskCardVariant {
    #[default]
    Card,
    Compact,
    Featured,
}

impl TaskCardVariant {
    /// Get CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskCardVariant::Card => "card",
            TaskCardVariant::Compact => "compact",
            TaskCardVariant::Featured => "featured",
        }
    }
}

/// Task card layout options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskCardLayout {
    /// Stacked layout: image on top, content below (default)
    #[default]
    Stacked,
    /// Side by side: media on left, content on right
    SideBySide,
}

impl TaskCardLayout {
    /// Get CSS class suffix
    pub fn class_suffix(&self) -> &'static str {
        match self {
            TaskCardLayout::Stacked => "stacked",
            TaskCardLayout::SideBySide => "side-by-side",
        }
    }
}
