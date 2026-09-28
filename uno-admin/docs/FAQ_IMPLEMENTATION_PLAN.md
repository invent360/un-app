# FAQ Implementation Plan

## Executive Summary

This document provides a comprehensive design for implementing an FAQ management system that integrates seamlessly with the existing uno-admin CMS architecture. The design leverages the generic `Section` type pattern established for the home page while adding FAQ-specific functionality.

---

## 1. Analysis Summary

### 1.1 Existing Architecture

**Schema System (`/src/api/schema_types.rs`):**
- `FieldType` enum with Text, RichText, Number, Boolean, Select, Repeater, etc.
- `FieldDefinition` with key, label, field_type, validation, translations
- `ContentSchema` with fields, settings, versioning
- `ContentItem` with status workflow (Draft → PendingReview → Approved → Published)

**Section System (`/src/api/section_types.rs`):**
- Generic `Section` struct with common fields:
  - `section_type: String`
  - `title: String`
  - `description: String`
  - `display_order: i32`
  - `is_visible: bool`
  - `highlights: Vec<String>`
  - `images: Vec<String>`
  - `videos: Vec<String>`
  - `links: Vec<SectionLink>`
  - `data: serde_json::Value` (type-specific data)

**Content Flow:**
1. Create draft → Save → Preview (with signed token) → Publish
2. Versioning with rollback support
3. Translation support (en, es, fr, ar, hi, tl, sw, pt, id)

**Folder Structure:**
```
docs/content/
├── 00_home/sections/XX_name/body.json
├── 01_tasks/sections/XX_name/body.json
├── 02_guides/sections/XX_name/body.json
├── 03_faq/  (empty - to be populated)
└── 04_errors/ (empty)
```

### 1.2 Current FAQ Implementation (uno-app)

**Location:** `/uno-app/src/routes/faq.rs`
**Status:** Hardcoded static data with 8 FAQ items

**Categories:**
- General (3 items)
- Earnings (2 items)
- Setup (2 items)
- Security (1 item)

**Database Ready:** Migration exists at `20260522100005_faq.up.sql` with full schema but no seed data.

---

## 2. FAQ Schema Design

### 2.1 Question Type Enum

```rust
// In /src/api/faq_types.rs

/// FAQ Question categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum FaqCategory {
    #[default]
    All,
    General,
    Earnings,
    Setup,
    Security,
}

impl FaqCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::General => "general",
            Self::Earnings => "earnings",
            Self::Setup => "setup",
            Self::Security => "security",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::General => "General",
            Self::Earnings => "Earnings",
            Self::Setup => "Setup",
            Self::Security => "Security",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::All => "#64748b",
            Self::General => "#3b82f6",
            Self::Earnings => "#22c55e",
            Self::Setup => "#8b5cf6",
            Self::Security => "#f59e0b",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::All => "folder",
            Self::General => "info",
            Self::Earnings => "revenue",
            Self::Setup => "settings",
            Self::Security => "shield",
        }
    }

    /// Get all selectable categories (excludes "All" which is filter-only)
    pub fn selectable_categories() -> Vec<Self> {
        vec![Self::General, Self::Earnings, Self::Setup, Self::Security]
    }
}
```

### 2.2 FAQ Question Structure

```rust
/// Individual FAQ Question
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FaqQuestion {
    /// Question category
    #[serde(default)]
    pub category: FaqCategory,

    /// The question text
    pub question: String,

    /// The answer text (supports rich text/markdown)
    pub answer: String,

    /// Display order within the category
    #[serde(default)]
    pub display_order: i32,

    /// Whether this question is featured/pinned
    #[serde(default)]
    pub is_featured: bool,

    /// Whether this question is visible
    #[serde(default = "default_true")]
    pub is_visible: bool,
}

fn default_true() -> bool {
    true
}
```

### 2.3 Extended Section Type for FAQ

The FAQ page extends the generic `Section` type. Only the following fields are used:
- `title` - Page title (e.g., "Frequently Asked Questions")
- `description` - Page description/subtitle
- `data.questions` - Array of FaqQuestion objects

