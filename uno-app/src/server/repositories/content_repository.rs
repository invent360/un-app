//! Content repository for database operations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::server::db::ConnectionPool;
use crate::types::{PageContent, ContentVersion, AppError, ContentListParams, UpsertContentRequest};

/// Dynamic type alias for ContentRepository trait object
pub type DynContentRepository = Arc<dyn ContentRepository + Send + Sync>;

/// Content repository trait defining database operations
#[async_trait]
pub trait ContentRepository {
    // Public operations

    /// Get all published content by type
    async fn get_published_by_type(&self, content_type: &str) -> Result<Vec<PageContent>, AppError>;

    /// Get single published content by slug
    async fn get_published_by_slug(&self, content_type: &str, slug: &str) -> Result<Option<PageContent>, AppError>;

    /// Get featured content by type
    async fn get_featured_by_type(&self, content_type: &str) -> Result<Vec<PageContent>, AppError>;

    /// Search content by query string
    async fn search(&self, content_type: &str, query: &str) -> Result<Vec<PageContent>, AppError>;

    // Admin operations

    /// Get content by ID (any status)
    async fn get_by_id(&self, id: i32) -> Result<Option<PageContent>, AppError>;

    /// List all content with filters
    async fn list(&self, params: &ContentListParams) -> Result<(Vec<PageContent>, i64), AppError>;

    /// Create new content
    async fn create(&self, request: &UpsertContentRequest, created_by: Option<&str>) -> Result<PageContent, AppError>;

    /// Update existing content (creates version)
    async fn update(&self, id: i32, request: &UpsertContentRequest, updated_by: Option<&str>) -> Result<PageContent, AppError>;

    /// Delete content (soft delete)
    async fn delete(&self, id: i32) -> Result<(), AppError>;

    /// Publish content
    async fn publish(&self, id: i32, published_by: Option<&str>) -> Result<PageContent, AppError>;

    /// Archive content
    async fn archive(&self, id: i32) -> Result<PageContent, AppError>;

    /// Get version history
    async fn get_versions(&self, content_id: i32) -> Result<Vec<ContentVersion>, AppError>;

    /// Get specific version
    async fn get_version(&self, content_id: i32, version: i32) -> Result<Option<ContentVersion>, AppError>;

    /// Get version by version ID (primary key)
    async fn get_version_by_id(&self, version_id: i32) -> Result<Option<ContentVersion>, AppError>;

    /// Revert to version
    async fn revert_to_version(&self, content_id: i32, version: i32, reverted_by: Option<&str>) -> Result<PageContent, AppError>;

    // Scheduling operations

    /// Update content schedule
    async fn update_schedule(
        &self,
        id: i32,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
    ) -> Result<PageContent, AppError>;

    /// Get contents due for publishing
    async fn get_pending_publish(&self) -> Result<Vec<PageContent>, AppError>;

    /// Get contents due for unpublishing
    async fn get_pending_unpublish(&self) -> Result<Vec<PageContent>, AppError>;
}

/// Concrete implementation of ContentRepository
pub struct ContentRepositoryImpl {
    db_pool: ConnectionPool,
}

impl ContentRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }

    /// Compute translation status from translations JSON
    fn compute_translation_status(
        content_type: &str,
        content: &serde_json::Value,
        translations: &Option<serde_json::Value>,
    ) -> serde_json::Value {
        use crate::types::{ContentType, SUPPORTED_LOCALES};

        let required_fields = ContentType::from_str(content_type)
            .map(|ct| ct.required_fields())
            .unwrap_or(&["title", "description"]);

        let mut status = serde_json::Map::new();

        // English is always complete (it's the base)
        status.insert("en".to_string(), serde_json::json!("complete"));

        // Check each supported locale
        for locale in SUPPORTED_LOCALES.iter().skip(1) {
            let locale_status = if let Some(trans) = translations {
                if let Some(locale_trans) = trans.get(*locale) {
                    let translated_count = required_fields.iter()
                        .filter(|field| {
                            locale_trans.get(*field)
                                .and_then(|v| v.as_str())
                                .map(|s| !s.is_empty())
                                .unwrap_or(false)
                        })
                        .count();

                    if translated_count == required_fields.len() {
                        "complete"
                    } else if translated_count > 0 {
                        "partial"
                    } else {
                        "missing"
                    }
                } else {
                    "missing"
                }
            } else {
                "missing"
            };

            status.insert(locale.to_string(), serde_json::json!(locale_status));
        }

        serde_json::Value::Object(status)
    }
}

