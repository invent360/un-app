# UNO-APP v2: Comprehensive Relaunch Requirements

**Document version:** 2.0 — supersedes v1.0 of the same date
**Date:** 29 September 2026
**Status:** Actionable development specification
**Repository:** https://github.com/invent360/un-app (audited at `111ef5df214d85ed95fe71ff0113e2f39418a135`)
**Target:** a trustworthy automatic licence distribution system for 2,500 licences at ~250 net productive additions per week

### What changed from v1.0

v1.0 was a competent first pass but carried four defects that this revision corrects:

1. **Task-count arithmetic in v1.0's Appendix B did not match its own tables** (SEC, FS and ECO priority splits were miscounted). Counts below are recomputed and verifiable.
2. **v1.0 asked no architectural question and answered none.** It asserted "PostgreSQL-only" without presenting the question, without the trade-offs, and without naming the *actual* architectural problem. See §2.4 and §3.
3. **v1.0 omitted 10 of the audit's 19 findings** and all of the audit's non-F-numbered sections (§7 data governance, §8 campaign data model, §9 remediation sequence, §10 corrections, §13 evidence limits).
4. **v1.0 got i18n wrong.** It proposed adding "English, Filipino, Swahili" — but Filipino (`tl`) and Swahili (`sw`) already ship. See §2.5 and WS8.

### Source documents and what each contributes

| Source | Contribution to this spec |
|---|---|
| `docs/marketing/UN_APP_COMPREHENSIVE_TECHNICAL_AUDIT_REV2.md` | F01–F19 findings, 7-phase remediation sequence, 13-item acceptance suite, §7–§10 non-numbered gaps, §13 evidence limits, §14 adjudications |
| `docs/marketing/UNETWORK_2500_LICENSE_MARKETING_PLAN.md` | 50/40/10 economics, 10-step user journey, three-inventory model, D7/D30 definitions, 8 launch gates, referral rules, channel taxonomy, dashboard/KPI spec |
| `docs/marketing/UNETWORK_REVENUE_CALCULATOR.html` | The forecast engine to be productised: 27 config inputs, 10-field pluggable task schema, cohort/churn/credit algorithm, scenario save-load, CSV export |
| Direct repository inspection (this session) | 10 additional findings (N01–N10) not present in the audit |

### Evidence labels used in this document

Following the audit's convention, so that no reader mistakes inference for observation:

- **Verified** — directly inspected source or independently recomputed
- **Conditional** — impact depends on deployment, feature selection or build environment
- **Reported** — third-party execution evidence not independently reproduced
- **Specified** — required by a source document (a requirement, not a code fact)
- **Proposed** — added by this document beyond the source documents

---

## 1. Executive summary

### 1.1 The one-sentence problem

The repository contains a plausible licence-distribution application whose schema, build pipeline and access control do not currently support its own code, and whose licence model cannot express the three-party 50/40/10 agreement the business depends on.

### 1.2 Three facts that determine the sequencing

**Fact 1 — the schema does not match the code, and almost none of it is drift.**
`migrations/00002_licenses.up.sql:8-33` creates a `licenses` table with `id VARCHAR(66)`, `node_id`, `owner_wallet_address`, `device_name`, `activation_start_at/end_at`, `is_active`, `is_leased`, `lease_from/to`, `lease_share_percentage`, `lease_min_uptime_percentage`, `uptime`. The active repository (`src/server/repositories/license_repository.rs:34-35`) inserts into `id, lease_code, valid_from, valid_to, split_type, claimed, bound_to_device, device_id, claimed_at, created_at`. **One column of ten overlaps, and its type is incompatible** (`VARCHAR(66)` vs `$2::uuid`). Every column the migration does define — wallet, uptime, `lease_share_percentage` — is referenced by zero lines of application code.

This is two different schemas, one of which is dead. It is not a migration that drifted; it is a model that was never reconciled. *(Verified)*

**Fact 2 — the build cannot run from a clean checkout.**
`.gitignore:8` ignores `Cargo.lock`. `Dockerfile:26` and `Dockerfile:46` both execute `COPY Cargo.toml Cargo.lock ./`. Any `actions/checkout` therefore produces a tree without the file the Dockerfile requires, and the image build fails before compiling a line of Rust. This is currently masked by a developer's local `Cargo.lock`. *(Verified)*

**Fact 3 — the security scaffolding is present and unwired.**
`src/main.rs:119-121` wraps exactly two layers: `VisitorTracker` and the request logger. The `Auth`, `CSRF`, `RateLimit`, `CORS`, `SecurityHeaders`, `RequestSizeLimit`, `Compression`, `RBAC` and `Audit` middlewares exported from `src/server/middleware/mod.rs` are dead code. Separately, `src/routes/debug.rs:49` and `:88` register `GetAllLicensesDebug` and `GetAllReferralsDebug` as unauthenticated `#[server(..., "/api")]` functions on an unconditionally-mounted route, each returning up to 1,000 rows. *(Verified)*

The audit characterised the middleware problem as "Medium to High depending on exposure" (F13). The signature-level evidence is stronger than that: `upload_files(mut payload: Multipart) -> HttpResponse` (`file_handler.rs:119`) and `confirm_license_claim(license_id, device_id, referral_code)` (`licenses.rs:198`) take **no authentication parameter of any kind**, which is not a matter of misconfiguration.

### 1.3 What the audit's proportionality judgement means for this plan

The audit is explicit: *"These are correctness and access-control problems, not evidence that 2,500 records would overwhelm the databases. Expanding infrastructure before fixing them would increase cost without resolving distribution integrity."*

This document is therefore weighted roughly 80% correctness and access control, 20% product and experience. The product work in WS9–WS10 is real and necessary, but it is worthless if it renders a claim flow that can issue the same lease code to two people.

### 1.4 Decision register

| # | Decision | Ruling | Basis |
|---|---|---|---|
| D1 | Databases | **Consolidate to PostgreSQL. Retire ScyllaDB.** | §2.4 |
| D2 | File storage | **Local Ember volume. No GCP/S3 in production paths.** | §2.3 |
| D3 | Rewriting from scratch | **No. Correct shared contracts, transaction boundaries and access control in place.** | Audit §9: *"Avoid a full rewrite by default"* |
| D4 | i18n and CMS | **Preserve, and extend by two languages only.** | §2.5 |
| D5 | Licence model | **Versioned three-party agreement in integer basis points; the enum is retired.** | Audit F05, §8.4 |
| D6 | Authoritative record | **Upstream Unetwork services.** Neither local database is authoritative for entitlement. | §3 |
| D7 | Productive definition | **Adopt the marketing plan's D7 definition verbatim as an acceptance predicate.** | §7.3 |

---

## 2. Answers to the five questions raised

### 2.1 UI redefinition and the discovered user journey

Covered in WS9. In summary: the current portal's component tree (`src/components/`: `chatbot`, `common`, `faq`, `layout`, `license`, `sections`, `stats`, `tasks`, `wizard`) is organised around *licence acquisition*. The marketing plan describes a different product — a *participant operations* product with a 10-step lifecycle, milestone touchpoints at D1/D3/D7/D30, three separate inventory pools, and an explicit honesty requirement that is incompatible with a conventional acquisition funnel.

The redefinition is not a reskin. It is a change of primary object from "licence" to "participant lease lifecycle". Full register in §9 of this document (WS9.1–WS9.9) and the state machines in §7.

### 2.2 Every earmarked audit gap as an actionable task

All 19 findings plus all 10 non-F-numbered sections plus 10 new findings are mapped to task IDs in §10 (Traceability matrix). No finding is dropped, merged away or silently reclassified.

Two findings change priority relative to the audit, on the strength of direct inspection:

- **F16 escalates.** The audit recorded *"Verified configuration pattern; artifact exposure unestablished"* and rated it "P0 investigation". Given F01 (no authentication at all) and the fact that `API_TOKEN`/`UNITY_JWT_TOKEN` are read via `option_env!` into a WASM-target path, this is a P0 remediation, not a P0 investigation. See SEC-11.
- **F10 escalates from conditional to P0.** The audit correctly rated the local-backend traversal "P1; **P0 if enabling local storage for untrusted uploads**", noting both consumers ship GCS-only. **We are deliberately switching to local storage** (D2). The condition is therefore activated by this programme, and F10 becomes P0. See FS-01.

### 2.3 File management moves to the local host

Agreed, and it is the right call on cost grounds: the assets are learner-facing guides, FAQs, task explainers and short videos — a low-cardinality, moderate-size, high-read/write-asymmetry corpus. Object storage is the wrong cost shape for it.

Four consequences the plan must own rather than discover late:

1. **It activates F10.** See §2.2. The containment work is not optional cleanup; it is a precondition of the migration.
2. **Availability becomes a single-host dependency.** Uploads and previews fail when the host is down. The plan accepts this in exchange for cost, and requires restore testing (OBS-05) rather than pretending a volume is durable.
3. **Serving is a real design decision.** Local storage means either serving bytes through the app (proxying, bandwidth cost, cache headers under our control) or fronting the volume with a static server/CDN. WS6 requires this decision be made explicitly, not inherited.
4. **`file-storage` already has a local backend** — it is feature-gated and unsafe, not absent. We harden it (FS-01) rather than write a replacement. This is materially cheaper than a greenfield adapter.

**Deliverable also required:** an asset inventory and staged migration with rollback, because the current GCS objects are the only copy of published content (FS-08).

### 2.4 Question: merge all databases into PostgreSQL, or keep PostgreSQL and ScyllaDB separate?

**Recommendation: merge into PostgreSQL. Retire ScyllaDB.**

The reasoning, in the order that actually decides it:

**(a) The scale argument is decisive and one-sided.**
The programme is capped at 2,500 concurrent leases. The largest plausible table is per-device-day reward telemetry: 2,500 devices × 365 days ≈ 9.1 × 10⁵ rows/year, plus the plan's trial population (3,211 lifetime onboarding attempts across ten weeks). Even a deliberately pessimistic 10⁷ rows/year is unremarkable for a single PostgreSQL instance with monthly partitions and a BRIN index on the time column. ScyllaDB's entire value proposition — horizontal write scaling across commodity nodes at high throughput — buys nothing at this volume, while its operational cost is real: a separate cluster to provision, monitor, back up, patch and secure.

**(b) The workload is the opposite of Scylla's shape.**
Everything that must be correct in this system is a multi-row invariant: an atomic reservation, a first-write-only referral attribution, a split whose components sum to exactly 10,000 basis points, a per-licence ledger where allocations reconcile to the pool. These need cross-row ACID transactions, foreign keys, unique constraints and serialisable-enough isolation. Scylla's lightweight transactions are scoped to a **single partition**; there is no cross-partition atomicity. Using a distributed store to enforce a money invariant is the wrong tool, and the failure mode is silent partial state rather than a transaction error.

**(c) The Scylla schema cannot satisfy its own code today.**
The audit records that the Scylla marketplace repository reads and writes `is_published`, a column absent from the committed CQL migrations. We would be consolidating *away from* a store that is already broken.

**(d) It directly serves the cost goal in D2.** A cost-reduction programme that retains a distributed database cluster has an internal contradiction. One instance on the Ember host is coherent; two engines are not.

**What consolidation does *not* fix — and this is the important caveat.** The architectural problem in this system is not the choice of engine. It is that *there are at least three distinct records of a licence* (upstream Unetwork state, the admin database, the portal database), as the audit states in §3. Merging two of them into one leaves that problem intact. The actual remedy is the single-authoritative-writer plus reconciliation model in §3 below, and it would still be required if ScyllaDB stayed.

**Recommended target:** one PostgreSQL instance, logical separation by schema (`portal`, `admin`, `audit`, `ledger`, `telemetry`), enforced by role and by naming convention rather than by separate databases. Read replicas and `telemetry` table partitioning are introduced only if measured latency or volume justifies them — not pre-emptively, per the audit's §7.4 proportionality judgement.

**Explicitly not recommended:** keeping Scylla for "telemetry" on the theory that time-series data is a distributed-store problem. At 10⁷ rows/year, a partitioned PostgreSQL table is simpler, cheaper and already covered by the same backup and restore story as the money tables. Revisit only on measured pressure.

**Migration discipline:** ScyllaDB becomes a *read-only rollback source* for the duration of the cutover. No new Scylla work is funded. Tasks DB-01 to DB-09.

### 2.5 Preserving internationalisation and CMS

Both capabilities exist and are worth preserving. The characterisation in v1.0 of what needs adding was wrong, so here is the verified inventory.

**i18n is two-layer, and both layers exist.**

*Code layer* — `src/locales/`, compile-time `phf::Map` static maps with O(1) lookup, plus an optional dynamic JSON loader:

| File | Language | Status |
|---|---|---|
| `en.rs` | English | Reference locale |
| `ar.rs` | Arabic | Complete, **RTL** |
| `es.rs` | Spanish | Complete |
| `fr.rs` | French | Complete |
| `hi.rs` | Hindi | Complete |
| `id.rs` | Indonesian | Complete |
| `pt.rs` | Portuguese | Complete |
| `sw.rs` | Swahili | Complete |
| `tl.rs` | Tagalog | **Incomplete — see below** |

Supporting modules: `intl.rs` (locale-aware number, currency, date and plural formatting) and `lazy_loader.rs` (bundle-size reduction strategy). `get_text_direction()` at `src/types/schema.rs:931` already returns `"rtl"` for `ar | he | fa | ur`, so RTL support is **already present for Arabic** — a foundation, not a task.

*Content layer* — CMS-driven, so content authors can translate without a code change:
- `content_items.translations JSONB` — `{"es": {...}, "fr": {...}}` (`00008_cms.up.sql:47+`)
- `content_items.translation_status JSONB` — per-locale `complete` / `partial` (`00008_cms.up.sql:48+`)
- Helpers `localize()`, `has_translation()`, `get_translation_status()` at `src/types/schema.rs:632-664`, all defaulting to `en` when a locale is absent
- `locale` field on FAQ (`src/types/faq.rs:26`) and content review (`src/types/content_review.rs:194`) records

**The correction.** The marketing plan's six launch markets map onto the shipped languages as follows:

| Market | Plan language requirement | Shipped | Action |
|---|---|---|---|
| Nigeria | English + local-language explainers | `en` | Add `yo`, `ha` (P2) |
| Philippines | English, Filipino after testing | `en`, `tl` | **Complete `tl` first** |
| India | English + one locally selected language | `en`, `hi` | Add `ta`, `te`, `bn-IN` (P2) |
| Kenya | English, Swahili as needed | `en`, `sw` | Covered |
| Bangladesh | **Bangla materials before expansion** | — | **Add `bn` (P0)** |
| Ghana | English initially | `en` | Add `ak`/`tw` when the market opens (P2) |

So: **two genuinely new locales for launch (Bangla, and completion of Tagalog), not four new languages.** v1.0's "add Filipino and Swahili" would have duplicated shipped work.

