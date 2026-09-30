-- Allocation Payment State Migration
-- Adds state tracking (accrued -> payable -> paid) to allocation_ledger
-- See: P5-02 in UNO_APP_V2_PHASED_IMPLEMENTATION_PLAN.md

-- ============================================
-- PAYMENT STATE ENUM
-- ============================================

DO $$ BEGIN
    CREATE TYPE allocation_state AS ENUM ('accrued', 'payable', 'paid');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- ADD STATE COLUMNS TO ALLOCATION LEDGER
-- ============================================

ALTER TABLE allocation_ledger
    ADD COLUMN IF NOT EXISTS state allocation_state NOT NULL DEFAULT 'accrued';

ALTER TABLE allocation_ledger
    ADD COLUMN IF NOT EXISTS state_changed_at TIMESTAMPTZ;

ALTER TABLE allocation_ledger
    ADD COLUMN IF NOT EXISTS state_changed_by VARCHAR(66);

ALTER TABLE allocation_ledger
    ADD COLUMN IF NOT EXISTS settlement_ref VARCHAR(255);

-- ============================================
-- INDEXES FOR STATE QUERIES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_ledger_state
    ON allocation_ledger(state);

CREATE INDEX IF NOT EXISTS idx_ledger_state_license
    ON allocation_ledger(license_id, state);

CREATE INDEX IF NOT EXISTS idx_ledger_settlement
    ON allocation_ledger(settlement_ref)
    WHERE settlement_ref IS NOT NULL;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON COLUMN allocation_ledger.state IS 'Payment lifecycle state: accrued (recorded), payable (approved for payment), paid (settled)';
COMMENT ON COLUMN allocation_ledger.state_changed_at IS 'Timestamp when state was last changed';
COMMENT ON COLUMN allocation_ledger.state_changed_by IS 'Actor who changed the state';
COMMENT ON COLUMN allocation_ledger.settlement_ref IS 'Reference to settlement record when paid';
