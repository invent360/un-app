#!/usr/bin/env python3
"""
Incremental Rewards Fetcher
Fetches rewards allocations from UnityEdge API going back to August 2005.
Implements rate limiting and progress persistence to avoid server overload.

Usage:
    export UNITYEDGE_AUTH_TOKEN="your_token_here"  # if auth required
    python scripts/fetch_rewards.py

Output:
    /Users/admin/Documents/unetwork/uno-admin/data/rewards/
    ├── progress.json           # Resume state
    ├── summary.json            # Final summary
    ├── rewards_chunk_0000.json # Records 1-500
    ├── rewards_chunk_0001.json # Records 501-1000
    └── ...
"""

import asyncio
import aiohttp
import ssl
import certifi
import json
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional
import logging

# Configuration
CONFIG = {
    "endpoint": "https://api.unityedge.io/rest/v1/rpc/rewards_get_allocations",
    "batch_size": 20,
    "delay_between_requests": 1.5,  # seconds - be respectful to the server
    "delay_on_error": 5.0,          # seconds - back off on errors
    "max_retries": 3,
    "target_date": datetime(2005, 8, 1, tzinfo=timezone.utc),
    "output_dir": "/Users/admin/Documents/unetwork/uno-admin/data/rewards",
    "progress_file": "/Users/admin/Documents/unetwork/uno-admin/data/rewards/progress.json",
    "chunk_size": 500,  # Save to new file every N records
}

# Setup logging
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s"
)
logger = logging.getLogger(__name__)


