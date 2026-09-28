-- Drop CMS Tables
-- Order: reverse of creation (respecting foreign key dependencies)

DROP TABLE IF EXISTS faq_items;
DROP TABLE IF EXISTS testimonials;
DROP TABLE IF EXISTS content_relations;
DROP TABLE IF EXISTS scheduled_publishes;
DROP TABLE IF EXISTS content_versions;
DROP TABLE IF EXISTS content_items;
DROP TABLE IF EXISTS content_schemas;

-- Drop enum types
DROP TYPE IF EXISTS content_status;
