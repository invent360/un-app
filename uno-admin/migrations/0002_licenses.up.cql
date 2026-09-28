-- Create licenses table (Unity license records)
CREATE TABLE IF NOT EXISTS licenses (
    id text PRIMARY KEY,
    license_id text,
    node_id text,
    lease_code text,
    alias text,
    agent_id text,
    ulo_name text,
    uno_share double,
    agent_share double,
    ulo_share double,
    uptime double,
    is_online boolean,
    is_active boolean,
    created_at timestamp,
    updated_at timestamp,
    deleted_at timestamp
);
