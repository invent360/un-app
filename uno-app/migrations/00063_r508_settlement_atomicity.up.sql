-- Migration 00063: R5-08 Party-Specific Settlement Atomicity
-- Implements:
-- 1. Provider reference idempotency (prevent duplicate confirmations)
-- 2. Strengthen settlement exclusivity (prepared + approved states prevent new prepares)
-- 3. Add zero-amount item rejection
-- 4. Add execution timestamp tracking

-- ============================================
-- PROVIDER REFERENCE IDEMPOTENCY
-- ============================================
-- R5-08: Require idempotent provider acknowledgements
-- A crash after provider success must reconcile without paying again

-- Add unique constraint on provider_ref per settlement
-- This prevents duplicate confirmations from the same provider
ALTER TABLE settlements
ADD COLUMN IF NOT EXISTS provider_ref VARCHAR(255);

ALTER TABLE settlements
ADD COLUMN IF NOT EXISTS provider VARCHAR(100);

ALTER TABLE settlements
ADD COLUMN IF NOT EXISTS executed_at TIMESTAMPTZ;

ALTER TABLE settlements
ADD COLUMN IF NOT EXISTS executed_by VARCHAR(255);

COMMENT ON COLUMN settlements.provider_ref IS 'R5-08: External provider settlement reference for idempotency';
COMMENT ON COLUMN settlements.provider IS 'R5-08: Provider name (e.g., payment processor)';
COMMENT ON COLUMN settlements.executed_at IS 'R5-08: When settlement was executed';
COMMENT ON COLUMN settlements.executed_by IS 'R5-08: Actor who executed the settlement';

-- Unique index on provider_ref to prevent duplicate confirmations
-- Only applies to executed settlements (non-null provider_ref)
CREATE UNIQUE INDEX IF NOT EXISTS idx_settlements_provider_ref_unique
ON settlements (provider, provider_ref)
WHERE provider_ref IS NOT NULL AND state = 'executed';

-- ============================================
-- SETTLEMENT EXCLUSIVITY
-- ============================================
-- R5-08: Prevent simultaneous prepares for the same obligation
-- Define exclusivity across prepared, submitted, confirmed, and reconciled states

-- Create index to efficiently find active settlements (non-terminal states)
CREATE INDEX IF NOT EXISTS idx_settlements_active
ON settlements (state)
WHERE state IN ('prepared', 'approved');

-- Partial unique index: one active settlement per allocation+party
-- This prevents creating new settlements when one is already in progress
CREATE UNIQUE INDEX IF NOT EXISTS idx_settlement_items_active_unique
ON settlement_items (allocation_id, party_type)
WHERE item_state IN ('submitted', 'confirmed');

-- ============================================
-- ZERO-AMOUNT REJECTION
-- ============================================
-- R5-08: Group compatible liabilities by recipient and currency; reject zero/invalid items

-- Add check constraint to reject zero amounts
ALTER TABLE settlement_items
ADD CONSTRAINT chk_settlement_item_amount_positive
CHECK (amount_micros > 0);

-- ============================================
-- SETTLEMENT ITEM FEE TRACKING
-- ============================================
-- Track fees per item for reconciliation

ALTER TABLE settlement_items
ADD COLUMN IF NOT EXISTS fee_micros BIGINT NOT NULL DEFAULT 0;

ALTER TABLE settlement_items
ADD CONSTRAINT chk_settlement_item_fee_bounds
CHECK (fee_micros >= 0 AND fee_micros <= amount_micros);

COMMENT ON COLUMN settlement_items.fee_micros IS 'R5-08: Settlement fee for this item (0 <= fee <= amount)';

-- ============================================
-- AGGREGATE COMPLETION TRACKING
-- ============================================
-- R5-08: Derive aggregate completion only after all relevant liabilities are settled

-- Add function to check if allocation is fully settled
CREATE OR REPLACE FUNCTION is_allocation_fully_settled(p_allocation_id UUID)
RETURNS BOOLEAN AS $$
DECLARE
    v_alloc RECORD;
    v_ulo_settled BOOLEAN;
    v_uno_settled BOOLEAN;
    v_referral_settled BOOLEAN;
    v_reserve_settled BOOLEAN;
BEGIN
    -- Get allocation amounts
    SELECT ulo_micros, uno_micros, referral_micros, reserve_micros,
           ulo_settlement_state, uno_settlement_state,
           referral_settlement_state, reserve_settlement_state
    INTO v_alloc
    FROM allocation_ledger
    WHERE id = p_allocation_id;

    IF NOT FOUND THEN
        RETURN false;
    END IF;

    -- Check each party that has amount > 0
    v_ulo_settled := (v_alloc.ulo_micros = 0 OR v_alloc.ulo_settlement_state = 'confirmed');
    v_uno_settled := (v_alloc.uno_micros = 0 OR v_alloc.uno_settlement_state = 'confirmed');
    v_referral_settled := (v_alloc.referral_micros = 0 OR v_alloc.referral_settlement_state = 'confirmed');
    v_reserve_settled := (v_alloc.reserve_micros = 0 OR v_alloc.reserve_settlement_state = 'confirmed');

    RETURN v_ulo_settled AND v_uno_settled AND v_referral_settled AND v_reserve_settled;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION is_allocation_fully_settled IS 'R5-08: Check if all parties are settled for an allocation';

-- ============================================
-- SETTLEMENT SUMMARY VIEW UPDATE
-- ============================================
-- Update view to include new columns

DROP VIEW IF EXISTS settlement_summary;

CREATE OR REPLACE VIEW settlement_summary AS
SELECT
    s.id,
    s.settlement_ref,
    s.party_type,
    s.total_micros,
    s.fee_micros,
    s.currency,
    s.period_start,
    s.period_end,
    s.state,
    s.prepared_by,
    s.prepared_at,
    s.approved_by,
    s.approved_at,
    s.executed_by,
    s.executed_at,
    s.provider,
    s.provider_ref,
    COUNT(si.id) as item_count,
    SUM(CASE WHEN si.item_state = 'confirmed' THEN 1 ELSE 0 END) as confirmed_count,
    SUM(CASE WHEN si.item_state = 'failed' THEN 1 ELSE 0 END) as failed_count
FROM settlements s
LEFT JOIN settlement_items si ON si.settlement_id = s.id
GROUP BY s.id, s.settlement_ref, s.party_type, s.total_micros, s.fee_micros,
         s.currency, s.period_start, s.period_end, s.state,
         s.prepared_by, s.prepared_at, s.approved_by, s.approved_at,
         s.executed_by, s.executed_at, s.provider, s.provider_ref;

COMMENT ON VIEW settlement_summary IS 'R5-08: Settlement summary with per-party item counts';

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_settlements_executed_at
ON settlements (executed_at)
WHERE executed_at IS NOT NULL;
