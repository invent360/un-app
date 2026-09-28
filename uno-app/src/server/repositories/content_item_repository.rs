//! Content Item repository for database operations on generic content

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;
use crate::server::db::ConnectionPool;
use crate::types::{
    AppError, ContentItem, ContentItemStatus, ContentItemVersion,
    ContentItemListParams, ContentItemSummary, ContentSchema,
    UpsertContentItemRequest, SUPPORTED_LOCALES,
};

/// Dynamic type alias for ContentItemRepository trait object
pub type DynContentItemRepository = Arc<dyn ContentItemRepository + Send + Sync>;

/// Content item repository trait defining database operations
#[async_trait]
pub trait ContentItemRepository {
    // ==========================================
    // Public operations (for uno-app consumption)
    // ==========================================

    /// Get all published content by schema type
    async fn get_published_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError>;

    /// Get single published content by slug
    async fn get_published_by_slug(&self, schema_id: &str, slug: &str) -> Result<Option<ContentItem>, AppError>;

    /// Get featured content by schema type
    async fn get_featured_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError>;

    /// Search content by query string
    async fn search(&self, schema_id: &str, query: &str) -> Result<Vec<ContentItem>, AppError>;

    // ==========================================
    // Admin operations
    // ==========================================

    /// Get content by UUID
    async fn get_by_id(&self, id: Uuid) -> Result<Option<ContentItem>, AppError>;

    /// List content with filters and pagination
    async fn list(&self, params: &ContentItemListParams) -> Result<(Vec<ContentItem>, i64), AppError>;

    /// Create new content item
    async fn create(&self, request: &UpsertContentItemRequest, created_by: Option<&str>) -> Result<ContentItem, AppError>;

    /// Update existing content item
    async fn update(&self, id: Uuid, request: &UpsertContentItemRequest, updated_by: Option<&str>) -> Result<ContentItem, AppError>;

    /// Delete content (soft delete)
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    /// Hard delete content (permanent)
    async fn hard_delete(&self, id: Uuid) -> Result<(), AppError>;

    /// Update content status
    async fn update_status(&self, id: Uuid, status: ContentItemStatus) -> Result<ContentItem, AppError>;

    /// Publish content
    async fn publish(&self, id: Uuid, published_by: Option<&str>) -> Result<ContentItem, AppError>;

    /// Archive content
    async fn archive(&self, id: Uuid) -> Result<ContentItem, AppError>;

    // ==========================================
    // Version operations
    // ==========================================

    /// Get version history for content
    async fn get_versions(&self, content_id: Uuid) -> Result<Vec<ContentItemVersion>, AppError>;

    /// Get specific version
    async fn get_version(&self, content_id: Uuid, version_number: i32) -> Result<Option<ContentItemVersion>, AppError>;

    /// Get version by UUID
    async fn get_version_by_id(&self, version_id: Uuid) -> Result<Option<ContentItemVersion>, AppError>;

    /// Revert to specific version
    async fn revert_to_version(&self, content_id: Uuid, version_number: i32, reverted_by: Option<&str>) -> Result<ContentItem, AppError>;

    // ==========================================
    // Scheduling operations
    // ==========================================

    /// Update publish/unpublish schedule
    async fn update_schedule(
        &self,
        id: Uuid,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
    ) -> Result<ContentItem, AppError>;

    /// Get content due for publishing
    async fn get_pending_publish(&self) -> Result<Vec<ContentItem>, AppError>;

    /// Get content due for unpublishing
    async fn get_pending_unpublish(&self) -> Result<Vec<ContentItem>, AppError>;

    // ==========================================
    // Bulk operations
    // ==========================================

    /// Reorder content items
    async fn reorder(&self, schema_id: &str, order: Vec<(Uuid, i32)>) -> Result<(), AppError>;

    /// Get content summaries for list view (with schema awareness)
    async fn get_summaries(&self, params: &ContentItemListParams, schema: &ContentSchema) -> Result<(Vec<ContentItemSummary>, i64), AppError>;

