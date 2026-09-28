//! FAQ repository for database operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{FaqItem, FaqCategory, AppError};

/// Dynamic type alias for FaqRepository trait object
pub type DynFaqRepository = Arc<dyn FaqRepository + Send + Sync>;

/// FAQ repository trait defining database operations
#[async_trait]
pub trait FaqRepository {
    /// Get all active FAQ items
    async fn get_all(&self) -> Result<Vec<FaqItem>, AppError>;

    /// Get FAQ items by category
    async fn get_by_category(&self, category: &str) -> Result<Vec<FaqItem>, AppError>;

    /// Get featured FAQ items
    async fn get_featured(&self) -> Result<Vec<FaqItem>, AppError>;

    /// Search FAQ items by query string (searches question and answer)
    async fn search(&self, query: &str) -> Result<Vec<FaqItem>, AppError>;

    /// Get all categories with item counts
    async fn get_categories(&self) -> Result<Vec<FaqCategory>, AppError>;

    /// Get a single FAQ item by ID
    async fn get_by_id(&self, id: i32) -> Result<Option<FaqItem>, AppError>;
}

/// Concrete implementation of FaqRepository
pub struct FaqRepositoryImpl {
    db_pool: ConnectionPool,
}

impl FaqRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl FaqRepository for FaqRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<FaqItem>, AppError> {
        let items = sqlx::query_as::<_, FaqItem>(
            r#"
            SELECT id, category, display_order, question_en, answer_en,
                   translations, is_featured, is_active, created_at, updated_at
            FROM faq_items
            WHERE is_active = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_by_category(&self, category: &str) -> Result<Vec<FaqItem>, AppError> {
        let items = sqlx::query_as::<_, FaqItem>(
            r#"
            SELECT id, category, display_order, question_en, answer_en,
                   translations, is_featured, is_active, created_at, updated_at
            FROM faq_items
            WHERE is_active = true AND category = $1
            ORDER BY display_order ASC, id ASC
            "#
        )
        .bind(category)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_featured(&self) -> Result<Vec<FaqItem>, AppError> {
        let items = sqlx::query_as::<_, FaqItem>(
            r#"
            SELECT id, category, display_order, question_en, answer_en,
                   translations, is_featured, is_active, created_at, updated_at
            FROM faq_items
            WHERE is_active = true AND is_featured = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn search(&self, query: &str) -> Result<Vec<FaqItem>, AppError> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        let items = sqlx::query_as::<_, FaqItem>(
            r#"
            SELECT id, category, display_order, question_en, answer_en,
                   translations, is_featured, is_active, created_at, updated_at
            FROM faq_items
            WHERE is_active = true
            AND (
                LOWER(question_en) LIKE $1
                OR LOWER(answer_en) LIKE $1
                OR translations::text ILIKE $1
            )
            ORDER BY
                CASE WHEN LOWER(question_en) LIKE $1 THEN 0 ELSE 1 END,
                display_order ASC,
                id ASC
            "#
        )
        .bind(&search_pattern)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_categories(&self) -> Result<Vec<FaqCategory>, AppError> {
        let categories = sqlx::query_as::<_, FaqCategory>(
            r#"
            SELECT category as name, COUNT(*) as count
            FROM faq_items
            WHERE is_active = true
            GROUP BY category
            ORDER BY count DESC, category ASC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(categories)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<FaqItem>, AppError> {
        let item = sqlx::query_as::<_, FaqItem>(
            r#"
            SELECT id, category, display_order, question_en, answer_en,
                   translations, is_featured, is_active, created_at, updated_at
            FROM faq_items
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(item)
    }
}
