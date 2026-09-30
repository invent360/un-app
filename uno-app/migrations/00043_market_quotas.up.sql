-- Migration: Market Quotas and Status (Phase 7)
-- P7-06: Governed six-market quotas, market readiness controls

-- Market status - controls whether a market is open for claims
CREATE TABLE IF NOT EXISTS market_status (
    country_code CHAR(2) PRIMARY KEY,

    -- Open/closed status
    is_open BOOLEAN NOT NULL DEFAULT FALSE,
    pause_reason TEXT,
    paused_at TIMESTAMPTZ,
    paused_by VARCHAR(255),
    resumed_at TIMESTAMPTZ,
    resumed_by VARCHAR(255),

    -- Readiness requirements (P7-06)
    requires_reviewed_content BOOLEAN NOT NULL DEFAULT FALSE,
    requires_local_support BOOLEAN NOT NULL DEFAULT FALSE,
    requires_local_language BOOLEAN NOT NULL DEFAULT FALSE,

    -- Readiness status
    content_review_status VARCHAR(20) DEFAULT 'pending' CHECK (content_review_status IN ('pending', 'in_review', 'approved', 'rejected')),
    content_reviewed_at TIMESTAMPTZ,
    content_reviewed_by VARCHAR(255),

    support_readiness_status VARCHAR(20) DEFAULT 'pending' CHECK (support_readiness_status IN ('pending', 'ready', 'not_ready')),
    support_ready_at TIMESTAMPTZ,
    support_ready_by VARCHAR(255),

    language_review_status VARCHAR(20) DEFAULT 'pending' CHECK (language_review_status IN ('pending', 'approved', 'rejected')),
    language_reviewed_at TIMESTAMPTZ,
    language_reviewed_by VARCHAR(255),

    -- Market metadata
    primary_locale VARCHAR(5),
    supported_locales VARCHAR(5)[] DEFAULT '{}',
    timezone VARCHAR(50),
    currency_code CHAR(3),

    -- Notes
    notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Market quotas - configurable limits per market
CREATE TABLE IF NOT EXISTS market_quotas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Market
    country_code CHAR(2) NOT NULL,

    -- Quota type
    quota_type VARCHAR(50) NOT NULL CHECK (quota_type IN (
        'daily_claims',      -- Max claims per day
        'weekly_claims',     -- Max claims per week
        'monthly_claims',    -- Max claims per month
        'total_active',      -- Max active licenses at any time
        'daily_registrations' -- Max new registrations per day
    )),

    -- Quota values
    max_value INT NOT NULL,
    current_value INT NOT NULL DEFAULT 0,
    warning_threshold INT, -- Alert when reaching this %

    -- Period (for daily/weekly/monthly quotas)
    period_start DATE,
    period_end DATE,

    -- Status
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    exhausted_at TIMESTAMPTZ, -- When quota was exhausted

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(country_code, quota_type, period_start)
);

-- Quota consumption log for audit
CREATE TABLE IF NOT EXISTS quota_consumption_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quota_id UUID NOT NULL REFERENCES market_quotas(id) ON DELETE CASCADE,

    -- Consumption details
    user_id VARCHAR(255) NOT NULL,
    license_id VARCHAR(66),
    amount INT NOT NULL DEFAULT 1,

    -- Context
    action VARCHAR(50) NOT NULL, -- 'claim', 'registration', etc.

    -- Timestamp
    consumed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Campaign sources and QR assets (P7-06)
CREATE TABLE IF NOT EXISTS campaign_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Campaign identity
    campaign_code VARCHAR(50) UNIQUE NOT NULL,
    campaign_name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Source type
    source_type VARCHAR(50) NOT NULL CHECK (source_type IN ('qr_code', 'referral_link', 'social_media', 'email', 'partner', 'organic', 'other')),

    -- Target market
    country_code CHAR(2),

    -- Attribution
    agent_id UUID REFERENCES agents(id) ON DELETE SET NULL,
    partner_name VARCHAR(255),

    -- QR/Link assets
    qr_code_url VARCHAR(512),
    short_url VARCHAR(255),
    full_url TEXT,

    -- Tracking
    utm_source VARCHAR(100),
    utm_medium VARCHAR(100),
    utm_campaign VARCHAR(100),
    utm_content VARCHAR(100),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    starts_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,

    -- Stats (denormalized for quick access)
    click_count INT NOT NULL DEFAULT 0,
    registration_count INT NOT NULL DEFAULT 0,
    claim_count INT NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255)
);

