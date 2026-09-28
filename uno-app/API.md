# UNO App API Documentation

## Overview

The UNO App API provides endpoints for license distribution, statistics, and FAQ management.

**Base URL:** `http://localhost:3000/api/v1`

**Content-Type:** `application/json`

---

## Authentication

### Public Endpoints
No authentication required.

### Admin Endpoints
Admin endpoints require HMAC-signed requests:

```json
{
  "client_id": "admin-client",
  "timestamp": 1234567890,
  "nonce": "unique-random-string",
  "signature": "hmac-sha256-hex-signature",
  "payload": { ... }
}
```

**Signature Generation:**
```
message = client_id + ":" + timestamp + ":" + nonce + ":" + JSON(payload)
signature = HMAC-SHA256(secret_key, message)
```

**Requirements:**
- Timestamp must be within 5 minutes of server time
- Nonce must be unique per request

---

## Public Endpoints

### Health Check

#### `GET /health`

Check application health status.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "database": "connected"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Always "ok" if responding |
| `version` | string | Application version |
| `database` | string | "connected" or "disconnected" |

---

### Licenses

#### `GET /licenses/variants`

Get available license split options.

**Response:**
```json
[
  {
    "id": 1,
    "user_share_percentage": 50,
    "operator_share_percentage": 50,
    "lease_duration_months": 12,
    "total_quantity": 100,
    "claimed_count": 25,
    "min_monthly_earnings": null,
    "max_monthly_earnings": null,
    "display_name": "50:50",
    "display_order": 1,
    "is_featured": false
  },
  {
    "id": 2,
    "user_share_percentage": 55,
    "operator_share_percentage": 45,
    "lease_duration_months": 12,
    "total_quantity": 50,
    "claimed_count": 10,
    "display_name": "55:45",
    "display_order": 2,
    "is_featured": false
  },
  {
    "id": 3,
    "user_share_percentage": 60,
    "operator_share_percentage": 40,
    "lease_duration_months": 12,
    "total_quantity": 25,
    "claimed_count": 5,
    "display_name": "60:40",
    "display_order": 3,
    "is_featured": true
  }
]
```

---

#### `POST /licenses/claim`

Claim a license using a lease code.

**Request:**
```json
{
  "lease_code": "ABCD-EFGH-IJKL",
  "device_id": "optional-device-identifier"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lease_code` | string | Yes | License lease code |
| `device_id` | string | No | Device identifier for binding |

**Success Response:**
```json
{
  "success": true,
  "license": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "lease_code": "ABCD-EFGH-IJKL",
    "split_type": "60:40",
    "user_share": 60,
    "operator_share": 40,
    "valid_from": "2024-01-01T00:00:00Z",
    "valid_to": "2025-01-01T00:00:00Z",
    "claimed": true
  },
  "license_id": "550e8400-e29b-41d4-a716-446655440000",
  "license_key": "ABCD-EFGH-IJKL",
  "error": null
}
```

**Error Response:**
```json
{
  "success": false,
  "license": null,
  "license_id": null,
  "license_key": null,
  "error": "License not found"
}
```

**Possible Errors:**
- "License not found"
- "License has already been claimed"
- "License has expired"
- "License is not yet valid"

---

### Statistics

#### `GET /stats/visitors`

Get visitor statistics for a time period.

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `period` | string | "week" | Time period: `day`, `week`, `month`, `year`, `all` |

**Response:**
```json
{
  "period": "week",
  "total_visitors": 1250,
  "unique_visitors": 1250,
  "countries": [
    {
      "country_code": "US",
      "country_name": "United States",
      "visitor_count": 450,
      "percentage": 36.0
    }
  ]
}
```

---

#### `GET /stats/countries`

Get top countries by visitor count.

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `period` | string | "week" | Time period |
| `limit` | integer | 10 | Number of countries to return (1-100) |

**Response:**
```json
[
  {
    "country_code": "US",
    "country_name": "United States",
    "visitor_count": 450,
    "percentage": 36.0
  },
  {
    "country_code": "GB",
    "country_name": "United Kingdom",
    "visitor_count": 200,
    "percentage": 16.0
  }
]
```

---

#### `GET /stats/dashboard`

Get combined dashboard statistics.

**Query Parameters:**
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `period` | string | "week" | Time period |

**Response:**
```json
{
  "visitors": {
    "period": "week",
    "total_visitors": 1250,
    "unique_visitors": 1250,
    "countries": [...]
  },
  "top_countries": [...]
}
```

---

### FAQ

#### `GET /faq`

