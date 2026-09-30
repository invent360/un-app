-- Phase 8: Webhook Adapters
-- Migration 00047: Webhook infrastructure for external integrations

-- ============================================
-- WEBHOOK ENDPOINTS
-- ============================================

-- Authentication types for webhooks
CREATE TYPE webhook_auth_type AS ENUM ('none', 'bearer', 'hmac', 'basic', 'api_key');

-- Webhook delivery status
CREATE TYPE webhook_delivery_status AS ENUM ('pending', 'delivered', 'failed', 'skipped', 'dead_letter');

-- Inbound webhook status
CREATE TYPE inbound_webhook_status AS ENUM ('pending', 'processed', 'rejected', 'skipped', 'duplicate');

-- Outbound webhook endpoint configurations
CREATE TABLE webhook_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    endpoint_url TEXT NOT NULL,

    -- Authentication
    auth_type webhook_auth_type NOT NULL DEFAULT 'none',
    auth_header_name VARCHAR(100) DEFAULT 'Authorization',
    auth_secret_encrypted BYTEA, -- encrypted with app key
    auth_secret_version INT NOT NULL DEFAULT 1,

    -- Event filtering
    event_types TEXT[] NOT NULL DEFAULT '{}', -- empty = all events
    country_codes TEXT[], -- NULL = all countries

    -- Request configuration
    http_method VARCHAR(10) NOT NULL DEFAULT 'POST',
    content_type VARCHAR(100) NOT NULL DEFAULT 'application/json',
    custom_headers JSONB DEFAULT '{}',
    payload_template TEXT, -- NULL for default JSON serialization

    -- Status and verification
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMPTZ,
    verification_token VARCHAR(255),
    last_success_at TIMESTAMPTZ,
    last_failure_at TIMESTAMPTZ,
    consecutive_failures INT NOT NULL DEFAULT 0,

    -- Circuit breaker
    circuit_open BOOLEAN NOT NULL DEFAULT FALSE,
    circuit_opened_at TIMESTAMPTZ,
    circuit_failure_threshold INT NOT NULL DEFAULT 5,
    circuit_reset_seconds INT NOT NULL DEFAULT 300,

    -- Retry configuration
    max_retries INT NOT NULL DEFAULT 3,
    retry_delay_seconds INT NOT NULL DEFAULT 60,
    retry_backoff_multiplier DECIMAL(3,1) NOT NULL DEFAULT 2.0,
    timeout_seconds INT NOT NULL DEFAULT 30,

    -- Rate limiting
    rate_limit_per_minute INT, -- NULL for no limit
    rate_limit_burst INT,

    -- Ownership
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webhook_endpoints_enabled ON webhook_endpoints(enabled) WHERE enabled = TRUE;
CREATE INDEX idx_webhook_endpoints_event_types ON webhook_endpoints USING gin(event_types);

-- ============================================
-- WEBHOOK DELIVERIES
-- ============================================

-- Outbound webhook delivery log
CREATE TABLE webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_id UUID NOT NULL REFERENCES webhook_endpoints(id) ON DELETE CASCADE,

    -- Event reference
    event_id UUID NOT NULL, -- reference to outbox_events.id
    event_type VARCHAR(100) NOT NULL,
    event_sequence BIGINT, -- for ordering

    -- Request details
    request_url TEXT NOT NULL,
    request_method VARCHAR(10) NOT NULL,
    request_headers JSONB,
    request_body TEXT,
    request_body_hash VARCHAR(64), -- SHA-256 for deduplication

    -- Delivery status
    status webhook_delivery_status NOT NULL DEFAULT 'pending',
    attempt_count INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 4, -- 1 initial + 3 retries

    -- Response details
    response_status INT,
    response_headers JSONB,
    response_body TEXT,
    response_time_ms INT,

    -- Error tracking
    error_code VARCHAR(50),
    error_message TEXT,

    -- Timing
    scheduled_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    next_retry_at TIMESTAMPTZ,

    -- Idempotency
    idempotency_key VARCHAR(255) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webhook_deliveries_pending ON webhook_deliveries(scheduled_at)
    WHERE status = 'pending';
CREATE INDEX idx_webhook_deliveries_retry ON webhook_deliveries(next_retry_at)
    WHERE status = 'failed' AND next_retry_at IS NOT NULL;
