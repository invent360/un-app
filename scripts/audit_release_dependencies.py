#!/usr/bin/env python3
"""Fail on RustSec findings reachable by the workspace release feature graph.

SQLx retains its unused MySQL/RSA package in Cargo.lock even though every
workspace package selects PostgreSQL. Check reachability before excluding that
single unfixed advisory from the lockfile scan.
"""

import os
import subprocess
import sys


def run(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, check=True, text=True, capture_output=True)


def main() -> int:
    tree = run("cargo", "tree", "--locked", "--all-features", "--prefix", "none", "-i", "rsa@0.9.10")
    if tree.stdout.strip():
        print("RUSTSEC-2023-0071 is reachable in the workspace feature graph", file=sys.stderr)
        print(tree.stdout, file=sys.stderr)
        return 1
    print("RUSTSEC-2023-0071: RSA is unreachable in the all-features workspace graph; checking all other advisories")

    audit_bin = os.environ.get("CARGO_AUDIT_BIN", "cargo")
    audit_args = ["audit", "--file", "Cargo.lock", "--ignore", "RUSTSEC-2023-0071"]
    if audit_db := os.environ.get("CARGO_AUDIT_DB"):
        audit_args.extend(["--db", audit_db, "--stale"])
    result = subprocess.run([audit_bin, *audit_args], check=False)
    return result.returncode


if __name__ == "__main__":
    raise SystemExit(main())
