-- Phase 8: Operator Metrics and Exception Tracking
-- Migration 00045: Operator dashboard metrics and exception monitoring

-- ============================================
-- OPERATOR METRICS SNAPSHOTS
-- ============================================

-- Daily aggregated metrics for operator dashboard
CREATE TABLE operator_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    metric_date DATE NOT NULL,
    country_code CHAR(2), -- NULL for global metrics

    -- Inventory metrics
    total_licenses INT NOT NULL DEFAULT 0,
    published_licenses INT NOT NULL DEFAULT 0,
    reserved_licenses INT NOT NULL DEFAULT 0,
    issued_licenses INT NOT NULL DEFAULT 0,
    expired_licenses INT NOT NULL DEFAULT 0,
    quarantined_licenses INT NOT NULL DEFAULT 0,

    -- Cohort metrics
    new_cohorts INT NOT NULL DEFAULT 0,
    d1_completions INT NOT NULL DEFAULT 0,
    d3_completions INT NOT NULL DEFAULT 0,
    d7_completions INT NOT NULL DEFAULT 0,
    d30_completions INT NOT NULL DEFAULT 0,
    d7_completion_rate DECIMAL(5,4), -- 0.0000 to 1.0000
    d30_completion_rate DECIMAL(5,4),
    churned_count INT NOT NULL DEFAULT 0,

    -- Financial metrics (in micros)
    total_rewards_micros BIGINT NOT NULL DEFAULT 0,
    total_allocated_micros BIGINT NOT NULL DEFAULT 0,
    participant_allocated_micros BIGINT NOT NULL DEFAULT 0,
    referral_allocated_micros BIGINT NOT NULL DEFAULT 0,
    uno_allocated_micros BIGINT NOT NULL DEFAULT 0,
    total_payable_micros BIGINT NOT NULL DEFAULT 0,
    total_paid_micros BIGINT NOT NULL DEFAULT 0,

    -- Support metrics
    open_tickets INT NOT NULL DEFAULT 0,
    new_tickets INT NOT NULL DEFAULT 0,
    resolved_tickets INT NOT NULL DEFAULT 0,
    escalated_tickets INT NOT NULL DEFAULT 0,
    avg_resolution_hours DECIMAL(8,2),
    avg_first_response_hours DECIMAL(8,2),

    -- Sync/Integration metrics
    last_sync_at TIMESTAMPTZ,
    sync_lag_seconds INT,
    pending_outbox_events INT NOT NULL DEFAULT 0,
    pending_inbox_events INT NOT NULL DEFAULT 0,
    failed_jobs INT NOT NULL DEFAULT 0,
    dead_letter_jobs INT NOT NULL DEFAULT 0,

    -- Agent metrics
    active_agents INT NOT NULL DEFAULT 0,
    pending_agent_approvals INT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(metric_date, country_code)
);

CREATE INDEX idx_operator_metrics_date ON operator_metrics(metric_date DESC);
CREATE INDEX idx_operator_metrics_country ON operator_metrics(country_code, metric_date DESC);

-- ============================================
-- EXCEPTION TRACKING
-- ============================================

-- Severity levels for exceptions
CREATE TYPE exception_severity AS ENUM ('info', 'warning', 'error', 'critical');

-- Exception tracking for operator visibility
CREATE TABLE operator_exceptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exception_type VARCHAR(50) NOT NULL, -- 'sync_failure', 'payment_error', 'quota_exceeded', etc.
    severity exception_severity NOT NULL DEFAULT 'warning',
    country_code CHAR(2),

    -- Entity reference (optional)
    entity_type VARCHAR(50), -- 'license', 'user', 'agent', 'ticket', etc.
    entity_id VARCHAR(100),

    -- Exception details
    message TEXT NOT NULL,
    details JSONB,
    stack_trace TEXT,

    -- Source information
    source_service VARCHAR(50), -- 'sync', 'payment', 'webhook', etc.
    correlation_id VARCHAR(100),

    -- Resolution
    resolved_at TIMESTAMPTZ,
    resolved_by VARCHAR(255),
    resolution_notes TEXT,
    auto_resolved BOOLEAN NOT NULL DEFAULT FALSE,

    -- Occurrence tracking
    occurrence_count INT NOT NULL DEFAULT 1,
    first_occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_operator_exceptions_unresolved ON operator_exceptions(created_at DESC)
    WHERE resolved_at IS NULL;
