# CMS Architecture Design

> **Status:** Design Phase
> **Target Stack:** Leptos/Rust (Frontend) + Axum (Backend) + PostgreSQL
> **Last Updated:** 2026-05-31

---

## Table of Contents

1. [Overview](#1-overview)
2. [System Architecture](#2-system-architecture)
3. [Core Design Decisions](#3-core-design-decisions)
4. [Generic Composable Content Model](#4-generic-composable-content-model)
5. [Field Types](#5-field-types)
6. [Content Schemas](#6-content-schemas)
7. [Database Schema](#7-database-schema)
8. [API Design](#8-api-design)
9. [Frontend Architecture](#9-frontend-architecture)
10. [Content Delivery Strategy](#10-content-delivery-strategy)
11. [Feature Priority Matrix](#11-feature-priority-matrix)

---

## 1. Overview

### Purpose

Build a CMS system where **uno-admin** can manage content for **uno-app** without requiring application redeployment.

### Key Principles

| Aspect | Structure (Dev-dependent) | Data (CMS-managed) |
|--------|--------------------------|-------------------|
| **What** | UI Components, layouts, rendering logic | Titles, descriptions, images, steps |
| **Changed by** | Developers | Non-technical users via CMS |
| **Requires** | App redeployment | API update only |
| **Stored in** | uno-app codebase | Database/API |

### Content Types

| Type | Purpose | Sections |
|------|---------|----------|
| `front` | Homepage/landing page content | Intro slides |
| `task` | Task descriptions for earning incentives | Telemetry, Caller ID, SMS, etc. |
| `guide` | Step-by-step tutorials | Setup, Activation, Withdrawal |
| `faq` | Frequently Asked Questions | General, Technical, Account |
| `error` | Error documentation | Error codes and solutions |

---

## 2. System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            UNO ECOSYSTEM                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐         ┌─────────────────┐         ┌─────────────┐   │
│  │  uno-admin  │◄───────►│   CMS Backend   │◄───────►│   uno-app   │   │
│  │  (Leptos)   │   API   │     (Axum)      │   CDN   │  (Consumer) │   │
│  └─────────────┘         └────────┬────────┘         └─────────────┘   │
│                                   │                                     │
│                          ┌────────▼────────┐                           │
│                          │  Storage Layer  │                           │
│                          │                 │                           │
│                          │ ┌─────────────┐ │                           │
│                          │ │  Database   │ │                           │
│                          │ │ (PostgreSQL)│ │                           │
│                          │ └─────────────┘ │                           │
│                          │ ┌─────────────┐ │                           │
│                          │ │    Media    │ │                           │
│                          │ │   Storage   │ │                           │
│                          │ └─────────────┘ │                           │
│                          └─────────────────┘                           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Design Decisions

### 3.1 Storage Strategy

| Strategy | Pros | Cons | Recommendation |
|----------|------|------|----------------|
| **File-based (JSON/YAML)** | Simple, git-friendly, no DB | No queries, no relations, concurrency issues | ❌ Not for production |
| **SQLite** | Embedded, simple deployment | Limited concurrency, no replication | ⚠️ Small scale only |
| **PostgreSQL** | Full ACID, JSON support, scalable | Requires setup | ✅ Recommended |
| **Hybrid (DB + S3)** | Best of both: structured data + media | More complexity | ✅ Ideal for media-heavy |

**Decision:** PostgreSQL with JSONB for flexible content + S3-compatible storage for media

### 3.2 Content Modeling Approach

| Approach | Description | Flexibility | Query Performance |
|----------|-------------|-------------|-------------------|
| **Rigid tables** | One table per content type | Low | High |
| **EAV (Entity-Attribute-Value)** | Generic key-value | High | Low |
| **JSONB columns** | Structured + flexible | Medium-High | Medium-High |
| **Hybrid** | Core fields as columns, extras as JSONB | High | High |

**Decision:** Schema-driven with JSONB storage - define content types via configuration, store values in JSONB

---

## 4. Generic Composable Content Model

### The Problem with Repetitive Structs

```rust
// ❌ BAD: Repetitive, rigid, hard to extend
pub struct TaskBody { title, description, images, ... }
pub struct GuideBody { title, description, thumbnail, ... }
pub struct FaqBody { title, description, ... }
// Every new content type = new struct + new form + new API handler
```

### The Solution: Schema-Driven Content

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    COMPOSABLE CONTENT ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   ContentSchema                ContentValue                             │
│   (defines structure)          (stores data)                            │
│   ┌─────────────────┐         ┌─────────────────┐                      │
│   │ fields: [       │         │ values: {       │                      │
│   │   Field::Text   │────────►│   "title": ...  │                      │
│   │   Field::Media  │         │   "cover": ...  │                      │
│   │   Field::List   │         │   "steps": ...  │                      │
│   │ ]               │         │ }               │                      │
│   └─────────────────┘         └─────────────────┘                      │
│                                                                         │
│   One system renders ALL content types dynamically                      │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Benefits

| Aspect | Before (Repetitive Structs) | After (Generic Schema) |
|--------|----------------------------|------------------------|
| Adding new content type | New struct + form + API | Just add schema definition |
| Code lines per type | ~200+ | ~50 (schema only) |
| Form component | One per type | One for all |
| Validation | Manual per struct | Automatic from schema |
| API handlers | One per type | Generic CRUD |
| Frontend rendering | Custom per type | Dynamic from schema |
| Extensibility | New code required | Config-driven |

---

## 5. Field Types

### 5.1 Field Type Definitions

```rust
/// All possible field types in the CMS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
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
    List(ListFieldConfig),           // Ordered list of items
    Group(GroupFieldConfig),         // Nested object
    Repeater(RepeaterFieldConfig),   // Array of groups (e.g., steps, tiers)

    // Relational fields
    Reference(ReferenceFieldConfig), // Link to other content
    ReferenceList(ReferenceListFieldConfig),
}
```

### 5.2 Field Configurations

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFieldConfig {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,      // Regex validation
    pub placeholder: Option<String>,
    pub multiline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextFieldConfig {
    pub max_length: Option<usize>,
    pub allowed_formats: Vec<RichTextFormat>, // bold, italic, links, etc.
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFieldConfig {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub unit: Option<String>,         // "$", "minutes", etc.
    pub precision: Option<u8>,        // Decimal places
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectFieldConfig {
    pub options: Vec<SelectOption>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFieldConfig {
    pub allowed_types: Vec<MediaType>, // image, video, document
    pub max_size_bytes: Option<u64>,
    pub aspect_ratio: Option<AspectRatio>,
    pub required_variants: Vec<String>, // "thumbnail", "medium"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeaterFieldConfig {
    pub fields: Vec<FieldDefinition>,  // Schema for each item
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub item_label: String,            // "Step", "Tier", "Slide"
    pub orderable: bool,               // Can reorder items
    pub collapsible: bool,             // UI hint
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFieldConfig {
    pub fields: Vec<FieldDefinition>,  // Nested fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceFieldConfig {
    pub allowed_types: Vec<String>,    // Which content types can be linked
    pub display_field: String,         // Which field to show as label
}
```

### 5.3 Field Definition Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub key: String,                   // Unique identifier within schema
    pub field_type: FieldType,
    pub label: String,                 // Display label
    pub description: Option<String>,   // Help text
    pub required: bool,
    pub translatable: bool,            // Should this be translated?
    pub show_in_list: bool,            // Show in content list view
    pub searchable: bool,              // Include in full-text search
    pub default_value: Option<serde_json::Value>,
    pub conditions: Option<Vec<FieldCondition>>, // Conditional visibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldCondition {
    pub field: String,                 // Field key to check
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    IsEmpty,
    IsNotEmpty,
}
```

### 5.4 Field Values

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum FieldValue {
    Null,
    Text(String),
    RichText(String),                  // Markdown/HTML
    Number(f64),
    Boolean(bool),
    Select(String),
    MultiSelect(Vec<String>),
    Date(DateTime<Utc>),
    Media(Uuid),                       // Reference to media asset
    MediaList(Vec<Uuid>),
    List(Vec<String>),                 // Simple string list
    Group(HashMap<String, FieldValue>),
    Repeater(Vec<HashMap<String, FieldValue>>),
    Reference(Uuid),                   // Reference to another content
    ReferenceList(Vec<Uuid>),
}

impl FieldValue {
    /// Validate value against field definition
    pub fn validate(&self, field: &FieldDefinition) -> Result<(), ValidationError> {
        match (&self, &field.field_type) {
            (FieldValue::Text(s), FieldType::Text(config)) => {
                if let Some(min) = config.min_length {
                    if s.len() < min {
                        return Err(ValidationError::TooShort { min, actual: s.len() });
                    }
                }
                if let Some(max) = config.max_length {
                    if s.len() > max {
                        return Err(ValidationError::TooLong { max, actual: s.len() });
                    }
                }
                Ok(())
            }
            // ... other validations
            _ => Ok(())
        }
    }
}
```

---

## 6. Content Schemas

### 6.1 Content Schema Definition

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSchema {
    pub id: String,                    // "task", "guide", "faq"
    pub name: String,                  // "Task", "Guide", "FAQ"
    pub name_plural: String,           // "Tasks", "Guides", "FAQs"
    pub description: Option<String>,
    pub icon: Option<String>,          // Icon identifier

    // Field definitions
    pub fields: Vec<FieldDefinition>,

    // Behavior settings
    pub settings: SchemaSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaSettings {
    pub has_slug: bool,                // Generate URL slug
    pub slug_field: Option<String>,    // Which field to generate slug from
    pub title_field: String,           // Which field is the "title"
    pub preview_fields: Vec<String>,   // Fields to show in list view
    pub orderable: bool,               // Can manually order items
    pub translatable: bool,            // Supports translations
    pub versioned: bool,               // Track version history
    pub singleton: bool,               // Only one instance (e.g., site settings)
    pub allowed_statuses: Vec<ContentStatus>,
}
```

### 6.2 Content Item

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: Uuid,
    pub schema_id: String,             // References ContentSchema.id
    pub slug: Option<String>,
    pub display_order: i32,
    pub status: ContentStatus,
    pub version: i32,

    // The actual content - flat key-value map
    pub data: HashMap<String, FieldValue>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    Review,
    Published,
    Scheduled,
    Archived,
}
```

### 6.3 Schema Registry

```rust
pub fn build_schemas() -> Vec<ContentSchema> {
    vec![
        // FRONT PAGE SCHEMA
        ContentSchema {
            id: "front".into(),
            name: "Front Page".into(),
            name_plural: "Front Pages".into(),
            description: Some("Homepage sections".into()),
            icon: Some("home".into()),
            fields: vec![
                field("title", "Title", FieldType::Text(TextFieldConfig {
                    max_length: Some(100),
                    ..Default::default()
                })).required(),

                field("description", "Description", FieldType::RichText(Default::default())),

                field("slides", "Slides", FieldType::Repeater(RepeaterFieldConfig {
                    item_label: "Slide".into(),
                    orderable: true,
                    collapsible: true,
                    min_items: Some(1),
                    max_items: Some(10),
                    fields: vec![
                        field("title", "Title", FieldType::Text(Default::default())).required(),
                        field("description", "Description", FieldType::Text(TextFieldConfig {
                            max_length: Some(500),
                            multiline: true,
                            ..Default::default()
                        })),
                        field("image", "Image", FieldType::Media(MediaFieldConfig {
                            allowed_types: vec![MediaType::Image],
                            aspect_ratio: Some(AspectRatio::new(16, 9)),
                            ..Default::default()
                        })).required(),
                        field("cta", "Call to Action", FieldType::Group(GroupFieldConfig {
                            fields: vec![
                                field("text", "Button Text", FieldType::Text(Default::default())),
                                field("action", "Action", FieldType::Select(SelectFieldConfig {
                                    options: vec![
                                        opt("navigate", "Navigate to page"),
                                        opt("external", "External link"),
                                        opt("modal", "Open modal"),
                                    ],
                                    ..Default::default()
                                })),
                                field("target", "Target", FieldType::Text(Default::default())),
                            ],
                        })),
                    ],
                })),
            ],
            settings: SchemaSettings {
                title_field: "title".into(),
                singleton: false,
                orderable: true,
                ..Default::default()
            },
        },

        // TASK SCHEMA
        ContentSchema {
            id: "task".into(),
            name: "Task".into(),
            name_plural: "Tasks".into(),
            description: Some("Earning tasks for users".into()),
            icon: Some("briefcase".into()),
            fields: vec![
                field("title", "Title", FieldType::Text(TextFieldConfig {
                    max_length: Some(100),
                    ..Default::default()
                })).required().translatable().show_in_list(),

                field("description", "Description", FieldType::RichText(Default::default()))
                    .translatable(),

                field("cover_images", "Cover Images", FieldType::MediaList(MediaListFieldConfig {
                    allowed_types: vec![MediaType::Image],
                    min_items: Some(1),
                    max_items: Some(4),
                    ..Default::default()
                })),

                field("status", "Status", FieldType::Select(SelectFieldConfig {
                    options: vec![
                        opt("active", "Active"),
                        opt("coming_soon", "Coming Soon"),
                        opt("deprecated", "Deprecated"),
                    ],
                    default: Some("active".into()),
                    ..Default::default()
                })).show_in_list(),

                field("difficulty", "Difficulty", FieldType::Select(SelectFieldConfig {
                    options: vec![
                        opt("easy", "Easy"),
                        opt("medium", "Medium"),
                        opt("hard", "Hard"),
                    ],
                    ..Default::default()
                })),

                field("duration", "Duration", FieldType::Text(TextFieldConfig {
                    placeholder: Some("e.g., Always running, On-demand".into()),
                    ..Default::default()
                })),

                field("earnings", "Earnings Tiers", FieldType::Repeater(RepeaterFieldConfig {
                    item_label: "Tier".into(),
                    orderable: true,
                    collapsible: true,
                    min_items: Some(1),
                    fields: vec![
                        field("name", "Tier Name", FieldType::Text(Default::default()))
                            .required()
                            .with_placeholder("e.g., 1 Device, 2-3 Devices"),
                        field("min_earnings", "Min Earnings", FieldType::Number(NumberFieldConfig {
                            min: Some(0.0),
                            unit: Some("$".into()),
                            precision: Some(2),
                            ..Default::default()
                        })).required(),
                        field("max_earnings", "Max Earnings", FieldType::Number(NumberFieldConfig {
                            min: Some(0.0),
                            unit: Some("$".into()),
                            precision: Some(2),
                            ..Default::default()
                        })).required(),
                        field("period", "Period", FieldType::Select(SelectFieldConfig {
                            options: vec![
                                opt("hour", "Per Hour"),
                                opt("day", "Per Day"),
                                opt("week", "Per Week"),
                                opt("month", "Per Month"),
                            ],
                            default: Some("month".into()),
                            ..Default::default()
                        })),
                        field("features", "Features", FieldType::List(ListFieldConfig {
                            item_type: Box::new(FieldType::Text(Default::default())),
                            ..Default::default()
                        })),
                        field("is_highlighted", "Highlight this tier", FieldType::Boolean(Default::default())),
                    ],
                })),

                field("requirements", "Requirements", FieldType::List(ListFieldConfig {
                    item_type: Box::new(FieldType::Text(Default::default())),
                    ..Default::default()
                })).translatable(),
            ],
            settings: SchemaSettings {
                has_slug: true,
                slug_field: Some("title".into()),
                title_field: "title".into(),
                preview_fields: vec!["title".into(), "status".into(), "difficulty".into()],
                orderable: true,
                translatable: true,
                versioned: true,
                ..Default::default()
            },
        },

        // GUIDE SCHEMA
        ContentSchema {
            id: "guide".into(),
            name: "Guide".into(),
            name_plural: "Guides".into(),
            description: Some("Step-by-step tutorials".into()),
            icon: Some("book-open".into()),
            fields: vec![
                field("title", "Title", FieldType::Text(TextFieldConfig {
                    max_length: Some(100),
                    ..Default::default()
                })).required().translatable().show_in_list(),

                field("description", "Description", FieldType::RichText(Default::default()))
                    .translatable(),

                field("thumbnail", "Thumbnail", FieldType::Media(MediaFieldConfig {
                    allowed_types: vec![MediaType::Image],
                    aspect_ratio: Some(AspectRatio::new(16, 9)),
                    ..Default::default()
                })),

                field("difficulty", "Difficulty", FieldType::Select(SelectFieldConfig {
                    options: vec![
                        opt("easy", "Easy"),
                        opt("medium", "Medium"),
                        opt("hard", "Hard"),
                    ],
                    ..Default::default()
                })).show_in_list(),

                field("duration_minutes", "Duration (minutes)", FieldType::Number(NumberFieldConfig {
                    min: Some(1.0),
                    max: Some(120.0),
                    step: Some(1.0),
                    unit: Some("min".into()),
                    ..Default::default()
                })),

                field("is_featured", "Featured", FieldType::Boolean(Default::default()))
                    .show_in_list(),

                field("requirements", "Requirements", FieldType::List(ListFieldConfig {
                    item_type: Box::new(FieldType::Text(Default::default())),
                    ..Default::default()
                })).translatable(),

                field("steps", "Steps", FieldType::Repeater(RepeaterFieldConfig {
                    item_label: "Step".into(),
                    orderable: true,
                    collapsible: true,
                    min_items: Some(1),
                    fields: vec![
                        field("title", "Step Title", FieldType::Text(Default::default()))
                            .required(),
                        field("description", "Instructions", FieldType::RichText(Default::default()))
                            .required(),
                        field("image", "Screenshot", FieldType::Media(MediaFieldConfig {
                            allowed_types: vec![MediaType::Image],
                            ..Default::default()
                        })),
                        field("video", "Video", FieldType::Media(MediaFieldConfig {
                            allowed_types: vec![MediaType::Video],
                            ..Default::default()
                        })),
                    ],
                })).translatable(),

                field("related_tasks", "Related Tasks", FieldType::ReferenceList(ReferenceListFieldConfig {
                    allowed_types: vec!["task".into()],
                    display_field: "title".into(),
                    ..Default::default()
                })),
            ],
            settings: SchemaSettings {
                has_slug: true,
                slug_field: Some("title".into()),
                title_field: "title".into(),
                preview_fields: vec!["title".into(), "difficulty".into(), "is_featured".into()],
                orderable: true,
                translatable: true,
                versioned: true,
                ..Default::default()
            },
        },

        // FAQ SCHEMA
        ContentSchema {
            id: "faq".into(),
            name: "FAQ".into(),
            name_plural: "FAQs".into(),
            fields: vec![
                field("category", "Category", FieldType::Select(SelectFieldConfig {
                    options: vec![
                        opt("general", "General"),
                        opt("earning", "Earnings"),
                        opt("technical", "Technical"),
                        opt("account", "Account"),
                    ],
                    ..Default::default()
                })).required().show_in_list(),

                field("question", "Question", FieldType::Text(Default::default()))
                    .required().translatable().show_in_list().searchable(),

                field("answer", "Answer", FieldType::RichText(Default::default()))
                    .required().translatable().searchable(),

                field("tags", "Tags", FieldType::MultiSelect(MultiSelectFieldConfig {
                    options: vec![],
                    allow_custom: true,
                    ..Default::default()
                })),

                field("related_guides", "Related Guides", FieldType::ReferenceList(ReferenceListFieldConfig {
                    allowed_types: vec!["guide".into()],
                    display_field: "title".into(),
                    ..Default::default()
                })),
            ],
            settings: SchemaSettings {
                title_field: "question".into(),
                orderable: true,
                translatable: true,
                ..Default::default()
            },
        },

        // ERROR DOCS SCHEMA
        ContentSchema {
            id: "error".into(),
            name: "Error".into(),
            name_plural: "Errors".into(),
            fields: vec![
                field("code", "Error Code", FieldType::Text(TextFieldConfig {
                    pattern: Some(r"^[A-Z0-9_]+$".into()),
                    ..Default::default()
                })).required().show_in_list(),

                field("title", "Title", FieldType::Text(Default::default()))
                    .required().translatable().show_in_list(),

                field("description", "Description", FieldType::RichText(Default::default()))
                    .translatable(),

                field("severity", "Severity", FieldType::Select(SelectFieldConfig {
                    options: vec![
                        opt("info", "Info"),
                        opt("warning", "Warning"),
                        opt("error", "Error"),
                        opt("critical", "Critical"),
                    ],
                    ..Default::default()
                })).show_in_list(),

                field("causes", "Possible Causes", FieldType::List(ListFieldConfig {
                    item_type: Box::new(FieldType::Text(Default::default())),
                    ..Default::default()
                })).translatable(),

                field("solutions", "Solutions", FieldType::Repeater(RepeaterFieldConfig {
                    item_label: "Solution".into(),
                    orderable: true,
                    collapsible: true,
                    fields: vec![
                        field("title", "Solution Title", FieldType::Text(Default::default()))
                            .required(),
                        field("steps", "Steps", FieldType::List(ListFieldConfig {
                            item_type: Box::new(FieldType::Text(Default::default())),
                            ..Default::default()
                        })).required(),
                        field("success_rate", "Success Rate", FieldType::Number(NumberFieldConfig {
                            min: Some(0.0),
                            max: Some(100.0),
                            unit: Some("%".into()),
                            ..Default::default()
                        })),
                    ],
                })).translatable(),

                field("related_errors", "Related Errors", FieldType::ReferenceList(ReferenceListFieldConfig {
                    allowed_types: vec!["error".into()],
                    display_field: "code".into(),
                    ..Default::default()
                })),
            ],
            settings: SchemaSettings {
                has_slug: true,
                slug_field: Some("code".into()),
                title_field: "title".into(),
                translatable: true,
                ..Default::default()
            },
        },
    ]
}
```

### 6.4 Builder Helpers

```rust
fn field(key: &str, label: &str, field_type: FieldType) -> FieldDefinition {
    FieldDefinition {
        key: key.into(),
        label: label.into(),
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

fn opt(value: &str, label: &str) -> SelectOption {
    SelectOption {
        value: value.into(),
        label: label.into(),
        description: None,
    }
}

impl FieldDefinition {
    fn required(mut self) -> Self {
        self.required = true;
        self
    }

    fn translatable(mut self) -> Self {
        self.translatable = true;
        self
    }

    fn show_in_list(mut self) -> Self {
        self.show_in_list = true;
        self
    }

    fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.into());
        self
    }

    fn with_placeholder(mut self, placeholder: &str) -> Self {
        // Apply to inner config based on type
        self
    }

    fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }

    fn when(mut self, field: &str, op: ConditionOperator, value: serde_json::Value) -> Self {
        let condition = FieldCondition {
            field: field.into(),
            operator: op,
            value,
        };
        self.conditions.get_or_insert_with(Vec::new).push(condition);
        self
    }
}
```

---

## 7. Database Schema

### 7.1 Core Tables

```sql
-- ENUMS
CREATE TYPE content_status AS ENUM (
    'draft', 'review', 'published', 'scheduled', 'archived'
);

CREATE TYPE translation_status AS ENUM (
    'pending', 'in_progress', 'review', 'approved', 'outdated'
);

CREATE TYPE cms_role AS ENUM (
    'viewer', 'translator', 'editor', 'publisher', 'admin'
);

-- SCHEMAS TABLE
CREATE TABLE content_schemas (
    id          VARCHAR(50) PRIMARY KEY,
    definition  JSONB NOT NULL,          -- The ContentSchema
    version     INTEGER NOT NULL DEFAULT 1,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CONTENT ITEMS TABLE
CREATE TABLE content_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_id       VARCHAR(50) NOT NULL REFERENCES content_schemas(id),
    slug            VARCHAR(255),
    display_order   INTEGER NOT NULL DEFAULT 0,
    status          content_status NOT NULL DEFAULT 'draft',
    version         INTEGER NOT NULL DEFAULT 1,

    data            JSONB NOT NULL,       -- All field values

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at    TIMESTAMPTZ,
    created_by      UUID NOT NULL,
    updated_by      UUID NOT NULL,

    scheduled_publish   TIMESTAMPTZ,
    scheduled_unpublish TIMESTAMPTZ,

    UNIQUE(schema_id, slug)
);

-- Indexes
CREATE INDEX idx_content_schema_status ON content_items(schema_id, status);
CREATE INDEX idx_content_display_order ON content_items(schema_id, display_order);
CREATE INDEX idx_content_published_at ON content_items(published_at) WHERE status = 'published';
CREATE INDEX idx_content_scheduled ON content_items(scheduled_publish) WHERE scheduled_publish IS NOT NULL;

-- Full-text search
CREATE INDEX idx_content_search ON content_items
    USING GIN (to_tsvector('english', data::text));
```

### 7.2 Translations

```sql
CREATE TABLE locales (
    code        VARCHAR(10) PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    direction   VARCHAR(3) NOT NULL DEFAULT 'ltr',
    is_default  BOOLEAN NOT NULL DEFAULT FALSE,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE content_translations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id      UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    locale_code     VARCHAR(10) NOT NULL REFERENCES locales(code),
    status          translation_status NOT NULL DEFAULT 'pending',
    data            JSONB NOT NULL,       -- Translated field values only

    translated_by   UUID REFERENCES cms_users(id),
    reviewed_by     UUID REFERENCES cms_users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(content_id, locale_code)
);
```

### 7.3 Media Library

```sql
CREATE TABLE media_folders (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(255) NOT NULL,
    parent_id   UUID REFERENCES media_folders(id),
    path        VARCHAR(1000) NOT NULL,  -- Materialized path
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE media_assets (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    filename            VARCHAR(255) NOT NULL,
    original_filename   VARCHAR(255) NOT NULL,
    mime_type           VARCHAR(100) NOT NULL,
    size_bytes          BIGINT NOT NULL,
    storage_path        VARCHAR(1000) NOT NULL,

    width               INTEGER,
    height              INTEGER,
    variants            JSONB NOT NULL DEFAULT '[]',

    alt_text            TEXT,
    caption             TEXT,
    tags                VARCHAR(100)[] DEFAULT '{}',
    folder_id           UUID REFERENCES media_folders(id),

    usage_count         INTEGER NOT NULL DEFAULT 0,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by          UUID NOT NULL REFERENCES cms_users(id)
);

CREATE INDEX idx_media_folder ON media_assets(folder_id);
CREATE INDEX idx_media_tags ON media_assets USING GIN(tags);

-- Track media usage
CREATE TABLE content_media_usage (
    content_id  UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    media_id    UUID NOT NULL REFERENCES media_assets(id) ON DELETE RESTRICT,
    field_path  VARCHAR(255),  -- e.g., "steps[0].image"
    PRIMARY KEY (content_id, media_id, field_path)
);
```

### 7.4 Version History

```sql
CREATE TABLE content_versions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id      UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    version_number  INTEGER NOT NULL,
    data            JSONB NOT NULL,
    change_summary  TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by      UUID NOT NULL REFERENCES cms_users(id),

    UNIQUE(content_id, version_number)
);
```

### 7.5 Users & Auth

```sql
CREATE TABLE cms_users (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email                   VARCHAR(255) NOT NULL UNIQUE,
    name                    VARCHAR(255) NOT NULL,
    password_hash           VARCHAR(255),
    role                    cms_role NOT NULL DEFAULT 'viewer',
    allowed_content_types   VARCHAR(50)[],
    allowed_locales         VARCHAR(10)[],
    is_active               BOOLEAN NOT NULL DEFAULT TRUE,
    last_login              TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE audit_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES cms_users(id),
    action      VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id   UUID NOT NULL,
    old_value   JSONB,
    new_value   JSONB,
    ip_address  INET,
    user_agent  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_user ON audit_log(user_id);
CREATE INDEX idx_audit_created ON audit_log(created_at DESC);
```

### 7.6 Content Relationships

```sql
CREATE TABLE content_relationships (
    source_id       UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    target_id       UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    relationship    VARCHAR(50) NOT NULL,  -- 'related', 'prerequisite', 'next_step'
    display_order   INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (source_id, target_id, relationship)
);
```

---

## 8. API Design

### 8.1 RESTful Endpoints

```
CONTENT
────────────────────────────────────────────────────────────
GET    /api/cms/content                    List all content
GET    /api/cms/content/:type              List by type
GET    /api/cms/content/:type/:slug        Get single item
POST   /api/cms/content/:type              Create new
PUT    /api/cms/content/:type/:slug        Update
DELETE /api/cms/content/:type/:slug        Delete (soft)

POST   /api/cms/content/:type/:slug/publish      Publish
POST   /api/cms/content/:type/:slug/unpublish    Unpublish
POST   /api/cms/content/:type/:slug/duplicate    Clone
POST   /api/cms/content/:type/reorder            Bulk reorder

TRANSLATIONS
────────────────────────────────────────────────────────────
GET    /api/cms/content/:type/:slug/translations
GET    /api/cms/content/:type/:slug/translations/:locale
PUT    /api/cms/content/:type/:slug/translations/:locale
POST   /api/cms/content/:type/:slug/translations/:locale/approve

VERSIONS
────────────────────────────────────────────────────────────
GET    /api/cms/content/:type/:slug/versions
GET    /api/cms/content/:type/:slug/versions/:version
POST   /api/cms/content/:type/:slug/versions/:version/restore
GET    /api/cms/content/:type/:slug/versions/:v1/diff/:v2

MEDIA
────────────────────────────────────────────────────────────
GET    /api/cms/media                      List all media
GET    /api/cms/media/:id                  Get media details
POST   /api/cms/media/upload               Upload new media
PUT    /api/cms/media/:id                  Update metadata
DELETE /api/cms/media/:id                  Delete (if unused)
GET    /api/cms/media/:id/usage            Get content using this

GET    /api/cms/media/folders              List folders
POST   /api/cms/media/folders              Create folder
PUT    /api/cms/media/folders/:id          Rename/move folder

SCHEMAS
────────────────────────────────────────────────────────────
GET    /api/cms/schemas                    List all schemas
GET    /api/cms/schemas/:id                Get schema definition

USERS (Admin only)
────────────────────────────────────────────────────────────
GET    /api/cms/users
POST   /api/cms/users
PUT    /api/cms/users/:id
DELETE /api/cms/users/:id

PUBLIC DELIVERY API (for uno-app)
────────────────────────────────────────────────────────────
GET    /api/v1/content/:type               Published content only
GET    /api/v1/content/:type/:slug
GET    /api/v1/content/:type/:slug?locale=es
```

### 8.2 Response Structures

```rust
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub meta: Option<ResponseMeta>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<ValidationError>>,
}

#[derive(Serialize)]
pub struct ResponseMeta {
    pub pagination: Option<Pagination>,
    pub version: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub total_items: i64,
    pub total_pages: i32,
}

#[derive(Serialize)]
pub struct ContentListItem {
    pub id: Uuid,
    pub schema_id: String,
    pub slug: Option<String>,
    pub title: String,                   // Extracted from data
    pub status: ContentStatus,
    pub display_order: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by_name: String,
    pub translation_status: HashMap<String, TranslationStatus>,
}

#[derive(Serialize)]
pub struct ContentDetailResponse {
    pub item: ContentItem,
    pub schema: ContentSchema,
    pub translations: Vec<TranslationSummary>,
    pub versions_count: i32,
    pub related_content: Vec<ContentRelationship>,
}
```

---

## 9. Frontend Architecture

### 9.1 Component Structure

```
uno-admin/src/
├── app.rs
├── routes/
│   └── cms/
│       ├── mod.rs
│       ├── dashboard.rs            # CMS overview
│       ├── content/
│       │   ├── mod.rs
│       │   ├── list.rs             # Content listing
│       │   ├── editor.rs           # Content editor
│       │   └── preview.rs          # Live preview
│       ├── media/
│       │   ├── mod.rs
│       │   ├── library.rs          # Media browser
│       │   └── uploader.rs
│       └── settings/
│           ├── mod.rs
│           ├── users.rs
│           └── locales.rs
├── components/
│   └── cms/
│       ├── mod.rs
│       ├── field_renderer.rs       # Generic field rendering
│       ├── fields/
│       │   ├── mod.rs
│       │   ├── text_input.rs
│       │   ├── rich_text.rs
│       │   ├── number_input.rs
│       │   ├── select_input.rs
│       │   ├── media_picker.rs
│       │   ├── list_editor.rs
│       │   ├── group_editor.rs
│       │   ├── repeater_editor.rs
│       │   └── reference_picker.rs
│       ├── media/
│       │   ├── mod.rs
│       │   ├── thumbnail.rs
│       │   └── folder_tree.rs
│       └── shared/
│           ├── mod.rs
│           ├── status_badge.rs
│           ├── translation_indicator.rs
│           ├── version_history.rs
│           └── publish_controls.rs
└── services/
    └── cms/
        ├── mod.rs
        ├── content_api.rs
        ├── schema_api.rs
        ├── media_api.rs
        └── user_api.rs
```

### 9.2 Dynamic Form Rendering

```rust
#[component]
pub fn FieldRenderer(
    field: FieldDefinition,
    value: RwSignal<FieldValue>,
    path: String,
    errors: Signal<Vec<ValidationError>>,
) -> impl IntoView {
    let is_visible = create_memo(move |_| {
        field.conditions.as_ref().map_or(true, |conditions| {
            conditions.iter().all(|c| evaluate_condition(c))
        })
    });

    view! {
        <Show when=move || is_visible.get()>
            <div class="field" class:required=field.required>
                <label>{&field.label}</label>
                {field.description.as_ref().map(|d| view! { <p class="help">{d}</p> })}

                {match &field.field_type {
                    FieldType::Text(config) => view! {
                        <TextInput value=value config=config.clone()/>
                    }.into_view(),

                    FieldType::RichText(config) => view! {
                        <RichTextEditor value=value config=config.clone()/>
                    }.into_view(),

                    FieldType::Number(config) => view! {
                        <NumberInput value=value config=config.clone()/>
                    }.into_view(),

                    FieldType::Boolean(_) => view! {
                        <Checkbox value=value/>
                    }.into_view(),

                    FieldType::Select(config) => view! {
                        <SelectInput value=value options=config.options.clone()/>
                    }.into_view(),

                    FieldType::Media(config) => view! {
                        <MediaPicker value=value config=config.clone()/>
                    }.into_view(),

                    FieldType::Repeater(config) => view! {
                        <RepeaterEditor value=value config=config.clone() path=path/>
                    }.into_view(),

                    // ... other field types
                    _ => view! { <span>"Unsupported"</span> }.into_view(),
                }}

                <FieldErrors path=path errors=errors/>
            </div>
        </Show>
    }
}

#[component]
pub fn ContentForm(
    schema: ContentSchema,
    data: RwSignal<HashMap<String, FieldValue>>,
    errors: Signal<Vec<ValidationError>>,
) -> impl IntoView {
    view! {
        <form class="content-form">
            <For
                each=move || schema.fields.clone()
                key=|field| field.key.clone()
            >
                {|field| {
                    let value = create_rw_signal(
                        data.get().get(&field.key).cloned().unwrap_or(FieldValue::Null)
                    );

                    create_effect(move |_| {
                        data.update(|d| {
                            d.insert(field.key.clone(), value.get());
                        });
                    });

                    view! {
                        <FieldRenderer
                            field=field.clone()
                            value=value
                            path=field.key.clone()
                            errors=errors
                        />
                    }
                }}
            </For>
        </form>
    }
}
```

---

## 10. Content Delivery Strategy

### 10.1 Delivery Options

| Option | Description | Pros | Cons | Best For |
|--------|-------------|------|------|----------|
| **Live API** | uno-app fetches from CMS API | Always current, dynamic | API dependency, latency | Frequently changing content |
| **Static Generation** | Build-time JSON export | Fast, no runtime dependency | Requires rebuild | Rarely changing content |
| **Hybrid CDN** | CDN-cached API responses | Fast + fairly current | Cache invalidation | Most cases |
| **Embedded Bundle** | Content in app bundle | Offline-first, fastest | App update required | Mobile apps |

**Recommendation:** Hybrid approach for uno-app

### 10.2 Static Export Structure

```
/content-export/
├── manifest.json              # Version, last updated, checksums
├── locales/
│   └── index.json             # Available locales
├── front/
│   ├── index.json             # List of sections
│   └── intro.json             # Section content
├── tasks/
│   ├── index.json             # All tasks list
│   ├── telemetry.json
│   ├── caller-id-testing.json
│   └── ...
├── guides/
│   ├── index.json
│   ├── setup.json
│   ├── setup.es.json          # Spanish translation
│   ├── setup.ar.json          # Arabic translation
│   └── ...
├── faq/
│   ├── index.json
│   └── ...
└── media/
    ├── manifest.json          # Media metadata
    └── [uuid]/
        ├── original.png
        ├── thumb.webp
        └── medium.webp
```

---

## 11. Feature Priority Matrix

| Feature | Priority | Complexity | Phase |
|---------|----------|------------|-------|
| Core CRUD operations | Critical | Medium | 1 |
| Schema-driven forms | Critical | High | 1 |
| Schema validation | Critical | Low | 1 |
| Media upload & library | Critical | Medium | 1 |
| Draft/Publish workflow | High | Low | 1 |
| Live preview | High | Medium | 1 |
| Version history | High | Medium | 2 |
| Translation management | High | High | 2 |
| Static content export | High | Medium | 2 |
| Role-based access | Medium | Medium | 2 |
| Content scheduling | Medium | Low | 3 |
| Content relationships | Medium | Medium | 3 |
| Full-text search | Medium | Medium | 3 |
| Bulk operations | Low | Medium | 3 |
| Audit log UI | Low | Low | 4 |
| Analytics integration | Low | High | 4 |

---

## Appendix A: CMS Evaluation

### Current Grade: B+

| Category | Grade |
|----------|-------|
| Content Structure | A- |
| Scalability | B |
| Developer Experience | B+ |
| Non-Technical User Experience | C+ |
| Data Integrity & Validation | C |
| Performance & Delivery | B- |
| Internationalization | B+ |

### After Implementation: A

With the generic schema-driven approach and all Phase 1-2 features implemented.
