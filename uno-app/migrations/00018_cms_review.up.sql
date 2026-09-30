-- Migration 00018: CMS Review Workflow Tables
-- Phase 1 (P1-03): Add missing tables for content review and preview functionality

-- Create review status enum
DO $$ BEGIN
    CREATE TYPE review_status AS ENUM ('pending', 'approved', 'changes_requested', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

-- Content versions table for tracking page_contents revisions
-- Note: content_item_versions (00012) tracks schema-driven CMS items
-- This tracks legacy page_contents revisions for the review workflow
CREATE TABLE IF NOT EXISTS content_versions (
    id SERIAL PRIMARY KEY,
    content_id INT NOT NULL REFERENCES page_contents(id) ON DELETE CASCADE,
    version INT NOT NULL,
    change_summary TEXT,
    content_snapshot JSONB, -- Stores the content state at this version
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255),
    CONSTRAINT uq_content_version UNIQUE (content_id, version)
);

-- Content reviews table for approval workflow
CREATE TABLE IF NOT EXISTS content_reviews (
    id SERIAL PRIMARY KEY,
    version_id INT NOT NULL REFERENCES content_versions(id) ON DELETE CASCADE,
    status review_status NOT NULL DEFAULT 'pending',
    submitted_by VARCHAR(255) NOT NULL,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_notes TEXT,
    reviewed_by VARCHAR(255),
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,
    CONSTRAINT uq_review_version UNIQUE (version_id)
);

-- Preview tokens for secure content preview before publishing
CREATE TABLE IF NOT EXISTS preview_tokens (
    id SERIAL PRIMARY KEY,
    version_id INT NOT NULL REFERENCES content_versions(id) ON DELETE CASCADE,
    token VARCHAR(64) UNIQUE NOT NULL,
    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_content_versions_content_id ON content_versions(content_id);
CREATE INDEX IF NOT EXISTS idx_content_reviews_status ON content_reviews(status);
CREATE INDEX IF NOT EXISTS idx_content_reviews_submitted_by ON content_reviews(submitted_by);
CREATE INDEX IF NOT EXISTS idx_content_reviews_submitted_at ON content_reviews(submitted_at DESC);
CREATE INDEX IF NOT EXISTS idx_preview_tokens_expires_at ON preview_tokens(expires_at);

-- Add status column to page_contents if not exists (for review state tracking)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'page_contents' AND column_name = 'status'
    ) THEN
        ALTER TABLE page_contents ADD COLUMN status VARCHAR(50) DEFAULT 'draft';
    END IF;
END $$;
