# UNO License API - Postman Guide

This guide explains how to use Postman to interact with the UNO License API.

## Prerequisites

1. **Server running**: Start the server with admin credentials:
   ```bash
   ADMIN_CLIENT_ID=admin ADMIN_SECRET_KEY=secret123 cargo leptos serve
   ```

2. **Base URL**: `http://127.0.0.1:3000`

---

## Authentication

All admin endpoints use **HMAC-SHA256 signed requests**. Each request must include:

| Field | Description |
|-------|-------------|
| `client_id` | Your admin client ID (e.g., `admin`) |
| `timestamp` | Unix timestamp in seconds |
| `nonce` | Random 32-character hex string |
| `signature` | HMAC-SHA256 signature |
| `payload` | The actual request data |

### Signature Format

The signature is computed over this string:
```
client_id:timestamp:nonce:payload_json
```

Where `payload_json` is the compact (no whitespace) JSON of your payload.

---

## Setting Up Postman

### Step 1: Create Environment Variables

1. Click the **Environment** dropdown → **Manage Environments** → **Add**
2. Create a new environment called `UNO Local` with these variables:

| Variable | Initial Value |
|----------|---------------|
| `base_url` | `http://127.0.0.1:3000` |
| `client_id` | `admin` |
| `secret_key` | `secret123` |

3. Select this environment from the dropdown.

### Step 2: Create the Pre-request Script

For **every authenticated request**, add this Pre-request Script:

```javascript
// Generate authentication parameters
const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

// Get the payload from request body
const payload = JSON.parse(pm.request.body.raw);
const payloadJson = JSON.stringify(payload);

// Get credentials from environment
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

// Create message to sign: client_id:timestamp:nonce:payload_json
const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;

// Generate HMAC-SHA256 signature
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

// Store for use in request body
pm.environment.set('timestamp', timestamp);
pm.environment.set('nonce', nonce);
pm.environment.set('signature', signature);
pm.environment.set('payload_json', payloadJson);

console.log('Generated auth:', { timestamp, nonce, signature: signature.substring(0, 20) + '...' });
```

---

## API Endpoints

### 1. Upload Licenses

**Endpoint**: `POST {{base_url}}/api/v1/admin/licenses`

**Headers**:
```
Content-Type: application/json
```

**Body** (raw JSON):
```json
{
  "client_id": "{{client_id}}",
  "timestamp": {{timestamp}},
  "nonce": "{{nonce}}",
  "signature": "{{signature}}",
  "payload": {
    "licenses": [
      {
        "id": "0x0111e1758d35de5306c4feec2e87db6fcf593d055b22a32a4d49e1c1d1cb9281",
        "lease_code": "7e457a39-5f5d-41cb-9eda-737ab99923c6",
        "valid_from": "2026-05-24T00:00:00Z",
        "valid_to": "2027-05-24T00:00:00Z",
        "split_type": "50:50"
      },
      {
        "id": "0x02a3f8b91c4d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f",
        "lease_code": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
        "valid_from": "2026-05-24T00:00:00Z",
        "valid_to": "2027-05-24T00:00:00Z",
        "split_type": "55:45"
      },
      {
        "id": "0x03b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b",
        "lease_code": "b2c3d4e5-f6a7-8901-bcde-f23456789012",
        "valid_from": "2026-05-24T00:00:00Z",
        "valid_to": "2027-05-24T00:00:00Z",
        "split_type": "60:40"
      }
    ],
    "idempotency_key": null
  }
}
```

**Pre-request Script**: Use the PAYLOAD ONLY (not the full signed request) for signing:
```javascript
// Define your payload
const payload = {
  "licenses": [
    {
      "id": "0x0111e1758d35de5306c4feec2e87db6fcf593d055b22a32a4d49e1c1d1cb9281",
      "lease_code": "7e457a39-5f5d-41cb-9eda-737ab99923c6",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    }
  ],
  "idempotency_key": null
};

// Generate authentication
const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

// Build the full signed request
const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

// Set as request body
pm.request.body.raw = JSON.stringify(signedRequest);
```

