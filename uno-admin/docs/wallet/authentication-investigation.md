# Wallet Authentication Investigation

## Overview
This document details the investigation of the unetwork dashboard wallet authentication process using MetaMask. The system uses **SIWE (Sign-In With Ethereum / EIP-4361)** combined with **Supabase Auth** for Web3 authentication.

## Goals
1. Understand how the wallet authentication process works
2. Apply the same approach to a locally created ERC20 wallet
3. Determine if an imported wallet can perform authentication with a custom wallet
4. Identify security loopholes and propose fixes

## Investigation Log

### Session Started
**Date:** 2026-05-30

---

## Step-by-Step Authentication Flow

### Phase 1: Role Selection (UI)

**Step 1: Login Page**
- **URL:** `https://manage.unetwork.app/auth/role`
- **Action:** User selects a role:
  - Node Operator - "Manage nodes and licenses you own on-chain"
  - License Operator - "Operate a single or multiple licenses"
  - Claim a License - "Explore the marketplace to choose and claim yours"

**Step 2: Sign In Options**
- **URL:** `https://manage.unetwork.app/auth/uno-options`
- **Options:**
  - Sign In With Web3 (wallet-based authentication)
  - Sign In With QR Code (alternative method)

### Phase 2: Wallet Connection

**Step 3: Wallet Connect Page**
- **URL:** `https://manage.unetwork.app/auth/wallet-connect?role=uno`
- **Initial State:** "No wallet connected"
- **Action:** User clicks "CONNECT WALLET"

**Step 4: Wallet Selection Modal**
- **Provider:** Web3Modal/AppKit (by Reown/WalletConnect)
- **Project ID:** `2b560727f3b2b11ae8001f4e5b4c53bb`
- **Available Wallets:**
  - WalletConnect (QR Code)
  - MetaMask (INSTALLED)
  - Brave Wallet (INSTALLED)
  - Binance Wallet
  - SafePal
  - 60+ more searchable

**API Call - Fetch Wallet List:**
```
GET https://api.web3modal.org/getWallets?projectId=2b560727f3b2b11ae8001f4e5b4c53bb&st=appkit&sv=react-wagmi-1.8.15&page=1&entries=4&chains=eip155%3A869
```

**Step 5: Select MetaMask**
- MetaMask extension popup appears
- Shows "Continue in MetaMask - Accept connection request in the wallet"

**Step 6: MetaMask Connection Popup**
- **Requesting Site:** manage.unetwork.app
- **Account:** Account 5 (Imported accounts) - $0.00
- **Actions:** Cancel / Connect

**On Connect - Background API Calls:**

1. **Balance Check:**
```
POST https://worldmobilechain-mainnet.g.alchemy.com/public
Payload: {"jsonrpc":"2.0","id":2,"method":"eth_getBalance","params":["0xeA1189990797619a57f9480d0EBc7ffcd220d6F6","latest"]}
Response: {"jsonrpc":"2.0","id":2,"result":"0x0"}
```

2. **Identity Lookup:**
```
GET https://rpc.walletconnect.org/v1/identity/0xeA1189990797619a57f9480d0EBc7ffcd220d6F6?sender=0xeA1189990797619a57f9480d0EBc7ffcd220d6F6&projectId=2b560727f3b2b11ae8001f4e5b4c53bb
Response: {"name":null,"avatar":null,"resolvedAt":"2026-05-30T14:50:36.392987287Z"}
```

3. **Analytics Event:**
```
POST https://pulse.walletconnect.org/batch?projectId=2b560727f3b2b11ae8001f4e5b4c53bb
Payload: [{"eventId":"...","props":{"type":"track","event":"CONNECT_SUCCESS","properties":{"method":"browser","name":"MetaMask","view":"Connect","walletRank":20,"caipNetworkId":"eip155:869"},"address":"0xeA1189990797619a57f9480d0EBc7ffcd220d6F6"}}]
```

### Phase 3: Signature & Authentication

**Step 7: Sign In With Wallet**
- **URL:** `https://manage.unetwork.app/auth/wallet-connect?role=uno`
- **UI State:** Wallet connected showing `0xeA11...d6F6` with "Disconnect" option
- **Action:** User clicks "SIGN IN WITH WALLET"

**Step 8: MetaMask Signature Request**
- **Popup Title:** "Signature request"
- **Network:** World Mobile Chain
- **Request from:** manage.unetwork.app

**SIWE Message Format (EIP-4361):**
```
unitynodes.io wants you to sign in with your Ethereum account:
0xea1189990797619a57f9480d0ebc7ffcd220d6f6

I accept the Unetwork Terms of Service: https://unitynodes.io/terms
URI: https://unitynodes.io
Version: 1
Chain ID: 869
Issued At: 2026-05-30T17:49:04.215Z
```

**User clicks "Confirm" to sign the message**

### Phase 4: Token Exchange

