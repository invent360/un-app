# Un-app: license distribution feasibility

**Prepared:** 29 September 2026
**Repository:** https://github.com/invent360/un-app
**Snapshot:** `111ef5df214d85ed95fe71ff0113e2f39418a135` — "Fix uno-admin: convert from submodule to regular directory"
**Companion document:** `UN_APP_COMPREHENSIVE_TECHNICAL_AUDIT.md` (broader code audit; this document is the decision-focused, evidence-verified companion)

**Question answered:** Can this system support automatic distribution of 2,500 licences and 250 productive additions per week?

**Short answer:** No — not from this repository, and not without fixing a defect that disables the claim path outright. Three findings drive that conclusion:

1. **The earning engine is not in this repository.** What is here is a marketing portal, a licence console, a shared client library, and a file-storage abstraction. Any assessment of earning throughput or operator retention has to be grounded in a system we cannot read.
2. **The claim path cannot execute.** `license_repository.rs` reads and writes 7 columns that no migration creates, and casts to a `split_type` enum that is never defined.
3. **80% of all distributed credits are Proof of Work**, not the connectivity and verification work the product is marketed on. The remainder averages **0.95 UP/day network-wide**, and the mobile-relevant task averages **0.069 UP/day**. Output is currently running at ~15% of its March 2026 peak.

Where this document quotes throughput, it comes from 20,189 committed reward events and a 475-row incentive export — not from executable code.

---

## 1. Corrections to earlier statements in this engagement

Stated for the record, because two earlier claims did not survive verification:

| Earlier claim | Verified finding |
|---|---|
| "9 of 10 license columns referenced by the repository are missing from the migration" | **7 of 10** are missing. `claimed_at` and `created_at` do exist. Precise set diff in §3.2. |
| "No KYC anywhere" | **False as a statement about the product.** KYC exists and ships — via **Sumsub**, running natively in the mobile app ("matching the existing iOS experience", Play Store release notes, 23 Sep 2026). It is absent from *this repository*, which is a different claim. |
| "No native mobile app exists" | **False as a statement about the product.** Both apps are live and shipping (§2.2). Correct statement: no mobile source is in this repository. |

The KYC and mobile-app corrections matter materially. An audit of this repository cannot be used to claim the product lacks features that live in a separate closed codebase.

---

## 2. What the repository actually is

### 2.1 The four components

| Component | Kind | Stack | Role |
|---|---|---|---|
| `uno-app` | Application | Leptos 0.8 SSR/WASM, Actix, SQLx, Postgres | Public marketing site + licence claim portal |
| `uno-api` | Library | Rust, `async-trait`, HMAC | DTOs, client traits, request signing. **Not a server.** |
| `uno-admin` | Application | Leptos SSR/WASM, Actix, Scylla | Internal operator console |
| `file-storage` | Library | Rust, GCS + local | Pluggable upload backend |

Two duplication problems, both confirmed by `diff -r`:

- `uno-api` and `uno-app/deps/uno-api` are **byte-identical**.
- `file-storage` and `uno-app/deps/file-storage` are **byte-identical**.

Neither is a workspace member. Divergence is a matter of time, and any fix to a shared library must currently be applied twice.

### 2.2 The earning engine is external — the central finding

Searching the whole repository for the identifiers that appear in the reward ledger, excluding the `data/` directories:

| Identifier | Implementations in repo |
|---|---|
| `amountMicros` | **0** |
| `license_operator` | **0** |
| `scout_report` | **0** |
| `snr-scout-test` | **0** |
| `cli_task` | **0** |
| `license_owner` | 1 — a display label in `uno-admin/src/ui/pages/jobs/list.rs` |
| `taskMetadata` | 1 — `uno-admin/src/db/seeder.rs` |

The reward ledger's schema, its payout computation, its task taxonomy, and its task dispatch **do not exist in this codebase.** The only reference is a seeded, read-only dataset and a UI string. Whatever mints a reward event, decides who is credited, and settles a split lives in a closed service.

The client applications are likewise separate and live:

