//! Schema-driven CMS type definitions
//!
//! This module provides a generic, composable content modeling system
//! where content types are defined via configuration rather than code.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================
// FIELD TYPE DEFINITIONS
// ============================================

/// All possible field types in the CMS
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub enum FieldType {
    // Simple fields
    Text(TextFieldConfig),
    RichText(RichTextFieldConfig),
    Number(NumberFieldConfig),
    Boolean(BooleanFieldConfig),
    Select(SelectFieldConfig),
    MultiSelect(MultiSelectFieldConfig),
    Date(DateFieldConfig),

    // Media fields
    Media(MediaFieldConfig),
    MediaList(MediaListFieldConfig),

    // Complex fields
    List(ListFieldConfig),
    Group(GroupFieldConfig),
    Repeater(RepeaterFieldConfig),
    Json(JsonFieldConfig),

    // Relational fields
    Reference(ReferenceFieldConfig),
    ReferenceList(ReferenceListFieldConfig),
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text(TextFieldConfig::default())
    }
}

// ============================================
// FIELD CONFIGURATIONS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TextFieldConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub multiline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RichTextFieldConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(default)]
    pub allowed_formats: Vec<RichTextFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RichTextFormat {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Link,
    List,
    OrderedList,
    Heading,
    Code,
    Blockquote,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NumberFieldConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BooleanFieldConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SelectFieldConfig {
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MultiSelectFieldConfig {
    pub options: Vec<SelectOption>,
    #[serde(default)]
    pub allow_custom: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_selections: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DateFieldConfig {
    #[serde(default)]
    pub include_time: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MediaFieldConfig {
    #[serde(default)]
    pub allowed_types: Vec<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(default)]
    pub required_variants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MediaListFieldConfig {
    #[serde(default)]
    pub allowed_types: Vec<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListFieldConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GroupFieldConfig {
    pub fields: Vec<FieldDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JsonFieldConfig {
    // Free-form JSON field - no specific config needed
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepeaterFieldConfig {
    pub fields: Vec<FieldDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    pub item_label: String,
    #[serde(default = "default_true")]
    pub orderable: bool,
    #[serde(default)]
    pub collapsible: bool,
    /// Optional custom UI component to use instead of default repeater
    /// e.g., "section_editor" for Guide sections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_component: Option<String>,
}

impl Default for RepeaterFieldConfig {
    fn default() -> Self {
        Self {
            fields: vec![],
            min_items: None,
            max_items: None,
            item_label: "Item".to_string(),
            orderable: true,
            collapsible: false,
            ui_component: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferenceFieldConfig {
    pub allowed_types: Vec<String>,
    pub display_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferenceListFieldConfig {
    pub allowed_types: Vec<String>,
    pub display_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
}

fn default_true() -> bool {
    true
}

// ============================================
// FIELD DEFINITION
// ============================================

/// A field definition within a schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDefinition {
    pub key: String,
    pub label: String,
    pub field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub translatable: bool,
    #[serde(default)]
    pub show_in_list: bool,
    #[serde(default)]
    pub searchable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FieldCondition>>,
}

impl FieldDefinition {
    pub fn new(key: &str, label: &str, field_type: FieldType) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            field_type,
            description: None,
            required: false,
            translatable: false,
            show_in_list: false,
            searchable: false,
            default_value: None,
            conditions: None,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn translatable(mut self) -> Self {
        self.translatable = true;
        self
    }

    pub fn show_in_list(mut self) -> Self {
        self.show_in_list = true;
        self
    }

    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn when(mut self, condition: FieldCondition) -> Self {
        self.conditions.get_or_insert_with(Vec::new).push(condition);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    IsEmpty,
    IsNotEmpty,
    GreaterThan,
    LessThan,
}

// ============================================
// CONTENT SCHEMA
// ============================================

/// Defines a content type's structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ContentSchema {
    pub id: String,
    pub name: String,
    pub name_plural: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub fields: Vec<FieldDefinition>,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub settings: SchemaSettings,
    pub version: i32,
    #[serde(default)]
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchemaSettings {
    #[serde(default)]
    pub has_slug: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug_field: Option<String>,
    pub title_field: String,
    #[serde(default)]
    pub preview_fields: Vec<String>,
    #[serde(default)]
    pub orderable: bool,
    #[serde(default)]
    pub translatable: bool,
    #[serde(default)]
    pub versioned: bool,
    #[serde(default)]
    pub singleton: bool,
}

impl ContentSchema {
    /// Get field definition by key
    pub fn get_field(&self, key: &str) -> Option<&FieldDefinition> {
        self.fields.iter().find(|f| f.key == key)
    }

    /// Get all required field keys
    pub fn required_fields(&self) -> Vec<&str> {
        self.fields
            .iter()
            .filter(|f| f.required)
            .map(|f| f.key.as_str())
            .collect()
    }

    /// Get all translatable field keys
    pub fn translatable_fields(&self) -> Vec<&str> {
        self.fields
            .iter()
            .filter(|f| f.translatable)
            .map(|f| f.key.as_str())
            .collect()
    }

    /// Get fields to show in list view
    pub fn list_fields(&self) -> Vec<&FieldDefinition> {
        self.fields
            .iter()
            .filter(|f| f.show_in_list)
            .collect()
    }

    /// Get searchable field keys
    pub fn searchable_fields(&self) -> Vec<&str> {
        self.fields
            .iter()
            .filter(|f| f.searchable)
            .map(|f| f.key.as_str())
            .collect()
    }
}

// ============================================
// FIELD VALUES
// ============================================

/// Type-safe field value storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FieldValue {
    Null,
    Text(String),
    Number(f64),
    Boolean(bool),
    Date(DateTime<Utc>),
    List(Vec<String>),
    MediaRef(String),
    MediaRefs(Vec<String>),
    Reference(String),
    References(Vec<String>),
    Group(HashMap<String, FieldValue>),
    Repeater(Vec<HashMap<String, serde_json::Value>>),
    // For flexibility, also support raw JSON
    Json(serde_json::Value),
}

impl Default for FieldValue {
    fn default() -> Self {
        FieldValue::Null
    }
}

impl From<&str> for FieldValue {
    fn from(s: &str) -> Self {
        FieldValue::Text(s.to_string())
    }
}

impl From<String> for FieldValue {
    fn from(s: String) -> Self {
        FieldValue::Text(s)
    }
}

impl From<f64> for FieldValue {
    fn from(n: f64) -> Self {
        FieldValue::Number(n)
    }
}

impl From<i64> for FieldValue {
    fn from(n: i64) -> Self {
        FieldValue::Number(n as f64)
    }
}

impl From<bool> for FieldValue {
    fn from(b: bool) -> Self {
        FieldValue::Boolean(b)
    }
}

impl From<Vec<String>> for FieldValue {
    fn from(v: Vec<String>) -> Self {
        FieldValue::List(v)
    }
}

impl From<serde_json::Value> for FieldValue {
    fn from(v: serde_json::Value) -> Self {
        FieldValue::Json(v)
    }
}

// ============================================
// CONTENT ITEM
// ============================================

/// Content status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "content_status", rename_all = "snake_case"))]
pub enum ContentItemStatus {
    #[default]
    Draft,
    PendingReview,
    Approved,
    Published,
    Scheduled,
    Archived,
}

impl ContentItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::PendingReview => "pending_review",
            Self::Approved => "approved",
            Self::Published => "published",
            Self::Scheduled => "scheduled",
            Self::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "draft" => Some(Self::Draft),
            "pending_review" => Some(Self::PendingReview),
            "approved" => Some(Self::Approved),
            "published" => Some(Self::Published),
            "scheduled" => Some(Self::Scheduled),
            "archived" => Some(Self::Archived),
            _ => None,
        }
    }

    pub fn is_editable(&self) -> bool {
        matches!(self, Self::Draft)
    }

    pub fn can_submit_for_review(&self) -> bool {
        matches!(self, Self::Draft)
    }

    pub fn can_publish(&self) -> bool {
        matches!(self, Self::Draft | Self::Approved)
    }
}

/// Generic content item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ContentItem {
    pub id: uuid::Uuid,
    pub schema_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub display_order: i32,
    pub status: ContentItemStatus,
    pub version: i32,

    // Generic data storage
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub data: serde_json::Value,

    // Translations per locale
    #[cfg_attr(feature = "ssr", sqlx(json))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<serde_json::Value>,

    // Translation status per locale
    #[cfg_attr(feature = "ssr", sqlx(json))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_status: Option<serde_json::Value>,

    // Display flags
    #[serde(default)]
    pub is_featured: bool,
    #[serde(default = "default_true")]
    pub is_active: bool,

    // Publishing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,

    // Scheduling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpublish_at: Option<DateTime<Utc>>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

impl ContentItem {
    /// Get a field value from data
    pub fn get_field(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Get the title field value (requires schema to know which field)
    pub fn get_title(&self, schema: &ContentSchema) -> Option<String> {
        self.data
            .get(&schema.settings.title_field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Get localized data, falling back to base data
    pub fn localize(&self, locale: &str) -> serde_json::Value {
        if locale == "en" {
            return self.data.clone();
        }

        self.translations
            .as_ref()
            .and_then(|t| t.get(locale))
            .cloned()
            .unwrap_or_else(|| self.data.clone())
    }

    /// Check if translation exists for locale
    pub fn has_translation(&self, locale: &str) -> bool {
        if locale == "en" {
            return true;
        }

        self.translations
            .as_ref()
            .map(|t| t.get(locale).is_some())
            .unwrap_or(false)
    }

    /// Get translation status for locale
    pub fn get_translation_status(&self, locale: &str) -> &str {
        if locale == "en" {
            return "complete";
        }

        self.translation_status
            .as_ref()
            .and_then(|ts| ts.get(locale))
            .and_then(|s| s.as_str())
            .unwrap_or("missing")
    }
}

/// Content item version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ContentItemVersion {
    pub id: uuid::Uuid,
    pub content_id: uuid::Uuid,
    pub version_number: i32,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    pub data: serde_json::Value,
    #[cfg_attr(feature = "ssr", sqlx(json))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

// ============================================
// API TYPES
// ============================================

/// Schema list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaListResponse {
    pub schemas: Vec<ContentSchema>,
}

/// Content list item (summary for list view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemSummary {
    pub id: uuid::Uuid,
    pub schema_id: String,
    pub slug: Option<String>,
    pub title: String,
    pub status: ContentItemStatus,
    pub display_order: i32,
    pub version: i32,
    pub published_version: Option<i32>,
    pub is_featured: bool,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<String>,
    pub translation_coverage: i32,
    // Dynamic preview fields based on schema
    pub preview_data: HashMap<String, serde_json::Value>,
}

/// Content detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemDetailResponse {
    pub item: ContentItem,
    pub schema: ContentSchema,
    pub versions_count: i32,
    pub latest_version: Option<ContentItemVersion>,
}

/// Create/update content request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertContentItemRequest {
    pub schema_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_featured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
    /// Task status for task-type content items (active, on_demand, coming_soon, deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

/// Content list query parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentItemListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContentItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_featured: Option<bool>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

/// Content list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemListResponse {
    pub items: Vec<ContentItemSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

// ============================================
// VALIDATION
// ============================================

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl ContentSchema {
    /// Validate content data against schema
    pub fn validate(&self, data: &serde_json::Value) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for field in &self.fields {
            let value = data.get(&field.key);

            // Check required
            if field.required {
                let is_empty = match value {
                    None => true,
                    Some(v) => v.is_null() || (v.is_string() && v.as_str().unwrap_or("").is_empty()),
                };

                if is_empty {
                    errors.push(ValidationError {
                        field: field.key.clone(),
                        message: format!("{} is required", field.label),
                        code: "required".to_string(),
                    });
                    continue;
                }
            }

            // Type-specific validation
            if let Some(v) = value {
                if let Some(err) = validate_field_value(v, &field.field_type, &field.key, &field.label) {
                    errors.push(err);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn validate_field_value(
    value: &serde_json::Value,
    field_type: &FieldType,
    key: &str,
    label: &str,
) -> Option<ValidationError> {
    match field_type {
        FieldType::Text(config) => {
            if let Some(s) = value.as_str() {
                if let Some(min) = config.min_length {
                    if s.len() < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at least {} characters", label, min),
                            code: "min_length".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max_length {
                    if s.len() > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at most {} characters", label, max),
                            code: "max_length".to_string(),
                        });
                    }
                }
                if let Some(ref pattern) = config.pattern {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        if !re.is_match(s) {
                            return Some(ValidationError {
                                field: key.to_string(),
                                message: format!("{} has invalid format", label),
                                code: "pattern".to_string(),
                            });
                        }
                    }
                }
            }
        }
        FieldType::Number(config) => {
            if let Some(n) = value.as_f64() {
                if let Some(min) = config.min {
                    if n < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at least {}", label, min),
                            code: "min".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max {
                    if n > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must be at most {}", label, max),
                            code: "max".to_string(),
                        });
                    }
                }
            }
        }
        FieldType::Repeater(config) => {
            if let Some(arr) = value.as_array() {
                if let Some(min) = config.min_items {
                    if arr.len() < min {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must have at least {} items", label, min),
                            code: "min_items".to_string(),
                        });
                    }
                }
                if let Some(max) = config.max_items {
                    if arr.len() > max {
                        return Some(ValidationError {
                            field: key.to_string(),
                            message: format!("{} must have at most {} items", label, max),
                            code: "max_items".to_string(),
                        });
                    }
                }
            }
        }
        // Add more validations as needed
        _ => {}
    }

    None
}

// ============================================
// LOCALE UTILITIES
// ============================================

/// Get text direction for locale
pub fn get_text_direction(locale: &str) -> &'static str {
    match locale {
        "ar" | "he" | "fa" | "ur" => "rtl",
        _ => "ltr",
    }
}