**A concrete defect found in `tl.rs`.** The Tagalog map is only partially translated — `nav.home` is the English `"Home"`, `nav.stats` is `"Stats"`, `nav.faq` is `"FAQ"`, `nav.contact` is the untranslated `"Contact"`, and `nav.common_errors` is the mixed `"Mga Error"`. A participant reading a Filipino locale sees untranslated navigation. This is a completion task (I18N-03), not a build task.

**i18n is not free to preserve — it is coupled to the broken CMS schema.** This is the non-obvious dependency. Content-layer translations live in `content_items`, which is committed in `00008` and therefore sound; but the review, publish and preview workflow *around* translated content depends on `content_reviews`, `preview_tokens` and `content_versions` — **none of which are created by any committed UP migration** (audit F14). Worse, `00008_cms.down.sql:4-13` drops `content_versions`, a table that `00012` never creates (it creates `content_item_versions`), so **rollback fails and any re-migration half-applies**.

Consequence: translated content can be stored but not safely reviewed, published or previewed. WS8 therefore sequences schema repair *before* localisation work, and I18N-01 is a preservation test rather than an assumption.

**CMS capability to preserve:** `content_schemas` (user-definable content types with JSONB field definitions, `is_system` protection), `content_items` (draft/publish lifecycle, versioning, featured/active flags), `content_relations`, `page_contents` (slug + content type), `content_item_versions` (`00012`), `roles` + `role_permissions` (`00007`), `audit_logs` (`00006`). The audit credits the CMS as a genuine asset: *"Existing inventory, upstream sync, referral, educational content and operational UI code can support the campaign once the connections are corrected."*

---

## 3. Target architecture

### 3.1 The three-record problem

```
┌─────────────────────────────────────────────────────────────┐
│  Upstream Unetwork services                    AUTHORITATIVE │
│  licence state · activation · credited rewards               │
└──────────────────────────┬──────────────────────────────────┘
                           │ HMAC-signed, replay-protected
                           │ + explicit reconciliation
              ┌────────────┴────────────┐
              ▼                         ▼
┌────────────────────────┐   ┌──────────────────────────────┐
│ uno-admin (PostgreSQL) │   │ uno-app portal (PostgreSQL)  │
│ inventory · agents ·   │   │ claims · referrals · CMS ·   │
│ publication · sync     │   │ RBAC · participants          │
│      ADMIN  schema     │   │       PORTAL schema          │
└────────────────────────┘   └──────────────────────────────┘
              └────────────┬────────────┘
                           ▼
              ┌──────────────────────────────┐
              │ PostgreSQL  ledger / audit /  │
              │ telemetry schemas             │
              │ the system of record for what │
              │ WE decided and WHY            │
              └──────────────────────────────┘
```

**Rules that follow:**

- **R1** — Entitlement is answered by upstream Unetwork services, never by a local flag. Audit §10: *"A local UUID derived from an upstream identifier proves that this application transforms the identifier. It does not prove the upstream licence is fictitious."*
- **R2** — Every licence write goes to a **single authoritative writer**. The audit (§7.4) requires: *"All reservation and claim writes must reach the authoritative writer."* Regional or read-replica settings must not be assumed to provide working read/write routing until traced end to end.
- **R3** — A successful local commit followed by a failed upstream call is a **visible retryable state**, never a silent success. (Audit §7.3)
- **R4** — Every licence record retains both an internal UUID **and** the full upstream identifier. (Audit F07)
- **R5** — Nothing is "published" locally until the remote portal has acknowledged that specific item. (Audit F06)

### 3.2 Deployment target

| Concern | Today | Target |
|---|---|---|
| Databases | PostgreSQL + ScyllaDB | One PostgreSQL instance, schema-separated |
| File storage | GCS + signed URLs, unsafe local backend | Local volume on the Ember host; no cloud SDK in production paths |
| Admin exposure | Loopback default, no request auth | Authenticated ingress + application-level authorization on every privileged route |
| Build | No tracked lockfile; nested workflows; no root CI | Reproducible workspace, committed lockfiles, root CI, container build as a mandatory gate |
| Health | HTTP 200 + `status: ok` regardless of DB | Separate liveness and readiness, bounded dependency checks |
| Regions | Multi-region Terraform options | Single region; replicas only on measured evidence |

---

## 4. Workstream task registers

**Priority definitions**

| Priority | Meaning | Gate |
|---|---|---|
| **P0** | Release blocker | Must complete before any public exposure |
| **P1** | Required for production launch | Must complete before the 2,500-licence campaign |
| **P2** | Enhancement | May follow initial launch |

Every task carries a source tag. Tags: `Fnn` = audit finding; `A§x` = audit non-numbered section; `M§x` = marketing plan section; `CALC` = revenue calculator; `Nnn` = new finding from this session; `PROP` = proposed by this document.

---

### WS0 — Build, CI and supply chain (prerequisite; nothing else is verifiable until this lands)

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| BLD-01 | Stop ignoring `Cargo.lock`, or remove the unconditional `COPY` | P0 | A clean `git clone` + `docker build` succeeds with no developer-local files | N01, F11 |
| BLD-02 | Commit `Cargo.lock` for every workspace member | P0 | Reproducible builds from locked dependency versions | F11 |
| BLD-03 | Move workflows to repository-root `.github/workflows` | P0 | GitHub Actions trigger automatically on push and PR | F11 |
| BLD-04 | Correct working directories, build contexts and component paths in the moved workflows | P0 | Moving the files alone is explicitly insufficient — paths must be fixed too | F11 |
| BLD-05 | Create a unified workspace `Cargo.toml` | P0 | All components build from a single workspace invocation | F11 |
| BLD-06 | Build both SSR and browser (WASM) targets in CI on every PR | P0 | Both targets verified, not just one | F11, A§9 |
| BLD-07 | Make container construction a mandatory CI gate | P0 | A Dockerfile regression fails the pipeline; `continue-on-error` removed from migrations | N01, A§12 |
| BLD-08 | Fix the `ci-success` gate: it declares 7 needs but checks 5 | P0 | `property-tests` and `security-audit` outcomes are evaluated; a red `cargo audit` fails the repo | N06 |
| BLD-09 | Point the three deploy jobs at the real Helm chart | P0 | They currently `cd` into `infrastructure/kubernetes/overlays/*`, which does not exist anywhere in the repository | N08 |
| BLD-10 | Fix the `infrastructure.yml` path filter to `infra/terraform/**` | P0 | A push to Terraform now triggers the workflow | N08 |
| BLD-11 | Remove developer-local path dependencies from the admin manifest | P0 | A fresh checkout reproduces the build; `optional = true` does not excuse an absent manifest | F11 |
| BLD-12 | Pin the Rust toolchain and add a minimal supported-version check | P1 | `rust-toolchain.toml` is honoured and validated | F11 |
| BLD-13 | Remove or unify the duplicated vendored `uno-api` / `file-storage` copies under `deps/` | P1 | One source of truth, with an automated equality check if duplication is retained | F11, A§12 |
| BLD-14 | Add a real coverage threshold | P1 | Coverage measures production paths; a report existing is not sufficient | N09, A§12 |

---

### WS1 — Schema and data model

The single most important workstream. Almost every other P0 fails at its first query until this lands.

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| DB-01 | Decide the authoritative `licenses` schema and delete or archive the dead one | P0 | One schema, explicitly chosen; the `VARCHAR(66)`/`node_id`/wallet/uptime variant is resolved, not merged-by-accident | N05, F14 |
| DB-02 | Add the nine missing licence columns | P0 | `lease_code`, `valid_from`, `valid_to`, `split_type`, `claimed`, `bound_to_device`, `device_id`, `claimed_at`, `created_at` exist and are typed for the active model | F14, N05 |
| DB-03 | Create the `split_type` enum, or retire it in favour of basis-point columns | P0 | The only `CREATE TYPE` in all migrations today is `content_status`; the licence enum does not exist | N04, F14 |
| DB-04 | Create the missing RBAC/CMS tables | P0 | `user_roles`, `content_reviews`, `preview_tokens` are created by committed UP migrations | F14 |
| DB-05 | Repair the `content_versions` / `content_item_versions` mismatch | P0 | `00008_cms.down.sql` must not drop a table no UP migration creates; rollback succeeds and re-migration is clean | N07 |
| DB-06 | Close the `00005` migration sequence gap | P0 | Numbering is contiguous, or the gap is documented in-file | N07 |
| DB-07 | Add `is_published` to the Scylla CQL schema, then migrate the data to PostgreSQL | P0 | Migration is complete with zero data loss before Scylla is retired | F14, D1 |
| DB-08 | Migrate the remaining admin data (inventory, agents, rewards, sync jobs, marketplace state) to PostgreSQL | P0 | Full backup taken first; parallel validation period with per-row comparison | F14, A§9 |
| DB-09 | Create the `ledger`, `audit` and `telemetry` schemas | P0 | Allocations, activity log and device-day telemetry have their own homes | §2.4, A§7.2 |
| DB-10 | Add a `telemetry` table partitioned by month | P1 | Handles ~10⁶ rows/year; BRIN index on the time column | §2.4 |
| DB-11 | Build the query-to-schema inventory for both databases | P0 | Every active repository query is mapped to a provisioned table and column; this is the check that would have caught DB-01 on day one | F14, A§9 |
| DB-12 | Add offline SQLx metadata generation verified against the migration-derived schema | P1 | Checked queries are covered; changing macros alone is not treated as proof of correctness | F14 |
| DB-13 | Add migration verification to CI | P0 | Migration failures block deployment and block readiness | F14, F11 |
| DB-14 | Ensure a single authoritative writer for all reservation/claim writes | P0 | Read-replica and regional settings traced and tested end to end before any are trusted | A§7.4 |
| DB-15 | Fix availability computation | P1 | Availability derives from the authoritative eligibility predicate; the `total − claimed − expired` double-subtraction of expired-and-claimed records is removed; future-starting records handled; result is never negative and equals the sum of per-agreement counts | F17 |
| DB-16 | Reject invalid split values at write time | P1 | No fallback to 50:50 in row conversion; unknown splits fail loudly | F17, F05 |
| DB-17 | Repair the CSV import contract and its fixtures | P1 | Non-empty lease code is required; two success fixtures that omit it are corrected rather than satisfied by manufacturing codes; row-level errors reported; header and headerless forms both exercised against the real parser | F18 |

---

### WS2 — Authentication, authorization and secrets

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| SEC-01 | Implement real authentication on all Leptos server functions | P0 | Unauthenticated requests fail **before** any database write or upstream call | F01, N03 |
| SEC-02 | Remove `/debug` from the route table and gate the three debug server functions | P0 | `get_all_licenses_debug`, `get_all_referrals_debug`, `get_all_visitor_stats_debug` are unreachable without authorization | N02 |
| SEC-03 | **Install the existing middleware** — Auth, CSRF, RateLimit, CORS, SecurityHeaders, RequestSizeLimit, Compression, RBAC, Audit | P0 | `main.rs` actually wraps them; module existence is not enforcement | F13, N03 |
| SEC-04 | Add operation-specific authorization checks | P0 | RBAC enforced on publication, settings and job operations | F01 |
| SEC-05 | Bind `confirm_license_claim` to an unforgeable session or token | P0 | The signature gains an authentication parameter; a caller cannot confirm by guessing an ID | F03, N03 |
| SEC-06 | Add CSRF protection for cookie-authenticated writes | P0 | State-changing requests without a valid token are rejected | F01 |
| SEC-07 | Scope job WebSocket subscriptions to authorized viewers | P0 | Job events never reach an unauthorized subscriber | F01 |
| SEC-08 | Secure CMS server functions | P0 | `publish_direct`, `publish_approved` and preview-token creation verify an authenticated principal; HMAC on separate Actix handlers does not secure them | F15 |
| SEC-09 | Derive audit actors from the authenticated principal | P0 | Changing a supplied `published_by` / `created_by` string cannot impersonate another operator | F15, F01 |
| SEC-10 | Record the human actor separately from the signing service client | P0 | Two distinct identities are stored, not one conflated field | F01 |
| SEC-11 | Remove privileged JWT configuration from browser build paths | P0 | No `API_TOKEN` / `UNITY_JWT_TOKEN` in WASM artifacts; **escalated from the audit's P0-investigation** because no authentication layer exists to mitigate exposure | F16, §2.2 |
| SEC-12 | Inspect release artifacts and build environments for shipped credentials | P0 | Determination of actual exposure is made and recorded, not assumed either way | F16 |
| SEC-13 | Rotate any credential confirmed exposed | P0 | Conditional on SEC-12 finding exposure | F16 |
| SEC-14 | Remove the FAQ preview signing-secret fallback and fail closed | P0 | Absent configuration disables protected preview issuance safely rather than using a known default | F16 |
| SEC-15 | Distinguish the unwired `dev-admin-key` latent defect from an active bypass, and fix it | P0 | The latent middleware fallback is removed; the finding is recorded accurately rather than over- or under-stated | F16 |
| SEC-16 | Treat the Supabase publishable key correctly | P1 | Not described as equivalent to a privileged bearer token; does not block SEC-11 remediation | F16 |
| SEC-17 | Add per-request, per-file and account quotas | P1 | Abuse limits enforced on public claim and application paths, independent of UI behaviour | F02, F13 |
| SEC-18 | Verify proxy trust before using forwarded IPs | P1 | Forwarded headers are validated before any abuse limit or geography logic depends on them | F13 |
| SEC-19 | Publish a precise external-responsibility contract | P1 | `uno-api` is documented as not separately deployed; route security belongs to consuming applications | A§12 |

---

### WS3 — Storage on the Ember volume

Local storage makes F10 reachable, which moves it to P0. See §2.2 and §2.3.

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| FS-01 | Enforce canonical path containment in the local backend | P0 | Traversal, absolute-path and boundary-prefix cases cannot read or delete outside the permitted root. **The existing `file://` check is a string prefix test, not a containment test** | F10, D2 |
| FS-02 | Disallow path components entirely; use opaque resource IDs | P0 | All file operations key on UUIDs; caller input is never joined to a base path | F10 |
| FS-03 | Enforce a symlink policy | P0 | Symlink escape is rejected, including via uploaded archives | F10 |
| FS-04 | Keep uploaded files outside executable and static application paths | P0 | No upload can land in a served or executed directory | F10 |
| FS-05 | Enforce size limits while streaming, not after buffering | P0 | An oversize stream is rejected before its entire body is buffered; the JSON payload limit is not relied on to bound multipart buffering | F02, F19, A§4.4 |
| FS-06 | Validate MIME from content bytes, not the caller-declared type | P1 | Extensions and content cannot disagree | F19 |
| FS-07 | Decide and implement the SVG policy | P1 | SVG is either rejected or sanitised and served under restrictive headers — an explicit decision, not a default | F19 |
| FS-08 | Produce an asset inventory and migrate existing GCS objects to the volume | P0 | Every existing asset is reachable at its new URL; staged migration with rollback | D2, A§15 |
| FS-09 | Decide and implement the serving path (app proxy vs static server vs CDN) | P0 | An explicit decision with cache headers, content types and range-request behaviour specified | D2 |
| FS-10 | Provision and mount the dedicated Ember volume | P0 | Data survives container restart; mount options prevent the volume hiding other container content | D2 |
| FS-11 | Share one storage client through the service factory | P1 | Client construction no longer scales with the number of image strings; configuration and HTTP-client construction are not repeated per URL | F19 |
| FS-12 | Cache parsed signing material where safe and measure signing latency | P1 | No excessive synchronous crypto work on request workers | F19 |
| FS-13 | Make signing and storage failures observable | P1 | A failure produces an actionable error, never a misleading fallback URL | F10, F19 |
| FS-14 | Remove cloud storage SDK calls from production code paths | P2 | No GCP/S3 API calls on the production request path | D2, user request |
| FS-15 | Advertise implemented backends precisely | P2 | S3 and Azure names appear in configuration detection but are **not implemented**; the project stops implying support it does not have | A§4.4 |