**Authentication Request:**
```
POST https://api.unityedge.io/auth/v1/token?grant_type=web3
```

**Request Headers:**
```
apikey: sb_publishable_yKqi0fu5vV6G4ryUIMJuzw_NCoFEl1c
authorization: Bearer sb_publishable_yKqi0fu5vV6G4ryUIMJuzw_NCoFEl1c
content-type: application/json;charset=UTF-8
x-client-info: supabase-js-web/2.87.1
x-supabase-api-version: 2024-01-01
```

**Request Payload:**
```json
{
  "chain": "ethereum",
  "message": "unitynodes.io wants you to sign in with your Ethereum account:\n0xea1189990797619a57f9480d0ebc7ffcd220d6f6\n\nI accept the Unetwork Terms of Service: https://unitynodes.io/terms\nURI: https://unitynodes.io\nVersion: 1\nChain ID: 869\nIssued At: 2026-05-30T17:49:04.215Z",
  "signature": "0xd6c970c7360baef9dc03df39170593c32c2ebc3a04ca93672f31568af9c278231570ac132dc81b194e9547d4b8e64ba3b9f6dc997c67a085a91100ddb9e421ce1c"
}
```

**Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsImtpZCI6InlHbDE2UkxxLzBzTGxac0ciLCJ0eXAiOiJKV1QifQ...",
  "token_type": "bearer",
  "expires_in": 3600,
  "expires_at": 1780166955,
  "refresh_token": "tg25y32zwuev",
  "user": {
    "id": "ad85231e-833c-4140-8510-12761e51e46e",
    "aud": "authenticated",
    "role": "authenticated",
    "app_metadata": {
      "provider": "web3",
      "providers": ["web3"]
    },
    "user_metadata": {
      "custom_claims": {
        "address": "0xea1189990797619a57f9480d0ebc7ffcd220d6f6",
        "chain": "ethereum",
        "domain": "unitynodes.io",
        "network": "869",
        "statement": null
      },
      "sub": "web3:ethereum:0xea1189990797619a57f9480d0ebc7ffcd220d6f6"
    },
    "identities": [{
      "identity_id": "46bcc341-203c-4e8e-ac38-1664a22272f9",
      "id": "web3:ethereum:0xea1189990797619a57f9480d0ebc7ffcd220d6f6",
      "provider": "web3"
    }]
  }
}
```

---

## Technical Findings

### Authentication Mechanism

**Protocol:** SIWE (Sign-In With Ethereum) - EIP-4361
**Backend:** Supabase Auth with Web3 provider extension
**Token Type:** JWT (JSON Web Token) with HS256 signing

### JWT Token Structure

**Header:**
```json
{
  "alg": "HS256",
  "kid": "yGl16RLq/0sLlZsG",
  "typ": "JWT"
}
```

**Payload (Decoded):**
```json
{
  "iss": "https://vtllpagtmncbkywsqccd.supabase.co/auth/v1",
  "sub": "ad85231e-833c-4140-8510-12761e51e46e",
  "aud": "authenticated",
  "exp": 1780166955,
  "iat": 1780163355,
  "email": "",
  "phone": "",
  "app_metadata": {
    "provider": "web3",
    "providers": ["web3"]
  },
  "user_metadata": {
    "custom_claims": {
      "address": "0xea1189990797619a57f9480d0ebc7ffcd220d6f6",
      "chain": "ethereum",
      "domain": "unitynodes.io",
      "network": "869",
      "statement": null
    },
    "email_verified": false,
    "phone_verified": false,
    "sub": "web3:ethereum:0xea1189990797619a57f9480d0ebc7ffcd220d6f6"
  },
  "role": "authenticated",
  "aal": "aal1",
  "amr": [{"method": "web3", "timestamp": 1780163355}],
  "session_id": "1e817c7b-97b5-46bf-b3d3-077900f60d8c",
  "is_anonymous": false
}
```

### API Endpoints Discovered

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/auth/v1/token?grant_type=web3` | POST | Exchange signature for JWT |
| `/auth/v1/user` | GET | Get current user profile |
| `/rest/v1/rpc/wallet_settings_get` | POST | Get wallet settings |
| `/rest/v1/rpc/licenses_get_uno_licenses_summary` | POST | Get license summary |
| `/rest/v1/rpc/rewards_get_allocations_summary` | POST | Get rewards summary |
| `/rest/v1/rpc/rewards_get_balance` | POST | Get rewards balance |

### Infrastructure Components

| Component | Provider | Purpose |
|-----------|----------|---------|
| Auth Backend | Supabase | Authentication & user management |
| API Gateway | Cloudflare | CDN, DDoS protection, rate limiting |
| RPC Node | Alchemy | Blockchain RPC (World Mobile Chain) |
| Wallet Connect | WalletConnect/Reown | Wallet connection protocol |
| Chain | World Mobile Chain | Chain ID 869 |

---

## SIWE Message Anatomy