CREATE INDEX idx_webhook_deliveries_endpoint ON webhook_deliveries(endpoint_id, created_at DESC);
CREATE INDEX idx_webhook_deliveries_event ON webhook_deliveries(event_id);
CREATE INDEX idx_webhook_deliveries_idempotency ON webhook_deliveries(idempotency_key);

-- ============================================
-- INBOUND WEBHOOKS
-- ============================================

-- Inbound webhook events (from CRM/external systems)
CREATE TABLE inbound_webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source identification
    source VARCHAR(50) NOT NULL, -- 'highLevel', 'stripe', 'custom', etc.
    source_ip VARCHAR(45),

    -- Event details
    event_type VARCHAR(100) NOT NULL,
    idempotency_key VARCHAR(255) NOT NULL,

    -- Payload
    raw_headers JSONB NOT NULL,
    raw_body TEXT NOT NULL,
    payload JSONB, -- parsed payload

    -- Signature verification
    signature VARCHAR(255),
    signature_valid BOOLEAN,
    signature_algorithm VARCHAR(50),

    -- Processing status
    status inbound_webhook_status NOT NULL DEFAULT 'pending',
    processed_at TIMESTAMPTZ,

    -- Validation results
    validation_errors TEXT[],
    scope_valid BOOLEAN, -- does event match expected scope/permissions

    -- Processing results
    result_entity_type VARCHAR(50),
    result_entity_id VARCHAR(255),
    result_action VARCHAR(50), -- 'created', 'updated', 'ignored', etc.

    -- Error tracking
    error_code VARCHAR(50),
    error_message TEXT,
    retry_count INT NOT NULL DEFAULT 0,
    max_retries INT NOT NULL DEFAULT 3,

    -- Timing
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    next_retry_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_inbound_webhooks_idempotency ON inbound_webhooks(source, idempotency_key);
CREATE INDEX idx_inbound_webhooks_pending ON inbound_webhooks(received_at)
    WHERE status = 'pending';
CREATE INDEX idx_inbound_webhooks_retry ON inbound_webhooks(next_retry_at)
    WHERE status = 'rejected' AND next_retry_at IS NOT NULL AND retry_count < max_retries;
CREATE INDEX idx_inbound_webhooks_source ON inbound_webhooks(source, received_at DESC);

-- ============================================
-- WEBHOOK SOURCE CONFIGURATIONS
-- ============================================

-- Configuration for inbound webhook sources
CREATE TABLE webhook_source_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source VARCHAR(50) NOT NULL UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,

    -- Verification
    signing_secret_encrypted BYTEA,
    signature_header VARCHAR(100), -- e.g., 'X-Hub-Signature-256'
    signature_algorithm VARCHAR(50) DEFAULT 'hmac-sha256',

    -- Allowed IPs (optional whitelist)
    allowed_ips TEXT[],

    -- Event mapping
    event_type_path VARCHAR(255), -- JSON path to extract event type
    idempotency_key_path VARCHAR(255), -- JSON path for idempotency key

    -- Processing configuration
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    auto_process BOOLEAN NOT NULL DEFAULT TRUE,
    require_signature BOOLEAN NOT NULL DEFAULT TRUE,

    -- Rate limiting
    rate_limit_per_minute INT,

    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- WEBHOOK BATCHES (for bulk operations)
-- ============================================

-- Batch webhook deliveries for efficiency
CREATE TABLE webhook_delivery_batches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_id UUID NOT NULL REFERENCES webhook_endpoints(id) ON DELETE CASCADE,

    -- Batch details
    batch_size INT NOT NULL,
    event_types TEXT[] NOT NULL,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'sending', 'completed', 'failed'
    deliveries_succeeded INT NOT NULL DEFAULT 0,
    deliveries_failed INT NOT NULL DEFAULT 0,

    -- Timing
    scheduled_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Error summary
    error_summary JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_webhook_batches_pending ON webhook_delivery_batches(scheduled_at)
    WHERE status = 'pending';

-- ============================================
-- STORED FUNCTIONS
-- ============================================

-- Function to queue a webhook delivery
CREATE OR REPLACE FUNCTION queue_webhook_delivery(
    p_event_id UUID,
    p_event_type VARCHAR(100),
    p_payload JSONB
)
RETURNS INT AS $$
DECLARE
    v_endpoint RECORD;
    v_delivery_count INT := 0;
    v_idempotency_key VARCHAR(255);