CREATE INDEX idx_operator_exceptions_severity ON operator_exceptions(severity, created_at DESC)
    WHERE resolved_at IS NULL;
CREATE INDEX idx_operator_exceptions_type ON operator_exceptions(exception_type, created_at DESC);
CREATE INDEX idx_operator_exceptions_entity ON operator_exceptions(entity_type, entity_id)
    WHERE entity_id IS NOT NULL;
CREATE INDEX idx_operator_exceptions_correlation ON operator_exceptions(correlation_id)
    WHERE correlation_id IS NOT NULL;

-- ============================================
-- MARGIN TRACKING
-- ============================================

-- Track profit margins by country/task
CREATE TABLE operator_margins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    country_code CHAR(2) NOT NULL,
    task_code VARCHAR(50),

    -- Revenue
    gross_revenue_micros BIGINT NOT NULL DEFAULT 0,

    -- Shares (from allocations)
    participant_share_micros BIGINT NOT NULL DEFAULT 0,
    referral_share_micros BIGINT NOT NULL DEFAULT 0,
    uno_share_micros BIGINT NOT NULL DEFAULT 0,

    -- Costs
    acquisition_cost_micros BIGINT NOT NULL DEFAULT 0,
    support_cost_micros BIGINT NOT NULL DEFAULT 0,
    hosting_cost_micros BIGINT NOT NULL DEFAULT 0,
    messaging_cost_micros BIGINT NOT NULL DEFAULT 0,
    other_cost_micros BIGINT NOT NULL DEFAULT 0,

    -- Derived
    total_cost_micros BIGINT GENERATED ALWAYS AS (
        acquisition_cost_micros + support_cost_micros + hosting_cost_micros +
        messaging_cost_micros + other_cost_micros
    ) STORED,
    net_margin_micros BIGINT GENERATED ALWAYS AS (
        uno_share_micros - (acquisition_cost_micros + support_cost_micros +
        hosting_cost_micros + messaging_cost_micros + other_cost_micros)
    ) STORED,
    margin_percentage DECIMAL(5,2),

    -- Counts
    active_licenses INT NOT NULL DEFAULT 0,
    productive_licenses INT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(period_start, period_end, country_code, task_code)
);

CREATE INDEX idx_operator_margins_period ON operator_margins(period_start, period_end);
CREATE INDEX idx_operator_margins_country ON operator_margins(country_code, period_start);

-- ============================================
-- STORED FUNCTIONS
-- ============================================

-- Function to aggregate daily metrics
CREATE OR REPLACE FUNCTION aggregate_daily_metrics(p_date DATE, p_country_code CHAR(2) DEFAULT NULL)
RETURNS UUID AS $$
DECLARE
    v_metric_id UUID;
    v_inventory RECORD;
    v_cohort RECORD;
    v_finance RECORD;
    v_support RECORD;
    v_sync RECORD;
