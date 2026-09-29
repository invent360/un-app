-- License Lifecycle Migration
-- Adds columns required by the marketplace claim system
-- See: DB-01, DB-02 in UNO_APP_V2_REQUIREMENTS.md

-- ============================================
-- SPLIT TYPE ENUM
-- ============================================
-- Represents the revenue split between parties (50/40/10 model)

DO $$ BEGIN
    CREATE TYPE split_type AS ENUM ('5050', '5545', '6040');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ============================================
-- ADD MISSING COLUMNS TO LICENSES TABLE
-- ============================================
-- These columns support the marketplace claim flow

-- Lease code for claiming
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS lease_code VARCHAR(64) UNIQUE;

-- Validity period for marketplace
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS valid_from TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS valid_to TIMESTAMPTZ;

-- Split type (revenue distribution)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS split_type split_type;

-- Claim status
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS claimed BOOLEAN DEFAULT false;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS claimed_by VARCHAR(255);

-- Device binding
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS bound_to_device BOOLEAN DEFAULT false;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS device_id VARCHAR(255);

-- Reservation system (for atomic claims)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS reserved_until TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS reservation_token VARCHAR(64);

-- Referral attribution (immutable after claim)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS referral_id INT REFERENCES referrals(id);
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS referral_attributed_at TIMESTAMPTZ;
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS referral_agreement_version INT;

-- Created timestamp (if not exists)
ALTER TABLE licenses ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();

-- ============================================
-- INDEXES FOR NEW COLUMNS
-- ============================================

CREATE INDEX IF NOT EXISTS idx_licenses_lease_code ON licenses(lease_code);
CREATE INDEX IF NOT EXISTS idx_licenses_claimed ON licenses(claimed);
CREATE INDEX IF NOT EXISTS idx_licenses_valid_from ON licenses(valid_from);
CREATE INDEX IF NOT EXISTS idx_licenses_valid_to ON licenses(valid_to);
CREATE INDEX IF NOT EXISTS idx_licenses_split_type ON licenses(split_type);
CREATE INDEX IF NOT EXISTS idx_licenses_reserved_until ON licenses(reserved_until) WHERE reserved_until IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_licenses_referral ON licenses(referral_id);

-- Composite index for availability queries
CREATE INDEX IF NOT EXISTS idx_licenses_available ON licenses(claimed, valid_from, valid_to)
WHERE claimed = false;

-- ============================================
-- AGREEMENT VERSIONS TABLE
-- ============================================
-- Tracks agreement terms and splits by version

CREATE TABLE IF NOT EXISTS agreement_versions (
    id SERIAL PRIMARY KEY,
    version INT UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    ulo_bps INT NOT NULL,           -- ULO share in basis points (e.g., 5000 = 50%)
    uno_bps INT NOT NULL,           -- UNO share in basis points (e.g., 4000 = 40%)
    referral_bps INT NOT NULL,      -- Referral share in basis points (e.g., 1000 = 10%)
    effective_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_to TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT valid_split_total CHECK (ulo_bps + uno_bps + referral_bps = 10000)
);

-- Insert the canonical 50/40/10 split as version 1
INSERT INTO agreement_versions (version, name, description, ulo_bps, uno_bps, referral_bps, effective_from)
VALUES (1, '50/40/10 Standard', 'Standard revenue split: 50% ULO, 40% UNO, 10% Referral', 5000, 4000, 1000, NOW())
ON CONFLICT (version) DO NOTHING;

-- ============================================
-- RESERVATIONS TABLE
-- ============================================
-- Tracks exclusive license reservations for atomic claiming

CREATE TABLE IF NOT EXISTS license_reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    license_id VARCHAR(66) NOT NULL REFERENCES licenses(id) ON DELETE CASCADE,
    session_token VARCHAR(64) UNIQUE NOT NULL,
    reserved_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    claimed_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    status VARCHAR(20) DEFAULT 'active',  -- active, claimed, expired, released

    CONSTRAINT one_active_reservation UNIQUE (license_id)
);

CREATE INDEX IF NOT EXISTS idx_reservations_session ON license_reservations(session_token);
CREATE INDEX IF NOT EXISTS idx_reservations_expires ON license_reservations(expires_at);
CREATE INDEX IF NOT EXISTS idx_reservations_status ON license_reservations(status);

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TYPE split_type IS 'Revenue split type: 5050, 5545, 6040';
COMMENT ON TABLE agreement_versions IS 'Tracks revenue split terms by version (50/40/10 model)';
COMMENT ON TABLE license_reservations IS 'Exclusive reservations for atomic license claiming';
COMMENT ON COLUMN licenses.referral_agreement_version IS 'Snapshot of agreement version at attribution time (immutable)';
