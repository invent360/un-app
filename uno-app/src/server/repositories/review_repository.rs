//! Review repository for content approval workflow

use async_trait::async_trait;
use std::sync::Arc;
use chrono::{Duration, Utc};
use rand::Rng;
use crate::server::db::ConnectionPool;
use crate::types::{
    AppError, ContentReview, PreviewToken, ReviewStatus,
    ReviewWithContent, SubmitReviewRequest, ReviewDecisionRequest, ReviewDecision,
};

/// Dynamic type alias for ReviewRepository trait object
pub type DynReviewRepository = Arc<dyn ReviewRepository + Send + Sync>;

/// Review repository trait defining database operations
#[async_trait]
pub trait ReviewRepository {
    // Review operations

    /// Submit content for review
    async fn submit_for_review(&self, request: &SubmitReviewRequest) -> Result<ContentReview, AppError>;

    /// Get review by ID
    async fn get_review(&self, review_id: i32) -> Result<Option<ContentReview>, AppError>;

    /// Get review by version ID
    async fn get_review_by_version(&self, version_id: i32) -> Result<Option<ContentReview>, AppError>;

    /// Process review decision (approve, request changes, reject)
    async fn process_decision(&self, request: &ReviewDecisionRequest) -> Result<ContentReview, AppError>;

    /// Get pending reviews
    async fn get_pending_reviews(&self, limit: Option<i32>) -> Result<Vec<ReviewWithContent>, AppError>;

    /// Get reviews by submitter
    async fn get_reviews_by_submitter(&self, submitter: &str, limit: Option<i32>) -> Result<Vec<ReviewWithContent>, AppError>;

    // Preview token operations

    /// Create a preview token
    async fn create_preview_token(&self, version_id: i32, created_by: &str, expires_in_hours: i32) -> Result<PreviewToken, AppError>;

    /// Get preview token by token string
    async fn get_preview_token(&self, token: &str) -> Result<Option<PreviewToken>, AppError>;

    /// Validate and get preview token (returns None if expired)
    async fn validate_preview_token(&self, token: &str) -> Result<Option<PreviewToken>, AppError>;

    /// Delete expired tokens
    async fn cleanup_expired_tokens(&self) -> Result<i64, AppError>;
}

/// Concrete implementation of ReviewRepository
pub struct ReviewRepositoryImpl {
    db_pool: ConnectionPool,
}

impl ReviewRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }

    /// Generate a secure random token
    fn generate_token() -> String {
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        hex::encode(bytes)
    }
}

#[async_trait]
impl ReviewRepository for ReviewRepositoryImpl {
    async fn submit_for_review(&self, request: &SubmitReviewRequest) -> Result<ContentReview, AppError> {
        // Check if review already exists for this version
        if let Some(existing) = self.get_review_by_version(request.version_id).await? {
            if existing.status_enum() == Some(ReviewStatus::Pending) {
                return Err(AppError::BadRequest("Review already pending for this version".into()));
            }
        }

        let review = sqlx::query_as::<_, ContentReview>(
            r#"
            INSERT INTO content_reviews (version_id, status, submitted_by, submitted_notes)
            VALUES ($1, 'pending', $2, $3)
            ON CONFLICT (version_id) DO UPDATE
            SET status = 'pending',
                submitted_by = EXCLUDED.submitted_by,
                submitted_at = NOW(),
                submitted_notes = EXCLUDED.submitted_notes,
                reviewed_by = NULL,
                reviewed_at = NULL,
                review_notes = NULL
            RETURNING id, version_id, status, submitted_by, submitted_at, submitted_notes,
                      reviewed_by, reviewed_at, review_notes
            "#
        )
        .bind(request.version_id)
        .bind(&request.submitted_by)
        .bind(&request.notes)
        .fetch_one(&self.db_pool)
        .await?;

        // Update page_contents status to pending_review
        sqlx::query(
            r#"
            UPDATE page_contents pc
            SET status = 'pending_review'
            FROM content_versions cv
            WHERE cv.id = $1 AND pc.id = cv.content_id
            "#
        )
        .bind(request.version_id)
        .execute(&self.db_pool)
        .await?;

        Ok(review)
    }

