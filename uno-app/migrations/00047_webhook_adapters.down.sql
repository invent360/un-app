-- Phase 8: Webhook Adapters - Rollback

DROP TRIGGER IF EXISTS trg_webhook_source_configs_updated ON webhook_source_configs;
DROP TRIGGER IF EXISTS trg_webhook_endpoints_updated ON webhook_endpoints;
DROP FUNCTION IF EXISTS update_webhook_timestamp;
DROP FUNCTION IF EXISTS record_delivery_result;
DROP FUNCTION IF EXISTS check_circuit_breaker;
DROP FUNCTION IF EXISTS receive_inbound_webhook;
DROP FUNCTION IF EXISTS queue_webhook_delivery;
DROP TABLE IF EXISTS webhook_delivery_batches;
DROP TABLE IF EXISTS webhook_source_configs;
DROP TABLE IF EXISTS inbound_webhooks;
DROP TABLE IF EXISTS webhook_deliveries;
DROP TABLE IF EXISTS webhook_endpoints;
DROP TYPE IF EXISTS inbound_webhook_status;
DROP TYPE IF EXISTS webhook_delivery_status;
DROP TYPE IF EXISTS webhook_auth_type;