class RewardsFetcher:
    def __init__(self, auth_token: Optional[str] = None):
        self.auth_token = auth_token
        self.session: Optional[aiohttp.ClientSession] = None
        self.total_fetched = 0
        self.current_skip = 0
        self.all_rewards = []
        self.oldest_date_seen: Optional[datetime] = None
        self.chunk_index = 0

        # Ensure output directory exists
        Path(CONFIG["output_dir"]).mkdir(parents=True, exist_ok=True)

    def _get_headers(self) -> dict:
        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json",
        }
        # Supabase requires apikey header
        api_key = os.environ.get("UNITYEDGE_API_KEY")
        if api_key:
            headers["apikey"] = api_key
        if self.auth_token:
            headers["Authorization"] = f"Bearer {self.auth_token}"
        return headers

    def _load_progress(self) -> dict:
        """Load progress from previous run if exists."""
        progress_path = Path(CONFIG["progress_file"])
        if progress_path.exists():
            with open(progress_path, "r") as f:
                progress = json.load(f)
                logger.info(f"Resuming from skip={progress['current_skip']}, "
                          f"total_fetched={progress['total_fetched']}")
                return progress
        return {"current_skip": 0, "total_fetched": 0, "chunk_index": 0}

    def _save_progress(self):
        """Persist current progress for resume capability."""
        progress = {
            "current_skip": self.current_skip,
            "total_fetched": self.total_fetched,
            "chunk_index": self.chunk_index,
            "oldest_date_seen": self.oldest_date_seen.isoformat() if self.oldest_date_seen else None,
            "last_updated": datetime.now(timezone.utc).isoformat()
        }
        with open(CONFIG["progress_file"], "w") as f:
            json.dump(progress, f, indent=2)

    def _save_chunk(self):
        """Save current batch of rewards to a file."""
        if not self.all_rewards:
            return

        filename = f"{CONFIG['output_dir']}/rewards_chunk_{self.chunk_index:04d}.json"
        with open(filename, "w") as f:
            json.dump({
                "chunk_index": self.chunk_index,
                "record_count": len(self.all_rewards),
                "date_range": {
                    "newest": self.all_rewards[0]["completedAt"] if self.all_rewards else None,
                    "oldest": self.all_rewards[-1]["completedAt"] if self.all_rewards else None,
                },
                "records": self.all_rewards
            }, f, indent=2)

        logger.info(f"Saved chunk {self.chunk_index} with {len(self.all_rewards)} records to {filename}")
        self.all_rewards = []
        self.chunk_index += 1

    def _parse_date(self, date_str: str) -> datetime:
        """Parse ISO date string to datetime."""
        date_str = date_str.replace("+00:00", "+0000").replace("Z", "+0000")
        try:
            return datetime.fromisoformat(date_str.replace("+0000", "+00:00"))
        except ValueError:
            return datetime.strptime(date_str[:19], "%Y-%m-%dT%H:%M:%S").replace(tzinfo=timezone.utc)

    async def _fetch_batch(self, skip: int) -> list:
        """Fetch a single batch of rewards with retry logic."""
        payload = {"skip": skip, "take": CONFIG["batch_size"]}

        for attempt in range(CONFIG["max_retries"]):
            try:
                async with self.session.post(
                    CONFIG["endpoint"],
                    json=payload,
                    headers=self._get_headers()
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        return data if isinstance(data, list) else []

                    elif response.status == 429:
                        retry_after = int(response.headers.get("Retry-After", 30))
                        logger.warning(f"Rate limited. Waiting {retry_after}s...")
                        await asyncio.sleep(retry_after)
                        continue

                    else:
                        logger.error(f"HTTP {response.status}: {await response.text()}")
                        await asyncio.sleep(CONFIG["delay_on_error"])

            except aiohttp.ClientError as e:
                logger.error(f"Request error (attempt {attempt + 1}): {e}")
                await asyncio.sleep(CONFIG["delay_on_error"])

        return []

    def _check_target_reached(self, records: list) -> bool:
        """Check if we've reached records from August 2005 or earlier."""
        if not records:
            return True

        oldest_in_batch = records[-1]
        oldest_date = self._parse_date(oldest_in_batch["completedAt"])
        self.oldest_date_seen = oldest_date

        if oldest_date <= CONFIG["target_date"]:
            logger.info(f"Reached target date! Oldest record: {oldest_date.isoformat()}")
            return True

        return False

    async def fetch_all(self):
        """Main fetch loop - incrementally fetches all rewards."""
        progress = self._load_progress()
        self.current_skip = progress["current_skip"]
        self.total_fetched = progress["total_fetched"]
        self.chunk_index = progress["chunk_index"]

        logger.info(f"Starting fetch from skip={self.current_skip}")
        logger.info(f"Target: Records until {CONFIG['target_date'].strftime('%B %Y')}")
        logger.info(f"Output: {CONFIG['output_dir']}")

        ssl_context = ssl.create_default_context(cafile=certifi.where())
        connector = aiohttp.TCPConnector(ssl=ssl_context)
        async with aiohttp.ClientSession(connector=connector) as self.session:
            while True:
                records = await self._fetch_batch(self.current_skip)

                if not records:
                    logger.info("No more records returned. Fetch complete.")
                    break

                self.all_rewards.extend(records)
                self.total_fetched += len(records)
                self.current_skip += CONFIG["batch_size"]

                if records:
                    newest = records[0]["completedAt"][:10]
                    oldest = records[-1]["completedAt"][:10]
                    logger.info(
                        f"Fetched {len(records)} records | "
                        f"Total: {self.total_fetched} | "
                        f"Range: {newest} to {oldest}"
                    )

                if self._check_target_reached(records):
                    break

                if len(self.all_rewards) >= CONFIG["chunk_size"]:
                    self._save_chunk()
                    self._save_progress()

                await asyncio.sleep(CONFIG["delay_between_requests"])

        if self.all_rewards:
            self._save_chunk()

        self._save_progress()
        self._generate_summary()

    def _generate_summary(self):
        """Generate a summary report of all fetched data."""
        summary = {
            "fetch_completed": datetime.now(timezone.utc).isoformat(),
            "total_records": self.total_fetched,
            "total_chunks": self.chunk_index,
            "oldest_date_seen": self.oldest_date_seen.isoformat() if self.oldest_date_seen else None,
            "target_date": CONFIG["target_date"].isoformat(),
        }

        summary_path = f"{CONFIG['output_dir']}/summary.json"
        with open(summary_path, "w") as f:
            json.dump(summary, f, indent=2)

        logger.info("=" * 50)
        logger.info("FETCH COMPLETE")
        logger.info(f"Total records: {self.total_fetched}")
        logger.info(f"Chunks saved: {self.chunk_index}")
        logger.info(f"Output: {CONFIG['output_dir']}")
        logger.info("=" * 50)


async def main():
    auth_token = os.environ.get("UNITYEDGE_AUTH_TOKEN")

    if not auth_token:
        logger.warning("No auth token found. Set UNITYEDGE_AUTH_TOKEN if needed.")

    fetcher = RewardsFetcher(auth_token=auth_token)
    await fetcher.fetch_all()


if __name__ == "__main__":
    asyncio.run(main())