-- Campaign attribution log
CREATE TABLE IF NOT EXISTS campaign_attributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Campaign
    campaign_id UUID NOT NULL REFERENCES campaign_sources(id) ON DELETE CASCADE,

    -- User/License
    user_id VARCHAR(255),
    license_id VARCHAR(66),

    -- Attribution type
    attribution_type VARCHAR(50) NOT NULL CHECK (attribution_type IN ('click', 'registration', 'claim', 'activation')),

    -- Context
    referrer_url TEXT,
    landing_url TEXT,
    user_agent TEXT,
    ip_address INET,
    country_code CHAR(2),

    -- Timestamp
    attributed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_market_status_open ON market_status(is_open);
CREATE INDEX idx_market_status_content ON market_status(content_review_status);
CREATE INDEX idx_market_status_support ON market_status(support_readiness_status);

CREATE INDEX idx_market_quotas_country ON market_quotas(country_code);
CREATE INDEX idx_market_quotas_type ON market_quotas(quota_type);
CREATE INDEX idx_market_quotas_enabled ON market_quotas(enabled, country_code);
CREATE INDEX idx_market_quotas_period ON market_quotas(period_start, period_end);

CREATE INDEX idx_quota_consumption_quota ON quota_consumption_log(quota_id);
CREATE INDEX idx_quota_consumption_user ON quota_consumption_log(user_id);
CREATE INDEX idx_quota_consumption_date ON quota_consumption_log(consumed_at);

CREATE INDEX idx_campaign_code ON campaign_sources(campaign_code);
CREATE INDEX idx_campaign_country ON campaign_sources(country_code);
CREATE INDEX idx_campaign_agent ON campaign_sources(agent_id);
CREATE INDEX idx_campaign_active ON campaign_sources(is_active, starts_at, ends_at);

CREATE INDEX idx_campaign_attr_campaign ON campaign_attributions(campaign_id);
CREATE INDEX idx_campaign_attr_user ON campaign_attributions(user_id);
CREATE INDEX idx_campaign_attr_license ON campaign_attributions(license_id);
CREATE INDEX idx_campaign_attr_date ON campaign_attributions(attributed_at);

-- Trigger for updated_at
CREATE TRIGGER market_status_updated
    BEFORE UPDATE ON market_status
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

CREATE TRIGGER market_quotas_updated
    BEFORE UPDATE ON market_quotas
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

CREATE TRIGGER campaign_sources_updated
    BEFORE UPDATE ON campaign_sources
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

-- Function to check and consume quota
CREATE OR REPLACE FUNCTION consume_market_quota(
    p_country_code CHAR(2),
    p_quota_type VARCHAR(50),
    p_user_id VARCHAR(255),
    p_license_id VARCHAR(66) DEFAULT NULL,
    p_action VARCHAR(50) DEFAULT 'claim'
)
RETURNS BOOLEAN AS $$
DECLARE
    v_quota market_quotas%ROWTYPE;
    v_can_consume BOOLEAN := FALSE;
BEGIN
    -- Find applicable quota
    SELECT * INTO v_quota
    FROM market_quotas
    WHERE country_code = p_country_code
      AND quota_type = p_quota_type
      AND enabled = TRUE
      AND (period_start IS NULL OR period_start <= CURRENT_DATE)
      AND (period_end IS NULL OR period_end >= CURRENT_DATE)
    FOR UPDATE;

    IF v_quota IS NULL THEN
        -- No quota configured, allow
        RETURN TRUE;
    END IF;

    IF v_quota.current_value < v_quota.max_value THEN
        -- Consume quota
        UPDATE market_quotas
        SET current_value = current_value + 1,
            exhausted_at = CASE WHEN current_value + 1 >= max_value THEN NOW() ELSE NULL END,
            updated_at = NOW()
        WHERE id = v_quota.id;

        -- Log consumption
        INSERT INTO quota_consumption_log (quota_id, user_id, license_id, action)
        VALUES (v_quota.id, p_user_id, p_license_id, p_action);

        v_can_consume := TRUE;
    END IF;

    RETURN v_can_consume;
