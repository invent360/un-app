-- Allocation Ledger Migration
-- Tracks revenue allocation per license for reconciliation
-- See: ECO-06 to ECO-08 in UNO_APP_V2_REQUIREMENTS.md

-- ============================================
-- ALLOCATION LEDGER TABLE
-- ============================================
-- Records each revenue allocation event with breakdown by party.
-- All amounts stored in micros (1 USD = 1,000,000 micros).

CREATE TABLE IF NOT EXISTS allocation_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- License reference
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,

    -- Agreement version at time of allocation
    agreement_version INT NOT NULL REFERENCES agreement_versions(version),

    -- Pool amount (pre-split)
    pool_micros BIGINT NOT NULL,
    pool_currency VARCHAR(3) NOT NULL DEFAULT 'USD',

    -- Allocated amounts (post-split)
    ulo_micros BIGINT NOT NULL,      -- User License Owner share
    uno_micros BIGINT NOT NULL,       -- U Network Operator share
    referral_micros BIGINT NOT NULL,  -- Referral share

    -- Split used (basis points)
    ulo_bps INT NOT NULL,
    uno_bps INT NOT NULL,
    referral_bps INT NOT NULL,

    -- Rounding remainder (assigned to UNO)
    remainder_micros BIGINT NOT NULL DEFAULT 0,

    -- Period this allocation covers
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,

    -- Allocation source (e.g., 'daily_sync', 'manual_adjustment')
    source VARCHAR(50) NOT NULL,

    -- External reference (e.g., upstream transaction ID)
    external_ref VARCHAR(255),

    -- Timestamps
    allocated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Reconciliation constraint: amounts must sum to pool
    CONSTRAINT allocation_reconciles CHECK (
        ulo_micros + uno_micros + referral_micros = pool_micros
    ),

    -- Split must sum to 10000 bps
    CONSTRAINT valid_split CHECK (
        ulo_bps + uno_bps + referral_bps = 10000
    )
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_ledger_license ON allocation_ledger(license_id);
CREATE INDEX IF NOT EXISTS idx_ledger_allocated_at ON allocation_ledger(allocated_at);
CREATE INDEX IF NOT EXISTS idx_ledger_period ON allocation_ledger(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_ledger_source ON allocation_ledger(source);
CREATE INDEX IF NOT EXISTS idx_ledger_agreement ON allocation_ledger(agreement_version);

-- ============================================
-- UNO CREDIT EXPENDITURE TABLE
-- ============================================
-- Tracks UNO-funded credit usage per license (ECO-06)
-- Used to calculate actual UNO contribution after deductions.

CREATE TABLE IF NOT EXISTS uno_credit_expenditure (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- License reference
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,

    -- Expenditure amount in micros
    amount_micros BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',

    -- What was purchased (e.g., 'compute_credit', 'storage_credit')
    expenditure_type VARCHAR(50) NOT NULL,

    -- Description of expenditure
    description TEXT,

    -- Period this expenditure covers
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,

    -- Upstream reference
    external_ref VARCHAR(255),

    -- Timestamps
    expended_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_expenditure_license ON uno_credit_expenditure(license_id);
CREATE INDEX IF NOT EXISTS idx_expenditure_type ON uno_credit_expenditure(expenditure_type);
CREATE INDEX IF NOT EXISTS idx_expenditure_period ON uno_credit_expenditure(period_start, period_end);

-- ============================================
-- POOL BALANCE VIEW
-- ============================================
-- Aggregated view for break-even monitoring (ECO-08)
-- Shows UNO contribution after deducting credit expenditure.

CREATE OR REPLACE VIEW pool_balance_summary AS
SELECT
    l.id AS license_id,
    l.split_type,

    -- Total allocations
    COALESCE(SUM(al.pool_micros), 0) AS total_pool_micros,
    COALESCE(SUM(al.ulo_micros), 0) AS total_ulo_micros,
    COALESCE(SUM(al.uno_micros), 0) AS total_uno_allocated_micros,
    COALESCE(SUM(al.referral_micros), 0) AS total_referral_micros,

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

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE allocation_ledger IS 'Records revenue allocations with 50/40/10 split breakdown';
COMMENT ON TABLE uno_credit_expenditure IS 'Tracks UNO-funded credit usage per license';
COMMENT ON VIEW pool_balance_summary IS 'Aggregated pool balance for break-even monitoring';
COMMENT ON COLUMN allocation_ledger.pool_micros IS 'Original pool amount in micros (1 USD = 1,000,000 micros)';
COMMENT ON COLUMN allocation_ledger.remainder_micros IS 'Rounding remainder assigned to UNO';
COMMENT ON COLUMN uno_credit_expenditure.amount_micros IS 'Credit expenditure in micros (deducted from UNO share)';
