-- R4-05: Media visibility, privacy and stored-byte integrity
-- Adds visibility control and separates original hash from stored hash
-- See: v2_relaunch_implementation_plan.md Gate C

-- ============================================
-- 1. VISIBILITY ENUM
-- ============================================
-- Controls who can access the media asset

CREATE TYPE asset_visibility AS ENUM (
    'public',   -- Anyone can access
    'private',  -- Only owner and granted users
    'draft'     -- Only owner can access (not even granted users)
);

-- ============================================
-- 2. ADD COLUMNS TO MEDIA_ASSETS
-- ============================================

-- Visibility control
ALTER TABLE media_assets
ADD COLUMN IF NOT EXISTS visibility asset_visibility NOT NULL DEFAULT 'draft';

-- Separate original hash (pre-transformation) from stored hash (post-transformation)
ALTER TABLE media_assets
ADD COLUMN IF NOT EXISTS original_hash CHAR(64);

COMMENT ON COLUMN media_assets.visibility IS 'R4-05: Access control - public, private, or draft';
COMMENT ON COLUMN media_assets.original_hash IS 'R4-05: SHA256 of original uploaded bytes (pre-transformation)';
COMMENT ON COLUMN media_assets.sha256_hash IS 'R4-05: SHA256 of stored bytes (post-transformation)';

-- ============================================
-- 3. INDEXES FOR VISIBILITY
-- ============================================

-- Index for visibility filtering
CREATE INDEX IF NOT EXISTS idx_media_assets_visibility
ON media_assets (visibility);

-- Composite index for owner visibility queries
CREATE INDEX IF NOT EXISTS idx_media_assets_owner_visibility
ON media_assets (owner_id, owner_type, visibility)
WHERE deleted_at IS NULL;

-- ============================================
-- 4. VISIBILITY CHECK FUNCTION
-- ============================================
-- Returns TRUE if the accessor can view the asset

CREATE OR REPLACE FUNCTION check_media_visibility(
    p_asset_id UUID,
    p_accessor_id VARCHAR(255),
    p_is_owner BOOLEAN DEFAULT FALSE
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
DECLARE
    v_visibility asset_visibility;
    v_owner_id VARCHAR(255);
    v_deleted_at TIMESTAMPTZ;
    v_has_grant BOOLEAN;
BEGIN
    -- Get asset visibility and ownership
    SELECT visibility, owner_id, deleted_at
    INTO v_visibility, v_owner_id, v_deleted_at
    FROM media_assets
    WHERE id = p_asset_id;

    -- Asset not found or deleted
    IF NOT FOUND OR v_deleted_at IS NOT NULL THEN
        RETURN FALSE;
    END IF;

    -- Public assets are always accessible
    IF v_visibility = 'public' THEN
        RETURN TRUE;
    END IF;

    -- Draft assets only accessible by owner
    IF v_visibility = 'draft' THEN
        RETURN p_is_owner OR (p_accessor_id IS NOT NULL AND p_accessor_id = v_owner_id);
    END IF;

    -- Private assets: check if accessor is owner
    IF p_is_owner OR (p_accessor_id IS NOT NULL AND p_accessor_id = v_owner_id) THEN
        RETURN TRUE;
    END IF;

    -- Check for valid grant (not expired)
    SELECT EXISTS (
        SELECT 1 FROM media_asset_grants
        WHERE asset_id = p_asset_id
          AND grantee_id = p_accessor_id
          AND permission IN ('read', 'write')
          AND (expires_at IS NULL OR expires_at > NOW())
    ) INTO v_has_grant;

    RETURN v_has_grant;
END;
$$;

COMMENT ON FUNCTION check_media_visibility IS 'R4-05: Check if accessor can view asset based on visibility policy';

-- ============================================
-- 5. INTEGRITY VERIFICATION FUNCTION
-- ============================================
-- Compares stored hash with expected value

CREATE OR REPLACE FUNCTION verify_media_integrity(
    p_asset_id UUID,
    p_computed_hash CHAR(64)
)
RETURNS TABLE (
    verified BOOLEAN,
    expected_hash CHAR(64),
    actual_hash CHAR(64),
    mismatch_type VARCHAR(50)
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_stored_hash CHAR(64);
    v_state asset_state;
BEGIN
    SELECT sha256_hash, state
    INTO v_stored_hash, v_state
    FROM media_assets
    WHERE id = p_asset_id;

    IF NOT FOUND THEN
        RETURN QUERY SELECT FALSE, NULL::CHAR(64), p_computed_hash, 'not_found'::VARCHAR(50);
        RETURN;
    END IF;

    IF v_stored_hash IS NULL THEN
        RETURN QUERY SELECT FALSE, NULL::CHAR(64), p_computed_hash, 'no_expected_hash'::VARCHAR(50);
        RETURN;
    END IF;

    IF v_stored_hash = p_computed_hash THEN
        RETURN QUERY SELECT TRUE, v_stored_hash, p_computed_hash, NULL::VARCHAR(50);
    ELSE
        -- Update state to quarantined if hash mismatch
        UPDATE media_assets
        SET state = 'quarantined',
            state_reason = 'Hash mismatch detected during integrity check'
        WHERE id = p_asset_id
          AND state != 'quarantined';

        RETURN QUERY SELECT FALSE, v_stored_hash, p_computed_hash, 'hash_mismatch'::VARCHAR(50);
    END IF;
END;
$$;

COMMENT ON FUNCTION verify_media_integrity IS 'R4-05: Verify stored bytes match expected hash';

-- ============================================
-- 6. UPDATE VISIBILITY FUNCTION
-- ============================================
-- Safely update visibility with audit trail

CREATE OR REPLACE FUNCTION update_media_visibility(
    p_asset_id UUID,
    p_new_visibility asset_visibility,
    p_actor_id VARCHAR(255)
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
DECLARE
    v_old_visibility asset_visibility;
    v_owner_id VARCHAR(255);
BEGIN
    -- Get current state
    SELECT visibility, owner_id
    INTO v_old_visibility, v_owner_id
    FROM media_assets
    WHERE id = p_asset_id
      AND deleted_at IS NULL
    FOR UPDATE;

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    -- Only owner can change visibility
    IF v_owner_id IS DISTINCT FROM p_actor_id THEN
        RETURN FALSE;
    END IF;

    -- No change needed
    IF v_old_visibility = p_new_visibility THEN
        RETURN TRUE;
    END IF;

    -- Update visibility
    UPDATE media_assets
    SET visibility = p_new_visibility,
        updated_at = NOW()
    WHERE id = p_asset_id;

    RETURN TRUE;
END;
$$;

COMMENT ON FUNCTION update_media_visibility IS 'R4-05: Update asset visibility with ownership check';

-- ============================================
-- 7. VIEW FOR ACCESSIBLE ASSETS
-- ============================================
-- Simplifies queries for accessible assets

CREATE OR REPLACE VIEW accessible_media_assets AS
SELECT
    ma.*,
    CASE
        WHEN ma.visibility = 'public' THEN TRUE
        ELSE FALSE
    END as is_public
FROM media_assets ma
WHERE ma.deleted_at IS NULL
  AND ma.state = 'ready';

COMMENT ON VIEW accessible_media_assets IS 'R4-05: Non-deleted, ready assets for visibility filtering';
