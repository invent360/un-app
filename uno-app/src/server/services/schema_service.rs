//! Schema service for business logic related to content schemas

use std::sync::Arc;
use crate::server::repositories::{DynSchemaRepository, DynAuditRepository};
use crate::types::{AppError, ContentSchema, FieldDefinition, FieldType, SchemaSettings, CreateAuditLog};

/// Schema service for managing content schemas
#[derive(Clone)]
pub struct SchemaServiceImpl {
    schema_repo: DynSchemaRepository,
    audit_repo: Option<DynAuditRepository>,
}

impl SchemaServiceImpl {
    pub fn new(schema_repo: DynSchemaRepository, audit_repo: Option<DynAuditRepository>) -> Self {
        Self { schema_repo, audit_repo }
    }

    /// Get all available schemas
    pub async fn get_all_schemas(&self) -> Result<Vec<ContentSchema>, AppError> {
        self.schema_repo.get_all().await
    }

    /// Get schema by ID
    pub async fn get_schema(&self, id: &str) -> Result<Option<ContentSchema>, AppError> {
        self.schema_repo.get_by_id(id).await
    }

    /// Get schema by ID, returning error if not found
    pub async fn get_schema_required(&self, id: &str) -> Result<ContentSchema, AppError> {
        self.schema_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", id)))
    }

