-- R4-04: Settlement items for per-recipient/per-party settlement
-- Immutable records preventing duplicate settlements of the same liability
-- See: v2_relaunch_implementation_plan.md Gate C

-- ============================================
-- 1. SETTLEMENT ITEMS TABLE
-- ============================================
-- Each row represents one allocation+party liability in a settlement
-- Immutable after creation; state changes tracked separately

CREATE TABLE IF NOT EXISTS settlement_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Settlement reference
    settlement_id UUID NOT NULL REFERENCES settlements(id) ON DELETE CASCADE,

    -- Allocation reference
    allocation_id UUID NOT NULL REFERENCES allocation_ledger(id) ON DELETE RESTRICT,

    -- Party and recipient details
    party_type VARCHAR(50) NOT NULL,  -- 'ulo', 'uno', 'referral', 'reserve'
    recipient_id VARCHAR(255),        -- license_id, referral_agent_id, or 'uno_treasury'
    recipient_type VARCHAR(50),       -- 'license_owner', 'referral_agent', 'uno_treasury', 'reserve_fund'

    -- Settlement amount (copied from allocation at settlement time for immutability)
    amount_micros BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',

    -- Item state (separate from settlement state for per-item tracking)
    item_state VARCHAR(50) NOT NULL DEFAULT 'submitted',
    -- submitted: included in settlement
    -- confirmed: provider confirmed payment
    -- failed: provider rejected
    -- reconciled: resolved after timeout

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT valid_party_type CHECK (
        party_type IN ('ulo', 'uno', 'referral', 'reserve')
    ),
    CONSTRAINT valid_item_state CHECK (
        item_state IN ('submitted', 'confirmed', 'failed', 'reconciled')
    ),
    CONSTRAINT amount_nonnegative CHECK (amount_micros >= 0),

    -- R4-04: Prevent same allocation+party from being settled twice
    -- A settlement can include an allocation only once per party type
    UNIQUE(allocation_id, party_type, settlement_id)
);

-- Prevent settling the same allocation+party in multiple settlements
-- Only one settlement can have 'submitted' or 'confirmed' state for a given allocation+party
CREATE UNIQUE INDEX IF NOT EXISTS idx_settlement_item_pending_unique
ON settlement_items (allocation_id, party_type)
WHERE item_state IN ('submitted', 'confirmed');

-- ============================================
-- 2. PER-PARTY STATE COLUMNS IN ALLOCATION LEDGER
-- ============================================
-- Track settlement state independently for each party

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS ulo_settlement_state VARCHAR(50) DEFAULT 'unsettled',
ADD COLUMN IF NOT EXISTS uno_settlement_state VARCHAR(50) DEFAULT 'unsettled',
ADD COLUMN IF NOT EXISTS referral_settlement_state VARCHAR(50) DEFAULT 'unsettled',
ADD COLUMN IF NOT EXISTS reserve_settlement_state VARCHAR(50) DEFAULT 'unsettled';

-- States: unsettled -> submitted -> confirmed -> reconciled/failed

COMMENT ON COLUMN allocation_ledger.ulo_settlement_state IS 'ULO party settlement state';
COMMENT ON COLUMN allocation_ledger.uno_settlement_state IS 'UNO party settlement state';
COMMENT ON COLUMN allocation_ledger.referral_settlement_state IS 'Referral party settlement state';
COMMENT ON COLUMN allocation_ledger.reserve_settlement_state IS 'Reserve party settlement state (no-referral case)';

-- ============================================
-- 3. REFERRAL AGENT TRACKING
-- ============================================
-- Link allocation referral share to specific agent

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS referral_agent_id UUID;

COMMENT ON COLUMN allocation_ledger.referral_agent_id IS 'Agent receiving referral share (if any)';

-- ============================================
-- 4. SETTLEMENT ENHANCEMENTS
-- ============================================

ALTER TABLE settlements
ADD COLUMN IF NOT EXISTS item_count INT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS idempotency_key VARCHAR(255),
ADD COLUMN IF NOT EXISTS provider_status VARCHAR(50),
ADD COLUMN IF NOT EXISTS provider_status_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS reconciliation_notes TEXT;

-- Idempotency key for retries
CREATE UNIQUE INDEX IF NOT EXISTS idx_settlement_idempotency
ON settlements (idempotency_key)
WHERE idempotency_key IS NOT NULL;

