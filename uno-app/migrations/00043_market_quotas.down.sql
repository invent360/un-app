-- Rollback: Market Quotas and Status

DROP TRIGGER IF EXISTS campaign_sources_updated ON campaign_sources;
DROP TRIGGER IF EXISTS market_quotas_updated ON market_quotas;
DROP TRIGGER IF EXISTS market_status_updated ON market_status;

DROP FUNCTION IF EXISTS check_market_readiness(CHAR(2));
DROP FUNCTION IF EXISTS consume_market_quota(CHAR(2), VARCHAR(50), VARCHAR(255), VARCHAR(66), VARCHAR(50));

DROP TABLE IF EXISTS campaign_attributions;
DROP TABLE IF EXISTS campaign_sources;
DROP TABLE IF EXISTS quota_consumption_log;
DROP TABLE IF EXISTS market_quotas;
DROP TABLE IF EXISTS market_status;
