//! Content service for business logic

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::server::repositories::DynContentRepository;
use crate::server::services::AuditServiceImpl;
use crate::types::{
    PageContent, LocalizedContent, AppError, ContentType, TextDirection,
    ContentListParams, UpsertContentRequest, ContentListResponse, ContentSummary,
    ContentDetailResponse, VersionSummary, ContentVersion, SUPPORTED_LOCALES,
};

/// Content service for CMS operations
#[derive(Clone)]
pub struct ContentServiceImpl {
    content_repo: DynContentRepository,
    audit_service: Option<AuditServiceImpl>,
}

impl ContentServiceImpl {
    pub fn new(content_repo: DynContentRepository) -> Self {
        Self {
            content_repo,
            audit_service: None,
        }
    }

    /// Create with audit service for logging
    pub fn with_audit(content_repo: DynContentRepository, audit_service: AuditServiceImpl) -> Self {
        Self {
            content_repo,
            audit_service: Some(audit_service),
        }
    }

    // ==========================================
    // Public API methods
    // ==========================================

    /// Get published content by type with localization
    pub async fn get_contents_by_type(&self, content_type: &str, locale: &str) -> Result<Vec<LocalizedContent>, AppError> {
        let items = self.content_repo.get_published_by_type(content_type).await?;

        Ok(items.into_iter().map(|item| self.localize_content(&item, locale)).collect())
    }

    /// Get single content by slug with localization
    pub async fn get_content_by_slug(&self, content_type: &str, slug: &str, locale: &str) -> Result<Option<LocalizedContent>, AppError> {
        let item = self.content_repo.get_published_by_slug(content_type, slug).await?;

        Ok(item.map(|item| self.localize_content(&item, locale)))
    }

    /// Get featured content by type
    pub async fn get_featured_contents(&self, content_type: &str, locale: &str) -> Result<Vec<LocalizedContent>, AppError> {
        let items = self.content_repo.get_featured_by_type(content_type).await?;

        Ok(items.into_iter().map(|item| self.localize_content(&item, locale)).collect())
    }

    /// Search content
    pub async fn search_contents(&self, content_type: &str, query: &str, locale: &str) -> Result<Vec<LocalizedContent>, AppError> {
        let items = self.content_repo.search(content_type, query).await?;

        Ok(items.into_iter().map(|item| self.localize_content(&item, locale)).collect())
    }

    /// Get content for preview by version_id
    /// Returns a single localized content item from a specific version
    pub async fn get_content_for_preview(&self, version_id: i32, locale: &str) -> Result<Option<LocalizedContent>, AppError> {
        let version = match self.content_repo.get_version_by_id(version_id).await? {
            Some(v) => v,
            None => return Ok(None),
        };

        // Get the parent content to access metadata
        let content = match self.content_repo.get_by_id(version.content_id).await? {
            Some(c) => c,
            None => return Ok(None),
        };

        // Build a temporary PageContent from the version data for localization
        let preview_content = PageContent {
            id: content.id,
            content_type: content.content_type.clone(),
            slug: content.slug.clone(),
            status: content.status.clone(),
            content: version.content,
            translations: version.translations,
            translation_status: content.translation_status,
            display_order: content.display_order,
            is_featured: content.is_featured,
            is_active: content.is_active,
            version: version.version,
            published_version: content.published_version,
            published_at: content.published_at,
            published_by: content.published_by,
            publish_at: content.publish_at,
            unpublish_at: content.unpublish_at,
            created_at: content.created_at,
            updated_at: content.updated_at,
            created_by: content.created_by,
            updated_by: content.updated_by,
        };

        Ok(Some(self.localize_content(&preview_content, locale)))
    }