```rust
/// FAQ Page Data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FaqPageData {
    /// Page title
    pub title: String,

    /// Page description
    pub description: String,

    /// Array of FAQ questions
    pub questions: Vec<FaqQuestion>,
}

impl FaqPageData {
    /// Create from JSON value
    pub fn from_json(value: &serde_json::Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    /// Convert to JSON value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    /// Get questions filtered by category
    pub fn questions_by_category(&self, category: FaqCategory) -> Vec<&FaqQuestion> {
        if category == FaqCategory::All {
            self.questions.iter().collect()
        } else {
            self.questions.iter().filter(|q| q.category == category).collect()
        }
    }

    /// Get visible questions sorted by display_order
    pub fn visible_questions(&self) -> Vec<&FaqQuestion> {
        let mut questions: Vec<_> = self.questions.iter().filter(|q| q.is_visible).collect();
        questions.sort_by_key(|q| q.display_order);
        questions
    }

    /// Get featured questions
    pub fn featured_questions(&self) -> Vec<&FaqQuestion> {
        self.questions.iter().filter(|q| q.is_featured && q.is_visible).collect()
    }

    /// Count questions by category
    pub fn category_counts(&self) -> std::collections::HashMap<FaqCategory, usize> {
        let mut counts = std::collections::HashMap::new();
        for q in &self.questions {
            if q.is_visible {
                *counts.entry(q.category).or_insert(0) += 1;
            }
        }
        counts
    }
}
```

### 2.4 FAQ Section Schema Definition

```rust
/// Get FAQ section schema for the schema-driven editor
pub fn get_faq_section_schema() -> SectionSchema {
    SectionSchema {
        section_type: "faq",
        label: "FAQ",
        description: "Frequently Asked Questions page with categorized Q&A",
        icon: "question",
        color: "#06b6d4",
        default_title: "Frequently Asked Questions",
        fields: vec![
            FieldDef::text("title", "Page Title", "Frequently Asked Questions"),
            FieldDef::textarea("description", "Page Description", "Find answers to common questions..."),
            FieldDef {
                key: "questions",
                label: "Questions",
                field_type: FieldType::Repeater(vec![
                    FieldDef {
                        key: "category",
                        label: "Category",
                        field_type: FieldType::Select(vec![
                            ("general", "General"),
                            ("earnings", "Earnings"),
                            ("setup", "Setup"),
                            ("security", "Security"),
                        ]),
                        required: true,
                        placeholder: "",
                    },
                    FieldDef::text("question", "Question", "What is UNO?"),
                    FieldDef::textarea("answer", "Answer", "UNO is a platform that..."),
                    FieldDef::boolean("is_featured", "Featured"),
                    FieldDef::boolean("is_visible", "Visible"),
                ]),
                required: true,
                placeholder: "",
            },
        ],
    }
}
```

---

## 3. FAQ CRUD Logic Design

### 3.1 API Client Functions (`/src/api/faq_client.rs`)

