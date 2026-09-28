//! FAQ Type Definitions
//!
//! Types for FAQ page content management.
//! Extends the generic Section pattern with FAQ-specific question structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema ID for FAQ page content
pub const FAQ_PAGE_SCHEMA_ID: &str = "faq";

// ============================================
// FAQ CATEGORY ENUM
// ============================================

/// FAQ Question categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum FaqCategory {
    /// All categories (used for filtering only)
    #[default]
    All,
    /// General questions about the platform
    General,
    /// Questions about earnings and payments
    Earnings,
    /// Questions about setup and installation
    Setup,
    /// Questions about security and privacy
    Security,
}

impl FaqCategory {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::General => "general",
            Self::Earnings => "earnings",
            Self::Setup => "setup",
            Self::Security => "security",
        }
    }

    /// Get the display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::General => "General",
            Self::Earnings => "Earnings",
            Self::Setup => "Setup",
            Self::Security => "Security",
        }
    }

    /// Get the color for UI badges
    pub fn color(&self) -> &'static str {
        match self {
            Self::All => "#64748b",
            Self::General => "#3b82f6",
            Self::Earnings => "#22c55e",
            Self::Setup => "#8b5cf6",
            Self::Security => "#f59e0b",
        }
    }

    /// Get the icon name for UI
    pub fn icon(&self) -> &'static str {
        match self {
            Self::All => "folder",
            Self::General => "info",
            Self::Earnings => "revenue",
            Self::Setup => "settings",
            Self::Security => "shield",
        }
    }

    /// Get all selectable categories (excludes "All" which is filter-only)
    pub fn selectable_categories() -> Vec<Self> {
        vec![Self::General, Self::Earnings, Self::Setup, Self::Security]
    }

    /// Get all categories including "All" for filtering
    pub fn all_categories() -> Vec<Self> {
        vec![Self::All, Self::General, Self::Earnings, Self::Setup, Self::Security]
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "all" => Some(Self::All),
            "general" => Some(Self::General),
            "earnings" => Some(Self::Earnings),
            "setup" => Some(Self::Setup),
            "security" => Some(Self::Security),
            _ => None,
        }
    }
}

// ============================================
// FAQ QUESTION STRUCT
// ============================================

fn default_true() -> bool {
    true
}

/// Individual FAQ Question
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FaqQuestion {
    /// Question category
    #[serde(default)]
    pub category: FaqCategory,

    /// The question text
    #[serde(default)]
    pub question: String,

    /// The answer text (supports markdown/rich text)
    #[serde(default)]
    pub answer: String,

    /// Display order within the category
    #[serde(default)]
    pub display_order: i32,

    /// Whether this question is featured/pinned
    #[serde(default)]
    pub is_featured: bool,

    /// Whether this question is visible
    #[serde(default = "default_true")]
    pub is_visible: bool,
}

impl FaqQuestion {
    /// Create a new FAQ question
    pub fn new(category: FaqCategory, question: &str, answer: &str) -> Self {
        Self {
            category,
            question: question.to_string(),
            answer: answer.to_string(),
            display_order: 0,
            is_featured: false,
            is_visible: true,
        }
    }

    /// Create a featured question
    pub fn featured(category: FaqCategory, question: &str, answer: &str) -> Self {
        Self {
            category,
            question: question.to_string(),
            answer: answer.to_string(),
            display_order: 0,
            is_featured: true,
            is_visible: true,
        }
    }

    /// Check if question matches search query (case-insensitive)
    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.question.to_lowercase().contains(&query_lower)
            || self.answer.to_lowercase().contains(&query_lower)
    }

    /// Get a truncated preview of the answer
    pub fn answer_preview(&self, max_chars: usize) -> String {
        if self.answer.len() <= max_chars {
            self.answer.clone()
        } else {
            format!("{}...", &self.answer[..max_chars])
        }
    }
}

// ============================================
// FAQ PAGE DATA STRUCT
// ============================================

/// FAQ Page Data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FaqPageData {
    /// Page title
    #[serde(default)]
    pub title: String,

    /// Page description/subtitle
    #[serde(default)]
    pub description: String,

    /// Array of FAQ questions
    #[serde(default)]
    pub questions: Vec<FaqQuestion>,
}

impl FaqPageData {
    /// Create a new FAQ page with default values
    pub fn new() -> Self {
        Self {
            title: "Frequently Asked Questions".to_string(),
            description: "Find answers to common questions.".to_string(),
            questions: Vec::new(),
        }
    }

