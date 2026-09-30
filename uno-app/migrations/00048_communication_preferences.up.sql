-- Phase 8: Communication Preferences
-- Migration 00048: User communication preferences and messaging infrastructure

-- ============================================
-- ENUM TYPES
-- ============================================

-- Communication channels
CREATE TYPE comm_channel AS ENUM ('email', 'push', 'sms', 'in_app');

-- Message categories
CREATE TYPE message_category AS ENUM ('transactional', 'marketing', 'support', 'cohort', 'system');

-- Suppression types
CREATE TYPE suppression_type AS ENUM ('bounce', 'complaint', 'unsubscribe', 'manual', 'invalid');

-- Template status
CREATE TYPE template_status AS ENUM ('draft', 'pending_approval', 'approved', 'archived');

-- Scheduled message status
CREATE TYPE scheduled_message_status AS ENUM ('scheduled', 'sending', 'sent', 'failed', 'cancelled');

-- ============================================
-- COMMUNICATION PREFERENCES
-- ============================================

-- User communication preferences and opt-outs
CREATE TABLE communication_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255) NOT NULL UNIQUE,

    -- Channel preferences
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    push_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    sms_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE,

    -- Category preferences
    transactional_enabled BOOLEAN NOT NULL DEFAULT TRUE, -- cannot opt out
    marketing_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    support_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    cohort_reminders_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    system_enabled BOOLEAN NOT NULL DEFAULT TRUE,

    -- Frequency preferences
    digest_frequency VARCHAR(20) DEFAULT 'immediate', -- 'immediate', 'daily', 'weekly'
    max_messages_per_day INT,

    -- Timezone and quiet hours
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    quiet_hours_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    quiet_hours_start TIME,
    quiet_hours_end TIME,

    -- Locale
    preferred_locale VARCHAR(10) NOT NULL DEFAULT 'en',

    -- Global opt-out
    global_opt_out BOOLEAN NOT NULL DEFAULT FALSE,
    opt_out_reason TEXT,
    opted_out_at TIMESTAMPTZ,

    -- Verification
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verified_at TIMESTAMPTZ,
    phone_verified BOOLEAN NOT NULL DEFAULT FALSE,
    phone_verified_at TIMESTAMPTZ,

    -- Contact info (may differ from account)
    notification_email VARCHAR(255),
    notification_phone VARCHAR(20),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comm_prefs_user ON communication_preferences(user_id);
CREATE INDEX idx_comm_prefs_opt_out ON communication_preferences(global_opt_out)
    WHERE global_opt_out = TRUE;

-- ============================================
-- SUPPRESSION LIST
-- ============================================

-- Suppression list (bounces, complaints, unsubscribes)
CREATE TABLE communication_suppressions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel comm_channel NOT NULL,
    identifier VARCHAR(255) NOT NULL, -- email address, phone number, device token
    identifier_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for faster lookups

    suppression_type suppression_type NOT NULL,
    source VARCHAR(100) NOT NULL, -- 'ses', 'twilio', 'fcm', 'manual', etc.

    -- Details
    reason TEXT,
    bounce_type VARCHAR(50), -- 'hard', 'soft', 'transient'
    complaint_type VARCHAR(50), -- 'abuse', 'fraud', etc.
    diagnostic_code VARCHAR(255),

    -- Scope
    permanent BOOLEAN NOT NULL DEFAULT FALSE,
    user_id VARCHAR(255), -- if associated with specific user

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ, -- for soft bounces

    -- External references
    external_id VARCHAR(255),
    external_timestamp TIMESTAMPTZ,

    UNIQUE(channel, identifier_hash)
);

CREATE INDEX idx_suppressions_lookup ON communication_suppressions(channel, identifier_hash);
CREATE INDEX idx_suppressions_active ON communication_suppressions(channel, identifier_hash)
    WHERE permanent = TRUE OR expires_at IS NULL OR expires_at > NOW();
CREATE INDEX idx_suppressions_user ON communication_suppressions(user_id)
    WHERE user_id IS NOT NULL;
CREATE INDEX idx_suppressions_type ON communication_suppressions(suppression_type, created_at DESC);

-- ============================================
-- MESSAGE TEMPLATES
-- ============================================

-- Message templates with multi-channel support
CREATE TABLE message_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_code VARCHAR(100) NOT NULL,
    version INT NOT NULL DEFAULT 1,
    channel comm_channel NOT NULL,
    locale VARCHAR(10) NOT NULL DEFAULT 'en',

    -- Content
    subject VARCHAR(500), -- for email
    title VARCHAR(255), -- for push/in-app
    body_template TEXT NOT NULL,
    html_template TEXT, -- for email
    action_url TEXT, -- for push/in-app deep link

    -- Template engine
    template_engine VARCHAR(20) NOT NULL DEFAULT 'handlebars', -- 'handlebars', 'liquid', 'plain'

    -- Metadata
    category message_category NOT NULL,
    description TEXT,
    variables TEXT[] NOT NULL DEFAULT '{}', -- expected variables
    sample_data JSONB, -- sample data for preview

    -- Status and approval
    status template_status NOT NULL DEFAULT 'draft',
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    rejection_reason TEXT,

    -- Usage tracking
    usage_count INT NOT NULL DEFAULT 0,
    last_used_at TIMESTAMPTZ,

    -- Ownership
    created_by VARCHAR(255) NOT NULL,
    updated_by VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(template_code, version, channel, locale)
);

