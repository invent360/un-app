-- Drop CMS Tables
-- Order: reverse of creation (respecting foreign key dependencies)
-- NOTE: content_item_versions is in migration 00012, not here

DROP TABLE IF EXISTS faq_items;
DROP TABLE IF EXISTS testimonials;
DROP TABLE IF EXISTS content_relations;
DROP TABLE IF EXISTS scheduled_publishes;
-- content_versions was removed - it's now content_item_versions in 00012
DROP TABLE IF EXISTS content_items;
DROP TABLE IF EXISTS content_schemas;

-- Drop enum types
DROP TYPE IF EXISTS content_status;