```rust
//! FAQ Page API client for CMS operations.

use super::faq_types::*;
use super::schema_types::*;
use leptos::prelude::*;
use server_fn::codec::PostUrl;

/// Schema ID for FAQ page content
pub const FAQ_PAGE_SCHEMA_ID: &str = "faq";

// ========================================
// Server Functions for FAQ Page
// ========================================

/// Fetch FAQ page content
#[server(GetFaqPage, "/api", endpoint = "get_faq_page")]
pub async fn get_faq_page() -> Result<FaqPageResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let params = ContentItemListParams {
            schema_id: Some(FAQ_PAGE_SCHEMA_ID.to_string()),
            status: None,
            search: None,
            is_featured: None,
            page: 1,
            per_page: 1,
        };

        let list = client.list_items(params).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if list.items.is_empty() {
            return Ok(FaqPageResponse {
                id: None,
                data: FaqPageData::default(),
                status: ContentItemStatus::Draft,
                version: 0,
                has_published_version: false,
            });
        }

        let detail = client.get_item(&list.items[0].id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let data = FaqPageData::from_json(&detail.item.data);

        Ok(FaqPageResponse {
            id: Some(detail.item.id),
            data,
            status: detail.item.status,
            version: detail.item.version,
            has_published_version: detail.item.published_version.is_some(),
        })
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Save FAQ page content (create or update)
#[server(SaveFaqPage, "/api", endpoint = "save_faq_page", input = PostUrl)]
pub async fn save_faq_page(
    id: Option<String>,
    data: String,  // JSON stringified FaqPageData
    translations: Option<String>,
    change_summary: Option<String>,
) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let faq_data: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid FAQ data: {}", e)))?;

        // Validate questions array exists
        let questions = faq_data.get("questions")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ServerFnError::new("FAQ page must have questions array"))?;

        // Validate each question
        for (i, question) in questions.iter().enumerate() {
            validate_faq_question(question, i)?;
        }

        let translations_value = translations
            .map(|t| serde_json::from_str(&t))
            .transpose()
            .map_err(|e| ServerFnError::new(format!("Invalid translations JSON: {}", e)))?;

        let request = UpsertContentItemRequest {
            schema_id: FAQ_PAGE_SCHEMA_ID.to_string(),
            slug: Some("faq".to_string()),
            data: faq_data,
            translations: translations_value,
            is_featured: Some(true),
            display_order: Some(1),
            change_summary,
            task_status: None,
        };

        let response = if let Some(existing_id) = id {
            client.update_item(&existing_id, request).await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        } else {
            client.create_item(request).await
                .map_err(|e| ServerFnError::new(e.to_string()))?
        };

        Ok(response.id)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Publish FAQ page
#[server(PublishFaqPage, "/api", endpoint = "publish_faq_page")]
pub async fn publish_faq_page(id: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        client.publish_item(&id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(true)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Get FAQ page versions
#[server(GetFaqPageVersions, "/api", endpoint = "get_faq_page_versions")]
pub async fn get_faq_page_versions(id: String) -> Result<Vec<ContentItemVersion>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let versions = client.get_item_versions(&id).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(versions)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Revert FAQ page to a specific version
#[server(RevertFaqPage, "/api", endpoint = "revert_faq_page")]
pub async fn revert_faq_page(id: String, version: i32) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::schema_client::SchemaClient;

        let client = SchemaClient::from_env()
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let item = client.revert_item(&id, version).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(item.id)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

/// Create preview URL for FAQ page
#[server(CreateFaqPreview, "/api", endpoint = "create_faq_preview")]
pub async fn create_faq_preview(content_id: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use uno_api::auth::{generate_preview_token, PreviewTokenPayload};

        let uno_app_url = std::env::var("UNO_APP_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let preview_secret = std::env::var("PREVIEW_SECRET_KEY")
            .unwrap_or_else(|_| "default-preview-secret-key-change-in-prod".to_string());

        let payload = PreviewTokenPayload::new(&content_id, FAQ_PAGE_SCHEMA_ID);
        let preview_token = generate_preview_token(&payload, preview_secret.as_bytes());

        Ok(format!("{}/faq?preview_token={}", uno_app_url, preview_token))
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server-side only"))
    }
}

// ========================================
// Response Types
// ========================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FaqPageResponse {
    pub id: Option<String>,
    pub data: FaqPageData,
    pub status: ContentItemStatus,
    pub version: i32,
    pub has_published_version: bool,
}

// ========================================
// Validation Helpers
// ========================================

#[cfg(feature = "ssr")]
fn validate_faq_question(question: &serde_json::Value, index: usize) -> Result<(), ServerFnError> {
    let q_text = question.get("question")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if q_text.trim().is_empty() {
        return Err(ServerFnError::new(format!(
            "Question {} must have question text",
            index + 1
        )));
    }

    let answer = question.get("answer")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if answer.trim().is_empty() {
        return Err(ServerFnError::new(format!(
            "Question {} must have an answer",
            index + 1
        )));
    }

    let category = question.get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    let valid_categories = ["general", "earnings", "setup", "security"];
    if !valid_categories.contains(&category) {
        return Err(ServerFnError::new(format!(
            "Question {} has invalid category: {}",
            index + 1, category
        )));
    }

    Ok(())
}
```

### 3.2 API Module Registration

Update `/src/api/mod.rs`:
```rust
pub mod faq_types;
pub mod faq_client;
pub use faq_types::*;
pub use faq_client::*;
```

---

## 4. FAQ Folder Structure

### 4.1 Create Folder Structure

```
/Users/admin/Documents/unetwork/docs/content/03_faq/
└── sections/
    └── 01_faq_page/
        └── body.json
```

### 4.2 FAQ Data File (`body.json`)