**License Input Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | No | Hex ID (64 chars). Auto-generated if omitted. |
| `lease_code` | string | Yes | UUID format license key |
| `valid_from` | ISO8601 | Yes | Start date |
| `valid_to` | ISO8601 | Yes | End date |
| `split_type` | string | Yes | `"50:50"`, `"55:45"`, or `"60:40"` |

**Success Response** (200):
```json
{
  "created": 3,
  "failed": 0,
  "errors": []
}
```

---

### 2. Search/Fetch Licenses

**Endpoint**: `POST {{base_url}}/api/v1/admin/licenses/search`

**Headers**:
```
Content-Type: application/json
```

**Pre-request Script**:
```javascript
const payload = {
  "filters": {
    "split_type": null,
    "claimed": null,
    "bound_to_device": null,
    "lease_code_search": null,
    "include_expired": false
  },
  "pagination": {
    "page": 1,
    "per_page": 20
  }
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Filter Options**:

| Filter | Type | Description |
|--------|------|-------------|
| `split_type` | string | `"50:50"`, `"55:45"`, `"60:40"`, or `null` for all |
| `claimed` | bool | `true`, `false`, or `null` for all |
| `bound_to_device` | bool | `true`, `false`, or `null` for all |
| `lease_code_search` | string | Partial match search, or `null` |
| `include_expired` | bool | Include expired licenses |

**Pagination**:

| Field | Type | Description |
|-------|------|-------------|
| `page` | int | Page number (starts at 1) |
| `per_page` | int | Results per page (max 100) |

**Success Response** (200):
```json
{
  "items": [
    {
      "id": "0111e175-8d35-de53-06c4-feec2e87db6f",
      "lease_code": "7e457a39-5f5d-41cb-9eda-737ab99923c6",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "Split5050",
      "claimed": false,
      "bound_to_device": false,
      "device_id": null,
      "claimed_at": null,
      "created_at": "2026-05-24T10:30:00Z"
    }
  ],
  "total": 20,
  "page": 1,
  "per_page": 20,
  "total_pages": 1
}
```

---

### 3. Get Summary Statistics

**Endpoint**: `POST {{base_url}}/api/v1/admin/summary`

**Pre-request Script**:
```javascript
const payload = {};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Success Response** (200):
```json
{
  "total": 20,
  "claimed": 3,
  "available": 17,
  "expired": 0,
  "by_split_type": [
    { "split_type": "Split5050", "total": 7, "claimed": 1, "available": 6 },
    { "split_type": "Split5545", "total": 6, "claimed": 1, "available": 5 },
    { "split_type": "Split6040", "total": 7, "claimed": 1, "available": 6 }
  ]
}
```

---

### 4. Revoke Licenses

**Endpoint**: `DELETE {{base_url}}/api/v1/admin/licenses`

**Pre-request Script**:
```javascript
const payload = {
  "license_ids": [
    "0111e175-8d35-de53-06c4-feec2e87db6f"
  ]
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

---

### 5. Health Check (No Auth)

**Endpoint**: `GET {{base_url}}/api/v1/admin/health`

No authentication required.

**Success Response** (200):
```json
{
  "status": "ok"
}
```

---

## Content Management API

The Content Management System (CMS) allows managing tasks, tutorials, and error documentation with full versioning, draft/published states, and internationalization support.

### 6. Create Content

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents`

