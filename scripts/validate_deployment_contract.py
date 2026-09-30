#!/usr/bin/env python3
"""Check checked-in deployment references without claiming the host is provisioned."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
target = json.loads((root / "deployment/target.json").read_text())
assert target["topology"] == "one-authoritative-writer"
assert target["writer_replicas"] == 1
assert target["media"]["backend"] == "local"
assert target["media"]["external_provisioning_required"]
assert target["media"]["actual_mount_check_required"]
assert target["media"]["writer_host_affinity_required"]
assert target["database"]["backend"] == "postgresql"
assert target["database"]["separate_from_media"]
assert not target["promotion_enabled"] or target["deployment_adapter"]
for service in target["services"]:
    dockerfile = root / service["dockerfile"]
    assert dockerfile.is_file(), f"Missing Dockerfile: {dockerfile}"
    assert service["readiness"].startswith("/")
    source = dockerfile.read_text()
    assert "COPY uno-api ./uno-api" in source and "COPY file-storage ./file-storage" in source
    assert "LEPTOS_ENV=PROD" in source
for workflow in ("ci.yml", "deploy.yml", "infrastructure.yml"):
    source = (root / ".github/workflows" / workflow).read_text()
    assert "infrastructure/kubernetes/overlays" not in source, f"Missing path in {workflow}"
    assert "infrastructure/terraform" not in source, f"Missing path in {workflow}"
print("Deployment paths and fail-closed single-writer contract verified; host attachment remains unconfirmed")
