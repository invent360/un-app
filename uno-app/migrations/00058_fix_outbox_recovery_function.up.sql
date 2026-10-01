-- R5-09: Fix recover_stale_publishing_events to use correct column name
-- The outbox table uses last_error, not error

CREATE OR REPLACE FUNCTION recover_stale_publishing_events(
    p_stale_threshold_secs INT DEFAULT 300
)
RETURNS INT
LANGUAGE plpgsql
AS $$
DECLARE
    v_recovered INT;
BEGIN
    UPDATE outbox
    SET
        status = 'pending',
        claimed_by = NULL,
        claimed_at = NULL,
        claim_expires_at = NULL,
        last_error = 'Recovered from stale publishing state (worker claim expired)'
    WHERE status = 'publishing'
      AND claim_expires_at IS NOT NULL
      AND claim_expires_at < NOW();

    GET DIAGNOSTICS v_recovered = ROW_COUNT;
    RETURN v_recovered;
END;
$$;

COMMENT ON FUNCTION recover_stale_publishing_events IS 'R5-09: Recover events stuck in publishing state with expired claims';
