-- Credit Orders Migration
-- UNO-funded credit order lifecycle with idempotency
-- See: P5-03 in UNO_APP_V2_PHASED_IMPLEMENTATION_PLAN.md

-- ============================================
-- CREDIT ORDER STATE ENUM
-- ============================================

DO $$ BEGIN
    CREATE TYPE credit_order_state AS ENUM (
        'pending',    -- Created, awaiting approval
        'approved',   -- Approved for execution
        'confirmed',  -- Successfully executed
        'failed',     -- Execution failed
        'unknown'     -- Ambiguous outcome, needs reconciliation
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- CREDIT ORDERS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS credit_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- License reference
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,

    -- Order amount
    amount_micros BIGINT NOT NULL CHECK (amount_micros > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',

    -- Period this credit covers
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,

    -- Payer type: 'uno' (platform-funded) or 'license_owner' (user-funded)
    payer_type VARCHAR(50) NOT NULL CHECK (payer_type IN ('uno', 'license_owner')),

    -- Order state
    state credit_order_state NOT NULL DEFAULT 'pending',

    -- Approval details
    approved_by VARCHAR(66),
    approved_at TIMESTAMPTZ,
    approval_notes TEXT,

    -- Provider interaction
    provider VARCHAR(50),
    provider_order_id VARCHAR(255),

    -- Idempotency key for duplicate prevention
    idempotency_key VARCHAR(255) UNIQUE NOT NULL,

    -- Outcome tracking
    confirmed_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    failure_reason TEXT,
    retry_count INT NOT NULL DEFAULT 0,
    last_retry_at TIMESTAMPTZ,

    -- Reconciliation
    reconciled_at TIMESTAMPTZ,
    reconciled_by VARCHAR(66),
    reconciliation_notes TEXT,

    -- Audit timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_credit_orders_license
    ON credit_orders(license_id);

CREATE INDEX IF NOT EXISTS idx_credit_orders_state
    ON credit_orders(state);

CREATE INDEX IF NOT EXISTS idx_credit_orders_payer
    ON credit_orders(payer_type);

CREATE INDEX IF NOT EXISTS idx_credit_orders_period
    ON credit_orders(period_start, period_end);

CREATE INDEX IF NOT EXISTS idx_credit_orders_created
    ON credit_orders(created_at);

CREATE INDEX IF NOT EXISTS idx_credit_orders_unknown
    ON credit_orders(state, last_retry_at)
    WHERE state = 'unknown';

-- ============================================
-- SETTLEMENT RECORDS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS settlements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Settlement identifiers
    settlement_ref VARCHAR(255) UNIQUE NOT NULL,

    -- Settlement scope
    party_type VARCHAR(50) NOT NULL CHECK (party_type IN ('ulo', 'uno', 'referral')),

    -- Amounts
    total_micros BIGINT NOT NULL CHECK (total_micros >= 0),
    fee_micros BIGINT NOT NULL DEFAULT 0 CHECK (fee_micros >= 0),
    net_micros BIGINT NOT NULL CHECK (net_micros >= 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',

    -- Period covered
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,

    -- Allocation IDs included in this settlement
    allocation_ids UUID[] NOT NULL,
    allocation_count INT NOT NULL,

    -- Approval
    prepared_by VARCHAR(66) NOT NULL,
    prepared_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approved_by VARCHAR(66),
    approved_at TIMESTAMPTZ,

    -- Execution
    executed_at TIMESTAMPTZ,
    executed_by VARCHAR(66),
    provider VARCHAR(50),
    provider_ref VARCHAR(255),

    -- Status
    state VARCHAR(50) NOT NULL DEFAULT 'prepared' CHECK (state IN ('prepared', 'approved', 'executed', 'failed', 'reconciled')),

    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- SETTLEMENT INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_settlements_party
    ON settlements(party_type);

CREATE INDEX IF NOT EXISTS idx_settlements_state
    ON settlements(state);

CREATE INDEX IF NOT EXISTS idx_settlements_period
    ON settlements(period_start, period_end);

-- ============================================
-- TRIGGER: Update updated_at
-- ============================================

CREATE OR REPLACE FUNCTION update_credit_order_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_credit_order_timestamp ON credit_orders;
CREATE TRIGGER trigger_credit_order_timestamp
    BEFORE UPDATE ON credit_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_credit_order_timestamp();

DROP TRIGGER IF EXISTS trigger_settlement_timestamp ON settlements;
CREATE TRIGGER trigger_settlement_timestamp
    BEFORE UPDATE ON settlements
    FOR EACH ROW
    EXECUTE FUNCTION update_credit_order_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE credit_orders IS 'UNO-funded credit order lifecycle with idempotency';
COMMENT ON COLUMN credit_orders.idempotency_key IS 'Unique key for duplicate prevention across retries';
COMMENT ON COLUMN credit_orders.payer_type IS 'Source of funds: uno (platform) or license_owner (user)';
COMMENT ON COLUMN credit_orders.state IS 'Order lifecycle: pending -> approved -> confirmed|failed|unknown';

COMMENT ON TABLE settlements IS 'Settlement records for payable allocations';
COMMENT ON COLUMN settlements.party_type IS 'Recipient party: ulo (user), uno (platform), or referral';
COMMENT ON COLUMN settlements.allocation_ids IS 'Array of allocation_ledger IDs included in this settlement';
