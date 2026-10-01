-- Rollback: R5-06 Fix Pagination
-- Restores the original (buggy) version for rollback purposes

CREATE OR REPLACE FUNCTION fetch_licenses_for_sync(
    p_cursor_timestamp TIMESTAMPTZ DEFAULT NULL,
    p_cursor_id VARCHAR(100) DEFAULT NULL,
    p_limit INT DEFAULT 100
) RETURNS TABLE (
    license_id VARCHAR(66),
    lease_code VARCHAR(100),
    claimed_at TIMESTAMPTZ,
    split_type TEXT,
    uno_share_pct NUMERIC(5,2),
    ulo_share_pct NUMERIC(5,2),
    agent_share_pct NUMERIC(5,2),
    issued_to VARCHAR(255),
    referral_id INT,
    is_last_batch BOOLEAN
) AS $$
DECLARE
    v_count INT;
BEGIN
    -- Original version (buggy)
    SELECT COUNT(*) INTO v_count
    FROM licenses l
    WHERE l.claimed = true
      AND (
          p_cursor_timestamp IS NULL
          OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
      );

    RETURN QUERY
    SELECT
        l.id::VARCHAR(66) as license_id,
        l.lease_code::VARCHAR(100),
        l.claimed_at,
        l.split_type::text,
        l.uno_share_pct,
        l.ulo_share_pct,
        l.agent_share_pct,
        l.issued_to,
        l.referral_id,
        (ROW_NUMBER() OVER (ORDER BY l.claimed_at ASC, l.id ASC) = v_count) as is_last_batch
    FROM licenses l
    WHERE l.claimed = true
      AND (
          p_cursor_timestamp IS NULL
          OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
      )
    ORDER BY l.claimed_at ASC, l.id ASC
    LIMIT p_limit;
END;
$$ LANGUAGE plpgsql;