- **Android** — "Unetwork App", `io.unetwork.app`, Unity Network, **10K+ downloads, 5.0★, 163 reviews**, updated 23 Sep 2026.
- **iOS** — "Unetwork App", Unity Network Limited, v1.2.4, released 23 Sep 2026, **4.49★, 47 ratings**, iOS 15.1+.
- Tasks visible in release notes: extended telemetry, task controls, **Sumsub KYC**, verification flows.
- Store description confirms: *"connectivity and diagnostic tool that allows approved devices to connect to Unetwork and participate in supported verification tasks."*
- `releases.unetwork.io/android/` returns HTTP 200. `unitynodes.io/terms` and the KYC attestation page both resolve.

**Consequence for planning:** this repository cannot be used to model earning capacity. It models a distribution front end and an operator console. Any per-device earnings forecast must come from the closed service, from the apps, or from a measurement campaign — not from this code.

---

## 3. Blocking defect: the claim path cannot run against a fresh database

### 3.1 What the code does

`uno-app/src/server/repositories/license_repository.rs` is the only repository that touches licences, and every query targets the `licenses` table. Its canonical column list, repeated in the INSERT (line 34) and every SELECT (lines 79, 91, 105, 129, 157, 193, …), is:

```
id, lease_code, valid_from, valid_to, split_type, claimed,
bound_to_device, device_id, claimed_at, created_at
```

### 3.2 Precise set diff against the migration

`uno-app/migrations/00002_licenses.up.sql` declares 34 columns. Set difference against the 10 the repository uses:

| Column | In migration? |
|---|---|
| `id` | yes |
| `claimed_at` | yes |
| `created_at` | yes |
| `lease_code` | **no** |
| `valid_from` | **no** |
| `valid_to` | **no** |
| `split_type` | **no** |
| `claimed` | **no** |
| `bound_to_device` | **no** |
| `device_id` | **no** |

**7 of 10 referenced columns do not exist.** Additionally:

- The PostgreSQL enum type `split_type` is **never created** in any of the 26 migration files. The casts `$5::split_type` (line 35), `split_type::text` (line 79 and elsewhere) and `WHERE split_type = $1::split_type` (lines 107, 159) all require a type that no migration provides. Adding the column alone would still fail.
- The migration's actual model is entirely different: `user_share_percentage`, `operator_share_percentage`, `lease_share_percentage`, `lease_from`, `lease_to`, `status`, `is_leased`, `uptime`, `claim_token`, `referral_code`. It has no lease code, no claim flag, no device binding, and no validity window under the names the code uses.

This is not a corner case. `get_first_unclaimed` (line 101) is the query behind "give me a licence to hand to the next visitor." It cannot execute. **The public claim flow is non-functional on any database built from the committed migrations.**

Two possible explanations, both requiring resolution before any launch:
1. Production has manual schema drift — someone applied unversioned DDL. Then the migrations are untrustworthy as a deployment artifact and every environment is unreproducible.
2. The repository is mid-refactor and the feature is genuinely down.

Either way, this must be resolved before distributing anything, because a partially-working claim path that silently fails is worse for a licensing programme than a visibly broken one.

---

## 4. The split model does not support a 50/40/10 programme

`uno-api/src/models/license.rs:27-38`:

```rust
pub enum SplitType {
    #[default] #[serde(rename = "50:50")] Split5050,
    #[serde(rename = "55:45")] Split5545,
    #[serde(rename = "60:40")] Split6040,
}
```

Three variants, all two-way. There is no three-way split, no referral leg, and no `50/40/10` anywhere in `uno-api` or `uno-app` (searched: `50, 40, 10`, `50_40_10`, `ULO_SHARE`, `ulo_share` — no matches).

The three-way concept exists **only in the admin console**: `uno-admin/src/models/license.rs:5-14` defines `LeaseSplit { uno_share, agent_share, ulo_share }`. And `uno-admin/src/logic/marketplace_service.rs` collapses that three-way split into a two-way `SplitType` at publication time.

So a 50/40/10 economics has **no home in the data model** that the public portal reads. Introducing it requires changing the `split_type` enum, the migration (which must be created anyway, §3), the claim repository, and the admin publish path. It is not a configuration change.

---

## 5. Reward data: what 20,189 committed events actually show

Source: `uno-admin/data/incentives/rewards.json` plus 24 files in `uno-admin/data/rewards/`. **25 files, 20,189 reward events.**

### 5.1 Volume

Window **2025-11-30 → 2026-06-06** = 188 days.
**107.4 events/day, 752 events/week.** Total value **1,706.223286 units** (1,706,223,286 `amountMicros`).

