-- Rollback: Voluntary Exit

DROP TRIGGER IF EXISTS market_waitlist_updated ON market_waitlist;
DROP TRIGGER IF EXISTS participant_exits_updated ON participant_exits;

DROP FUNCTION IF EXISTS initiate_exit(VARCHAR(255), VARCHAR(66), VARCHAR(20), VARCHAR(100), TEXT, VARCHAR(255), VARCHAR(20));
DROP FUNCTION IF EXISTS calculate_exit_balance(VARCHAR(255), VARCHAR(66));

DROP TABLE IF EXISTS market_waitlist;
DROP TABLE IF EXISTS exit_audit_log;
DROP TABLE IF EXISTS exit_feedback;
DROP TABLE IF EXISTS participant_exits;