CREATE INDEX idx_templates_code ON message_templates(template_code, channel, locale)
    WHERE status = 'approved';
CREATE INDEX idx_templates_category ON message_templates(category, channel)
    WHERE status = 'approved';
CREATE INDEX idx_templates_pending ON message_templates(created_at DESC)
    WHERE status = 'pending_approval';

-- ============================================
-- SCHEDULED MESSAGES
-- ============================================

-- Scheduled messages queue
CREATE TABLE scheduled_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255) NOT NULL,
    channel comm_channel NOT NULL,

    -- Template reference (optional)
    template_id UUID REFERENCES message_templates(id),
    template_code VARCHAR(100),
    template_version INT,

    -- Recipient
    recipient_address VARCHAR(255) NOT NULL, -- email, phone, device token
    recipient_name VARCHAR(255),

    -- Content (either from template or direct)
    subject VARCHAR(500),
    title VARCHAR(255),
    body TEXT NOT NULL,
    html_body TEXT,
    action_url TEXT,

    -- Template data (if using template)
    template_data JSONB,

    -- Scheduling
    scheduled_at TIMESTAMPTZ NOT NULL,
    local_time_preference BOOLEAN NOT NULL DEFAULT TRUE,
    user_timezone VARCHAR(50),

    -- Category for preference checking
    category message_category NOT NULL DEFAULT 'transactional',

    -- Priority (higher = more important)
    priority INT NOT NULL DEFAULT 0,

    -- Status
    status scheduled_message_status NOT NULL DEFAULT 'scheduled',
    sent_at TIMESTAMPTZ,
    external_id VARCHAR(255), -- provider message ID

    -- Error handling
    attempt_count INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 3,
    error_code VARCHAR(50),
    error_message TEXT,
    next_retry_at TIMESTAMPTZ,

    -- Correlation
    correlation_id VARCHAR(255),
    triggered_by VARCHAR(100), -- 'cohort_reminder', 'support_reply', etc.
    entity_type VARCHAR(50),
    entity_id VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_scheduled_pending ON scheduled_messages(scheduled_at)
    WHERE status = 'scheduled';
CREATE INDEX idx_scheduled_retry ON scheduled_messages(next_retry_at)
    WHERE status = 'failed' AND next_retry_at IS NOT NULL;
CREATE INDEX idx_scheduled_user ON scheduled_messages(user_id, created_at DESC);
CREATE INDEX idx_scheduled_correlation ON scheduled_messages(correlation_id)
    WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_scheduled_priority ON scheduled_messages(priority DESC, scheduled_at)
    WHERE status = 'scheduled';

-- ============================================
-- MESSAGE DELIVERY LOG
-- ============================================

-- Historical log of all sent messages
CREATE TABLE message_delivery_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scheduled_message_id UUID REFERENCES scheduled_messages(id),

    user_id VARCHAR(255) NOT NULL,
    channel comm_channel NOT NULL,
    category message_category NOT NULL,

    -- Content summary (not full content for storage efficiency)
    template_code VARCHAR(100),
    subject VARCHAR(500),
    recipient_address VARCHAR(255) NOT NULL,

    -- Delivery status
    delivered BOOLEAN NOT NULL,
    delivered_at TIMESTAMPTZ,

    -- Provider info
    provider VARCHAR(50), -- 'ses', 'twilio', 'fcm', etc.
    external_id VARCHAR(255),

    -- Events (open, click, bounce, complaint)
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,
    bounced_at TIMESTAMPTZ,
    complained_at TIMESTAMPTZ,

    -- Metadata
    metadata JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_delivery_log_user ON message_delivery_log(user_id, created_at DESC);
CREATE INDEX idx_delivery_log_external ON message_delivery_log(provider, external_id)
    WHERE external_id IS NOT NULL;
CREATE INDEX idx_delivery_log_template ON message_delivery_log(template_code, created_at DESC)
    WHERE template_code IS NOT NULL;

-- ============================================
-- DEVICE TOKENS
-- ============================================

