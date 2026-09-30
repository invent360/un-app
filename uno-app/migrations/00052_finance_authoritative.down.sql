-- R3-08: Rollback finance authoritative changes

-- Restore original pool_balance_summary view (without reserve_micros)
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

-- Drop provider indexes
DROP INDEX IF EXISTS idx_ledger_provider_event_unique;
DROP INDEX IF EXISTS idx_ledger_provider_id;
DROP INDEX IF EXISTS idx_ledger_reward_event_id;

-- Restore original reconciliation constraint
ALTER TABLE allocation_ledger
DROP CONSTRAINT IF EXISTS allocation_reconciles;

ALTER TABLE allocation_ledger
ADD CONSTRAINT allocation_reconciles CHECK (
    ulo_micros + uno_micros + referral_micros = pool_micros
);

-- Remove columns (must set reserve_micros = 0 first if data exists)
ALTER TABLE allocation_ledger
DROP COLUMN IF EXISTS reserve_micros,
DROP COLUMN IF EXISTS reward_event_id,
DROP COLUMN IF EXISTS provider_id;