**Pre-request Script**:
```javascript
const payload = {
  "content_type": "task",
  "slug": "telemetry-collection",
  "content": {
    "title": "Telemetry Collection",
    "description": "Share anonymous network data to earn rewards",
    "image": "/images/tasks/telemetry.png",
    "status": "active",
    "duration": "Always running",
    "difficulty": "easy",
    "requirements": ["Stable internet connection", "Background app permission"],
    "earnings_tiers": [
      {
        "name": "1 Device",
        "min_earnings": 5,
        "max_earnings": 8,
        "period": "month",
        "features": ["Basic connection tasks", "Minimal battery usage"],
        "is_popular": false
      },
      {
        "name": "2-3 Devices",
        "min_earnings": 15,
        "max_earnings": 25,
        "period": "month",
        "features": ["All task types enabled", "Higher uptime bonus"],
        "is_popular": true
      }
    ],
    "direction": "ltr"
  },
  "translations": {
    "es": {
      "title": "Recolección de Telemetría",
      "description": "Comparte datos de red anónimos para ganar recompensas",
      "direction": "ltr"
    },
    "ar": {
      "title": "جمع القياس عن بعد",
      "description": "شارك بيانات الشبكة المجهولة لكسب المكافآت",
      "direction": "rtl"
    }
  },
  "display_order": 1,
  "is_featured": true,
  "change_summary": "Initial content creation"
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Content Types**:

| Type | Description |
|------|-------------|
| `task` | Earning tasks (telemetry, surveys, etc.) |
| `tutorial` | How-to guides and tutorials |
| `error` | Common error documentation |

**Success Response** (201):
```json
{
  "id": 1,
  "content_type": "task",
  "slug": "telemetry-collection",
  "status": "draft",
  "version": 1,
  "published_version": null,
  "published_at": null,
  "created_at": "2026-05-25T10:00:00Z",
  "updated_at": "2026-05-25T10:00:00Z"
}
```

---

### 7. List Contents

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/list`

**Pre-request Script**:
```javascript
const payload = {
  "content_type": "task",
  "status": null,
  "search": null,
  "page": 1,
  "per_page": 20
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Filter Options**:

| Filter | Type | Description |
|--------|------|-------------|
| `content_type` | string | `"task"`, `"tutorial"`, `"error"`, or `null` for all |
| `status` | string | `"draft"`, `"published"`, `"archived"`, or `null` for all |
| `search` | string | Search in title/slug, or `null` |
| `page` | int | Page number (starts at 1) |
| `per_page` | int | Results per page (default 20) |

**Success Response** (200):
```json
{
  "items": [
    {
      "id": 1,
      "content_type": "task",
      "slug": "telemetry-collection",
      "title": "Telemetry Collection",
      "status": "published",
      "version": 2,
      "published_version": 1,
      "is_featured": true,
      "translation_coverage": 44,
      "updated_at": "2026-05-25T12:00:00Z"
    }
  ],
  "total": 5,
  "page": 1,
  "per_page": 20
}
```

---

### 8. Get Content Detail

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/{id}/get`

