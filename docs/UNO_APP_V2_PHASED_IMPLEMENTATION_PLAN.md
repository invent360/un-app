# UNO v2 phased implementation plan

Prepared 29 September 2026. Status: execution plan; no phase is certified complete.

Sources: [relaunch requirements](UNO_APP_V2_RELAUNCH_REQUIREMENTS.md) and [implementation analysis](UNO_APP_V2_IMPLEMENTATION_ANALYSIS.md). This plan covers all pending requirements in `uno-app`, `uno-admin`, `uno-api`, `file-storage` and their dependency graph. `ember-multichain` remains excluded. Existing authentication, workspace, storage and readiness repairs are inputs to verify and finish, not completed release gates.

## Delivery rules

- Keep PostgreSQL as the sole application-owned operational authority after cutover. Preserve Rust, Leptos/Actix, useful Ember UI components, ten locales and CMS data.
- Keep all new issuance, funding, payments and campaigns disabled until their applicable gates pass. Every route alias, server function, worker command and WebSocket must enforce the same service invariants.
- Each task must include its migration impact, real-boundary tests and evidence. Finish vertical slices: schema → service → entry points → UI → recovery/observability. Do not close a ticket because a model, screen or table exists.
- Use synthetic fixtures for development. Reconcile representative authorised source exports before production migration. No invented credentials, dates, eligibility, activation, rewards or successful provider outcomes.
- An unavailable external API gets an explicit unavailable/manual-reconciliation path. Automated functionality becomes enabled only after the supported provider contract is verified. Manual evidence must be attributable and auditable.
- Perform tests and reviews in every phase. Phase 9 aggregates release evidence; it does not postpone correctness testing.
- Production deployment, external account configuration, payment execution, outbound messaging and destructive retirement are scheduled actions with their own operational authorization. Creating this plan does not execute them.
- Record task states as `not started`, `in progress`, `blocked`, `implemented awaiting evidence`, or `verified`. A phase passes only when all mandatory tasks and its exit gate are verified.

## Phase overview and dependencies

| Phase | Outcome | Prerequisites | Accountable owner |
|---|---|---|---|
| 0 | Reproducible baseline and confirmed contracts | Current working tree | Platform/backend lead |
| 1 | Canonical PostgreSQL schema and upgrade path | Phase 0 | Data/backend lead |
| 2 | Complete identity, scoped authorization and privacy controls | Phases 0–1 | Security/backend lead |
| 3 | Durable worker, integration authentication and recovery | Phases 1–2 | Backend/integration lead |
| 4 | Correct inventory, publication, referrals and owner-bound issuance | Phases 1–3 | Distribution/backend lead |
| 5 | Reward ledger, credit funding and settlement | Phases 1–4 | Finance/backend lead |
| 6 | Recoverable local media, governed CMS and locale preservation | Phases 1–3; asset/content baseline from Phase 0 | Storage/CMS lead |
| 7 | Complete participant, agent and support journeys | Phases 2, 4–6 | Product/frontend lead |
| 8 | Operator forecasting, reporting and optional integration adapters | Phases 3, 5–7 | Analytics/integration lead |
| 9 | Verified release, migration, recovery and controlled pilot | Phases 0–8 | Operations/QA lead |
| 10 | Remaining P2 features, approved retirement and evidence-led expansion | Phase 9; feature-specific provider access | Product/platform lead |

Phases 4–5 and Phase 6 can overlap after their shared prerequisites pass. Approved UI design can proceed earlier against fixed contracts, but acceptance waits for the actual services. This is scheduling flexibility, not permission to bypass a gate. Assign named owners/reviewers and estimate work after Phase 0; dates cannot be justified from the current source alone.

## Phase 0 — reproducible baseline and contracts

**Primary code:** root workspace/toolchain/lockfile, both app manifests and `app.rs`, Dockerfiles, `.github/workflows`, `scripts/check_dependency_provenance.py`, dependency copies.

