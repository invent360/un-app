# UNO Admin API Integration Documentation

This document describes the REST API integration between **uno-admin** (admin dashboard) and **uno-app** (backend API server).

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              UNO Admin Dashboard                             │
│                              http://localhost:3002                           │
│                                                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐              │
│  │  Content List   │  │  Content Editor │  │ Reviews Dashboard│              │
│  │  (list.rs)      │  │  (editor.rs)    │  │  (dashboard.rs)  │              │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘              │
│           │                    │                    │                        │
│           └────────────────────┼────────────────────┘                        │
│                                │                                             │
│                    ┌───────────▼───────────┐                                 │
│                    │    ContentClient      │                                 │
│                    │  (content_client.rs)  │                                 │
│                    │                       │                                 │
│                    │  - HMAC-SHA256 Auth   │                                 │
│                    │  - HTTP/JSON          │                                 │
│                    └───────────┬───────────┘                                 │
└────────────────────────────────┼─────────────────────────────────────────────┘
                                 │
                                 │ HTTP POST/PUT/GET
                                 │ HMAC Signed Requests
                                 ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              UNO App Backend                                 │
│                              http://localhost:3000                           │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                         REST API Layer                               │    │
│  │                     /api/v1/admin/* endpoints                        │    │
│  │                                                                      │    │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐      │    │
│  │  │content_admin_   │  │review_admin_    │  │  image_handler  │      │    │
│  │  │handler.rs       │  │handler.rs       │  │                 │      │    │
│  │  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘      │    │
│  └───────────┼────────────────────┼────────────────────┼────────────────┘    │
│              │                    │                    │                     │
│              └────────────────────┼────────────────────┘                     │
│                                   │                                          │
│                       ┌───────────▼───────────┐                              │
│                       │   ServiceFactory      │                              │
│                       │                       │                              │
│                       │ - ContentService      │                              │
│                       │ - ReviewRepository    │                              │
│                       │ - ClientRegistry      │                              │
│                       └───────────┬───────────┘                              │
└───────────────────────────────────┼──────────────────────────────────────────┘
                                    │
                                    ▼
                          ┌─────────────────┐
                          │   PostgreSQL    │
                          │   :5438         │
                          └─────────────────┘
```

## Authentication

All admin API endpoints require **HMAC-SHA256** authentication.

### Request Format

```json
{
  "client_id": "uno-admin",
  "timestamp": 1780112289,
  "nonce": "550e8400-e29b-41d4-a716-446655440000",
  "signature": "a1b2c3d4e5f6...",
  "payload": {
    // Request-specific data
  }
}
```

### Signature Computation

```
message = "{client_id}:{timestamp}:{nonce}:{payload_json}"
signature = HMAC-SHA256(secret_key, message)
```

### Environment Variables

| Variable | uno-admin | uno-app |
|----------|-----------|---------|
| Client ID | `UNO_CLIENT_ID=uno-admin` | `ADMIN_CLIENT_ID=uno-admin` |
| Secret Key | `UNO_SECRET_KEY=your-secret` | `ADMIN_SECRET_KEY=your-secret` |
| API URL | `UNO_API_URL=http://localhost:3000` | N/A |

---

## API Endpoints

### 1. Content Management

#### POST `/api/v1/admin/contents`
Create new content.

**uno-admin caller:** `src/ui/pages/content/editor.rs:69`
```rust
client.create_content(request)
```

**Request Payload:**
```json
{
  "content_type": "task|guide|faq|error",
  "slug": "unique-slug",
  "content": { "title": "...", "description": "..." },
  "translations": {},
  "display_order": 0,
  "is_featured": false,
  "change_summary": "Initial creation"
}
```

**Response:** `201 Created`
```json
{
  "id": 100,
  "content_type": "task",
  "slug": "unique-slug",
  "status": "draft",
  "version": 1,
  "created_at": "2026-05-30T03:38:09Z"
}
```

---

#### PUT `/api/v1/admin/contents/{id}`
Update existing content.

**uno-admin caller:** `src/ui/pages/content/editor.rs:63`
```rust
client.update_content(content_id, request)
```

**Request Payload:** Same as create

**Response:** `200 OK`

---

#### POST `/api/v1/admin/contents/list`
List all content with filters.

**uno-admin caller:** `src/ui/pages/content/list.rs:79`
```rust
client.list_contents(params)
```

**Request Payload:**
```json
{
  "content_type": "task",      // optional
  "status": "published",        // optional
  "include_drafts": true,       // optional
  "page": 1,                    // optional, default 1
  "per_page": 20                // optional, default 20
}
```

**Response:** `200 OK`
```json
{
  "items": [
    {
      "id": 86,
      "content_type": "task",
      "slug": "telemetry",
      "title": "Telemetry Collection",
      "status": "published",
      "version": 1,
      "published_version": 1,
      "is_featured": true,
      "translation_coverage": 11,
      "updated_at": "2026-05-29T08:21:09Z"
    }
  ],
  "total": 15,
  "page": 1,
  "per_page": 20
}
```

---

#### POST `/api/v1/admin/contents/{id}/get`
Get single content detail.

**uno-admin caller:** `src/ui/pages/content/editor.rs:25`
```rust
client.get_content(id)
```

**Response:** `200 OK` - Full content object

---

#### POST `/api/v1/admin/contents/{id}/delete`
Soft delete content.

**uno-admin caller:** `src/api/content_client.rs:502`
```rust
client.delete_content(id)
```

**Response:** `200 OK`
```json
{
  "success": true,
  "message": "Content deleted"
}
```

---

#### POST `/api/v1/admin/contents/{id}/publish`
Publish content (legacy endpoint).

**uno-admin caller:** `src/api/content_client.rs:518`
```rust
client.publish_content(id, published_by)
```

**Request Payload:**
```json
{
  "published_by": "admin"
}
```

**Response:** `200 OK`

---

#### POST `/api/v1/admin/contents/{id}/archive`
Archive content.

**uno-admin caller:** `src/api/content_client.rs:537`
```rust
client.archive_content(id)
```

---

#### POST `/api/v1/admin/contents/{id}/revert`
Revert content to previous version.

**uno-admin caller:** `src/api/content_client.rs:552`
```rust
client.revert_content(id, version, reverted_by)
```

---

### 2. Review Workflow

#### POST `/api/v1/admin/reviews/submit`
Submit content version for review.

**uno-admin caller:** `src/ui/pages/content/editor.rs:87`
```rust
client.submit_for_review(version_id, &submitted_by, notes)
```

**Request Payload:**
```json
{
  "version_id": 16,
  "submitted_by": "admin",
  "notes": "Ready for review"
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "version_id": 16,
  "status": "pending",
  "submitted_by": "admin",
  "submitted_at": "2026-05-30T04:00:00Z"
}
```

---

#### POST `/api/v1/admin/reviews/pending`
Get pending reviews for dashboard.

**uno-admin caller:** `src/ui/pages/reviews/dashboard.rs:17`
```rust
client.get_pending_reviews(Some(50))
```

**Request Payload:**
```json
{
  "limit": 50
}
```

**Response:** `200 OK`
```json
{
  "reviews": [
    {
      "review": {
        "id": 1,
        "version_id": 16,
        "status": "pending",
        "submitted_by": "author",
        "submitted_at": "2026-05-30T04:00:00Z"
      },
      "content_id": 100,
      "content_type": "task",
      "slug": "new-task",
      "title": "New Task",
      "version": 1,
      "change_summary": "Initial creation"
    }
  ],
  "total": 1
}
```

---

#### POST `/api/v1/admin/reviews/my-submissions`
Get reviews submitted by a user.

**uno-admin caller:** `src/api/content_client.rs:694`
```rust
client.get_my_submissions(submitter, limit)
```

---

#### POST `/api/v1/admin/reviews/{id}/approve`
Approve a pending review.

**uno-admin caller:** `src/ui/pages/reviews/dashboard.rs:32`
```rust
client.approve_review(review_id, &reviewed_by, notes)
```

**Request Payload:**
```json
{
  "reviewed_by": "reviewer",
  "notes": "Looks good!"
}
```

---

#### POST `/api/v1/admin/reviews/{id}/request-changes`
Request changes on a review.

**uno-admin caller:** `src/ui/pages/reviews/dashboard.rs:49`
```rust
client.request_changes(review_id, &reviewed_by, &notes)
```

**Request Payload:**
```json
{
  "reviewed_by": "reviewer",
  "notes": "Please fix the typo in paragraph 2"
}
```

---

#### POST `/api/v1/admin/reviews/{id}/reject`
Reject a review.

**uno-admin caller:** `src/ui/pages/reviews/dashboard.rs:66`
```rust
client.reject_review(review_id, &reviewed_by, notes)
```

---

### 3. Publishing

#### POST `/api/v1/admin/publish/direct`
Publish content directly (skip review).

**uno-admin caller:** `src/ui/pages/content/editor.rs:104`
```rust
client.publish_direct(content_id, &published_by, commit_message)
```

**Request Payload:**
```json
{
  "content_id": 100,
  "published_by": "admin",
  "commit_message": "Direct publish for urgent fix"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "published_count": 1,
  "failed_count": 0,
  "errors": []
}
```

---

#### POST `/api/v1/admin/publish/batch`
Batch publish multiple approved content items.

**uno-admin caller:** `src/ui/pages/publish/queue.rs:42`
```rust
client.publish_approved(content_ids, &published_by)
```

**Request Payload:**
```json
{
  "content_ids": [100, 101, 102],
  "published_by": "admin"
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "published_count": 3,
  "failed_count": 0,
  "errors": []
}
```

---

### 4. Preview Tokens

#### POST `/api/v1/admin/preview-tokens`
Create a preview token for draft content.

**uno-admin caller:** `src/ui/pages/content/editor.rs:120`
```rust
client.create_preview_token(version_id, &created_by, Some(24))
```

**Request Payload:**
```json
{
  "version_id": 16,
  "created_by": "reviewer",
  "expires_in_hours": 24
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "version_id": 16,
  "token": "a1b2c3d4e5f6...",
  "created_by": "reviewer",
  "created_at": "2026-05-30T04:00:00Z",
  "expires_at": "2026-05-31T04:00:00Z"
}
```

---

#### GET `/api/v1/preview/{token}`
Get preview content by token (public, no HMAC).

**Response:** `200 OK` - Full content version

---

### 5. Version History

#### POST `/api/v1/admin/versions/{content_id}/history`
Get version history for content.

**uno-admin caller:** `src/ui/pages/content/editor.rs:136`
```rust
client.get_version_history(content_id, Some(20))
```

**Request Payload:**
```json
{
  "limit": 20
}
```

**Response:** `200 OK`
```json
[
  {
    "id": 16,
    "content_id": 100,
    "version": 1,
    "content": { "title": "...", "description": "..." },
    "translations": {},
    "change_summary": "Initial creation",
    "created_at": "2026-05-30T03:38:09Z",
    "created_by": "admin"
  }
]
```

---

#### POST `/api/v1/admin/versions/{content_id}/compare`
Compare two versions.

**uno-admin caller:** `src/api/content_client.rs:811`
```rust
client.compare_versions(content_id, version_a, version_b)
```

**Request Payload:**
```json
{
  "version_a": 1,
  "version_b": 2
}
```

**Response:** `200 OK`
```json
{
  "version_a": 1,
  "version_b": 2,
  "content_id": 100,
  "changes": [
    {
      "field": "title",
      "locale": "en",
      "old_value": "Old Title",
      "new_value": "New Title",
      "change_type": "modified"
    }
  ],
  "total_changes": 1
}
```

---

#### POST `/api/v1/admin/versions/{content_id}/revert`
Revert to a previous version.

**uno-admin caller:** `src/ui/pages/content/editor.rs:151`
```rust
client.revert_to_version(content_id, version, &reverted_by)
```

**Request Payload:**
```json
{
  "version": 1,
  "reverted_by": "admin"
}
```

---

## uno-admin Page to API Mapping

| Page | File | API Endpoints Used |
|------|------|-------------------|
| Content List | `src/ui/pages/content/list.rs` | `POST /contents/list` |
| Content Editor | `src/ui/pages/content/editor.rs` | `POST /contents/{id}/get`, `POST /contents`, `PUT /contents/{id}`, `POST /reviews/submit`, `POST /publish/direct`, `POST /preview-tokens`, `POST /versions/{id}/history`, `POST /versions/{id}/revert` |
| Reviews Dashboard | `src/ui/pages/reviews/dashboard.rs` | `POST /reviews/pending`, `POST /reviews/{id}/approve`, `POST /reviews/{id}/request-changes`, `POST /reviews/{id}/reject`, `POST /preview-tokens` |
| Publish Queue | `src/ui/pages/publish/queue.rs` | `POST /contents/list`, `POST /publish/batch` |

---

## Error Responses

All endpoints return errors in this format:

```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Invalid HMAC signature |
| `NOT_FOUND` | 404 | Resource not found |
| `VALIDATION_ERROR` | 400 | Invalid request data |
| `SERVICE_UNAVAILABLE` | 503 | Database not connected |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Testing

### Using Python

```python
import json, time, uuid, hmac, hashlib, requests

BASE_URL = "http://localhost:3000"
CLIENT_ID = "uno-admin"
SECRET_KEY = "your-secure-secret-key-here-change-in-production"

def sign_request(payload):
    timestamp = int(time.time())
    nonce = str(uuid.uuid4())
    payload_json = json.dumps(payload, separators=(',', ':'))
    message = f"{CLIENT_ID}:{timestamp}:{nonce}:{payload_json}"
    signature = hmac.new(SECRET_KEY.encode(), message.encode(), hashlib.sha256).hexdigest()
    return {
        "client_id": CLIENT_ID,
        "timestamp": timestamp,
        "nonce": nonce,
        "signature": signature,
        "payload": payload
    }

# Example: List contents
payload = {}
signed = sign_request(payload)
response = requests.post(f"{BASE_URL}/api/v1/admin/contents/list", json=signed)
print(response.json())
```

### Using curl

```bash
# Health check (no auth required)
curl http://localhost:3000/api/v1/health

# For authenticated endpoints, use the Python script above
# or implement HMAC signing in your preferred language
```

---

## File References

### uno-app (Backend)

| File | Description |
|------|-------------|
| `src/server/handlers/mod.rs` | Route configuration |
| `src/server/handlers/content_admin_handler.rs` | Content CRUD handlers |
| `src/server/handlers/review_admin_handler.rs` | Review workflow handlers |
| `src/server/app/service_factory.rs` | Dependency injection |
| `src/types/content_admin.rs` | Request/response types |
| `src/types/content_review.rs` | Review workflow types |

### uno-admin (Frontend)

| File | Description |
|------|-------------|
| `src/api/content_client.rs` | HTTP client with HMAC signing |
| `src/ui/pages/content/list.rs` | Content list page |
| `src/ui/pages/content/editor.rs` | Content editor page |
| `src/ui/pages/reviews/dashboard.rs` | Reviews dashboard |
| `src/ui/pages/publish/queue.rs` | Publish queue page |
