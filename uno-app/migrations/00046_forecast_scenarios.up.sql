-- Phase 8: Forecast Scenarios
-- Migration 00046: Scenario-based financial forecasting

-- ============================================
-- FORECAST SCENARIOS
-- ============================================

-- Scenario status
CREATE TYPE forecast_status AS ENUM ('draft', 'active', 'approved', 'archived');

-- Main scenario table
CREATE TABLE forecast_scenarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version INT NOT NULL DEFAULT 1,
    status forecast_status NOT NULL DEFAULT 'draft',

    -- Authorship and approval
    created_by VARCHAR(255) NOT NULL,
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,

    -- Time parameters
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    snapshot_date DATE NOT NULL, -- actuals as-of date
    forecast_weeks INT NOT NULL DEFAULT 10,

    -- Algorithm version for reproducibility
    algorithm_version VARCHAR(20) NOT NULL DEFAULT '1.0',

    -- Global assumptions (JSONB)
    assumptions JSONB NOT NULL DEFAULT '{
        "agreement_shares": {
            "participant_bps": 5000,
            "referral_bps": 1000,
            "uno_bps": 4000
        },
        "hosting_cost_per_user_micros": 50000,
        "messaging_cost_per_message_micros": 500,
        "default_churn_rate": 0.05
    }',

    -- Metadata
    tags TEXT[] DEFAULT '{}',
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT forecast_dates_valid CHECK (end_date > start_date),
    CONSTRAINT forecast_weeks_positive CHECK (forecast_weeks > 0 AND forecast_weeks <= 52)
);

CREATE INDEX idx_forecast_scenarios_status ON forecast_scenarios(status, created_at DESC);
CREATE INDEX idx_forecast_scenarios_created_by ON forecast_scenarios(created_by, created_at DESC);

-- ============================================
-- FORECAST TASKS
-- ============================================

-- Task configurations within a scenario
CREATE TABLE forecast_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scenario_id UUID NOT NULL REFERENCES forecast_scenarios(id) ON DELETE CASCADE,
    task_code VARCHAR(50) NOT NULL,
    task_name VARCHAR(255) NOT NULL,
    country_code CHAR(2) NOT NULL,

    -- Task parameters
    device_type VARCHAR(20), -- 'android', 'ios', 'desktop', NULL for any
    daily_rate_micros BIGINT NOT NULL,
    max_capacity INT, -- NULL for unlimited
    launch_date DATE,
    end_date DATE,

    -- Conversion assumptions
    registration_to_claim_rate DECIMAL(5,4) NOT NULL DEFAULT 0.30,
    claim_to_activation_rate DECIMAL(5,4) NOT NULL DEFAULT 0.80,

    -- Retention assumptions
    d7_retention_rate DECIMAL(5,4) NOT NULL DEFAULT 0.70,
    d30_retention_rate DECIMAL(5,4) NOT NULL DEFAULT 0.50,
    monthly_churn_rate DECIMAL(5,4) NOT NULL DEFAULT 0.05,

    -- Productivity assumptions
    avg_daily_earnings_micros BIGINT NOT NULL DEFAULT 0,
    productivity_rate DECIMAL(5,4) NOT NULL DEFAULT 0.80, -- % of active users earning

    -- Cost assumptions
    acquisition_cost_per_user_micros BIGINT NOT NULL DEFAULT 0,
    support_cost_per_user_monthly_micros BIGINT NOT NULL DEFAULT 0,

    -- Capacity planning
    weekly_claim_target INT,
    total_target INT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(scenario_id, task_code, country_code)
);

CREATE INDEX idx_forecast_tasks_scenario ON forecast_tasks(scenario_id);
CREATE INDEX idx_forecast_tasks_country ON forecast_tasks(country_code);

-- ============================================
-- FORECAST RESULTS
-- ============================================