| Task | Concrete deliverable |
|---|---|
| P0-01 | Record current changes and partial-test evidence; preserve the original requirements. Inventory registered routes/server functions/WebSockets, repositories/queries, schedulers, secrets/configuration and runtime dependencies. Distinguish code findings from deployed facts. |
| P0-02 | Reproduce and fix the Cargo Leptos `AnyNestedRoute`/`AnyView` failure. Verify both applications using actual development and release SSR/hydration builds, not only `cargo check`. Preserve routing behavior and feature combinations. |
| P0-03 | Finish workspace/dependency consolidation. Remove obsolete API/storage copies after verifying no consumer, script or container references them. Retain needed Ember UI assets. Verify default production features contain neither cloud storage nor the excluded wallet stub. |
| P0-04 | Fix CI paths, app feature matrix, image jobs and artifact collection; define the mandatory worker checks to add with the Phase 3 worker. Remove failure tolerance. Replace missing root infrastructure references with a validated deployment design for one authoritative writer and the actual Ember volume contract. |
| P0-05 | Export a route/query/schema/content/locale/media baseline. Record entity counts, hashes, slugs, content history, assets and source provenance. Define the authorised process for obtaining deployed schemas and representative PostgreSQL/Scylla exports. |
| P0-06 | Publish a decision/contract register: participant/operator identity provider and contact verification; upstream inventory, activation, safe release, activity, funding and settlement; eligibility/KYC evidence; dedicated volume attachment/permissions/failover; backup destination; agreement/no-referral policy and rounding. Each unknown has an owner, enablement condition and safe fallback. |
| P0-07 | Establish an isolated integration harness with PostgreSQL, temporary media and actual application route registration. Tests fail if required services are absent. Replace missing-server success handling; identify and replace simulated acceptance assertions in their owning phases. |

**Exit gate G0:** clean-checkout actual app builds and container builds pass; dependency provenance is recorded; CI treats mandatory failures as blocking; fixtures are reproducible; invalid deployment paths are caught before deployment. Unresolved external facts are explicitly represented and cannot enable unsafe features. The existing passing compile/tests are revalidated against the resulting revision.

## Phase 1 — canonical schema and migration compatibility

**Primary code:** `uno-app/migrations`, portal repositories/adapters, `uno-api` models/contracts, `uno-admin` database/repositories, shared service placement.

| Task | Concrete deliverable |
|---|---|
| P1-01 | Design one authoritative operational schema for users/sessions/scopes, eligibility, licences/offers, allocation, agreements/referrals, CMS/media, finance, jobs/outbox/inbox, support and scenario records. Define foreign keys, unique constraints, transition invariants, UTC/source currencies and audit fields. Use bounded typed financial fields. |
| P1-02 | Preserve a generated internal UUID plus full unique `(provider, upstream_id)`. Remove truncated/random replacement external identities. Produce a reversible legacy mapping and quarantine collisions/ambiguous rows. Never turn `claimed=true` into verified activation. |
| P1-03 | Align every active SQL binding/row model and licence enum/string contract. Add/reconcile CMS review/version/preview and role tables by choosing one canonical representation. Reconcile duplicate adapters rather than leaving incompatible queries dormant. |
| P1-04 | Implement forward migrations and staged backfills against actual deployed history. Resolve migration 17 compatibility explicitly: fresh-install success does not justify changing an already-applied checksum. Preserve historical IDs, balances, referral approval states, locales and CMS history. |
| P1-05 | Move admin repositories from Scylla to PostgreSQL and share domain services with the portal. Decide whether a small shared domain crate is needed for both apps and the worker; keep `uno-api` contracts/adapters usable without importing a frontend. Remove independent publication authorities and cross-database write assumptions. |
| P1-06 | Build authorised export → staging → validation → canonical import tooling, including dry run, input manifests/hashes, row errors, duplicate detection and reconciliation reports. Keep legacy stores read-only migration sources at cutover, not runtime fallbacks. |
| P1-07 | Separate migration, application and reporting DB roles. Pin a tested PostgreSQL version, tune pool limits on a declared host, and execute every active repository against fresh and upgraded fixtures. |

**Exit gate G1:** fresh and representative upgrade fixtures execute real inventory, identity, CMS, media, finance and job queries. Counts/IDs/amounts/content hashes reconcile or appear in an explicit quarantine report. Runtime apps use PostgreSQL repositories; rollback-compatible schema/code and retained source mappings exist. Production cutover/credential retirement remains Phase 9/10.

