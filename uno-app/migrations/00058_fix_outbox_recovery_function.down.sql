-- R5-09: Revert to original function (with incorrect column)
-- This would fail if the old migration is run again, but that's acceptable

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
        last_error = 'Recovered from stale publishing state'
    WHERE status = 'publishing'
      AND claim_expires_at IS NOT NULL
      AND claim_expires_at < NOW();

    GET DIAGNOSTICS v_recovered = ROW_COUNT;
    RETURN v_recovered;
END;
$$;
