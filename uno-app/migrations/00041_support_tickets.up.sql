-- Migration: Support Ticket System (Phase 7)
-- P7-04, P7-06: Support queues, ticket management, escalation

-- Ticket status lifecycle
CREATE TYPE ticket_status AS ENUM (
    'open',           -- New ticket, awaiting assignment
    'in_progress',    -- Agent working on it
    'waiting_user',   -- Waiting for user response
    'resolved',       -- Resolution provided
    'closed'          -- Closed (manually or auto after resolution)
);

-- Ticket priority levels
CREATE TYPE ticket_priority AS ENUM (
    'low',            -- Non-urgent questions
    'normal',         -- Standard support requests
    'high',           -- Time-sensitive issues
    'urgent'          -- Critical/blocking issues
);

-- Ticket categories for routing
CREATE TYPE ticket_category AS ENUM (
    'account',        -- Account/login issues
    'technical',      -- App/device technical issues
    'billing',        -- Payment/payout questions
    'claim',          -- License claim issues
    'eligibility',    -- Eligibility questions
    'other'           -- Catch-all
);

-- Main support tickets table
CREATE TABLE IF NOT EXISTS support_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(20) UNIQUE NOT NULL,

    -- User info
    user_id VARCHAR(255) NOT NULL,
    user_email VARCHAR(255),
    user_country VARCHAR(2),

    -- Ticket details
    category ticket_category NOT NULL,
    priority ticket_priority NOT NULL DEFAULT 'normal',
    status ticket_status NOT NULL DEFAULT 'open',
    subject VARCHAR(255) NOT NULL,
    description TEXT,

    -- Assignment
    assigned_agent_id UUID REFERENCES agents(id) ON DELETE SET NULL,
    assigned_at TIMESTAMPTZ,

    -- Escalation
    escalated BOOLEAN NOT NULL DEFAULT FALSE,
    escalated_at TIMESTAMPTZ,
    escalation_reason TEXT,
    escalated_by VARCHAR(255),

    -- Resolution
    resolved_at TIMESTAMPTZ,
    resolved_by VARCHAR(255),
    resolution_notes TEXT,
    resolution_type VARCHAR(50), -- 'answered', 'fixed', 'referred', 'no_action_needed'

    -- SLA tracking
    first_response_at TIMESTAMPTZ,
    first_response_sla_met BOOLEAN,
    resolution_sla_met BOOLEAN,

    -- Related entities
    license_id VARCHAR(66),

    -- Metadata
    tags VARCHAR(50)[] DEFAULT '{}',
    custom_fields JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ
);

-- Ticket messages (conversation thread)
CREATE TABLE IF NOT EXISTS support_ticket_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,

    -- Sender info
    sender_type VARCHAR(20) NOT NULL CHECK (sender_type IN ('user', 'agent', 'system')),
    sender_id VARCHAR(255) NOT NULL,
    sender_name VARCHAR(255),

    -- Message content
    message TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text' CHECK (message_type IN ('text', 'html', 'system')),

    -- Attachments
    attachments JSONB DEFAULT '[]',

    -- Visibility
    is_internal BOOLEAN NOT NULL DEFAULT FALSE, -- Internal notes not visible to user

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMPTZ,
    edited_by VARCHAR(255)
);

-- Ticket history for audit trail
CREATE TABLE IF NOT EXISTS support_ticket_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,

    -- Change details
    action VARCHAR(50) NOT NULL, -- 'created', 'updated', 'assigned', 'escalated', 'resolved', 'closed', 'reopened'
    field_changed VARCHAR(50),
    old_value TEXT,
    new_value TEXT,

    -- Actor
    changed_by VARCHAR(255) NOT NULL,
    changed_by_type VARCHAR(20) NOT NULL CHECK (changed_by_type IN ('user', 'agent', 'system')),

    -- Metadata
    change_reason TEXT,

    -- Timestamp
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Support queue assignments (for fair distribution)
CREATE TABLE IF NOT EXISTS support_queue_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id UUID NOT NULL REFERENCES agents(id) ON DELETE CASCADE,

    -- Queue settings
    categories ticket_category[] NOT NULL DEFAULT '{}', -- Categories this agent handles
    countries VARCHAR(2)[] NOT NULL DEFAULT '{}', -- Countries this agent handles (empty = all)
    max_concurrent_tickets INT NOT NULL DEFAULT 10,

    -- Status
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    away_reason TEXT,
    away_until TIMESTAMPTZ,

    -- Stats
    current_ticket_count INT NOT NULL DEFAULT 0,
    total_tickets_handled INT NOT NULL DEFAULT 0,
    avg_resolution_time_mins INT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(agent_id)
);

