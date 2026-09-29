//! Claim cursor for cursor-based pagination
//!
//! Implements composite key (claimed_at, id) cursor pagination for claim sync.
//! Preserves full upstream identifier alongside internal UUID.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Cursor for claim-based pagination using composite key
///
/// Uses `(claimed_at, id)` for deterministic, resumable pagination.
/// This avoids issues with offset-based pagination (missed/duplicate rows
/// when data changes during sync).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimCursor {
    /// Timestamp of the last processed claim
    pub claimed_at: DateTime<Utc>,
    /// UUID of the last processed claim (for tie-breaking)
    pub id: String,
}

impl ClaimCursor {
    /// Create a new cursor from the last processed claim
    pub fn new(claimed_at: DateTime<Utc>, id: impl Into<String>) -> Self {
        Self {
            claimed_at,
            id: id.into(),
        }
    }

    /// Create a cursor from epoch (beginning of time) for initial sync
    pub fn from_epoch() -> Self {
        Self {
            claimed_at: DateTime::UNIX_EPOCH,
            id: String::new(),
        }
    }

    /// Encode cursor to base64 string for API transmission
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(self).unwrap_or_default();
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, json)
    }

    /// Decode cursor from base64 string
    pub fn decode(encoded: &str) -> Option<Self> {
        let bytes = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded,
        )
        .ok()?;
        let json = String::from_utf8(bytes).ok()?;
        serde_json::from_str(&json).ok()
    }

    /// SQL WHERE clause fragment for cursor-based pagination
    ///
    /// Returns a clause that selects rows AFTER this cursor position.
    /// Uses composite key comparison: `(claimed_at, id) > (cursor.claimed_at, cursor.id)`
    pub fn where_clause(&self) -> String {
        format!(
            "(claimed_at, id) > ('{}', '{}')",
            self.claimed_at.format("%Y-%m-%d %H:%M:%S%.6f"),
            self.id
        )
    }

    /// SQL ORDER BY clause for cursor-based pagination
    pub fn order_clause() -> &'static str {
        "claimed_at ASC, id ASC"
    }
}

/// Sync checkpoint for durable pagination state
///
/// Persisted to database only AFTER successful reconciliation.
/// This ensures interrupted syncs can resume from a known-good point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCheckpoint {
    /// Sync job identifier
    pub sync_id: String,
    /// Sync type (e.g., "claims", "rewards", "licenses")
    pub sync_type: String,
    /// Current cursor position (None = start from beginning)
    pub cursor: Option<ClaimCursor>,
    /// Number of records processed so far
    pub records_processed: i64,
    /// Number of records reconciled (matched with upstream)
    pub records_reconciled: i64,
    /// Whether this checkpoint has been verified via reconciliation
    pub is_reconciled: bool,
    /// Timestamp when checkpoint was last updated
    pub updated_at: DateTime<Utc>,
}

impl SyncCheckpoint {
    /// Create a new checkpoint for a sync job
    pub fn new(sync_id: impl Into<String>, sync_type: impl Into<String>) -> Self {
        Self {
            sync_id: sync_id.into(),
            sync_type: sync_type.into(),
            cursor: None,
            records_processed: 0,
            records_reconciled: 0,
            is_reconciled: false,
            updated_at: Utc::now(),
        }
    }

    /// Advance the checkpoint to a new cursor position
    ///
    /// NOTE: This updates in-memory state only. Call `persist()` only after
    /// successful reconciliation to ensure durability.
    pub fn advance(&mut self, cursor: ClaimCursor, records_in_batch: i64) {
        self.cursor = Some(cursor);
        self.records_processed += records_in_batch;
        self.updated_at = Utc::now();
        // is_reconciled stays false until explicitly marked
    }

    /// Mark checkpoint as reconciled
    ///
    /// Called only after verifying records match upstream source.
    pub fn mark_reconciled(&mut self, reconciled_count: i64) {
        self.records_reconciled = reconciled_count;
        self.is_reconciled = true;
        self.updated_at = Utc::now();
    }

    /// Check if checkpoint can be safely persisted
    ///
    /// Only reconciled checkpoints should be persisted to avoid
    /// data loss on interrupted syncs.
    pub fn can_persist(&self) -> bool {
        self.is_reconciled
    }

    /// Serialize to JSON for database storage
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

/// Claim record with upstream identifier preserved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimRecord {
    /// Internal UUID (our system)
    pub internal_id: String,
    /// Upstream identifier (external system)
    pub upstream_id: String,
    /// When the claim was made
    pub claimed_at: DateTime<Utc>,
    /// License ID this claim is for
    pub license_id: String,
    /// Session token used for claim
    pub session_token: Option<String>,
    /// Referral code if any
    pub referral_code: Option<String>,
    /// Whether claim is confirmed
    pub is_confirmed: bool,
}

impl ClaimRecord {
    /// Create cursor pointing after this record
    pub fn as_cursor(&self) -> ClaimCursor {
        ClaimCursor::new(self.claimed_at, &self.internal_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_encode_decode() {
        let cursor = ClaimCursor::new(Utc::now(), "test-id-123");
        let encoded = cursor.encode();
        let decoded = ClaimCursor::decode(&encoded).expect("should decode");
        assert_eq!(cursor.id, decoded.id);
    }

    #[test]
    fn test_cursor_from_epoch() {
        let cursor = ClaimCursor::from_epoch();
        assert_eq!(cursor.claimed_at, DateTime::UNIX_EPOCH);
        assert!(cursor.id.is_empty());
    }

    #[test]
    fn test_cursor_where_clause() {
        let cursor = ClaimCursor::new(
            DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
                .unwrap()
                .with_timezone(&Utc),
            "abc-123",
        );
        let clause = cursor.where_clause();
        assert!(clause.contains("2024-01-15"));
        assert!(clause.contains("abc-123"));
    }

    #[test]
    fn test_checkpoint_advance() {
        let mut checkpoint = SyncCheckpoint::new("job-1", "claims");
        assert_eq!(checkpoint.records_processed, 0);
        assert!(!checkpoint.is_reconciled);

        let cursor = ClaimCursor::new(Utc::now(), "claim-1");
        checkpoint.advance(cursor.clone(), 100);

        assert_eq!(checkpoint.records_processed, 100);
        assert_eq!(checkpoint.cursor.as_ref().unwrap().id, "claim-1");
        assert!(!checkpoint.can_persist());

        checkpoint.mark_reconciled(100);
        assert!(checkpoint.can_persist());
    }

    #[test]
    fn test_checkpoint_json_roundtrip() {
        let mut checkpoint = SyncCheckpoint::new("job-1", "claims");
        checkpoint.advance(ClaimCursor::new(Utc::now(), "test"), 50);
        checkpoint.mark_reconciled(50);

        let json = checkpoint.to_json();
        let restored = SyncCheckpoint::from_json(&json).expect("should parse");

        assert_eq!(restored.sync_id, "job-1");
        assert_eq!(restored.records_processed, 50);
        assert!(restored.is_reconciled);
    }
}
