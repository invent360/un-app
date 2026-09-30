#!/usr/bin/env python3
"""Fail if either SSR application resolves duplicate or cloud storage sources."""
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
host = next(line.removeprefix("host: ") for line in subprocess.check_output(
    ["rustc", "-vV"], cwd=root, text=True).splitlines() if line.startswith("host: "))
for application in ("uno-app", "uno-admin"):
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--format-version", "1",
        "--no-default-features", "--features", f"{application}/ssr",
        "--filter-platform", host,
    ], cwd=root))
    packages = {package["id"]: package for package in metadata["packages"]}
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    app = next(package for package in packages.values() if package["name"] == application)
    reachable = set()
    pending = [app["id"]]
    while pending:
        package_id = pending.pop()
        if package_id in reachable:
            continue
        reachable.add(package_id)
        pending.extend(nodes[package_id]["dependencies"])
    for shared in ("uno-api", "file-storage"):
        resolved = [packages[key] for key in reachable if packages[key]["name"] == shared]
        expected = root / shared / "Cargo.toml"
        if len(resolved) != 1 or Path(resolved[0]["manifest_path"]).resolve() != expected:
            raise SystemExit(f"{application} must consume the canonical {shared} source")
        if shared == "file-storage":
            features = set(nodes[resolved[0]["id"]]["features"])
            if "local" not in features or features & {"gcs", "full"}:
                raise SystemExit(f"{application} must use local-only storage")
    if any(packages[key]["name"] == "ember-multichain" for key in reachable):
        raise SystemExit(f"{application} must exclude the deferred wallet integration")
    print(f"{application}: canonical shared sources, local-only storage, wallet excluded")