Get all FAQ items.

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `category` | string | Filter by category |
| `locale` | string | Locale for translations (e.g., "en", "de") |

**Response:**
```json
[
  {
    "id": 1,
    "question": "How do I claim a license?",
    "answer": "Enter your lease code on the claim page...",
    "category": "getting-started",
    "order": 1,
    "is_featured": true,
    "locale": "en"
  }
]
```

---

#### `GET /faq/search`

Search FAQ items.

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | Yes | Search query |
| `locale` | string | No | Locale for translations |

---

#### `GET /faq/categories`

Get FAQ categories with item counts.

**Response:**
```json
[
  { "name": "getting-started", "count": 5 },
  { "name": "earnings", "count": 8 },
  { "name": "technical", "count": 3 }
]
```

---

#### `GET /faq/featured`

Get featured FAQ items.

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `locale` | string | Locale for translations |

---

#### `GET /faq/{id}`

Get a single FAQ item by ID.

**Path Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | FAQ item ID |

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `locale` | string | Locale for translations |

**404 Response:**
```json
{
  "error": "FAQ item not found",
  "code": "NOT_FOUND"
}
```

---

## Admin Endpoints

All admin endpoints require HMAC authentication (see Authentication section).

### `GET /admin/health`

Admin health check (no authentication required).

**Response:**
```json
{
  "status": "ok"
}
```

---

### `POST /admin/licenses`

Publish a batch of new licenses.

**Payload:**
```json
{
  "licenses": [
    {
      "lease_code": "ABCD-EFGH-IJKL",
      "valid_from": "2024-01-01T00:00:00Z",
      "valid_to": "2025-01-01T00:00:00Z",
      "split_type": "60:40"
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "created": 10,
  "errors": []
}
```

---

### `DELETE /admin/licenses`

Revoke licenses.

**Payload:**
```json
{
  "license_ids": [
    "550e8400-e29b-41d4-a716-446655440000"
  ]
}
```

**Response:**
```json
{
  "success": true,
  "revoked": 1
}
```

---

### `POST /admin/licenses/import`

Import licenses from CSV data.

**Payload:**
```json
{
  "csv_data": "lease_code,valid_from,valid_to,split_type\nABCD-EFGH-IJKL,2024-01-01,2025-01-01,60:40",
  "skip_header": true
}
```

**Response:**
```json
{
  "success": true,
  "imported": 100,
  "skipped": 2,
  "errors": ["Row 5: Invalid date format"]
}
```

---

### `POST /admin/licenses/search`

Search licenses with filters.

**Payload:**
```json
{
  "filters": {
    "split_type": "60:40",
    "claimed": false,
    "expired": false
  },
  "pagination": {
    "page": 1,
    "per_page": 20
  }
}
```

**Response:**
```json
{
  "items": [...],
  "total": 150,
  "page": 1,
  "per_page": 20,
  "total_pages": 8
}
```

---

### `POST /admin/summary`

Get license summary statistics.

**Payload:**
```json
{}
```

**Response:**
```json
{
  "total": 1000,
  "claimed": 250,
  "available": 700,
  "expired": 50,
  "by_split_type": [
    {
      "split_type": "50:50",
      "total": 500,
      "claimed": 125,
      "available": 350
    },
    {
      "split_type": "55:45",
      "total": 300,
      "claimed": 75,
      "available": 200
    },
    {
      "split_type": "60:40",
      "total": 200,
      "claimed": 50,
      "available": 150
    }
  ]
}
```

---

## Error Responses

All errors follow this format:

```json
{
  "error": "Human-readable error message",
  "code": "ERROR_CODE"
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `SERVICE_UNAVAILABLE` | 503 | Database not connected |
| `UNAUTHORIZED` | 401 | Invalid HMAC signature or expired timestamp |
| `NOT_FOUND` | 404 | Requested resource not found |
| `BAD_REQUEST` | 400 | Invalid request data |
| `INTERNAL_ERROR` | 500 | Server error |
| `IMPORT_ERROR` | 400 | CSV import validation failed |

---

## Request Tracing

All requests include a unique request ID for tracing:

**Request Header (optional):**
```
x-request-id: your-correlation-id
```

If not provided, a UUID is generated automatically.

**Response Header:**
```
x-request-id: 861af131-b859-43ba-aad2-3a856f6dd29d
```

---

## Rate Limiting

Rate limits may apply to prevent abuse. When rate limited:

**Response:**
```json
{
  "error": "Rate limit exceeded",
  "code": "RATE_LIMIT_EXCEEDED"
}
```

**Headers:**
```
Retry-After: 60
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1234567890
```
