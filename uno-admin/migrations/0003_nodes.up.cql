-- Create nodes table (node aggregation)
CREATE TABLE IF NOT EXISTS nodes (
    id text PRIMARY KEY,
    node_id text,
    name text,
    total_licenses int,
    online_licenses int,
    total_earnings_micros bigint,
    is_active boolean,
    created_at timestamp,
    updated_at timestamp,
    deleted_at timestamp
);
