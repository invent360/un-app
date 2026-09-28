# UNO Web Application - Phase 1: MVP Foundation

## Overview
Implementing Phase 1 of the UNO Web Application as specified in `plans/05_webapp_implementation_plan.md`.

**Goal**: Create MVP foundation with database, CSV import, basic API, and minimal UI for license claiming.

---

## Week 1: Database & Backend Setup

### 1.1 Project Structure
- [ ] Create `uno-api` Rust backend project (Actix-web)
- [ ] Set up workspace with shared types crate
- [ ] Configure SQLx for PostgreSQL
- [ ] Set up environment configuration

### 1.2 Database Schema
- [ ] Create migration for `licenses` table
- [ ] Create migration for `license_variants` table
- [ ] Create migration for `license_claims` table
- [ ] Create migration for `visitor_stats` table
- [ ] Create migration for `performance_stats` table
- [ ] Create migration for `faq_items` table
- [ ] Create migration for `chatbot_knowledge` table
- [ ] Create migration for `contact_info` table
- [ ] Create database views (v_available_variants, v_unclaimed_licenses)
- [ ] Create `claim_license` function
- [ ] Create `sync_license_variants` function
- [ ] Add seed data for FAQ items

### 1.3 CSV Import Service
- [ ] Parse Unetwork CSV format
- [ ] Map CSV fields to database schema
- [ ] Handle duplicate detection
- [ ] Create license variants from imports
- [ ] Generate import report

### 1.4 Basic API Structure
- [ ] Set up Actix-web server
- [ ] Configure CORS
- [ ] Set up rate limiting middleware
- [ ] Configure logging
- [ ] Health check endpoint

---

## Week 2: Core API Endpoints

### 2.1 License Endpoints
- [ ] GET /api/v1/licenses/variants
- [ ] POST /api/v1/licenses/claim

### 2.2 FAQ Endpoint
- [ ] GET /api/v1/faq (basic)

### 2.3 Admin Endpoints
- [ ] POST /api/v1/admin/import-licenses (protected)

---

## Week 3: Minimal UI & Testing

### 3.1 Frontend Structure
- [ ] Integrate with existing Leptos frontend
- [ ] Create license selection page
- [ ] Create simple claim flow
- [ ] Create license reveal page

### 3.2 Testing
- [ ] Unit tests for CSV import
- [ ] Unit tests for claim logic
- [ ] Integration tests for API endpoints
- [ ] Manual testing with real CSV

---

## Gate 1 Verification Criteria
- [ ] Can import Unetwork CSV successfully
- [ ] Variants are correctly aggregated
- [ ] Can claim a license and receive key
- [ ] Claimed license shows as unavailable

---

## Current Status
**Started**: Phase 1 implementation
**Current Task**: Setting up project structure

---

## Progress Log

### Session 1 - Initial Setup
- Created tasks/todo.md for tracking
- Analyzed existing codebase structure
- Ready to create backend project structure