-- Push notification device tokens
CREATE TABLE device_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255) NOT NULL,

    -- Token details
    token TEXT NOT NULL,
    token_hash VARCHAR(64) NOT NULL, -- SHA-256 for uniqueness
    platform VARCHAR(20) NOT NULL, -- 'ios', 'android', 'web'
    platform_version VARCHAR(50),

    -- Device info
    device_id VARCHAR(255),
    device_name VARCHAR(255),
    app_version VARCHAR(50),

    -- Status
    active BOOLEAN NOT NULL DEFAULT TRUE,
    invalidated_at TIMESTAMPTZ,
    invalidation_reason VARCHAR(100),

    -- Last activity
    last_used_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    last_failure_at TIMESTAMPTZ,
    consecutive_failures INT NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(token_hash)
);

CREATE INDEX idx_device_tokens_user ON device_tokens(user_id, active)
    WHERE active = TRUE;
CREATE INDEX idx_device_tokens_platform ON device_tokens(platform, active)
    WHERE active = TRUE;

-- ============================================
-- STORED FUNCTIONS
-- ============================================

-- Check if user can receive message
CREATE OR REPLACE FUNCTION can_receive_message(
    p_user_id VARCHAR(255),
    p_channel comm_channel,
    p_category message_category,
    p_recipient VARCHAR(255)
)
RETURNS JSONB AS $$
DECLARE
    v_prefs RECORD;
    v_suppressed BOOLEAN;
    v_result JSONB;
