-- Rollback: Restore NOT NULL constraints on license columns
-- Note: This will fail if any NULL values exist in these columns

-- Restore NOT NULL on owner_wallet_address
ALTER TABLE licenses
    ALTER COLUMN owner_wallet_address SET NOT NULL;

-- Restore NOT NULL on node_id
ALTER TABLE licenses
    ALTER COLUMN node_id SET NOT NULL;