#[async_trait]
impl ContentRepository for ContentRepositoryImpl {
    async fn get_published_by_type(&self, content_type: &str) -> Result<Vec<PageContent>, AppError> {
        let items = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE content_type = $1 AND status = 'published' AND is_active = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .bind(content_type)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_published_by_slug(&self, content_type: &str, slug: &str) -> Result<Option<PageContent>, AppError> {
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE content_type = $1 AND slug = $2 AND status = 'published' AND is_active = true
            "#
        )
        .bind(content_type)
        .bind(slug)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn get_featured_by_type(&self, content_type: &str) -> Result<Vec<PageContent>, AppError> {
        let items = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE content_type = $1 AND status = 'published' AND is_active = true AND is_featured = true
            ORDER BY display_order ASC, id ASC
            "#
        )
        .bind(content_type)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn search(&self, content_type: &str, query: &str) -> Result<Vec<PageContent>, AppError> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        let items = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE content_type = $1 AND status = 'published' AND is_active = true
            AND (
                content->>'title' ILIKE $2
                OR content->>'description' ILIKE $2
                OR translations::text ILIKE $2
            )
            ORDER BY display_order ASC, id ASC
            "#
        )
        .bind(content_type)
        .bind(&search_pattern)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<PageContent>, AppError> {
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn list(&self, params: &ContentListParams) -> Result<(Vec<PageContent>, i64), AppError> {
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(20).min(100);
        let offset = ((page - 1) * per_page) as i64;

        // Build query dynamically based on params
        let include_drafts = params.include_drafts.unwrap_or(true);

        let (items, total) = if let Some(ref content_type) = params.content_type {
            if let Some(ref status) = params.status {
                // Filter by type and status
                let items = sqlx::query_as::<_, PageContent>(
                    r#"
                    SELECT id, content_type, slug, status, content,
                           COALESCE(translations, '{}'::jsonb) as translations,
                           COALESCE(translation_status, '{}'::jsonb) as translation_status,
                           display_order, is_featured, is_active, version,
                           published_version, published_at, published_by,
                           publish_at, unpublish_at,
                           created_at, updated_at, created_by, updated_by
                    FROM page_contents
                    WHERE is_active = true AND content_type = $1 AND status = $2
                    ORDER BY content_type, display_order ASC, id ASC
                    LIMIT $3 OFFSET $4
                    "#
                )
                .bind(content_type)
                .bind(status)
                .bind(per_page as i64)
                .bind(offset)
                .fetch_all(&self.db_pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM page_contents WHERE is_active = true AND content_type = $1 AND status = $2"
                )
                .bind(content_type)
                .bind(status)
                .fetch_one(&self.db_pool)
                .await?;

                (items, total.0)
            } else if include_drafts {
                // Filter by type only, include all statuses
                let items = sqlx::query_as::<_, PageContent>(
                    r#"
                    SELECT id, content_type, slug, status, content,
                           COALESCE(translations, '{}'::jsonb) as translations,
                           COALESCE(translation_status, '{}'::jsonb) as translation_status,
                           display_order, is_featured, is_active, version,
                           published_version, published_at, published_by,
                           publish_at, unpublish_at,
                           created_at, updated_at, created_by, updated_by
                    FROM page_contents
                    WHERE is_active = true AND content_type = $1
                    ORDER BY display_order ASC, id ASC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(content_type)
                .bind(per_page as i64)
                .bind(offset)
                .fetch_all(&self.db_pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM page_contents WHERE is_active = true AND content_type = $1"
                )
                .bind(content_type)
                .fetch_one(&self.db_pool)
                .await?;

                (items, total.0)
            } else {
                // Filter by type, published only
                let items = sqlx::query_as::<_, PageContent>(
                    r#"
                    SELECT id, content_type, slug, status, content,
                           COALESCE(translations, '{}'::jsonb) as translations,
                           COALESCE(translation_status, '{}'::jsonb) as translation_status,
                           display_order, is_featured, is_active, version,
                           published_version, published_at, published_by,
                           publish_at, unpublish_at,
                           created_at, updated_at, created_by, updated_by
                    FROM page_contents
                    WHERE is_active = true AND content_type = $1 AND status = 'published'
                    ORDER BY display_order ASC, id ASC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(content_type)
                .bind(per_page as i64)
                .bind(offset)
                .fetch_all(&self.db_pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM page_contents WHERE is_active = true AND content_type = $1 AND status = 'published'"
                )
                .bind(content_type)
                .fetch_one(&self.db_pool)
                .await?;

                (items, total.0)
            }
        } else if include_drafts {
            // All types, all statuses
            let items = sqlx::query_as::<_, PageContent>(
                r#"
                SELECT id, content_type, slug, status, content,
                       COALESCE(translations, '{}'::jsonb) as translations,
                       COALESCE(translation_status, '{}'::jsonb) as translation_status,
                       display_order, is_featured, is_active, version,
                       published_version, published_at, published_by,
                       publish_at, unpublish_at,
                       created_at, updated_at, created_by, updated_by
                FROM page_contents
                WHERE is_active = true
                ORDER BY content_type, display_order ASC, id ASC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(per_page as i64)
            .bind(offset)
            .fetch_all(&self.db_pool)
            .await?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM page_contents WHERE is_active = true"
            )
            .fetch_one(&self.db_pool)
            .await?;

            (items, total.0)
        } else {
            // All types, published only
            let items = sqlx::query_as::<_, PageContent>(
                r#"
                SELECT id, content_type, slug, status, content,
                       COALESCE(translations, '{}'::jsonb) as translations,
                       COALESCE(translation_status, '{}'::jsonb) as translation_status,
                       display_order, is_featured, is_active, version,
                       published_version, published_at, published_by,
                       publish_at, unpublish_at,
                       created_at, updated_at, created_by, updated_by
                FROM page_contents
                WHERE is_active = true AND status = 'published'
                ORDER BY content_type, display_order ASC, id ASC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(per_page as i64)
            .bind(offset)
            .fetch_all(&self.db_pool)
            .await?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM page_contents WHERE is_active = true AND status = 'published'"
            )
            .fetch_one(&self.db_pool)
            .await?;

            (items, total.0)
        };

        Ok((items, total))
    }

    async fn create(&self, request: &UpsertContentRequest, created_by: Option<&str>) -> Result<PageContent, AppError> {
        // Compute translation status
        let translation_status = Self::compute_translation_status(
            &request.content_type,
            &request.content,
            &request.translations,
        );

        let item = sqlx::query_as::<_, PageContent>(
            r#"
            INSERT INTO page_contents (
                content_type, slug, content, translations, translation_status,
                display_order, is_featured, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            RETURNING id, content_type, slug, status, content,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      display_order, is_featured, is_active, version,
                      published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(&request.content_type)
        .bind(&request.slug)
        .bind(&request.content)
        .bind(&request.translations)
        .bind(&translation_status)
        .bind(request.display_order.unwrap_or(0))
        .bind(request.is_featured.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.db_pool)
        .await?;

        // Create initial version
        sqlx::query(
            r#"
            INSERT INTO content_versions (content_id, version, content, translations, change_summary, created_by)
            VALUES ($1, 1, $2, $3, 'Initial creation', $4)
            "#
        )
        .bind(item.id)
        .bind(&request.content)
        .bind(&request.translations)
        .bind(created_by)
        .execute(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn update(&self, id: i32, request: &UpsertContentRequest, updated_by: Option<&str>) -> Result<PageContent, AppError> {
        // Get current version
        let current = self.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content not found".into()))?;

        let new_version = current.version + 1;

        // Compute translation status
        let translation_status = Self::compute_translation_status(
            &request.content_type,
            &request.content,
            &request.translations,
        );

        // Update content - set status back to draft if it was published
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            UPDATE page_contents
            SET content = $1, translations = $2, translation_status = $3,
                display_order = COALESCE($4, display_order),
                is_featured = COALESCE($5, is_featured), version = $6,
                status = CASE WHEN status = 'published' THEN 'draft' ELSE status END,
                updated_by = $7, updated_at = NOW()
            WHERE id = $8
            RETURNING id, content_type, slug, status, content,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      display_order, is_featured, is_active, version,
                      published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(&request.content)
        .bind(&request.translations)
        .bind(&translation_status)
        .bind(request.display_order)
        .bind(request.is_featured)
        .bind(new_version)
        .bind(updated_by)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        // Create version record
        sqlx::query(
            r#"
            INSERT INTO content_versions (content_id, version, content, translations, change_summary, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(id)
        .bind(new_version)
        .bind(&request.content)
        .bind(&request.translations)
        .bind(&request.change_summary)
        .bind(updated_by)
        .execute(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE page_contents SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn publish(&self, id: i32, published_by: Option<&str>) -> Result<PageContent, AppError> {
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            UPDATE page_contents
            SET status = 'published', published_version = version,
                published_at = NOW(), published_by = $1
            WHERE id = $2
            RETURNING id, content_type, slug, status, content,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      display_order, is_featured, is_active, version,
                      published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(published_by)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn archive(&self, id: i32) -> Result<PageContent, AppError> {
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            UPDATE page_contents
            SET status = 'archived'
            WHERE id = $1
            RETURNING id, content_type, slug, status, content,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      display_order, is_featured, is_active, version,
                      published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn get_versions(&self, content_id: i32) -> Result<Vec<ContentVersion>, AppError> {
        let versions = sqlx::query_as::<_, ContentVersion>(
            r#"
            SELECT id, content_id, version, content, translations,
                   change_summary, created_at, created_by
            FROM content_versions
            WHERE content_id = $1
            ORDER BY version DESC
            "#
        )
        .bind(content_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(versions)
    }

    async fn get_version(&self, content_id: i32, version: i32) -> Result<Option<ContentVersion>, AppError> {
        let ver = sqlx::query_as::<_, ContentVersion>(
            r#"
            SELECT id, content_id, version, content, translations,
                   change_summary, created_at, created_by
            FROM content_versions
            WHERE content_id = $1 AND version = $2
            "#
        )
        .bind(content_id)
        .bind(version)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(ver)
    }

    async fn get_version_by_id(&self, version_id: i32) -> Result<Option<ContentVersion>, AppError> {
        let ver = sqlx::query_as::<_, ContentVersion>(
            r#"
            SELECT id, content_id, version, content, translations,
                   change_summary, created_at, created_by
            FROM content_versions
            WHERE id = $1
            "#
        )
        .bind(version_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(ver)
    }

    async fn revert_to_version(&self, content_id: i32, version: i32, reverted_by: Option<&str>) -> Result<PageContent, AppError> {
        // Get the version to revert to
        let old_version = self.get_version(content_id, version).await?
            .ok_or_else(|| AppError::NotFound("Version not found".into()))?;

        // Get current content for content_type
        let current = self.get_by_id(content_id).await?
            .ok_or_else(|| AppError::NotFound("Content not found".into()))?;

        // Create update request from old version
        let request = UpsertContentRequest {
            content_type: current.content_type.clone(),
            slug: current.slug.clone(),
            content: old_version.content,
            translations: old_version.translations,
            display_order: None,
            is_featured: None,
            change_summary: Some(format!("Reverted to version {}", version)),
        };

        self.update(content_id, &request, reverted_by).await
    }

    // Scheduling implementations

    async fn update_schedule(
        &self,
        id: i32,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
    ) -> Result<PageContent, AppError> {
        let item = sqlx::query_as::<_, PageContent>(
            r#"
            UPDATE page_contents
            SET publish_at = $2, unpublish_at = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, content_type, slug, status, content,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      display_order, is_featured, is_active, version,
                      published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(id)
        .bind(publish_at)
        .bind(unpublish_at)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn get_pending_publish(&self) -> Result<Vec<PageContent>, AppError> {
        let items = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE publish_at IS NOT NULL
              AND publish_at <= NOW()
              AND status IN ('draft', 'approved')
              AND is_active = true
            ORDER BY publish_at ASC
            LIMIT 100
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_pending_unpublish(&self) -> Result<Vec<PageContent>, AppError> {
        let items = sqlx::query_as::<_, PageContent>(
            r#"
            SELECT id, content_type, slug, status, content,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   display_order, is_featured, is_active, version,
                   published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM page_contents
            WHERE unpublish_at IS NOT NULL
              AND unpublish_at <= NOW()
              AND status = 'published'
              AND is_active = true
            ORDER BY unpublish_at ASC
            LIMIT 100
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }
}