---

### WS4 — Claim, reservation and lease lifecycle

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| CLM-01 | Make reservation atomic and persistent | P0 | Selection and reservation persist in one short transaction, commit, then the user proceeds. The transaction is never held open across human onboarding | F03 |
| CLM-02 | Remove the ineffective `FOR UPDATE SKIP LOCKED` usage or bring it inside the reservation transaction | P0 | The lock actually protects the claim path rather than only the read | F03 |
| CLM-03 | Bind reservations to a claimant/session with an unguessable token | P0 | Ownership is enforced server-side | F03 |
| CLM-04 | Implement reservation expiry with automatic release | P0 | Expired reservations release without manual intervention | F03 |
| CLM-05 | Reveal the lease code only at the intended issuance point | P0 | A masked key in the browser is recognised as presentation only; the underlying response no longer carries the credential early | F03 |
| CLM-06 | Make retries idempotent **for the same authenticated owner** | P0 | A legitimate retry returns the original result; a different caller who knows the ID gets nothing | F03 |
| CLM-07 | Add ownership verification to the confirm-by-ID path | P0 | Cross-owner confirmation fails; a second confirmation of an already-claimed licence is not a success | F03 |
| CLM-08 | Add a current validity check to confirmation | P0 | Expired, released and revoked inventory cannot be confirmed | F03, F03 |
| CLM-09 | Implement the lease state machine | P0 | `available → reserved → issued → upstream-activated → productive`, with expiry, cancellation and revocation defined separately | A§5, A§8 |
| CLM-10 | Record an actor, evidence and timestamp for every transition | P0 | Each transition is individually auditable; a locally expired timer does not justify reissuing a code without upstream confirmation | A§5 |
| CLM-11 | Enforce the three-inventory model | P0 | `unassigned`, `assigned/in onboarding` and `productive` are separate counts, plus `expired` / `released` / `quarantined` for auditability | M§6 |
| CLM-12 | Enforce the 2,500 concurrent-occupancy hard cap | P0 | Every active or trial lease occupies inventory; concurrent occupancy can never exceed 2,500; a licence is never oversold into a queue | M§6, M§11 |
| CLM-13 | Implement the standby queue | P1 | Once inventory is full, applicants wait on a consented waiting list rather than receiving an unusable code | M§6, M§10 |
| CLM-14 | Reserve a code only after screening, with a reasonable activation window | P1 | No code is issued before eligibility confirmation; no threat of arbitrary confiscation appears in the flow | M§6 |
| CLM-15 | Stop relying on a masked key as a security control | P1 | The wizard's confirm-on-copy behaviour is understood as a local claim event, not activation | A§5, §2.1 |

---

### WS5 — The 50/40/10 agreement and the referral ledger

This is the commercial core. The audit is unusually valuable here (§8.4) because it **rejects** a third-party claim that no split arithmetic exists.

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| ECO-01 | Retire the `Split6040` / 50:50 / 55:45 enum in favour of explicit basis points | P0 | The enum means user:operator, so under a 40/10/50 agreement the portal falls into the 50:50 branch and **omits the referral allocation entirely** | F05, A§4.2 |
| ECO-02 | Store `ulo_bps`, `uno_bps`, `referral_bps` with an agreement version | P0 | All participant percentages survive round trips unchanged; 50/40/10 renders and reconciles as exactly 50/40/10 | F05 |
| ECO-03 | Enforce the 10,000-basis-point invariant at every write | P0 | No write can persist a split that does not total 10,000 | F05 |
| ECO-04 | Define whether the pool is before or after ecosystem deductions | P0 | An explicit, recorded decision, not an implicit one | F05 |
| ECO-05 | Preserve and correct `LeaseSplit::agent_portion_of_uno()` | P0 | The existing `agent_share / (uno_share + agent_share)` = `10 / 50` = 20% of the owner-side half is retained and documented — it is **not** 10% of the retained UNO's 40% | A§8.4 |
| ECO-06 | Replace `LeaseSplit::validate()` with real enforcement | P0 | Current check only approximates 100%; it does not establish nonnegative, finite, bounded components or per-write enforcement | A§8.4 |
| ECO-07 | Use checked integer basis-point arithmetic | P0 | No floating-point rounding errors in money allocation | A§8.4, PROP |
| ECO-08 | Document an explicit rounding policy | P0 | Treatment of fractional UP is specified, and participant amounts reconcile exactly to the applicable pool accounting for rounding and reversals | A§8.4 |
| ECO-09 | Store the original amount, currency/unit, pool definition and agreement version per allocation | P0 | A complete allocation audit trail exists | A§8.4 |
| ECO-10 | Verify the upstream semantics assumption in `ComputedAllocation::compute()` | P0 | The code comment asserts the upstream amount is already the owner-side share after ULO allocation; this is verified against real upstream data, not assumed | A§8.4 |
| RFR-01 | Make referral attribution first-write-only, bound inside the owner's claim transaction | P0 | Replaying confirmation with a different referral code changes nothing and creates no second payable event | F04 |
| RFR-02 | Snapshot the agreement version at attribution time | P0 | A later change to the split does not retroactively alter an existing attribution | F04 |
| RFR-03 | Separate attributed / accrued / payable / paid commission states | P0 | The referral lifecycle is tracked independently of attribution | F04, A§8.3 |
| RFR-04 | Add an audited privileged correction process for attribution changes | P1 | Corrections require explicit approval and leave a trail | F04 |
| RFR-05 | Enforce **single-level** referral as a ledger invariant, not a policy document | P0 | A second-level attribution is structurally impossible | M§4.4, user request |
| RFR-06 | Enforce one attributable referral source per licence | P0 | No stacked 10% cuts | M§4.4 |
| RFR-07 | Stop the sync loop from silently promoting pending applicants to active | P1 | Pending and rejected applications stay pending and rejected across repeated sync cycles; approval is an authorized transition, never a sync side effect | F08 |
| RFR-08 | Preserve commission explicitly — never reset to a default | P0 | The current default 3% never appears; the 10% referral is maintained across sync | F08 |
| RFR-09 | Paginate the referral list transport beyond the fixed 1,000-record limit | P1 | All pages synchronise; add tombstone and rejection handling | F08 |
| RFR-10 | Implement the first-qualified-source rule with an explicit attribution window and dispute process | P1 | The window length is specified — the plan never states a number, which is a gap to close | M§4.4, M§8 |
| RFR-11 | Implement the per-licence referral ledger | P0 | Per licence: pool, ULO allocation, UNO allocation, referral allocation, UNO-paid credits, adjustments, settled referral payout | M§8 |
| RFR-12 | Hold the 10% in a separately tracked referral/support reserve for referrer-less applicants | P1 | UNO stays conservatively at 40% in the model; the share is never silently turned into 50% UNO and a referrer is never fabricated | M§4.4 |
| RFR-13 | Preserve earned referral amounts when a lease ends | P1 | No clawback of legitimate ULO earnings to fund referral adjustments | M§4.4 |
| RFR-14 | Pay from actual settled rewards, never projections | P0 | Payout is gated on reconciliation | M§4.4 |
| RFR-15 | Enforce the no-commission list | P0 | No commission for KYC submissions, fake installs, self-referrals or duplicate attribution; referrers never collect documents, credentials, private keys, credit payments or registration fees | M§8 |
| RFR-16 | Suspend disputed payouts for review without punishing legitimate shared-network users | P1 | Shared-IP false positives are not auto-punished | M§13 |
| RFR-17 | Disallow HighLevel's affiliate programme as the compensation mechanism | P0 | Its 40% tier-1 plus 5% tier-2 structure would build exactly the multi-level arrangement M§4.4 forbids | A §2 of vendor supplement, M§4.4 |

---

### WS6 — Publication, synchronisation and reconciliation

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| PUB-01 | Reject publication of incomplete inventory | P0 | A missing lease code **fails** publication instead of substituting a random UUID | F05 |
| PUB-02 | Preserve verified upstream codes and dates | P0 | No manufactured UUIDs, no "current time + one year" substitutions; whether Unetwork accepts the resulting offer is never asserted | F05 |
| PUB-03 | Return per-item results with stable source IDs and error codes | P0 | A deliberately mixed valid/invalid batch produces exact per-row outcomes | F06 |
| PUB-04 | Mark only the rows the remote actually accepted as published | P0 | `insert_batch` no longer continues past per-insertion errors and returns a bare count | F06 |
| PUB-05 | Implement durable idempotency using the idempotency key already in the contract | P0 | The contract accepts the field; the service does not currently use it | A§4.2 |
| PUB-06 | Implement unpublication as an acknowledged remote state transition | P0 | It calls the portal revoke/remove API and handles in-flight reservations safely | F06 |
| PUB-07 | Verify an unpublished licence cannot be newly reserved by any public path | P0 | Including direct repository calls, not only the UI route | F06 |
| SYN-01 | Preserve the full upstream identifier alongside the internal UUID | P0 | The current conversion truncates a long hex ID to its first 32 characters and falls back to a **random UUID** on parse failure; no information is lost and malformed IDs are not silently replaced | F07 |
| SYN-02 | Implement cursor pagination on a composite key | P0 | `(claimed_at, id)`; repeated polling cannot re-fetch the same oldest 100 rows | F07 |
| SYN-03 | Remove the fixed 1,000-record list limits | P0 | The 100-claim poll and the 1,000-row UI refresh both sit below the 2,500 campaign size | F07 |
| SYN-04 | Persist the sync checkpoint only after successful reconciliation | P0 | An interrupted sync resumes safely; `poll_and_sync` currently passes no cursor at all | F07 |
| SYN-05 | Unify the identity contract across sync paths | P0 | One path uses returned portal IDs directly against admin IDs, another converts — the inconsistency is removed | F07 |
| SYN-06 | Fix the initial length-based slicing that mishandles standard hyphenated UUIDs | P1 | Both hex and hyphenated forms parse correctly | F07 |
| SYN-07 | Add a reconciliation worker for eventual consistency | P1 | A local commit followed by a failed HTTP call is a visible retryable state, not a silent success | A§7.3 |
| SYN-08 | Use an outbox for cross-database publication and revocation | P1 | Publication survives a crash between the local commit and the remote call | A§7.3 |
| SYN-09 | Bind HMAC signatures to a canonical method/path/body envelope | P0 | Changing the resource path invalidates the signature; payload-type reuse across routes is prevented | F09 |
| SYN-10 | Consume nonces with a durable TTL store | P0 | A captured request cannot replay. Today the nonce is never stored, so any signed request replays indefinitely inside the acceptance window | F09 |
| SYN-11 | Reject timestamps too far in the future | P1 | `age.abs() > 300` currently accepts timestamps 5 minutes ahead | N09, F09 |
| SYN-12 | Separate operation idempotency from replay defence | P1 | Legitimate retry behaviour is explicit rather than conflated with nonce rejection | F09 |
| SYN-13 | Rotate credentials and scope service-client permissions | P1 | Service clients hold least privilege | F09 |

---

### WS7 — Durable jobs, health and observability

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| JOB-01 | Replace the bounded in-memory Tokio channel with a durable queue or outbox | P0 | Interrupted commands resume; persisted job metadata alone does not prove this today | A§7.3 |
| JOB-02 | Add worker leases | P1 | Two admin instances do not run conflicting sync work | A§7.3, A§7.3 |
| JOB-03 | Add bounded retries with dead-letter handling | P1 | Failed jobs are tracked separately and are inspectable | A§7.3 |
| JOB-04 | Implement explicit restart recovery | P0 | Recovery is tested, not assumed | A§7.3 |
| JOB-05 | Separate liveness from readiness | P0 | Liveness reports process state; readiness checks essential dependencies with bounded timeouts | F12 |
| JOB-06 | Make readiness fail when claims cannot work | P0 | A database outage removes the instance from claim-serving while liveness stays separately observable; recovery restores readiness | F12 |
| JOB-07 | Fail startup on required configuration and migration failures in production | P0 | UI-only mode requires an explicit development flag | F12 |
| JOB-08 | Stop labelling the database "connected" without a query | P0 | The factory existing is not evidence of connectivity | F12 |
| JOB-09 | Fix the Cloudflare monitor's expectation of HTTP 200 | P1 | Monitoring reflects real readiness, so a nonfunctional claim service cannot stay healthy to routing | F12 |
| JOB-10 | Add actual HTTP-plus-database integration tests around F01–F08 | P0 | Concurrent allocation safety, authorization and reconciliation are demonstrated, not asserted | F13, A§12 |
| JOB-11 | Remove tests that pass when their server is unreachable | P0 | A test that prints a message and succeeds against a dead server is not counted as an integration test | A§12, N09 |
| JOB-12 | Replace the 22 self-referential placeholder tests with real ones | P0 | At minimum: a repository round-trip for `License`, an HMAC replay test, and an ownership test | N09, PROP |
| JOB-13 | Run the Playwright E2E suite in a workflow | P1 | It is currently never executed by any workflow | N09 |
| JOB-14 | Emit structured, redacted logs | P1 | Event IDs and outcomes are logged; complete lease codes and bearer tokens never are | A§7.1 |

---

### WS8 — i18n and CMS preservation