-- Weekly projection results
CREATE TABLE forecast_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scenario_id UUID NOT NULL REFERENCES forecast_scenarios(id) ON DELETE CASCADE,
    week_number INT NOT NULL, -- 1, 2, 3...
    week_start DATE NOT NULL,
    week_end DATE NOT NULL,
    country_code CHAR(2), -- NULL for global totals
    task_code VARCHAR(50), -- NULL for country/global totals

    -- Inventory projections
    opening_licenses INT NOT NULL DEFAULT 0,
    new_claims INT NOT NULL DEFAULT 0,
    activations INT NOT NULL DEFAULT 0,
    churned INT NOT NULL DEFAULT 0,
    closing_licenses INT NOT NULL DEFAULT 0,
    active_licenses INT NOT NULL DEFAULT 0,
    productive_licenses INT NOT NULL DEFAULT 0,

    -- Revenue projections (micros)
    gross_revenue_micros BIGINT NOT NULL DEFAULT 0,
    participant_share_micros BIGINT NOT NULL DEFAULT 0,
    referral_share_micros BIGINT NOT NULL DEFAULT 0,
    uno_share_micros BIGINT NOT NULL DEFAULT 0,

    -- Cost projections (micros)
    acquisition_cost_micros BIGINT NOT NULL DEFAULT 0,
    support_cost_micros BIGINT NOT NULL DEFAULT 0,
    hosting_cost_micros BIGINT NOT NULL DEFAULT 0,
    messaging_cost_micros BIGINT NOT NULL DEFAULT 0,
    other_cost_micros BIGINT NOT NULL DEFAULT 0,
    total_cost_micros BIGINT GENERATED ALWAYS AS (
        acquisition_cost_micros + support_cost_micros + hosting_cost_micros +
        messaging_cost_micros + other_cost_micros
    ) STORED,

    -- Profit projections
    net_profit_micros BIGINT GENERATED ALWAYS AS (
        uno_share_micros - (acquisition_cost_micros + support_cost_micros +
        hosting_cost_micros + messaging_cost_micros + other_cost_micros)
    ) STORED,
    cumulative_profit_micros BIGINT NOT NULL DEFAULT 0,
    break_even_reached BOOLEAN NOT NULL DEFAULT FALSE,

    -- Funding projections
    funding_required_micros BIGINT NOT NULL DEFAULT 0,
    cumulative_funding_micros BIGINT NOT NULL DEFAULT 0,

    -- Metadata
    is_actual BOOLEAN NOT NULL DEFAULT FALSE, -- TRUE for weeks with actual data

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(scenario_id, week_number, country_code, task_code)
);

CREATE INDEX idx_forecast_results_scenario ON forecast_results(scenario_id, week_number);
CREATE INDEX idx_forecast_results_country ON forecast_results(scenario_id, country_code);

-- ============================================
-- GOLDEN TEST FIXTURES
-- ============================================

-- Golden fixtures for forecast validation
CREATE TABLE forecast_golden_fixtures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fixture_name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,

    -- Input configuration
    input_scenario JSONB NOT NULL,
    input_tasks JSONB NOT NULL,

    -- Expected outputs
    expected_results JSONB NOT NULL,
    expected_totals JSONB NOT NULL,

    -- Validation
    algorithm_version VARCHAR(20) NOT NULL,
    tolerance_percentage DECIMAL(5,4) NOT NULL DEFAULT 0.001, -- 0.1% tolerance

    -- Approval
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    finance_approved BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- SCENARIO SNAPSHOTS (for versioning)
-- ============================================

-- Immutable snapshots of scenarios for audit
CREATE TABLE forecast_scenario_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scenario_id UUID NOT NULL REFERENCES forecast_scenarios(id),
    snapshot_version INT NOT NULL,

    -- Full scenario state
    scenario_data JSONB NOT NULL,
    tasks_data JSONB NOT NULL,
    results_data JSONB NOT NULL,

    -- Metadata
    snapshot_reason VARCHAR(100), -- 'approval', 'export', 'comparison'
    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(scenario_id, snapshot_version)
);

CREATE INDEX idx_forecast_snapshots_scenario ON forecast_scenario_snapshots(scenario_id, snapshot_version DESC);

-- ============================================
-- STORED FUNCTIONS
-- ============================================

-- Function to calculate weekly projections for a scenario
CREATE OR REPLACE FUNCTION run_forecast(p_scenario_id UUID)
RETURNS INT AS $$
DECLARE
    v_scenario RECORD;
    v_week_count INT := 0;
    v_week_num INT;
    v_week_start DATE;
    v_prev_results RECORD;
    v_task RECORD;
    v_cumulative_profit BIGINT := 0;
