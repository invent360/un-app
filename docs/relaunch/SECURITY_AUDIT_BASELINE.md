# Security audit baseline

`cargo-audit 0.22.2` scanned the root `Cargo.lock` on 29 September 2026 against 1,277 RustSec advisories. The initial scan failed on five vulnerabilities. After dependency changes, the guarded audit in `scripts/audit_release_dependencies.py` passes with two informational unmaintained-package warnings. It excludes only RUSTSEC-2023-0071 after first proving that `rsa 0.9.10` is absent from the all-features workspace dependency graph; the unused package remains in the SQLx lockfile resolution. If RSA becomes reachable, the guard fails before invoking cargo-audit. All other advisories remain blocking.

| Advisory | Locked crate | Dependency path / remediation lead |
|---|---|---|
| [RUSTSEC-2026-0258](https://rustsec.org/advisories/RUSTSEC-2026-0258) | `h2 0.3.27` | Removed Actix HTTP/2 default features; package left the lockfile. HTTP/1 transport needs release regression checks. |
| [RUSTSEC-2024-0421](https://rustsec.org/advisories/RUSTSEC-2024-0421) | `idna 0.5.0` | Upgraded `validator` to 0.20; package left the lockfile. |
| [RUSTSEC-2025-0132](https://rustsec.org/advisories/RUSTSEC-2025-0132) | `maxminddb 0.24.0` | Upgraded to 0.27 and adapted GeoIP lookup; package left the lockfile. |
| [RUSTSEC-2026-0235](https://rustsec.org/advisories/RUSTSEC-2026-0235) | `rkyv 0.7.46` | Disabled unused Lightning CSS sourcemap/bundler features; package left the lockfile. |
| [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071) | `rsa 0.9.10` | Replaced direct GCS signing with `ring` and tested its synthetic-key path. No root feature activates RSA, but the package remains in SQLx lockfile resolution. The guarded audit permits this one unreachable advisory only. |

The lockfile scan includes optional and build dependencies; this table is not an exploitability finding for the deployed feature set. The audit job remains mandatory, and the explicit RSA reachability guard is part of its pass condition. Re-run the audit and actual app builds against the final checkout before certifying G0.
