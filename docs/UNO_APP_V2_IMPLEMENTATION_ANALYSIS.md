# UNO v2 requirements against the implementation

Reviewed 29 September 2026 against baseline `4371df0ce5dfdc8479a63ea9a1cda3db4bcb2176` and the current working-tree repairs. Scope: `uno-app`, `uno-admin`, `uno-api`, `file-storage`, and their dependency graph. `deps/ember-multichain` is excluded at the owner's request; a replacement wallet implementation is not a prerequisite in this scope.

## Assessment

The [requirements](UNO_APP_V2_RELAUNCH_REQUIREMENTS.md) are applicable to this codebase, but the implementation is not ready for relaunch. Existing screens, models, repositories, and migrations provide a useful foundation. Their presence and the checked task list do not establish working production flows. The highest risks are the active claim path, incompatible repository/schema contracts, cross-database publication, and financial records that do not yet implement an authoritative ledger.

Foundational repairs are present in this working tree. They improve authentication, shared dependency resolution, local media operations, and readiness, but they do not complete the specification. No full REV ticket or end-to-end relaunch gate is certified closed here.

The [phased implementation plan](UNO_APP_V2_PHASED_IMPLEMENTATION_PLAN.md) maps the remaining requirements to concrete tasks, dependencies and acceptance gates. Plan creation does not change the implementation status below.

## Requirement coverage

| Ticket | Current status | Evidence and remaining work |
|---|---|---|
| REV-01: one dependency source | Partial; build checks pass | Both apps now consume root `uno-api` and `file-storage` through a root workspace/lockfile. A provenance script verifies actual resolved sources and local-only storage. Ember UI crates remain. Obsolete tracked API/storage copies have been removed; ignored build caches may remain locally. Container contexts were corrected; release images and clean-checkout builds are unverified. |
| REV-02: human authentication | Partial; verified-session boundary tests pass | The unsigned payload extractor is replaced with RS256 verification, trusted `kid` keys, issuer/audience/expiry checks and bounded session lifetime. Admin API/server-function/WebSocket paths require verified operator sessions and MFA. CMS review functions check distinct permissions and authenticated actors. Resource-level authorization, service-level enforcement, issuer/login integration, session revocation and all CMS aliases remain to be demonstrated. |
| REV-03: machine authentication | Partial | `UnoApiClient` now sends an explicitly configured bearer key alongside existing signatures. Missing or weak keys fail locally. Other clients and active endpoints still need a complete method/path/body-bound, shared replay contract, idempotency records and real client-to-portal tests. The process-local nonce helper does not satisfy this ticket. |
| REV-04: secure claim state machine | Open; issuance paused | Factory still uses `LicenseService::new`, leaving the atomic repository unconfigured. Wizard still calls legacy reserve/confirm. The active service now rejects reserve, confirm, claim, release and code retrieval with `LicenseUnavailable`; public claim returns 503. Atomic reserve exposes the credential; atomic confirm checks the claimed branch before caller ownership. Participant ownership, hashed secrets, frozen agreements/referrals and real race tests remain. Do not enable the atomic path merely to remove the optional state. |
| REV-05: schema reconciliation | Partial; fresh migrations pass | Migration 17's invalid UUID-to-VARCHAR foreign key was corrected and applied to an isolated PostgreSQL 16 fixture. Migrations 1–17 are recorded successful there. Licence UUID bindings still conflict with VARCHAR IDs; split strings conflict with enum labels; CMS review queries reference missing tables. Upgrade mapping/backfills and real repository-query coverage remain. |
| REV-06: wallet stubs | Excluded by scope | Production admin dependency and wallet module registration were removed. Provenance check confirms neither app resolves `ember-multichain`. Its source remains unchanged. Wallet login/signing functionality is unavailable; no genuine wallet integration is claimed. |
| REV-07: publication and identity | Open; P0 blocker | Marketplace mapping still fabricates a credential when missing, defaults validity, and marks locally selected IDs published after partial portal success. `License::with_custom_id` still truncates external IDs into UUIDs or substitutes random IDs. CSV parsing now requires a credential and unambiguous dates, but full identity/provenance and per-item acknowledgements remain. |
| REV-08: referral policy | Open; P0 blocker | Freeze qualified attribution and explicit no-referral outcomes against an accepted agreement. Enforce single-level attribution in the active service and schema. Existing referral models/migrations do not prove these behaviors. |
| REV-09: financial ledger | Open; P0 blocker | Basis-point models and ledger migrations are useful components. The active lifecycle must enforce versioned 50/40/10 agreements, bounded shares, deterministic integer rounding, unique upstream event IDs, immutable entries, funding reconciliation and retry-safe posting. |
| REV-10: durable work/readiness | Partial | Portal health now performs a bounded `SELECT 1`, returns 503 without a usable database and exposes separate liveness. Production DB failure prevents startup; UI-only mode is explicit and development-only. Readiness still does not establish schema, volume, upstream or worker health. Durable job/outbox types remain insufficiently connected to active schedulers. |
| REV-11: local media migration | Partial; filesystem boundary tests pass | Both apps compile local-only storage. Root backend uses retained directory descriptors, no-follow operations, decoded/re-encoded PNG/JPEG/WebP, unique filenames, staging writes and sync. Missing roots are not created. Production marker identity is required when constructing the client. Multipart buffering is bounded, but is not streaming-to-staging. Dedicated Ember volume/mount verification, startup checks, long-lived shared clients, database media ownership/visibility, crash reconciliation, legacy URL conversion, migration and restore rehearsal remain. |
| REV-12: enforceable delivery | Partial; local frontends pass | Rust is pinned; workspace Dockerfiles use root context; app site output directories are distinct. Both Cargo Leptos development frontend builds now pass after moving the conditional route inside a single route tree; release debug routes are compile-time rejected. CI has mandatory source/build/test/image/security jobs, but hosted execution and release image builds remain unverified. Deployment workflows now validate a single-writer contract and fail closed pending a real deployment adapter; absent infrastructure paths are no longer invoked. |
| REV-13: genuine acceptance tests | Partial | New tests exercise real JWT verification, Actix authorization middleware, registered admin/public claim routes against migrated PostgreSQL, and local filesystem operations. These do not exercise every production handler or actual licence/database races. Existing acceptance tests still simulate claims; they now fail if their HTTP server is absent and must not be used as launch evidence. |
| REV-14: participant journey | Partial existing UI | Retain the economics-first wizard, but connect it to verified participant identity and the secure issuance service. Eligibility, resumable status, official activation, funding/productivity distinctions and support flows need actual end-to-end evidence. |
| REV-15: locales/CMS governance | Partial existing components | Preserve ten locales and existing CMS data. Verify translation completeness, Bangla and RTL rendering. Authentication repairs do not supply missing review tables, immutable versions, permission scopes, safe previews and complete transition/approval enforcement. |
| REV-16: operator forecasting | Partial existing pure logic | Forecast engine exists. Operator UI, versioned scenarios, exports, calculator parity and explicit forecast-versus-observation reporting remain. |
| REV-17: agent/CRM operations | Partial existing components | Build agent permissions and reconciliation on authoritative operational records. Optional CRM adapters need outbox delivery, deduplication and outage isolation; they must not become the distribution authority. |
| REV-18: governance | Partial existing types/tables | Retention and launch-gate tables exist after fresh migration. Implement actual cleanup/archive jobs, audit actor/scope coverage, privacy handling and operational evidence rather than recording type existence as completion. |