See §2.5 for the verified inventory and the reasoning. Schema repair (WS1) precedes this workstream.

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| I18N-01 | Add a regression test that proves existing localisation still works | P0 | All nine shipped locales render; the `phf::Map` lookups, `intl.rs` formatting and `lazy_loader.rs` paths are covered. Preservation is tested, not assumed | §2.5 |
| I18N-02 | Add Bangla (`bn`) | P0 | Bangladesh explicitly requires *"Bangla materials before expansion"*; this is the one genuinely missing launch locale | M§5, §2.5 |
| I18N-03 | Complete the Tagalog (`tl`) translation | P0 | `nav.home`, `nav.stats`, `nav.faq`, `nav.contact` and `nav.common_errors` are currently untranslated or mixed | §2.5 |
| I18N-04 | Add a content-layer translation workflow to the CMS | P1 | Editors can supply translations and set per-locale `complete` / `partial` status without a code change | §2.5, PROP |
| I18N-05 | Add a translation-completeness dashboard | P1 | Per-locale completion is visible per content type using the existing `translation_status` JSONB | §2.5, PROP |
| I18N-06 | Add `yo`, `ha` (Nigeria), `ta`, `te`, `bn-IN` (India) | P2 | Added when those markets open; Swahili and Tagalog are already shipped | M§5, §2.5 |
| I18N-07 | Add `ak`/`tw` for Ghana | P2 | Ghana runs English-only initially | M§5 |
| I18N-08 | Verify RTL layout beyond Arabic | P2 | `get_text_direction` already returns `rtl` for `ar/he/fa/ur`; layout is confirmed rather than assumed complete | §2.5 |
| I18N-09 | Make locale-aware economics display a first-class requirement | P1 | Currency formatting uses `intl.rs`, not hardcoded USD, in the payout and forecast surfaces | §2.5, M§4 |
| CMS-01 | Preserve the schema-driven content type system | P0 | `content_schemas` with JSONB field definitions and `is_system` protection continues to work | A§12, §2.5 |
| CMS-02 | Preserve the content item lifecycle | P0 | Draft → review → publish, with versioning, `is_featured` and `is_active` | A§12 |
| CMS-03 | Repair the review/publish/preview tables | P0 | `content_reviews` and `preview_tokens` exist; the rollback/`content_versions` mismatch is fixed first | F14, N07 |
| CMS-04 | Add RBAC-protected, scoped, expiring preview tokens | P1 | A public preview endpoint may legitimately authorise via a scoped expiring token; secure issuance and validation are the controls | F15 |
| CMS-05 | Add a locale field to content items and FAQs where missing | P1 | `locale` exists on content review and FAQ types; extend to all translatable types | §2.5 |
| CMS-06 | Add a one-page partner brief as a CMS content type | P1 | The brief is editable without code: eligibility, realistic net examples, exact cost payer, official links, prohibited claims, support route, attribution, compensation, and a "who should not join" box | M§8, PROP |
| CMS-07 | Add the approved transparency/eligibility page as a CMS content type | P1 | Countries, tasks, costs and exit terms are editable per market | M§14, PROP |

---

### WS9 — Product: the redefined user journey and interface

The current component tree is organised around licence acquisition. This workstream re-centres it on the participant lease lifecycle. Requirements are `Specified` from the marketing plan unless marked `PROP`.

#### WS9.1 The 10-step journey (M§9)

| ID | Step | Pri | Acceptance criteria |
|---|---|---|---|
| UI-01 | Landing page | P0 | Plain benefit; small-reward caveat; credit cost **and payer**; device list; country screen; current vs projected tasks; the reward split; the official participation link. No implication of cost-free earnings |
| UI-02 | Two-minute eligibility form | P0 | Adult confirmation; country; Android/iOS/Windows; existing connection and data cap; charging reliability; languages; consent; acknowledgement that credits are UNO-funded and other costs remain. **Must not collect household income or identity-document images** |
| UI-03 | Economics check | P0 | Shows a measured local pilot range *only when available*, the UNO-funded credits, other costs, and what happens if tasks are unavailable. Unsuitable applicants exit without pressure |
| UI-04 | Official onboarding handoff | P0 | The participant controls their own account, device permissions and KYC. Current official downloads only. **Never instruct users to disable device security globally** |
| UI-05 | Licence assignment | P0 | Binds in the official system, logs attribution and inventory status, records UNO credit funding and the first credited task |
| UI-06 | D1 contact | P1 | Resolves installation and connectivity failures using **opted-in reminders only** |
| UI-07 | D3 check | P1 | Compares expected against actual task availability and data use; **pauses unsupported combinations** rather than leaving a costly idle licence |
| UI-08 | D7 review | P1 | Quality check, first referral-ledger reconciliation, and publication of anonymised cohort metrics |
| UI-09 | Withdrawal support | P0 | Demonstrates the real process when the participant qualifies. **Never promises instant cash-out or local-bank conversion without a supported route** |
| UI-10 | D30 renewal | P1 | Shows actual net outcome, confirms whether to continue, reconciles the recurring referral share, offers an optional referral link |

#### WS9.2 Honesty and compliance surfaces (P0, all)

The marketing plan is unusually explicit that misleading presentation is a launch risk, not a style preference.

| ID | Task | Acceptance criteria | Source |
|---|---|---|---|
| UI-11 | "Who should not join" box | Present on landing and in the partner brief: costly data, negative measured net outcomes, unsupported phone/country, unwillingness to complete required verification, expectation of salary-scale income | M§8 |
| UI-12 | Cost-payer disclosure | States plainly who pays activation credits (UNO), that the ULO pays no licence-acquisition fee, and that internet, electricity, device wear, withdrawal costs and time may still matter | M§1, M§6 |
| UI-13 | Small-reward positioning | *"small supplementary rewards for optional device tasks"*. Not employment, not a salary, not guaranteed daily income, not a reason to buy a phone, broadband or GPU | M§1 |
| UI-14 | No-cost-avoidance notice | *"Please do not buy a phone or new data plan just to join"* and *"do not buy extra data merely to keep a licence active"* | M§12 |
| UI-15 | Prohibited-claims guardrails | No salary, guaranteed-income, NFT-ownership-gift or downline framing anywhere in the product. This is a **CMS content constraint plus a copy-review checklist**, not only a code concern | M§6, M§8 |
| UI-16 | Accurate-reward statement | *"The supplied historical export records Proof of Work and Caller-ID credits recently; individual results depend on eligible work."* The portfolio aggregate is never presented as a typical participant outcome | M§3 |
| UI-17 | No-pressure exit | Plain-language exit terms and a reachable exit route; no confiscation threats | M§6 |

#### WS9.3 Economics and transparency surfaces

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| UI-18 | Live 50/40/10 split display | P0 | Renders the actual configured basis points, not hardcoded strings. Shows the three-way split to the ULO and never describes the referral payment as an additional deduction from the ULO's 50% | M§8, M§12 |
| UI-19 | Per-licence referral ledger view | P0 | Pool, ULO allocation, UNO allocation, referral allocation, UNO-paid credits, adjustments and settled payout, per licence | M§8 |
| UI-20 | Participant earnings transparency | P1 | Actual measured net outcome; **not** raw allocations, which are not active devices | M§9, A§8.5 |
| UI-21 | Interactive forecast calculator | P1 | The `UNETWORK_REVENUE_CALCULATOR.html` engine as an in-product surface — see WS10 | CALC, M§16.8 |
| UI-22 | Task compatibility and troubleshooting pages | P1 | SEO-friendly, accurate, and CMS-editable | M§7 |
| UI-23 | Searchable FAQ plus one pinned setup video | P1 | Exactly one pinned setup video, archived so instructions do not drift | M§9 |
| UI-24 | Weekly control dashboard (internal) | P0 | Tracks country, source and cohort **together**. Minimum fields: opted-in prospects, eligibility pass rate, installation, first accepted reward, D7, D30, churn, active inventory, realised UP/device-day, credits payer, support minutes, data costs, acquisition spend, accrued referral-share liability, complaints, payout blockers | M§13 |
| UI-25 | KPI and trigger panel | P0 | 14 indicators with target and action: opt-in capacity 1,000/wk; prospect→first reward ~35%; D7 ≥85%; D30 ≥75%; weekly churn ≤2%; net productive additions ≥250; D30 acquisition cost ≤3 months of positive contribution; ULO net outcome positive; UNO contribution positive; task capacity; participant willingness; promoter viability; paid experiment ≤$30; source quality | M§13 |
| UI-26 | Inventory visualisation | P1 | The three inventories plus expired/released/quarantined, live | M§6 |
| UI-27 | Funnel view | P0 | `lead → eligible → reserved → issued → activated → D7 productive → D30 retained`, reporting gross additions **and** net retained, with claims based on the stage actually measured | A§8.5 |
| UI-28 | Reconciliation dashboard | P1 | Portal / admin / upstream state comparison with explained discrepancies | A§9 |
| UI-29 | Evidence register | P1 | Dated entries for source URL and build, export period and hash, billing evidence, pilot cohort, permission scope and decision owner | M§15 |
| UI-30 | Decision log | P1 | The UNO owns it; the coordinator gathers evidence and reports failures. A gate passes with dated evidence, not a checklist assertion | M§1 |

#### WS9.4 Attractive-not-misleading additions (`PROP`)

These are the changes intended to make the product materially better to use. Each is justified by a stated source problem, not by aesthetics alone.

| ID | Task | Pri | Rationale | Source |
|---|---|---|---|---|
| UI-31 | One-page participant status header | P0 | Participants currently have no single view of "where am I in the lifecycle, what happens next, what is my measured net". Replaces navigation-by-guessing across the current component tree | A§5, M§9 |
| UI-32 | Next-best-action card | P0 | The plan's own operational problem is idle licences and unsupported device/task combinations. Surfacing the next action directly reduces support load and D3 pauses | M§9 |
| UI-33 | Device and task compatibility checker | P0 | Pre-qualifies before a code is reserved, so screening happens before inventory occupancy rather than after | M§6, M§9 |
| UI-34 | Offline-tolerant claim wizard with resumable steps | P1 | Mobile-first markets with intermittent connectivity. Resumption must not create a second reservation | A§7.2, PROP |
| UI-35 | Referrer dashboard with a single unambiguous link | P1 | Referrers need to see attributed licences, accrued versus payable versus paid, and their honest per-hour reality ($0.56/month per licence at a $5.60 pool) | M§8, PROP |
| UI-36 | Partner self-service console | P2 | Converts the weekly partner rhythm from manual reporting to self-serve, without exposing participant PII | M§7, PROP |
| UI-37 | Cohort comparison view | P1 | The hold triggers depend on comparing incumbent and new cohorts on reward per eligible device-day and zero-reward days | M§10, PROP |
| UI-38 | Anonymised cohort metrics publication | P1 | Explicitly required at D7; doubles as a trust and recruitment asset | M§9 |
| UI-39 | Accessibility and low-bandwidth mode | P1 | The target markets are bandwidth-constrained. Images and video are the heaviest assets on a local host serving directly | FS-09, PROP |
| UI-40 | Consent and preference centre | P0 | Every lifecycle touchpoint is opt-in. Reminders, WhatsApp/Telegram conversations and cohort publication all require explicit, revocable consent — no purchased lists, no scraped members, no unsolicited bulk messaging | M§7, M§9 |
| UI-41 | Withdrawal and exit flow with a published threshold rule | P0 | The plan withdrew its 39-day example as unverified. The product must therefore show *measured* accrual and a stated rule, not a promised date | M§9 |
| UI-42 | Prohibited-claim linter over CMS content | P2 | Automates part of UI-15: scan CMS content for banned framings (salary, guaranteed income, downline, NFT gift) before publish | M§8, PROP |

---

### WS10 — Productising the revenue forecast engine

