//! Locale review service
//!
//! Provides per-locale translation review workflow for CMS content.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::db::ConnectionPool;
use crate::types::AppError;

// ============================================
// TYPES
// ============================================

/// Locale review status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleReviewStatus {
    Pending,
    InReview,
    Approved,
    ChangesRequested,
    Rejected,
}

impl LocaleReviewStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InReview => "in_review",
            Self::Approved => "approved",
            Self::ChangesRequested => "changes_requested",
            Self::Rejected => "rejected",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "in_review" => Some(Self::InReview),
            "approved" => Some(Self::Approved),
            "changes_requested" => Some(Self::ChangesRequested),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

/// Locale review entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocaleReview {
    pub id: Uuid,
    pub version_id: Uuid,
    pub locale: String,
    pub status: String,
    pub reviewer_id: Option<String>,
    pub review_notes: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub translation_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Submit review input
#[derive(Debug, Clone)]
pub struct SubmitLocaleReviewInput {
    pub version_id: Uuid,
    pub locale: String,
    pub created_by: Option<String>,
}

/// Approve locale input
#[derive(Debug, Clone)]
pub struct ApproveLocaleInput {
    pub version_id: Uuid,
    pub locale: String,
    pub reviewer_id: String,
    pub review_notes: Option<String>,
    pub translation_score: Option<i32>,
}

/// Request changes input
#[derive(Debug, Clone)]
pub struct RequestChangesInput {
    pub version_id: Uuid,
    pub locale: String,
    pub reviewer_id: String,
    pub review_notes: String,
}

// ============================================
// TRAIT DEFINITION
// ============================================

pub type DynLocaleReviewService = Arc<dyn LocaleReviewService + Send + Sync>;

#[async_trait]
pub trait LocaleReviewService: Send + Sync {
    /// Submit a locale translation for review
    async fn submit_for_review(&self, input: SubmitLocaleReviewInput) -> Result<LocaleReview, AppError>;

    /// Start reviewing a locale
    async fn start_review(&self, version_id: Uuid, locale: &str, reviewer_id: &str) -> Result<LocaleReview, AppError>;

    /// Approve a locale translation
    async fn approve_locale(&self, input: ApproveLocaleInput) -> Result<LocaleReview, AppError>;

    /// Request changes for a locale translation
    async fn request_changes(&self, input: RequestChangesInput) -> Result<LocaleReview, AppError>;

    /// Reject a locale translation
    async fn reject_locale(&self, version_id: Uuid, locale: &str, reviewer_id: &str, reason: &str) -> Result<LocaleReview, AppError>;

    /// Get review status for a version
    async fn get_reviews_for_version(&self, version_id: Uuid) -> Result<Vec<LocaleReview>, AppError>;

    /// Get pending reviews by locale
    async fn get_pending_by_locale(&self, locale: &str, limit: i32) -> Result<Vec<LocaleReview>, AppError>;

    /// Get reviews assigned to a reviewer
    async fn get_reviews_for_reviewer(&self, reviewer_id: &str, limit: i32) -> Result<Vec<LocaleReview>, AppError>;
}

// ============================================
// IMPLEMENTATION
// ============================================

pub struct LocaleReviewServiceImpl {
    pool: ConnectionPool,
}

impl LocaleReviewServiceImpl {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LocaleReviewService for LocaleReviewServiceImpl {
    async fn submit_for_review(&self, input: SubmitLocaleReviewInput) -> Result<LocaleReview, AppError> {
        let review = sqlx::query_as::<_, LocaleReview>(
            r#"
            INSERT INTO content_locale_reviews (version_id, locale, status, created_by)
            VALUES ($1, $2, 'pending', $3)
            ON CONFLICT (version_id, locale) DO UPDATE
            SET status = 'pending', updated_at = NOW()
            RETURNING id, version_id, locale, status, reviewer_id, review_notes,
                      reviewed_at, translation_score, created_at, created_by, updated_at
            "#,
        )
        .bind(input.version_id)
        .bind(&input.locale)
        .bind(input.created_by.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            version_id = %input.version_id,
            locale = %input.locale,
            "Locale review submitted"
        );

        Ok(review)
    }

    async fn start_review(&self, version_id: Uuid, locale: &str, reviewer_id: &str) -> Result<LocaleReview, AppError> {
        let review = sqlx::query_as::<_, LocaleReview>(
            r#"
            UPDATE content_locale_reviews
            SET status = 'in_review', reviewer_id = $3, updated_at = NOW()
            WHERE version_id = $1 AND locale = $2 AND status = 'pending'
            RETURNING id, version_id, locale, status, reviewer_id, review_notes,
                      reviewed_at, translation_score, created_at, created_by, updated_at
            "#,
        )
        .bind(version_id)
        .bind(locale)
        .bind(reviewer_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            version_id = %version_id,
            locale = %locale,
            reviewer = %reviewer_id,
            "Locale review started"
        );

        Ok(review)
    }

