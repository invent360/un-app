-- Rollback: Support Ticket System

DROP TRIGGER IF EXISTS support_queue_updated ON support_queue_assignments;
DROP TRIGGER IF EXISTS support_tickets_updated ON support_tickets;
DROP FUNCTION IF EXISTS auto_assign_ticket(UUID);
DROP FUNCTION IF EXISTS generate_ticket_number();
DROP FUNCTION IF EXISTS update_support_ticket_timestamp();

DROP TABLE IF EXISTS support_canned_responses;
DROP TABLE IF EXISTS support_queue_assignments;
DROP TABLE IF EXISTS support_ticket_history;
DROP TABLE IF EXISTS support_ticket_messages;
DROP TABLE IF EXISTS support_tickets;

DROP TYPE IF EXISTS ticket_category;
DROP TYPE IF EXISTS ticket_priority;
DROP TYPE IF EXISTS ticket_status;