BEGIN
    -- Generate idempotency key
    v_idempotency_key := p_event_id::TEXT || ':' || p_event_type;

    -- Find matching enabled endpoints
    FOR v_endpoint IN
        SELECT * FROM webhook_endpoints
        WHERE enabled = TRUE
          AND circuit_open = FALSE
          AND (event_types = '{}' OR p_event_type = ANY(event_types))
    LOOP
        -- Skip if already queued (idempotency)
        IF EXISTS (
            SELECT 1 FROM webhook_deliveries
            WHERE endpoint_id = v_endpoint.id
              AND idempotency_key = v_idempotency_key || ':' || v_endpoint.id::TEXT
        ) THEN
            CONTINUE;
        END IF;

        -- Create delivery record
        INSERT INTO webhook_deliveries (
            endpoint_id,
            event_id,
            event_type,
            request_url,
            request_method,
            request_body,
            request_body_hash,
            scheduled_at,
            idempotency_key
        )
        VALUES (
            v_endpoint.id,
            p_event_id,
            p_event_type,
            v_endpoint.endpoint_url,
            v_endpoint.http_method,
            p_payload::TEXT,
            encode(sha256(p_payload::TEXT::BYTEA), 'hex'),
            NOW(),
            v_idempotency_key || ':' || v_endpoint.id::TEXT
        );

        v_delivery_count := v_delivery_count + 1;
    END LOOP;

    RETURN v_delivery_count;
END;
$$ LANGUAGE plpgsql;

