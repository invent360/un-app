-- Migration 00062 DOWN: Revert R5-07 Finance Authoritative Improvements

-- ============================================
-- REMOVE UPDATED_AT TRIGGER
-- ============================================

DROP TRIGGER IF EXISTS trg_alert_policy_updated ON alert_policies;
DROP FUNCTION IF EXISTS update_alert_policy_timestamp();

-- ============================================
-- REMOVE AUTO-QUARANTINE TRIGGER
-- ============================================

DROP TRIGGER IF EXISTS trg_auto_quarantine ON allocation_ledger;
DROP FUNCTION IF EXISTS auto_quarantine_check();

-- ============================================
-- REMOVE VIEW
-- ============================================

DROP VIEW IF EXISTS allocations_pending_review;

-- ============================================
-- REMOVE FUNCTIONS
-- ============================================

DROP FUNCTION IF EXISTS resolve_quarantine(UUID, allocation_quarantine_status, VARCHAR, TEXT);
DROP FUNCTION IF EXISTS quarantine_allocation(UUID, TEXT, VARCHAR);
DROP FUNCTION IF EXISTS check_alert_policy(VARCHAR, VARCHAR);

-- ============================================
-- REMOVE INDEXES
-- ============================================

DROP INDEX IF EXISTS idx_allocation_batch_id;
DROP INDEX IF EXISTS idx_allocation_reward_source;
DROP INDEX IF EXISTS idx_ledger_batch_event_unique;
DROP INDEX IF EXISTS idx_allocation_quarantine;

-- ============================================
-- REMOVE RECIPIENT TRACKING COLUMNS
-- ============================================

ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS reserve_recipient_id;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS uno_recipient_id;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS ulo_recipient_id;

-- ============================================
-- REMOVE AGREEMENT SNAPSHOT
-- ============================================

ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS agreement_snapshot;

-- Note: Cannot safely remove NOT NULL from agreement_version without knowing original state

-- ============================================
-- REMOVE REWARD SOURCE TRACKING
-- ============================================

ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS is_batch_member;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS reward_batch_id;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS reward_source;

-- ============================================
-- REMOVE QUARANTINE COLUMNS
-- ============================================

ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS quarantine_resolution_notes;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS quarantine_reviewed_at;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS quarantine_reviewed_by;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS quarantine_reason;
ALTER TABLE allocation_ledger DROP COLUMN IF EXISTS quarantine_status;

DROP TYPE IF EXISTS allocation_quarantine_status;

-- ============================================
-- REMOVE ALERT POLICIES TABLE
-- ============================================

DROP INDEX IF EXISTS idx_alert_policies_active;
DROP INDEX IF EXISTS idx_alert_policies_code;
DROP TABLE IF EXISTS alert_policies;

-- ============================================
-- RESTORE CASCADE DELETE (CAUTION)
-- ============================================
-- Note: Restoring CASCADE is potentially dangerous for financial data
-- Only do this if you're certain no financial records need preservation

ALTER TABLE uno_credit_expenditure
DROP CONSTRAINT IF EXISTS uno_credit_expenditure_license_id_fkey;

ALTER TABLE uno_credit_expenditure
ADD CONSTRAINT uno_credit_expenditure_license_id_fkey
FOREIGN KEY (license_id) REFERENCES licenses(id) ON DELETE CASCADE;

ALTER TABLE allocation_ledger
DROP CONSTRAINT IF EXISTS allocation_ledger_license_id_fkey;

ALTER TABLE allocation_ledger
ADD CONSTRAINT allocation_ledger_license_id_fkey
FOREIGN KEY (license_id) REFERENCES licenses(id) ON DELETE CASCADE;