```json
{
  "title": "Frequently Asked Questions",
  "description": "Find answers to common questions about UNO, earnings, setup, and security.",
  "questions": [
    {
      "category": "general",
      "question": "What is UNO?",
      "answer": "UNO is a platform that allows you to earn passive income by sharing your unused internet bandwidth. Our app runs in the background and performs small network tasks that generate earnings for you.",
      "display_order": 1,
      "is_featured": true,
      "is_visible": true
    },
    {
      "category": "general",
      "question": "What devices are supported?",
      "answer": "UNO works on Android phones and WiFi routers. iOS support is coming soon. For best results, keep the app running on a device with a stable WiFi connection.",
      "display_order": 2,
      "is_featured": false,
      "is_visible": true
    },
    {
      "category": "general",
      "question": "Can I use multiple devices?",
      "answer": "Yes! You can run UNO on multiple devices with different licenses. Each device earns independently based on its connection quality and uptime.",
      "display_order": 3,
      "is_featured": false,
      "is_visible": true
    },
    {
      "category": "earnings",
      "question": "How much can I earn?",
      "answer": "Earnings vary based on your device type, internet connection quality, and uptime. On average, users earn between $3-15 per month per device.",
      "display_order": 1,
      "is_featured": true,
      "is_visible": true
    },
    {
      "category": "earnings",
      "question": "How do I get paid?",
      "answer": "Earnings accumulate in your account and can be withdrawn once you reach the minimum threshold. We support various payment methods including cryptocurrency and traditional bank transfers.",
      "display_order": 2,
      "is_featured": false,
      "is_visible": true
    },
    {
      "category": "setup",
      "question": "How do I install the app?",
      "answer": "Download the UNO app from the Google Play Store, create an account, and register your license key. The app will start running tasks automatically.",
      "display_order": 1,
      "is_featured": false,
      "is_visible": true
    },
    {
      "category": "setup",
      "question": "What is a license key?",
      "answer": "A license key is a unique code that activates your UNO account. You can get a free license from our website. Each license determines your earning split with the network operator.",
      "display_order": 2,
      "is_featured": false,
      "is_visible": true
    },
    {
      "category": "security",
      "question": "Is my data safe?",
      "answer": "Yes! We only use your internet bandwidth for legitimate network tasks. We never access, collect, or transmit your personal data or browsing history.",
      "display_order": 1,
      "is_featured": true,
      "is_visible": true
    }
  ]
}
```

---

## 5. UI Components

### 5.1 FAQ Editor Page (`/src/ui/pages/content/faq_editor.rs`)

The FAQ editor follows the same pattern as `home_editor.rs`:

1. **State Management:**
   - `item_id`: Current FAQ content item ID
   - `title`: Page title
   - `description`: Page description
   - `questions`: Array of FaqQuestion
   - `status`: Draft/Published
   - `version`: Content version
   - `has_changes`: Dirty flag
   - `save_error/save_success`: UI feedback

2. **UI Layout:**
   - Header with back button, status badge, version
   - Save Draft / Preview / Publish buttons
   - Page Settings card (title, description)
   - Questions Editor card (repeater with category filter)
   - Quick Stats card (total questions, by category, version)

3. **Actions:**
   - Load existing FAQ page on mount
   - Save draft (create or update)
   - Generate preview URL
   - Publish to live

### 5.2 FAQ Question Editor Component (`/src/ui/components/forms/faq_question_editor.rs`)

A specialized component for editing FAQ questions:

1. **Features:**
   - Category dropdown (General, Earnings, Setup, Security)
   - Question text input
   - Answer textarea (with optional rich text)
   - Featured toggle
   - Visibility toggle
   - Drag-to-reorder
   - Delete confirmation

2. **Category Filter:**
   - Tabs to filter questions by category
   - Show counts per category
   - Color-coded category badges

### 5.3 Reuse of Existing Components

The FAQ editor reuses these existing components:
- `Icon` - Icon display
- Text input styling from home editor
- Status badge styling
- Modal pattern from `SectionEditorModal`
- Repeater/list pattern from `HomeSectionEditor`

---

## 6. Route Registration

### 6.1 Add FAQ Route in `app.rs`

```rust
// In the Routes view
<Route path="/content/faq" view=FaqPageEditor />
```

### 6.2 Add FAQ to CMS Navigation

Update `/src/ui/state/types.rs` or navigation component to include FAQ:

```rust
// Add to CMS content types
pub const FAQ_CONTENT_TYPE: &str = "faq";

// Navigation entry
{
    label: "FAQ Page",
    path: "/content/faq",
    icon: "question",
}
```

---

## 7. Implementation Roadmap

### Phase 1: Foundation (Types & Schema)
1. Create `/src/api/faq_types.rs` with:
   - `FaqCategory` enum
   - `FaqQuestion` struct
   - `FaqPageData` struct