BEGIN
    -- Calculate inventory metrics
    SELECT
        COUNT(*) FILTER (WHERE TRUE) as total,
        COUNT(*) FILTER (WHERE status = 'published') as published,
        COUNT(*) FILTER (WHERE status = 'reserved') as reserved,
        COUNT(*) FILTER (WHERE status = 'issued') as issued,
        COUNT(*) FILTER (WHERE status = 'expired') as expired,
        COUNT(*) FILTER (WHERE status = 'quarantined') as quarantined
    INTO v_inventory
    FROM licenses
    WHERE (p_country_code IS NULL OR country_code = p_country_code);

    -- Calculate cohort metrics
    SELECT
        COUNT(*) FILTER (WHERE cohort_date = p_date) as new_cohorts,
        COUNT(*) FILTER (WHERE d1_completed AND d1_completed_at::DATE = p_date) as d1,
        COUNT(*) FILTER (WHERE d3_completed AND d3_completed_at::DATE = p_date) as d3,
        COUNT(*) FILTER (WHERE d7_completed AND d7_completed_at::DATE = p_date) as d7,
        COUNT(*) FILTER (WHERE d30_completed AND d30_completed_at::DATE = p_date) as d30,
        COALESCE(
            COUNT(*) FILTER (WHERE d7_completed)::DECIMAL /
            NULLIF(COUNT(*) FILTER (WHERE cohort_date <= p_date - INTERVAL '7 days'), 0),
            0
        ) as d7_rate,
        COALESCE(
            COUNT(*) FILTER (WHERE d30_completed)::DECIMAL /
            NULLIF(COUNT(*) FILTER (WHERE cohort_date <= p_date - INTERVAL '30 days'), 0),
            0
        ) as d30_rate
    INTO v_cohort
    FROM participant_cohorts
    WHERE (p_country_code IS NULL OR country_code = p_country_code);

    -- Calculate financial metrics
    SELECT
        COALESCE(SUM(amount_micros), 0) as total_rewards,
        COALESCE(SUM(amount_micros) FILTER (WHERE status IN ('allocated', 'payable', 'paid')), 0) as allocated,
        COALESCE(SUM(amount_micros) FILTER (WHERE status = 'payable'), 0) as payable,
        COALESCE(SUM(amount_micros) FILTER (WHERE status = 'paid'), 0) as paid
    INTO v_finance
    FROM allocation_ledger
    WHERE created_at::DATE <= p_date;

    -- Calculate support metrics
    SELECT
        COUNT(*) FILTER (WHERE status IN ('open', 'in_progress', 'waiting_user')) as open_tickets,
        COUNT(*) FILTER (WHERE created_at::DATE = p_date) as new_tickets,
        COUNT(*) FILTER (WHERE resolved_at::DATE = p_date) as resolved_tickets,
        COUNT(*) FILTER (WHERE escalated_at IS NOT NULL AND escalated_at::DATE = p_date) as escalated,
        AVG(EXTRACT(EPOCH FROM (resolved_at - created_at)) / 3600)
            FILTER (WHERE resolved_at IS NOT NULL) as avg_resolution
    INTO v_support
    FROM support_tickets
    WHERE (p_country_code IS NULL);

    -- Sync metrics
    SELECT
        MAX(processed_at) as last_sync,
        COUNT(*) FILTER (WHERE status = 'pending') as pending_outbox,
        COUNT(*) FILTER (WHERE status = 'pending') as pending_inbox,
        COUNT(*) FILTER (WHERE status = 'failed') as failed,
        COUNT(*) FILTER (WHERE status = 'dead_letter') as dead_letter
    INTO v_sync
    FROM outbox_events;

    -- Insert or update metrics
    INSERT INTO operator_metrics (
        metric_date, country_code,
        total_licenses, published_licenses, reserved_licenses, issued_licenses,
        expired_licenses, quarantined_licenses,
        new_cohorts, d1_completions, d3_completions, d7_completions, d30_completions,
        d7_completion_rate, d30_completion_rate,
        total_rewards_micros, total_allocated_micros, total_payable_micros, total_paid_micros,
        open_tickets, new_tickets, resolved_tickets, escalated_tickets, avg_resolution_hours,
        last_sync_at, pending_outbox_events
    )
    VALUES (
        p_date, p_country_code,
        COALESCE(v_inventory.total, 0), COALESCE(v_inventory.published, 0),
        COALESCE(v_inventory.reserved, 0), COALESCE(v_inventory.issued, 0),
        COALESCE(v_inventory.expired, 0), COALESCE(v_inventory.quarantined, 0),
        COALESCE(v_cohort.new_cohorts, 0), COALESCE(v_cohort.d1, 0),
        COALESCE(v_cohort.d3, 0), COALESCE(v_cohort.d7, 0), COALESCE(v_cohort.d30, 0),
        v_cohort.d7_rate, v_cohort.d30_rate,
        COALESCE(v_finance.total_rewards, 0), COALESCE(v_finance.allocated, 0),
        COALESCE(v_finance.payable, 0), COALESCE(v_finance.paid, 0),
        COALESCE(v_support.open_tickets, 0), COALESCE(v_support.new_tickets, 0),
        COALESCE(v_support.resolved_tickets, 0), COALESCE(v_support.escalated, 0),
        v_support.avg_resolution,
        v_sync.last_sync, COALESCE(v_sync.pending_outbox, 0)
    )
    ON CONFLICT (metric_date, country_code) DO UPDATE SET
        total_licenses = EXCLUDED.total_licenses,
        published_licenses = EXCLUDED.published_licenses,
        reserved_licenses = EXCLUDED.reserved_licenses,
        issued_licenses = EXCLUDED.issued_licenses,
        expired_licenses = EXCLUDED.expired_licenses,
        quarantined_licenses = EXCLUDED.quarantined_licenses,
        new_cohorts = EXCLUDED.new_cohorts,
        d1_completions = EXCLUDED.d1_completions,
        d3_completions = EXCLUDED.d3_completions,
        d7_completions = EXCLUDED.d7_completions,
        d30_completions = EXCLUDED.d30_completions,
        d7_completion_rate = EXCLUDED.d7_completion_rate,
        d30_completion_rate = EXCLUDED.d30_completion_rate,
        total_rewards_micros = EXCLUDED.total_rewards_micros,
        total_allocated_micros = EXCLUDED.total_allocated_micros,
        total_payable_micros = EXCLUDED.total_payable_micros,
        total_paid_micros = EXCLUDED.total_paid_micros,
        open_tickets = EXCLUDED.open_tickets,
        new_tickets = EXCLUDED.new_tickets,
        resolved_tickets = EXCLUDED.resolved_tickets,
        escalated_tickets = EXCLUDED.escalated_tickets,
        avg_resolution_hours = EXCLUDED.avg_resolution_hours,
        last_sync_at = EXCLUDED.last_sync_at,
        pending_outbox_events = EXCLUDED.pending_outbox_events
    RETURNING id INTO v_metric_id;

    RETURN v_metric_id;
