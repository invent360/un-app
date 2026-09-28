//! Testimonials repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::server::db::ConnectionPool;
use crate::types::AppError;

/// Testimonial data from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Testimonial {
    pub id: i32,
    pub quote: String,
    pub author_name: String,
    pub author_location: Option<String>,
    pub author_avatar: Option<String>,
    pub rating: i16,
    pub is_featured: bool,
    pub is_active: bool,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Testimonial {
    /// Convert to JSON value for embedding in home section data
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "quote": self.quote,
            "author_name": self.author_name,
            "author_location": self.author_location.clone().unwrap_or_default(),
            "author_avatar": self.author_avatar.clone(),
            "rating": self.rating,
            "is_featured": self.is_featured
        })
    }
}

/// Dynamic type alias for TestimonialRepository trait object
pub type DynTestimonialRepository = Arc<dyn TestimonialRepository + Send + Sync>;

/// Testimonial repository trait defining database operations
#[async_trait]
pub trait TestimonialRepository {
    /// Get all active testimonials ordered by display_order
    async fn get_active(&self) -> Result<Vec<Testimonial>, AppError>;

    /// Get featured testimonials
    async fn get_featured(&self) -> Result<Vec<Testimonial>, AppError>;

    /// Get testimonials with limit
    async fn get_active_with_limit(&self, limit: i64) -> Result<Vec<Testimonial>, AppError>;
}

/// Concrete implementation of TestimonialRepository
pub struct TestimonialRepositoryImpl {
    db_pool: ConnectionPool,
}

impl TestimonialRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TestimonialRepository for TestimonialRepositoryImpl {
    async fn get_active(&self) -> Result<Vec<Testimonial>, AppError> {
        let items = sqlx::query_as::<_, Testimonial>(
            r#"
            SELECT id, quote, author_name, author_location, author_avatar,
                   rating, is_featured, is_active, display_order, created_at, updated_at
            FROM testimonials
            WHERE is_active = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_featured(&self) -> Result<Vec<Testimonial>, AppError> {
        let items = sqlx::query_as::<_, Testimonial>(
            r#"
            SELECT id, quote, author_name, author_location, author_avatar,
                   rating, is_featured, is_active, display_order, created_at, updated_at
            FROM testimonials
            WHERE is_active = true AND is_featured = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_active_with_limit(&self, limit: i64) -> Result<Vec<Testimonial>, AppError> {
        let items = sqlx::query_as::<_, Testimonial>(
            r#"
            SELECT id, quote, author_name, author_location, author_avatar,
                   rating, is_featured, is_active, display_order, created_at, updated_at
            FROM testimonials
            WHERE is_active = true
            ORDER BY display_order ASC, id ASC
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }
}
