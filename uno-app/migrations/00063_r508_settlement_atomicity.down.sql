-- Migration 00063 DOWN: Revert R5-08 Settlement Atomicity

-- ============================================
-- REMOVE INDEXES
-- ============================================

DROP INDEX IF EXISTS idx_settlements_executed_at;

-- ============================================
-- REMOVE VIEWS
-- ============================================

DROP VIEW IF EXISTS settlement_summary;

-- ============================================
-- REMOVE FUNCTIONS
-- ============================================

DROP FUNCTION IF EXISTS is_allocation_fully_settled(UUID);

-- ============================================
-- REMOVE CONSTRAINTS
-- ============================================

ALTER TABLE settlement_items
DROP CONSTRAINT IF EXISTS chk_settlement_item_fee_bounds;

ALTER TABLE settlement_items
DROP COLUMN IF EXISTS fee_micros;

ALTER TABLE settlement_items
DROP CONSTRAINT IF EXISTS chk_settlement_item_amount_positive;

-- ============================================
-- REMOVE EXCLUSIVITY INDEXES
-- ============================================

DROP INDEX IF EXISTS idx_settlement_items_active_unique;
DROP INDEX IF EXISTS idx_settlements_active;

-- ============================================
-- REMOVE PROVIDER REF COLUMNS AND INDEX
-- ============================================

DROP INDEX IF EXISTS idx_settlements_provider_ref_unique;

ALTER TABLE settlements
DROP COLUMN IF EXISTS executed_by;

ALTER TABLE settlements
DROP COLUMN IF EXISTS executed_at;

ALTER TABLE settlements
DROP COLUMN IF EXISTS provider;

ALTER TABLE settlements
DROP COLUMN IF EXISTS provider_ref;
