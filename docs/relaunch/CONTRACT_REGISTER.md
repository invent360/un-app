# Relaunch contract register

Status: Phase 0 source inventory. Source adapters are evidence of intended interfaces, not confirmation of deployed/provider behavior. Contracts must be reviewed before their feature is enabled. Wallet integration is excluded.

| Contract | Local decision/evidence | Confirmation owner | Enablement condition / safe fallback |
|---|---|---|---|
| Operational authority | One PostgreSQL writer; canonical ID mapping in Phase 1 | Backend/data | No cutover until authorised deployed-schema/export reconciliation; source schemas alone are insufficient |
| Participant/operator identity | Existing shared verifier consumes issuer/audience/trusted RSA keys; it does not issue sessions | Security/product | Confirm provider/contact verification, role/MFA claims, callback/logout/revocation and account linking in Phase 2; access fails closed without configuration |
| Upstream inventory | `uno-api` has PostgREST RPC/Edge Function adapters; URL defaults to `api.unityedge.io`; token is explicit | Integration | Confirm supported contract and authorised fixture; incomplete credentials/dates/IDs are quarantined, never generated |
| Activation/activity/rewards | Local sync types and intended RPCs are not authoritative provider evidence | Integration/finance | Confirm event IDs, pagination, freshness, reward basis and country/device eligibility; show pending/unknown with audited manual evidence where needed |
| Safe release/reuse | Credential exposure is distinct from local reservation expiry | Integration/security | Verify revocation/rotation/reuse guarantees; exposed credentials remain occupied/quarantined without evidence |
| Funding/settlement | UNO funds credits; 50/40/10 versioned agreements required | Finance/integration | Confirm provider deduplication/outcome queries, units/currency/fees/settlement topology; ambiguous outcomes require reconciliation, no blind retry |
| No-referral disposition | Not determined by a NULL database field | Product/finance | Approve explicit versioned policy before enabling a no-referral offer; do not silently reassign the 10% |
| Country/task/device/KYC/redemption | Separate versioned rule states and minimum provider evidence | Product/local operations | Current reviewed evidence required; unknown market/task capability remains unknown/waitlisted |
| Ember media volume | Host-bound single writer; mount `/var/lib/uno-app/media`; external volume, marker, actual mount verification, separate DB volume | Platform | Confirm real attachment/reschedule/ownership/failover semantics before production; missing volume fails readiness, no container-layer fallback |
| Backup/recovery | Encrypted separate host/disk, coherent DB marker + immutable media manifest | Operations | Confirm authorised destination/retention and rehearse restore; local same-host backup is insufficient |
| CMS/locales | Preserve schema/slug/history/relations/assets and ten registered locales | CMS/local reviewers | Source manifests captured; obtain authorised live export and reconcile; stale critical translations block affected campaigns |
| CRM/channels | Optional outbox adapters; baseline web/email support independent | Growth/integration | Verified endpoint/access/template/consent capability required; disabled adapter does not block participant status/help |
| Release promotion | Mandatory build/test/migration/security/evidence gates; one writer | Platform/QA | Exact revision/image digest, verified deployment target and Phase 9 engineering/commercial gates; no automatic multi-region promotion |

No live credentials are recorded here. Production schema/export/provider/volume facts remain external confirmations, with safe feature states until confirmed. Phase 0 can establish runnable fixtures and deployment contracts without asserting those facts are known.
