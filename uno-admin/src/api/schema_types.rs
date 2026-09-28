//! Schema-driven CMS type definitions
//!
//! These types mirror the ones in uno-app for schema-driven content management.

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
    Text(TextFieldConfig),
    RichText(RichTextFieldConfig),
    Number(NumberFieldConfig),
    Boolean(BooleanFieldConfig),
    Select(SelectFieldConfig),
    MultiSelect(MultiSelectFieldConfig),
    Date(DateFieldConfig),
    Media(MediaFieldConfig),
    MediaList(MediaListFieldConfig),
    List(ListFieldConfig),
    Group(GroupFieldConfig),
    Repeater(RepeaterFieldConfig),
    Json(JsonFieldConfig),
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
pub struct ContentSchema {
    pub id: String,
    pub name: String,
    pub name_plural: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub fields: Vec<FieldDefinition>,
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
}

// ============================================
// SCHEMA API TYPES
// ============================================

/// Request to create or update a schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertSchemaRequest {
    pub id: String,
    pub name: String,
    pub name_plural: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub settings: SchemaSettings,
    pub version: i32,
    #[serde(default)]
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Response wrapper for schema list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaListResponse {
    pub schemas: Vec<ContentSchema>,
}

/// Response from create/update schema operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOperationResponse {
    pub id: String,
    pub name: String,
    pub version: i32,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

// ============================================
// CONTENT ITEM
// ============================================

/// Content item status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
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

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::PendingReview => "Pending Review",
            Self::Approved => "Approved",
            Self::Published => "Published",
            Self::Scheduled => "Scheduled",
            Self::Archived => "Archived",
        }
    }
}

/// Response from content item create/update operations (partial item)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemOperationResponse {
    pub id: String,
    pub schema_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub status: ContentItemStatus,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

/// Generic content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: String,  // UUID as string
    pub schema_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub display_order: i32,
    pub status: ContentItemStatus,
    pub version: i32,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_status: Option<serde_json::Value>,
    #[serde(default)]
    pub is_featured: bool,
    #[serde(default = "default_true")]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpublish_at: Option<DateTime<Utc>>,
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
}

/// Content item version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemVersion {
    pub id: String,
    pub content_id: String,
    pub version_number: i32,
    pub data: serde_json::Value,
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

/// Content item summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemSummary {
    pub id: String,
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
    pub preview_data: HashMap<String, serde_json::Value>,
}

/// Content item detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemDetailResponse {
    pub item: ContentItem,
    pub schema: ContentSchema,
    pub versions_count: i32,
    pub latest_version: Option<ContentItemVersion>,
}

/// Content item list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItemListResponse {
    pub items: Vec<ContentItemSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

/// Create/update content item request
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

/// Content item list query parameters
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

fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 20 }

// ============================================
// SUPPORTED LOCALES
// ============================================

pub const SUPPORTED_LOCALES: &[&str] = &["en", "es", "fr", "ar", "hi", "tl", "sw", "pt", "id"];

/// Get text direction for locale
pub fn get_text_direction(locale: &str) -> &'static str {
    match locale {
        "ar" | "he" | "fa" | "ur" => "rtl",
        _ => "ltr",
    }
}
