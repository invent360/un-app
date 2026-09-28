-- Visitor Tracking Tables
-- Combines: visitors, visitor_stats

-- ============================================
-- VISITORS TABLE
-- ============================================

CREATE TABLE IF NOT EXISTS visitors (
    id SERIAL PRIMARY KEY,
    fingerprint VARCHAR(64) NOT NULL,
    ip_address VARCHAR(45),
    user_agent TEXT,
    country_code VARCHAR(2),
    first_seen_at TIMESTAMPTZ DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ DEFAULT NOW(),
    visit_count INT DEFAULT 1,

    CONSTRAINT unique_fingerprint UNIQUE (fingerprint)
);

-- ============================================
-- VISITOR STATS TABLE (Daily Aggregates)
-- ============================================

CREATE TABLE IF NOT EXISTS visitor_stats (
    id SERIAL PRIMARY KEY,
    stat_date DATE NOT NULL,
    country_code VARCHAR(2),
    unique_visitors INT DEFAULT 0,
    total_visits INT DEFAULT 0,
    claims_count INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT unique_stat UNIQUE (stat_date, country_code)
);

-- ============================================
-- INDEXES
-- ============================================

CREATE INDEX IF NOT EXISTS idx_visitors_fingerprint ON visitors(fingerprint);
CREATE INDEX IF NOT EXISTS idx_visitors_country ON visitors(country_code);
CREATE INDEX IF NOT EXISTS idx_visitors_last_seen ON visitors(last_seen_at);

CREATE INDEX IF NOT EXISTS idx_visitor_stats_date ON visitor_stats(stat_date);
CREATE INDEX IF NOT EXISTS idx_visitor_stats_country ON visitor_stats(country_code);