## Phase 2 — identity, authorization and governance

**Primary code:** shared principal/authentication, portal extractors/middleware/services, admin handlers/WebSockets, CMS services, identity repositories and consent/audit records.

| Task | Concrete deliverable |
|---|---|
| P2-01 | Integrate a verified contact/login flow with trusted sessions, account linking, logout/revocation, expiry and key rotation. Bind server-side participant identity to onboarding and allocation. Keep provider KYC status separate and minimise stored evidence. No wallet integration is required. |
| P2-02 | Implement roles/scopes for participant, operator, support, country agent, author, reviewer, publisher, finance and integration worker. Enforce resource/operation/transition permissions in shared services as well as transport entry points. Require MFA for privileged roles and recent authentication for sensitive overrides. |
| P2-03 | Enumerate and test every REST/server-function/WebSocket/background mutation path. Use principal-derived audit actors, scoped event subscriptions, cookie CSRF/origin protections and secure session attributes. Prevent generic server functions and legacy aliases from bypassing checks. |
| P2-04 | Remove reachable fallback secrets and privileged browser/build-time tokens. Enforce required startup configuration, define rotation, redact logs/traces/exports and inspect release JS/WASM/binaries. Investigate historical exposure before deciding whether rotation/history cleanup is necessary. |
| P2-05 | Apply bounded request/stream sizes, principal/account quotas, rate limits and a documented trusted-proxy configuration. Preserve legitimate shared-IP access; IP is not identity or eligibility authority. |
| P2-06 | Implement versioned consent, category-specific retention, access/export/deletion/anonymisation workflows and immutable security audit events. Preserve earned liabilities and records required by approved financial retention. Replace sensitive public fixtures with synthetic equivalents. |
| P2-07 | Introduce server-enforced pause controls and launch-gate checks for issuance, markets, funding, referral payments, uploads and campaigns. Start disabled; record actor/reason/evidence/expiry for changes. |

**Exit gate G2:** valid scoped identities succeed for secured operations; anonymous, forged, expired, revoked, wrong-role and cross-scope calls cause no writes or upstream effects. Alternate aliases and direct service calls are covered; unsafe legacy issuance remains disabled until G4 proves its replacement. Missing required config prevents startup; release artifacts contain no privileged credentials; privacy operations preserve financial entitlements. Full owner-bound issuance acceptance is G4, not inferred from authentication tests.

## Phase 3 — durable work and service integrations

**Primary code:** PostgreSQL job/outbox/inbox repositories, active schedulers, sync services, `uno-api` clients/authentication and a separately runnable worker binary.

| Task | Concrete deliverable |
|---|---|
| P3-01 | Persist commands and versioned events with idempotency keys, payload schema, attempts, next-run time, lease owner/expiry and outcome. Transactional mutations create their outbox work in the same commit. Expose a worker that consumes shared domain services. |
| P3-02 | Implement claim/heartbeat/lease-expiry recovery, bounded retries/backoff, dead letters and audited operator replay. Do not keep DB transactions open across provider HTTP calls. Replace active in-process schedulers/pollers with durable work. |
| P3-03 | Complete machine service identities and client compatibility for every used endpoint. Where signatures remain necessary, bind client/method/canonical path/query/body digest/timestamp/nonce; consume nonces atomically in PostgreSQL across processes. Separate replay prevention from operation idempotency. Internal shared-service calls need no artificial HTTP signing. |
| P3-04 | Implement inbox deduplication, explicit pending/unknown outcomes and supported provider idempotency. After ambiguous timeout, reconcile before retrying. If the provider cannot safely deduplicate/query an effect, suspend automated retry and route it to reconciliation. |
| P3-05 | Implement durable provider/composite cursors, backfill and checkpoints that advance with applied records. Preserve equal-timestamp ordering and source event IDs; eliminate fixed limits presented as complete sync. |
| P3-06 | Connect activation/activity/reward/funding evidence adapters only to confirmed contracts. Store source reference, observation time and freshness. Expose a constrained audited manual workflow for unsupported capabilities. |
| P3-07 | Separate liveness, core readiness, media readiness and integration freshness/degradation. Validate schema/config at startup and disable unsafe new actions during relevant provider outages while preserving status/help. |

