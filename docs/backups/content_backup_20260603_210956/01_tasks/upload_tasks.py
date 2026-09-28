#!/usr/bin/env python3
"""
Upload task and guide content to uno-app CMS.

Usage:
    python upload_tasks.py

Requires:
    - uno-app server running on localhost:3000
    - ADMIN_CLIENT_ID and ADMIN_SECRET_KEY set in uno-app's .env
"""

import json
import time
import uuid
import hmac
import hashlib
import requests
from pathlib import Path

# Configuration
BASE_URL = "http://localhost:3000"
CLIENT_ID = "uno-admin"
SECRET_KEY = "your-secure-secret-key-here-change-in-production"
TASKS_DIR = Path(__file__).parent

# Task definitions
TASKS = [
    ("01_telemetry", "telemetry"),
    ("02_caller_id_testing", "caller-id-testing"),
    ("03_sms_testing", "sms-testing"),
    ("04_connectivity_verification", "connectivity-verification"),
    ("05_entropy_generation", "entropy-generation"),
]


def sign_request(payload: dict) -> dict:
    """Sign a request with HMAC-SHA256."""
    timestamp = int(time.time())
    nonce = str(uuid.uuid4())
    payload_json = json.dumps(payload, separators=(',', ':'))

    message = f"{CLIENT_ID}:{timestamp}:{nonce}:{payload_json}"
    signature = hmac.new(
        SECRET_KEY.encode(),
        message.encode(),
        hashlib.sha256
    ).hexdigest()

    return {
        "client_id": CLIENT_ID,
        "timestamp": timestamp,
        "nonce": nonce,
        "signature": signature,
        "payload": payload
    }


def create_content(content_type: str, slug: str, content: dict) -> dict:
    """Create new content via API."""
    payload = {
        "content_type": content_type,
        "slug": slug,
        "content": content,
        "translations": {},
        "display_order": 0,
        "is_featured": False,
        "change_summary": "Initial upload from task data"
    }

    signed = sign_request(payload)

    response = requests.post(
        f"{BASE_URL}/api/v1/admin/contents",
        json=signed,
        headers={"Content-Type": "application/json"}
    )

    return response.json(), response.status_code


def publish_content(content_id: int) -> dict:
    """Publish content via API."""
    payload = {"published_by": "upload-script"}
    signed = sign_request(payload)

    response = requests.post(
        f"{BASE_URL}/api/v1/admin/contents/{content_id}/publish",
        json=signed,
        headers={"Content-Type": "application/json"}
    )

    return response.json(), response.status_code


def upload_task(task_dir: Path, slug: str):
    """Upload a task from its directory."""
    desc_path = task_dir / "description" / "description.json"

    if not desc_path.exists():
        print(f"  Skipping {slug} - no description.json")
        return

    print(f"Uploading task: {slug}")

    with open(desc_path) as f:
        desc = json.load(f)

    # Build image paths
    image_base = f"/assets/tasks/{task_dir.name}"
    images = [f"{image_base}/{img}" for img in desc.get("images", [])]
    front_image = f"{image_base}/0.png"

    # Build content
    content = {
        "title": desc["title"],
        "description": desc["description"],
        "image": front_image,
        "images": images,
        "status": desc.get("status", "active"),
        "difficulty": desc.get("difficulty"),
        "duration": desc.get("duration"),
        "earnings_estimate": desc.get("earnings", []),
        "requirements": desc.get("requirements", []),
    }

    result, status = create_content("task", slug, content)

    print(f"  Response status: {status}, body: {result}")
    if status == 201:
        print(f"  Created task {slug} (id: {result.get('id')})")
        # Publish
        pub_result, pub_status = publish_content(result["id"])
        if pub_status == 200:
            print(f"  Published task {slug}")
        else:
            print(f"  Warning: Failed to publish: {pub_result}")
    elif "already exists" in str(result) or "duplicate" in str(result).lower():
        print(f"  Task {slug} already exists, skipping...")
    else:
        print(f"  Error creating task: {result}")


def upload_guide(task_dir: Path, task_slug: str):
    """Upload an activation guide from its directory."""
    guide_path = task_dir / "activation" / "activation.json"

    if not guide_path.exists():
        print(f"  Skipping guide for {task_slug} - no activation.json")
        return

    guide_slug = f"{task_slug}-activation-guide"
    print(f"Uploading guide: {guide_slug}")

    with open(guide_path) as f:
        guide = json.load(f)

    # Build image paths for steps
    image_base = f"/assets/tasks/{task_dir.name}"
    steps = []
    for step in guide.get("steps", []):
        step_data = {
            "order": step["order"],
            "title": step["title"],
            "description": step["description"],
            "image": f"{image_base}/{step['image']}" if step.get("image") else None
        }
        steps.append(step_data)

    # Thumbnail
    thumbnail = f"{image_base}/0.png"

    # Build content
    content = {
        "title": guide["title"],
        "description": guide["description"],
        "thumbnail": thumbnail,
        "steps": steps,
        "duration_minutes": guide.get("duration_minutes"),
        "difficulty": guide.get("difficulty"),
    }

    result, status = create_content("guide", guide_slug, content)

    if status == 201:
        print(f"  Created guide {guide_slug} (id: {result.get('id')})")
        # Publish
        pub_result, pub_status = publish_content(result["id"])
        if pub_status == 200:
            print(f"  Published guide {guide_slug}")
        else:
            print(f"  Warning: Failed to publish: {pub_result}")
    elif "already exists" in str(result) or "duplicate" in str(result).lower():
        print(f"  Guide {guide_slug} already exists, skipping...")
    else:
        print(f"  Error creating guide: {result}")


def main():
    print("=== UNO Content Upload Tool ===\n")

    for dir_name, slug in TASKS:
        task_dir = TASKS_DIR / dir_name

        if not task_dir.exists():
            print(f"Directory not found: {task_dir}")
            continue

        upload_task(task_dir, slug)
        upload_guide(task_dir, slug)
        print()

    print("=== Upload Complete ===")


if __name__ == "__main__":
    main()
