//! Durable nonce repository for replay prevention
//!
//! Provides atomic nonce consumption to prevent HMAC request replay attacks.
//! Nonces are stored in PostgreSQL and shared across all worker processes.

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::types::AppError;

/// Repository for managing consumed nonces
#[async_trait]
pub trait NonceRepository: Send + Sync {
    /// Atomically consume a nonce, returning false if already consumed.
    ///
    /// This uses INSERT ... ON CONFLICT DO NOTHING to ensure atomic
    /// check-and-consume semantics. If the nonce was already consumed,
    /// the insert fails silently and we return false.
    async fn consume_nonce(
        &self,
        client_id: &str,
        nonce: &str,
        timestamp: i64,
    ) -> Result<bool, AppError>;

    /// Check if a nonce has been consumed (without consuming it).
    async fn is_nonce_consumed(&self, client_id: &str, nonce: &str) -> Result<bool, AppError>;

    /// Cleanup nonces older than the specified timestamp.
    ///
    /// Returns the number of nonces deleted.
    async fn cleanup_expired(&self, older_than_timestamp: i64) -> Result<i64, AppError>;

    /// Cleanup nonces consumed more than the specified seconds ago.
    ///
    /// Returns the number of nonces deleted.
    async fn cleanup_older_than_secs(&self, older_than_secs: i64) -> Result<i64, AppError>;
}

/// Dynamic type alias for NonceRepository
pub type DynNonceRepository = Arc<dyn NonceRepository>;

/// PostgreSQL implementation of NonceRepository
#[derive(Clone)]
pub struct NonceRepositoryImpl {
    pool: PgPool,
}

impl NonceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NonceRepository for NonceRepositoryImpl {
    async fn consume_nonce(
        &self,
        client_id: &str,
        nonce: &str,
        timestamp: i64,
    ) -> Result<bool, AppError> {
        // Use INSERT ... ON CONFLICT DO NOTHING for atomic check-and-consume
        // The result will have rows_affected = 1 if inserted (new nonce)
        // or rows_affected = 0 if conflict (already consumed)
        let result = sqlx::query(
            r#"
            INSERT INTO consumed_nonces (client_id, nonce, timestamp)
            VALUES ($1, $2, $3)
            ON CONFLICT (client_id, nonce) DO NOTHING
            "#,
        )
        .bind(client_id)
        .bind(nonce)
        .bind(timestamp)
        .execute(&self.pool)
        .await?;

        // rows_affected = 1 means the nonce was new and is now consumed
        // rows_affected = 0 means the nonce was already consumed (conflict)
        Ok(result.rows_affected() == 1)
    }

    async fn is_nonce_consumed(&self, client_id: &str, nonce: &str) -> Result<bool, AppError> {
        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM consumed_nonces
                WHERE client_id = $1 AND nonce = $2
            )
            "#,
        )
        .bind(client_id)
        .bind(nonce)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    async fn cleanup_expired(&self, older_than_timestamp: i64) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM consumed_nonces
            WHERE timestamp < $1
            "#,
        )
        .bind(older_than_timestamp)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    async fn cleanup_older_than_secs(&self, older_than_secs: i64) -> Result<i64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM consumed_nonces
            WHERE consumed_at < NOW() - ($1 || ' seconds')::INTERVAL
            "#,
        )
        .bind(older_than_secs.to_string())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_repository_impl_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NonceRepositoryImpl>();
    }
}
