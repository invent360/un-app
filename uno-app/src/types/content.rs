//! CMS content type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported locales for content
pub const SUPPORTED_LOCALES: &[&str] = &["en", "es", "fr", "ar", "hi", "tl", "sw", "pt", "id", "bn"];

/// Content status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    #[default]
    Draft,
    PendingReview,
    Approved,
    Published,
    Archived,
}

impl ContentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentStatus::Draft => "draft",
            ContentStatus::PendingReview => "pending_review",
            ContentStatus::Approved => "approved",
            ContentStatus::Published => "published",
            ContentStatus::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "draft" => Some(ContentStatus::Draft),
            "pending_review" => Some(ContentStatus::PendingReview),
            "approved" => Some(ContentStatus::Approved),
            "published" => Some(ContentStatus::Published),
            "archived" => Some(ContentStatus::Archived),
            _ => None,
        }
    }

    /// Check if status allows editing
    pub fn is_editable(&self) -> bool {
        matches!(self, ContentStatus::Draft)
    }

    /// Check if content can be submitted for review
    pub fn can_submit_for_review(&self) -> bool {
        matches!(self, ContentStatus::Draft)
    }

    /// Check if content can be published directly
    pub fn can_publish_directly(&self) -> bool {
        matches!(self, ContentStatus::Draft | ContentStatus::Approved)
    }
}

impl std::fmt::Display for ContentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Content type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Task,
    Guide,
    Faq,
    Error,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Task => "task",
            ContentType::Guide => "guide",
            ContentType::Faq => "faq",
            ContentType::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "task" => Some(ContentType::Task),
            "guide" => Some(ContentType::Guide),
            "faq" => Some(ContentType::Faq),
            "error" => Some(ContentType::Error),
            _ => None,
        }
    }

    /// Get required fields for this content type
    pub fn required_fields(&self) -> &'static [&'static str] {
        match self {
            ContentType::Task => &["title", "description"],
            ContentType::Guide => &["title", "description"],
            ContentType::Faq => &["question", "answer"],
            ContentType::Error => &["error_code", "title", "description", "solution"],
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Translation status for a locale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TranslationStatus {
    Complete,
    Partial,
    #[default]
    Missing,
    Auto,
}

impl TranslationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TranslationStatus::Complete => "complete",
            TranslationStatus::Partial => "partial",
            TranslationStatus::Missing => "missing",
            TranslationStatus::Auto => "auto",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "complete" => TranslationStatus::Complete,
            "partial" => TranslationStatus::Partial,
            "auto" => TranslationStatus::Auto,
            _ => TranslationStatus::Missing,
        }
    }
}

/// Text direction for RTL support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

impl TextDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextDirection::Ltr => "ltr",
            TextDirection::Rtl => "rtl",
        }
    }

    pub fn from_locale(locale: &str) -> Self {
        match locale {
            "ar" | "he" | "fa" | "ur" => TextDirection::Rtl,
            _ => TextDirection::Ltr,
        }
    }
}

/// Earnings tier for tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsTier {
    pub name: String,
    pub min_earnings: f64,
    pub max_earnings: f64,
    pub period: String,  // "month", "day", "week"
    pub features: Vec<String>,
    pub is_popular: bool,
}

impl EarningsTier {
    pub fn earnings_display(&self) -> String {
        format!("${:.0}-${:.0}/{}", self.min_earnings, self.max_earnings, self.period)
    }
}

/// Task content schema (stored in JSONB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContent {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default = "default_task_status")]
    pub status: String,  // "active", "coming_soon"
    #[serde(default)]
    pub earnings_estimate: Vec<EarningsTier>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default)]
    pub requirements: Vec<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default = "default_direction")]
    pub direction: String,
}

fn default_task_status() -> String {
    "active".to_string()
}

fn default_direction() -> String {
    "ltr".to_string()
}

/// Guide stage (step within a section)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideStage {
    #[serde(default)]
    pub order: i32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    /// Single image (legacy format)
    #[serde(default)]
    pub image: Option<String>,
    /// Multiple images (CMS format)
    #[serde(default)]
    pub images: Vec<String>,
    /// Videos (CMS format)
    #[serde(default)]
    pub videos: Vec<String>,
}

impl GuideStage {
    /// Get the primary image (from `image` field or first of `images` array)
    pub fn primary_image(&self) -> Option<String> {
        self.image.clone().or_else(|| self.images.first().cloned())
    }
}