    /// Create a new schema
    pub async fn create_schema(&self, schema: ContentSchema, user_id: Option<&str>) -> Result<ContentSchema, AppError> {
        // Validate schema
        self.validate_schema(&schema)?;

        // Check if schema ID already exists
        if self.schema_repo.get_by_id(&schema.id).await?.is_some() {
            return Err(AppError::BadRequest(format!("Schema '{}' already exists", schema.id)));
        }

        let created = self.schema_repo.create(&schema).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "schema".to_string(),
                entity_id: None,
                action: "create".to_string(),
                actor_id: user_id.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: None,
                new_values: serde_json::to_value(&created).ok(),
                metadata: Some(serde_json::json!({"schema_id": created.id})),
            }).await;
        }

        Ok(created)
    }

    /// Update an existing schema
    pub async fn update_schema(&self, id: &str, schema: ContentSchema, user_id: Option<&str>) -> Result<ContentSchema, AppError> {
        // Get existing schema
        let existing = self.schema_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", id)))?;

        // Validate schema
        self.validate_schema(&schema)?;

        // Check for breaking changes if schema has content
        if self.schema_repo.has_content(id).await? {
            self.check_breaking_changes(&existing, &schema)?;
        }

        let old_value = serde_json::to_value(&existing).ok();
        let updated = self.schema_repo.update(id, &schema).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "schema".to_string(),
                entity_id: None,
                action: "update".to_string(),
                actor_id: user_id.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: old_value,
                new_values: serde_json::to_value(&updated).ok(),
                metadata: Some(serde_json::json!({"schema_id": updated.id})),
            }).await;
        }

        Ok(updated)
    }

    /// Delete a schema
    pub async fn delete_schema(&self, id: &str, user_id: Option<&str>) -> Result<(), AppError> {
        let existing = self.schema_repo.get_by_id(id).await?
            .ok_or_else(|| AppError::NotFound(format!("Schema '{}' not found", id)))?;

        self.schema_repo.delete(id).await?;

        // Log audit
        if let Some(ref audit_repo) = self.audit_repo {
            let _ = audit_repo.create(CreateAuditLog {
                entity_type: "schema".to_string(),
                entity_id: None,
                action: "delete".to_string(),
                actor_id: user_id.unwrap_or("system").to_string(),
                actor_name: None,
                old_values: serde_json::to_value(&existing).ok(),
                new_values: None,
                metadata: Some(serde_json::json!({"schema_id": id})),
            }).await;
        }

        Ok(())
    }

    /// Validate schema structure
    fn validate_schema(&self, schema: &ContentSchema) -> Result<(), AppError> {
        // Validate ID format (alphanumeric + underscore)
        if schema.id.is_empty() || !schema.id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AppError::BadRequest(
                "Schema ID must be alphanumeric with underscores only".into()
            ));
        }

        // Validate name
        if schema.name.trim().is_empty() {
            return Err(AppError::BadRequest("Schema name is required".into()));
        }

        // Validate fields
        if schema.fields.is_empty() {
            return Err(AppError::BadRequest("Schema must have at least one field".into()));
        }

        // Check for duplicate field keys
        let mut seen_keys = std::collections::HashSet::new();
        for field in &schema.fields {
            if !seen_keys.insert(&field.key) {
                return Err(AppError::BadRequest(
                    format!("Duplicate field key: {}", field.key)
                ));
            }

            // Validate field key format
            if field.key.is_empty() || !field.key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(AppError::BadRequest(
                    format!("Invalid field key format: {}", field.key)
                ));
            }

            // Validate field
            self.validate_field(&field)?;
        }

        // Validate settings
        self.validate_settings(&schema.settings, &schema.fields)?;

        Ok(())
    }

    /// Validate a field definition
    fn validate_field(&self, field: &FieldDefinition) -> Result<(), AppError> {
        if field.label.trim().is_empty() {
            return Err(AppError::BadRequest(
                format!("Field '{}' must have a label", field.key)
            ));
        }

        // Validate nested fields in repeaters and groups
        match &field.field_type {
            FieldType::Repeater(config) => {
                if config.fields.is_empty() {
                    return Err(AppError::BadRequest(
                        format!("Repeater field '{}' must have at least one nested field", field.key)
                    ));
                }
                for nested in &config.fields {
                    self.validate_field(nested)?;
                }
            }
            FieldType::Group(config) => {
                if config.fields.is_empty() {
                    return Err(AppError::BadRequest(
                        format!("Group field '{}' must have at least one nested field", field.key)
                    ));
                }
                for nested in &config.fields {
                    self.validate_field(nested)?;
                }
            }
            FieldType::Select(config) => {
                if config.options.is_empty() {
                    return Err(AppError::BadRequest(
                        format!("Select field '{}' must have at least one option", field.key)
                    ));
                }
            }
            FieldType::MultiSelect(config) => {
                if config.options.is_empty() && !config.allow_custom {
                    return Err(AppError::BadRequest(
                        format!("MultiSelect field '{}' must have options or allow custom values", field.key)
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate schema settings
    fn validate_settings(&self, settings: &SchemaSettings, fields: &[FieldDefinition]) -> Result<(), AppError> {
        // Validate title_field exists
        if !settings.title_field.is_empty() {
            if !fields.iter().any(|f| f.key == settings.title_field) {
                return Err(AppError::BadRequest(
                    format!("Title field '{}' not found in schema fields", settings.title_field)
                ));
            }
        }

        // Validate slug_field exists if has_slug is true
        if settings.has_slug {
            if let Some(ref slug_field) = settings.slug_field {
                if !fields.iter().any(|f| f.key == *slug_field) {
                    return Err(AppError::BadRequest(
                        format!("Slug field '{}' not found in schema fields", slug_field)
                    ));
                }
            }
        }

        // Validate preview_fields exist
        for preview_field in &settings.preview_fields {
            if !fields.iter().any(|f| f.key == *preview_field) {
                return Err(AppError::BadRequest(
                    format!("Preview field '{}' not found in schema fields", preview_field)
                ));
            }
        }

        Ok(())
    }

    /// Check for breaking changes when updating schema with existing content
    fn check_breaking_changes(&self, existing: &ContentSchema, new: &ContentSchema) -> Result<(), AppError> {
        // Check if any required fields were added (would break existing content)
        for new_field in &new.fields {
            if new_field.required {
                let existed = existing.fields.iter().any(|f| f.key == new_field.key);
                if !existed {
                    return Err(AppError::BadRequest(format!(
                        "Cannot add required field '{}' to schema with existing content. \
                         Make the field optional or migrate existing content first.",
                        new_field.key
                    )));
                }
            }
        }

        // Check if field types changed for existing fields
        for new_field in &new.fields {
            if let Some(existing_field) = existing.fields.iter().find(|f| f.key == new_field.key) {
                let type_changed = std::mem::discriminant(&existing_field.field_type)
                    != std::mem::discriminant(&new_field.field_type);

                if type_changed {
                    return Err(AppError::BadRequest(format!(
                        "Cannot change field type for '{}' in schema with existing content. \
                         Create a new field or migrate existing content first.",
                        new_field.key
                    )));
                }
            }
        }

        Ok(())
    }

    /// Get fields that are translatable
    pub fn get_translatable_fields<'a>(&self, schema: &'a ContentSchema) -> Vec<&'a FieldDefinition> {
        schema.fields.iter().filter(|f| f.translatable).collect()
    }

    /// Get fields that should be shown in list view
    pub fn get_list_fields<'a>(&self, schema: &'a ContentSchema) -> Vec<&'a FieldDefinition> {
        schema.fields.iter().filter(|f| f.show_in_list).collect()
    }

    /// Get fields that are searchable
    pub fn get_searchable_fields<'a>(&self, schema: &'a ContentSchema) -> Vec<&'a FieldDefinition> {
        schema.fields.iter().filter(|f| f.searchable).collect()
    }
}
