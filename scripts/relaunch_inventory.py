#!/usr/bin/env python3
"""Record source interfaces and hashes without exporting credentials or data rows."""
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "docs/relaunch/evidence/source-inventory.json"

def get_files():
    """Get list of files using rg if available, otherwise use pathlib."""
    if shutil.which("rg"):
        return subprocess.check_output([
            "rg", "--files", "--hidden", "-g", "!.git/**", "-g", "!**/target/**",
            "-g", "!target/**", "-g", "!**/node_modules/**", "-g", "!**/.env*",
        ], cwd=ROOT, text=True).splitlines()

    # Fallback to pathlib
    result = []
    exclude = {".git", "target", "node_modules"}
    for path in ROOT.rglob("*"):
        if path.is_file():
            # Check if any parent directory should be excluded
            parts = path.relative_to(ROOT).parts
            if any(p in exclude for p in parts):
                continue
            if any(p.startswith(".env") for p in parts):
                continue
            result.append(str(path.relative_to(ROOT)))
    return result

files = get_files()
report = {"kind": "source-only baseline; not a deployed schema/data inventory",
          "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
          "interfaces": [], "query_calls": [], "scheduler_sites": [], "websocket_sites": [],
          "manifest_dependencies": {}, "configuration_names": [],
          "migrations": [], "locales": [], "data_manifests": [], "media_manifests": []}
config = set()
for name in sorted(files):
    path = ROOT / name
    if any(name.startswith(f"{app}/src/") for app in ("uno-app", "uno-admin", "uno-api", "file-storage")) and path.suffix == ".rs":
        source = path.read_text()
        patterns = {
            "route_literal": r'\.route\(\s*"([^"\n]+)"',
            "scope_literal": r'(?:web::)?scope\(\s*"([^"\n]+)"',
            "server_function": r'#\[server\(([^\]\n]*)\)',
            "http_attribute": r'#\[(?:actix_web::)?(?:get|post|delete|put|patch)\("([^"\n]+)"',
        }
        for kind, pattern in patterns.items():
            for match in re.finditer(pattern, source):
                report["interfaces"].append({"path": name, "line": source.count("\n", 0, match.start()) + 1,
                                             "kind": kind, "literal": match.group(1),
                                             "authorization_review": "required"})
        for match in re.finditer(r'\b(?:sqlx::query(?:_as|_scalar)?|(?:session|db|pool)\.query(?:_unpaged)?)\b', source):
            report["query_calls"].append({"path": name, "line": source.count("\n", 0, match.start()) + 1,
                                          "call": match.group(), "schema_backed_test": "required"})
        for kind, pattern, destination in (
            ("scheduler", r'\b(?:tokio::spawn|tokio::time::interval|actix_rt::spawn|cron::)\b', "scheduler_sites"),
            ("websocket", r'\b(?:actix_ws::handle|WebSocket::new|ws_jobs_handler)\b', "websocket_sites"),
        ):
            for match in re.finditer(pattern, source):
                report[destination].append({"path": name, "line": source.count("\n", 0, match.start()) + 1,
                                            "kind": kind, "implementation_review": "required"})
        config.update(re.findall(r'(?:env::var|get_required|get_or_default)\(\s*"([A-Z][A-Z0-9_]+)"', source))
    if name.startswith("uno-app/migrations/") and name.endswith(".sql"):
        data = path.read_bytes()
        report["migrations"].append({"path": name, "sha256": hashlib.sha256(data).hexdigest()})
    if re.fullmatch(r"uno-app/src/locales/(?:en|es|tl|hi|sw|pt|fr|ar|id|bn)\.rs", name):
        data = path.read_bytes()
        keys = sorted(re.findall(r'"([^"\n]+)"\s*=>', data.decode()))
        report["locales"].append({"locale": path.stem, "path": name, "keys": keys,
                                 "key_count": len(keys), "sha256": hashlib.sha256(data).hexdigest()})
    if (name.startswith(("docs/content/", "docs/backups/", "uno-admin/data/")) and path.suffix in (".json", ".sql", ".csv")) or name == "statistics-incentives-All.csv":
        data = path.read_bytes()
        report["data_manifests"].append({"path": name, "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
                                         "provenance": "unverified source artifact; restricted review required"})
    if name.startswith(("docs/content/", "docs/backups/")) and path.suffix.lower() in (".png", ".jpg", ".jpeg", ".webp", ".svg", ".gif", ".avif"):
        data = path.read_bytes()
        report["media_manifests"].append({"path": name, "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
                                          "provenance": "unverified source asset; visibility and live use unconfirmed"})
report["configuration_names"] = sorted(config)
for package in ("uno-app", "uno-admin", "uno-api", "file-storage"):
    manifest = tomllib.loads((ROOT / package / "Cargo.toml").read_text())
    report["manifest_dependencies"][package] = sorted(manifest.get("dependencies", {}))
report["notes"] = ["Route fragments must be composed with scopes and manually classified; literals are not a complete permission audit.",
                    "Query-call locations include dynamically constructed queries; source presence does not prove schema compatibility.",
                    "Scheduler/WebSocket sites are static leads for review, not a complete runtime task or subscription inventory.",
                    "Only file hashes/counts/translation keys are exported; no source data rows, environment values or credentials."]
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(json.dumps(report, indent=2) + "\n")
print(f"Inventory: {len(report['interfaces'])} interface literals, {len(report['query_calls'])} query calls, "
      f"{len(report['scheduler_sites'])} scheduler sites, {len(report['websocket_sites'])} WebSocket sites, "
      f"{len(report['locales'])} locales, {len(report['data_manifests'])} data manifests, "
      f"{len(report['media_manifests'])} source media manifests")
