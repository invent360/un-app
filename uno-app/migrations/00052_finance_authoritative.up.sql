-- R3-08: Make finance authoritative
-- Adds provider-scoped duplicate prevention and reserve allocation support
-- See: v2_relaunch_implementation_plan.md Gate C

-- ============================================
-- 1. ADD PROVIDER AND EVENT COLUMNS
-- ============================================
-- Enables provider-scoped deduplication of reward events
-- Previously only license_id + external_ref was checked, allowing
-- duplicate events from different providers

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS provider_id VARCHAR(100),
ADD COLUMN IF NOT EXISTS reward_event_id VARCHAR(255);

COMMENT ON COLUMN allocation_ledger.provider_id IS 'Provider identifier (e.g., unetwork, marketplace)';
COMMENT ON COLUMN allocation_ledger.reward_event_id IS 'Unique event ID from provider for deduplication';

-- ============================================
-- 2. ADD RESERVE MICROS COLUMN
-- ============================================
-- R3-07: When no referral, the referral share goes to reserve
-- instead of being added to UNO's share

ALTER TABLE allocation_ledger
ADD COLUMN IF NOT EXISTS reserve_micros BIGINT NOT NULL DEFAULT 0;

COMMENT ON COLUMN allocation_ledger.reserve_micros IS 'Reserve allocation in micros (no-referral case per R3-07)';

-- ============================================
-- 3. UPDATE RECONCILIATION CONSTRAINT
-- ============================================
-- Include reserve_micros in the sum check
-- Must drop and recreate since ALTER CHECK not supported

ALTER TABLE allocation_ledger
DROP CONSTRAINT IF EXISTS allocation_reconciles;

ALTER TABLE allocation_ledger
ADD CONSTRAINT allocation_reconciles CHECK (
    ulo_micros + uno_micros + referral_micros + reserve_micros = pool_micros
);

-- ============================================
-- 4. UNIQUE CONSTRAINT FOR DUPLICATE PREVENTION
-- ============================================
-- Prevent duplicate reward events from the same provider
-- Partial index: only applies when both columns are populated

CREATE UNIQUE INDEX IF NOT EXISTS idx_ledger_provider_event_unique
ON allocation_ledger (provider_id, reward_event_id)
WHERE provider_id IS NOT NULL AND reward_event_id IS NOT NULL;

-- ============================================
-- 5. INDEXES FOR PROVIDER QUERIES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_ledger_provider_id
ON allocation_ledger(provider_id)
WHERE provider_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ledger_reward_event_id
ON allocation_ledger(reward_event_id)
WHERE reward_event_id IS NOT NULL;

-- ============================================
-- 6. UPDATE POOL BALANCE VIEW
-- ============================================
-- Include reserve_micros in the summary view

DROP VIEW IF EXISTS pool_balance_summary;

CREATE VIEW pool_balance_summary AS
SELECT
    l.id AS license_id,
    l.split_type,

    -- Total allocations
    COALESCE(SUM(al.pool_micros), 0) AS total_pool_micros,
    COALESCE(SUM(al.ulo_micros), 0) AS total_ulo_micros,
    COALESCE(SUM(al.uno_micros), 0) AS total_uno_allocated_micros,
    COALESCE(SUM(al.referral_micros), 0) AS total_referral_micros,
    COALESCE(SUM(al.reserve_micros), 0) AS total_reserve_micros,

    -- Credit expenditure
    COALESCE((
        SELECT SUM(ce.amount_micros)
        FROM uno_credit_expenditure ce
        WHERE ce.license_id = l.id
    ), 0) AS total_credit_expenditure_micros,

    -- Net UNO contribution (ECO-07)
    COALESCE(SUM(al.uno_micros), 0) - COALESCE((
        SELECT SUM(ce.amount_micros)
        FROM uno_credit_expenditure ce
        WHERE ce.license_id = l.id
    ), 0) AS net_uno_contribution_micros,

    -- Break-even status (ECO-08)
    -- Alert threshold: $5.60 = 5,600,000 micros
    CASE
        WHEN COALESCE(SUM(al.uno_micros), 0) - COALESCE((
            SELECT SUM(ce.amount_micros)
            FROM uno_credit_expenditure ce
            WHERE ce.license_id = l.id
        ), 0) < 5600000 THEN true
        ELSE false
    END AS below_break_even_threshold

FROM licenses l
LEFT JOIN allocation_ledger al ON al.license_id = l.id
GROUP BY l.id, l.split_type;

COMMENT ON VIEW pool_balance_summary IS 'Aggregated pool balance for break-even monitoring (includes reserve)';
