-- License System Tables
-- Combines: licenses, license_variants, license_claims

-- ============================================
-- LICENSES TABLE (Raw License Import)
-- ============================================

CREATE TABLE IF NOT EXISTS licenses (
    id VARCHAR(66) PRIMARY KEY,
    node_id VARCHAR(66) NOT NULL,
    owner_wallet_address VARCHAR(42) NOT NULL,
    device_name VARCHAR(255),
    activation_start_at TIMESTAMPTZ,
    activation_end_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    is_leased BOOLEAN DEFAULT false,
    lease_from TIMESTAMPTZ,
    lease_to TIMESTAMPTZ,
    lease_share_percentage DECIMAL(5,2),
    lease_min_uptime_percentage DECIMAL(5,2),
    uptime DECIMAL(5,2) DEFAULT 0,
    imported_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT valid_share CHECK (
        lease_share_percentage IS NULL OR
        (lease_share_percentage >= 0 AND lease_share_percentage <= 100)
    ),
    CONSTRAINT valid_uptime CHECK (
        lease_min_uptime_percentage IS NULL OR
        (lease_min_uptime_percentage >= 0 AND lease_min_uptime_percentage <= 100)
    )
);

-- ============================================
-- LICENSE VARIANTS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS license_variants (
    id SERIAL PRIMARY KEY,
    user_share_percentage INT NOT NULL,
    operator_share_percentage INT NOT NULL,
    lease_duration_months INT NOT NULL DEFAULT 12,
    min_uptime_percentage DECIMAL(5,2) NOT NULL DEFAULT 75.00,
    total_quantity INT NOT NULL DEFAULT 0,
    claimed_count INT NOT NULL DEFAULT 0,
    min_monthly_earnings DECIMAL(10,2),
    max_monthly_earnings DECIMAL(10,2),
    display_name VARCHAR(100),
    display_order INT DEFAULT 0,
    is_featured BOOLEAN DEFAULT false,
    status VARCHAR(20) DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT valid_split CHECK (user_share_percentage + operator_share_percentage = 100),
    CONSTRAINT unique_split UNIQUE (user_share_percentage, operator_share_percentage, lease_duration_months)
);

-- ============================================
-- LICENSE CLAIMS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS license_claims (
    id SERIAL PRIMARY KEY,
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id),
    variant_id INT NOT NULL REFERENCES license_variants(id),
    claim_token VARCHAR(64) UNIQUE NOT NULL,
    claimed_at TIMESTAMPTZ DEFAULT NOW(),
    session_fingerprint VARCHAR(64),
    country_code VARCHAR(2),
    status VARCHAR(20) DEFAULT 'claimed',
    referral_code VARCHAR(20),

    CONSTRAINT one_claim_per_license UNIQUE (license_id)
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_licenses_is_leased ON licenses(is_leased);
CREATE INDEX IF NOT EXISTS idx_licenses_share_percentage ON licenses(lease_share_percentage);
CREATE INDEX IF NOT EXISTS idx_licenses_node_id ON licenses(node_id);

CREATE INDEX IF NOT EXISTS idx_claims_variant_id ON license_claims(variant_id);
CREATE INDEX IF NOT EXISTS idx_claims_claimed_at ON license_claims(claimed_at);
CREATE INDEX IF NOT EXISTS idx_claims_country ON license_claims(country_code);
CREATE INDEX IF NOT EXISTS idx_claims_token ON license_claims(claim_token);
CREATE INDEX IF NOT EXISTS idx_claims_referral ON license_claims(referral_code);