BEGIN
    -- Get user preferences
    SELECT * INTO v_prefs
    FROM communication_preferences
    WHERE user_id = p_user_id;

    -- Default result
    v_result := jsonb_build_object(
        'can_send', TRUE,
        'reason', NULL
    );

    -- Check global opt-out
    IF v_prefs IS NOT NULL AND v_prefs.global_opt_out THEN
        -- Always allow transactional
        IF p_category != 'transactional' THEN
            RETURN jsonb_build_object(
                'can_send', FALSE,
                'reason', 'global_opt_out'
            );
        END IF;
    END IF;

    -- Check channel preference
    IF v_prefs IS NOT NULL THEN
        IF (p_channel = 'email' AND NOT v_prefs.email_enabled) OR
           (p_channel = 'push' AND NOT v_prefs.push_enabled) OR
           (p_channel = 'sms' AND NOT v_prefs.sms_enabled) OR
           (p_channel = 'in_app' AND NOT v_prefs.in_app_enabled) THEN
            IF p_category != 'transactional' THEN
                RETURN jsonb_build_object(
                    'can_send', FALSE,
                    'reason', 'channel_disabled'
                );
            END IF;
        END IF;
    END IF;

    -- Check category preference
    IF v_prefs IS NOT NULL THEN
        IF (p_category = 'marketing' AND NOT v_prefs.marketing_enabled) OR
           (p_category = 'support' AND NOT v_prefs.support_enabled) OR
           (p_category = 'cohort' AND NOT v_prefs.cohort_reminders_enabled) THEN
            RETURN jsonb_build_object(
                'can_send', FALSE,
                'reason', 'category_disabled'
            );
        END IF;
    END IF;

    -- Check suppression list
    SELECT EXISTS (
        SELECT 1 FROM communication_suppressions
        WHERE channel = p_channel
          AND identifier_hash = encode(sha256(lower(p_recipient)::BYTEA), 'hex')
          AND (permanent = TRUE OR expires_at IS NULL OR expires_at > NOW())
    ) INTO v_suppressed;

    IF v_suppressed THEN
        RETURN jsonb_build_object(
            'can_send', FALSE,
            'reason', 'suppressed'
        );
    END IF;

    -- Check quiet hours
    IF v_prefs IS NOT NULL AND v_prefs.quiet_hours_enabled AND
       v_prefs.quiet_hours_start IS NOT NULL AND v_prefs.quiet_hours_end IS NOT NULL THEN
        DECLARE
            v_user_time TIME;
        BEGIN
            v_user_time := (NOW() AT TIME ZONE v_prefs.timezone)::TIME;

            IF v_prefs.quiet_hours_start < v_prefs.quiet_hours_end THEN
                -- Normal range (e.g., 22:00 to 08:00 doesn't wrap)
                IF v_user_time >= v_prefs.quiet_hours_start AND v_user_time < v_prefs.quiet_hours_end THEN
                    IF p_category NOT IN ('transactional', 'system') THEN
                        RETURN jsonb_build_object(
                            'can_send', FALSE,
                            'reason', 'quiet_hours',
                            'retry_after', v_prefs.quiet_hours_end
                        );
                    END IF;
                END IF;
            ELSE
                -- Wraps midnight (e.g., 22:00 to 08:00)
                IF v_user_time >= v_prefs.quiet_hours_start OR v_user_time < v_prefs.quiet_hours_end THEN
                    IF p_category NOT IN ('transactional', 'system') THEN
                        RETURN jsonb_build_object(
                            'can_send', FALSE,
                            'reason', 'quiet_hours',
                            'retry_after', v_prefs.quiet_hours_end
                        );
                    END IF;
                END IF;
            END IF;
        END;
    END IF;

    RETURN v_result;
END;
$$ LANGUAGE plpgsql;

-- Add suppression
CREATE OR REPLACE FUNCTION add_suppression(
    p_channel comm_channel,
    p_identifier VARCHAR(255),
    p_type suppression_type,
    p_source VARCHAR(100),
    p_reason TEXT DEFAULT NULL,
    p_permanent BOOLEAN DEFAULT FALSE,
    p_user_id VARCHAR(255) DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    v_suppression_id UUID;
    v_hash VARCHAR(64);
BEGIN
    v_hash := encode(sha256(lower(p_identifier)::BYTEA), 'hex');

    INSERT INTO communication_suppressions (
        channel, identifier, identifier_hash,
        suppression_type, source, reason, permanent, user_id
    )
    VALUES (
        p_channel, p_identifier, v_hash,
        p_type, p_source, p_reason, p_permanent, p_user_id
    )
    ON CONFLICT (channel, identifier_hash) DO UPDATE SET
        suppression_type = EXCLUDED.suppression_type,
        source = EXCLUDED.source,
        reason = EXCLUDED.reason,
        permanent = EXCLUDED.permanent OR communication_suppressions.permanent,
        created_at = NOW()
    RETURNING id INTO v_suppression_id;

    RETURN v_suppression_id;
END;
$$ LANGUAGE plpgsql;

-- Schedule message with preference checking
CREATE OR REPLACE FUNCTION schedule_message(
    p_user_id VARCHAR(255),
    p_channel comm_channel,
    p_category message_category,
    p_recipient VARCHAR(255),
    p_subject VARCHAR(500),
    p_body TEXT,
    p_scheduled_at TIMESTAMPTZ DEFAULT NOW(),
    p_template_code VARCHAR(100) DEFAULT NULL,
    p_template_data JSONB DEFAULT NULL,
    p_correlation_id VARCHAR(255) DEFAULT NULL
)
RETURNS JSONB AS $$
DECLARE
    v_can_receive JSONB;
    v_message_id UUID;
    v_prefs RECORD;
BEGIN
    -- Check if user can receive message
    v_can_receive := can_receive_message(p_user_id, p_channel, p_category, p_recipient);

    IF NOT (v_can_receive->>'can_send')::BOOLEAN THEN
        RETURN jsonb_build_object(
            'scheduled', FALSE,
            'reason', v_can_receive->>'reason'
        );
    END IF;

    -- Get user preferences for timezone
    SELECT * INTO v_prefs
    FROM communication_preferences
    WHERE user_id = p_user_id;

    -- Create scheduled message
    INSERT INTO scheduled_messages (
        user_id, channel, category,
        recipient_address, subject, body,
        scheduled_at, user_timezone,
        template_code, template_data,
        correlation_id
    )
    VALUES (
        p_user_id, p_channel, p_category,
        p_recipient, p_subject, p_body,
        p_scheduled_at, COALESCE(v_prefs.timezone, 'UTC'),
        p_template_code, p_template_data,
        p_correlation_id
    )
    RETURNING id INTO v_message_id;

    RETURN jsonb_build_object(
        'scheduled', TRUE,
        'message_id', v_message_id
    );
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- TRIGGERS
-- ============================================

-- Update timestamp trigger
CREATE OR REPLACE FUNCTION update_comm_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_comm_prefs_updated
    BEFORE UPDATE ON communication_preferences
    FOR EACH ROW EXECUTE FUNCTION update_comm_timestamp();

CREATE TRIGGER trg_templates_updated
    BEFORE UPDATE ON message_templates
    FOR EACH ROW EXECUTE FUNCTION update_comm_timestamp();

CREATE TRIGGER trg_device_tokens_updated
    BEFORE UPDATE ON device_tokens
    FOR EACH ROW EXECUTE FUNCTION update_comm_timestamp();

-- ============================================
-- COMMENTS
-- ============================================

COMMENT ON TABLE communication_preferences IS 'User communication preferences and opt-outs';
COMMENT ON TABLE communication_suppressions IS 'Suppression list for bounces, complaints, and unsubscribes';
COMMENT ON TABLE message_templates IS 'Multi-channel message templates with approval workflow';
COMMENT ON TABLE scheduled_messages IS 'Queue of scheduled messages to be sent';
COMMENT ON TABLE message_delivery_log IS 'Historical log of all sent messages';
COMMENT ON TABLE device_tokens IS 'Push notification device tokens';
COMMENT ON FUNCTION can_receive_message IS 'Check if user can receive message based on preferences and suppressions';
COMMENT ON FUNCTION add_suppression IS 'Add or update suppression entry';
COMMENT ON FUNCTION schedule_message IS 'Schedule a message with automatic preference checking';