```
{domain} wants you to sign in with your Ethereum account:
{address}

{statement}
URI: {uri}
Version: {version}
Chain ID: {chain-id}
Nonce: {nonce}           // NOT PRESENT - SECURITY CONCERN
Issued At: {issued-at}
Expiration Time: {expiration-time}  // NOT PRESENT
```

### Current Implementation Fields:
- **Domain:** unitynodes.io
- **Address:** User's Ethereum address (lowercase)
- **Statement:** "I accept the Unetwork Terms of Service: https://unitynodes.io/terms"
- **URI:** https://unitynodes.io
- **Version:** 1
- **Chain ID:** 869 (World Mobile Chain)
- **Issued At:** ISO 8601 timestamp

---

## Authentication Flow Diagram

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Browser   │     │  MetaMask   │     │  Supabase   │     │  Alchemy    │
│   (dApp)    │     │   Wallet    │     │   Auth      │     │    RPC      │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │                   │
       │ 1. Connect Wallet │                   │                   │
       │──────────────────>│                   │                   │
       │                   │                   │                   │
       │ 2. eth_accounts   │                   │                   │
       │<──────────────────│                   │                   │
       │                   │                   │                   │
       │ 3. eth_getBalance │                   │                   │
       │───────────────────────────────────────────────────────────>
       │                   │                   │                   │
       │ 4. Generate SIWE Message              │                   │
       │ (with timestamp)  │                   │                   │
       │                   │                   │                   │
       │ 5. personal_sign  │                   │                   │
       │──────────────────>│                   │                   │
       │                   │                   │                   │
       │ 6. User Confirms  │                   │                   │
       │<──────────────────│                   │                   │
       │   (signature)     │                   │                   │
       │                   │                   │                   │
       │ 7. POST /auth/v1/token?grant_type=web3                    │
       │ (message + signature)                 │                   │
       │──────────────────────────────────────>│                   │
       │                   │                   │                   │
       │                   │  8. Verify signature                  │
       │                   │  - ecrecover(message, signature)      │
       │                   │  - Compare with address in message    │
       │                   │                   │                   │
       │ 9. JWT Token + User                   │                   │
       │<──────────────────────────────────────│                   │
       │                   │                   │                   │
       │ 10. Authenticated API calls           │                   │
       │ (Bearer token)    │                   │                   │
       │──────────────────────────────────────>│                   │
       │                   │                   │                   │
```

---

## Key Cryptographic Operations

### 1. Message Signing (Client-Side - MetaMask)
```javascript
// MetaMask uses personal_sign (EIP-191)
const signature = await ethereum.request({
  method: 'personal_sign',
  params: [message, address]
});
```

### 2. Signature Verification (Server-Side)
```javascript
// Server recovers address from signature
const recoveredAddress = ethers.verifyMessage(message, signature);
// Compare with address in SIWE message
if (recoveredAddress.toLowerCase() === addressFromMessage.toLowerCase()) {
  // Authentication successful
}
```

---

## Response Headers of Interest

```
sb-auth-refresh-token-prefix: tg25y
sb-auth-session-id: 1e817c7b-97b5-46bf-b3d3-077900f60d8c
sb-auth-user-id: ad85231e-833c-4140-8510-12761e51e46e
sb-project-ref: vtllpagtmncbkywsqccd
```

---

---

## Security Analysis

**See detailed analysis:** [security-analysis.md](./security-analysis.md)

### Critical Finding: Missing Nonce

The SIWE implementation is **missing the required `Nonce` field**, which is a critical security vulnerability enabling replay attacks.

**Evidence:**
- No API call to fetch nonce before message generation
- Message generated entirely client-side with only timestamp
- Network tab inspection confirms no nonce endpoint called
- MetaMask signature request shows no Nonce field

**Current Message (VULNERABLE):**
```
unitynodes.io wants you to sign in with your Ethereum account:
0xea1189990797619a57f9480d0ebc7ffcd220d6f6

I accept the Unetwork Terms of Service: https://unitynodes.io/terms
URI: https://unitynodes.io
Version: 1
Chain ID: 869
Issued At: 2026-05-30T18:15:50.085Z
```

**Required Message (SECURE):**
```
unitynodes.io wants you to sign in with your Ethereum account:
0xea1189990797619a57f9480d0ebc7ffcd220d6f6

I accept the Unetwork Terms of Service: https://unitynodes.io/terms
URI: https://unitynodes.io
Version: 1
Chain ID: 869
Nonce: a8f3b2c1d4e5f6789012  <-- MISSING (server-generated, single-use)
Issued At: 2026-05-30T18:15:50.085Z
Expiration Time: 2026-05-30T18:20:50.085Z  <-- MISSING (recommended)
```

---

## Next Steps

1. [x] Analyze security vulnerabilities in the current implementation
2. [ ] Test with locally created wallet (using ethers.js or similar)
3. [ ] Test with imported wallet to custom implementation
4. [ ] Document security recommendations
