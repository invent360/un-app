-- Migration 00028: License Eligibility Rules
-- Phase 4 (P4-03): Country/task/device eligibility restrictions
--
-- Provides:
-- - Reusable eligibility rulesets
-- - Country/region restrictions
-- - Task type restrictions
-- - Device type restrictions
-- - License-level eligibility columns

-- ============================================
-- ELIGIBILITY RULESETS TABLE
-- ============================================
-- Reusable named eligibility configurations

CREATE TABLE IF NOT EXISTS eligibility_rulesets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,

    -- Country restrictions (ISO 3166-1 alpha-2 codes)
    -- NULL = all countries, empty array = no countries
    allowed_countries TEXT[],
    blocked_countries TEXT[],

    -- Task type restrictions
    -- NULL = all task types, empty array = no task types
    allowed_task_types TEXT[],
    blocked_task_types TEXT[],

    -- Device type restrictions
    -- Values: 'phone', 'tablet', 'desktop', 'tv', 'wearable'
    allowed_device_types TEXT[],
    blocked_device_types TEXT[],

    -- Minimum requirements
    min_app_version VARCHAR(20),          -- e.g., "2.0.0"
    min_os_version VARCHAR(20),           -- e.g., "14.0"
    requires_verification BOOLEAN NOT NULL DEFAULT FALSE,

    -- Time-based restrictions
    valid_from TIMESTAMPTZ,               -- NULL = no start restriction
    valid_to TIMESTAMPTZ,                 -- NULL = no end restriction
    allowed_hours_start INT,              -- 0-23, NULL = no hour restriction
    allowed_hours_end INT,                -- 0-23, NULL = no hour restriction
    allowed_days_of_week INT[],           -- 0=Sun, 6=Sat, NULL = all days

    -- Metadata
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),

    -- Constraints
    CONSTRAINT valid_hours CHECK (
        (allowed_hours_start IS NULL AND allowed_hours_end IS NULL) OR
        (allowed_hours_start >= 0 AND allowed_hours_start <= 23 AND
         allowed_hours_end >= 0 AND allowed_hours_end <= 23)
    )
);

-- Index for quick lookup by name
CREATE INDEX IF NOT EXISTS idx_eligibility_rulesets_name
ON eligibility_rulesets (name) WHERE is_active = TRUE;

-- ============================================
-- LICENSE ELIGIBILITY COLUMNS
-- ============================================
-- Direct eligibility settings on licenses

-- Ruleset reference (for inherited eligibility)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS eligibility_ruleset_id INT
    REFERENCES eligibility_rulesets(id);

-- Direct country restrictions (override ruleset if set)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS allowed_countries TEXT[];
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS blocked_countries TEXT[];

-- Direct task type restrictions
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS allowed_task_types TEXT[];

-- Direct device restrictions
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS allowed_device_types TEXT[];

-- Verification requirement
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS requires_verification BOOLEAN DEFAULT FALSE;

-- Index for licenses with eligibility rules
CREATE INDEX IF NOT EXISTS idx_licenses_eligibility_ruleset
ON licenses (eligibility_ruleset_id) WHERE eligibility_ruleset_id IS NOT NULL;

-- ============================================
-- ELIGIBILITY CHECK LOG
-- ============================================
-- Track eligibility check results for debugging/analytics

CREATE TABLE IF NOT EXISTS eligibility_check_log (
    id BIGSERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL,
    user_id VARCHAR(255),
    check_type VARCHAR(50) NOT NULL,       -- 'claim', 'reservation', 'validation'

    -- Request context
    country_code CHAR(2),
    device_type VARCHAR(20),
    task_type VARCHAR(50),
    app_version VARCHAR(20),
    os_version VARCHAR(20),
    ip_address INET,

    -- Result
    is_eligible BOOLEAN NOT NULL,
    failure_reason TEXT,
    failure_code VARCHAR(50),              -- Machine-readable failure code

    -- Applied rules
    ruleset_id INT REFERENCES eligibility_rulesets(id),
    rules_evaluated JSONB,                 -- Details of which rules were checked

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for finding checks by license
CREATE INDEX IF NOT EXISTS idx_elig_check_license
ON eligibility_check_log (license_id, created_at DESC);

-- Index for analytics by country
CREATE INDEX IF NOT EXISTS idx_elig_check_country
ON eligibility_check_log (country_code, is_eligible, created_at DESC);

-- Index for failure analysis
CREATE INDEX IF NOT EXISTS idx_elig_check_failures
ON eligibility_check_log (failure_code, created_at DESC)
WHERE is_eligible = FALSE;

-- ============================================
-- DEFAULT RULESETS
-- ============================================

-- Global ruleset (all allowed)
INSERT INTO eligibility_rulesets (name, description, created_by)
VALUES ('global', 'Default global ruleset - no restrictions', 'system')
ON CONFLICT (name) DO NOTHING;

-- Pilot countries ruleset
INSERT INTO eligibility_rulesets (
    name, description,
    allowed_countries,
    allowed_device_types,
    created_by
)
VALUES (
    'pilot_countries',
    'Pilot launch countries only',
    ARRAY['US', 'CA', 'GB', 'AU', 'NZ', 'IE'],
    ARRAY['phone', 'tablet'],
    'system'
)
ON CONFLICT (name) DO NOTHING;

-- Mobile-only ruleset
INSERT INTO eligibility_rulesets (
    name, description,
    allowed_device_types,
    created_by
)
VALUES (
    'mobile_only',
    'Mobile devices only (phone and tablet)',
    ARRAY['phone', 'tablet'],
    'system'
)
ON CONFLICT (name) DO NOTHING;

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE eligibility_rulesets IS 'Reusable eligibility rule configurations';
COMMENT ON COLUMN eligibility_rulesets.allowed_countries IS 'ISO 3166-1 alpha-2 country codes that can claim';
COMMENT ON COLUMN eligibility_rulesets.blocked_countries IS 'ISO 3166-1 alpha-2 country codes that cannot claim';
COMMENT ON COLUMN eligibility_rulesets.allowed_task_types IS 'Task types this ruleset permits';
COMMENT ON COLUMN eligibility_rulesets.allowed_device_types IS 'Device types: phone, tablet, desktop, tv, wearable';
COMMENT ON COLUMN eligibility_rulesets.allowed_hours_start IS 'UTC hour (0-23) when claims start being allowed';
COMMENT ON COLUMN eligibility_rulesets.allowed_days_of_week IS 'Days of week (0=Sun to 6=Sat) when claims are allowed';

COMMENT ON COLUMN licenses.eligibility_ruleset_id IS 'Reference to shared eligibility ruleset';
COMMENT ON COLUMN licenses.allowed_countries IS 'Direct country restrictions (overrides ruleset)';
COMMENT ON COLUMN licenses.requires_verification IS 'Whether user must be verified to claim';

COMMENT ON TABLE eligibility_check_log IS 'Audit log of eligibility checks for debugging';
COMMENT ON COLUMN eligibility_check_log.failure_code IS 'Machine-readable: country_blocked, device_blocked, etc.';
COMMENT ON COLUMN eligibility_check_log.rules_evaluated IS 'JSON details of which rules were checked';