**Exit gate G3:** two workers, process kills, expired leases, duplicate/reordered events and provider timeouts do not lose accepted commands or duplicate supported external effects. At least 3,000 mixed/equal-timestamp events reconcile across interrupted pagination. Real admin-to-portal requests work; tampering and cross-worker replay fail.

## Phase 4 — inventory, publication, referrals and secure claims

**Primary code:** shared licence/offer/agreement services, `license_admin`, marketplace mapper, CSV importer, portal factory/repositories/REST/server functions and wizard reserve/confirm actions.

| Task | Concrete deliverable |
|---|---|
| P4-01 | Publish a versioned CSV/API import contract requiring real source identity, provider, credential, dates and agreement provenance. Support documented header/headerless forms, validation-only mode, restricted original-upload retention and exact accepted/rejected/unchanged totals. Missing facts become errors/quarantine, never generated lease codes or default one-year validity. |
| P4-02 | Implement one publication/withdrawal authority with stable correlation IDs and per-item acknowledgements. Admin marks only acknowledged items published. Bulk dry run, failed-subset retry and export preserve original row identity. Withdrawal blocks new reservations atomically and surfaces effects on existing allocations. |
| P4-03 | Implement versioned country/task/device rules and eligibility decisions, separating registration, execution, verification and redemption. Availability uses one predicate across summaries and allocation: published, valid, verified, eligible, non-quarantined, unoccupied and within capacity. Unknown rules cannot become approval. |
| P4-04 | Model reservation, issuance, upstream state, funding and productivity separately. Reserve under a consistent lock order and unique open-allocation constraint, binding owner, hashed high-entropy secret, expiry, agreement and frozen referral/no-referral snapshot. Never return a usable credential before issuance. |
| P4-05 | Check ownership before every idempotent/retrieval branch. Recheck dates, offer state, eligibility, reservation expiry and applicable gates at issuance. Persist retry outcomes and expose owner-only recovery; never trust a new confirmation referral or caller actor. |
| P4-06 | Implement approval/rejection/suspension for agents, disclosed attribution window, first-qualified-source selection and single-level enforcement. Corrections require a privileged actor, reason and append-only history, with consistent impact on later payable records. |
| P4-07 | Implement cancel/expiry/release and exposure tracking. Unissued expired reservations can free capacity; issued credentials remain occupied/quarantined until verified safe revocation/rotation/reuse. A productivity drop cannot silently recycle a credential. |
| P4-08 | Wire the factory, every REST/server-function alias and wizard to this service. Remove unsafe legacy paths and optional production claim configuration. Add owner-scoped resume/status without credentials in URLs, analytics, public caches or browser logs. |

**Exit gate G4:** 100 real concurrent applicants competing for 10 licences yield at most 10 distinct owners. Wrong-owner/wrong-secret calls fail before and after issuance; retries return one outcome; reserve-to-confirm expiry/withdrawal is enforced; occupancy never exceeds usable owned inventory. Mixed publication results reconcile exactly; frozen attribution cannot be hijacked; legacy aliases have identical controls or are removed.

## Phase 5 — finance, funding and settlement

**Primary code:** agreement/reward/allocation models, PostgreSQL finance repositories, funding/settlement services, worker adapters and admin finance screens.

| Task | Concrete deliverable |
|---|---|
| P5-01 | Enforce versioned 5,000/4,000/1,000 basis-point agreements end to end. Bound each share and the total; decide and document the no-referral disposition before enabling that offer. Preserve provider reward basis/currency and avoid duplicate gross-up or referral deduction. |
| P5-02 | Implement checked integer minor-unit or fixed-decimal arithmetic with a documented residual-rounding policy. Store unique provider reward events and append-only allocations/reversals, including referral liabilities. Distinguish accrued, payable and paid. |
| P5-03 | Implement UNO-funded credit orders with amount/currency/period/payer, scoped approval, provider idempotency and confirmed/failed/unknown outcomes. Reconcile ambiguous timeouts before retry; expose manual resolution with evidence where provider automation is unsupported. |
| P5-04 | Implement approved settlement topology, authorisation, thresholds/fees, statements and reconciliation. If UNO receives an owner-side 50%, reserve referral liability before treating UNO's 40% as spendable. Never promise an unverified withdrawal channel. |
| P5-05 | Persist actual acquisition, support, hosting, media, messaging, credits and other expenses with source evidence. Report profit, liability, cash and exclusions separately; forecasts do not write actual ledger entries. |
| P5-06 | Connect participant/agent/operator read models to ledger outcomes and discrepancies, with scope checks, period definitions and audited correction workflows. |

