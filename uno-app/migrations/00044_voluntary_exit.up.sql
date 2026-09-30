-- Migration: Voluntary Exit (Phase 7)
-- P7-03, P7-06: Voluntary exit flow, final balance, payout processing

-- Participant exits - tracks voluntary and admin-initiated exits
CREATE TABLE IF NOT EXISTS participant_exits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Participant info
    user_id VARCHAR(255) NOT NULL,
    license_id VARCHAR(66) NOT NULL,

    -- Exit details
    exit_type VARCHAR(20) NOT NULL CHECK (exit_type IN ('voluntary', 'admin', 'system', 'expiry')),
    exit_reason VARCHAR(100),
    exit_details TEXT,

    -- Financial settlement
    final_balance_micros BIGINT NOT NULL DEFAULT 0, -- Amount owed to participant
    pending_earnings_micros BIGINT NOT NULL DEFAULT 0, -- Earnings not yet settled
    deductions_micros BIGINT NOT NULL DEFAULT 0, -- Any deductions (chargebacks, etc.)
    net_payout_micros BIGINT NOT NULL DEFAULT 0, -- Final amount to pay

    -- Payout status
    payout_requested BOOLEAN NOT NULL DEFAULT FALSE,
    payout_status VARCHAR(20) DEFAULT 'pending' CHECK (payout_status IN ('pending', 'processing', 'completed', 'failed', 'cancelled', 'not_required')),
    payout_method VARCHAR(50), -- 'bank_transfer', 'mobile_money', 'crypto', etc.
    payout_details JSONB DEFAULT '{}', -- Payment-specific details
    payout_reference VARCHAR(100), -- External payment reference
    payout_initiated_at TIMESTAMPTZ,
    payout_completed_at TIMESTAMPTZ,
    payout_failed_reason TEXT,

    -- Eligibility check before exit
    has_pending_tasks BOOLEAN NOT NULL DEFAULT FALSE,
    has_pending_support BOOLEAN NOT NULL DEFAULT FALSE,
    cooldown_ends_at TIMESTAMPTZ, -- Minimum holding period

    -- Actor
    requested_by VARCHAR(255) NOT NULL, -- user_id or admin_id
    requested_by_type VARCHAR(20) NOT NULL CHECK (requested_by_type IN ('user', 'agent', 'admin', 'system')),
    processed_by VARCHAR(255),
    processed_by_type VARCHAR(20) CHECK (processed_by_type IN ('agent', 'admin', 'system')),

    -- Status workflow
    status VARCHAR(20) NOT NULL DEFAULT 'requested' CHECK (status IN ('requested', 'pending_payout', 'processing', 'completed', 'cancelled', 'rejected')),
    rejection_reason TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,

    -- Constraints
    UNIQUE(license_id) -- One exit per license
);

-- Exit feedback (optional survey)
CREATE TABLE IF NOT EXISTS exit_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exit_id UUID NOT NULL REFERENCES participant_exits(id) ON DELETE CASCADE,

    -- Feedback questions
    primary_reason VARCHAR(100), -- Main reason for leaving
    satisfaction_score INT CHECK (satisfaction_score BETWEEN 1 AND 5),
    would_recommend BOOLEAN,
    earnings_met_expectations BOOLEAN,
    support_quality_score INT CHECK (support_quality_score BETWEEN 1 AND 5),

    -- Free-form feedback
    comments TEXT,
    improvement_suggestions TEXT,

    -- Metadata
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(exit_id)
);

-- Exit audit log
CREATE TABLE IF NOT EXISTS exit_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exit_id UUID NOT NULL REFERENCES participant_exits(id) ON DELETE CASCADE,

    -- Action details
    action VARCHAR(50) NOT NULL, -- 'requested', 'approved', 'payout_initiated', 'payout_completed', etc.
    old_status VARCHAR(20),
    new_status VARCHAR(20),

    -- Actor
    actor_id VARCHAR(255) NOT NULL,
    actor_type VARCHAR(20) NOT NULL CHECK (actor_type IN ('user', 'agent', 'admin', 'system')),

    -- Details
    details JSONB DEFAULT '{}',
    notes TEXT,

    -- Timestamp
    logged_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Waitlist for unsupported markets (P7-03)
CREATE TABLE IF NOT EXISTS market_waitlist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- User info
    user_id VARCHAR(255),
    email VARCHAR(255) NOT NULL,
    country_code CHAR(2) NOT NULL,

    -- Consent
    consent_given BOOLEAN NOT NULL DEFAULT FALSE,
    consent_given_at TIMESTAMPTZ,
    consent_version VARCHAR(20),

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'waiting' CHECK (status IN ('waiting', 'notified', 'converted', 'unsubscribed')),

    -- Notification
    notified_at TIMESTAMPTZ,
    notification_reference VARCHAR(100),

    -- Source
    source VARCHAR(50), -- 'eligibility_check', 'landing_page', 'referral', etc.
    campaign_id UUID REFERENCES campaign_sources(id) ON DELETE SET NULL,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(email, country_code)
);

-- Indexes
CREATE INDEX idx_exits_user ON participant_exits(user_id);
CREATE INDEX idx_exits_license ON participant_exits(license_id);
CREATE INDEX idx_exits_status ON participant_exits(status);
CREATE INDEX idx_exits_payout_status ON participant_exits(payout_status) WHERE payout_status IN ('pending', 'processing');
CREATE INDEX idx_exits_created ON participant_exits(created_at DESC);
CREATE INDEX idx_exits_type ON participant_exits(exit_type);

