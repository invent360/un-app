-- R4-04: Rollback settlement items

-- Drop views
DROP VIEW IF EXISTS settlement_summary;
DROP VIEW IF EXISTS unsettled_liabilities;

-- Drop indexes
DROP INDEX IF EXISTS idx_settlement_items_party;
DROP INDEX IF EXISTS idx_settlement_items_recipient;
DROP INDEX IF EXISTS idx_settlement_items_state;
DROP INDEX IF EXISTS idx_settlement_items_allocation;
DROP INDEX IF EXISTS idx_settlement_items_settlement;
DROP INDEX IF EXISTS idx_settlement_idempotency;
DROP INDEX IF EXISTS idx_settlement_item_pending_unique;
DROP INDEX IF EXISTS idx_ledger_ulo_settlement_state;
DROP INDEX IF EXISTS idx_ledger_uno_settlement_state;
DROP INDEX IF EXISTS idx_ledger_referral_settlement_state;

-- Drop settlement columns
ALTER TABLE settlements
DROP COLUMN IF EXISTS item_count,
DROP COLUMN IF EXISTS idempotency_key,
DROP COLUMN IF EXISTS provider_status,
DROP COLUMN IF EXISTS provider_status_at,
DROP COLUMN IF EXISTS reconciliation_notes;

-- Drop allocation columns
ALTER TABLE allocation_ledger
DROP COLUMN IF EXISTS ulo_settlement_state,
DROP COLUMN IF EXISTS uno_settlement_state,
DROP COLUMN IF EXISTS referral_settlement_state,
DROP COLUMN IF EXISTS reserve_settlement_state,
DROP COLUMN IF EXISTS referral_agent_id;

-- Drop settlement items table
DROP TABLE IF EXISTS settlement_items;