/// Guide content schema (represents a guide section)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideContent {
    #[serde(default)]
    pub title: String,
    /// Description field - accepts both "description" and "summary" (legacy) from JSON
    #[serde(default, alias = "summary")]
    pub description: String,
    #[serde(default)]
    pub video_url: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Cover images for this guide/section
    #[serde(default)]
    pub cover_images: Vec<String>,
    /// Stages (accepts both "stages" and "steps" for backward compatibility)
    #[serde(default, alias = "steps")]
    pub stages: Vec<GuideStage>,
    #[serde(default)]
    pub duration_minutes: Option<i32>,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default = "default_direction")]
    pub direction: String,
}

impl GuideContent {
    /// Get the primary cover image (from `thumbnail` or first of `cover_images`)
    pub fn primary_image(&self) -> Option<String> {
        self.thumbnail.clone().or_else(|| self.cover_images.first().cloned())
    }
}

/// Error content schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContent {
    pub error_code: String,
    pub title: String,
    pub description: String,
    pub solution: String,
    #[serde(default)]
    pub related_errors: Vec<String>,
    #[serde(default = "default_direction")]
    pub direction: String,
}

/// Page content entity from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PageContent {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub status: String,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub content: serde_json::Value,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub translations: Option<serde_json::Value>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub translation_status: Option<serde_json::Value>,
    pub display_order: i32,
    pub is_featured: bool,
    pub is_active: bool,
    pub version: i32,
    pub published_version: Option<i32>,
    pub published_at: Option<DateTime<Utc>>,
    pub published_by: Option<String>,
    pub publish_at: Option<DateTime<Utc>>,
    pub unpublish_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl PageContent {
    /// Get localized content, falling back to English
    pub fn localize(&self, locale: &str) -> serde_json::Value {
        if locale == "en" {
            return self.content.clone();
        }

        self.translations
            .as_ref()
            .and_then(|t| t.get(locale))
            .cloned()
            .unwrap_or_else(|| self.content.clone())
    }

    /// Check if a translation exists for the given locale
    pub fn has_translation(&self, locale: &str) -> bool {
        if locale == "en" {
            return true;
        }

        self.translations
            .as_ref()
            .map(|t| t.get(locale).is_some())
            .unwrap_or(false)
    }

    /// Get text direction for a locale
    pub fn get_direction(&self, locale: &str) -> String {
        let content = self.localize(locale);
        content
            .get("direction")
            .and_then(|d| d.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| TextDirection::from_locale(locale).as_str().to_string())
    }

    /// Get translation status for a locale
    pub fn get_translation_status(&self, locale: &str) -> TranslationStatus {
        if locale == "en" {
            return TranslationStatus::Complete;
        }

        self.translation_status
            .as_ref()
            .and_then(|ts| ts.get(locale))
            .and_then(|s| s.as_str())
            .map(TranslationStatus::from_str)
            .unwrap_or(TranslationStatus::Missing)
    }

    /// Calculate translation coverage percentage
    pub fn translation_coverage(&self) -> i32 {
        let mut complete_count = 1; // English is always complete

        if let Some(ref ts) = self.translation_status {
            for locale in SUPPORTED_LOCALES.iter().skip(1) {
                if let Some(status) = ts.get(*locale).and_then(|s| s.as_str()) {
                    if status == "complete" {
                        complete_count += 1;
                    }
                }
            }
        }

        ((complete_count as f64 / SUPPORTED_LOCALES.len() as f64) * 100.0) as i32
    }

    /// Get content type enum
    pub fn content_type_enum(&self) -> Option<ContentType> {
        ContentType::from_str(&self.content_type)
    }

    /// Get status enum
    pub fn status_enum(&self) -> Option<ContentStatus> {
        ContentStatus::from_str(&self.status)
    }
}

/// Content version entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ContentVersion {
    pub id: i32,
    pub content_id: i32,
    pub version: i32,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub content: serde_json::Value,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub translations: Option<serde_json::Value>,
    pub change_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

/// Localized content response for public API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedContent {
    pub id: i32,
    pub content_type: String,
    pub slug: String,
    pub content: serde_json::Value,
    pub direction: String,
    pub is_featured: bool,
    pub is_fallback: bool,  // True if falling back to English
    pub published_at: Option<DateTime<Utc>>,
}