| Month | Events | Value (units) |
|---|---|---|
| 2025-11 | 38 | 1.30 |
| 2025-12 | 1,952 | 98.96 |
| 2026-01 | 2,614 | 323.86 |
| 2026-02 | 2,984 | 309.60 |
| 2026-03 | **5,876** | 423.40 |
| 2026-04 | 4,497 | 323.01 |
| 2026-05 | 1,986 | 198.31 |
| 2026-06 (partial, 6 days) | 242 | 27.79 |

Volume grew ~155× from November to the March peak, then **fell 66% from March to May**, and is declining further. The dataset ends in a downturn.

### 5.2 Attribution — the decisive problem

- **All 20,189 events are credited to a single account**, `c49872f6-db20-4794-a40d-5d495d9693fa`. One account holds 100% of recorded value.
- Only **484 events (2.40%)** carry a `taskMetadata` block. **19,705 events (97.60%)** have none — 1,581.14 units of value that cannot be attributed to any human.
- Those 484 events map to **exactly four distinct human operators**:

| Operator | Events | Value (units) |
|---|---|---|
| `1ec99689-…` | 411 | 96.50 |
| `0db060ac-…` | 5 | 12.50 |
| `a01536c5-…` | 65 | 8.58 |
| `629155ce-…` | 3 | 7.50 |
| **Total** | **484** | **125.08** |

**Four humans in six months.** A single operator (411 of 484 events) accounts for 77% of attributable value.

### 5.3 The 50:50 split has never materially executed

| `type` | Events | Share of events | Value | Share of value |
|---|---|---|---|---|
| `license_owner` | 20,184 | 99.9752% | 1,693.7233 u | 99.2674% |
| `license_operator` | 5 | 0.0248% | 12.5000 u | 0.7326% |

Five operator events in the entire history. The user-facing proposition — that a claim splits value between the participant and the platform — is **not what the data shows.** 99.3% of value flowed to `license_owner`.

### 5.4 Concentration and per-event economics

- **168 distinct licences** across 20,189 events. Median 107 events per licence; max 636; five licences account for 1,973 events.
- Per event: min **0**, p50 **55,842** (0.0558 u), mean **84,513** (0.0845 u), max **2,500,000** (2.5 u), across 10,576 distinct values. Payouts are computed, not fixed-rate.
- Only **5 distinct `taskKey` values** exist, and **19,593 events (97%) have `taskKey = null`**. Attributable task types: `snr-scout-test` (444), `cli_task` (112), `snr-mapquest` (20), `snr-scoutquest` (20).

The null-heavy shape means the bulk of value is **not** obviously task-completion income. It is consistent with lease or uptime-derived accrual, which would be the "passive income" model — but that cannot be confirmed without the closed service.

### 5.5 Privacy

`taskMetadata` and node records contain real-looking user IDs, node hashes, licence IDs, and mobile carrier/package data: Ghanaian carriers, Tango, Airtel, AirtelTigo, MTN, MTN Zambia, Digicel. This is production-derived personal data committed to a public repository. It warrants review under GDPR/UK GDPR and Kenya's DPA, independent of the licensing question.

---

## 6. The incentives CSV: 80% of all credits are Proof of Work

The committed ledger ends 2026-06-06. The incentive export `statistics-incentives-All.csv` runs 302 days to 2026-09-28 and is the fresher dataset. It changes the conclusion about what participants actually earn.

**475 records, 303 dates, 1,442.46 UP total = 4.78 UP/day network-wide.**

| Task | Rows | UP | Share |
|---|---|---|---|
| **Proof of Work** | 302 | **1,155.70** | **80.12%** |
| Scout | 27 | 190.09 | 13.18% |
| Other | 9 | 75.00 | 5.20% |
| CLI / Caller-ID | 135 | 20.87 | 1.45% |
| Runner Calls | 2 | 0.81 | 0.06% |

Proof of Work has a row on **every single one of the 302 days** — continuous, always-on compute. It is not a connectivity or diagnostic task. The store describes the product as *"a connectivity and diagnostic tool that allows approved devices to connect to Unetwork and participate in supported verification tasks."* The credit ledger does not corroborate that framing: **four fifths of the economics is proof-of-work mining.**