**Exit gate G5:** $100 pool allocations reconcile to 50/40/10 under the defined rounding policy; duplicate/reordered/reversed events and repeated funding/payment commands produce no duplicate liability or payment. Opening balances, liabilities, credit orders and settlements reconcile to source fixtures. Negative, overflowing or invalid shares are rejected at all boundaries; finance reviews basis and settlement semantics.

## Phase 6 — local media, CMS and locale preservation

**Primary code:** `file-storage`, both upload/serve handlers, media metadata/reconciler, CMS repositories/services/editor, locale bundles/loaders and deployment volume configuration.

| Task | Concrete deliverable |
|---|---|
| P6-01 | Provision the confirmed dedicated persistent Ember volume with identity and actual mount verification, bounded startup/probe checks and required ownership. Keep media separate from PostgreSQL/WAL capacity. Share long-lived local storage instances; remove runtime cloud adapters/fallbacks from both consumers. |
| P6-02 | Stream authorised uploads into same-volume staging files with per-file/total/account/inode quotas, hashes, decoding and content policy. Keep no-follow descriptor-relative operations and immutable names. Reject malformed streams and path/type/decompression attacks through actual handlers. |
| P6-03 | Implement upload-session/asset/version metadata and `uploading`/`ready`/quarantined/missing/deleted states. Recover filesystem/DB crash windows; only ready authorised assets are served. Use soft deletion/reference-aware garbage collection, including old CMS versions and backups. |
| P6-04 | Make public publication explicit. Implement owner/editor private access and short-lived scoped grants; correct content headers/cache policies, thumbnails and responsive derivatives. Map legacy URLs to opaque asset/version IDs; arbitrary filesystem URLs are never accepted. |
| P6-05 | Inventory and copy authorised legacy assets once, verifying counts/bytes/hashes/permissions. Rewrite active/draft/translated/historic CMS and generated references. Preserve rollback references; runtime cloud requests and bucket URLs must disappear. |
| P6-06 | Finish shared CMS author→review→approve→publish, immutable versions, scoped preview expiry/revocation, locale review, schedules, relations, concurrency conflicts, cache invalidation and revert. Unify FAQ/content read paths, preserve slugs/redirects and system schemas, sanitise structured rich text and retain testimonial consent. |
| P6-07 | Preserve all ten locale bundles/keys/data: `en`, `es`, `tl`, `hi`, `sw`, `pt`, `fr`, `ar`, `id`, `bn`. Fix Bangla lazy loading, SSR/hydration agreement, preference order, localised dates/numbers/currencies/errors and Arabic RTL. Track source-version/fallback/stale translations; require reviewed critical terms. |
| P6-08 | Implement disk/inode thresholds, graceful disk-full behavior and coherent encrypted metadata/media backups to a separate host/disk. Preserve manifest hashes and DB recovery markers; perform a representative restore before full release rehearsal. |

**Exit gate G6:** actual upload/private download/publish/delete/restart/restore works with cloud credentials absent and bucket endpoints blocked. Missing/wrong mounts stop media readiness without creating an empty replacement. Crash recovery never serves an uncommitted/missing object; retained content assets survive GC. CMS lifecycle/permission/preview tests pass and content/locale/hash counts reconcile. Arabic and Bangla run through real render/load paths. Restore demonstrates the proposed RPO/RTO or explicitly blocks approval of those targets.

## Phase 7 — participant, agent and support journeys

**Primary code:** portal navigation/pages/wizard, admin agent/support views, design components, eligibility/account APIs, CMS content, consent and notification policies.

