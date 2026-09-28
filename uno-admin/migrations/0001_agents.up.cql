-- Create agents table (sales representatives)
CREATE TABLE IF NOT EXISTS agents (
    id text PRIMARY KEY,
    name text,
    email text,
    country text,
    commission_percent double,
    is_active boolean,
    created_at timestamp,
    updated_at timestamp,
    deleted_at timestamp
);
