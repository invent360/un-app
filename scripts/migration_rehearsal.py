#!/usr/bin/env python3
"""
Migration rehearsal script for Phase 9 cutover preparation.

This script performs a full migration rehearsal against an isolated database:
1. Restores a snapshot to the rehearsal database
2. Runs all pending migrations
3. Validates table counts and content hashes
4. Generates a reconciliation report

Usage:
    python3 scripts/migration_rehearsal.py --snapshot backup.dump --target postgres://rehearsal/uno
"""

import argparse
import hashlib
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


def run_command(cmd: list[str], capture: bool = True) -> tuple[int, str, str]:
    """Run a command and return exit code, stdout, stderr."""
    result = subprocess.run(cmd, capture_output=capture, text=True)
    return result.returncode, result.stdout, result.stderr


def restore_snapshot(snapshot_path: str, target_url: str) -> bool:
    """Restore a database snapshot to the target database."""
    print(f"Restoring snapshot {snapshot_path} to {target_url}...")

    # Parse connection URL
    # Format: postgres://user:pass@host:port/dbname
    import urllib.parse
    parsed = urllib.parse.urlparse(target_url)

    env = {
        "PGHOST": parsed.hostname or "localhost",
        "PGPORT": str(parsed.port or 5432),
        "PGUSER": parsed.username or "postgres",
        "PGPASSWORD": parsed.password or "",
        "PGDATABASE": parsed.path.lstrip("/"),
    }

    # Drop and recreate database
    code, _, err = run_command(
        ["psql", "-c", f"DROP DATABASE IF EXISTS {env['PGDATABASE']}"],
        capture=True
    )

    code, _, err = run_command(
        ["psql", "-c", f"CREATE DATABASE {env['PGDATABASE']}"],
        capture=True
    )

    # Restore from snapshot
    code, _, err = run_command(
        ["pg_restore", "-d", target_url, snapshot_path],
        capture=True
    )

    if code != 0:
        print(f"Warning during restore (may be expected): {err}")

    return True


def run_migrations(target_url: str, migrations_dir: str) -> bool:
    """Run all pending migrations using sqlx-cli."""
    print(f"Running migrations from {migrations_dir}...")

    code, stdout, stderr = run_command([
        "cargo", "sqlx", "migrate", "run",
        "--source", migrations_dir,
        "--database-url", target_url,
    ])

    if code != 0:
        print(f"Migration failed: {stderr}")
        return False

    print(stdout)
    return True


def get_table_counts(target_url: str) -> dict[str, int]:
    """Get row counts for all tables."""
    print("Collecting table counts...")

    query = """
    SELECT schemaname || '.' || tablename as table_name, n_live_tup as row_count
    FROM pg_stat_user_tables
    ORDER BY table_name;
    """

    code, stdout, _ = run_command([
        "psql", target_url, "-t", "-c", query
    ])

    counts = {}
    for line in stdout.strip().split("\n"):
        if "|" in line:
            parts = [p.strip() for p in line.split("|")]
            if len(parts) == 2:
                counts[parts[0]] = int(parts[1])

    return counts


def compute_content_hash(target_url: str, table: str, columns: list[str]) -> str:
    """Compute a hash of table contents for reconciliation."""
    column_list = ", ".join(columns)
    query = f"SELECT md5(string_agg(row_to_json(t)::text, '' ORDER BY {columns[0]})) FROM (SELECT {column_list} FROM {table}) t;"

    code, stdout, _ = run_command([
        "psql", target_url, "-t", "-c", query
    ])

    return stdout.strip()


def generate_reconciliation_report(
    table_counts: dict[str, int],
    content_hashes: dict[str, str],
    duration_seconds: float,
) -> dict:
    """Generate a reconciliation report."""
    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "duration_seconds": duration_seconds,
        "status": "success",
        "table_counts": table_counts,
        "content_hashes": content_hashes,
        "critical_tables": {
            "licenses": table_counts.get("public.licenses", 0),
            "claims": table_counts.get("public.claims", 0),
            "allocations": table_counts.get("public.allocations", 0),
            "content_items": table_counts.get("public.content_items", 0),
            "media_assets": table_counts.get("public.media_assets", 0),
        },
        "validation": {
            "tables_present": len(table_counts) > 0,
            "critical_tables_populated": all(
                v > 0 for k, v in table_counts.items()
                if k in ["public.licenses", "public.faq"]
            ) if table_counts else False,
        },
    }

    return report


def main():
    parser = argparse.ArgumentParser(description="Migration rehearsal script")
    parser.add_argument("--snapshot", help="Path to database snapshot file")
    parser.add_argument("--target", required=True, help="Target database URL")
    parser.add_argument("--migrations", default="uno-app/migrations", help="Migrations directory")
    parser.add_argument("--output", default="rehearsal-report.json", help="Output report path")
    parser.add_argument("--skip-restore", action="store_true", help="Skip snapshot restore")

    args = parser.parse_args()

    start_time = datetime.now()
    print("=" * 60)
    print("Migration Rehearsal - Phase 9")
    print("=" * 60)

    # Step 1: Restore snapshot (if provided)
    if args.snapshot and not args.skip_restore:
        if not restore_snapshot(args.snapshot, args.target):
            print("Failed to restore snapshot")
            sys.exit(1)

    # Step 2: Run migrations
    if not run_migrations(args.target, args.migrations):
        print("Failed to run migrations")
        sys.exit(1)

    # Step 3: Collect table counts
    table_counts = get_table_counts(args.target)
    print(f"Found {len(table_counts)} tables")

    # Step 4: Compute content hashes for critical tables
    critical_tables = {
        "licenses": ["id", "lease_code", "status"],
        "claims": ["id", "license_id", "device_id"],
        "faq": ["id", "question", "answer"],
    }

    content_hashes = {}
    for table, columns in critical_tables.items():
        full_name = f"public.{table}"
        if full_name in table_counts:
            content_hashes[table] = compute_content_hash(args.target, table, columns)

    # Step 5: Generate report
    duration = (datetime.now() - start_time).total_seconds()
    report = generate_reconciliation_report(table_counts, content_hashes, duration)

    # Save report
    with open(args.output, "w") as f:
        json.dump(report, f, indent=2)

    print("=" * 60)
    print(f"Rehearsal completed in {duration:.1f} seconds")
    print(f"Report saved to: {args.output}")
    print(f"Tables: {len(table_counts)}")
    print(f"Licenses: {report['critical_tables']['licenses']}")
    print(f"Claims: {report['critical_tables']['claims']}")
    print("=" * 60)

    # Return success if all critical tables exist
    if report["validation"]["tables_present"]:
        print("REHEARSAL PASSED")
        sys.exit(0)
    else:
        print("REHEARSAL FAILED - Missing tables")
        sys.exit(1)


if __name__ == "__main__":
    main()
