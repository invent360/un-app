#!/usr/bin/env python3
"""Check formatting of owned Rust changes without rewriting legacy files."""
import os
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[1]
base = os.environ.get("BASE_SHA", "").strip()
if not base or set(base) == {"0"}:
    result = subprocess.run(["git", "rev-parse", "HEAD^"], cwd=root,
                            stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
    base = result.stdout.strip() if result.returncode == 0 else ""
if not base:
    raise SystemExit("A real comparison commit is required for the format gate")
changed = subprocess.check_output(["git", "diff", "--name-only", "--diff-filter=ACMR", f"{base}...HEAD"],
                                  cwd=root, text=True).splitlines()
owned = ("uno-app/src/", "uno-app/tests/", "uno-admin/src/", "uno-admin/tests/",
         "uno-api/src/", "uno-api/tests/", "file-storage/src/", "file-storage/tests/")
rust = [str(root / name) for name in changed if name.endswith(".rs") and name.startswith(owned)]
if not rust:
    print("No owned Rust source changed")
    sys.exit(0)
print(f"Checking {len(rust)} changed Rust files")
sys.exit(subprocess.run(["rustfmt", "--edition", "2021", "--check", "--config", "skip_children=true", *rust],
                        cwd=root).returncode)
