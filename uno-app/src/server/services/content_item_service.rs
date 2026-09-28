//! Content Item service for business logic related to generic content

use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::repositories::{
    DynContentItemRepository, DynSchemaRepository, DynAuditRepository
};
use crate::types::{
    AppError, ContentItem, ContentItemStatus, ContentItemVersion,
    ContentItemListParams, ContentItemSummary, ContentItemListResponse,
    ContentItemDetailResponse, UpsertContentItemRequest, ContentSchema,
    ValidationError, SUPPORTED_LOCALES, get_text_direction, CreateAuditLog,
};

/// Content Item service for managing schema-driven content
#[derive(Clone)]
pub struct ContentItemServiceImpl {
    content_repo: DynContentItemRepository,
    schema_repo: DynSchemaRepository,
    audit_repo: Option<DynAuditRepository>,
}

impl ContentItemServiceImpl {
    pub fn new(
        content_repo: DynContentItemRepository,
        schema_repo: DynSchemaRepository,
        audit_repo: Option<DynAuditRepository>,
    ) -> Self {
        Self { content_repo, schema_repo, audit_repo }
    }

    // ==========================================
    // Public API (for uno-app consumption)
    // ==========================================

    /// Get all published content by schema type
    pub async fn get_published_by_schema(&self, schema_id: &str, locale: &str) -> Result<Vec<serde_json::Value>, AppError> {
        // Verify schema exists
        self.schema_repo.get_by_id(schema_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", schema_id)))?;

        let items = self.content_repo.get_published_by_schema(schema_id).await?;

        // Localize all items
        Ok(items.iter().map(|item| self.localize_content(item, locale)).collect())
    }

    /// Get single published content by slug
    pub async fn get_published_by_slug(&self, schema_id: &str, slug: &str, locale: &str) -> Result<Option<serde_json::Value>, AppError> {
        match self.content_repo.get_published_by_slug(schema_id, slug).await? {
            Some(item) => Ok(Some(self.localize_content(&item, locale))),
            None => Ok(None),
        }
    }

    /// Get featured content by schema type
    pub async fn get_featured(&self, schema_id: &str, locale: &str) -> Result<Vec<serde_json::Value>, AppError> {
        let items = self.content_repo.get_featured_by_schema(schema_id).await?;
        Ok(items.iter().map(|item| self.localize_content(item, locale)).collect())
    }

    /// Get featured content by schema type (raw, without localization)
    pub async fn get_featured_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError> {
        self.content_repo.get_featured_by_schema(schema_id).await
    }

    /// Get published content by schema type (raw, without localization)
    pub async fn get_published_items_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError> {
        self.content_repo.get_published_by_schema(schema_id).await
    }

    /// Get all content by schema type regardless of status (for preview mode)
    pub async fn get_items_by_schema(&self, schema_id: &str) -> Result<Vec<ContentItem>, AppError> {
        let params = ContentItemListParams {
            schema_id: Some(schema_id.to_string()),
            status: None,
            search: None,
            is_featured: None,
            page: 1,
            per_page: 100, // Get first 100 items
        };
        let (items, _) = self.content_repo.list(&params).await?;
        Ok(items)
    }

    /// Search content
    pub async fn search(&self, schema_id: &str, query: &str) -> Result<Vec<ContentItem>, AppError> {
        self.content_repo.search(schema_id, query).await
    }

    /// Get localized content for public API
    /// Also converts storage URLs (gcs://, s3://) to browser-loadable signed URLs
    pub fn localize_content(&self, item: &ContentItem, locale: &str) -> serde_json::Value {
        let mut result = item.localize(locale);

        // Add text direction
        if let serde_json::Value::Object(ref mut map) = result {
            map.insert("direction".to_string(), serde_json::json!(get_text_direction(locale)));
        }

        // Convert cloud storage URLs to display URLs
        crate::server::utils::convert_storage_urls(&mut result);

        result
    }

    // ==========================================
    // Admin API
    // ==========================================

    /// Get content by ID with full detail
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ContentItemDetailResponse>, AppError> {
        let item = match self.content_repo.get_by_id(id).await? {
            Some(item) => item,
            None => return Ok(None),
        };

        let schema = self.schema_repo.get_by_id(&item.schema_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", item.schema_id)))?;

        let versions = self.content_repo.get_versions(id).await?;

        Ok(Some(ContentItemDetailResponse {
            item,
            schema,
            versions_count: versions.len() as i32,
            latest_version: versions.into_iter().next(),
        }))
    }

    /// Get content detail (alias for get_by_id)
    pub async fn get_detail(&self, id: Uuid) -> Result<Option<ContentItemDetailResponse>, AppError> {
        self.get_by_id(id).await
    }

    /// List content with filters
    pub async fn list(&self, params: &ContentItemListParams) -> Result<ContentItemListResponse, AppError> {
        let (items, total) = self.content_repo.list(params).await?;

        let total_pages = ((total as f64) / (params.per_page as f64)).ceil() as i32;

        // If schema_id is specified, get summaries with preview data
        let items_response = if let Some(ref schema_id) = params.schema_id {
            if let Some(schema) = self.schema_repo.get_by_id(schema_id).await? {
                let (summaries, _) = self.content_repo.get_summaries(params, &schema).await?;
                return Ok(ContentItemListResponse {
                    items: summaries,
                    total,
                    page: params.page,
                    per_page: params.per_page,
                    total_pages,
                });
            }
            // Fall through to create basic summaries
            self.items_to_summaries(items).await?
        } else {
            self.items_to_summaries(items).await?
        };

        Ok(ContentItemListResponse {
            items: items_response,
            total,
            page: params.page,
            per_page: params.per_page,
            total_pages,
        })
    }

    /// Convert items to summaries
    async fn items_to_summaries(&self, items: Vec<ContentItem>) -> Result<Vec<ContentItemSummary>, AppError> {
        let mut summaries = Vec::new();

        for item in items {
            let schema = self.schema_repo.get_by_id(&item.schema_id).await?;

            let title = schema.as_ref()
                .and_then(|s| item.data.get(&s.settings.title_field))
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string();

            let preview_data = schema.as_ref()
                .map(|s| {
                    s.settings.preview_fields.iter()
                        .filter_map(|f| item.data.get(f).map(|v| (f.clone(), v.clone())))
                        .collect()
                })
                .unwrap_or_default();

            let translation_coverage = self.calculate_translation_coverage(&item.translation_status);

            summaries.push(ContentItemSummary {
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
            });
        }

        Ok(summaries)
    }

    /// Calculate translation coverage percentage
    fn calculate_translation_coverage(&self, translation_status: &Option<serde_json::Value>) -> i32 {
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

    /// Create new content
    pub async fn create(&self, mut request: UpsertContentItemRequest, created_by: Option<&str>) -> Result<ContentItem, AppError> {
        // Validate schema exists
        let schema = self.schema_repo.get_by_id(&request.schema_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", request.schema_id)))?;

        // Merge task_status into data if provided (for task-type schemas)
        if let Some(ref status) = request.task_status {
            if let serde_json::Value::Object(ref mut map) = request.data {
                map.insert("task_status".to_string(), serde_json::json!(status));
            }
        }

        // Validate content against schema
        self.validate_content(&schema, &request.data)?;

        let item = self.content_repo.create(&request, created_by).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "create".to_string(),
                actor_id: created_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: serde_json::to_value(&item).ok(),
                metadata: Some(serde_json::json!({"content_id": item.id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    /// Update existing content
    pub async fn update(&self, id: Uuid, mut request: UpsertContentItemRequest, updated_by: Option<&str>) -> Result<ContentItem, AppError> {
        // Get existing item
        let existing = self.content_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        // Validate schema exists
        let schema = self.schema_repo.get_by_id(&request.schema_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", request.schema_id)))?;

        // Merge task_status into data if provided (for task-type schemas)
        if let Some(ref status) = request.task_status {
            if let serde_json::Value::Object(ref mut map) = request.data {
                map.insert("task_status".to_string(), serde_json::json!(status));
            }
        }

        // Validate content against schema
        self.validate_content(&schema, &request.data)?;

        let old_value = serde_json::to_value(&existing).ok();
        let item = self.content_repo.update(id, &request, updated_by).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "update".to_string(),
                actor_id: updated_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: old_value,
                new_values: serde_json::to_value(&item).ok(),
                metadata: Some(serde_json::json!({"content_id": item.id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    /// Delete content (soft delete)
    pub async fn delete(&self, id: Uuid, deleted_by: Option<&str>) -> Result<(), AppError> {
        let existing = self.content_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        self.content_repo.delete(id).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "delete".to_string(),
                actor_id: deleted_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: serde_json::to_value(&existing).ok(),
                new_values: None,
                metadata: Some(serde_json::json!({"content_id": id.to_string()})),
            }).await;
        }

        Ok(())
    }

    /// Publish content
    pub async fn publish(&self, id: Uuid, published_by: Option<&str>) -> Result<ContentItem, AppError> {
        let existing = self.content_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        // Verify content can be published
        if !existing.status.can_publish() {
            return Err(AppError::BadRequest(format!(
                "Content with status '{}' cannot be published",
                existing.status.as_str()
            )));
        }

        let item = self.content_repo.publish(id, published_by).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "publish".to_string(),
                actor_id: published_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({"status": "published", "published_version": item.version})),
                metadata: Some(serde_json::json!({"content_id": id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    /// Archive content
    pub async fn archive(&self, id: Uuid, archived_by: Option<&str>) -> Result<ContentItem, AppError> {
        let item = self.content_repo.archive(id).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "archive".to_string(),
                actor_id: archived_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({"status": "archived"})),
                metadata: Some(serde_json::json!({"content_id": id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    // ==========================================
    // Version operations
    // ==========================================

    /// Get version history
    pub async fn get_versions(&self, content_id: Uuid) -> Result<Vec<ContentItemVersion>, AppError> {
        self.content_repo.get_versions(content_id).await
    }

    /// Get specific version
    pub async fn get_version(&self, content_id: Uuid, version_number: i32) -> Result<Option<ContentItemVersion>, AppError> {
        self.content_repo.get_version(content_id, version_number).await
    }

    /// Revert to specific version
    pub async fn revert_to_version(&self, content_id: Uuid, version_number: i32, reverted_by: Option<&str>) -> Result<ContentItem, AppError> {
        let item = self.content_repo.revert_to_version(content_id, version_number, reverted_by).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "revert".to_string(),
                actor_id: reverted_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({"reverted_to_version": version_number})),
                metadata: Some(serde_json::json!({"content_id": content_id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    /// Compare two versions
    pub fn compare_versions(&self, v1: &ContentItemVersion, v2: &ContentItemVersion) -> serde_json::Value {
        let mut changes = Vec::new();

        // Compare data fields
        if let (serde_json::Value::Object(data1), serde_json::Value::Object(data2)) =
            (&v1.data, &v2.data)
        {
            // Find changed fields
            for (key, value1) in data1.iter() {
                let value2 = data2.get(key);
                if value2 != Some(value1) {
                    changes.push(serde_json::json!({
                        "field": key,
                        "type": if value2.is_some() { "modified" } else { "removed" },
                        "old_value": value1,
                        "new_value": value2
                    }));
                }
            }

            // Find added fields
            for (key, value2) in data2.iter() {
                if !data1.contains_key(key) {
                    changes.push(serde_json::json!({
                        "field": key,
                        "type": "added",
                        "old_value": null,
                        "new_value": value2
                    }));
                }
            }
        }

        serde_json::json!({
            "version1": v1.version_number,
            "version2": v2.version_number,
            "changes": changes
        })
    }

    // ==========================================
    // Scheduling operations
    // ==========================================

    /// Update publish/unpublish schedule
    pub async fn update_schedule(
        &self,
        id: Uuid,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
        updated_by: Option<&str>,
    ) -> Result<ContentItem, AppError> {
        // Validate dates
        if let (Some(pub_at), Some(unpub_at)) = (&publish_at, &unpublish_at) {
            if pub_at >= unpub_at {
                return Err(AppError::BadRequest(
                    "Publish date must be before unpublish date".into()
                ));
            }
        }

        let item = self.content_repo.update_schedule(id, publish_at, unpublish_at).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "schedule".to_string(),
                actor_id: updated_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({
                    "publish_at": publish_at,
                    "unpublish_at": unpublish_at
                })),
                metadata: Some(serde_json::json!({"content_id": id.to_string()})),
            }).await;
        }

        Ok(item)
    }

    /// Process scheduled content (called by scheduler)
    pub async fn process_scheduled_content(&self) -> Result<(i32, i32), AppError> {
        let mut published = 0;
        let mut unpublished = 0;

        // Process pending publishes
        let pending_publish = self.content_repo.get_pending_publish().await?;
        for item in pending_publish {
            if let Ok(_) = self.content_repo.publish(item.id, Some("scheduler")).await {
                published += 1;

                // Clear schedule
                let _ = self.content_repo.update_schedule(item.id, None, item.unpublish_at).await;

                // Log audit
                if let Some(ref audit_repo) = self.audit_repo {
                    let _ = audit_repo.create(CreateAuditLog {
                        entity_type: "content".to_string(),
                        entity_id: None,
                        action: "publish".to_string(),
                        actor_id: "scheduler".to_string(),
                        actor_name: None,
                        old_values: None,
                        new_values: Some(serde_json::json!({"scheduled": true})),
                        metadata: Some(serde_json::json!({"content_id": item.id.to_string()})),
                    }).await;
                }
            }
        }

        // Process pending unpublishes
        let pending_unpublish = self.content_repo.get_pending_unpublish().await?;
        for item in pending_unpublish {
            if let Ok(_) = self.content_repo.archive(item.id).await {
                unpublished += 1;

                // Clear schedule
                let _ = self.content_repo.update_schedule(item.id, None, None).await;

                // Log audit
                if let Some(ref audit_repo) = self.audit_repo {
                    let _ = audit_repo.create(CreateAuditLog {
                        entity_type: "content".to_string(),
                        entity_id: None,
                        action: "archive".to_string(),
                        actor_id: "scheduler".to_string(),
                        actor_name: None,
                        old_values: None,
                        new_values: Some(serde_json::json!({"scheduled": true})),
                        metadata: Some(serde_json::json!({"content_id": item.id.to_string()})),
                    }).await;
                }
            }
        }

        Ok((published, unpublished))
    }

    // ==========================================
    // Validation
    // ==========================================

    /// Validate content against schema
    pub fn validate_content(&self, schema: &ContentSchema, data: &serde_json::Value) -> Result<(), AppError> {
        match schema.validate(data) {
            Ok(()) => Ok(()),
            Err(errors) => {
                let error_messages: Vec<String> = errors.iter()
                    .map(|e| format!("{}: {}", e.field, e.message))
                    .collect();

                Err(AppError::BadRequest(format!(
                    "Validation failed: {}",
                    error_messages.join("; ")
                )))
            }
        }
    }

    /// Validate translations against schema
    pub fn validate_translations(
        &self,
        schema: &ContentSchema,
        translations: &serde_json::Value,
    ) -> Result<(), AppError> {
        let translatable_fields: Vec<&str> = schema.translatable_fields();

        if let serde_json::Value::Object(locales) = translations {
            for (locale, locale_data) in locales {
                // Validate locale is supported
                if !SUPPORTED_LOCALES.contains(&locale.as_str()) {
                    return Err(AppError::BadRequest(format!(
                        "Unsupported locale: {}",
                        locale
                    )));
                }

                // Validate translation only contains translatable fields
                if let serde_json::Value::Object(fields) = locale_data {
                    for field_key in fields.keys() {
                        if !translatable_fields.contains(&field_key.as_str()) {
                            return Err(AppError::BadRequest(format!(
                                "Field '{}' is not translatable",
                                field_key
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // ==========================================
    // Translation operations
    // ==========================================

    /// Update translation for a specific locale
    pub async fn update_translation(
        &self,
        id: Uuid,
        locale: &str,
        data: serde_json::Value,
        status: Option<&str>,
        updated_by: Option<&str>,
    ) -> Result<ContentItem, AppError> {
        // Validate locale
        if !SUPPORTED_LOCALES.contains(&locale) {
            return Err(AppError::BadRequest(format!("Unsupported locale: {}", locale)));
        }

        // Get existing item
        let mut item = self.content_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Content item not found".into()))?;

        // Get schema to validate translatable fields
        let schema = self.schema_repo.get_by_id(&item.schema_id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", item.schema_id)))?;

        // Validate translation data only contains translatable fields
        let translatable_fields: Vec<&str> = schema.translatable_fields();
        if let serde_json::Value::Object(fields) = &data {
            for field_key in fields.keys() {
                if !translatable_fields.contains(&field_key.as_str()) {
                    return Err(AppError::BadRequest(format!(
                        "Field '{}' is not translatable",
                        field_key
                    )));
                }
            }
        }

        // Update translations
        let mut translations = item.translations.unwrap_or_else(|| serde_json::json!({}));
        if let serde_json::Value::Object(ref mut map) = translations {
            map.insert(locale.to_string(), data.clone());
        }

        // Update translation status
        let mut translation_status = item.translation_status.unwrap_or_else(|| serde_json::json!({}));
        if let serde_json::Value::Object(ref mut map) = translation_status {
            let status_value = status.unwrap_or("draft").to_string();
            map.insert(locale.to_string(), serde_json::json!(status_value));
        }

        // Update the item
        let updated_item = self.content_repo.update_translations(id, translations, translation_status, updated_by).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "update_translation".to_string(),
                actor_id: updated_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({
                    "locale": locale,
                    "status": status
                })),
                metadata: Some(serde_json::json!({"content_id": id.to_string()})),
            }).await;
        }

        Ok(updated_item)
    }

    // ==========================================
    // Bulk operations
    // ==========================================

    /// Reorder content items by providing ordered list of UUIDs
    pub async fn reorder(&self, schema_id: &str, order: &[Uuid], updated_by: Option<&str>) -> Result<(), AppError> {
        // Convert to (Uuid, i32) pairs with indices as display_order
        let order_pairs: Vec<(Uuid, i32)> = order.iter()
            .enumerate()
            .map(|(i, id)| (*id, i as i32))
            .collect();

        self.content_repo.reorder(schema_id, order_pairs.clone()).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "content".to_string(),
                entity_id: None,
                action: "reorder".to_string(),
                actor_id: updated_by.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: Some(serde_json::json!({"order": order})),
                metadata: Some(serde_json::json!({"schema_id": schema_id})),
            }).await;
        }

        Ok(())
    }
}
