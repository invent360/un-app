-- Create license_analytics table for daily uptime records
-- Stores historical uptime data synced from Unity API with required_uptime for color coding

CREATE TABLE IF NOT EXISTS license_analytics (
    license_id text,
    date text,                    -- YYYY-MM-DD format
    uptime double,                -- Actual uptime (0.0 to 1.0)
    required_uptime double,       -- Min uptime requirement at time of record
    created_at timestamp,
    updated_at timestamp,
    PRIMARY KEY (license_id, date)
) WITH CLUSTERING ORDER BY (date DESC);