| Task | Concrete deliverable |
|---|---|
| P7-01 | Implement consistent accessible typography/forms/status/errors/tables and mobile layouts at 360/390px, with one primary action and full loading/empty/denied/offline/provider/out-of-stock states. Essential landing/eligibility/help is useful through SSR before hydration. |
| P7-02 | Deliver country landing, eligibility, honest economics/consent, verified account/resume, official setup and owner-bound offer/claim as a resumable full-page journey. Keep safe referral/UTM context, server progress and versioned acceptance. Copy means issued, never activated/earning. |
| P7-03 | Deliver participant home, task details, rewards, funding freshness, help/case reference, renewal/pause/exit and consented unsupported-market waitlist. Show unknown/projected facts explicitly and supported payout guidance only. Preserve read/help access when new admissions pause. |
| P7-04 | Deliver scoped country/agent queues, approved materials/links, application decisions, attributed/accrued/payable/paid commissions and support handoff. Agents cannot change ownership, KYC verdicts or finance; support reassignment does not change beneficiary. |
| P7-05 | Implement D1 installation, D3 activity/data-cost, D7 productivity and D30 follow-up. D7 means rewarded activity on at least four of seven days with required status/rules; D30 uses the original activated-cohort denominator and separate active/productive measures. Notification jobs check consent, timezone, deduplication and current state. |
| P7-06 | Add governed six-market quotas, campaign sources/QR assets, support queues/categories/escalation/time records and voluntary exit. Pause acquisition when eligibility, inventory, funding or support readiness fails. Bangladesh requires reviewed Bangla content/support. |
| P7-07 | Complete all critical journey translations and reviewed facts. Validate keyboard/screen-reader/focus/zoom/contrast, Arabic RTL, network interruptions, reload/back/navigation and hydration failures on real devices. Conduct representative usability sessions; record whether users understand share/credit payer without prompting. |

**Exit gate G7:** representative scoped users complete the real journey and recover from interruptions without premature success states. Agents/support can identify blockers while cross-agent access fails. Ten-locale regression passes; pilot-market local reviewers approve obligations/help; core accessibility and moderated usability evidence meets the requirement targets.

## Phase 8 — operator tools, forecasting and optional adapters

**Primary code:** admin cockpit/finance/operations/scenario UI, shared forecast engine/task registry, reports, outbox/CRM/webhook adapters and suppression records.

| Task | Concrete deliverable |
|---|---|
| P8-01 | Deliver operator inventory/funding/sync/queue/error/cohort/margin views with denominators, time windows and drill-down exceptions. Report observed activity separately from issued/heartbeat counts; reconcile agent statements and support costs. |
| P8-02 | Version and persist scenarios, author/provenance/input dates, algorithm version and task assumptions. Match supplied calculator JSON/CSV contracts with golden fixtures for credit anniversaries, churn, trials, caps, settlement lag, fees/taxes and capital. Label legacy compatibility assumptions. |
| P8-03 | Add task/scenario editor and import/export/comparison. Generate week-1-onward revenue/shares/costs/profit/cash/funding/occupancy/break-even for 10 weeks and longer. Zero supply remains zero rewards; unknown capacity stays unknown; overlapping trials/reservations/active devices share the inventory ceiling. |
| P8-04 | Separate observed/inferred/illustrative rates and actuals/forecasts. A new task changes only eligible compatible exposure and its configured incremental costs; combined synthetic pools cannot double-count actual task rates. Finance signs off basis and rounding. |
| P8-05 | Provide versioned minimal-data outbound events and permitted inbound CRM/support fields through durable outbox/inbox contracts. Verify signatures/scopes, deduplicate out-of-order webhooks, propagate opt-outs and isolate outages. CRM cannot alter eligibility, frozen referral ownership, issuance or rewards. |
| P8-06 | Implement optional-channel policy: approved templates, local-time scheduling, retry limits, global pause, suppression and human handoff. AI uses reviewed facts and escalates unknown financial/country questions. Keep pixels/PII off credential/KYC/private screens. Baseline email/web support works without CRM/advertising services. |

**Exit gate G8:** scenario import/export reproduces results and finance-approved golden outputs; forecasts cannot mutate actual ledgers. Operator metrics reconcile to source events. Disabled CRM leaves onboarding/support functional; duplicate callbacks create one permitted update; opt-out persists across retries. Enabled external adapters have verified capability/access and contract tests, otherwise their unavailable state is explicit.

