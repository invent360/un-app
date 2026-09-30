-- Rollback allocation payment state migration

DROP INDEX IF EXISTS idx_ledger_settlement;
DROP INDEX IF EXISTS idx_ledger_state_license;
DROP INDEX IF EXISTS idx_ledger_state;

ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS settlement_ref;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS state_changed_by;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS state_changed_at;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS state;

DROP TYPE IF EXISTS allocation_state;