-- ============================================
-- 5. INDEXES FOR SETTLEMENT ITEMS
-- ============================================

CREATE INDEX IF NOT EXISTS idx_settlement_items_settlement
ON settlement_items(settlement_id);

CREATE INDEX IF NOT EXISTS idx_settlement_items_allocation
ON settlement_items(allocation_id);

CREATE INDEX IF NOT EXISTS idx_settlement_items_state
ON settlement_items(item_state);

CREATE INDEX IF NOT EXISTS idx_settlement_items_recipient
ON settlement_items(recipient_id)
WHERE recipient_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_settlement_items_party
ON settlement_items(party_type);

-- ============================================
-- 6. PER-PARTY SETTLEMENT STATE INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_ledger_ulo_settlement_state
ON allocation_ledger(ulo_settlement_state)
WHERE ulo_settlement_state != 'unsettled';

CREATE INDEX IF NOT EXISTS idx_ledger_uno_settlement_state
ON allocation_ledger(uno_settlement_state)
WHERE uno_settlement_state != 'unsettled';

CREATE INDEX IF NOT EXISTS idx_ledger_referral_settlement_state
ON allocation_ledger(referral_settlement_state)
WHERE referral_settlement_state != 'unsettled';

-- ============================================
-- 7. VIEWS FOR SETTLEMENT AGGREGATION
-- ============================================

-- View: Unsettled liabilities by party
CREATE OR REPLACE VIEW unsettled_liabilities AS
SELECT
    al.id AS allocation_id,
    al.license_id,
    al.referral_agent_id,
    al.pool_currency AS currency,

    -- ULO liability
    CASE WHEN al.ulo_settlement_state = 'unsettled' THEN al.ulo_micros ELSE 0 END AS ulo_unsettled_micros,
    al.ulo_settlement_state,

    -- UNO liability
    CASE WHEN al.uno_settlement_state = 'unsettled' THEN al.uno_micros ELSE 0 END AS uno_unsettled_micros,
    al.uno_settlement_state,

    -- Referral liability
    CASE WHEN al.referral_settlement_state = 'unsettled' AND al.referral_micros > 0 THEN al.referral_micros ELSE 0 END AS referral_unsettled_micros,
    al.referral_settlement_state,

    -- Reserve liability (R3-07)
    CASE WHEN al.reserve_settlement_state = 'unsettled' AND al.reserve_micros > 0 THEN al.reserve_micros ELSE 0 END AS reserve_unsettled_micros,
    al.reserve_settlement_state,

    al.period_start,
    al.period_end,
    al.allocated_at

FROM allocation_ledger al
WHERE al.state = 'payable'
  AND (
      al.ulo_settlement_state = 'unsettled'
      OR al.uno_settlement_state = 'unsettled'
      OR (al.referral_settlement_state = 'unsettled' AND al.referral_micros > 0)
      OR (al.reserve_settlement_state = 'unsettled' AND al.reserve_micros > 0)
  );

-- View: Settlement item summary
CREATE OR REPLACE VIEW settlement_summary AS
SELECT
    s.id AS settlement_id,
    s.settlement_ref,
    s.party_type,
    s.state AS settlement_state,
    COUNT(si.id) AS item_count,
    SUM(si.amount_micros) AS total_amount_micros,
    SUM(CASE WHEN si.item_state = 'confirmed' THEN si.amount_micros ELSE 0 END) AS confirmed_micros,
    SUM(CASE WHEN si.item_state = 'submitted' THEN si.amount_micros ELSE 0 END) AS pending_micros,
    SUM(CASE WHEN si.item_state = 'failed' THEN si.amount_micros ELSE 0 END) AS failed_micros,
    COUNT(DISTINCT si.recipient_id) AS recipient_count,
    s.period_start,
    s.period_end,
    s.created_at
FROM settlements s
LEFT JOIN settlement_items si ON si.settlement_id = s.id
GROUP BY s.id, s.settlement_ref, s.party_type, s.state, s.period_start, s.period_end, s.created_at;

-- ============================================
-- 8. COMMENTS
-- ============================================

COMMENT ON TABLE settlement_items IS 'R4-04: Immutable settlement item records for per-recipient, per-party settlement';
COMMENT ON VIEW unsettled_liabilities IS 'Allocations with unsettled party liabilities';
COMMENT ON VIEW settlement_summary IS 'Aggregated settlement statistics by settlement';