    /// Update translations for a content item
    async fn update_translations(
        &self,
        id: Uuid,
        translations: serde_json::Value,
        translation_status: serde_json::Value,
        updated_by: Option<&str>,
    ) -> Result<ContentItem, AppError>;
}

/// Concrete implementation of ContentItemRepository
pub struct ContentItemRepositoryImpl {
    db_pool: ConnectionPool,
}

impl ContentItemRepositoryImpl {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }

    /// Compute translation status from translations JSON
    fn compute_translation_status(
        schema: &ContentSchema,
        data: &serde_json::Value,
        translations: &Option<serde_json::Value>,
    ) -> serde_json::Value {
        let translatable_fields: Vec<&str> = schema.translatable_fields();

        let mut status = serde_json::Map::new();

        // English is always complete (it's the base)
        status.insert("en".to_string(), serde_json::json!("complete"));

        // Check each supported locale
        for locale in SUPPORTED_LOCALES.iter().skip(1) {
            let locale_status = if let Some(trans) = translations {
                if let Some(locale_trans) = trans.get(*locale) {
                    let translated_count = translatable_fields.iter()
                        .filter(|field| {
                            locale_trans.get(*field)
                                .and_then(|v| v.as_str())
                                .map(|s| !s.is_empty())
                                .unwrap_or(false)
                        })
                        .count();

                    if translatable_fields.is_empty() {
                        "complete"
                    } else if translated_count == translatable_fields.len() {
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

    /// Calculate translation coverage percentage
    fn calculate_translation_coverage(translation_status: &Option<serde_json::Value>) -> i32 {
        let mut complete_count = 1; // English is always complete

        if let Some(ref ts) = translation_status {
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

    /// Generate slug from title field
    fn generate_slug(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[async_trait]
impl ContentItemRepository for ContentItemRepositoryImpl {
    // ==========================================
    // Public operations
    // ==========================================

    async fn get_published_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError> {
        let items = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
            WHERE schema_id = $1 AND status = 'published' AND is_active = true
            ORDER BY display_order ASC, created_at ASC
            "#
        )
        .bind(schema_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn get_published_by_slug(&self, schema_id: &str, slug: &str) -> Result<Option<ContentItem>, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
            WHERE schema_id = $1 AND slug = $2 AND status = 'published' AND is_active = true
            "#
        )
        .bind(schema_id)
        .bind(slug)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn get_featured_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError> {
        let items = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
            WHERE schema_id = $1 AND status = 'published' AND is_active = true AND is_featured = true
            ORDER BY display_order ASC, created_at ASC
            "#
        )
        .bind(schema_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    async fn search(&self, schema_id: &str, query: &str) -> Result<Vec<ContentItem>, AppError> {
        let search_pattern = format!("%{}%", query.to_lowercase());

        let items = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
            WHERE schema_id = $1 AND status = 'published' AND is_active = true
            AND (
                data::text ILIKE $2
                OR translations::text ILIKE $2
            )
            ORDER BY display_order ASC, created_at ASC
            "#
        )
        .bind(schema_id)
        .bind(&search_pattern)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(items)
    }

    // ==========================================
    // Admin operations
    // ==========================================

    async fn get_by_id(&self, id: Uuid) -> Result<Option<ContentItem>, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn list(&self, params: &ContentItemListParams) -> Result<(Vec<ContentItem>, i64), AppError> {
        let page = params.page.max(1);
        let per_page = params.per_page.min(100).max(1);
        let offset = ((page - 1) * per_page) as i64;

        // Build dynamic query based on params
        let mut conditions = vec!["is_active = true".to_string()];
        let mut bind_index = 1;

        if let Some(ref schema_id) = params.schema_id {
            conditions.push(format!("schema_id = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(ref status) = params.status {
            conditions.push(format!("status = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(is_featured) = params.is_featured {
            conditions.push(format!("is_featured = ${}", bind_index));
            bind_index += 1;
        }

        if let Some(ref search) = params.search {
            conditions.push(format!("(data::text ILIKE ${0} OR translations::text ILIKE ${0})", bind_index));
            bind_index += 1;
        }

        let where_clause = conditions.join(" AND ");

        // This is a simplified version - in production you'd use a query builder
        // For now, we'll handle the common cases
        let (items, total) = if let Some(ref schema_id) = params.schema_id {
            if let Some(ref status) = params.status {
                let items = sqlx::query_as::<_, ContentItem>(
                    &format!(
                        r#"
                        SELECT id, schema_id, slug, display_order, status, version,
                               data,
                               COALESCE(translations, '{{}}'::jsonb) as translations,
                               COALESCE(translation_status, '{{}}'::jsonb) as translation_status,
                               is_featured, is_active, published_version, published_at, published_by,
                               publish_at, unpublish_at,
                               created_at, updated_at, created_by, updated_by
                        FROM content_items
                        WHERE schema_id = $1 AND status = $2 AND is_active = true
                        ORDER BY display_order ASC, created_at DESC
                        LIMIT $3 OFFSET $4
                        "#
                    )
                )
                .bind(schema_id)
                .bind(status.as_str())
                .bind(per_page as i64)
                .bind(offset)
                .fetch_all(&self.db_pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM content_items WHERE schema_id = $1 AND status = $2 AND is_active = true"
                )
                .bind(schema_id)
                .bind(status.as_str())
                .fetch_one(&self.db_pool)
                .await?;

                (items, total.0)
            } else {
                // Schema only
                let items = sqlx::query_as::<_, ContentItem>(
                    r#"
                    SELECT id, schema_id, slug, display_order, status, version,
                           data,
                           COALESCE(translations, '{}'::jsonb) as translations,
                           COALESCE(translation_status, '{}'::jsonb) as translation_status,
                           is_featured, is_active, published_version, published_at, published_by,
                           publish_at, unpublish_at,
                           created_at, updated_at, created_by, updated_by
                    FROM content_items
                    WHERE schema_id = $1 AND is_active = true
                    ORDER BY display_order ASC, created_at DESC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(schema_id)
                .bind(per_page as i64)
                .bind(offset)
                .fetch_all(&self.db_pool)
                .await?;

                let total: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM content_items WHERE schema_id = $1 AND is_active = true"
                )
                .bind(schema_id)
                .fetch_one(&self.db_pool)
                .await?;

                (items, total.0)
            }
        } else {
            // All schemas
            let items = sqlx::query_as::<_, ContentItem>(
                r#"
                SELECT id, schema_id, slug, display_order, status, version,
                       data,
                       COALESCE(translations, '{}'::jsonb) as translations,
                       COALESCE(translation_status, '{}'::jsonb) as translation_status,
                       is_featured, is_active, published_version, published_at, published_by,
                       publish_at, unpublish_at,
                       created_at, updated_at, created_by, updated_by
                FROM content_items
                WHERE is_active = true
                ORDER BY schema_id, display_order ASC, created_at DESC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(per_page as i64)
            .bind(offset)
            .fetch_all(&self.db_pool)
            .await?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM content_items WHERE is_active = true"
            )
            .fetch_one(&self.db_pool)
            .await?;

            (items, total.0)
        };

        Ok((items, total))
    }

    async fn create(&self, request: &UpsertContentItemRequest, created_by: Option<&str>) -> Result<ContentItem, AppError> {
        // Generate slug if not provided
        let slug = request.slug.clone().or_else(|| {
            request.data.get("title")
                .and_then(|t| t.as_str())
                .map(Self::generate_slug)
        });

        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            INSERT INTO content_items (
                schema_id, slug, data, translations,
                display_order, is_featured, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(&request.schema_id)
        .bind(&slug)
        .bind(&request.data)
        .bind(&request.translations)
        .bind(request.display_order.unwrap_or(0))
        .bind(request.is_featured.unwrap_or(false))
        .bind(created_by)
        .fetch_one(&self.db_pool)
        .await?;

        // Create initial version
        sqlx::query(
            r#"
            INSERT INTO content_item_versions (content_id, version_number, data, translations, change_summary, created_by)
            VALUES ($1, 1, $2, $3, 'Initial creation', $4)
            "#
        )
        .bind(item.id)
        .bind(&request.data)
        .bind(&request.translations)
        .bind(created_by)
        .execute(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn update(&self, id: Uuid, request: &UpsertContentItemRequest, updated_by: Option<&str>) -> Result<ContentItem, AppError> {
        tracing::info!("Updating content item {} with slug: {:?}", id, request.slug);

        // Get current item
        let current = self.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        // Only bump version if content was already published (has a published_version)
        // Draft content edits should not increment version
        let should_bump_version = current.published_version.is_some();
        let new_version = if should_bump_version {
            current.version + 1
        } else {
            current.version
        };

        // Update item - reset status to draft if it was published
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET data = $1, translations = $2,
                display_order = COALESCE($3, display_order),
                is_featured = COALESCE($4, is_featured),
                version = $5,
                status = CASE WHEN status = 'published' THEN 'draft'::content_status ELSE status END,
                updated_by = $6, updated_at = NOW(),
                slug = COALESCE($8, slug)
            WHERE id = $7
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(&request.data)
        .bind(&request.translations)
        .bind(request.display_order)
        .bind(request.is_featured)
        .bind(new_version)
        .bind(updated_by)
        .bind(id)
        .bind(&request.slug)
        .fetch_one(&self.db_pool)
        .await?;

        tracing::info!("Updated content item {} - new slug: {:?}", id, item.slug);

        // Only create version record if we bumped the version (i.e., editing published content)
        if should_bump_version {
            sqlx::query(
                r#"
                INSERT INTO content_item_versions (content_id, version_number, data, translations, change_summary, created_by)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#
            )
            .bind(id)
            .bind(new_version)
            .bind(&request.data)
            .bind(&request.translations)
            .bind(&request.change_summary)
            .bind(updated_by)
            .execute(&self.db_pool)
            .await?;
        }

        Ok(item)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE content_items SET is_active = false, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<(), AppError> {
        // Versions are deleted by CASCADE
        sqlx::query("DELETE FROM content_items WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, id: Uuid, status: ContentItemStatus) -> Result<ContentItem, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    async fn publish(&self, id: Uuid, published_by: Option<&str>) -> Result<ContentItem, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET status = 'published', published_version = version,
                published_at = NOW(), published_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
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

    async fn archive(&self, id: Uuid) -> Result<ContentItem, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET status = 'archived', updated_at = NOW()
            WHERE id = $1
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }

    // ==========================================
    // Version operations
    // ==========================================

    async fn get_versions(&self, content_id: Uuid) -> Result<Vec<ContentItemVersion>, AppError> {
        let versions = sqlx::query_as::<_, ContentItemVersion>(
            r#"
            SELECT id, content_id, version_number, data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   change_summary, created_at, created_by
            FROM content_item_versions
            WHERE content_id = $1
            ORDER BY version_number DESC
            "#
        )
        .bind(content_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(versions)
    }

    async fn get_version(&self, content_id: Uuid, version_number: i32) -> Result<Option<ContentItemVersion>, AppError> {
        let version = sqlx::query_as::<_, ContentItemVersion>(
            r#"
            SELECT id, content_id, version_number, data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   change_summary, created_at, created_by
            FROM content_item_versions
            WHERE content_id = $1 AND version_number = $2
            "#
        )
        .bind(content_id)
        .bind(version_number)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(version)
    }

    async fn get_version_by_id(&self, version_id: Uuid) -> Result<Option<ContentItemVersion>, AppError> {
        let version = sqlx::query_as::<_, ContentItemVersion>(
            r#"
            SELECT id, content_id, version_number, data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   change_summary, created_at, created_by
            FROM content_item_versions
            WHERE id = $1
            "#
        )
        .bind(version_id)
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(version)
    }

    async fn revert_to_version(&self, content_id: Uuid, version_number: i32, reverted_by: Option<&str>) -> Result<ContentItem, AppError> {
        // Get the version to revert to
        let old_version = self.get_version(content_id, version_number).await?
            .ok_or_else(|| AppError::NotFound("Version not found".into()))?;

        // Get current content
        let current = self.get_by_id(content_id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        // Create update request from old version
        let request = UpsertContentItemRequest {
            schema_id: current.schema_id.clone(),
            slug: current.slug.clone(),
            data: old_version.data,
            translations: old_version.translations,
            display_order: None,
            is_featured: None,
            change_summary: Some(format!("Reverted to version {}", version_number)),
            task_status: None,
        };

        self.update(content_id, &request, reverted_by).await
    }

    // ==========================================
    // Scheduling operations
    // ==========================================

    async fn update_schedule(
        &self,
        id: Uuid,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
    ) -> Result<ContentItem, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET publish_at = $2, unpublish_at = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
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

    async fn get_pending_publish(&self) -> Result<Vec<ContentItem>, AppError> {
        let items = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
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

    async fn get_pending_unpublish(&self) -> Result<Vec<ContentItem>, AppError> {
        let items = sqlx::query_as::<_, ContentItem>(
            r#"
            SELECT id, schema_id, slug, display_order, status, version,
                   data,
                   COALESCE(translations, '{}'::jsonb) as translations,
                   COALESCE(translation_status, '{}'::jsonb) as translation_status,
                   is_featured, is_active, published_version, published_at, published_by,
                   publish_at, unpublish_at,
                   created_at, updated_at, created_by, updated_by
            FROM content_items
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

    // ==========================================
    // Bulk operations
    // ==========================================

    async fn reorder(&self, schema_id: &str, order: Vec<(Uuid, i32)>) -> Result<(), AppError> {
        for (id, display_order) in order {
            sqlx::query(
                "UPDATE content_items SET display_order = $1, updated_at = NOW() WHERE id = $2 AND schema_id = $3"
            )
            .bind(display_order)
            .bind(id)
            .bind(schema_id)
            .execute(&self.db_pool)
            .await?;
        }
        Ok(())
    }

    async fn get_summaries(&self, params: &ContentItemListParams, schema: &ContentSchema) -> Result<(Vec<ContentItemSummary>, i64), AppError> {
        let (items, total) = self.list(params).await?;

        let summaries = items.into_iter().map(|item| {
            // Extract title from data using schema's title_field
            let title = item.data.get(&schema.settings.title_field)
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string();

            // Extract preview data for configured fields
            let preview_data: std::collections::HashMap<String, serde_json::Value> = schema.settings.preview_fields
                .iter()
                .filter_map(|field| {
                    item.data.get(field).map(|v| (field.clone(), v.clone()))
                })
                .collect();

            let translation_coverage = Self::calculate_translation_coverage(&item.translation_status);

            ContentItemSummary {
                id: item.id,
                schema_id: item.schema_id,
                slug: item.slug,
                title,
                status: item.status,
                display_order: item.display_order,
                version: item.version,
                published_version: item.published_version,
                is_featured: item.is_featured,
                updated_at: item.updated_at,
                updated_by: item.updated_by,
                translation_coverage,
                preview_data,
            }
        }).collect();

        Ok((summaries, total))
    }

    async fn update_translations(
        &self,
        id: Uuid,
        translations: serde_json::Value,
        translation_status: serde_json::Value,
        updated_by: Option<&str>,
    ) -> Result<ContentItem, AppError> {
        let item = sqlx::query_as::<_, ContentItem>(
            r#"
            UPDATE content_items
            SET translations = $1, translation_status = $2,
                updated_by = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING id, schema_id, slug, display_order, status, version,
                      data,
                      COALESCE(translations, '{}'::jsonb) as translations,
                      COALESCE(translation_status, '{}'::jsonb) as translation_status,
                      is_featured, is_active, published_version, published_at, published_by,
                      publish_at, unpublish_at,
                      created_at, updated_at, created_by, updated_by
            "#
        )
        .bind(&translations)
        .bind(&translation_status)
        .bind(updated_by)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(item)
    }
}
