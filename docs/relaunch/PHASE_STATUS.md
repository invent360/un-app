# Relaunch phase status

Updated 29 September 2026. These are implementation and evidence states against the [phased plan](../UNO_APP_V2_PHASED_IMPLEMENTATION_PLAN.md), not release approvals. `ember-multichain` is excluded.

| Phase | State | Gate evidence / remaining condition |
|---|---|---|
| 0 | In progress | Both apps pass local SSR/hydration checks and library suites; source inventory and fresh PostgreSQL migrations pass. A `uno-app` release image from before the final dependency changes built at 66 MB and served `/` (200), `/api/v1/ready` (200, database connected) and the deliberately disabled public claim route (503) against the isolated PostgreSQL fixture; rebuild of the final revision is due. `uno-admin` release WASM passes but native image build exceeds the local 4 GB VM even at optimization level 0; a larger CI runner remains unverified. The guarded RustSec audit and scoped Clippy gates now pass locally. The representative deployed-source baseline and hosted CI evidence remain open. |
| 1 | Not started | G0 must pass; obtain authorised deployed schema/export for upgrade mapping. |
| 2 | Not started | Requires G1 and verified identity provider contract. |
| 3 | Not started | Requires G1–G2 and integration contracts. |
| 4 | Not started | Issuance remains unavailable until G4. |
| 5 | Not started | Funding/settlement remains unavailable until G5. |
| 6 | Not started | Requires real media attachment/backup contract and G6 restore evidence. |
| 7 | Not started | Requires secure underlying services and reviewed market facts. |
| 8 | Not started | Requires reconciled service outcomes and optional provider contracts. |
| 9 | Not started | Requires G0–G8, exact artifact evidence, production runbook and controlled pilot approval. |
| 10 | Not started | Requires G9 and applicable optional-provider/retirement approvals. |

Phase 0 task detail:

| Task | State | Evidence / remaining condition |
|---|---|---|
| P0-01 | Implemented awaiting evidence | `scripts/relaunch_inventory.py` produces `evidence/source-inventory.json` without data rows or secrets: 288 interface literals, 200 query calls, 12 scheduler sites, 3 WebSocket sites, ten locales, 66 data files and 183 source media assets. Source findings are separated from deployed facts. Review against a clean checkout. |
| P0-02 | In progress | Both development frontend builds and current SSR/hydration checks pass; a prior `uno-app` Docker release SSR+WASM image built and started successfully, with HTTP smoke checks for `/` (200), readiness (200) and fail-closed claim (503). Rebuild the final revision. `uno-admin` Docker release WASM passes; native server compilation is killed by the 4 GB local VM, including with sequential build stages, LTO disabled and optimization level 0. Verify the server image on a larger runner before closing. **Known constraint: uno-admin server requires 8GB+ memory to build.** |
| P0-03 | Verified | Stale `uno-app/deps/{file-storage,uno-api}` directories removed. `scripts/check_dependency_provenance.py` passes: both apps consume canonical shared sources, local-only storage, wallet excluded. SSR/WASM builds pass after cleanup. |
| P0-04 | In progress | Mandatory CI and fail-closed deployment preflight written; image digest recording added to CI containers job with artifact upload (90-day retention). Hosted CI execution remains unverified. Four of five initial RustSec findings were removed from the lockfile; the remaining unfixed RSA advisory is permitted only after `scripts/audit_release_dependencies.py` proves RSA unreachable in the all-features workspace graph. The guarded audit passes locally against 1,277 advisories. CI now denies Clippy correctness/suspicious lints in both apps while recording the large legacy warning backlog; this scoped gate passes locally. Full `-D warnings` is not clean. |
| P0-05 | Blocked | Source manifest/locale/data-file hashes captured and `BASELINE_EXPORT_PROTOCOL.md` defines the authorised read-only export; deployed schema, entity/content/media counts and representative exports are unavailable in this workspace. **External dependency: requires authorized production database access.** |
| P0-06 | Implemented awaiting evidence | `CONTRACT_REGISTER.md` records decisions, unknowns, owners and safe fallbacks; external contracts remain unconfirmed. |
| P0-07 | Verified | `phase0_http.rs` requires `DATABASE_URL` via `.expect()` and fails if database unavailable. Fresh isolated PostgreSQL 16 migrations and actual registered-route denial test pass; portal/admin library suites pass 84/84 and 28/28. `acceptance_tests.rs` contains simulation/documentation tests (Mock*) separate from integration tests; these serve as logic validation and are documented as non-integration evidence. |

No later phase has been certified. A green unit test, source inventory or code-only adapter cannot substitute for provider, deployed-data, volume or recovery evidence.
