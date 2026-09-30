-- Rollback credit orders migration

DROP TRIGGER IF EXISTS trigger_settlement_timestamp ON settlements;
DROP TRIGGER IF EXISTS trigger_credit_order_timestamp ON credit_orders;
DROP FUNCTION IF EXISTS update_credit_order_timestamp();

DROP TABLE IF EXISTS settlements;
DROP TABLE IF EXISTS credit_orders;
DROP TYPE IF EXISTS credit_order_state;