## Code evidence for the principal blockers

- [Factory](../uno-app/src/server/app/service_factory.rs): constructs the legacy licence service despite creating a claim repository.
- [Wizard review](../uno-app/src/components/wizard/stages/review.rs), [claim](../uno-app/src/components/wizard/stages/claim.rs), and [server functions](../uno-app/src/api/licenses.rs): active legacy reserve/confirm path.
- [Atomic claim repository](../uno-app/src/server/repositories/claim_repository.rs): pre-issuance credential exposure, ownership ordering, inconsistent UUID bindings and reservation lifecycle defects.
- [Licence repository](../uno-app/src/server/repositories/license_repository.rs) and [migration 14](../uno-app/migrations/00014_license_lifecycle.up.sql): ID and split-label contract mismatch. Compilation cannot detect dynamically constructed SQL errors.
- [CMS review repository](../uno-app/src/server/repositories/review_repository.rs): queries `content_versions`, `content_reviews` and `preview_tokens`, which the current migration set does not create under those names.
- [Admin database](../uno-admin/src/db/mod.rs), [marketplace publication](../uno-admin/src/logic/marketplace_service.rs), and [licence identity model](../uno-api/src/models/license.rs): Scylla persistence and unsafe cross-system identity/publication semantics.
- [Existing acceptance suite](../uno-app/tests/acceptance_tests.rs): simulated exclusivity; missing-server handling now fails.

## Validation of the working-tree repairs

Run from the repository root with the pinned toolchain and root lockfile:

```sh
cargo check --locked -p uno-app --features ssr --lib --bin uno-app
cargo check --locked -p uno-admin --features ssr --lib --bin uno-admin
cargo check --locked -p uno-app --lib --features hydrate --target wasm32-unknown-unknown
cargo check --locked -p uno-admin --lib --features hydrate --target wasm32-unknown-unknown
cargo test --locked -p uno-api --features web-auth,services,client
cargo test --locked -p file-storage --no-default-features --features local
python3 scripts/check_dependency_provenance.py
cargo sqlx migrate run --source uno-app/migrations
```

