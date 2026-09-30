-- Phase 8: Communication Preferences - Rollback

DROP TRIGGER IF EXISTS trg_device_tokens_updated ON device_tokens;
DROP TRIGGER IF EXISTS trg_templates_updated ON message_templates;
DROP TRIGGER IF EXISTS trg_comm_prefs_updated ON communication_preferences;
DROP FUNCTION IF EXISTS update_comm_timestamp;
DROP FUNCTION IF EXISTS schedule_message;
DROP FUNCTION IF EXISTS add_suppression;
DROP FUNCTION IF EXISTS can_receive_message;
DROP TABLE IF EXISTS device_tokens;
DROP TABLE IF EXISTS message_delivery_log;
DROP TABLE IF EXISTS scheduled_messages;
DROP TABLE IF EXISTS message_templates;
DROP TABLE IF EXISTS communication_suppressions;
DROP TABLE IF EXISTS communication_preferences;
DROP TYPE IF EXISTS scheduled_message_status;
DROP TYPE IF EXISTS template_status;
DROP TYPE IF EXISTS suppression_type;
DROP TYPE IF EXISTS message_category;
DROP TYPE IF EXISTS comm_channel;

