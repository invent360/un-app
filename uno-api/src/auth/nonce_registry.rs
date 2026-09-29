//! Nonce registry for replay attack prevention.
//!
//! Tracks used nonces to prevent request replay attacks within the
//! validity window. Nonces are automatically cleaned up after expiry.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::error::AuthError;

/// Registry for tracking used nonces.
///
/// Thread-safe nonce tracking with automatic expiry cleanup.
/// Nonces are kept for slightly longer than max_age_secs to handle
/// edge cases at the boundary.
pub struct NonceRegistry {
    /// Map of nonce -> (client_id, timestamp)
    nonces: RwLock<HashMap<String, (String, i64)>>,
    /// How long to keep nonces (should match max_age_secs + buffer)
    retention_secs: i64,
}

impl NonceRegistry {
    /// Create a new nonce registry.
    ///
    /// `retention_secs` should be at least max_age_secs + some buffer
    /// to ensure nonces aren't pruned before they're no longer valid.
    pub fn new(retention_secs: i64) -> Self {
        Self {
            nonces: RwLock::new(HashMap::new()),
            retention_secs,
        }
    }

    /// Create with default retention (10 minutes).
    pub fn with_default_retention() -> Self {
        Self::new(600)
    }

    /// Check if a nonce has been used and register it if not.
    ///
    /// Returns Ok(()) if the nonce is fresh (and registers it).
    /// Returns Err(NonceReused) if the nonce was already used.
    ///
    /// This is atomic - the check and registration happen together.
    pub fn check_and_register(
        &self,
        client_id: &str,
        nonce: &str,
        timestamp: i64,
    ) -> Result<(), AuthError> {
        // Compose key: client_id:nonce to allow same nonce from different clients
        let key = format!("{}:{}", client_id, nonce);

        let mut nonces = self.nonces.write().expect("nonce registry lock poisoned");

        // Clean up expired entries while we have the lock
        let now = chrono::Utc::now().timestamp();
        let cutoff = now - self.retention_secs;
        nonces.retain(|_, (_, ts)| *ts > cutoff);

        // Check if nonce exists
        if nonces.contains_key(&key) {
            return Err(AuthError::NonceReused);
        }

        // Register the nonce
        nonces.insert(key, (client_id.to_string(), timestamp));

        Ok(())
    }

    /// Get the number of tracked nonces (for monitoring).
    pub fn len(&self) -> usize {
        self.nonces.read().expect("nonce registry lock poisoned").len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Force cleanup of expired nonces.
    ///
    /// This is called automatically during check_and_register,
    /// but can be called manually for maintenance.
    pub fn cleanup_expired(&self) {
        let mut nonces = self.nonces.write().expect("nonce registry lock poisoned");
        let now = chrono::Utc::now().timestamp();
        let cutoff = now - self.retention_secs;
        nonces.retain(|_, (_, ts)| *ts > cutoff);
    }
}

impl Default for NonceRegistry {
    fn default() -> Self {
        Self::with_default_retention()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_nonce_accepted() {
        let registry = NonceRegistry::new(300);
        let now = chrono::Utc::now().timestamp();

        let result = registry.check_and_register("client1", "nonce1", now);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_nonce_rejected() {
        let registry = NonceRegistry::new(300);
        let now = chrono::Utc::now().timestamp();

        // First use succeeds
        assert!(registry.check_and_register("client1", "nonce1", now).is_ok());

        // Second use fails
        let result = registry.check_and_register("client1", "nonce1", now);
        assert!(matches!(result, Err(AuthError::NonceReused)));
    }

    #[test]
    fn test_same_nonce_different_clients_allowed() {
        let registry = NonceRegistry::new(300);
        let now = chrono::Utc::now().timestamp();

        // Same nonce from different clients is allowed
        assert!(registry.check_and_register("client1", "shared_nonce", now).is_ok());
        assert!(registry.check_and_register("client2", "shared_nonce", now).is_ok());
    }

    #[test]
    fn test_expired_nonce_cleaned_up() {
        let registry = NonceRegistry::new(10); // 10 second retention
        let old_timestamp = chrono::Utc::now().timestamp() - 20; // 20 seconds ago

        // Register with old timestamp
        assert!(registry.check_and_register("client1", "old_nonce", old_timestamp).is_ok());

        // Trigger cleanup by registering another nonce
        let now = chrono::Utc::now().timestamp();
        assert!(registry.check_and_register("client1", "new_nonce", now).is_ok());

        // Old nonce should be cleaned up, so same nonce should be accepted
        // (This would only happen in edge case where client reuses nonce after expiry)
        assert!(registry.check_and_register("client1", "old_nonce", now).is_ok());
    }

    #[test]
    fn test_len_tracking() {
        let registry = NonceRegistry::new(300);
        let now = chrono::Utc::now().timestamp();

        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());

        registry.check_and_register("client1", "nonce1", now).ok();
        assert_eq!(registry.len(), 1);

        registry.check_and_register("client1", "nonce2", now).ok();
        assert_eq!(registry.len(), 2);

        // Duplicate doesn't increase count
        registry.check_and_register("client1", "nonce1", now).ok();
        assert_eq!(registry.len(), 2);
    }
}