-- Canned responses for common issues
CREATE TABLE IF NOT EXISTS support_canned_responses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Response details
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    category ticket_category,

    -- Metadata
    tags VARCHAR(50)[] DEFAULT '{}',
    locale VARCHAR(5) NOT NULL DEFAULT 'en',

    -- Usage tracking
    use_count INT NOT NULL DEFAULT 0,
    last_used_at TIMESTAMPTZ,

    -- Ownership
    created_by VARCHAR(255) NOT NULL,
    is_shared BOOLEAN NOT NULL DEFAULT TRUE, -- Available to all agents

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX idx_support_tickets_user ON support_tickets(user_id);
CREATE INDEX idx_support_tickets_status ON support_tickets(status);
CREATE INDEX idx_support_tickets_assigned ON support_tickets(assigned_agent_id) WHERE assigned_agent_id IS NOT NULL;
CREATE INDEX idx_support_tickets_category ON support_tickets(category);
CREATE INDEX idx_support_tickets_priority ON support_tickets(priority);
CREATE INDEX idx_support_tickets_created ON support_tickets(created_at DESC);
CREATE INDEX idx_support_tickets_country ON support_tickets(user_country) WHERE user_country IS NOT NULL;
CREATE INDEX idx_support_tickets_open ON support_tickets(status, created_at) WHERE status IN ('open', 'in_progress');
CREATE INDEX idx_support_tickets_escalated ON support_tickets(escalated, escalated_at) WHERE escalated = TRUE;

CREATE INDEX idx_support_messages_ticket ON support_ticket_messages(ticket_id);
CREATE INDEX idx_support_messages_created ON support_ticket_messages(ticket_id, created_at);

CREATE INDEX idx_support_history_ticket ON support_ticket_history(ticket_id);
CREATE INDEX idx_support_history_changed ON support_ticket_history(ticket_id, changed_at DESC);

CREATE INDEX idx_support_queue_agent ON support_queue_assignments(agent_id);
CREATE INDEX idx_support_queue_available ON support_queue_assignments(is_available) WHERE is_available = TRUE;

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_support_ticket_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER support_tickets_updated
    BEFORE UPDATE ON support_tickets
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

CREATE TRIGGER support_queue_updated
    BEFORE UPDATE ON support_queue_assignments
    FOR EACH ROW
    EXECUTE FUNCTION update_support_ticket_timestamp();

-- Function to generate ticket number
CREATE OR REPLACE FUNCTION generate_ticket_number()
RETURNS VARCHAR(20) AS $$
DECLARE
    year_part VARCHAR(4);
    seq_num INT;
    ticket_num VARCHAR(20);
BEGIN
    year_part := TO_CHAR(NOW(), 'YYYY');

    -- Get next sequence number for this year
    SELECT COALESCE(MAX(CAST(SUBSTRING(ticket_number FROM 8) AS INT)), 0) + 1
    INTO seq_num
    FROM support_tickets
    WHERE ticket_number LIKE 'TKT' || year_part || '%';

    ticket_num := 'TKT' || year_part || LPAD(seq_num::TEXT, 6, '0');
    RETURN ticket_num;
END;
$$ LANGUAGE plpgsql;

-- Function to auto-assign ticket to available agent
CREATE OR REPLACE FUNCTION auto_assign_ticket(p_ticket_id UUID)
RETURNS UUID AS $$
DECLARE
    v_ticket support_tickets%ROWTYPE;
    v_agent_id UUID;
BEGIN
    -- Get ticket details
    SELECT * INTO v_ticket FROM support_tickets WHERE id = p_ticket_id;

    IF v_ticket IS NULL THEN
        RETURN NULL;
    END IF;

    -- Find available agent with capacity
    SELECT sqa.agent_id INTO v_agent_id
    FROM support_queue_assignments sqa
    JOIN agents a ON a.id = sqa.agent_id
    WHERE sqa.is_available = TRUE
      AND sqa.current_ticket_count < sqa.max_concurrent_tickets
      AND a.status = 'approved'
      AND (sqa.countries = '{}' OR v_ticket.user_country = ANY(sqa.countries))
      AND (sqa.categories = '{}' OR v_ticket.category = ANY(sqa.categories))
    ORDER BY sqa.current_ticket_count ASC, RANDOM()
    LIMIT 1
    FOR UPDATE SKIP LOCKED;

    IF v_agent_id IS NOT NULL THEN
        -- Assign ticket
        UPDATE support_tickets
        SET assigned_agent_id = v_agent_id,
            assigned_at = NOW(),
            status = 'in_progress'
        WHERE id = p_ticket_id;

        -- Update agent count
        UPDATE support_queue_assignments
        SET current_ticket_count = current_ticket_count + 1
        WHERE agent_id = v_agent_id;

        -- Log assignment
        INSERT INTO support_ticket_history (ticket_id, action, field_changed, new_value, changed_by, changed_by_type)
        VALUES (p_ticket_id, 'assigned', 'assigned_agent_id', v_agent_id::TEXT, 'system', 'system');
    END IF;

    RETURN v_agent_id;
END;
$$ LANGUAGE plpgsql;

-- Comments
COMMENT ON TABLE support_tickets IS 'Support ticket tracking for participant and agent issues';
COMMENT ON TABLE support_ticket_messages IS 'Conversation thread for support tickets';
COMMENT ON TABLE support_ticket_history IS 'Audit trail for ticket changes';
COMMENT ON TABLE support_queue_assignments IS 'Agent queue configuration for ticket routing';
COMMENT ON TABLE support_canned_responses IS 'Pre-written responses for common issues';