    /// Get all content of a type for preview (replacing published with draft version)
    /// Used when previewing a specific version in context of the full page
    pub async fn get_contents_for_preview(&self, content_type: &str, locale: &str, preview_version_id: i32) -> Result<Vec<LocalizedContent>, AppError> {
        // Get the preview version to know which content_id it belongs to
        let preview_version = match self.content_repo.get_version_by_id(preview_version_id).await? {
            Some(v) => v,
            None => {
                // Invalid preview version, fall back to published
                return self.get_contents_by_type(content_type, locale).await;
            }
        };

        // Get all published content
        let mut items = self.content_repo.get_published_by_type(content_type).await?;

        // Find and replace the content that matches the preview version's content_id
        let preview_content_id = preview_version.content_id;
        let mut found_preview = false;

        for item in items.iter_mut() {
            if item.id == preview_content_id {
                // Replace this item's content with the preview version
                item.content = preview_version.content.clone();
                item.translations = preview_version.translations.clone();
                item.version = preview_version.version;
                found_preview = true;
                break;
            }
        }

        // If the preview content isn't published yet, add it to the list
        if !found_preview {
            if let Some(content) = self.content_repo.get_by_id(preview_content_id).await? {
                let mut preview_item = content;
                preview_item.content = preview_version.content;
                preview_item.translations = preview_version.translations;
                preview_item.version = preview_version.version;
                items.push(preview_item);
            }
        }

        Ok(items.into_iter().map(|item| self.localize_content(&item, locale)).collect())
    }

    // ==========================================
    // Admin API methods
    // ==========================================

    /// List all content (admin)
    pub async fn list_contents(&self, params: ContentListParams) -> Result<ContentListResponse, AppError> {
        let (items, total) = self.content_repo.list(&params).await?;
        let page = params.page.unwrap_or(1);
        let per_page = params.per_page.unwrap_or(20);

        let summaries: Vec<ContentSummary> = items.into_iter().map(|item| {
            // Extract title from content JSON
            let title = item.content.get("title")
                .and_then(|v| v.as_str())
                .or_else(|| item.content.get("question").and_then(|v| v.as_str()))
                .or_else(|| item.content.get("error_code").and_then(|v| v.as_str()))
                .unwrap_or("Untitled")
                .to_string();

            let translation_coverage = item.translation_coverage();
            ContentSummary {
                id: item.id,
                content_type: item.content_type,
                slug: item.slug,
                title,
                status: item.status,
                version: item.version,
                published_version: item.published_version,
                is_featured: item.is_featured,
                translation_coverage,
                updated_at: item.updated_at,
            }
        }).collect();

        Ok(ContentListResponse {
            items: summaries,
            total,
            page,
            per_page,
        })
    }

    /// Get content detail (admin)
    pub async fn get_content(&self, id: i32) -> Result<Option<ContentDetailResponse>, AppError> {
        let item = match self.content_repo.get_by_id(id).await? {
            Some(i) => i,
            None => return Ok(None),
        };

        let versions = self.content_repo.get_versions(id).await?;
        let version_summaries: Vec<VersionSummary> = versions.into_iter().map(|v| VersionSummary {
            version: v.version,
            change_summary: v.change_summary,
            created_at: v.created_at,
            created_by: v.created_by,
        }).collect();

        // Parse translation status
        let translation_status: HashMap<String, String> = item.translation_status
            .as_ref()
            .and_then(|ts| serde_json::from_value(ts.clone()).ok())
            .unwrap_or_else(|| {
                // Default: English complete, others missing
                let mut map = HashMap::new();
                map.insert("en".to_string(), "complete".to_string());
                for locale in SUPPORTED_LOCALES.iter().skip(1) {
                    map.insert(locale.to_string(), "missing".to_string());
                }
                map
            });

        let translation_coverage = item.translation_coverage();
        let translations = item.translations.unwrap_or_else(|| serde_json::json!({}));

        Ok(Some(ContentDetailResponse {
            id: item.id,
            content_type: item.content_type,
            slug: item.slug,
            status: item.status,
            content: item.content,
            translations,
            translation_status,
            translation_coverage,
            display_order: item.display_order,
            is_featured: item.is_featured,
            version: item.version,
            published_version: item.published_version,
            published_at: item.published_at,
            versions: version_summaries,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }))
    }

    /// Create or update content (admin)
    pub async fn upsert_content(&self, id: Option<i32>, request: UpsertContentRequest, user: Option<&str>) -> Result<PageContent, AppError> {
        // Validate content schema based on type
        self.validate_content(&request.content_type, &request.content)?;

        match id {
            Some(id) => self.content_repo.update(id, &request, user).await,
            None => self.content_repo.create(&request, user).await,
        }
    }