BEGIN
    -- Get scenario
    SELECT * INTO v_scenario FROM forecast_scenarios WHERE id = p_scenario_id;
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Scenario not found: %', p_scenario_id;
    END IF;

    -- Clear existing results
    DELETE FROM forecast_results WHERE scenario_id = p_scenario_id;

    -- Generate weekly projections
    FOR v_week_num IN 1..v_scenario.forecast_weeks LOOP
        v_week_start := v_scenario.start_date + ((v_week_num - 1) * 7);

        -- For each task in scenario
        FOR v_task IN SELECT * FROM forecast_tasks WHERE scenario_id = p_scenario_id LOOP
            -- Get previous week's results
            SELECT * INTO v_prev_results
            FROM forecast_results
            WHERE scenario_id = p_scenario_id
              AND week_number = v_week_num - 1
              AND country_code = v_task.country_code
              AND task_code = v_task.task_code;

            -- Calculate projections (simplified algorithm)
            INSERT INTO forecast_results (
                scenario_id, week_number, week_start, week_end,
                country_code, task_code,
                opening_licenses, new_claims, churned, closing_licenses,
                active_licenses, productive_licenses,
                gross_revenue_micros, participant_share_micros,
                referral_share_micros, uno_share_micros,
                acquisition_cost_micros, support_cost_micros,
                cumulative_profit_micros
            )
            VALUES (
                p_scenario_id, v_week_num, v_week_start, v_week_start + 6,
                v_task.country_code, v_task.task_code,
                COALESCE(v_prev_results.closing_licenses, 0),
                COALESCE(v_task.weekly_claim_target, 10),
                GREATEST(0, (COALESCE(v_prev_results.closing_licenses, 0) * v_task.monthly_churn_rate / 4)::INT),
                COALESCE(v_prev_results.closing_licenses, 0) +
                    COALESCE(v_task.weekly_claim_target, 10) -
                    GREATEST(0, (COALESCE(v_prev_results.closing_licenses, 0) * v_task.monthly_churn_rate / 4)::INT),
                COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10),
                ((COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.productivity_rate)::INT,
                -- Revenue = productive * daily_rate * 7 days
                (((COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.productivity_rate)::INT * v_task.daily_rate_micros * 7),
                -- 50% participant
                (((COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.productivity_rate)::INT * v_task.daily_rate_micros * 7 * 50 / 100),
                -- 10% referral
                (((COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.productivity_rate)::INT * v_task.daily_rate_micros * 7 * 10 / 100),
                -- 40% UNO
                (((COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.productivity_rate)::INT * v_task.daily_rate_micros * 7 * 40 / 100),
                -- Acquisition cost
                COALESCE(v_task.weekly_claim_target, 10) * v_task.acquisition_cost_per_user_micros,
                -- Support cost
                (COALESCE(v_prev_results.closing_licenses, 0) + COALESCE(v_task.weekly_claim_target, 10)) * v_task.support_cost_per_user_monthly_micros / 4,
                v_cumulative_profit
            );

            v_week_count := v_week_count + 1;
        END LOOP;
    END LOOP;

    -- Update cumulative profits
    UPDATE forecast_results fr SET
        cumulative_profit_micros = (
            SELECT SUM(net_profit_micros)
            FROM forecast_results fr2
            WHERE fr2.scenario_id = fr.scenario_id
              AND fr2.week_number <= fr.week_number
        ),
        break_even_reached = (
            SELECT SUM(net_profit_micros) >= 0
            FROM forecast_results fr2
            WHERE fr2.scenario_id = fr.scenario_id
              AND fr2.week_number <= fr.week_number
        )
    WHERE scenario_id = p_scenario_id;

    RETURN v_week_count;
END;
$$ LANGUAGE plpgsql;

-- Function to export scenario as JSON
CREATE OR REPLACE FUNCTION export_scenario(p_scenario_id UUID)
RETURNS JSONB AS $$
DECLARE
    v_result JSONB;
BEGIN
    SELECT jsonb_build_object(
        'scenario', (SELECT row_to_json(s) FROM forecast_scenarios s WHERE id = p_scenario_id),
        'tasks', (SELECT jsonb_agg(row_to_json(t)) FROM forecast_tasks t WHERE scenario_id = p_scenario_id),
        'results', (SELECT jsonb_agg(row_to_json(r)) FROM forecast_results r WHERE scenario_id = p_scenario_id ORDER BY week_number)
    ) INTO v_result;

    RETURN v_result;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- TRIGGERS
-- ============================================

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_forecast_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_forecast_scenarios_updated
    BEFORE UPDATE ON forecast_scenarios
    FOR EACH ROW EXECUTE FUNCTION update_forecast_timestamp();

CREATE TRIGGER trg_forecast_tasks_updated
    BEFORE UPDATE ON forecast_tasks
    FOR EACH ROW EXECUTE FUNCTION update_forecast_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE forecast_scenarios IS 'Scenario definitions for financial forecasting';
COMMENT ON TABLE forecast_tasks IS 'Task configurations within a forecast scenario';
COMMENT ON TABLE forecast_results IS 'Weekly projection results from forecast execution';
COMMENT ON TABLE forecast_golden_fixtures IS 'Golden test fixtures for forecast validation';
COMMENT ON TABLE forecast_scenario_snapshots IS 'Immutable snapshots for audit and comparison';
COMMENT ON FUNCTION run_forecast IS 'Executes forecast calculations for a scenario';
COMMENT ON FUNCTION export_scenario IS 'Exports scenario with tasks and results as JSON';