END;
$$ LANGUAGE plpgsql;

-- Function to check market readiness
CREATE OR REPLACE FUNCTION check_market_readiness(p_country_code CHAR(2))
RETURNS TABLE (
    is_ready BOOLEAN,
    is_open BOOLEAN,
    content_ready BOOLEAN,
    support_ready BOOLEAN,
    language_ready BOOLEAN,
    blocking_reasons TEXT[]
) AS $$
DECLARE
    v_status market_status%ROWTYPE;
    v_reasons TEXT[] := '{}';
BEGIN
    SELECT * INTO v_status FROM market_status WHERE country_code = p_country_code;

    IF v_status IS NULL THEN
        RETURN QUERY SELECT FALSE, FALSE, FALSE, FALSE, FALSE, ARRAY['Market not configured'];
        RETURN;
    END IF;

    -- Check each requirement
    IF v_status.requires_reviewed_content AND v_status.content_review_status != 'approved' THEN
        v_reasons := array_append(v_reasons, 'Content not reviewed');
    END IF;

    IF v_status.requires_local_support AND v_status.support_readiness_status != 'ready' THEN
        v_reasons := array_append(v_reasons, 'Local support not ready');
    END IF;

    IF v_status.requires_local_language AND v_status.language_review_status != 'approved' THEN
        v_reasons := array_append(v_reasons, 'Language not reviewed');
    END IF;

    IF NOT v_status.is_open THEN
        v_reasons := array_append(v_reasons, COALESCE('Market paused: ' || v_status.pause_reason, 'Market closed'));
    END IF;

    RETURN QUERY SELECT
        (array_length(v_reasons, 1) IS NULL OR array_length(v_reasons, 1) = 0),
        v_status.is_open,
        (NOT v_status.requires_reviewed_content OR v_status.content_review_status = 'approved'),
        (NOT v_status.requires_local_support OR v_status.support_readiness_status = 'ready'),
        (NOT v_status.requires_local_language OR v_status.language_review_status = 'approved'),
        v_reasons;
END;
$$ LANGUAGE plpgsql;

-- Seed initial market status for pilot markets
-- Bangladesh requires reviewed Bangla content/support (P7-06)
INSERT INTO market_status (country_code, is_open, requires_reviewed_content, requires_local_support, requires_local_language, primary_locale, timezone, currency_code)
VALUES
    ('BD', FALSE, TRUE, TRUE, TRUE, 'bn', 'Asia/Dhaka', 'BDT'),
    ('NG', FALSE, FALSE, FALSE, FALSE, 'en', 'Africa/Lagos', 'NGN'),
    ('PH', FALSE, FALSE, FALSE, FALSE, 'tl', 'Asia/Manila', 'PHP'),
    ('KE', FALSE, FALSE, FALSE, FALSE, 'sw', 'Africa/Nairobi', 'KES'),
    ('IN', FALSE, TRUE, FALSE, TRUE, 'hi', 'Asia/Kolkata', 'INR'),
    ('PK', FALSE, TRUE, FALSE, FALSE, 'en', 'Asia/Karachi', 'PKR')
ON CONFLICT (country_code) DO NOTHING;

-- Comments
COMMENT ON TABLE market_status IS 'Market open/closed status and readiness requirements';
COMMENT ON TABLE market_quotas IS 'Configurable quotas per market (daily/weekly/total limits)';
COMMENT ON TABLE campaign_sources IS 'Campaign tracking sources with QR codes and attribution';
COMMENT ON TABLE campaign_attributions IS 'Campaign click/registration/claim attribution log';
COMMENT ON COLUMN market_status.requires_reviewed_content IS 'P7-06: Bangladesh requires reviewed Bangla content';
COMMENT ON COLUMN market_status.requires_local_support IS 'P7-06: Bangladesh requires local language support';
