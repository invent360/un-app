-- Durable nonce consumption for replay prevention
-- Replaces in-memory nonce registry with database-backed storage
-- Shared across all workers for distributed replay protection

CREATE TABLE IF NOT EXISTS consumed_nonces (
    -- Composite primary key for efficient lookups
    client_id VARCHAR(100) NOT NULL,
    nonce VARCHAR(64) NOT NULL,

    -- Timestamp from the signed request (for window validation)
    timestamp BIGINT NOT NULL,

    -- When the nonce was consumed (for cleanup)
    consumed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (client_id, nonce)
);

-- Index for cleanup of expired nonces (older than retention window)
CREATE INDEX IF NOT EXISTS idx_consumed_nonces_timestamp
ON consumed_nonces (timestamp);

-- Index for cleanup by consumed_at (for time-based retention)
CREATE INDEX IF NOT EXISTS idx_consumed_nonces_consumed_at
ON consumed_nonces (consumed_at);

-- Comment for documentation
COMMENT ON TABLE consumed_nonces IS 'Tracks consumed nonces for HMAC request replay prevention. Nonces older than 5 minutes can be cleaned up.';