CREATE INDEX idx_exit_feedback_exit ON exit_feedback(exit_id);
CREATE INDEX idx_exit_audit_exit ON exit_audit_log(exit_id);
CREATE INDEX idx_exit_audit_action ON exit_audit_log(action, logged_at);

CREATE INDEX idx_waitlist_country ON market_waitlist(country_code);
CREATE INDEX idx_waitlist_email ON market_waitlist(email);
CREATE INDEX idx_waitlist_status ON market_waitlist(status);

-- Trigger for updated_at
CREATE TRIGGER participant_exits_updated
    BEFORE UPDATE ON participant_exits
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

CREATE TRIGGER market_waitlist_updated
    BEFORE UPDATE ON market_waitlist
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

-- Function to calculate final balance for exit
CREATE OR REPLACE FUNCTION calculate_exit_balance(p_user_id VARCHAR(255), p_license_id VARCHAR(66))
RETURNS TABLE (
    final_balance_micros BIGINT,
    pending_earnings_micros BIGINT,
    deductions_micros BIGINT,
    net_payout_micros BIGINT,
    has_pending_tasks BOOLEAN,
    cooldown_ends_at TIMESTAMPTZ
) AS $$
DECLARE
    v_balance BIGINT := 0;
    v_pending BIGINT := 0;
    v_deductions BIGINT := 0;
    v_has_pending BOOLEAN := FALSE;
    v_cooldown TIMESTAMPTZ;
BEGIN
    -- Get settled balance from allocation_ledger
    SELECT COALESCE(SUM(ulo_amount_micros), 0) INTO v_balance
    FROM allocation_ledger
    WHERE license_id = p_license_id
      AND user_id = p_user_id;

    -- Get pending (unsettled) earnings
    -- This would come from pending allocations not yet in ledger
    -- For now, return 0 as pending
    v_pending := 0;

    -- Check for deductions (chargebacks, etc.)
    -- For now, return 0
    v_deductions := 0;

    -- Check for pending tasks
    -- Would check job_queue or similar
    v_has_pending := FALSE;

    -- Calculate cooldown (30 days from claim)
    SELECT issued_at + INTERVAL '30 days' INTO v_cooldown
    FROM licenses
    WHERE id = p_license_id;

    RETURN QUERY SELECT
        v_balance,
        v_pending,
        v_deductions,
        GREATEST(v_balance + v_pending - v_deductions, 0),
        v_has_pending,
        v_cooldown;
END;
$$ LANGUAGE plpgsql;

-- Function to initiate exit
CREATE OR REPLACE FUNCTION initiate_exit(
    p_user_id VARCHAR(255),
    p_license_id VARCHAR(66),
    p_exit_type VARCHAR(20),
    p_reason VARCHAR(100) DEFAULT NULL,
    p_details TEXT DEFAULT NULL,
    p_requested_by VARCHAR(255) DEFAULT NULL,
    p_requested_by_type VARCHAR(20) DEFAULT 'user'
)
RETURNS UUID AS $$
DECLARE
    v_exit_id UUID;
    v_balance RECORD;
BEGIN
    -- Check if exit already exists
    IF EXISTS (SELECT 1 FROM participant_exits WHERE license_id = p_license_id) THEN
        RAISE EXCEPTION 'Exit already initiated for this license';
    END IF;

    -- Calculate balance
    SELECT * INTO v_balance FROM calculate_exit_balance(p_user_id, p_license_id);

    -- Create exit record
    INSERT INTO participant_exits (
        user_id, license_id, exit_type, exit_reason, exit_details,
        final_balance_micros, pending_earnings_micros, deductions_micros, net_payout_micros,
        has_pending_tasks, cooldown_ends_at,
        requested_by, requested_by_type,
        payout_status
    ) VALUES (
        p_user_id, p_license_id, p_exit_type, p_reason, p_details,
        v_balance.final_balance_micros, v_balance.pending_earnings_micros,
        v_balance.deductions_micros, v_balance.net_payout_micros,
        v_balance.has_pending_tasks, v_balance.cooldown_ends_at,
        COALESCE(p_requested_by, p_user_id), p_requested_by_type,
        CASE WHEN v_balance.net_payout_micros > 0 THEN 'pending' ELSE 'not_required' END
    )
    RETURNING id INTO v_exit_id;

    -- Log the action
    INSERT INTO exit_audit_log (exit_id, action, new_status, actor_id, actor_type, details)
    VALUES (v_exit_id, 'requested', 'requested', COALESCE(p_requested_by, p_user_id), p_requested_by_type,
            jsonb_build_object('reason', p_reason, 'balance', v_balance.net_payout_micros));

    RETURN v_exit_id;
END;
$$ LANGUAGE plpgsql;

-- Comments
COMMENT ON TABLE participant_exits IS 'Tracks voluntary and system-initiated participant exits';
COMMENT ON TABLE exit_feedback IS 'Optional feedback survey when participants exit';
COMMENT ON TABLE exit_audit_log IS 'Audit trail for exit process';
COMMENT ON TABLE market_waitlist IS 'Waitlist for users in unsupported markets';
COMMENT ON COLUMN participant_exits.cooldown_ends_at IS 'Minimum holding period before exit is allowed';
COMMENT ON COLUMN participant_exits.net_payout_micros IS 'Final amount to pay after all deductions';