    async fn approve_locale(&self, input: ApproveLocaleInput) -> Result<LocaleReview, AppError> {
        let review = sqlx::query_as::<_, LocaleReview>(
            r#"
            UPDATE content_locale_reviews
            SET status = 'approved',
                reviewer_id = $3,
                review_notes = $4,
                translation_score = $5,
                reviewed_at = NOW(),
                updated_at = NOW()
            WHERE version_id = $1 AND locale = $2
            RETURNING id, version_id, locale, status, reviewer_id, review_notes,
                      reviewed_at, translation_score, created_at, created_by, updated_at
            "#,
        )
        .bind(input.version_id)
        .bind(&input.locale)
        .bind(&input.reviewer_id)
        .bind(input.review_notes.as_deref())
        .bind(input.translation_score)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            version_id = %input.version_id,
            locale = %input.locale,
            reviewer = %input.reviewer_id,
            score = ?input.translation_score,
            "Locale approved"
        );

        Ok(review)
    }

    async fn request_changes(&self, input: RequestChangesInput) -> Result<LocaleReview, AppError> {
        let review = sqlx::query_as::<_, LocaleReview>(
            r#"
            UPDATE content_locale_reviews
            SET status = 'changes_requested',
                reviewer_id = $3,
                review_notes = $4,
                reviewed_at = NOW(),
                updated_at = NOW()
            WHERE version_id = $1 AND locale = $2
            RETURNING id, version_id, locale, status, reviewer_id, review_notes,
                      reviewed_at, translation_score, created_at, created_by, updated_at
            "#,
        )
        .bind(input.version_id)
        .bind(&input.locale)
        .bind(&input.reviewer_id)
        .bind(&input.review_notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            version_id = %input.version_id,
            locale = %input.locale,
            reviewer = %input.reviewer_id,
            "Changes requested for locale"
        );

        Ok(review)
    }

    async fn reject_locale(&self, version_id: Uuid, locale: &str, reviewer_id: &str, reason: &str) -> Result<LocaleReview, AppError> {
        let review = sqlx::query_as::<_, LocaleReview>(
            r#"
            UPDATE content_locale_reviews
            SET status = 'rejected',
                reviewer_id = $3,
                review_notes = $4,
                reviewed_at = NOW(),
                updated_at = NOW()
            WHERE version_id = $1 AND locale = $2
            RETURNING id, version_id, locale, status, reviewer_id, review_notes,
                      reviewed_at, translation_score, created_at, created_by, updated_at
            "#,
        )
        .bind(version_id)
        .bind(locale)
        .bind(reviewer_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            version_id = %version_id,
            locale = %locale,
            reviewer = %reviewer_id,
            "Locale rejected"
        );

        Ok(review)
    }

    async fn get_reviews_for_version(&self, version_id: Uuid) -> Result<Vec<LocaleReview>, AppError> {
        let reviews = sqlx::query_as::<_, LocaleReview>(
            r#"
            SELECT id, version_id, locale, status, reviewer_id, review_notes,
                   reviewed_at, translation_score, created_at, created_by, updated_at
            FROM content_locale_reviews
            WHERE version_id = $1
            ORDER BY locale
            "#,
        )
        .bind(version_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(reviews)
    }

    async fn get_pending_by_locale(&self, locale: &str, limit: i32) -> Result<Vec<LocaleReview>, AppError> {
        let reviews = sqlx::query_as::<_, LocaleReview>(
            r#"
            SELECT id, version_id, locale, status, reviewer_id, review_notes,
                   reviewed_at, translation_score, created_at, created_by, updated_at
            FROM content_locale_reviews
            WHERE locale = $1 AND status = 'pending'
            ORDER BY created_at ASC
            LIMIT $2
            "#,
        )
        .bind(locale)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(reviews)
    }

    async fn get_reviews_for_reviewer(&self, reviewer_id: &str, limit: i32) -> Result<Vec<LocaleReview>, AppError> {
        let reviews = sqlx::query_as::<_, LocaleReview>(
            r#"
            SELECT id, version_id, locale, status, reviewer_id, review_notes,
                   reviewed_at, translation_score, created_at, created_by, updated_at
            FROM content_locale_reviews
            WHERE reviewer_id = $1 AND status IN ('in_review', 'pending')
            ORDER BY created_at ASC
            LIMIT $2
            "#,
        )
        .bind(reviewer_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(reviews)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_review_status() {
        assert_eq!(LocaleReviewStatus::Approved.as_str(), "approved");
        assert_eq!(LocaleReviewStatus::from_str("approved"), Some(LocaleReviewStatus::Approved));
    }
}
