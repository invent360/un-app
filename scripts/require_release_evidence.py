#!/usr/bin/env python3
"""Fail closed until Phase 9 supplies a verified deployment adapter and gates."""
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
target = json.loads((root / "deployment/target.json").read_text())
if not target["promotion_enabled"] or not target["deployment_adapter"]:
    raise SystemExit("Release promotion disabled: no verified Ember deployment adapter")
adapter = root / target["deployment_adapter"]
if not adapter.is_file():
    raise SystemExit("Release promotion disabled: deployment adapter missing")
evidence_path = root / "deployment/release-evidence.json"
if not evidence_path.is_file():
    raise SystemExit("Release promotion disabled: evidence manifest missing")
evidence = json.loads(evidence_path.read_text())
revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
if evidence.get("revision") != revision:
    raise SystemExit("Release evidence does not match the checked-out revision")
for gate in ("build", "schema", "security", "inventory", "finance", "storage", "preservation", "ux", "operations", "commercial"):
    if evidence.get("gates", {}).get(gate) != "verified":
        raise SystemExit(f"Release gate {gate} is not verified")
if not evidence.get("image_digests"):
    raise SystemExit("Release image digests missing")
print("Release evidence manifest passes local structural checks; provider-side attestation remains required")
