//! FAQ-related type definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// FAQ item from database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct FaqItem {
    pub id: i32,
    pub category: String,
    pub display_order: i32,
    pub question_en: String,
    pub answer_en: String,
    /// JSONB translations: { "es": { "question": "...", "answer": "..." }, ... }
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub translations: Option<serde_json::Value>,
    pub is_featured: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FaqItem {
    /// Get question in the specified locale, fallback to English
    pub fn question(&self, locale: &str) -> String {
        if locale == "en" {
            return self.question_en.clone();
        }

        self.translations
            .as_ref()
            .and_then(|t| t.get(locale))
            .and_then(|l| l.get("question"))
            .and_then(|q| q.as_str())
            .map(String::from)
            .unwrap_or_else(|| self.question_en.clone())
    }

    /// Get answer in the specified locale, fallback to English
    pub fn answer(&self, locale: &str) -> String {
        if locale == "en" {
            return self.answer_en.clone();
        }

        self.translations
            .as_ref()
            .and_then(|t| t.get(locale))
            .and_then(|l| l.get("answer"))
            .and_then(|a| a.as_str())
            .map(String::from)
            .unwrap_or_else(|| self.answer_en.clone())
    }
}

/// FAQ item for API response (localized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItemResponse {
    pub id: i32,
    pub category: String,
    pub question: String,
    pub answer: String,
    pub is_featured: bool,
}

impl FaqItemResponse {
    /// Create from FaqItem with specified locale
    pub fn from_item(item: &FaqItem, locale: &str) -> Self {
        Self {
            id: item.id,
            category: item.category.clone(),
            question: item.question(locale),
            answer: item.answer(locale),
            is_featured: item.is_featured,
        }
    }
}

/// FAQ category with count
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct FaqCategory {
    pub name: String,
    pub count: i64,
}

/// FAQ search request parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FaqSearchParams {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(default)]
    pub featured_only: bool,
}

/// FAQ list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqListResponse {
    pub items: Vec<FaqItemResponse>,
    pub categories: Vec<FaqCategory>,
    pub total: usize,
}