Strip it out and the actual verification-task economy is:

- **All non-PoW work combined: 286.77 UP over 302 days = 0.95 UP/day, network-wide.**
- **CLI / Caller-ID — the one task that genuinely looks like the mobile proposition — earns 20.87 UP over the entire 302-day history: 0.069 UP/day for the whole network.**

### 6.1 The decline is real and accelerating

| Month | UP | Change |
|---|---|---|
| 2025-12 | 49.48 | — |
| 2026-01 | 161.93 | +227.3% |
| 2026-02 | 154.80 | −4.4% |
| 2026-03 | **211.70** | +36.8% |
| 2026-04 | 198.76 | −6.1% |
| 2026-05 | 198.31 | −0.2% |
| 2026-06 | 139.59 | −29.6% |
| 2026-07 | 249.03 | +78.4% |
| 2026-08 | **46.59** | **−81.3%** |
| 2026-09 (to 28th) | **31.63** | −32.1% |

**August and September 2026 are running at roughly 20% and 15% of the March peak.** July's spike did not hold. Proof of Work specifically fell from 3.83 UP/day across the full window to 1.54 UP/day in the last 90 days — **−60%**. In the last 90 days Scout became the largest earner (177.55 UP) and Proof of Work fell to second (138.74 UP).

### 6.2 Implication for the "$0.10/day/device" model

Any per-device revenue figure derived from the **blended** 4.78 UP/day average is dominated by proof-of-work hashing. A participant cannot reproduce that on a phone running the app; PoW requires sustained compute, and the app's own store copy never mentions it. Per-device figures for the connectivity/verification proposition must therefore be built from the **non-PoW** series — where the network-wide ceiling is 0.95 UP/day and the relevant mobile task averages 0.069 UP/day.

This is the single largest modelling error available in this dataset, and it is easy to commit because the CSV does not distinguish the two without task-level analysis.

---

## 7. Capacity: the 250/week target against observed behaviour

The two numbers must not be conflated. **Reward events** and **productive operator additions** are different quantities, and the target is the second.

| Metric | Observed | Target | Gap |
|---|---|---|---|
| Reward events / week | ~752 | — | n/a |
| Attributable human operators | **4 in 188 days** | 250 / week | **≈ 60× short** |
| New operators / month | **~0.7** | 1,000 | **≈ 1,400× short** |
| Active licences | 168 | 2,500 | 15× growth, unproven |

A 250/week target requires 1,000 new productive operators per month. The observed rate of *attributable* new human operators is roughly one every six weeks. Even if the 19,705 unattributed events each represent a distinct untracked operator — which the data cannot support, since 168 licences and a median of 107 events each is consistent with a small number of devices running continuously — the ceiling implied is on the order of tens, not thousands.

**The bottleneck is operator acquisition and activation, not reward-event throughput.** The system demonstrably processes volume; it has never demonstrated it can onboard people at scale. Store-side evidence corroborates this: on a 10K+ download base, recent reviews report *"the incentive is stuck, not a single incentive added"*, *"3 days and zero incentive added"*, and *"support queries that take a month or more to get to"*. Downloads are not converting to earnings.

**Recommendation:** do not treat 250/week as a marketing target against the current system. Treat it as an activation-funnel problem requiring measurement. Instrument install → KYC → licence claim → first task → first reward, per device, and establish the real conversion rate before committing to volume.

---

## 8. Marketing claims versus what the code and data support

| Claim | Location | Reality |
|---|---|---|
| "a connectivity and diagnostic tool… participate in supported verification tasks" | Play Store / App Store copy | **80.12%** of all credits are Proof of Work (§6). The credit ledger does not corroborate the framing. |
| "Expected Monthly Earnings: **$5 - $15**" | `uno-app/src/locales/en.rs:221-222` | Marketing copy, hardcoded. Blended network rate is 4.78 UP/day **including** PoW. Excluding PoW: 0.95 UP/day network-wide. |
| "users earn between **$3-15 per month** per device" | `uno-app/seeds/faq_data.sql:9` | A $3/month device rate is $0.10/day. The network produces $0.069/day from CLI/Caller-ID in total. Unsustainable at any device count above a handful. |
| "Start earning passive income!" | `en.rs:213` | The claim path does not execute (§3). |
| Licence price **$1.99** | — | No occurrence in executable code. Archived/marketing documents only. |
| **Withdrawal at a $5 minimum** | guide seed content only | No threshold constant, no payout rail, no settlement job anywhere in the repository. Not implemented in this codebase. |
| 50:50 participant/platform split | `en.rs` wizard copy | 5 operator events ever; 99.3% of value to `license_owner` (§5.3). |
| Sustainable recurring income | marketing plan assumption | Network output fell **−81%** in August and **−32%** in September 2026 versus the March peak (§6.1). PoW specifically −60% over 90 days. |