    /// Create from JSON value
    pub fn from_json(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    /// Convert to JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    /// Get questions filtered by category
    pub fn questions_by_category(&self, category: FaqCategory) -> Vec<&FaqQuestion> {
        if category == FaqCategory::All {
            self.questions.iter().collect()
        } else {
            self.questions
                .iter()
                .filter(|q| q.category == category)
                .collect()
        }
    }

    /// Get visible questions sorted by display_order
    pub fn visible_questions(&self) -> Vec<&FaqQuestion> {
        let mut questions: Vec<_> = self.questions.iter().filter(|q| q.is_visible).collect();
        questions.sort_by_key(|q| q.display_order);
        questions
    }

    /// Get visible questions filtered by category and sorted
    pub fn visible_questions_by_category(&self, category: FaqCategory) -> Vec<&FaqQuestion> {
        let mut questions: Vec<_> = self
            .questions
            .iter()
            .filter(|q| {
                q.is_visible && (category == FaqCategory::All || q.category == category)
            })
            .collect();
        questions.sort_by_key(|q| q.display_order);
        questions
    }

    /// Get featured questions
    pub fn featured_questions(&self) -> Vec<&FaqQuestion> {
        self.questions
            .iter()
            .filter(|q| q.is_featured && q.is_visible)
            .collect()
    }

    /// Count questions by category (visible only)
    pub fn category_counts(&self) -> HashMap<FaqCategory, usize> {
        let mut counts = HashMap::new();
        for q in &self.questions {
            if q.is_visible {
                *counts.entry(q.category).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Get total count of visible questions
    pub fn visible_count(&self) -> usize {
        self.questions.iter().filter(|q| q.is_visible).count()
    }

    /// Search questions by query
    pub fn search(&self, query: &str) -> Vec<&FaqQuestion> {
        if query.trim().is_empty() {
            return self.visible_questions();
        }
        self.questions
            .iter()
            .filter(|q| q.is_visible && q.matches_search(query))
            .collect()
    }

    /// Add a question
    pub fn add_question(&mut self, question: FaqQuestion) {
        self.questions.push(question);
    }

    /// Remove a question by index
    pub fn remove_question(&mut self, index: usize) -> Option<FaqQuestion> {
        if index < self.questions.len() {
            Some(self.questions.remove(index))
        } else {
            None
        }
    }

    /// Reorder questions - move item from one index to another
    pub fn reorder(&mut self, from: usize, to: usize) {
        if from < self.questions.len() && to < self.questions.len() {
            let item = self.questions.remove(from);
            self.questions.insert(to, item);
            // Update display_order for all questions
            for (i, q) in self.questions.iter_mut().enumerate() {
                q.display_order = i as i32;
            }
        }
    }
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faq_category_from_str() {
        assert_eq!(FaqCategory::from_str("general"), Some(FaqCategory::General));
        assert_eq!(FaqCategory::from_str("EARNINGS"), Some(FaqCategory::Earnings));
        assert_eq!(FaqCategory::from_str("invalid"), None);
    }

    #[test]
    fn test_faq_question_search() {
        let q = FaqQuestion::new(
            FaqCategory::General,
            "What is UNO?",
            "UNO is a platform for passive income.",
        );
        assert!(q.matches_search("uno"));
        assert!(q.matches_search("PLATFORM"));
        assert!(q.matches_search("passive"));
        assert!(!q.matches_search("bitcoin"));
    }

    #[test]
    fn test_faq_page_filtering() {
        let mut page = FaqPageData::new();
        page.add_question(FaqQuestion::new(
            FaqCategory::General,
            "Q1",
            "A1",
        ));
        page.add_question(FaqQuestion::new(
            FaqCategory::Earnings,
            "Q2",
            "A2",
        ));
        page.add_question(FaqQuestion::new(
            FaqCategory::General,
            "Q3",
            "A3",
        ));

        assert_eq!(page.questions_by_category(FaqCategory::All).len(), 3);
        assert_eq!(page.questions_by_category(FaqCategory::General).len(), 2);
        assert_eq!(page.questions_by_category(FaqCategory::Earnings).len(), 1);
        assert_eq!(page.questions_by_category(FaqCategory::Setup).len(), 0);
    }

    #[test]
    fn test_faq_page_category_counts() {
        let mut page = FaqPageData::new();
        page.add_question(FaqQuestion::new(FaqCategory::General, "Q1", "A1"));
        page.add_question(FaqQuestion::new(FaqCategory::General, "Q2", "A2"));
        page.add_question(FaqQuestion::new(FaqCategory::Earnings, "Q3", "A3"));

        let counts = page.category_counts();
        assert_eq!(counts.get(&FaqCategory::General), Some(&2));
        assert_eq!(counts.get(&FaqCategory::Earnings), Some(&1));
        assert_eq!(counts.get(&FaqCategory::Setup), None);
    }

    #[test]
    fn test_faq_page_json_roundtrip() {
        let mut page = FaqPageData::new();
        page.title = "Test FAQ".to_string();
        page.add_question(FaqQuestion::featured(
            FaqCategory::Security,
            "Is it safe?",
            "Yes, very safe.",
        ));

        let json = page.to_json();
        let restored = FaqPageData::from_json(&json);

        assert_eq!(page.title, restored.title);
        assert_eq!(page.questions.len(), restored.questions.len());
        assert_eq!(page.questions[0].question, restored.questions[0].question);
    }
}
