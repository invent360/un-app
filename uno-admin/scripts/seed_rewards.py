#!/usr/bin/env python3
"""
Standalone seeder script for importing rewards from chunk files into ScyllaDB.

Usage:
    pip install scylla-driver
    python scripts/seed_rewards.py

Environment variables:
    SCYLLA_DB_HOST - ScyllaDB host (default: 127.0.0.1)
    SCYLLA_DB_PORT - ScyllaDB port (default: 9042)
    SCYLLA_DB_KEYSPACE - Database keyspace (default: unity_dashboard)
    REWARDS_CHUNKS_DIR - Directory containing chunk files (default: ./data/rewards)
"""

import json
import os
from pathlib import Path
from datetime import datetime
from cassandra.cluster import Cluster
from cassandra.query import SimpleStatement

# Configuration
CONFIG = {
    "host": os.environ.get("SCYLLA_DB_HOST", "127.0.0.1"),
    "port": int(os.environ.get("SCYLLA_DB_PORT", "9042")),
    "keyspace": os.environ.get("SCYLLA_DB_KEYSPACE", "unity_dashboard"),
    "chunks_dir": os.environ.get("REWARDS_CHUNKS_DIR", "./data/rewards"),
}


def parse_timestamp(ts_str: str) -> datetime:
    """Parse ISO timestamp string to datetime."""
    # Handle various formats
    ts_str = ts_str.replace("+00:00", "Z").rstrip("Z")
    # Remove microseconds if present (keep only milliseconds)
    if "." in ts_str:
        parts = ts_str.split(".")
        ms_part = parts[1][:3]  # Take first 3 digits (milliseconds)
        ts_str = parts[0] + "." + ms_part
    try:
        return datetime.fromisoformat(ts_str)
    except ValueError:
        return datetime.strptime(ts_str[:19], "%Y-%m-%dT%H:%M:%S")


def get_existing_ids(session, keyspace: str) -> set:
    """Get all existing reward IDs from the database."""
    query = f"SELECT id FROM {keyspace}.rewards"
    rows = session.execute(query)
    return {row.id for row in rows}


def insert_reward(session, keyspace: str, reward: dict) -> bool:
    """Insert a single reward record."""
    query = f"""
        INSERT INTO {keyspace}.rewards (
            id, user_id, type, description, node_id, license_id,
            license_lease_id, task_key, task_metadata, completed_at,
            created_at, amount_micros
        ) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
    """

    # Parse timestamps
    completed_at = parse_timestamp(reward["completedAt"])
    created_at = parse_timestamp(reward["createdAt"])

    # Convert task_metadata to string if present
    task_metadata = None
    if reward.get("taskMetadata"):
        task_metadata = json.dumps(reward["taskMetadata"]) if isinstance(reward["taskMetadata"], dict) else str(reward["taskMetadata"])

    session.execute(query, (
        reward["id"],
        reward["userId"],
        reward["type"],
        reward.get("description"),
        reward["nodeId"],
        reward["licenseId"],
        reward["licenseLeaseId"],
        reward.get("taskKey"),
        task_metadata,
        completed_at,
        created_at,
        reward["amountMicros"],
    ))
    return True


def seed_from_chunks(session, keyspace: str, chunks_dir: str) -> int:
    """Seed rewards from chunk files."""
    dir_path = Path(chunks_dir)
    if not dir_path.exists() or not dir_path.is_dir():
        raise ValueError(f"Chunks directory not found: {chunks_dir}")

    # Find chunk files
    chunk_files = sorted([
        f for f in dir_path.iterdir()
        if f.name.startswith("rewards_chunk_") and f.name.endswith(".json")
    ])

    if not chunk_files:
        raise ValueError("No chunk files found")

    print(f"Found {len(chunk_files)} chunk files")

    # Get existing IDs
    existing_ids = get_existing_ids(session, keyspace)
    print(f"Found {len(existing_ids)} existing rewards in database")

    total_inserted = 0
    total_skipped = 0
    total_errors = 0

    for chunk_file in chunk_files:
        with open(chunk_file, "r") as f:
            chunk = json.load(f)

        chunk_inserted = 0
        chunk_skipped = 0

        for reward in chunk["records"]:
            if reward["id"] in existing_ids:
                chunk_skipped += 1
                continue

            try:
                insert_reward(session, keyspace, reward)
                chunk_inserted += 1
                existing_ids.add(reward["id"])  # Track inserted IDs
            except Exception as e:
                print(f"  Error inserting {reward['id']}: {e}")
                total_errors += 1

        total_inserted += chunk_inserted
        total_skipped += chunk_skipped

        print(f"Chunk {chunk['chunk_index']} ({chunk_file.name}): +{chunk_inserted} inserted, {chunk_skipped} skipped | Total: {total_inserted}")

    return total_inserted, total_skipped, total_errors


def main():
    print("=== Rewards Seeder ===")
    print(f"Host: {CONFIG['host']}:{CONFIG['port']}")
    print(f"Keyspace: {CONFIG['keyspace']}")
    print(f"Chunks dir: {CONFIG['chunks_dir']}")
    print()

    # Connect to ScyllaDB
    print(f"Connecting to ScyllaDB...")
    cluster = Cluster([CONFIG["host"]], port=CONFIG["port"])
    session = cluster.connect(CONFIG["keyspace"])
    print(f"Connected to keyspace: {CONFIG['keyspace']}")
    print()

    try:
        inserted, skipped, errors = seed_from_chunks(
            session, CONFIG["keyspace"], CONFIG["chunks_dir"]
        )

        print()
        print("=== COMPLETE ===")
        print(f"Inserted: {inserted}")
        print(f"Skipped (duplicates): {skipped}")
        print(f"Errors: {errors}")

    finally:
        cluster.shutdown()


if __name__ == "__main__":
    main()