END;
$$ LANGUAGE plpgsql;

-- Function to record an exception with deduplication
CREATE OR REPLACE FUNCTION record_exception(
    p_type VARCHAR(50),
    p_severity exception_severity,
    p_message TEXT,
    p_country_code CHAR(2) DEFAULT NULL,
    p_entity_type VARCHAR(50) DEFAULT NULL,
    p_entity_id VARCHAR(100) DEFAULT NULL,
    p_details JSONB DEFAULT NULL,
    p_source_service VARCHAR(50) DEFAULT NULL,
    p_correlation_id VARCHAR(100) DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_existing_id UUID;
    v_exception_id UUID;
BEGIN
    -- Check for existing unresolved exception of same type/entity
    SELECT id INTO v_existing_id
    FROM operator_exceptions
    WHERE exception_type = p_type
      AND COALESCE(entity_type, '') = COALESCE(p_entity_type, '')
      AND COALESCE(entity_id, '') = COALESCE(p_entity_id, '')
      AND resolved_at IS NULL
      AND created_at > NOW() - INTERVAL '24 hours'
    LIMIT 1;

    IF v_existing_id IS NOT NULL THEN
        -- Update existing exception
        UPDATE operator_exceptions
        SET occurrence_count = occurrence_count + 1,
            last_occurred_at = NOW(),
            details = COALESCE(p_details, details),
            severity = GREATEST(severity, p_severity)
        WHERE id = v_existing_id
        RETURNING id INTO v_exception_id;
    ELSE
        -- Create new exception
        INSERT INTO operator_exceptions (
            exception_type, severity, country_code,
            entity_type, entity_id, message, details,
            source_service, correlation_id
        )
        VALUES (
            p_type, p_severity, p_country_code,
            p_entity_type, p_entity_id, p_message, p_details,
            p_source_service, p_correlation_id
        )
        RETURNING id INTO v_exception_id;
    END IF;

    RETURN v_exception_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE operator_metrics IS 'Daily aggregated metrics for operator dashboard views';
COMMENT ON TABLE operator_exceptions IS 'Exception tracking for operator visibility and resolution';
COMMENT ON TABLE operator_margins IS 'Profit margin tracking by country and task';
COMMENT ON FUNCTION aggregate_daily_metrics IS 'Aggregates all metrics for a given date';
COMMENT ON FUNCTION record_exception IS 'Records an exception with automatic deduplication';
