#!/bin/bash

# =============================================================================
# License Upload Example - CURL Request
# =============================================================================
#
# This script demonstrates how to upload licenses to the UNO API
# with proper HMAC-SHA256 authentication.
#
# Prerequisites:
# 1. Start PostgreSQL database
# 2. Run migrations
# 3. Start the server with:
#    ADMIN_CLIENT_ID=admin ADMIN_SECRET_KEY=secret123 cargo leptos serve
#
# Usage:
#    ./example_curl.sh
#
# Or with custom credentials:
#    ADMIN_CLIENT_ID=myid ADMIN_SECRET_KEY=mykey API_BASE_URL=http://localhost:3000 ./example_curl.sh
# =============================================================================

set -e

# Configuration
ADMIN_CLIENT_ID="${ADMIN_CLIENT_ID:-admin}"
ADMIN_SECRET_KEY="${ADMIN_SECRET_KEY:-secret123}"
API_BASE_URL="${API_BASE_URL:-http://127.0.0.1:3000}"

# Generate authentication parameters
TIMESTAMP=$(date +%s)
NONCE=$(openssl rand -hex 16)

# =============================================================================
# PAYLOAD: 20 licenses with random split types (50:50, 55:45, 60:40)
# =============================================================================
read -r -d '' PAYLOAD << 'PAYLOAD_EOF' || true
{
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
    },
    {
      "id": "0x04c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c",
      "lease_code": "c3d4e5f6-a7b8-9012-cdef-345678901234",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x05d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d",
      "lease_code": "d4e5f6a7-b8c9-0123-defa-456789012345",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "55:45"
    },
    {
      "id": "0x06e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e",
      "lease_code": "e5f6a7b8-c9d0-1234-efab-567890123456",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    },
    {
      "id": "0x07f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f",
      "lease_code": "f6a7b8c9-d0e1-2345-fabc-678901234567",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x08a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a",
      "lease_code": "a7b8c9d0-e1f2-3456-abcd-789012345678",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "55:45"
    },
    {
      "id": "0x09b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b",
      "lease_code": "b8c9d0e1-f2a3-4567-bcde-890123456789",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    },
    {
      "id": "0x10c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c",
      "lease_code": "c9d0e1f2-a3b4-5678-cdef-901234567890",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x11d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d",
      "lease_code": "d0e1f2a3-b4c5-6789-defa-012345678901",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "55:45"
    },
    {
      "id": "0x12e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e",
      "lease_code": "e1f2a3b4-c5d6-7890-efab-123456789012",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    },
    {
      "id": "0x13f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f",
      "lease_code": "f2a3b4c5-d6e7-8901-fabc-234567890123",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x14a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a",
      "lease_code": "a3b4c5d6-e7f8-9012-abcd-345678901234",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "55:45"
    },
    {
      "id": "0x15b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b",
      "lease_code": "b4c5d6e7-f8a9-0123-bcde-456789012345",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    },
    {
      "id": "0x16c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c",
      "lease_code": "c5d6e7f8-a9b0-1234-cdef-567890123456",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x17d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d",
      "lease_code": "d6e7f8a9-b0c1-2345-defa-678901234567",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "55:45"
    },
    {
      "id": "0x18e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e",
      "lease_code": "e7f8a9b0-c1d2-3456-efab-789012345678",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    },
    {
      "id": "0x19f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f",
      "lease_code": "f8a9b0c1-d2e3-4567-fabc-890123456789",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "50:50"
    },
    {
      "id": "0x20a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a",
      "lease_code": "a9b0c1d2-e3f4-5678-abcd-901234567890",
      "valid_from": "2026-05-24T00:00:00Z",
      "valid_to": "2027-05-24T00:00:00Z",
      "split_type": "60:40"
    }
  ],
  "idempotency_key": null
}
PAYLOAD_EOF

# Compact the payload (remove whitespace)
PAYLOAD_COMPACT=$(echo "$PAYLOAD" | jq -c '.')

# Create the message to sign: client_id:timestamp:nonce:payload_json
MESSAGE="${ADMIN_CLIENT_ID}:${TIMESTAMP}:${NONCE}:${PAYLOAD_COMPACT}"

# Generate HMAC-SHA256 signature
SIGNATURE=$(printf '%s' "$MESSAGE" | openssl dgst -sha256 -hmac "$ADMIN_SECRET_KEY" | sed 's/^.* //')

# Build the signed request
SIGNED_REQUEST=$(jq -n \
  --arg client_id "$ADMIN_CLIENT_ID" \
  --argjson timestamp "$TIMESTAMP" \
  --arg nonce "$NONCE" \
  --arg signature "$SIGNATURE" \
  --argjson payload "$PAYLOAD_COMPACT" \
  '{
    client_id: $client_id,
    timestamp: $timestamp,
    nonce: $nonce,
    signature: $signature,
    payload: $payload
  }')

echo "=============================================="
echo "License Upload Request"
echo "=============================================="
echo ""
echo "Endpoint: POST ${API_BASE_URL}/api/v1/admin/licenses"
echo ""
echo "Authentication:"
echo "  Client ID: ${ADMIN_CLIENT_ID}"
echo "  Timestamp: ${TIMESTAMP}"
echo "  Nonce:     ${NONCE}"
echo "  Signature: ${SIGNATURE}"
echo ""
echo "Payload Summary:"
echo "  Total Licenses: 20"
echo "  Split Types:"
echo "    - 50:50: 7 licenses"
echo "    - 55:45: 6 licenses"
echo "    - 60:40: 7 licenses"
echo ""
echo "=============================================="
echo "CURL Command (copy & paste)"
echo "=============================================="
echo ""
cat << EOF
curl -X POST '${API_BASE_URL}/api/v1/admin/licenses' \\
  -H 'Content-Type: application/json' \\
  -d '${SIGNED_REQUEST}'
EOF

echo ""
echo "=============================================="
echo "Executing Request..."
echo "=============================================="
echo ""

# Execute the curl request
RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${API_BASE_URL}/api/v1/admin/licenses" \
  -H "Content-Type: application/json" \
  -d "$SIGNED_REQUEST")

# Extract body and status code
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response:"
echo "$BODY" | jq . 2>/dev/null || echo "$BODY"
echo ""
echo "HTTP Status Code: $HTTP_CODE"
echo ""
echo "=============================================="