## Phase 9 — release evidence, cutover and controlled pilot

**Primary code/artifacts:** mandatory CI integration/browser/container jobs, deployment manifest, worker runtime, telemetry/alerts, migration tools, backup/restore/cutover/rollback runbooks and executable launch gates.

| Task | Concrete deliverable |
|---|---|
| P9-01 | Replace the remaining simulated acceptance suite with actual HTTP/server-function/WebSocket/DB/filesystem/browser tests. Demonstrate that disconnected services and seeded vulnerable behaviors fail. Include every alias and feature combination, multi-worker replay, cross-owner issuance, suspended referrals, duplicate payments and media crash windows. |
| P9-02 | Enforce pinned clean-checkout SSR/WASM/worker/release-container/fresh-upgrade-migration checks plus security audit, artifact secret inspection and deploy-manifest validation. Record SHA, toolchain, migration versions, image digests and evidence per gate. Resolve required fmt/clippy/audit failures rather than hiding them. |
| P9-03 | Load-test declared host conditions: 100-client scarce-inventory burst, at least 3,000 sync events, core API p95 target, mobile/Web Vitals targets and queue restart. Correctness remains mandatory even if a latency target fails. |
| P9-04 | Wire dashboards/alerts for allocation conflicts, funding unknowns, stale sync, queue age/dead letters, webhook errors, disk/inodes/missing assets, DB latency and backup failure. Exercise pause controls, escalation and incident runbooks without losing existing status/help. |
| P9-05 | Rehearse migration on a restored representative snapshot; reconcile IDs/counts/currency totals/liabilities/publication/CMS/locales/media. Perform independent-host loss recovery with compatible code/schema/media and measure metadata RPO ≤15 min, media RPO ≤24 h and core RTO ≤4 h against the proposed targets. |
| P9-06 | Prepare production cutover: approved write freeze, drain/reconcile in-flight operations, final export/delta, validation and traffic switch to one writer. Preserve legacy read-only evidence. Define rollback before writes; after v2 accepts writes use compatible replay/recovery or forward fix, never stale v1 state. |
| P9-07 | Activate executable gates with owners, evidence, review/expiry and live checks of country approval, inventory, funding, support and legal/commercial readiness. Missing/expired evidence blocks issuance/campaigns. |
| P9-08 | When engineering and market gates pass, run a bounded pilot of up to 30 consenting participants across two verified markets; Nigeria/Philippines remain conditional choices. Observe actual activation, participant costs/net benefit, support burden, D7 and D30 cohort outcomes before scale approval. |

**Exit gate G9:** all mandatory engineering/preservation/security/recovery tests pass for the exact release; full migration and rollback/restore evidence is reconciled; production configuration and single-writer/media topology are verified. Pilot is separately authorised by commercial/support readiness, and expansion awaits D30 evidence. Passing software checks does not guarantee supply, positive economics or the 250 net/week target.

## Phase 10 — remaining P2 work, retirement and expansion

**Primary code:** localization/editor tooling, experimentation, constrained forecast extensions, optional HighLevel/Plai/channel adapters and retirement/scale runbooks.

| Task | Concrete deliverable |
|---|---|
| P10-01 | Add translation export/import, missing-key reports and pseudo-localisation with validated audited imports and no overwrite of unrelated locales. Existing ten locales stay supported. |
| P10-02 | Add approved server-recorded content experiments. Keep agreement shares/material costs fixed; evaluate qualified activation and retained contribution rather than clicks alone. |
| P10-03 | Extend forecasting with country/device cohorts, actual opening credit ages, delayed releases, shared budgets and incompatible tasks. Occupancy and task-supply constraints remain explicit. |
| P10-04 | Implement optional HighLevel contact/pipeline/template mapping and Plai/native campaign-source adapters against verified access and endpoints. Do not infer lead sync from embedding or invent a provider API. Keep these separately enabled and outage-isolated. |
| P10-05 | After reconciliation/retention approval, remove production Scylla/cloud-storage credentials and runtime remnants; archive manifests/runbooks. Review source deletion separately. Re-test builds and restored recovery after retirement. |
| P10-06 | Review pilot evidence, then gated cohorts toward 100/250 productive participants and the operating target only where task supply, staffing, funding and economics support it. Additional markets/languages/automation require their own evidence; no Yoruba support is implied by a locale mapping. |