    /// Create new content (admin)
    pub async fn create_content(&self, request: UpsertContentRequest, user: Option<&str>) -> Result<PageContent, AppError> {
        self.validate_content(&request.content_type, &request.content)?;
        let content = self.content_repo.create(&request, user).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_created(
                content.id,
                &content.content_type,
                &content.slug,
                user.unwrap_or("unknown"),
            ).await;
        }

        Ok(content)
    }

    /// Update existing content (admin)
    pub async fn update_content(&self, id: i32, request: UpsertContentRequest, user: Option<&str>) -> Result<PageContent, AppError> {
        self.validate_content(&request.content_type, &request.content)?;

        // Get old version for audit
        let old_content = self.content_repo.get_by_id(id).await?;
        let old_version = old_content.as_ref().map(|c| c.version).unwrap_or(0);

        let content = self.content_repo.update(id, &request, user).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_updated(
                content.id,
                old_version,
                content.version,
                user.unwrap_or("unknown"),
                request.change_summary.as_deref(),
            ).await;
        }

        Ok(content)
    }

    /// Publish content (admin)
    pub async fn publish_content(&self, id: i32, published_by: Option<&str>) -> Result<PageContent, AppError> {
        // Get old status for audit
        let old_content = self.content_repo.get_by_id(id).await?;
        let old_status = old_content.as_ref().map(|c| c.status.clone()).unwrap_or_default();

        let content = self.content_repo.publish(id, published_by).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_published(
                content.id,
                &old_status,
                content.version,
                published_by.unwrap_or("unknown"),
            ).await;
        }

        Ok(content)
    }

    /// Archive content (admin)
    pub async fn archive_content(&self, id: i32) -> Result<PageContent, AppError> {
        // Get old status for audit
        let old_content = self.content_repo.get_by_id(id).await?;
        let old_status = old_content.as_ref().map(|c| c.status.clone()).unwrap_or_default();

        let content = self.content_repo.archive(id).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_archived(
                content.id,
                &old_status,
                "unknown", // archive doesn't have actor
            ).await;
        }

        Ok(content)
    }

    /// Revert content to version (admin)
    pub async fn revert_content(&self, id: i32, version: i32, reverted_by: Option<&str>) -> Result<PageContent, AppError> {
        // Get old version for audit
        let old_content = self.content_repo.get_by_id(id).await?;
        let old_version = old_content.as_ref().map(|c| c.version).unwrap_or(0);

        let content = self.content_repo.revert_to_version(id, version, reverted_by).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_reverted(
                content.id,
                old_version,
                content.version,
                version,
                reverted_by.unwrap_or("unknown"),
            ).await;
        }

        Ok(content)
    }

    /// Delete content (admin)
    pub async fn delete_content(&self, id: i32) -> Result<(), AppError> {
        self.content_repo.delete(id).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_deleted(id, "unknown").await;
        }

        Ok(())
    }

    /// Get content by ID (admin)
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PageContent>, AppError> {
        self.content_repo.get_by_id(id).await
    }

    /// Publish content (admin) - alias for publish_content
    pub async fn publish(&self, id: i32, published_by: Option<&str>) -> Result<PageContent, AppError> {
        self.content_repo.publish(id, published_by).await
    }

    /// Get version history for content
    pub async fn get_versions(&self, content_id: i32) -> Result<Vec<ContentVersion>, AppError> {
        self.content_repo.get_versions(content_id).await
    }

    /// Get specific version
    pub async fn get_version(&self, content_id: i32, version: i32) -> Result<Option<ContentVersion>, AppError> {
        self.content_repo.get_version(content_id, version).await
    }

    /// Revert to specific version (admin)
    pub async fn revert_to_version(&self, content_id: i32, version: i32, reverted_by: Option<&str>) -> Result<PageContent, AppError> {
        self.content_repo.revert_to_version(content_id, version, reverted_by).await
    }

    /// Get version by version ID (primary key)
    pub async fn get_version_by_id(&self, version_id: i32) -> Result<Option<ContentVersion>, AppError> {
        self.content_repo.get_version_by_id(version_id).await
    }

    // ==========================================
    // Scheduling methods
    // ==========================================

    /// Update content schedule
    pub async fn update_schedule(
        &self,
        id: i32,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
    ) -> Result<PageContent, AppError> {
        self.update_schedule_with_actor(id, publish_at, unpublish_at, None).await
    }

    /// Update content schedule with actor for audit logging
    pub async fn update_schedule_with_actor(
        &self,
        id: i32,
        publish_at: Option<DateTime<Utc>>,
        unpublish_at: Option<DateTime<Utc>>,
        actor_id: Option<&str>,
    ) -> Result<PageContent, AppError> {
        // Validate: unpublish_at must be after publish_at
        if let (Some(pub_at), Some(unpub_at)) = (publish_at, unpublish_at) {
            if unpub_at <= pub_at {
                return Err(AppError::ValidationError(
                    "Unpublish time must be after publish time".into()
                ));
            }
        }

        let content = self.content_repo.update_schedule(id, publish_at, unpublish_at).await?;

        // Audit log
        if let Some(ref audit) = self.audit_service {
            let _ = audit.log_content_scheduled(
                content.id,
                publish_at,
                unpublish_at,
                actor_id.unwrap_or("unknown"),
            ).await;
        }

        Ok(content)
    }

    /// Process scheduled content (called by background scheduler)
    /// Returns (published_count, unpublished_count)
    pub async fn process_scheduled_content(&self) -> Result<(u32, u32), AppError> {
        let mut published = 0u32;
        let mut unpublished = 0u32;

        // Process pending publishes
        let to_publish = self.content_repo.get_pending_publish().await?;
        for content in to_publish {
            match self.content_repo.publish(content.id, Some("scheduler")).await {
                Ok(_) => {
                    tracing::info!(
                        "Auto-published content {} (slug: {}, type: {})",
                        content.id, content.slug, content.content_type
                    );
                    published += 1;
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to auto-publish content {}: {}",
                        content.id, e
                    );
                }
            }
        }

        // Process pending unpublishes
        let to_unpublish = self.content_repo.get_pending_unpublish().await?;
        for content in to_unpublish {
            match self.content_repo.archive(content.id).await {
                Ok(_) => {
                    tracing::info!(
                        "Auto-unpublished content {} (slug: {}, type: {})",
                        content.id, content.slug, content.content_type
                    );
                    unpublished += 1;
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to auto-unpublish content {}: {}",
                        content.id, e
                    );
                }
            }
        }

        Ok((published, unpublished))
    }


    // ==========================================
    // Helper methods
    // ==========================================

    /// Localize content for a given locale
    /// Also converts storage URLs (gcs://, s3://) to browser-loadable signed URLs
    fn localize_content(&self, item: &PageContent, locale: &str) -> LocalizedContent {
        let mut content = item.localize(locale);
        let direction = item.get_direction(locale);
        let is_fallback = locale != "en" && !item.has_translation(locale);

        // Convert cloud storage URLs to display URLs
        crate::server::utils::convert_storage_urls(&mut content);

        LocalizedContent {
            id: item.id,
            content_type: item.content_type.clone(),
            slug: item.slug.clone(),
            content,
            direction,
            is_featured: item.is_featured,
            is_fallback,
            published_at: item.published_at,
        }
    }

    /// Validate content against schema
    fn validate_content(&self, content_type: &str, content: &serde_json::Value) -> Result<(), AppError> {
        let ct = ContentType::from_str(content_type);

        match ct {
            Some(ContentType::Task) => {
                if content.get("title").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Task must have a non-empty title".into()));
                }
                if content.get("description").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Task must have a non-empty description".into()));
                }
            }
            Some(ContentType::Guide) => {
                if content.get("title").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Guide must have a non-empty title".into()));
                }
                if content.get("description").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Guide must have a non-empty description".into()));
                }
            }
            Some(ContentType::Error) => {
                if content.get("error_code").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Error must have an error_code".into()));
                }
                if content.get("title").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Error must have a title".into()));
                }
                if content.get("solution").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("Error must have a solution".into()));
                }
            }
            Some(ContentType::Faq) => {
                if content.get("question").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("FAQ must have a question".into()));
                }
                if content.get("answer").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    return Err(AppError::ValidationError("FAQ must have an answer".into()));
                }
            }
            None => {
                // Unknown content type - allow but warn
                tracing::warn!("Unknown content type: {}", content_type);
            }
        }

        Ok(())
    }
}
