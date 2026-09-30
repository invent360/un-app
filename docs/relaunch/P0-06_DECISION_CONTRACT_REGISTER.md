# P0-06: Decision and Contract Register

**Phase 0 Task** | **Status**: Initial Draft | **Date**: 2026-09-29

This document records every external dependency, integration contract, and architectural decision required before enabling production features. Each unknown has an owner, enablement condition, and safe fallback.

---

## Table of Contents

1. [Identity and Authentication](#1-identity-and-authentication)
2. [Upstream Inventory Provider](#2-upstream-inventory-provider)
3. [Activation and Activity Tracking](#3-activation-and-activity-tracking)
4. [Funding and Credits](#4-funding-and-credits)
5. [Settlement and Payments](#5-settlement-and-payments)
6. [Eligibility and KYC](#6-eligibility-and-kyc)
7. [Storage and Volume](#7-storage-and-volume)
8. [Backup and Recovery](#8-backup-and-recovery)
9. [Agreement and Revenue Split](#9-agreement-and-revenue-split)
10. [Rounding Policy](#10-rounding-policy)
11. [Referral Policy](#11-referral-policy)
12. [CRM and Messaging](#12-crm-and-messaging)

---

## 1. Identity and Authentication

### 1.1 Participant Identity Provider

| Attribute | Value |
|-----------|-------|
| **Contract** | JWT RS256 tokens from external identity provider |
| **Required Fields** | `sub`, `iss`, `aud`, `exp`, `iat`, `kid` |
| **Optional Fields** | `amr` (authentication methods), `auth_time`, `email`, `phone_number` |
| **Status** | UNCONFIRMED |
| **Owner** | Security Lead |
| **Enablement Condition** | Trusted issuer configured via `UNO_SESSION_ISSUER` |
| **Safe Fallback** | New registration and claims disabled; existing sessions read-only |
| **Evidence Required** | Verified issuer public keys, token lifecycle documentation |

**Environment Variables:**
- `UNO_SESSION_PUBLIC_KEYS` (JSON object of kid → PEM)
- `UNO_SESSION_ISSUER` (trusted issuer URL)
- `UNO_SESSION_AUDIENCE` (expected audience claim)

### 1.2 Contact Verification

| Attribute | Value |
|-----------|-------|
| **Contract** | Verified email and/or phone from identity provider |
| **Status** | UNCONFIRMED |
| **Owner** | Security Lead |
| **Enablement Condition** | Provider includes `email_verified` or `phone_number_verified` claims |
| **Safe Fallback** | Manual verification via support workflow |
| **Evidence Required** | Sample verified tokens, provider documentation |

### 1.3 Operator/Admin Authentication

| Attribute | Value |
|-----------|-------|
| **Contract** | MFA-enabled sessions with editorial/operator roles |
| **Required Claims** | `amr` must include `mfa` for privileged operations |
| **Status** | IMPLEMENTED (session verification) |
| **Owner** | Security Lead |
| **Enablement Condition** | Valid MFA session with appropriate role |
| **Safe Fallback** | Privileged operations require recent `auth_time` |

---

## 2. Upstream Inventory Provider

### 2.1 License Inventory Source

| Attribute | Value |
|-----------|-------|
| **Contract** | Unetwork API for license data sync |
| **Endpoint** | Configured via `UNETWORK_API_KEY` and `UNETWORK_BASE_URL` |
| **Status** | PARTIAL (client exists, contract unverified) |
| **Owner** | Backend/Integration Lead |
| **Enablement Condition** | API credentials verified, endpoint accessible |
| **Safe Fallback** | Manual CSV import with validation-only mode |
| **Evidence Required** | API contract documentation, test credentials, sync test results |

**Required Capabilities:**
- Get all license IDs
- Get license details (including validity dates, credentials)
- Get lease details
- Pagination with cursor support

### 2.2 License Import Contract

| Attribute | Value |
|-----------|-------|
| **Contract** | CSV/API import with full provenance |
| **Required Fields** | External ID, provider, credential, start date, end date, agreement version |
| **Rejected** | Missing credentials, invalid dates, fabricated IDs |
| **Status** | PARTIAL (CSV parsing exists, full validation pending) |
| **Owner** | Backend Lead |
| **Enablement Condition** | Validation passes with zero quarantined rows |
| **Safe Fallback** | Validation-only mode with detailed error report |

---

## 3. Activation and Activity Tracking

### 3.1 Activation Evidence

| Attribute | Value |
|-----------|-------|
| **Contract** | Provider confirms device activated with license |
| **Status** | UNCONFIRMED |
| **Owner** | Integration Lead |
| **Enablement Condition** | Provider API supports activation query |
| **Safe Fallback** | Manual verification workflow with evidence upload |
| **Evidence Required** | API documentation, sample activation responses |

**Key Principle:** Copying a license code = ISSUED, not ACTIVATED. Activation requires provider confirmation.

### 3.2 Activity/Productivity Evidence

| Attribute | Value |
|-----------|-------|
| **Contract** | Provider reports daily device activity/task completion |
| **Status** | UNCONFIRMED |
| **Owner** | Integration Lead |
| **Enablement Condition** | Activity endpoint accessible with pagination |
| **Safe Fallback** | Display "activity unknown" state |
| **Evidence Required** | API documentation, sample activity data |

### 3.3 D1/D3/D7/D30 Metrics

| Metric | Definition | Status |
|--------|------------|--------|
| **D1** | Installation confirmed within 24h of issuance | UNCONFIRMED |
| **D3** | Activity/data-cost observation by day 3 | UNCONFIRMED |
| **D7** | Rewarded activity on >=4 of 7 days | DEFINED |
| **D30** | Cohort retention using original denominator | DEFINED |

---

## 4. Funding and Credits

### 4.1 Credit Provider Contract

| Attribute | Value |
|-----------|-------|
| **Contract** | UNO funds credits for participants |
| **Provider** | To be confirmed |
| **Status** | UNCONFIRMED |
| **Owner** | Finance/Backend Lead |
| **Enablement Condition** | Provider API/workflow documented and tested |
| **Safe Fallback** | Credit funding disabled; manual reconciliation |
| **Evidence Required** | Provider contract, API documentation, test transactions |

### 4.2 Credit Order Workflow

| Attribute | Value |
|-----------|-------|
| **Contract** | Idempotent credit orders with confirmation |
| **Fields** | Amount, currency, period, payer, approval, idempotency key |
| **Outcomes** | Confirmed, failed, unknown (pending reconciliation) |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Finance Lead |
| **Enablement Condition** | Provider idempotency verified |
| **Safe Fallback** | Unknown outcomes routed to manual reconciliation |

---

## 5. Settlement and Payments

### 5.1 Settlement Topology

| Attribute | Value |
|-----------|-------|
| **Contract** | Revenue flows from provider → UNO → participants/agents |
| **Status** | UNCONFIRMED |
| **Owner** | Finance Lead |
| **Enablement Condition** | Settlement flow documented and tested |
| **Safe Fallback** | Settlement disabled; balances shown as "pending" |
| **Evidence Required** | Settlement contract, timing, thresholds, fees |

### 5.2 Payment Authorization

| Attribute | Value |
|-----------|-------|
| **Contract** | Thresholds, approvals, and audit trail for payments |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Finance Lead |
| **Enablement Condition** | Approval workflow configured |
| **Safe Fallback** | All payments require manual approval |

### 5.3 Withdrawal Channels

| Attribute | Value |
|-----------|-------|
| **Contract** | Supported payout methods by country |
| **Status** | UNCONFIRMED |
| **Owner** | Operations Lead |
| **Enablement Condition** | Per-country payout verification |
| **Safe Fallback** | Display "payout guidance unavailable" |

---

## 6. Eligibility and KYC

### 6.1 Country/Task Eligibility Rules

| Attribute | Value |
|-----------|-------|
| **Contract** | Versioned rules per country/task/device |
| **Status** | PARTIAL (rules structure exists) |
| **Owner** | Product/Backend Lead |
| **Enablement Condition** | Rules reviewed and approved per market |
| **Safe Fallback** | Unknown eligibility = not eligible |

### 6.2 KYC Evidence

| Attribute | Value |
|-----------|-------|
| **Contract** | Provider-verified identity, minimal local storage |
| **Status** | UNCONFIRMED |
| **Owner** | Compliance Lead |
| **Enablement Condition** | KYC provider integrated |
| **Safe Fallback** | Manual review workflow |
| **Principle** | Keep provider KYC status separate; minimize stored evidence |

---

## 7. Storage and Volume

### 7.1 Dedicated Ember Volume

| Attribute | Value |
|-----------|-------|
| **Contract** | Persistent local storage on dedicated volume |
| **Required** | Mount at `FILE_STORAGE_LOCAL_PATH` with `.uno-volume` marker |
| **Status** | PARTIAL (local backend implemented, mount unverified) |
| **Owner** | Platform/Operations Lead |
| **Enablement Condition** | Volume marker present, identity verified |
| **Safe Fallback** | Media readiness fails; uploads disabled |
| **Evidence Required** | Volume provisioning documentation, mount verification |

### 7.2 Volume Permissions

| Attribute | Value |
|-----------|-------|
| **Contract** | Application has read/write; no world access |
| **Filesystem** | No-follow operations, descriptor-relative paths |
| **Status** | IMPLEMENTED (local backend) |
| **Owner** | Platform Lead |
| **Evidence Required** | Permission tests, traversal/symlink rejection tests |

### 7.3 Volume Capacity

| Attribute | Value |
|-----------|-------|
| **Contract** | Separate from PostgreSQL/WAL capacity |
| **Thresholds** | Disk/inode monitoring with graceful full behavior |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Operations Lead |
| **Enablement Condition** | Monitoring configured |
| **Safe Fallback** | Uploads disabled at threshold |

---

## 8. Backup and Recovery

### 8.1 PostgreSQL Backup

| Attribute | Value |
|-----------|-------|
| **Contract** | Regular backups to separate host/disk |
| **Targets** | Metadata RPO <= 15 min, RTO <= 4 hours |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Operations Lead |
| **Enablement Condition** | Backup destination configured and tested |
| **Safe Fallback** | Alert on backup failure |
| **Evidence Required** | Restore drill results |

### 8.2 Media Backup

| Attribute | Value |
|-----------|-------|
| **Contract** | Media backups to separate location |
| **Targets** | Media RPO <= 24 hours |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Operations Lead |
| **Enablement Condition** | Backup job configured |
| **Evidence Required** | Restore drill with hash verification |

### 8.3 Recovery Testing

| Attribute | Value |
|-----------|-------|
| **Contract** | Regular restore drills |
| **Frequency** | Before launch, quarterly thereafter |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Operations Lead |
| **Evidence Required** | Documented restore with reconciliation |

---

## 9. Agreement and Revenue Split

### 9.1 Version 1 Agreement

| Attribute | Value |
|-----------|-------|
| **Contract** | ULO 50% / UNO 40% / Referral 10% |
| **Basis Points** | `ulo_bps=5000`, `uno_bps=4000`, `referral_bps=1000` |
| **Sum Check** | Must equal 10000 |
| **Status** | DEFINED (models exist) |
| **Owner** | Finance/Product Lead |
| **Enablement Condition** | Agreement version locked per issuance |
| **Safe Fallback** | Invalid shares rejected at all boundaries |

### 9.2 No-Referral Disposition

| Attribute | Value |
|-----------|-------|
| **Contract** | If no eligible referrer, 10% goes to referral/support reserve |
| **Status** | NOT DECIDED |
| **Owner** | Finance Lead |
| **Decision Required** | Where does the 10% go when no referrer exists? |
| **Options** | Support reserve, UNO, redistribute to participant |
| **Safe Fallback** | Hold in reserve until policy defined |

---

## 10. Rounding Policy

### 10.1 Financial Arithmetic

| Attribute | Value |
|-----------|-------|
| **Contract** | Integer minor units (cents) with documented residual rounding |
| **Example** | $100 pool: $50.00 ULO, $40.00 UNO, $10.00 referral |
| **Residual** | Sub-cent remainders handled per policy |
| **Status** | PARTIALLY IMPLEMENTED |
| **Owner** | Finance Lead |
| **Decision Required** | Residual allocation (favor participant? accumulate?) |
| **Safe Fallback** | Reject amounts that don't divide cleanly |

### 10.2 Currency Handling

| Attribute | Value |
|-----------|-------|
| **Contract** | Preserve source currency; convert at documented rate |
| **Storage** | Integer minor units with currency code |
| **Status** | PARTIAL |
| **Owner** | Finance Lead |
| **Evidence Required** | Rounding test with edge cases |

---

## 11. Referral Policy

### 11.1 Attribution Rules

| Attribute | Value |
|-----------|-------|
| **Contract** | First-qualified-source rule with configurable window |
| **Freeze Point** | Attribution frozen at allocation/terms acceptance |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Product/Backend Lead |
| **Enablement Condition** | Attribution window configured |
| **Safe Fallback** | No referral attribution |

### 11.2 Single-Level Enforcement

| Attribute | Value |
|-----------|-------|
| **Contract** | One level of referral commission only |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Backend Lead |
| **Evidence Required** | Schema enforcement, service validation |

### 11.3 Correction Workflow

| Attribute | Value |
|-----------|-------|
| **Contract** | Corrections require privileged actor, reason, immutable history |
| **Status** | NOT IMPLEMENTED |
| **Owner** | Backend Lead |
| **Safe Fallback** | Corrections routed to support |

---

## 12. CRM and Messaging

### 12.1 HighLevel Integration (Optional)

| Attribute | Value |
|-----------|-------|
| **Contract** | Contact/pipeline mapping, outbound events |
| **Status** | NOT CONFIGURED |
| **Owner** | Integration Lead |
| **Enablement Condition** | API access verified, templates approved |
| **Safe Fallback** | Manual support workflow; feature disabled |

### 12.2 Plai Integration (Optional)

| Attribute | Value |
|-----------|-------|
| **Contract** | Campaign source tracking |
| **Status** | NOT CONFIGURED |
| **Owner** | Integration Lead |
| **Enablement Condition** | API access verified |
| **Safe Fallback** | UTM parameters only |

### 12.3 Messaging Baseline

| Attribute | Value |
|-----------|-------|
| **Contract** | Email/web support works without CRM |
| **Status** | NOT VERIFIED |
| **Owner** | Operations Lead |
| **Enablement Condition** | Fallback support workflow tested |

---

## Summary: Unconfirmed Dependencies

| # | Contract | Owner | Blocking Feature |
|---|----------|-------|------------------|
| 1 | Identity provider | Security Lead | Registration, claims |
| 2 | Unetwork license API | Integration Lead | License sync |
| 3 | Activation provider | Integration Lead | Activation status |
| 4 | Activity provider | Integration Lead | Productivity metrics |
| 5 | Credit provider | Finance Lead | Credit funding |
| 6 | Settlement flow | Finance Lead | Payments |
| 7 | Withdrawal channels | Operations Lead | Payouts |
| 8 | KYC provider | Compliance Lead | Eligibility |
| 9 | Ember volume | Platform Lead | Media uploads |
| 10 | Backup destination | Operations Lead | Data recovery |
| 11 | No-referral disposition | Finance Lead | Revenue allocation |
| 12 | Rounding residuals | Finance Lead | Exact splits |

---

## Next Actions

1. Assign named owners to each unconfirmed contract
2. Obtain documentation for each external provider
3. Create test fixtures for each integration
4. Document safe fallbacks in launch gate checks
5. Review and approve agreement version 1
6. Decide no-referral disposition and rounding policy