Two compounding problems: the claim flow is broken, and the per-device earnings figure that all of this copy rests on cannot be derived from anything in this repository — and where a blended figure can be derived, it is dominated by a proof-of-work workload the product never mentions.

---

## 9. Security

Full detail in the companion audit; `file-storage` findings are new and severe.

### Critical

1. **Unauthenticated upload and delete in `uno-app`.** `/api/admin/files/upload` and `DELETE /api/admin/files/{resource_id}` are registered at `handlers/mod.rs:108` on the bare service config, **outside** the `/api/v1/admin` scope (`handlers/mod.rs:37`). No auth extractor on the handlers. `AdminAuth`, `CsrfProtection`, `RateLimiter`, `SecurityHeaders` are all exported from `middleware/mod.rs:15-19` and **never applied** — the only two active `.wrap()` calls in the process are `VisitorTracker` and `logger` (`main.rs:119,121`). Anonymous destructive deletion of arbitrary resource files.
2. **Same, worse, in `uno-admin`.** `main.rs:77-113` has **zero active `.wrap()` calls**; the only compression wrap is commented out. An admin panel with fully open upload, delete, and signed-URL issuance.

### High

3. **MIME allow-list is client-asserted.** `file_handler.rs:167-169` takes the content type from the multipart header and passes it to a pure string compare (`client.rs:260-306`). No magic-byte sniffing anywhere. `image/svg+xml` is explicitly allow-listed (`config/builder.rs:87`). Extensions are never validated.
4. **Stored XSS.** Files are served from the app's own origin via a bare redirect (`file_handler.rs:36-39`) with no `Content-Disposition` and no `X-Content-Type-Options`. GCS replays the attacker-declared `Content-Type`. The middleware that would set `nosniff` is dead code.
5. **Unauthenticated 7-day signing oracle.** `GET /api/files/display-url` mints a signed URL for any caller-supplied object name (`file_handler.rs:56-73`), defaulting to the maximum GCS lifetime (`config/builder.rs:70`). Anonymous read of any object plus bucket enumeration.
6. **Path traversal in the local backend (latent).** `PathBuf::join` on an unvalidated `resource_id` (`local/client.rs:45-47,129`) and a *textual* `starts_with` prefix check in `parse_storage_url` (`local/client.rs:310-322`) that accepts `file:///data/files/../../../etc/shadow` → arbitrary read and delete. Not currently reachable: both consumers request `features = ["gcs"]`. Becomes live the moment anyone sets `FILE_STORAGE_BACKEND=local`, which the crate's own docs advertise.

### Medium

7. **Fail-open signed URLs.** Any signing error silently degrades to an unsigned public URL (`client.rs:87-101`), contradicting the crate's "Secure by default" claim in `lib.rs:9`.
8. **Over-broad credential scope and key disclosure risk.** `devstorage.read_write` is account-wide (`auth.rs:72-73`), and `ServiceAccountKey` derives `Debug` while holding the raw private key (`auth.rs:11-16`) — it will print in full to any log or panic.
9. **No rate limit, no quota.** `StorageError::RateLimited` (`error.rs:61`) is never constructed. The 10 MB cap is per-file, enforced after full in-memory buffering, with no cap on file count or request size.
10. **No at-rest encryption, retention, or lifecycle policy.** Not present in any form. Orphaned objects accumulate indefinitely.

### Build and hygiene

