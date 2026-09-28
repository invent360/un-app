# Preview System Design - No Code Duplication

## Requirement
Preview CMS changes using the **exact same code** as the live page. The only difference is the content version served from the database.

---

## Current State Analysis

### What Already Exists (and should be used)
1. **Guides page** (`/guides`) already supports `?preview_token=xxx` query param
2. **`get_guides_with_preview()`** API function exists (needs completion)
3. **`PreviewBanner`** component exists and works
4. **Content versioning** is fully implemented (draft, published, version history)
5. **Status system** distinguishes draft from published content

### What's Missing (small gaps to fill)
1. Preview token validation in `get_guides_with_preview()`
2. Return draft content when valid preview token provided
3. Token generation in uno-admin

---

## Clean Architecture Design

### Flow Diagram
```
┌─────────────────┐     ┌──────────────────────────────────────────────────────┐
│   uno-admin     │     │                     uno-app                          │
│                 │     │                                                      │
│  Content Editor │     │   ┌──────────┐    ┌─────────────┐    ┌──────────┐  │
│       │         │     │   │  Guides  │───▶│ API: get_   │───▶│ Content  │  │
│       ▼         │     │   │   Page   │    │ guides_with │    │ Service  │  │
│  [Preview Link] │────────▶│          │    │ _preview()  │    │          │  │
│  /guides?       │     │   │ Same CSS │    │             │    │ Returns  │  │
│  preview_token= │     │   │ Same JS  │    │ Validates   │    │ draft or │  │
│  xxx            │     │   │ Same     │    │ token       │    │ published│  │
│                 │     │   │ components│    │             │    │ based on │  │
└─────────────────┘     │   └──────────┘    └─────────────┘    │ token    │  │
                        │        │                              └──────────┘  │
                        │        ▼                                            │
                        │   ┌──────────────┐                                  │
                        │   │PreviewBanner │  (only shows if preview mode)    │
                        │   └──────────────┘                                  │
                        └──────────────────────────────────────────────────────┘
```

### Key Principle: **Zero Page/Component Duplication**
- **Same page**: `/guides` serves both live and preview
- **Same components**: `GuideCard`, `GuideModal`, `GuideStepper`
- **Same CSS**: All styling identical
- **Same WASM**: Single build, behavior differs by parameter

### What Changes Based on Preview Token
| Aspect | Without Token | With Valid Token |
|--------|---------------|------------------|
| Content | Published version only | Draft/latest version |
| Banner | Hidden | Shows PreviewBanner |
| URL | `/guides` | `/guides?preview_token=xxx` |
| API | `get_published_items_by_schema()` | `get_draft_items_by_schema()` |

---

## Implementation Plan

### Phase 1: Complete Existing Infrastructure (uno-app)

**File: `/src/api/guides.rs`**
```rust
// Current (incomplete):
if preview_token.is_some() {
    // TODO: Implement preview token validation
    is_preview = true;
}

// Should become:
if let Some(token) = preview_token {
    if validate_preview_token(&token, "guide").await? {
        is_preview = true;
        // Use get_draft_items_by_schema instead of published
    }
}
```

**File: `/src/server/services/content_item_service.rs`**
Add method to fetch draft/latest content:
```rust
pub async fn get_items_for_preview(
    &self,
    schema_id: &str,
    content_id: Option<&str>,  // Optional: specific content or all
) -> Result<Vec<ContentItem>, ApiError>
```

### Phase 2: Token Generation (uno-admin)

**Preview Token Structure:**
```json
{
  "content_id": "uuid",
  "schema_id": "guide",
  "version": 3,
  "expires_at": "2024-01-15T10:00:00Z",
  "created_by": "user_id"
}
```

**Signed with HMAC** using shared secret between uno-admin and uno-app.

### Phase 3: Update Preview Link Generation (uno-admin)

**File: `/src/ui/pages/content/editor.rs`**

Change preview_action to generate URL pointing to actual page:
```rust
// Instead of: /admin-preview/guide/{content_id}
// Generate:   /guides?preview_token={signed_token}
```

---

## Security Considerations

1. **Token Expiry**: Tokens expire after configurable time (e.g., 1 hour)
2. **Token Scope**: Token only valid for specific content_id and schema
3. **Signature Verification**: HMAC signature prevents tampering
4. **No Session Required**: Token is self-contained, no user login needed for preview
5. **Audit Logging**: Log all preview token generations and usage

---

## Benefits of This Approach

1. **Zero code duplication** - Same components render live and preview
2. **Guaranteed visual parity** - Preview IS the real page
3. **Simpler maintenance** - One codebase, not two
4. **Better testing** - Changes tested on actual page
5. **Secure** - Signed, expiring tokens with audit trail

---

## Files to Remove (Duplicate Code Created Previously)

### uno-app
- `AdminPreviewPage` component from `/src/routes/preview.rs` (keep `PreviewPage` for token-based preview)
- `AdminPreviewContent` component
- `PreviewGuideCard` / `PreviewSectionCard` components
- Route `/admin-preview/{schema_id}/{content_id}` from `/src/app.rs`

### uno-admin
- `create_schema_content_preview` server function (if it generates admin-preview URLs)

---

## Next Steps

1. [ ] Remove duplicate preview code (listed above)
2. [ ] Complete `validate_preview_token()` in uno-app
3. [ ] Add `get_items_for_preview()` to content service
4. [ ] Update `get_guides_with_preview()` to use draft content
5. [ ] Generate signed preview tokens in uno-admin
6. [ ] Update preview link to point to `/guides?preview_token=xxx`