**Exit gate G10:** all applicable P2 features have their own verified acceptance evidence; enabled optional providers pass real contract tests; source/credential retirement is approved and recoverable. Full-scope completion requires these tasks as well as G0–G9. A disabled unconfigured optional provider is recorded as conditional scope, not falsely certified as an implemented live integration.

## Requirement-to-phase crosswalk

Ranges are inclusive. A requirement spanning phases closes only when the last required acceptance evidence is complete. The source document's per-requirement acceptance criteria remain binding, including details not restated in this plan.

| Requirement IDs | Implementation phases |
|---|---|
| BUILD-01–BUILD-02 | 0, 3, 9 |
| DB-01–DB-03 | 1, 9 |
| DATA-01 | 4 |
| SEC-01–SEC-05 | 2, 4, 6, 9 |
| SEC-06 | 3, 9 |
| SEC-07 | 2, 9, 10 |
| LIC-01–LIC-10 | 1, 3, 4, 9 |
| REF-01–REF-04 | 4, 5, 7 |
| FIN-01–FIN-06 | 1, 5, 8, 9 |
| FS-01–FS-08 | 6, 9 |
| CMS-01–CMS-08 | 0, 1, 2, 6, 7 |
| I18N-01–I18N-06 | 6, 7, 9 |
| I18N-07 | 10 |
| UX-01–UX-21 | 2, 4, 5, 6, 7, 8, 9 |
| MKT-01–MKT-03 | 4, 7, 8, 9 |
| MKT-04 | 10 |
| SUP-01–SUP-04 | 3, 7, 8, 9 |
| FORE-01–FORE-05 | 8 |
| FORE-06 | 10 |
| INT-01–INT-02 | 3, 8 |
| INT-03 | 10 |
| INT-04 | 7, 8, 10 when an optional channel is enabled |
| OPS-01–OPS-04 | 2, 3, 6, 7, 9 |
| QA-01 | Every implementation phase; aggregate closure in 9 and feature extensions in 10 |
| MIG-01 | 0 |
| MIG-02 | 1, 9 |
| MIG-03 | 6, 9 |
| MIG-04–MIG-06 | 9 |
| MIG-07 | 10 |
| REV-01 | 0, 9 |
| REV-02 | 2, 6, 9 |
| REV-03 | 3, 9 |
| REV-04 | 4, 7, 9 |
| REV-05 | 1, 9 |
| REV-06 | Wallet integration excluded; production exclusion verified in 0 and 9 |
| REV-07 | 1, 3, 4, 9 |
| REV-08 | 4, 5, 7 |
| REV-09 | 5, 9 |
| REV-10 | 3, 9 |
| REV-11 | 6, 9, 10 |
| REV-12 | 0, 3, 9 |
| REV-13 | Every implementation phase; release closure in 9 |
| REV-14 | 7, 9, 10 |
| REV-15 | 1, 2, 6, 7, 9, 10 |
| REV-16 | 8, 10 |
| REV-17 | 4, 7, 8, 9, 10 |
| REV-18 | 2, 3, 7, 9, 10 |

F01–F19 retain the original requirement document's audit-to-development mapping; closing the corresponding implementation tasks does not close an audit finding until its real-boundary closure test passes. The unnumbered architectural, performance, recovery, commercial and definition-of-done requirements are covered by the phase tasks and gates above.

## Evidence and execution tracking

For each Pn task, record accountable person, reviewer, relevant requirement IDs, affected paths/routes/migrations, status, external dependency, test command/fixture, reconciliation output, PR/revision and operational notes. Store sanitised evidence under a consistent release revision; never put real credentials or personal source exports into public test artifacts.

At each phase review, update the implementation analysis with verified outcomes and remaining blockers. Keep the requirements as the acceptance authority. A green compile or a passing count of unit tests cannot change an open end-to-end requirement to complete.

Immediate execution order is P0-01 through P0-07, starting with the actual Leptos frontend build failure and route/dependency inventory. Schema and issuance changes follow the verified baseline. No production rollout is part of this planning deliverable.
