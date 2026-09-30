-- Migration: Make license columns nullable for portal inserts (Gate A - R3-02)
-- The uno-admin portal creates licenses without owner_wallet_address and node_id
-- These are populated later by the device/owner during activation

-- Make owner_wallet_address nullable (populated during device linking)
ALTER TABLE licenses
    ALTER COLUMN owner_wallet_address DROP NOT NULL;

-- Make node_id nullable (populated during device assignment)
ALTER TABLE licenses
    ALTER COLUMN node_id DROP NOT NULL;

-- Add comments for documentation
COMMENT ON COLUMN licenses.owner_wallet_address IS 'Owner wallet address, populated during device linking. May be NULL for unlinked licenses.';
COMMENT ON COLUMN licenses.node_id IS 'Node ID, populated during device assignment. May be NULL for unassigned licenses.';
