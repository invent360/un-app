# Security Analysis - Wallet Authentication

## Executive Summary

The unetwork wallet authentication implementation has **critical security vulnerabilities** related to missing SIWE (Sign-In With Ethereum) standard fields. The most severe issue is the **absence of a server-generated nonce**, which enables replay attacks.

---

## Vulnerability #1: Missing Nonce (CRITICAL)

### Severity: **CRITICAL**

### Description
The SIWE message does not include a `Nonce` field. According to EIP-4361, the nonce is a **required** field that must be:
1. Generated server-side
2. Cryptographically random
3. Single-use (consumed after verification)

### Current Implementation
```
unitynodes.io wants you to sign in with your Ethereum account:
0xea1189990797619a57f9480d0ebc7ffcd220d6f6

I accept the Unetwork Terms of Service: https://unitynodes.io/terms
URI: https://unitynodes.io
Version: 1
Chain ID: 869
Issued At: 2026-05-30T18:15:50.085Z
```

**Missing:** `Nonce: <random-value>`

### Evidence
- Network tab inspection shows **no API call** to fetch a nonce before message generation
- Message is generated entirely client-side using only timestamp
- Screenshots confirm no nonce field in MetaMask signature request

### Attack Vector: Replay Attack

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Attacker   │     │    Victim    │     │   Server     │
└──────┬───────┘     └──────┬───────┘     └──────┬───────┘
       │                    │                    │
       │  1. Intercept signed message            │
       │<───────────────────│                    │
       │                    │                    │
       │  2. Store message + signature           │
       │                    │                    │
       │  [Hours/Days later...]                  │
       │                    │                    │
       │  3. Replay same message + signature     │
       │─────────────────────────────────────────>
       │                    │                    │
       │  4. Server accepts (no nonce check)     │
       │<─────────────────────────────────────────
       │                    │                    │
       │  5. Attacker authenticated as victim    │
       │                    │                    │
```

### Attack Scenarios

1. **Network Interception (MITM)**
   - Attacker intercepts the POST to `/auth/v1/token?grant_type=web3`
   - Captures `message` + `signature`
   - Replays later to authenticate as victim

2. **Malicious dApp**
   - User signs a SIWE message on a malicious site
   - Attacker uses the signature on unetwork

3. **Browser Extension Attack**
   - Malicious extension captures signed messages
   - Replays to gain access

4. **Timestamp-Only "Protection" is Insufficient**
   - The `Issued At` timestamp provides minimal protection
   - Server may accept old timestamps (unclear validation window)
   - Same timestamp + signature can be reused indefinitely

### Impact
- **Account Takeover**: Attacker can authenticate as any user whose signature they've captured
- **Persistent Access**: Single captured signature provides unlimited future access
- **Cross-Site Attacks**: Signatures from other sites could potentially work

### Remediation

**Server-side changes required:**

```javascript
// 1. Add nonce generation endpoint
app.get('/auth/nonce', (req, res) => {
  const nonce = crypto.randomBytes(16).toString('hex');
  // Store nonce with expiry (e.g., 5 minutes)
  await redis.setex(`nonce:${nonce}`, 300, 'valid');
  res.json({ nonce });
});

// 2. Validate nonce during authentication
app.post('/auth/token', (req, res) => {
  const { message, signature } = req.body;
  const parsedMessage = parseSiweMessage(message);

  // Check nonce exists and hasn't been used
  const nonceValid = await redis.get(`nonce:${parsedMessage.nonce}`);
  if (!nonceValid) {
    return res.status(401).json({ error: 'Invalid or expired nonce' });
  }

  // Consume nonce (single-use)
  await redis.del(`nonce:${parsedMessage.nonce}`);

  // Continue with signature verification...
});
```

**Client-side changes required:**

```javascript
// Before requesting signature
const { nonce } = await fetch('/auth/nonce').then(r => r.json());