Both SSR and hydration checks pass locally. Shared API tests and six new local filesystem tests pass. Existing ignored integration/documentation tests are not credited as verified behavior. The PostgreSQL check used an isolated empty PostgreSQL 16 database, with all 17 migrations successful after the migration-17 repair. It did not migrate a production snapshot or execute every repository query. Compiler warnings remain.

The new [session boundary suite](../uno-api/tests/session_boundary.rs) rejects missing claims, bad signatures/algorithms, expired/future tokens and invalid issuer/audience. It checks MFA/role/recent-auth requirements and cookie-write origins, and verifies denied requests do not reach a mutation handler. The RSA fixtures are synthetic test-only keys.

The [local filesystem suite](../file-storage/tests/local_boundary.rs) covers opaque immutable URLs, image decoding/re-encoding, appended payload removal, mismatched content types/extensions, dimension limits, filesystem URLs, traversal, symlink resources/objects, retained-root behavior during path replacement and missing-root failure.

Both applications now pass actual Cargo Leptos development frontend builds with the lockfile and matching `wasm-bindgen` CLI 0.2.129. A new registered-route test passes against PostgreSQL 16 with all 17 migrations, checking unauthenticated admin rejection, direct legacy service refusal and public claim 503. The portal release image builds locally with optimized WASM and native SSR and is 66 MB. Admin release WASM passes, but native server compilation is killed by the 4 GB local Docker VM even with separate frontend/server stages and LTO disabled. The Dockerfiles install checksum-pinned upstream build-tool binaries. Admin image validation on a larger runner and full hosted CI remain pending. These repairs do not establish a full acceptance or production migration gate.

## Configuration and compatibility implications

Human sessions require `UNO_SESSION_PUBLIC_KEYS` (JSON object of trusted key ID to RSA public PEM), `UNO_SESSION_ISSUER`, `UNO_SESSION_AUDIENCE` and `UNO_PUBLIC_ORIGIN`. Tokens must have a trusted `kid`, required claims, a supported role and `amr` including `mfa` for editorial/operator permissions. Direct publishing additionally requires recent `auth_time`. No login/token issuer has been added; deployment must supply a real trusted identity flow. API keys cannot substitute for these human sessions.

Portal startup also requires `DATABASE_URL`, an explicit `ADMIN_API_KEY` of at least 32 characters and `CSRF_SECRET_KEY` of at least 32 characters. Preview signing requires an explicit `PREVIEW_SECRET_KEY`. Development without a database requires `UNO_UI_ONLY=true`; this flag does not permit database-free production operation. Upstream adapters require an explicit `UNETWORK_API_KEY`.

Provision `FILE_STORAGE_LOCAL_PATH` before starting local storage. Production additionally requires `FILE_STORAGE_VOLUME_ID` and a matching `.uno-volume` marker. A marker is not proof of a mounted dedicated volume. New local URLs use `local://resource/uuid.ext`; existing `file://` database URLs need an audited migration. The image-only content policy may reject older non-image assets. Existing media has not been migrated or assigned visibility, and public serving remains a release blocker for private material.

Migration 17 was changed because it could not apply to the supplied base schema. Before upgrading any existing installation, inspect its actual schema and `_sqlx_migrations` checksums. If that migration was already applied through a separately altered schema, develop/rehearse a forward correction and an explicit migration-history compatibility procedure. Do not clear checksums or assume the fresh-install test proves upgrade safety.

## Recommended implementation sequence

1. Reconcile the canonical PostgreSQL schema and preserve full upstream IDs. Migrate representative Scylla/CMS data with reversible mapping and repository-backed tests.
2. Implement participant/session ownership and one secure claim service; route every REST/server-function/wizard alias through it. Prove concurrency, wrong-owner retries and expiry against real PostgreSQL.
3. Repair publication acknowledgements, eligibility and referral/agreement snapshots, then post money-related activity through an idempotent integer ledger.
4. Connect durable jobs/outbox, complete machine replay/idempotency, and move resource authorization into shared services.
5. Complete media ownership/visibility, provision the dedicated volume, rehearse restore and correct the deployment target. Build and verify both release images.
6. Finish participant/operator/agent/CMS and localized journeys, forecasting and support operations. Run genuine acceptance tests and the controlled pilot gates.

The milestone order in requirement section 18.5 is sound, with the wallet task excluded. The remaining P0 work is primarily integration and correctness; redesigning more screens before closing it would not make issuance or finances safe.