11. **`uno-app` image build fails.** `Dockerfile:26,46` copies `Cargo.lock`, which does not exist and is gitignored (`.gitignore:19`, `uno-app/.gitignore:8`).
12. **`uno-admin` is not buildable from a fresh clone.** `Cargo.toml:62,65,68,71` reference absolute machine paths — `../../../../Dev-x/polkanight/ember/ember-multichain` and three `ember-fx` crates — which resolve to `/Dev-x/polkanight/ember/...` and do not exist. They are `optional = true`, so this bites only when those features are enabled, but no environment outside one developer's machine can build it.
13. **Zero tests on security-relevant code.** Five unit tests in the whole crate, all for URL parsing. The V4 signing path, MIME validation, size enforcement, and both backends are untested.

---

## 10. What has to happen before a 2,500-licence programme

**Blocking — nothing ships until these are done**

1. Determine whether production schema drift exists. If it does, capture it as a versioned migration. If it does not, the claim feature is down and must be rebuilt. **Do not launch a claim flow whose runtime behaviour is unknown.**
2. Create the `split_type` enum and reconcile the 7 missing columns between `license_repository.rs` and `00002_licenses.up.sql`.
3. Authenticate and authorize the file endpoints; apply the middleware that already exists. Treat this as an incident, not a backlog item.
4. Rebuild the `uno-app` image (generate and commit `Cargo.lock`) and make `uno-admin` build from a clean checkout.

**Required to make the economics real**

5. Design a three-way split into `uno-api` as a first-class type, with a migration, before promising 50/40/10 anywhere.
6. Implement or locate the withdrawal threshold and settlement path. Users are being told about a $5 minimum that has no implementation in this codebase.
7. **Separate Proof of Work from the verification-task proposition before any per-device figure is published.** Blended UP/day is 80% PoW (§6). Either the PoW workload is disclosed as a requirement, or all per-device economics must be modelled on the 0.95 UP/day non-PoW series. The current blended rate will not survive contact with a real device.
8. Replace hardcoded earnings copy with values derived from observed, per-device, measured data.

**Required to make 250/week answerable**

9. Instrument the full funnel — install → KYC → claim → first task → first reward — with per-device identity. The current ledger attributes 97.6% of value to nobody, which makes the funnel unmeasurable.
10. Establish the true per-device event cadence and payout distribution from the closed service. The 0.0845 u mean event cannot be converted to a monthly figure without it.
11. Reconcile the ledger model: one account holding 100% of 20,189 events is not a multi-party economy, and should be explained before it is presented as one.
12. **Establish why output fell 81% in August 2026 and 60% for PoW over 90 days.** A supply that halves in a month cannot underwrite a multi-year licence promise. Determine whether this is PoW difficulty, a task-sourcing change, a payout-policy change, or participant churn.

**Governance**

13. Remove committed production personal data and assess notification obligations.
14. Establish a workspace so `uno-api` and `file-storage` stop being duplicated verbatim.

---

## 11. Verdict

| Question | Verdict |
|---|---|
| Is there an automatic licence distribution system? | The **scaffolding** exists — admin import, publish, split, public claim UI. The **claim path does not execute** against the committed schema. |
| Can it distribute 2,500 licences? | Not in its current state. Blocked on §10.1-10.2, and unverifiable end-to-end because the earning engine is external. |
| Can it sustain 250 productive additions per week? | **No evidence it ever has.** Four attributable human operators in six months. The gap is ~60×, and it is an activation problem, not a throughput problem. |
| Is the split model ready for 50/40/10? | No. `SplitType` is two-way with three variants; the three-way type exists only in the admin console and is collapsed on publish. |
| Is the earning proposition real? | **Weaker than marketed.** 80% of credits are Proof of Work, which the product never mentions. Genuine verification work yields 0.95 UP/day network-wide. |
| Is the output sustainable? | **Not currently.** August 2026 fell 81%; September 32%; PoW down 60% over 90 days. |
| Is it safe to expose publicly? | Not as wired. Unauthenticated upload and delete, stored XSS, a signing oracle, and entirely dead middleware. |
| Is the production data trustworthy for forecasting? | No. 97.6% of reward events unattributed, 100% credited to one account, and the incentive series is in steep decline. |

The most useful next step is not a feature. It is to answer three questions with instrumentation rather than inference:

1. **How many distinct people have ever actually earned money, and how many are still active?**
2. **What fraction of credits is Proof of Work, and is that disclosed to participants?**
3. **Why did output fall 81% in August, and can it be reversed?**

Every planning figure in this engagement depends on those answers, and neither the committed ledger nor the incentive export can supply them.
