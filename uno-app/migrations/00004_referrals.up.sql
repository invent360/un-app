-- Referral System Tables

-- ============================================
-- REFERRAL CODES TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS referral_codes (
    id SERIAL PRIMARY KEY,
    code VARCHAR(20) UNIQUE NOT NULL,
    owner_name VARCHAR(100),
    owner_email VARCHAR(255),
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    max_uses INT,
    current_uses INT DEFAULT 0,
    bonus_percentage DECIMAL(5,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- ============================================
-- REFERRAL STATS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS referral_stats (
    id SERIAL PRIMARY KEY,
    referral_code_id INT NOT NULL REFERENCES referral_codes(id),
    stat_date DATE NOT NULL,
    claims_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_referral_stat UNIQUE (referral_code_id, stat_date)
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_referral_codes_code ON referral_codes(code);
CREATE INDEX IF NOT EXISTS idx_referral_codes_active ON referral_codes(is_active);
CREATE INDEX IF NOT EXISTS idx_referral_stats_code ON referral_stats(referral_code_id);
CREATE INDEX IF NOT EXISTS idx_referral_stats_date ON referral_stats(stat_date);
