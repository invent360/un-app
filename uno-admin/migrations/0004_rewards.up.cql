-- Create rewards table (reward payouts for licenses)
-- Source: data/incentives/rewards_*.json
CREATE TABLE IF NOT EXISTS rewards (
    id text PRIMARY KEY,
    user_id text,
    type text,
    description text,
    node_id text,
    license_id text,
    license_lease_id text,
    task_key text,
    task_metadata text,
    completed_at timestamp,
    created_at timestamp,
    amount_micros bigint
);