    async fn get_review(&self, review_id: i32) -> Result<Option<ContentReview>, AppError> {
        let review = sqlx::query_as::<_, ContentReview>(
            r#"
            SELECT id, version_id, status, submitted_by, submitted_at, submitted_notes,
                   reviewed_by, reviewed_at, review_notes
            FROM content_reviews
            WHERE id = $1
            "#
        )
        .bind(review_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(review)
    }

    async fn get_review_by_version(&self, version_id: i32) -> Result<Option<ContentReview>, AppError> {
        let review = sqlx::query_as::<_, ContentReview>(
            r#"
            SELECT id, version_id, status, submitted_by, submitted_at, submitted_notes,
                   reviewed_by, reviewed_at, review_notes
            FROM content_reviews
            WHERE version_id = $1
            "#
        )
        .bind(version_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(review)
    }

    async fn process_decision(&self, request: &ReviewDecisionRequest) -> Result<ContentReview, AppError> {
        // Get the review first
        let review = self.get_review(request.review_id).await?
            .ok_or_else(|| AppError::NotFound("Review not found".into()))?;

        // Check it's pending
        if review.status_enum() != Some(ReviewStatus::Pending) {
            return Err(AppError::BadRequest("Review is not pending".into()));
        }

        // Check reviewer is not the submitter
        if review.submitted_by == request.reviewed_by {
            return Err(AppError::BadRequest("Cannot review your own submission".into()));
        }

        let new_status = match request.decision {
            ReviewDecision::Approve => "approved",
            ReviewDecision::RequestChanges => "changes_requested",
            ReviewDecision::Reject => "rejected",
        };

        let updated_review = sqlx::query_as::<_, ContentReview>(
            r#"
            UPDATE content_reviews
            SET status = $1::review_status, reviewed_by = $2, reviewed_at = NOW(), review_notes = $3
            WHERE id = $4
            RETURNING id, version_id, status, submitted_by, submitted_at, submitted_notes,
                      reviewed_by, reviewed_at, review_notes
            "#
        )
        .bind(new_status)
        .bind(&request.reviewed_by)
        .bind(&request.notes)
        .bind(request.review_id)
        .fetch_one(&self.db_pool)
        .await?;

        // Update page_contents status based on decision
        let content_status = match request.decision {
            ReviewDecision::Approve => "approved",
            ReviewDecision::RequestChanges => "draft",
            ReviewDecision::Reject => "draft",
        };

        sqlx::query(
            r#"
            UPDATE page_contents pc
            SET status = $1
            FROM content_versions cv
            WHERE cv.id = $2 AND pc.id = cv.content_id
            "#
        )
        .bind(content_status)
        .bind(review.version_id)
        .execute(&self.db_pool)
        .await?;

        Ok(updated_review)
    }

    async fn get_pending_reviews(&self, limit: Option<i32>) -> Result<Vec<ReviewWithContent>, AppError> {
        let limit = limit.unwrap_or(50);

        let reviews = sqlx::query_as::<_, ReviewWithContentRow>(
            r#"
            SELECT
                cr.id, cr.version_id, cr.status, cr.submitted_by, cr.submitted_at,
                cr.submitted_notes, cr.reviewed_by, cr.reviewed_at, cr.review_notes,
                pc.id as content_id, pc.content_type, pc.slug,
                pc.content->>'title' as title,
                cv.version, cv.change_summary
            FROM content_reviews cr
            JOIN content_versions cv ON cr.version_id = cv.id
            JOIN page_contents pc ON cv.content_id = pc.id
            WHERE cr.status = 'pending'
            ORDER BY cr.submitted_at ASC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(reviews.into_iter().map(|r| r.into()).collect())
    }

    async fn get_reviews_by_submitter(&self, submitter: &str, limit: Option<i32>) -> Result<Vec<ReviewWithContent>, AppError> {
        let limit = limit.unwrap_or(50);

        let reviews = sqlx::query_as::<_, ReviewWithContentRow>(
            r#"
            SELECT
                cr.id, cr.version_id, cr.status, cr.submitted_by, cr.submitted_at,
                cr.submitted_notes, cr.reviewed_by, cr.reviewed_at, cr.review_notes,
                pc.id as content_id, pc.content_type, pc.slug,
                pc.content->>'title' as title,
                cv.version, cv.change_summary
            FROM content_reviews cr
            JOIN content_versions cv ON cr.version_id = cv.id
            JOIN page_contents pc ON cv.content_id = pc.id
            WHERE cr.submitted_by = $1
            ORDER BY cr.submitted_at DESC
            LIMIT $2
            "#
        )
        .bind(submitter)
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(reviews.into_iter().map(|r| r.into()).collect())
    }

    async fn create_preview_token(&self, version_id: i32, created_by: &str, expires_in_hours: i32) -> Result<PreviewToken, AppError> {
        let token = Self::generate_token();
        let expires_at = Utc::now() + Duration::hours(expires_in_hours as i64);

        let preview_token = sqlx::query_as::<_, PreviewToken>(
            r#"
            INSERT INTO preview_tokens (version_id, token, created_by, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, version_id, token, created_by, created_at, expires_at
            "#
        )
        .bind(version_id)
        .bind(&token)
        .bind(created_by)
        .bind(expires_at)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(preview_token)
    }

    async fn get_preview_token(&self, token: &str) -> Result<Option<PreviewToken>, AppError> {
        let preview_token = sqlx::query_as::<_, PreviewToken>(
            r#"
            SELECT id, version_id, token, created_by, created_at, expires_at
            FROM preview_tokens
            WHERE token = $1
            "#
        )
        .bind(token)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(preview_token)
    }

    async fn validate_preview_token(&self, token: &str) -> Result<Option<PreviewToken>, AppError> {
        let preview_token = sqlx::query_as::<_, PreviewToken>(
            r#"
            SELECT id, version_id, token, created_by, created_at, expires_at
            FROM preview_tokens
            WHERE token = $1 AND expires_at > NOW()
            "#
        )
        .bind(token)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(preview_token)
    }

    async fn cleanup_expired_tokens(&self) -> Result<i64, AppError> {
        let result = sqlx::query(
            "DELETE FROM preview_tokens WHERE expires_at < NOW()"
        )
        .execute(&self.db_pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }
}

/// Internal row type for joining review with content
#[derive(Debug, sqlx::FromRow)]
struct ReviewWithContentRow {
    // Review fields
    id: i32,
    version_id: i32,
    status: String,
    submitted_by: String,
    submitted_at: chrono::DateTime<Utc>,
    submitted_notes: Option<String>,
    reviewed_by: Option<String>,
    reviewed_at: Option<chrono::DateTime<Utc>>,
    review_notes: Option<String>,
    // Content fields
    content_id: i32,
    content_type: String,
    slug: String,
    title: Option<String>,
    version: i32,
    change_summary: Option<String>,
}

impl From<ReviewWithContentRow> for ReviewWithContent {
    fn from(row: ReviewWithContentRow) -> Self {
        ReviewWithContent {
            review: ContentReview {
                id: row.id,
                version_id: row.version_id,
                status: row.status,
                submitted_by: row.submitted_by,
                submitted_at: row.submitted_at,
                submitted_notes: row.submitted_notes,
                reviewed_by: row.reviewed_by,
                reviewed_at: row.reviewed_at,
                review_notes: row.review_notes,
            },
            content_id: row.content_id,
            content_type: row.content_type,
            slug: row.slug,
            title: row.title.unwrap_or_else(|| "Untitled".to_string()),
            version: row.version,
            change_summary: row.change_summary,
        }
    }
}
