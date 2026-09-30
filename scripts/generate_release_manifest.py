#!/usr/bin/env python3
"""
Generate release manifest with artifact hashes and digests.

This script collects all build artifacts and their checksums into a single
release manifest for deployment verification and audit purposes.
"""

import json
import os
import hashlib
from datetime import datetime, timezone
from pathlib import Path


def compute_file_hash(filepath: Path) -> str:
    """Compute SHA256 hash of a file."""
    sha256 = hashlib.sha256()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256.update(chunk)
    return sha256.hexdigest()


def read_digest_file(filepath: Path) -> str:
    """Read a digest from a file, handling various formats."""
    if not filepath.exists():
        return "not-found"

    content = filepath.read_text().strip()
    # Handle formats like "sha256:..." or just the hash
    if ":" in content:
        return content.split(":")[-1].strip()
    # Handle formats like "hash  filename"
    if " " in content:
        return content.split()[0].strip()
    return content


def collect_artifacts(artifacts_dir: Path) -> dict:
    """Collect all artifact hashes from downloaded artifacts."""
    artifacts = {}

    if not artifacts_dir.exists():
        return artifacts

    for artifact_path in artifacts_dir.iterdir():
        if artifact_path.is_dir():
            for file_path in artifact_path.iterdir():
                if file_path.is_file():
                    key = f"{artifact_path.name}/{file_path.name}"
                    if file_path.suffix == ".txt":
                        artifacts[key] = read_digest_file(file_path)
                    elif file_path.suffix == ".sarif":
                        artifacts[key] = "security-scan-results"
                    else:
                        artifacts[key] = compute_file_hash(file_path)

    return artifacts


def generate_manifest() -> dict:
    """Generate the complete release manifest."""
    # Get git information
    git_sha = os.environ.get("GITHUB_SHA", "local-build")
    git_ref = os.environ.get("GITHUB_REF", "refs/heads/local")
    run_id = os.environ.get("GITHUB_RUN_ID", "local")
    run_number = os.environ.get("GITHUB_RUN_NUMBER", "0")

    # Collect artifacts
    artifacts_dir = Path("artifacts")
    artifacts = collect_artifacts(artifacts_dir)

    manifest = {
        "version": "1.0",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "git": {
            "sha": git_sha,
            "ref": git_ref,
        },
        "ci": {
            "run_id": run_id,
            "run_number": run_number,
            "workflow": "CI",
        },
        "artifacts": artifacts,
        "containers": {
            "uno-app": artifacts.get("uno-app-image-digest/uno-app-digest.txt", "not-built"),
            "uno-admin": artifacts.get("uno-admin-image-digest/uno-admin-digest.txt", "not-built"),
        },
        "binaries": {
            "worker": artifacts.get("worker-binary-hash/worker-hash.txt", "not-built"),
        },
        "security_scans": {
            "uno-app": "trivy-scan-uno-app/trivy-uno-app.sarif" in artifacts,
            "uno-admin": "trivy-scan-uno-admin/trivy-uno-admin.sarif" in artifacts,
        },
        "checksums": {
            "manifest_schema": "1.0",
        },
    }

    return manifest


def main():
    manifest = generate_manifest()

    # Write manifest
    manifest_path = Path("release-manifest.json")
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"Release manifest generated: {manifest_path}")
    print(json.dumps(manifest, indent=2))


if __name__ == "__main__":
    main()