-- Function to process inbound webhook with deduplication
CREATE OR REPLACE FUNCTION receive_inbound_webhook(
    p_source VARCHAR(50),
    p_event_type VARCHAR(100),
    p_idempotency_key VARCHAR(255),
    p_headers JSONB,
    p_body TEXT,
    p_signature VARCHAR(255) DEFAULT NULL,
    p_source_ip VARCHAR(45) DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_webhook_id UUID;
    v_existing_id UUID;
    v_payload JSONB;
BEGIN
    -- Check for existing webhook with same idempotency key
    SELECT id INTO v_existing_id
    FROM inbound_webhooks
    WHERE source = p_source
      AND idempotency_key = p_idempotency_key;

    IF v_existing_id IS NOT NULL THEN
        -- Mark as duplicate and return existing ID
        UPDATE inbound_webhooks
        SET status = 'duplicate'
        WHERE id = v_existing_id
          AND status = 'pending';

        RETURN v_existing_id;
    END IF;

    -- Try to parse JSON payload
    BEGIN
        v_payload := p_body::JSONB;
    EXCEPTION WHEN OTHERS THEN
        v_payload := NULL;
    END;

    -- Create new inbound webhook record
    INSERT INTO inbound_webhooks (
        source,
        source_ip,
        event_type,
        idempotency_key,
        raw_headers,
        raw_body,
        payload,
        signature
    )
    VALUES (
        p_source,
        p_source_ip,
        p_event_type,
        p_idempotency_key,
        p_headers,
        p_body,
        v_payload,
        p_signature
    )
    RETURNING id INTO v_webhook_id;

    RETURN v_webhook_id;
END;
$$ LANGUAGE plpgsql;

-- Function to check and reset circuit breaker
CREATE OR REPLACE FUNCTION check_circuit_breaker(p_endpoint_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    v_endpoint RECORD;
BEGIN
    SELECT * INTO v_endpoint
    FROM webhook_endpoints
    WHERE id = p_endpoint_id;

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    -- If circuit is open, check if reset time has passed
    IF v_endpoint.circuit_open THEN
        IF v_endpoint.circuit_opened_at + (v_endpoint.circuit_reset_seconds || ' seconds')::INTERVAL < NOW() THEN
            -- Reset circuit
            UPDATE webhook_endpoints
            SET circuit_open = FALSE,
                circuit_opened_at = NULL,
                consecutive_failures = 0,
                updated_at = NOW()
            WHERE id = p_endpoint_id;

            RETURN TRUE; -- Circuit is now closed
        END IF;

        RETURN FALSE; -- Circuit still open
    END IF;

    RETURN TRUE; -- Circuit was already closed
END;
$$ LANGUAGE plpgsql;

-- Function to record delivery result and update circuit breaker
CREATE OR REPLACE FUNCTION record_delivery_result(
    p_delivery_id UUID,
    p_success BOOLEAN,
    p_response_status INT DEFAULT NULL,
    p_response_body TEXT DEFAULT NULL,
    p_error_code VARCHAR(50) DEFAULT NULL,
    p_error_message TEXT DEFAULT NULL,
    p_response_time_ms INT DEFAULT NULL
)
RETURNS VOID AS $$
DECLARE
    v_delivery RECORD;
    v_endpoint RECORD;
BEGIN
    -- Get delivery
    SELECT * INTO v_delivery
    FROM webhook_deliveries
    WHERE id = p_delivery_id;

    IF NOT FOUND THEN
        RETURN;
    END IF;

    -- Update delivery record
    IF p_success THEN
        UPDATE webhook_deliveries
        SET status = 'delivered',
            response_status = p_response_status,
            response_body = p_response_body,
            response_time_ms = p_response_time_ms,
            completed_at = NOW()
        WHERE id = p_delivery_id;

        -- Update endpoint success
        UPDATE webhook_endpoints
        SET last_success_at = NOW(),
            consecutive_failures = 0,
            updated_at = NOW()
        WHERE id = v_delivery.endpoint_id;
    ELSE
        -- Increment attempt count
        UPDATE webhook_deliveries
        SET status = CASE
                WHEN attempt_count + 1 >= max_attempts THEN 'dead_letter'::webhook_delivery_status
                ELSE 'failed'::webhook_delivery_status
            END,
            attempt_count = attempt_count + 1,
            response_status = p_response_status,
            response_body = p_response_body,
            error_code = p_error_code,
            error_message = p_error_message,
            response_time_ms = p_response_time_ms,
            next_retry_at = CASE
                WHEN attempt_count + 1 < max_attempts
                THEN NOW() + ((60 * POWER(2, attempt_count)) || ' seconds')::INTERVAL
                ELSE NULL
            END,
            completed_at = CASE
                WHEN attempt_count + 1 >= max_attempts THEN NOW()
                ELSE NULL
            END
        WHERE id = p_delivery_id;

        -- Get endpoint for circuit breaker
        SELECT * INTO v_endpoint
        FROM webhook_endpoints
        WHERE id = v_delivery.endpoint_id;

        -- Update endpoint failures and check circuit breaker
        UPDATE webhook_endpoints
        SET last_failure_at = NOW(),
            consecutive_failures = consecutive_failures + 1,
            circuit_open = CASE
                WHEN consecutive_failures + 1 >= circuit_failure_threshold THEN TRUE
                ELSE circuit_open
            END,
            circuit_opened_at = CASE
                WHEN consecutive_failures + 1 >= circuit_failure_threshold AND NOT circuit_open THEN NOW()
                ELSE circuit_opened_at
            END,
            updated_at = NOW()
        WHERE id = v_delivery.endpoint_id;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- TRIGGERS
-- ============================================

-- Update timestamp trigger for webhook_endpoints
CREATE OR REPLACE FUNCTION update_webhook_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_webhook_endpoints_updated
    BEFORE UPDATE ON webhook_endpoints
    FOR EACH ROW EXECUTE FUNCTION update_webhook_timestamp();

CREATE TRIGGER trg_webhook_source_configs_updated
    BEFORE UPDATE ON webhook_source_configs
    FOR EACH ROW EXECUTE FUNCTION update_webhook_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE webhook_endpoints IS 'Outbound webhook endpoint configurations with circuit breaker';
COMMENT ON TABLE webhook_deliveries IS 'Outbound webhook delivery log with retry tracking';
COMMENT ON TABLE inbound_webhooks IS 'Inbound webhook events from external systems';
COMMENT ON TABLE webhook_source_configs IS 'Configuration for inbound webhook sources';
COMMENT ON TABLE webhook_delivery_batches IS 'Batch webhook deliveries for efficiency';
COMMENT ON FUNCTION queue_webhook_delivery IS 'Queue webhook deliveries for matching endpoints with idempotency';
COMMENT ON FUNCTION receive_inbound_webhook IS 'Receive and deduplicate inbound webhooks';
COMMENT ON FUNCTION check_circuit_breaker IS 'Check and reset circuit breaker for endpoint';
COMMENT ON FUNCTION record_delivery_result IS 'Record delivery result and update circuit breaker state';

