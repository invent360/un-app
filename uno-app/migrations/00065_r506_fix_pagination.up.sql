-- Migration: R5-06 Fix Pagination is_last_batch Detection
-- Fixes the buggy is_last_batch detection in fetch_licenses_for_sync function
-- The original used ROW_NUMBER() = COUNT which was always false after LIMIT

-- ============================================
-- Fix fetch_licenses_for_sync function
-- ============================================
-- Correct approach: Fetch LIMIT + 1 rows, if fewer returned, it's the last batch

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
    v_fetch_count INT;
    v_is_last BOOLEAN;
BEGIN
    -- R5-06 FIX: Correct last batch detection using LIMIT + 1 approach
    -- Count how many rows we WOULD get with LIMIT + 1
    SELECT COUNT(*) INTO v_fetch_count
    FROM (
        SELECT 1
        FROM licenses l
        WHERE l.claimed = true
          AND (
              p_cursor_timestamp IS NULL
              OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
          )
        LIMIT (p_limit + 1)
    ) sub;

    -- If we get p_limit or fewer rows, this is the last batch
    v_is_last := (v_fetch_count <= p_limit);

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
        v_is_last as is_last_batch  -- All rows in this batch share the same flag
    FROM licenses l
    WHERE l.claimed = true
      AND (
          p_cursor_timestamp IS NULL
          OR (l.claimed_at, l.id) > (p_cursor_timestamp, COALESCE(p_cursor_id, ''))
      )
    ORDER BY l.claimed_at ASC, l.id ASC
    LIMIT p_limit;  -- Return only the requested limit
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- Comment explaining the fix
-- ============================================
COMMENT ON FUNCTION fetch_licenses_for_sync IS 'R5-06 FIX: Returns licenses for sync with correct is_last_batch detection using LIMIT+1 approach';