2. Update `/src/api/mod.rs` to export FAQ types
3. Add `get_faq_section_schema()` to section schema definitions

### Phase 2: API Layer
1. Create `/src/api/faq_client.rs` with:
   - `get_faq_page()` server function
   - `save_faq_page()` server function
   - `publish_faq_page()` server function
   - `get_faq_page_versions()` server function
   - `revert_faq_page()` server function
   - `create_faq_preview()` server function
2. Register server functions in main.rs
3. Add validation helpers

### Phase 3: UI Components
1. Create `/src/ui/components/forms/faq_question_editor.rs`
   - Category select dropdown
   - Question/answer fields
   - Featured/visible toggles
   - Add/edit/delete/reorder actions
2. Create `/src/ui/pages/content/faq_editor.rs`
   - Page layout following home_editor.rs pattern
   - Integration with FAQ API functions
   - State management for questions

### Phase 4: Navigation & Routing
1. Add FAQ route in `app.rs`
2. Add FAQ to CMS navigation/sidebar
3. Update content list page to show FAQ as option

### Phase 5: Content Migration
1. Create folder structure:
   ```
   /docs/content/03_faq/sections/01_faq_page/body.json
   ```
2. Migrate hardcoded FAQ data from uno-app
3. Create seed script for database population

### Phase 6: Integration Testing
1. Test create/edit/save/publish flow
2. Test preview generation
3. Test version history and rollback
4. Test category filtering
5. Test search functionality
6. Test translations (if applicable)

### Phase 7: Uno-App Integration
1. Update uno-app FAQ page to fetch from CMS API
2. Add preview mode support
3. Remove hardcoded FAQ data
4. Test end-to-end flow

---

## 8. Security Considerations

1. **Authentication:** All API endpoints protected by HMAC-SHA256 signing
2. **Validation:** Server-side validation of all question data
3. **XSS Prevention:** Sanitize answer content (especially if rich text)
4. **CSRF Protection:** Server functions use signed requests
5. **Preview Tokens:** Signed with secret, time-limited

---

## 9. Performance Considerations

1. **Caching:** FAQ content rarely changes, ideal for CDN caching
2. **Pagination:** Not needed (FAQ typically <50 items)
3. **Lazy Loading:** Category sections can lazy-load on expand
4. **JSON Size:** Single JSON file, efficient for delivery

---

## 10. File Summary

### New Files to Create:
```
/src/api/faq_types.rs                          - Types and structs
/src/api/faq_client.rs                         - Server functions
/src/ui/components/forms/faq_question_editor.rs - Question editor component
/src/ui/pages/content/faq_editor.rs            - FAQ page editor
/docs/content/03_faq/sections/01_faq_page/body.json - Initial FAQ data
```

### Files to Modify:
```
/src/api/mod.rs                   - Export FAQ modules
/src/api/section_types.rs         - Add "faq" to get_section_schema (optional)
/src/app.rs                       - Add FAQ route
/src/ui/components/forms/mod.rs   - Export FaqQuestionEditor
/src/ui/pages/content/mod.rs      - Export FaqPageEditor
```

---

## 11. JSON Schema Reference

For completeness, here's the JSON schema for the FAQ body.json:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "FAQ Page Data",
  "type": "object",
  "required": ["title", "questions"],
  "properties": {
    "title": {
      "type": "string",
      "description": "Page title"
    },
    "description": {
      "type": "string",
      "description": "Page description/subtitle"
    },
    "questions": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["category", "question", "answer"],
        "properties": {
          "category": {
            "type": "string",
            "enum": ["general", "earnings", "setup", "security"]
          },
          "question": {
            "type": "string"
          },
          "answer": {
            "type": "string"
          },
          "display_order": {
            "type": "integer",
            "default": 0
          },
          "is_featured": {
            "type": "boolean",
            "default": false
          },
          "is_visible": {
            "type": "boolean",
            "default": true
          }
        }
      }
    }
  }
}
```

---

## 12. Conclusion

This design provides a modular, secure, and performant FAQ management system that:
- Reuses the existing Section-based architecture
- Follows established patterns from home/tasks/guides
- Minimizes code duplication
- Supports the full CRUD + publish workflow
- Enables category-based organization
- Maintains consistency with the rest of the CMS

The implementation can be completed incrementally following the phased roadmap, with each phase delivering testable functionality.