`UNETWORK_REVENUE_CALCULATOR.html` is a self-contained, dependency-free, 21 KB forecasting engine. It is a genuine asset and the plan's §16.8 explicitly expects it to remain the canonical model. The requirement is to make it a first-class, testable, server-side component rather than a file someone opens locally.

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| FC-01 | Port `simulate()` to a typed, tested server-side module | P0 | The existing implementation is a pure function, so this is a lift rather than a rewrite. It must be byte-compatible with the HTML's outputs for the default scenario | CALC |
| FC-02 | Port `validate()` with the same rules | P0 | Horizon capped at 260 weeks; weeks/capacity/newNet/amortWeeks positive integers; churn/tax/fee/shares within [0,1]; device mix totals 1; reward shares total 1; no negative values | CALC |
| FC-03 | Fix the `upUsd = 0` silent-zeroing defect | P0 | It currently validates and zeroes the entire business with no warning. Reject it or warn explicitly | CALC |
| FC-04 | Fix the `capital` / `amortWeeks` truncation defect | P0 | With `amortWeeks > weeks`, half the capital is never expensed and nothing says so. Warn and surface the unallocated balance | CALC |
| FC-05 | Preserve the credit-renewal sawtooth semantics | P0 | Credits renew on every 30-day anniversary per cohort, so weekly credit cost oscillates. This is intentional and must not be "smoothed" away | CALC |
| FC-06 | Preserve and document the `failures/14` support term | P0 | It reads like a per-day per-failure rate but yields exactly one half-day of support per failure. Naively simplifying it to `failures/2` inside the day loop over-charges support 7×. Document it in code | CALC |
| FC-07 | Assert the revenue identity in tests | P0 | `ulo + uno + referral == pool` and `pretax == uno − operatingCosts − capitalCharge`. The current code computes `ulo` and `referral` and never uses them, so a future split change could leak money silently | CALC |
| FC-08 | Implement the 10-field pluggable task schema | P1 | Name, enabled, device booleans, daily reward, rate basis, eligible fraction, activity factor, start/end week, daily pool cap, extra cost | CALC, M§16.8 |
| FC-09 | Implement the three rate bases correctly | P1 | `pool` needs no reconstruction; `historical_uno` divides by 0.50 (the owner's export convention, **not** the proposed 40%); `ulo` divides by the current ULO share. A reward must never be grossed up twice | CALC, M§16.8 |
| FC-10 | Enforce the illustrative-placeholder guard | P0 | An active illustrative pool combined with active named tasks is rejected, preventing accidental double counting. The default active task is `Illustrative combined reward pool — REPLACE` | CALC, M§16.8 |
| FC-11 | Named-task rate defaults stay zero and disabled | P0 | Proof of Work, Scout, Other, CLI/Caller-ID, Runner Calls, Extended Telemetry, Ugrid, Entropy, GPU Contribution — all disabled with rate 0 until measured values are supplied | CALC |
| FC-12 | Persist scenarios server-side | P1 | The HTML has no `localStorage` and loses all edits on reload; only manual JSON save/load survives, and the import handler ignores its own `version` field. Scenarios become first-class records | CALC, PROP |
| FC-13 | Scenario save/load with schema versioning | P1 | `version` is written but never checked on import, and a JSON missing a key produces a raw JS error. Validate the schema and fail with a clear message | CALC |
| FC-14 | CSV export of all 31 computed fields | P1 | The on-screen table shows 20 of 31; `attempts`, `deviceDays`, `cumulativeCash` and others are export-only. The export must be complete and stable | CALC |
| FC-15 | Render the full weekly table in-product | P1 | Include opening, churn, additions, attempts, deviceDays, operatingCosts, pretax, cashCosts, cumulativeCash and requiredFunding | CALC |
| FC-16 | Clarify the ambiguous output labels | P1 | `Credit blocks` is formatted as currency, not a count. The "Week N" cards silently report week 10. A dangling `*` footnote marker on ULO/Referral headers has no footnote. All fixed | CALC |
| FC-17 | Reconcile `net` and `closing` against cohort state | P1 | The stored `closing` value is never read back and can drift by float noise; `net` is a local and never reaches the CSV. Derive both from cohort state | CALC |
| FC-18 | Fix negative-zero rendering | P2 | The `1e-9` epsilon guard can produce `-$0.00` that is not styled as negative | CALC |
| FC-19 | Port the chart | P1 | Inline SVG, two series (UNO earned, net profit), with a real y-axis floor, collision-aware legend and accessible labelling. Add the missing intermediate ticks and tooltips | CALC, PROP |
| FC-20 | Add optimistic recompute and incremental render | P2 | The current renderer rebuilds up to 5,200 cells and the full SVG on every change with no debounce or diffing | CALC |
| FC-21 | Seed the model with the three named scenarios | P1 | Downside $5 pool/month, Reference $7.50, Upside $10 — as presets that reset tasks but preserve non-task assumptions | CALC |
| FC-22 | Add a config reset control | P2 | Only tasks have presets; config can only be reset by reloading, which also loses the JSON | CALC |
| FC-23 | Add a pool-identity assertion to the UI | P1 | The displayed "50/40/10" must come from the same basis-point values as the ledger, never from a hardcoded string | CALC, ECO-02, PROP |

---

### WS11 — Data governance, privacy and launch gates

| ID | Task | Pri | Acceptance criteria | Source |
|---|---|---|---|---|
| GOV-01 | Treat lease codes as claim credentials | P0 | Redacted from logs; never published raw in audit examples or UI beyond the intended issuance point | A§7.1, A§7.2 |
| GOV-02 | Define data retention, deletion handling and access scopes | P0 | Written and enforced; log event IDs and outcomes rather than complete codes or tokens | A§7.1 |
| GOV-03 | Assess whether storing raw visitor IPs is necessary | P0 | Justified or removed. The portal currently records IP and user-agent through visitor middleware | A§7.1 |
| GOV-04 | Keep KYC with the authorised provider | P0 | No identity-document images in the app. The eligibility form must not collect them | A§7.1, M§9 |
| GOV-05 | Refer to sensitive external identity data rather than copying it | P1 | The data model stores references, not duplicated PII | A§8.5 |
| GOV-06 | Determine the provenance and authorisation of the committed reward exports | P0 | The repository contains reward exports with user, node, licence and lease identifiers, amounts, timestamps and task metadata. Confirm whether they are real, synthetic or authorised test data | A§7.2 |
| GOV-07 | Restrict exposure and use synthetic fixtures in tests | P0 | Raw identifiers do not appear in test data or documentation | A§7.2 |
| GOV-08 | Assess history cleanup where warranted | P1 | Deleting a file from the latest commit does not remove historical exposure; plan accordingly | A§7.2 |
| GOV-09 | Do not treat IP geography as identity | P0 | MaxMind lookup and geographic fallback are not a country-admission service. Country admission is a rules table, not a geolocation guess | A§7.1, M§5 |
| GOV-10 | Implement the eight launch gates as product features | P0 | A gate passes with **dated evidence**, not a checklist assertion. See §7.4 below | M§1, PROP |
| GOV-11 | Implement the weekly operating rhythm | P1 | Monday reconcile inventory and reward data; Tuesday–Thursday partner sessions and clinics; Friday source/cohort review and bounded budget changes; weekend optional self-service onboarding and a scheduled support window | M§7, PROP |
| GOV-12 | Do not label every unexcluded country "supported" | P0 | Maintain an international waiting list for markets with no launch quota | M§5 |
| GOV-13 | Keep forecast assumptions separate from observed results | P0 | Distinct fields, distinct tables. This is what makes the pilot honest | A§8.5 |
| GOV-14 | Store the funnel stage as measured, not as claimed | P0 | Campaign claims are based on the stage actually measured | A§8.5 |

---

## 5. Explicit commercial and software gate separation

The audit (§13.4) draws a distinction that must survive into the product:

> **Software gate:** reproducible builds and migrations; authorized writes; exclusive issuance; correct agreements; durable reconciliation; upstream activation evidence.
>
> **Commercial gate:** measured task-specific device exposure and credited rewards; participant net benefit after data/power/time; verified UNO credit/support/acquisition costs; task-supply and country/device eligibility evidence; observed activation and retention.

**Requirement:** the product must present these as two independent gates and must not allow a passing software gate to imply a passing commercial gate, or the reverse. Task GOV-10 carries this.

The audit's own conclusion is the correct framing for this document:

> "The present evidence does not substantiate a profitable 2,500-licence rollout or guarantee 250 productive additions weekly. It also does not prove the business is impossible. Software repairs cannot create customer demand, but can improve activation, reliability, measurement and cost. Scale only when both gates are satisfied."

---

## 6. Target data model

Minimum campaign data model, from the audit §8.5, with additions required by this programme. Every field below is either Specified or Proposed; none are optional.

| Entity | Required fields |
|---|---|
| **Upstream licence** | `upstream_licence_id` (**full, untruncated**), `internal_id`, `lease_code`, `valid_from`, `valid_to`, `upstream_state`, `retrieved_at` |
| **Agreement** | `agreement_version`, `ulo_bps`, `uno_bps`, `referral_bps` (invariant: sum = 10,000), `pool_definition` (before/after ecosystem deductions), `effective_from` |
| **Participant** | `participant_ref` (not raw identity), `country`, `device_class` (android/ios/windows), `eligibility_result`, `eligibility_evidence_at`, `consent_state` |
| **Reservation** | `reservation_id`, `licence_ref`, `claimant_ref`, `session_token` (unguessable), `reserved_at`, `expires_at`, `released_at` |
| **Issuance** | `issued_at`, `issued_to_ref`, `lease_code_revealed_at`, `device_id` (nullable until bound), `bound_to_device` |
| **Activation** | `upstream_activated_at`, `activation_evidence_ref` — **never inferred from a local flag** |
| **Productivity** | `d7_state`, `d7_evaluated_at`, `rewarded_days_last_7`, `zero_reward_days`, `up_per_eligible_device_day` |
| **Referral attribution** | `referral_id`, `source_code`, `source_type` (partner/ambassador/participant-referral/organic/paid), `attributed_at`, `agreement_version_snapshot`, `attribution_window_expires_at`, `immutable` |
| **Commission lifecycle** | `accrued_at`, `eligible_at`, `payable_at`, `paid_at`, `reversed_at`, `dispute_state` — **four distinct states, not one** |
| **Allocation ledger** | `pool_amount`, `pool_currency_unit`, `ulo_amount`, `uno_amount`, `referral_amount`, `rounding_adjustment`, `reversal_of` |
| **Credit funding** | `funded_by` (UNO), `credit_block_id`, `block_start`, `block_end`, `amount`, `invoice_ref` |
| **Support** | `support_minutes`, `touchpoint` (D1/D3/D7/D30) |
| **Funnel** | `stage`, `stage_entered_at`, `country`, `source`, `cohort`, `measured` (bool) |
| **Sync** | `sync_checkpoint (claimed_at, id)`, `last_reconciled_at`, `upstream_identity_confirmed` |
| **Reconciliation** | `reconciliation_event`, `local_state`, `admin_state`, `upstream_state`, `discrepancy`, `explained_by` |

Three structural rules:

1. **Gross and net are both stored.** Report gross additions *and* net retained. Counting net change rather than gross code claims is a hard requirement (M§13).
2. **Raw allocations are never charted as active devices** (M§13). The funnel's terminal stages are D7-productive and D30-retained, both measured.
3. **Everything is a reference.** Sensitive external identity data is referenced, not copied (A§8.5).

---

## 7. State machines

Three separate models. v1.0 conflated them; they serve different purposes and must not be merged.

### 7.1 Lease lifecycle (audit §5, `Specified`)

```
available ──▶ reserved ──▶ issued ──▶ upstream-activated ──▶ productive
     │            │           │              │                    │
     └────────────┴───────────┴──────────────┴────────────────────┘
                    │           │              │
              expiry · cancellation · revocation · quarantine
```

Each transition carries its own timestamp, actor, evidence and retry policy. A reservation needs an owner and an expiry. An issued code must not be reissued merely because a local timer expired unless upstream confirms reuse is safe. "Productive" requires an agreed observation window and actual credited work — not connectivity and not a copied key.

### 7.2 Inventory model (marketing plan §6, `Specified`)

Three separate inventories, plus auditability states:

| Inventory | Meaning |
|---|---|
| `unassigned` | Available for screening and reservation |
| `assigned_in_onboarding` | Reserved or issued, not yet productive |
| `productive` | D7-productive, the only state that counts toward the 2,500 target |
| `expired` / `released` / `quarantined` | Auditability states, each with a reason |

**Hard invariant:** every active or trial lease occupies inventory, and concurrent occupancy must never exceed 2,500. `remaining = 2,500 − verified current productive licences`.

### 7.3 Funnel (audit §8.5 + marketing plan §9, `Specified`)

```
lead → eligible → reserved → issued → activated → D7 productive → D30 retained
```

**D7 productive definition, adopted verbatim as an acceptance predicate (M§9):**

> verified unique consenting adult; official active licence/device; accepted rewarded activity on at least four of the last seven days; no unresolved payout/credit blocker; applicable platform rules met. Proposed screening preference: internet available at least 90% of the participant's intended operating period, with actual task qualification tracked separately. This preference does not override a lease requiring greater uptime.

**D30 is not 75% of D7.** Both denominators are the original activated cohort (M§9).

### 7.4 Launch gates (marketing plan §1, `Specified`)

Implemented as product features under GOV-10. A gate passes with dated evidence.

| Gate | Required evidence | If it fails |
|---|---|---|
| Permission and settlement | Applicable lease terms; platform-approved recruitment route and copy; supported 50/40/10 accounting and UNO credit funding | Use only authorised discovery; resolve settlement before promising referral payouts |
| Product and eligibility | Official Android build installs on intended devices; applicable attestation/KYC works; an accepted task reward appears; eligible withdrawal route checked | Pause the affected device/market; do not infer support from a download link |
| Data and billing | Export recipient and task labels defined; funded licence-months reconciled; actual credit invoice, exemptions, renewal and reassignment rules recorded | No per-device earnings claims or large credit commitment |
| Task availability at increasing scale | Incumbent versus new-cohort accepted activity, reward per eligible device-day, zero-reward days; platform guidance on task/country quotas and onboarding capacity | Hold the next cohort; diagnose workload, eligibility or connectivity rather than assume marketing can fix it |
| UNO commercial margin | Realised contribution after credits/support is positive; expected retained contribution covers acquisition plus allocated overhead | Reduce acquisition cost, improve productivity or negotiate credit price; **do not pass costs back to ULOs** |
| Participant value | Positive measured net outcomes plus informed uptake and voluntary continued participation in each tested market | Change the offer or stop that market; positive cash alone is insufficient |
| Promoter and funnel capacity | Partners accept disclosed compensation; two completed weekly cohorts demonstrate sufficient unique opt-ins, conversion and retention | Replace unproven channel quotas or extend preparation |
| Funding and inventory | Credits, earned referral liabilities and support are funded; active trials plus productive licences stay within 2,500 | Limit admissions to funded, legally reusable slots |

### 7.5 Capacity validation ladder (marketing plan §10, `Specified`)

30 → 100 → 250, each only while the §7.4 gates pass.

**Hold triggers (any of):** a greater than 20% fall in incumbent reward per comparable eligible device-day; a greater than 10 percentage-point rise in zero-reward days; either party's economics failing. On hold, investigate — do not buy replacements.

**Downside arithmetic, for the forecast surfaces:** 0.60 × 0.60 × 0.60 × 0.70 = **15.12%** end-to-end conversion. At that rate, 250 net additions requires ~1,654 opt-ins per week, and 2,500 licences take ~20 production weeks (~35 at 5% churn).

---

## 8. Sequencing

Seven phases, following the audit's remediation sequence (A§9) with this session's findings inserted. The audit's caution is retained: *"No calendar commitment is justified from source size alone."*

| Phase | Goal | Workstreams | Exit evidence |
|---|---|---|---|
| **1. Contain** | Close exposed surfaces | WS0 (BLD-01, BLD-07, BLD-08), WS2 (SEC-01 to SEC-03, SEC-11 to SEC-15) | All privileged entry points deny unauthorized callers; `/debug` unreachable; middleware actually installed; exposed credentials rotated if confirmed |
| **2. Reproduce** | Make the build and schema real | WS0 (remainder), WS1 (DB-01 to DB-06, DB-11, DB-13) | A clean checkout builds SSR, WASM and the image; fresh-install and upgrade tests execute actual licence/RBAC/CMS queries |
| **3. Trust the inventory** | Honest publication | WS1 (DB-07, DB-08), WS6 (PUB-01 to PUB-07) | Import/publish/revoke reconciles with deliberately invalid and duplicate records |
| **4. Make claims exclusive** | One lease, one owner | WS3 (FS-01 to FS-10), WS4 (CLM-01 to CLM-15) | Concurrent HTTP/DB tests, replay tests and abandoned-reservation tests pass |
| **5. Make money correct** | 50/40/10 that reconciles | WS5 (all ECO and RFR tasks) | Allocations reconcile exactly in integer basis points; attribution is immutable |
| **6. Make sync durable and support the product** | Recoverable jobs, the participant journey | WS6 (SYN-01 to SYN-13), WS7, WS8, WS9, WS10, WS11 | Over 2,500 records reconcile across restarts; the 10-step journey is walkable; the forecast engine is server-side and tested |
| **7. Release gradually** | Validate with real participants | Pilot: 30 → 100 → 250, gates in §7.4 | Controlled pilot with matched local and upstream records and explained discrepancies |

**Phases 1–2 are strictly sequential and blocking.** Nothing in WS3–WS11 can be verified while the image cannot build and the schema cannot satisfy a single query.

**Explicit non-sequencing:** WS9 (product/UI) can be prototyped in parallel from Phase 2, because it depends on the *contracts* (ECO-02 basis points, §7 state machines) rather than the implementations. Landing on the wrong contract and then rebuilding the UI is the expensive mistake; landing on the right contract with a placeholder backend is cheap.

---

## 9. Acceptance suite

### 9.1 The audit's 13 essential items (must all pass before launch)

| # | Item | Verifying tasks |
|---|---|---|
| 1 | Every privileged route, server function and WebSocket denies an unauthorized caller | SEC-01 to SEC-10, SEC-02 |
| 2 | Many simultaneous sessions competing for a smaller inventory never acquire the same credential as different owners | CLM-01 to CLM-06 |
| 3 | Cross-owner confirmation fails; revoked/expired inventory unavailable; legitimate owner retries are safe | CLM-06 to CLM-08 |
| 4 | Partial publication failures are visible per item; retries do not duplicate; unpublish takes effect remotely | PUB-03 to PUB-07 |
| 5 | 50/40/10 survives import, publication, UI display, reward allocation and export without reinterpretation | ECO-01 to ECO-10, FC-23, UI-18 |
| 6 | Replay cannot replace attribution; pending/rejected applicants stay unapproved through sync | RFR-01, RFR-03, RFR-07, SYN-10 |
| 7 | Over 2,500 events with equal timestamps, duplicate deliveries and interrupted workers reconcile without omissions | SYN-01 to SYN-05, JOB-01, JOB-04 |
| 8 | Unauthorized access, traversal and oversize streaming requests are rejected | FS-01 to FS-05, SEC-17 |
| 9 | Database or upstream outages produce accurate readiness and retry states without false claim success | JOB-05 to JOB-08, SYN-07 |
| 10 | Fresh installs and upgrades run actual queries; CI cannot pass with migration failures or unreachable test servers | DB-11, DB-13, BLD-08, JOB-11 |
| 11 | Validated integer shares, rounding, duplicate/reversed reward events and upstream pool semantics reconcile | ECO-02, ECO-06 to ECO-10, FC-07, FC-04 |
| 12 | Release artifacts and build configuration contain no privileged service token | SEC-11 to SEC-13 |
| 13 | For each pilot participant, code issued, upstream activation, credit funding, credited task output and cash redemption are distinguishable | CLM-09, GOV-14, UI-27 |

### 9.2 Additional acceptance items specific to this relaunch

| # | Item |
|---|---|
| 14 | A clean `git clone` builds SSR, WASM and the container image with no developer-local files |
| 15 | CI evaluates all declared jobs, and a `cargo audit` failure blocks merge |
| 16 | Concurrent occupancy never exceeds 2,500 across all three inventories combined |
| 17 | A second-level referral attribution is structurally impossible |
| 18 | The forecast engine returns byte-identical results to `UNETWORK_REVENUE_CALCULATOR.html` for the default scenario |
| 19 | `ulo + uno + referral == pool` and `pretax == uno − operatingCosts − capitalCharge` hold for every generated row |
| 20 | No cloud storage SDK call appears on any production request path, and a container restart preserves all content |
| 21 | All nine shipped locales still render, plus `bn`; no `tl` key falls back to English |
| 22 | The eight launch gates each record dated evidence, and a passing software gate is never presented as a passing commercial gate |
| 23 | A withdrawn licence is unreachable by every public claim path, including direct repository calls |
| 24 | No screen presents raw allocations as active devices, or a promise of instant cash-out, salary, guaranteed income, NFT ownership or downline income |

---

## 10. Traceability matrix

Every audit finding, every audit non-numbered section and every new finding maps to at least one task. Nothing is dropped.

### 10.1 Audit findings F01–F19

| Finding | Audit severity / priority | Tasks |
|---|---|---|
| F01 Privileged admin actions lack an authenticated requester | Critical if exposed / P0 | SEC-01, SEC-04, SEC-06, SEC-07, SEC-10 |
| F02 File mutation handlers have no access checks | Critical where credentials permit deletion / P0 | FS-01 to FS-05, SEC-17 |
| F03 Reservation is a read; confirmation is not owner-bound | High / P0 | CLM-01 to CLM-08, CLM-15 |
| F04 Referral attribution can be overwritten after claim | High / P0 before commission payments | RFR-01 to RFR-04 |
| F05 Publication manufactures lease data; reverses share meaning | High / P0 | PUB-01, PUB-02, ECO-01 to ECO-04, DB-03, DB-16 |
| F06 Partial publication failures hidden; unpublish is local | High / P0 | PUB-03 to PUB-07 |
| F07 Identity conversion and bounded polling break reconciliation | High / P0 for the campaign | SYN-01 to SYN-06 |
| F08 Referral sync can undermine approval status | High / P1 before recruitment opens | RFR-07, RFR-08, RFR-09 |
| F09 HMAC freshness is not replay protection | Medium, higher for money ops / P1 | SYN-09 to SYN-13 |
| F10 Local storage lacks path containment | High for the local-backend condition / **P0 here, because D2 activates it** | FS-01 to FS-04, FS-13 |
| F11 Build and CI are not reproducible | High operational risk / P0 for release | BLD-01 to BLD-07, BLD-11 to BLD-13, DB-13 |
| F12 Health can report success without a database | High operational risk / P1 | JOB-05 to JOB-09 |
| F13 Scaffolding and tests overstate assurance | Medium to High / P1 | SEC-03, SEC-17, SEC-18, JOB-10 to JOB-14, BLD-08, BLD-14 |
| F14 Committed schemas do not support the active repositories | **Release blocker** / P0 | DB-01 to DB-09, DB-11, DB-12, DB-13, CMS-03 |
| F15 CMS server functions are unguarded privileged entry points | High, potentially Critical / P0 | SEC-08, SEC-09, CMS-04 |
| F16 Build-time JWT capture | **P0 remediation here** (escalated from P0 investigation) | SEC-11 to SEC-16 |
| F17 Availability disagrees with claim eligibility | Medium / P1 | DB-15, DB-16 |
| F18 Import contract and tests disagree | P1 before bulk import | DB-17 |
| F19 Storage client lifetime and upload content policy | Medium / P1 | FS-06, FS-07, FS-11, FS-12 |

### 10.2 Audit non-numbered sections

| Section | Tasks |
|---|---|
| §1 Executive assessment (6 go/no-go decisions) | §5 above; GOV-10; the software/commercial gate split |
| §3 Architecture and responsibility boundaries | §3.1; DB-14; R1–R5 |
| §4.2 Durable idempotency not implemented despite the contract field | PUB-05 |
| §4.4 Multipart buffering; unimplemented S3/Azure backends | FS-05, FS-15 |
| §5 Stage table; target state machine | §7.1; CLM-09, CLM-10 |
| §7.1 Identity and sensitive data | GOV-01 to GOV-05, GOV-09, JOB-14 |
| §7.2 Committed reward exports and data governance | GOV-06 to GOV-08 |
| §7.3 Background work and resilience | JOB-01 to JOB-04, SYN-07, SYN-08 |
| §7.4 Infrastructure proportional to the campaign | §3.2; DB-14 |
| §8 Support for the proposed economics (6 requirements) | WS5; §6 data model |
| §8.4 Existing commission arithmetic | ECO-05, ECO-06, ECO-10 |
| §8.5 Minimum campaign data model | §6; UI-27 |
| §9 Remediation sequence; 13 acceptance items | §8; §9.1 |
| §10 Corrections to earlier interpretations | R1; §5; UI-20, GOV-09 |
| §12 Build evidence; assets worth preserving | BLD-14, JOB-10, JOB-11, SEC-19, CMS-01, CMS-02 |
| §13.1 Dataset identity and deduplication | GOV-06; §11 open questions |
| §13.4 Commercial and software release gates are separate | §5; GOV-10 |
| §14 Adjudications and outstanding inputs | §11 open questions; §12 non-goals |

### 10.3 New findings from this session (not in the audit)

| ID | Finding | Evidence | Tasks |
|---|---|---|---|
| N01 | **Build blocker.** `Cargo.lock` is gitignored but `COPY`ed by the Dockerfile | `.gitignore:8`, `Dockerfile:26,46` | BLD-01, BLD-07 |
| N02 | **Unauthenticated data exfiltration path.** Three debug server functions on an unconditionally-mounted route, up to 1,000 rows each | `src/routes/debug.rs:49,88,118` | SEC-02 |
| N03 | **Security middleware is dead code.** Only `VisitorTracker` and the logger are wrapped; `upload_files` and `confirm_license_claim` take no auth parameter | `src/main.rs:119-121`, `file_handler.rs:119`, `licenses.rs:198` | SEC-01, SEC-03, SEC-05 |
| N04 | The `split_type` enum does not exist; the only `CREATE TYPE` is `content_status` | `migrations/00008_cms.up.sql:9` | DB-03 |
| N05 | The licences schema is a **different schema**, not a drifted one. 1 of 10 columns overlaps and its type is incompatible | `00002_licenses.up.sql:8-33` vs `license_repository.rs:34-35` | DB-01, DB-02, DB-11 |
| N06 | CI gate miscount: 7 jobs declared in `needs`, 5 evaluated | `ci.yml:200` vs `205-209` | BLD-08 |
| N07 | Migration `00005` missing; `00008_cms.down.sql` drops `content_versions`, which `00012` never creates | `migrations/`, `00008_cms.down.sql:4-13` | DB-05, DB-06 |
| N08 | Deploy jobs `cd` into a nonexistent path; the Terraform workflow watches a path filter that does not match | `infrastructure.yml:7,11` | BLD-09, BLD-10 |
| N09 | 22 of 26 Rust tests assert on values they construct themselves; the 4 that touch a server soft-pass when it is down. Playwright is never run | `tests/` | JOB-11 to JOB-13, BLD-14 |
| N10 | HMAC accepts future timestamps (`.abs() > 300`) and never stores the nonce, so any signed request replays within the window | `hmac.rs:32,40-63` | SYN-10, SYN-11 |

### 10.4 Marketing plan and calculator requirements

| Requirement | Tasks |
|---|---|
| 10-step journey (M§9) | UI-01 to UI-10 |
| Three inventories + 2,500 cap (M§6) | CLM-11, CLM-12, UI-26, acceptance 16 |
| D7 / D30 definitions (M§9) | §7.3, UI-27, GOV-14 |
| 50/40/10 economics (M§4) | WS5 in full |
| Referral agreement (M§4.4) | RFR-05 to RFR-17 |
| Eight launch gates (M§1) | §7.4, GOV-10 |
| Weekly dashboard + KPIs (M§13) | UI-24, UI-25, GOV-11 |
| Channel taxonomy and source attribution (M§7) | §6 source_type, UI-27, UI-36 |
| Partner brief and "who should not join" (M§8) | CMS-06, UI-11 |
| Copy blocks and prohibited claims (M§12) | UI-12 to UI-17, UI-42 |
| Country allocation and language needs (M§5) | I18N-02 to I18N-08 |
| Capacity ladder and hold triggers (M§10) | §7.5, UI-37 |
| Forecast engine and task schema (M§16.8, CALC) | WS10 in full |

---

## 11. Open questions requiring a decision before Phase 3

These are not implementation details. Each changes the data model or the product's legal position.

| # | Question | Why it blocks | Owner |
|---|---|---|---|
| Q1 | Is `ups` in the incentive export the UNO's credited share, or the complete distributable pool? | The entire `P = 2H` reconstruction depends on it. The plan warns: *"If `ups` is already the combined pool, do not double it."* | UNO, against the upstream ledger |
| Q2 | Are the committed reward exports real, synthetic or authorised test data? | A§7.2 requires this before any of it is used in tests or examples | UNO |
| Q3 | Is the pool defined before or after ecosystem deductions? | ECO-04. A pool definition stated two different ways in two places makes every allocation number untrustworthy | UNO + platform |
| Q4 | What is the numeric attribution window for the first-qualified-source rule? | RFR-10. The plan never states a number | UNO |
| Q5 | Where does the 10% go for a referrer-less applicant, in practice? | RFR-12. The plan requires a separately tracked reserve and conservatively keeps UNO at 40% | UNO |
| Q6 | Is the platform's 50/40/10 settlement supported, or is an authorised operator accounting arrangement used? | Launch gate 1: *"resolve settlement before promising referral payouts"* | Platform |
| Q7 | Is promotion and subleasing authorised, and is the recruitment copy approved? | Launch gate 1, and the precondition for every acquisition channel | Platform |
| Q8 | What is the maximum domain age per country/category (CPM, CVR)? | The plan shows a downside case that *"continues losing money and eventually exceeds the illustrative $25,000 funding balance"* | UNO + platform |
| Q9 | What is the current verified credit price, and are exemptions, renewals and reassignment rules documented? | Launch gate 3; the $1.99 and $3.99 figures are planning inputs, not verified terms | Platform + billing |
| Q10 | Is there an authorised recovery path for a reservation abandoned mid-onboarding? | CLM-04, CLM-10. Affects how aggressively inventory can be recycled | UNO |
| Q11 | Which markets have a confirmed eligible withdrawal route? | UI-09 forbids promising cash-out without one; launch gate 2 requires it checked | UNO |
| Q12 | Does the portal need to serve a public preview route at all? | CMS-04. The audit rejected "public preview necessarily requires HMAC" in favour of scoped expiring tokens, but only if a preview route is actually needed | Product |

---

## 12. Non-goals

Stated explicitly so they are not treated as oversights.

| Non-goal | Rationale |
|---|---|
| Full rewrite | Audit §9: *"Avoid a full rewrite by default: the highest-value work is correcting shared contracts, transaction boundaries and access controls."* The repositories and HMAC and CMS work are real assets (A§12) |
| Building task supply, KYC or country eligibility | Audit §8.2: software distribution *"cannot create paid task demand, eliminate KYC or country restrictions, ensure stable participant connectivity, or turn a copied code into a profitable device"* |
| A participant earnings calculator promising a return | The plan withdrew its 39-day example and its $12,000 revenue forecast as unverified. The product shows *measured* accrual only |
| Reimplementing the local economic/capacity engine | Audit §14 adjudicated this proposition **Reject**: *"define upstream responsibilities and verified outcomes"* |
| Multi-region or read-replica infrastructure | Audit §7.4: introduce only on measured latency, availability or traffic evidence |
| ScyllaDB feature work | D1. Scylla becomes a read-only rollback source only |
| Device-farm, emulator or bought-account defences beyond logging | The plan rejects these channels outright; the product's job is to detect duplicate/fake outcomes for suspension review, not to build an anti-fraud industry (M§13) |
| A public admin surface | Audit §1: *"No, without authenticated ingress and application-level authorization"* |
| On-chain licence enforcement | Audit §10: *"The presence of wallet/crypto dependencies does not establish on-chain licence enforcement."* No such enforcement was established, and none is required here |
| An Android task client | Audit §10: *"A public Rust/WASM portal is not the Android task-execution client."* APK permissions and mobile task performance are out of scope for this repository |

---

## 13. Document control

| Item | Value |
|---|---|
| Supersedes | `UNO_APP_V2_REQUIREMENTS.md` v1.0 (29 September 2026) |
| Primary sources | `UN_APP_COMPREHENSIVE_TECHNICAL_AUDIT_REV2.md`, `UNETWORK_2500_LICENSE_MARKETING_PLAN.md`, `UNETWORK_REVENUE_CALCULATOR.html` |
| Supporting | `HIGHLEVEL_PLAI_VENDOR_DUE_DILIGENCE_SUPPLEMENT.md` (RFR-17) |
| Audited commit | `111ef5df214d85ed95fe71ff0113e2f39418a135` |
| Total P0 tasks | 150 |
| Total P1 tasks | 76 |
| Total P2 tasks | 10 |
| **Total tasks** | **236** |

### Task count by workstream

Counts below are **computed from the task rows**, not estimated. Every task ID appears exactly once (verified: no duplicates). Priority assignment is per-row; the WS9.2 honesty-and-compliance table (UI-11 to UI-17) carries no per-row priority column because its section heading declares all seven to be P0.

| Workstream | P0 | P1 | P2 | Total |
|---|---:|---:|---:|---:|
| WS0 Build & CI (BLD) | 11 | 3 | 0 | 14 |
| WS1 Schema & data model (DB) | 12 | 5 | 0 | 17 |
| WS2 Auth, authorization & secrets (SEC) | 15 | 4 | 0 | 19 |
| WS3 Storage on the Ember volume (FS) | 8 | 5 | 2 | 15 |
| WS4 Claim & lease lifecycle (CLM) | 12 | 3 | 0 | 15 |
| WS5 Agreement arithmetic (ECO) | 10 | 0 | 0 | 10 |
| WS5 Referral & attribution (RFR) | 10 | 7 | 0 | 17 |
| WS6 Publication integrity (PUB) | 7 | 0 | 0 | 7 |
| WS6 Sync & reconciliation (SYN) | 7 | 6 | 0 | 13 |
| WS7 Jobs, health & observability (JOB) | 9 | 5 | 0 | 14 |
| WS8 i18n (I18N) | 3 | 3 | 3 | 9 |
| WS8 CMS (CMS) | 3 | 4 | 0 | 7 |
| WS9 Product & user journey (UI) | 23 | 17 | 2 | 42 |
| WS10 Forecast engine (FC) | 9 | 11 | 3 | 23 |
| WS11 Data governance & gates (GOV) | 11 | 3 | 0 | 14 |
| **Total** | **150** | **76** | **10** | **236** |

---

## 14. Concrete Implementation Roadmap

This section maps requirements to specific files, modules, and implementation strategies based on comprehensive codebase analysis of all four components.

### 14.1 Codebase Inventory Summary

| Component | Location | Lines of Code | Primary Technology | Database |
|-----------|----------|---------------|-------------------|----------|
| **uno-admin** | `/uno-admin/` | ~55,425 | Leptos + Actix + ScyllaDB | ScyllaDB (13 migrations) |
| **uno-app** | `/uno-app/` | ~38,835 | Leptos + Actix + PostgreSQL | PostgreSQL (13 migrations) |
| **uno-api** | `/uno-api/` | ~5,710 | Pure Rust library | N/A |
| **file-storage** | `/file-storage/` | ~2,500 | Pure Rust library | N/A |

### 14.2 Phase 1: Contain Exposed Surfaces (Week 1)

**Goal:** Close security gaps identified in F01, F02, F15, F16.

#### 1.1 Authentication Middleware Implementation

**Target Files:**
- `uno-app/src/main.rs` (lines 64-122) — Add middleware to chain
- `uno-app/src/server/middleware/auth_middleware.rs` — Extend beyond Bearer token
- `uno-admin/src/main.rs` — No auth middleware exists; must add

**Implementation Strategy:**

```
SEC-01: Wire authentication middleware
├─ File: uno-app/src/main.rs:119-121
│  ├─ Current: Only VisitorTracker and RequestLogger are wrapped
│  └─ Change: Add AdminAuth middleware before API routes
├─ File: uno-admin/src/main.rs:69-111
│  ├─ Current: No middleware protection on any route
│  └─ Change: Create new auth_middleware.rs, wrap all /api/* routes
└─ Dependency: uno-api/src/auth/ (HMAC verification exists)
```

**Specific Changes:**

| Task | File | Line | Change |
|------|------|------|--------|
| SEC-01 | `uno-app/src/main.rs` | 119 | Add `.wrap(AdminAuthMiddleware::new())` after VisitorTracker |
| SEC-02 | `uno-app/src/routes/debug.rs` | 49,88,118 | Move behind feature flag or delete entirely |
| SEC-03 | `uno-app/src/main.rs` | 89 | Add CSRF middleware: `.wrap(CsrfMiddleware::new())` |
| SEC-04 | `uno-admin/src/ws/handler.rs` | * | Add session validation before WebSocket upgrade |

#### 1.2 Leptos Server Function Protection

**Critical Finding:** CMS server functions lack authentication (F15).

**Target Files:**
- `uno-app/src/api/cms_review.rs` — `publish_direct`, `publish_approved`, preview token creation
- `uno-app/src/api/licenses.rs` — `confirm_license_claim` (line 198)

**Implementation:**

```rust
// Before (cms_review.rs:15-30)
#[server(SubmitForReview, "/api/cms")]
pub async fn submit_for_review(
    version_id: i32,
    submitted_by: String,  // ← Caller-supplied, can be spoofed
    notes: Option<String>,
) -> Result<ContentReview, ServerFnError>

// After
#[server(SubmitForReview, "/api/cms")]
pub async fn submit_for_review(
    version_id: i32,
    notes: Option<String>,
) -> Result<ContentReview, ServerFnError> {
    let session = extract_authenticated_session().await?;  // NEW
    let submitted_by = session.principal_id;               // Derived, not supplied
    // ...
}
```

#### 1.3 Build-Time Secret Removal (F16)

**Target Files:**
- `uno-admin/src/api/config.rs` — Uses `option_env!("API_TOKEN")` and `option_env!("UNITY_JWT_TOKEN")`
- `uno-app/src/api/faq.rs` — Preview signing secret fallback

**Implementation:**

| Task | File | Current | Change |
|------|------|---------|--------|
| SEC-11 | `uno-admin/src/api/config.rs` | `option_env!("API_TOKEN")` compiled into WASM | Remove from browser builds; use runtime-only server config |
| SEC-12 | `uno-app/src/api/faq.rs` | Fallback to known preview secret | Fail closed when config absent |
| SEC-13 | Build artifacts | Unknown exposure | Audit with `strings` on release WASM |

---

### 14.3 Phase 2: Reproducible Build & Schema (Weeks 2-3)

**Goal:** Fresh checkout builds; schema supports all queries.

#### 2.1 Build System Fixes

**Critical Finding (N01):** `Cargo.lock` is gitignored but required by Dockerfile.

**Target Files:**
- `.gitignore` (line 8) — Remove `Cargo.lock`
- `uno-app/Dockerfile` (lines 26, 46) — `COPY Cargo.lock` will now work
- `uno-admin/Cargo.toml` (line 52) — External path dependencies

**Current External Dependencies (Must Remove):**
```toml
# uno-admin/Cargo.toml:52-56
ember-multichain = { path = "../../../../Dev-x/polkanight/ember/ember-multichain" }
ember-fx-components = { path = "../../../../Dev-x/polkanight/ember/ember-fx/crates/components" }
ember-fx-core = { path = "../../../../Dev-x/polkanight/ember/ember-fx/crates/core" }
ember-fx-icons = { path = "../../../../Dev-x/polkanight/ember/ember-fx/crates/icons" }
```

**Resolution Options:**
1. Vendor these crates into `deps/`
2. Publish to private registry
3. Use git dependencies with commit pinning

#### 2.2 CI/CD Pipeline Fixes

**Target Files:**
- `uno-app/.github/workflows/ci.yml` — Must move to repo root `.github/workflows/`
- `uno-app/.github/workflows/deploy.yml` — Same
- `uno-app/.github/workflows/infrastructure.yml` — Path filters don't match

**Current Issues (N06, N08):**
```yaml
# ci.yml:200 declares 7 jobs in needs:
needs: [check, fmt, clippy, test, build-ssr, build-wasm, docker]

# But lines 205-209 only evaluate 5:
- check
- fmt
- clippy
- test
# Missing: build-ssr, build-wasm, docker
```

**Fix:**
```yaml
# Move to: .github/workflows/ci.yml (repo root)
# Update all paths: working-directory: uno-app
# Fix needs array to match evaluated jobs
```

#### 2.3 Database Schema Reconciliation

**Critical Finding (N05):** Licenses schema has 1 of 10 required columns.

**Current Schema (`uno-app/migrations/00002_licenses.up.sql:8-33`):**
```sql
CREATE TABLE licenses (
    id VARCHAR(66) PRIMARY KEY,
    node_id VARCHAR(66),
    owner_wallet_address VARCHAR(42),
    -- ... 11 columns total
)
```

**Required Schema (from `license_repository.rs:34-35`):**
```sql
-- INSERT references these 10 columns:
id, lease_code, valid_from, valid_to, split_type,
claimed, bound_to_device, device_id, claimed_at, created_at
```

**Migration Strategy:**

| Task | Migration File | Action |
|------|----------------|--------|
| DB-01 | `00014_license_lifecycle.up.sql` | Add 9 missing columns + split_type enum |
| DB-03 | New enum | `CREATE TYPE split_type AS ENUM ('5050', '5545', '6040')` |
| DB-04 | `00015_rbac_complete.up.sql` | Add `user_roles`, `content_versions`, `content_reviews`, `preview_tokens` |
| DB-05 | Verify | Migration 00005 is missing; create placeholder or renumber |

**New Migration Content:**

```sql
-- 00014_license_lifecycle.up.sql
DO $$ BEGIN
    CREATE TYPE split_type AS ENUM ('5050', '5545', '6040');
EXCEPTION WHEN duplicate_object THEN null; END $$;

ALTER TABLE licenses
    ADD COLUMN IF NOT EXISTS lease_code VARCHAR(64) UNIQUE,
    ADD COLUMN IF NOT EXISTS valid_from TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS valid_to TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS split_type split_type DEFAULT '5050',
    ADD COLUMN IF NOT EXISTS claimed BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS bound_to_device BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS device_id VARCHAR(255),
    ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();

CREATE INDEX IF NOT EXISTS idx_licenses_unclaimed
    ON licenses(split_type) WHERE claimed = false;
```

---

### 14.4 Phase 3: Local Storage Migration (Week 4)

**Goal:** Migrate from GCS to local Ember volume storage.

#### 3.1 Path Traversal Fix (F10)

**Critical Vulnerability in `file-storage/src/backends/local/client.rs:45-47`:**

```rust
// CURRENT (VULNERABLE)
fn absolute_path(&self, relative_path: &str) -> PathBuf {
    PathBuf::from(&self.config.base_path).join(relative_path)
}

// ATTACK: relative_path = "../../../etc/passwd"
// RESULT: Escapes base_path containment
```

**Fix Implementation:**

```rust
// file-storage/src/backends/local/client.rs
use std::path::Path;

fn absolute_path(&self, relative_path: &str) -> Result<PathBuf, StorageError> {
    let base = Path::new(&self.config.base_path).canonicalize()?;

    // Reject path components that could escape
    if relative_path.contains("..") || relative_path.starts_with('/') {
        return Err(StorageError::InvalidPath("Path traversal attempt".into()));
    }

    let full = base.join(relative_path);
    let canonical = full.canonicalize().unwrap_or(full.clone());

    // Verify result is under base_path
    if !canonical.starts_with(&base) {
        return Err(StorageError::InvalidPath("Path escapes storage root".into()));
    }

    Ok(canonical)
}
```

#### 3.2 Storage Backend Configuration

**Target Files:**
- `file-storage/src/factory.rs` — Enable local backend by default
- `file-storage/Cargo.toml` — Change default feature to `local`
- `uno-app/Cargo.toml` — Update feature flag

**Environment Variables for Ember Volume:**

```bash
# Production config
FILE_STORAGE_BACKEND=local
FILE_STORAGE_LOCAL_PATH=/data/uploads  # Ember volume mount point
FILE_STORAGE_LOCAL_URL=/files          # Public URL prefix
```

#### 3.3 GCS Asset Migration

**Migration Script (`scripts/migrate_gcs_to_local.rs`):**

```rust
async fn migrate_assets() {
    let gcs_client = create_gcs_client().await?;
    let local_client = create_local_client().await?;

    // List all GCS objects
    let objects = gcs_client.list_all_objects().await?;

    for obj in objects {
        // Download from GCS
        let content = gcs_client.download(&obj.name).await?;

        // Upload to local
        local_client.upload(&obj.name, &content).await?;

        // Update database URL references
        update_storage_urls(&obj.storage_url, &local_storage_url).await?;
    }
}
```

---

### 14.5 Phase 4: Claim System Fixes (Weeks 5-6)

**Goal:** Exclusive reservations with immutable attribution.

#### 4.1 Atomic Reservation Transaction (F03)

**Current Issue (`uno-app/src/server/services/license_service.rs:47-65`):**

```rust
// CURRENT: Read-only, no reservation persistence
pub async fn reserve_by_split_type(&self, split_type: SplitType) -> Result<License, AppError> {
    self.repository.get_first_unclaimed(split_type).await  // Just reads!
}
```

**Fixed Implementation:**

```rust
// uno-app/src/server/services/license_service.rs
pub async fn reserve_by_split_type(
    &self,
    split_type: SplitType,
    session_token: &str,
    expires_in: Duration,
) -> Result<Reservation, AppError> {
    let mut tx = self.pool.begin().await?;

    // SELECT ... FOR UPDATE SKIP LOCKED within transaction
    let license = sqlx::query_as!(
        License,
        r#"
        SELECT * FROM licenses
        WHERE split_type = $1
          AND claimed = false
          AND reserved_until < NOW()
        FOR UPDATE SKIP LOCKED
        LIMIT 1
        "#,
        split_type.to_db_str()
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NoAvailableLicenses)?;

    // Create reservation record
    let reservation = sqlx::query_as!(
        Reservation,
        r#"
        INSERT INTO reservations (license_id, session_token, expires_at)
        VALUES ($1, $2, NOW() + $3)
        RETURNING *
        "#,
        license.id,
        session_token,
        expires_in
    )
    .fetch_one(&mut *tx)
    .await?;

    // Update license reserved_until
    sqlx::query!(
        "UPDATE licenses SET reserved_until = $1 WHERE id = $2",
        reservation.expires_at,
        license.id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(reservation)
}
```

#### 4.2 New Reservations Table

**Migration (`00016_reservations.up.sql`):**

```sql
CREATE TABLE reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    license_id VARCHAR(66) REFERENCES licenses(id) ON DELETE CASCADE,
    session_token VARCHAR(64) NOT NULL,
    claimant_ref VARCHAR(255),
    reserved_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    released_at TIMESTAMPTZ,
    confirmed_at TIMESTAMPTZ,
    UNIQUE(license_id, session_token)
);

CREATE INDEX idx_reservations_expires ON reservations(expires_at)
    WHERE released_at IS NULL AND confirmed_at IS NULL;

-- Add column to licenses
ALTER TABLE licenses ADD COLUMN reserved_until TIMESTAMPTZ;
```

#### 4.3 Immutable Referral Attribution (F04)

**Current Issue (`uno-app/src/server/repositories/claim_repository.rs`):**

```rust
// CURRENT: No first-write-only check
async fn set_referral(&self, license_id: &str, referral_id: i32) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE licenses SET referral_id = $1 WHERE id = $2",
        referral_id, license_id
    ).execute(&self.pool).await?;
    Ok(())
}
```

**Fixed Implementation:**

```rust
async fn set_referral(
    &self,
    license_id: &str,
    referral_id: i32,
    agreement_version: i32,
) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE licenses
        SET referral_id = $1,
            referral_agreement_version = $2,
            referral_attributed_at = NOW()
        WHERE id = $3
          AND referral_id IS NULL  -- First-write-only
        "#,
        referral_id, agreement_version, license_id
    )
    .execute(&self.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::ReferralAlreadyAttributed);
    }
    Ok(())
}
```

---

### 14.6 Phase 5: HMAC Replay Protection (Week 7)

**Goal:** Complete HMAC security with nonce registry.

#### 5.1 Current Gap Analysis (F09, N10)

**File: `uno-api/src/auth/hmac.rs:40-63`**

```rust
// CURRENT: Accepts future timestamps, never stores nonce
let age = (now - request.timestamp).abs();  // .abs() allows FUTURE!
if age > max_age_secs {
    return Err(AuthError::TimestampExpired);
}
// Nonce is verified in signature but never stored → replay possible
```

#### 5.2 Nonce Registry Implementation

**New File: `uno-api/src/auth/nonce_registry.rs`**

```rust
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

pub struct NonceRegistry {
    entries: RwLock<HashMap<(String, String), Instant>>,  // (client_id, nonce) → timestamp
    max_age: Duration,
}

impl NonceRegistry {
    pub fn new(max_age_secs: u64) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            max_age: Duration::from_secs(max_age_secs),
        }
    }

    /// Returns true if nonce is fresh (not seen before), false if replay
    pub fn check_and_register(&self, client_id: &str, nonce: &str) -> bool {
        let key = (client_id.to_string(), nonce.to_string());
        let now = Instant::now();

        // Cleanup old entries
        {
            let mut entries = self.entries.write().unwrap();
            entries.retain(|_, ts| now.duration_since(*ts) < self.max_age);
        }

        // Check and register
        let mut entries = self.entries.write().unwrap();
        if entries.contains_key(&key) {
            false  // Replay detected
        } else {
            entries.insert(key, now);
            true  // Fresh nonce
        }
    }
}
```

#### 5.3 Updated Verification Function

**File: `uno-api/src/auth/hmac.rs`**

```rust
pub fn verify_signature_with_replay_protection(
    request: &SignedRequest<impl Serialize>,
    secret: &[u8],
    max_age_secs: i64,
    nonce_registry: &NonceRegistry,
) -> Result<(), AuthError> {
    let now = chrono::Utc::now().timestamp();

    // Reject FUTURE timestamps (remove .abs())
    let age = now - request.timestamp;
    if age < 0 {
        return Err(AuthError::TimestampInFuture);
    }
    if age > max_age_secs {
        return Err(AuthError::TimestampExpired);
    }

    // Check nonce uniqueness
    if !nonce_registry.check_and_register(&request.client_id, &request.nonce) {
        return Err(AuthError::NonceReused);
    }

    // Verify signature (existing logic)
    verify_signature(request, secret)
}
```

---

### 14.7 Phase 6: Economic Model & Split System (Week 8)

**Goal:** Implement 50/40/10 with basis points.

#### 6.1 New Split Model

**Replace `uno-api/src/models/license.rs:25-82`:**

```rust
/// Revenue split in basis points (1/100th of a percent)
/// Invariant: ulo_bps + uno_bps + referral_bps = 10000
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSplit {
    pub ulo_bps: u16,       // e.g., 5000 = 50%
    pub uno_bps: u16,       // e.g., 4000 = 40%
    pub referral_bps: u16,  // e.g., 1000 = 10%
    pub agreement_version: i32,
}

impl RevenueSplit {
    pub const SPLIT_50_40_10: Self = Self {
        ulo_bps: 5000,
        uno_bps: 4000,
        referral_bps: 1000,
        agreement_version: 1,
    };

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.ulo_bps + self.uno_bps + self.referral_bps != 10000 {
            return Err("Split must total 10000 basis points");
        }
        if self.ulo_bps > 10000 || self.uno_bps > 10000 || self.referral_bps > 10000 {
            return Err("Individual share cannot exceed 10000 bps");
        }
        Ok(())
    }

    /// Calculate allocation from pool amount (in micros)
    pub fn allocate(&self, pool_micros: i64) -> AllocationResult {
        let ulo = (pool_micros * self.ulo_bps as i64) / 10000;
        let uno = (pool_micros * self.uno_bps as i64) / 10000;
        let referral = (pool_micros * self.referral_bps as i64) / 10000;
        let rounding = pool_micros - ulo - uno - referral;

        AllocationResult {
            pool_micros,
            ulo_micros: ulo,
            uno_micros: uno,
            referral_micros: referral,
            rounding_micros: rounding,  // Always assign to UNO
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllocationResult {
    pub pool_micros: i64,
    pub ulo_micros: i64,
    pub uno_micros: i64,
    pub referral_micros: i64,
    pub rounding_micros: i64,
}
```

#### 6.2 Database Schema for Splits

**Migration (`00017_agreement_splits.up.sql`):**

```sql
CREATE TABLE agreement_versions (
    version INT PRIMARY KEY,
    ulo_bps SMALLINT NOT NULL CHECK (ulo_bps >= 0 AND ulo_bps <= 10000),
    uno_bps SMALLINT NOT NULL CHECK (uno_bps >= 0 AND uno_bps <= 10000),
    referral_bps SMALLINT NOT NULL CHECK (referral_bps >= 0 AND referral_bps <= 10000),
    effective_from TIMESTAMPTZ NOT NULL,
    effective_to TIMESTAMPTZ,
    pool_definition TEXT NOT NULL,  -- "before_deductions" or "after_deductions"
    created_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT bps_sum_check CHECK (ulo_bps + uno_bps + referral_bps = 10000)
);

-- Insert the canonical 50/40/10 split
INSERT INTO agreement_versions (version, ulo_bps, uno_bps, referral_bps, effective_from, pool_definition)
VALUES (1, 5000, 4000, 1000, NOW(), 'after_deductions');

-- Add agreement reference to licenses
ALTER TABLE licenses ADD COLUMN agreement_version INT REFERENCES agreement_versions(version);
```

---

### 14.8 Phase 7: UI/UX Journey Implementation (Weeks 9-10)

**Goal:** Complete user journey from landing to D30.

#### 7.1 Wizard State Machine Enhancement

**File: `uno-app/src/components/wizard/state.rs`**

```rust
/// Enhanced wizard stages matching the marketing plan journey
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WizardStage {
    // Existing stages
    Eligibility,      // Was "Signup"
    EconomicsReview,  // NEW: Show 50/40/10 split
    Availability,
    Reserve,          // NEW: Atomic reservation
    Confirm,          // Was "Claim"
    Download,
    Activate,
    Review,
    WhatNext,

    // New touchpoint stages
    D1Support,
    D3TaskCheck,
    D7Review,
    WithdrawalGuide,
    D30Renewal,
}

impl WizardStage {
    /// Stages that require license reservation
    pub fn requires_reservation(&self) -> bool {
        matches!(self, Self::Reserve | Self::Confirm | Self::Download | Self::Activate)
    }

    /// Marketing plan 10-step journey
    pub fn journey_step(&self) -> u8 {
        match self {
            Self::Eligibility => 1,
            Self::EconomicsReview => 2,
            Self::Availability => 3,
            Self::Reserve => 4,
            Self::Confirm => 5,
            Self::Download => 6,
            Self::Activate => 7,
            Self::Review => 8,
            Self::D1Support => 9,
            Self::WhatNext => 10,
            _ => 0,  // Post-journey touchpoints
        }
    }
}
```

#### 7.2 50/40/10 Split Calculator Component

**New File: `uno-app/src/components/economics/split_calculator.rs`**

```rust
use leptos::*;

#[component]
pub fn SplitCalculator(
    #[prop(into)] pool_amount: Signal<f64>,
) -> impl IntoView {
    let ulo_share = move || pool_amount.get() * 0.50;
    let uno_share = move || pool_amount.get() * 0.40;
    let referral_share = move || pool_amount.get() * 0.10;

    view! {
        <div class="split-calculator">
            <h3>{t!("economics.split_title")}</h3>
            <div class="split-breakdown">
                <div class="split-item ulo">
                    <span class="label">{t!("economics.you_receive")}</span>
                    <span class="percentage">"50%"</span>
                    <span class="amount">{move || format!("${:.2}", ulo_share())}</span>
                </div>
                <div class="split-item uno">
                    <span class="label">{t!("economics.uno_receives")}</span>
                    <span class="percentage">"40%"</span>
                    <span class="amount">{move || format!("${:.2}", uno_share())}</span>
                </div>
                <div class="split-item referral">
                    <span class="label">{t!("economics.referral_receives")}</span>
                    <span class="percentage">"10%"</span>
                    <span class="amount">{move || format!("${:.2}", referral_share())}</span>
                </div>
            </div>
            <p class="credits-note">
                {t!("economics.credits_paid_by_uno")}
            </p>
        </div>
    }
}
```

---

### 14.9 Phase 8: Sync & Reconciliation (Week 11)

**Goal:** Complete pagination, checkpoint persistence, job durability.

#### 8.1 Cursor-Based Pagination for Claims

**File: `uno-app/src/server/repositories/claim_repository.rs`**

```rust
pub struct ClaimCursor {
    pub claimed_at: DateTime<Utc>,
    pub id: String,
}

pub async fn get_claimed_since_cursor(
    &self,
    cursor: Option<ClaimCursor>,
    limit: i32,
) -> Result<(Vec<ClaimedLicense>, Option<ClaimCursor>), AppError> {
    let claims = match cursor {
        Some(c) => {
            sqlx::query_as!(
                ClaimedLicense,
                r#"
                SELECT * FROM licenses
                WHERE claimed = true
                  AND (claimed_at, id) > ($1, $2)
                ORDER BY claimed_at ASC, id ASC
                LIMIT $3
                "#,
                c.claimed_at, c.id, limit
            )
            .fetch_all(&self.pool)
            .await?
        }
        None => {
            sqlx::query_as!(
                ClaimedLicense,
                r#"
                SELECT * FROM licenses
                WHERE claimed = true
                ORDER BY claimed_at ASC, id ASC
                LIMIT $1
                "#,
                limit
            )
            .fetch_all(&self.pool)
            .await?
        }
    };

    let next_cursor = claims.last().map(|c| ClaimCursor {
        claimed_at: c.claimed_at,
        id: c.id.clone(),
    });

    Ok((claims, next_cursor))
}
```

#### 8.2 Durable Job Queue

**New File: `uno-admin/src/logic/durable_job_queue.rs`**

```rust
/// Persistent job queue backed by PostgreSQL
pub struct DurableJobQueue {
    pool: PgPool,
}

impl DurableJobQueue {
    pub async fn enqueue(&self, job: Job) -> Result<String, DbError> {
        let job_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO job_queue (id, job_type, payload, status, created_at)
            VALUES ($1, $2, $3, 'pending', NOW())
            "#,
            job_id, job.job_type, serde_json::to_value(&job.payload)?
        )
        .execute(&self.pool)
        .await?;

        Ok(job_id)
    }

    pub async fn dequeue(&self, worker_id: &str) -> Result<Option<Job>, DbError> {
        // Acquire lease on next pending job
        sqlx::query_as!(
            Job,
            r#"
            UPDATE job_queue
            SET status = 'running',
                worker_id = $1,
                started_at = NOW(),
                lease_expires_at = NOW() + INTERVAL '5 minutes'
            WHERE id = (
                SELECT id FROM job_queue
                WHERE status = 'pending'
                   OR (status = 'running' AND lease_expires_at < NOW())
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING *
            "#,
            worker_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn complete(&self, job_id: &str, result: JobResult) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE job_queue
            SET status = 'completed',
                completed_at = NOW(),
                result = $2
            WHERE id = $1
            "#,
            job_id, serde_json::to_value(&result)?
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
```

---

### 14.10 Implementation Complexity Matrix

| Task Group | Files Modified | New Files | Est. Complexity | Dependencies |
|------------|----------------|-----------|-----------------|--------------|
| **SEC-01 to SEC-10** | 8 | 2 | Medium | uno-api auth |
| **BLD-01 to BLD-07** | 6 | 0 | Low | None |
| **DB-01 to DB-09** | 2 | 5 migrations | High | None |
| **FS-01 to FS-07** | 3 | 1 | Medium | None |
| **CLM-01 to CLM-12** | 4 | 2 migrations | High | DB tasks |
| **ECO-01 to ECO-08** | 3 | 1 migration | Medium | None |
| **SYN-01 to SYN-09** | 5 | 2 | High | DB, CLM tasks |
| **UI-01 to UI-15** | 12 | 6 | High | ECO tasks |
| **JOB-01 to JOB-04** | 3 | 2 | Medium | DB tasks |

---

### 14.11 Critical Path Dependencies

```
Phase 1 (Contain)
    └── SEC-01 to SEC-13 (auth/secrets)
           │
Phase 2 (Reproduce)
    ├── BLD-01 to BLD-07 (build fixes)
    └── DB-01 to DB-09 (schema)
           │
           ├─────────────────────┐
           │                     │
Phase 3 (Storage)           Phase 4 (Claims)
    └── FS-01 to FS-07      └── CLM-01 to CLM-12
           │                     │
           └─────────┬───────────┘
                     │
Phase 5 (Economics)
    └── ECO-01 to ECO-08
           │
           ├───────────────────────┐
           │                       │
Phase 6 (Sync)                Phase 7 (UI)
    └── SYN-01 to SYN-09      └── UI-01 to UI-15
           │                       │
           └───────────┬───────────┘
                       │
Phase 8 (Jobs)
    └── JOB-01 to JOB-04
```

---

### 14.12 Testing Strategy

| Test Category | Framework | Target Coverage | Location |
|---------------|-----------|-----------------|----------|
| Unit tests | `cargo test` | 80% of business logic | `*/tests/*.rs` |
| Integration tests | `sqlx-test` | All repository methods | `uno-app/tests/` |
| Concurrent claim tests | `tokio::test` | CLM-01 to CLM-08 acceptance | `uno-app/tests/concurrency_tests.rs` |
| HMAC replay tests | `cargo test` | SYN-10, SYN-11 | `uno-api/tests/auth_tests.rs` |
| E2E tests | Playwright | Full wizard journey | `uno-app/end2end/` |
| Migration tests | `sqlx migrate` | Fresh install + upgrade | CI pipeline |

---

### 14.13 Rollback Plan

| Phase | Rollback Trigger | Rollback Action |
|-------|------------------|-----------------|
| Phase 1 | Auth breaks admin access | Revert middleware commits; use IP allowlist |
| Phase 2 | Migration failures | `sqlx migrate revert`; restore DB backup |
| Phase 3 | Storage corruption | Switch back to GCS via env var |
| Phase 4 | Claim system failures | Feature flag disable; manual claim processing |
| Phase 5 | Split calculation errors | Use hardcoded 50/40/10 constants |
| Phase 6 | Sync job failures | Manual reconciliation scripts |
| Phase 7 | UI regression | Feature flag to old wizard |

---

### 14.14 Monitoring & Observability

**New Metrics to Add:**

| Metric | Type | Purpose |
|--------|------|---------|
| `claim_reservation_duration_seconds` | Histogram | Track reservation latency |
| `claim_concurrent_attempts` | Gauge | Detect contention |
| `nonce_replay_attempts_total` | Counter | Security monitoring |
| `sync_reconciliation_discrepancies` | Counter | Data integrity |
| `job_queue_depth` | Gauge | Backlog monitoring |
| `storage_write_errors_total` | Counter | Storage health |

**Structured Logging Fields:**

```rust
tracing::info!(
    license_id = %license.id,
    session_token = %session_token,
    reservation_id = %reservation.id,
    split_type = %split_type,
    "License reserved successfully"
);
```

---

### 14.15 Definition of Done Checklist

For each phase:

- [ ] All P0 tasks for the phase completed
- [ ] Unit tests pass with >80% coverage
- [ ] Integration tests pass
- [ ] No `cargo clippy` warnings
- [ ] `cargo audit` shows no vulnerabilities
- [ ] Migration runs on fresh database
- [ ] Migration runs on existing database (upgrade path)
- [ ] Feature works in production-like environment
- [ ] Documentation updated
- [ ] Code review approved
- [ ] Performance benchmarks acceptable