const message = new SiweMessage({
  domain: 'unitynodes.io',
  address: walletAddress,
  statement: 'I accept the Unetwork Terms of Service...',
  uri: 'https://unitynodes.io',
  version: '1',
  chainId: 869,
  nonce: nonce,  // <-- Add server-generated nonce
  issuedAt: new Date().toISOString(),
  expirationTime: new Date(Date.now() + 5 * 60 * 1000).toISOString()  // 5 min expiry
});
```

---

## Vulnerability #2: Missing Expiration Time (MEDIUM)

### Severity: **MEDIUM**

### Description
The SIWE message does not include an `Expiration Time` field. While optional in EIP-4361, it's strongly recommended for security.

### Risk
- Signed messages remain valid indefinitely
- Combined with missing nonce, this compounds replay attack risk
- Old signatures never "expire"

### Remediation
Add expiration time to SIWE message:

```
Expiration Time: 2026-05-30T18:20:50.085Z  // 5 minutes after Issued At
```

Server should reject messages where:
- `Expiration Time` has passed
- `Issued At` is too old (e.g., > 5 minutes)

---

## Vulnerability #3: Timestamp Validation Unknown (LOW-MEDIUM)

### Severity: **LOW-MEDIUM**

### Description
It's unclear how strictly the server validates the `Issued At` timestamp. If validation is weak or missing:
- Very old signatures could be accepted
- Clock skew attacks possible

### Questions to Investigate
1. Does the server reject messages with old `Issued At`?
2. What's the acceptable time window?
3. Is clock skew handled?

### Remediation
Server should enforce:
```javascript
const issuedAt = new Date(parsedMessage.issuedAt);
const now = new Date();
const maxAge = 5 * 60 * 1000; // 5 minutes

if (now - issuedAt > maxAge) {
  return res.status(401).json({ error: 'Message expired' });
}

if (issuedAt > now + 60000) { // 1 minute future tolerance
  return res.status(401).json({ error: 'Invalid timestamp' });
}
```

---

## Vulnerability #4: Domain Mismatch (INFO)

### Severity: **INFORMATIONAL**

### Observation
- SIWE message domain: `unitynodes.io`
- Actual request origin: `manage.unetwork.app`
- API domain: `api.unityedge.io`

While this works, it could confuse users and complicate domain validation.

### Recommendation
Consider aligning domains or clearly documenting the relationship.

---

## Additional Security Considerations

### 1. API Key Exposure
The Supabase publishable key is exposed in client requests:
```
apikey: sb_publishable_yKqi0fu5vV6G4ryUIMJuzw_NCoFEl1c
```

**Status:** Expected behavior - publishable keys are designed to be public. Row Level Security (RLS) should protect data.

### 2. JWT Security
- Algorithm: HS256 (symmetric)
- Expiry: 3600 seconds (1 hour)
- Refresh token provided

**Recommendation:** Consider RS256 for better key management at scale.

### 3. Session Management
- Session ID tracked in JWT and headers
- Refresh token mechanism in place

**Status:** Appears adequate, but review refresh token rotation policy.

---

## Risk Summary

| Vulnerability | Severity | Exploitability | Impact | Status |
|--------------|----------|----------------|--------|--------|
| Missing Nonce | CRITICAL | Easy | Account Takeover | **NEEDS FIX** |
| Missing Expiration | MEDIUM | Easy | Extended Attack Window | **NEEDS FIX** |
| Timestamp Validation | LOW-MEDIUM | Unknown | Depends on server | **INVESTIGATE** |
| Domain Mismatch | INFO | N/A | Confusion | **OPTIONAL** |

---

## Recommended Fix Priority

1. **IMMEDIATE**: Add server-generated nonce with single-use validation
2. **HIGH**: Add expiration time to SIWE messages
3. **MEDIUM**: Implement strict timestamp validation
4. **LOW**: Review and document domain strategy

---

## Testing Recommendations

### Replay Attack Test
1. Capture a valid authentication request (message + signature)
2. Wait 1 hour
3. Replay the same request
4. **Expected (current):** Authentication succeeds (VULNERABLE)
5. **Expected (fixed):** Authentication fails with "Invalid nonce"

### Timestamp Test
1. Craft a SIWE message with `Issued At` set to 1 day ago
2. Sign with valid wallet
3. Attempt authentication
4. Document whether it succeeds or fails

---

## References

- [EIP-4361: Sign-In with Ethereum](https://eips.ethereum.org/EIPS/eip-4361)
- [SIWE Specification](https://docs.login.xyz/general-information/siwe-overview)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