**Pre-request Script**:
```javascript
const payload = {};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Success Response** (200):
```json
{
  "id": 1,
  "content_type": "task",
  "slug": "telemetry-collection",
  "status": "published",
  "content": {
    "title": "Telemetry Collection",
    "description": "Share anonymous network data to earn rewards",
    "direction": "ltr"
  },
  "translations": {
    "es": { "title": "Recolección de Telemetría", "direction": "ltr" },
    "ar": { "title": "جمع القياس عن بعد", "direction": "rtl" }
  },
  "translation_status": {
    "en": "complete",
    "es": "complete",
    "ar": "complete",
    "fr": "missing",
    "hi": "missing"
  },
  "translation_coverage": 44,
  "display_order": 1,
  "is_featured": true,
  "version": 2,
  "published_version": 1,
  "published_at": "2026-05-25T11:00:00Z",
  "versions": [
    { "version": 1, "change_summary": "Initial creation", "created_at": "2026-05-25T10:00:00Z" },
    { "version": 2, "change_summary": "Added Arabic translation", "created_at": "2026-05-25T12:00:00Z" }
  ],
  "created_at": "2026-05-25T10:00:00Z",
  "updated_at": "2026-05-25T12:00:00Z"
}
```

---

### 9. Update Content

**Endpoint**: `PUT {{base_url}}/api/v1/admin/contents/{id}`

**Pre-request Script**:
```javascript
const payload = {
  "content_type": "task",
  "slug": "telemetry-collection",
  "content": {
    "title": "Telemetry Collection",
    "description": "Updated description with more details",
    "direction": "ltr"
  },
  "translations": {
    "es": { "title": "Recolección de Telemetría", "description": "Descripción actualizada" }
  },
  "change_summary": "Updated description"
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Note**: Updates create a new version automatically. The content stays as draft until published.

---

### 10. Publish Content

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/{id}/publish`

**Pre-request Script**:
```javascript
const payload = {
  "published_by": "admin@example.com"
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Success Response** (200):
```json
{
  "id": 1,
  "content_type": "task",
  "slug": "telemetry-collection",
  "status": "published",
  "version": 2,
  "published_version": 2,
  "published_at": "2026-05-25T14:00:00Z"
}
```

---

### 11. Archive Content

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/{id}/archive`

**Pre-request Script**:
```javascript
const payload = {};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

---

### 12. Revert Content

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/{id}/revert`

**Pre-request Script**:
```javascript
const payload = {
  "version": 1,
  "reverted_by": "admin@example.com"
};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Note**: Reverting creates a new version with the content from the specified version.

---

### 13. Delete Content

**Endpoint**: `POST {{base_url}}/api/v1/admin/contents/{id}/delete`

**Pre-request Script**:
```javascript
const payload = {};

const timestamp = Math.floor(Date.now() / 1000);
const nonce = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

const payloadJson = JSON.stringify(payload);
const clientId = pm.environment.get('client_id');
const secretKey = pm.environment.get('secret_key');

const message = `${clientId}:${timestamp}:${nonce}:${payloadJson}`;
const signature = CryptoJS.HmacSHA256(message, secretKey).toString(CryptoJS.enc.Hex);

const signedRequest = {
    client_id: clientId,
    timestamp: timestamp,
    nonce: nonce,
    signature: signature,
    payload: payload
};

pm.request.body.raw = JSON.stringify(signedRequest);
```

**Note**: This performs a soft delete (sets `is_active = false`).

---

## Public Content Endpoints (No Auth)

These endpoints are for fetching published content from the frontend.

### Get Published Content by Type

**Endpoint**: `GET {{base_url}}/api/v1/contents/{type}?locale=en`

**Parameters**:
- `type`: `task`, `tutorial`, or `error`
- `locale`: Language code (e.g., `en`, `es`, `ar`)

**Example**: `GET /api/v1/contents/task?locale=es`

### Get Featured Content

**Endpoint**: `GET {{base_url}}/api/v1/contents/{type}/featured?locale=en`

### Get Single Content by Slug

**Endpoint**: `GET {{base_url}}/api/v1/contents/{type}/{slug}?locale=en`

**Example**: `GET /api/v1/contents/task/telemetry-collection?locale=ar`

---

## Troubleshooting

### 401 Unauthorized - Invalid Signature

1. Check that `client_id` and `secret_key` match the server's `ADMIN_CLIENT_ID` and `ADMIN_SECRET_KEY`
2. Ensure the payload JSON is compact (no extra whitespace)
3. Verify timestamp is current (within 5 minutes)
4. Make sure nonce is unique for each request

### 404 Not Found

Ensure the server is running and admin routes are properly configured.

### Request Timeout

The signature expires after 5 minutes. Generate a fresh signature for each request.

---

## Quick Test Collection

Import this collection to get started quickly:

```json
{
  "info": {
    "name": "UNO License API",
    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
  },
  "item": [
    {
      "name": "Health Check",
      "request": {
        "method": "GET",
        "url": "{{base_url}}/api/v1/admin/health"
      }
    }
  ]
}
```
