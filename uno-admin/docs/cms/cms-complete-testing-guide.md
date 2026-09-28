# CMS Complete Testing Guide - Manual Testing Documentation

**Version**: 2.1
**Date**: June 1, 2026
**Testers**: UI/QA Team
**Applications**: uno-admin (Admin Dashboard), uno-app (Public Frontend)
**Scope**: End-to-end CMS workflow testing

---

## Table of Contents

1. [Overview & Objectives](#1-overview--objectives)
2. [Test Environment Setup](#2-test-environment-setup)
3. [Content Types Reference](#3-content-types-reference)
4. [Phase 1: Schema Management](#4-phase-1-schema-management)
5. [Phase 2: Content Creation](#5-phase-2-content-creation)
   - [5.1 Tasks Content](#51-tasks-content)
   - [5.2 Guides Content](#52-guides-content)
   - [5.3 FAQ Content](#53-faq-content)
   - [5.4 Common Errors Content](#54-common-errors-content)
6. [Phase 3: Content Preview](#6-phase-3-content-preview)
7. [Phase 4: Review Workflow](#7-phase-4-review-workflow)
8. [Phase 5: Publish Queue & Publishing](#8-phase-5-publish-queue--publishing)
9. [Phase 6: Version Control & History](#9-phase-6-version-control--history)
10. [Phase 7: Public Frontend Verification](#10-phase-7-public-frontend-verification)
11. [Phase 8: Translation Management](#11-phase-8-translation-management)
12. [Phase 9: Advanced Features](#12-phase-9-advanced-features)
13. [Phase 10: Error Scenarios & Edge Cases](#13-phase-10-error-scenarios--edge-cases)
14. [Quick Reference Cards](#14-quick-reference-cards)
15. [Bug Report Template](#15-bug-report-template)
16. [Sign-Off Checklist](#16-sign-off-checklist)

---

## 1. Overview & Objectives

### Purpose
This document provides step-by-step instructions for manually testing the complete CMS workflow from content creation in uno-admin through to display on uno-app public pages. All tests are designed to work **without requiring uno-app redeployment**.

### Testing Flow Overview
```
┌─────────────────────────────────────────────────────────────────────────┐
│                        CMS TESTING WORKFLOW                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  1. SCHEMA          2. CREATE           3. PREVIEW                      │
│  ┌─────────┐        ┌─────────┐        ┌─────────┐                      │
│  │ Define  │───────►│ Content │───────►│ Preview │                      │
│  │ Schema  │        │ Editor  │        │ on App  │                      │
│  └─────────┘        └─────────┘        └─────────┘                      │
│                          │                  │                           │
│                          ▼                  │                           │
│  4. REVIEW          5. PUBLISH          6. LIVE                         │
│  ┌─────────┐        ┌─────────┐        ┌─────────┐                      │
│  │ Submit  │───────►│ Approve │───────►│ Public  │                      │
│  │ Review  │        │ & Push  │        │ Display │                      │
│  └─────────┘        └─────────┘        └─────────┘                      │
│       │                                     │                           │
│       ▼                                     ▼                           │
│  ┌─────────┐                          ┌─────────┐                       │
│  │ Request │                          │ Version │                       │
│  │ Changes │                          │ Control │                       │
│  └─────────┘                          └─────────┘                       │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Key Principle
All content changes should be visible on uno-app **immediately after publishing** without any server restart or redeployment.

---

## 2. Test Environment Setup

### Application URLs

| Environment | uno-admin URL | uno-app URL |
|-------------|---------------|-------------|
| Development | http://localhost:3001 | http://localhost:3000 |
| Staging | https://admin.staging.uno.network | https://staging.uno.network |
| Production | https://admin.uno.network | https://uno.network |

### Test Accounts

| Role | Email | Permissions |
|------|-------|-------------|
| Admin | admin@test.com | Full access - all features |
| Publisher | publisher@test.com | Create, Edit, Publish, Revert, Import, Schedule |
| Reviewer | reviewer@test.com | Create, Edit, Review, Approve/Reject |
| Author | author@test.com | Create, Edit, Submit for Review only |

### Starting the Applications

#### uno-app (Public Frontend)

```bash
cd /path/to/uno-app
ADMIN_CLIENT_ID=admin ADMIN_SECRET_KEY=secret123 cargo leptos serve
```

This starts uno-app on `http://localhost:3000` by default.

#### uno-admin (Admin Dashboard)

```bash
cd /path/to/uno-admin
LEPTOS_SITE_ADDR=127.0.0.1:3002 ADMIN_CLIENT_ID=admin ADMIN_SECRET_KEY=secret123 API_TOKEN=$TOKEN cargo leptos serve
```

This starts uno-admin on `http://localhost:3002`.

**Environment Variables:**

| Variable | Required | Description |
| -------- | -------- | ----------- |
| `ADMIN_CLIENT_ID` | Yes | Client ID for admin authentication |
| `ADMIN_SECRET_KEY` | Yes | Secret key for admin authentication |
| `LEPTOS_SITE_ADDR` | No | Server address (default: 127.0.0.1:3000) |
| `API_TOKEN` | No | API token for unetwork API (not needed for CMS testing) |

### Browser Setup

1. Use Chrome 120+ as primary browser
2. Open DevTools (F12) → Network tab → Disable cache
3. Have both uno-admin and uno-app open in separate tabs
4. Keep console open to monitor for errors

### Pre-Test Checklist
- [ ] Both uno-admin and uno-app are running
- [ ] You can access the admin dashboard
- [ ] Test accounts are available
- [ ] Browser cache is disabled
- [ ] Console is open for error monitoring

---

## 3. Content Types Reference

### Available Content Types (Schemas)

| Schema ID | Display Name | Public Page | Description |
|-----------|--------------|-------------|-------------|
| `task` | Task | `/tasks` | Step-by-step tasks for users |
| `guide` | Guide | `/guides` | Comprehensive how-to guides |
| `faq` | FAQ | `/faq` | Frequently asked questions |
| `error` | Common Error | `/errors` | Error messages and solutions |

### Schema Field Reference

#### Task Schema
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | Text | Yes | Task title |
| description | Text (multiline) | Yes | Short description |
| steps | Repeater | Yes | List of steps to complete |
| difficulty | Select | No | easy, medium, hard |
| estimated_time | Text | No | e.g., "5 minutes" |
| category | Select | No | Task category |
| tags | List | No | Related tags |

#### Guide Schema
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | Text | Yes | Guide title |
| summary | Rich Text | Yes | Executive summary |
| thumbnail | Media | No | Guide thumbnail image |
| video_url | Text | No | Video URL (YouTube, etc.) |
| difficulty | Select | No | easy, medium, hard |
| duration_minutes | Number | No | Estimated reading time |
| prerequisites | List | No | Required knowledge |
| sections | Repeater (Section Editor) | Yes | Multi-section content with steps |
| related_tasks | Reference List | No | Links to related tasks |

**Section Fields (within sections repeater):**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| title | Text | Yes | Section title |
| summary | Rich Text | No | Section summary |
| cover_images | Media List | No | Section cover images (max 500KB each) |
| steps | Repeater | Yes | Steps within the section |

**Step Fields (within steps repeater):**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| order | Number | No | Step order number |
| title | Text | Yes | Step title |
| description | Rich Text | Yes | Step description |
| images | Media List | No | Step images (max 500KB each) |
| videos | Media List | No | Step videos |

#### FAQ Schema
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| question | Text | Yes | The question |
| answer | Rich Text | Yes | Detailed answer |
| category | Select | Yes | FAQ category |
| helpful_count | Number | No | User helpfulness votes |

#### Error Schema
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| error_code | Text | Yes | Error code (e.g., E001) |
| error_message | Text | Yes | The error message |
| cause | Rich Text | Yes | Why this error occurs |
| solution | Rich Text | Yes | How to fix it |
| platform | MultiSelect | No | web, mobile, api |

---

## 4. Phase 1: Schema Management

### 4.1 View Schema List

**Test ID**: SCH-001
**Objective**: Verify schema list displays correctly

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login to uno-admin as Admin | Dashboard loads | |
| 2 | Click "CMS" in sidebar | CMS section opens | |
| 3 | Click "Schemas" tab | Schema list page loads | |
| 4 | Verify table columns | Shows: ID, Name, Fields count, System badge, Actions | |
| 5 | Verify existing schemas | task, guide, faq, error schemas visible | |
| 6 | Verify "New Schema" button | Button visible for Admin/Publisher | |

### 4.2 View Schema Details

**Test ID**: SCH-002
**Objective**: Verify schema editor shows field configuration

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click Edit on "task" schema | Schema editor opens | |
| 2 | Verify basic info | ID, Name, Description fields populated | |
| 3 | Verify fields section | All task fields listed with types | |
| 4 | Verify settings panel | Shows: has_slug, versioned, translatable, orderable | |
| 5 | Click "Configure" on a field | Configuration panel expands | |
| 6 | Verify field config options | Shows type-specific settings | |

### 4.3 Create Custom Schema

**Test ID**: SCH-003
**Objective**: Verify new schema creation works

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click "New Schema" | Empty editor opens | |
| 2 | Enter ID: `announcement` | Field accepts input | |
| 3 | Enter Name: `Announcement` | Field accepts input | |
| 4 | Enter Description: `System announcements` | Field accepts input | |
| 5 | Click "+ Add Field" | New field row appears | |
| 6 | Set field: key=`title`, type=Text, Required | Field configured | |
| 7 | Add field: key=`message`, type=Rich Text | Second field added | |
| 8 | Add field: key=`priority`, type=Select | Third field added | |
| 9 | Configure Select options: low, medium, high | Options entered | |
| 10 | Enable "Versioned" setting | Checkbox checked | |
| 11 | Click "Save Schema" | Success message, schema saved | |
| 12 | Return to schema list | New "announcement" schema visible | |

### 4.4 Edit Schema Field Configuration

**Test ID**: SCH-004
**Objective**: Verify field configuration UI works

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Edit "announcement" schema | Editor opens | |
| 2 | Click Configure on "title" field | Config panel expands | |
| 3 | Set Min Length: 5 | Value accepted | |
| 4 | Set Max Length: 100 | Value accepted | |
| 5 | Set Placeholder: "Enter title..." | Value accepted | |
| 6 | Enable "Translatable" checkbox | Checkbox checked | |
| 7 | Enable "Show in List" checkbox | Checkbox checked | |
| 8 | Click "Save Schema" | Changes saved | |
| 9 | Reload page | Configuration persisted | |

---

## 5. Phase 2: Content Creation

> **Important**: When creating or editing content, always ensure the **"Schema-driven form"** checkbox is enabled in the Content section. This displays the correct fields for each content type based on its schema definition.

### 5.1 Tasks Content

#### TC-TASK-001: Create New Task

**Objective**: Create a complete task item from scratch

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to Content → New Content | Editor opens | |
| 2 | Select Type: "Task" | Task form fields appear | |
| 3 | Enter Title: `How to Reset Your Password` | Text entered | |
| 4 | Enter Description: `Step-by-step guide to resetting your account password` | Text entered | |
| 5 | Add Step 1: `Go to the login page` | Step added | |
| 6 | Add Step 2: `Click 'Forgot Password'` | Step added | |
| 7 | Add Step 3: `Enter your email address` | Step added | |
| 8 | Add Step 4: `Check your inbox for reset link` | Step added | |
| 9 | Add Step 5: `Click the link and set new password` | Step added | |
| 10 | Select Difficulty: `easy` | Option selected | |
| 11 | Enter Estimated Time: `5 minutes` | Text entered | |
| 12 | Enter Slug: `reset-password` | Slug entered | |
| 13 | Click "Save Draft" | Success: "Content saved", status = Draft | |
| 14 | Note the Content ID | ID displayed in URL (e.g., `/content/123`) | |
| 15 | Verify in content list | New task visible with status "Draft" | |

#### TC-TASK-002: Edit Existing Task

**Objective**: Modify a task and verify version increment

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open the task created in TC-TASK-001 | Editor loads with data | |
| 2 | Note current version | Shows "v1" | |
| 3 | Change Title to: `How to Reset Your Password (Updated)` | Text changed | |
| 4 | Add Step 6: `Confirm your new password` | Step added | |
| 5 | Enter Change Summary: `Added confirmation step` | Summary entered | |
| 6 | Click "Save Draft" | Success message | |
| 7 | Verify version | Now shows "v2" | |
| 8 | Check Version History | Shows v1 and v2 with change summaries | |

#### TC-TASK-003: Create Task with All Field Types

**Objective**: Test all available field types in task schema

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create new Task content | Editor opens | |
| 2 | Fill all text fields | Text accepted | |
| 3 | Use rich text editor for body | Formatting works (bold, italic, lists) | |
| 4 | Add multiple steps (repeater) | Can add/remove/reorder steps | |
| 5 | Select from dropdown (difficulty) | Options display correctly | |
| 6 | Add tags (list field) | Multiple tags can be added | |
| 7 | Save draft | All field values saved | |
| 8 | Reload page | All values preserved | |

### 5.2 Guides Content

> **Note**: Guide sections use a specialized Section Editor with:
> - Table/accordion view showing sections with collapsible step lists
> - Modal-based section adding (click "Add Section" to open form)
> - **File upload for images** (not URL input) with 500KB size limit per file
> - Image preview thumbnails after upload
> - Inline step form (expands below "Add Step" button, not a separate modal)
> - Vertically aligned move up/down arrows for reordering

#### TC-GUIDE-001: Create Comprehensive Guide

**Objective**: Create a multi-section guide with steps using the Section Editor

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to Content → New Content | Editor opens | |
| 2 | Select Type: "Guide" | Guide form fields appear with Section Editor | |
| 3 | Enter Title: `Getting Started with UNO Network` | Text entered | |
| 4 | Enter Summary: `Complete beginner's guide to using the UNO Network platform` | Text entered | |
| 5 | Click "Add Section" button | Section modal opens with form fields | |
| 6 | **In Section Modal - Section 1:** | | |
| | - Enter Section Title: `Introduction` | Title entered | |
| | - Enter Section Summary: `Welcome to UNO Network...` | Summary entered (rich text editor) | |
| | - Click [+] button next to Cover Images | File picker dialog opens | |
| | - Select image file from local drive (under 500KB) | Image uploads and shows thumbnail preview | |
| | - Click "Add Step" button | **Step form expands inline below the button** | |
| 7 | **In Inline Step Form - Step 1:** | | |
| | - Enter Order: `1` | Order number entered | |
| | - Enter Step Title: `Download the App` | Title entered | |
| | - Enter Description: `Visit the app store...` | Description entered (rich text) | |
| | - Click [+] for Step Images | File picker opens | |
| | - Select one or more images (each under 500KB) | Images show as thumbnail previews | |
| | - Click "Save Step" button | Step saved, appears in steps list | |
| 8 | Click "Add Step" again, add 2 more steps | Steps visible in list within modal | |
| 9 | Click "Add Section" button in modal | Modal closes, Section 1 appears in table | |
| 10 | Verify Section 1 in table | Row shows title, step count badge | |
| 11 | Click expand arrow (chevron) on Section 1 | Accordion expands showing steps in read-only view | |
| 12 | **Add Section 2:** | | |
| | - Click "Add Section" button | New modal opens | |
| | - Title: `Creating Your Account` | Title entered | |
| | - Add 3-4 steps with images via file upload | Steps visible in modal | |
| | - Click "Add Section" | Section added to table | |
| 13 | **Add Section 3:** `Your First Transaction` | Third section added to table | |
| 14 | Verify move arrows on sections | Up/down arrows vertically aligned in single column | |
| 15 | Add Prerequisites: `Basic computer knowledge`, `Email account` | List items added | |
| 16 | Enter Slug: `getting-started` | Slug entered | |
| 17 | Click "Save Draft" | Success message | |
| 18 | Verify sections saved | All 3 sections visible in table with accordions | |

#### TC-GUIDE-002: Edit Section with Multiple Media (File Upload)

**Objective**: Test editing sections and adding multiple images via file upload with size validation

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Edit the guide from TC-GUIDE-001 | Editor opens with sections table | |
| 2 | Click edit (pencil icon) on Section 1 | Section modal opens with existing data | |
| 3 | Click [+] for Cover Images | File picker dialog opens | |
| 4 | Select multiple image files (each under 500KB) | All images upload and show thumbnail previews | |
| 5 | Verify image previews | Small thumbnail images displayed in a row | |
| 6 | Click [x] on an image thumbnail | Image removed from list | |
| 7 | Click edit on an existing step | Step expands inline for editing | |
| 8 | Add multiple images to step via file picker | Images shown as thumbnails | |
| 9 | Add multiple videos to step via file picker | Videos shown in list | |
| 10 | Click "Save Step" | Step changes saved | |
| 11 | Click "Update Section" button | Modal closes, section updated | |
| 12 | Verify section row in table | Shows updated step count | |
| 13 | Expand accordion to verify steps | Steps show with updated content | |
| 14 | Save draft | All changes persisted | |
| 15 | Reload page | Multiple media files preserved as data URLs | |

#### TC-GUIDE-003: Guide with Reference Fields

**Objective**: Test reference field functionality

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Edit the guide from TC-GUIDE-001 | Editor opens | |
| 2 | Scroll to "Related Tasks" field | Reference picker visible | |
| 3 | Click the reference picker | Dropdown opens with search | |
| 4 | Search for existing task | Results appear | |
| 5 | Select a task | Task added to references | |
| 6 | Add another reference | Multiple references allowed | |
| 7 | Save draft | References saved | |
| 8 | Reload page | References preserved | |

#### TC-GUIDE-004: Section Reordering and Deletion

**Objective**: Test section management features including vertically aligned move arrows

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Edit a guide with multiple sections | Sections shown in table with move arrows | |
| 2 | Verify move arrows layout | Up/down arrows are **vertically stacked** in single column | |
| 3 | Click up arrow on section 2 | Section moves to position 1 | |
| 4 | Click down arrow on section 1 | Section moves to position 2 | |
| 5 | Verify first section | Up arrow is hidden (can't move up) | |
| 6 | Verify last section | Down arrow is hidden (can't move down) | |
| 7 | Click delete (trash icon) on a section | Confirmation prompt (if any), section removed | |
| 8 | Verify remaining sections | Correct sections remain, reordering still works | |
| 9 | Edit a section and delete a step | Step removed from list | |
| 10 | Save draft | Deletions and reordering persisted | |
| 11 | Reload page | Order and deletions preserved | |

#### TC-GUIDE-005: Image Upload Size Validation

**Objective**: Test that images exceeding 500KB are rejected with clear error message

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create or edit a guide | Section Editor visible | |
| 2 | Click "Add Section" or edit existing | Section modal opens | |
| 3 | Click [+] to add Cover Image | File picker opens | |
| 4 | Select an image **larger than 500KB** | Error message displayed | |
| 5 | Verify error message | Shows: "File '[filename]' is too large (XKB). Maximum size is 500KB." | |
| 6 | Verify image NOT added | No thumbnail appears in list | |
| 7 | Select an image **under 500KB** | Image uploads successfully | |
| 8 | Verify thumbnail preview | Small preview image displayed | |
| 9 | Repeat test in step's Images field | Same 500KB limit applies | |
| 10 | Try adding multiple files (mix of valid/invalid) | Only valid files added, errors shown for oversized | |

> **Note**: The 500KB limit prevents API payload size errors. The actix-web server has a 2MB JSON payload limit, and base64-encoded images can be large.

### 5.3 FAQ Content

#### TC-FAQ-001: Create FAQ Entry

**Objective**: Create a FAQ question and answer

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to Content → New Content | Editor opens | |
| 2 | Select Type: "FAQ" | FAQ form fields appear | |
| 3 | Enter Question: `How do I contact customer support?` | Text entered | |
| 4 | Enter Answer (rich text): | | |
| | `You can contact our support team through:` | | |
| | `• Email: support@uno.network` | | |
| | `• Live chat on our website` | | |
| | `• Phone: 1-800-UNO-HELP` | Rich text with bullet list | |
| 5 | Select Category: `Support` | Category selected | |
| 6 | Enter Slug: `contact-support` | Slug entered | |
| 7 | Click "Save Draft" | Success message | |
| 8 | Verify FAQ saved | Visible in content list | |

#### TC-FAQ-002: Create Multiple FAQs for Category Testing

**Objective**: Create FAQs in different categories

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create FAQ in "Account" category | Saved successfully | |
| 2 | Create FAQ in "Billing" category | Saved successfully | |
| 3 | Create FAQ in "Technical" category | Saved successfully | |
| 4 | Filter content list by type "FAQ" | All 4 FAQs visible | |
| 5 | Verify category values | Each shows correct category | |

### 5.4 Common Errors Content

#### TC-ERROR-001: Create Error Documentation

**Objective**: Document a common error with solution

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to Content → New Content | Editor opens | |
| 2 | Select Type: "Error" | Error form fields appear | |
| 3 | Enter Error Code: `E1001` | Text entered | |
| 4 | Enter Error Message: `Connection timeout` | Text entered | |
| 5 | Enter Cause (rich text): | | |
| | `This error occurs when:` | | |
| | `• Network connection is unstable` | | |
| | `• Server is temporarily unavailable` | Rich text entered | |
| 6 | Enter Solution (rich text): | | |
| | `To resolve this error:` | | |
| | `1. Check your internet connection` | | |
| | `2. Wait 30 seconds and retry` | | |
| | `3. Contact support if issue persists` | Rich text entered | |
| 7 | Select Platform: `web`, `mobile` (multi-select) | Multiple options selected | |
| 8 | Enter Slug: `e1001-connection-timeout` | Slug entered | |
| 9 | Click "Save Draft" | Success message | |

---

## 6. Phase 3: Content Preview

### 6.1 Generate and Access Preview

#### TC-PREV-001: Basic Preview Generation

**Objective**: Generate preview and view on uno-app

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open draft Task content | Editor loads | |
| 2 | Verify status is "Draft" | Status badge shows "Draft" | |
| 3 | Click "Preview" button | New tab opens with uno-app | |
| 4 | Check URL | Contains `?preview_token=...` parameter | |
| 5 | Verify preview banner | Yellow/amber banner at top: "PREVIEW MODE" | |
| 6 | Verify content displays | Task title and details visible | |
| 7 | Verify styling matches | Looks same as published content | |

#### TC-PREV-002: Preview Specific Content Types

**Test for each content type**

| Content Type | Preview URL Pattern | Expected Display | Pass/Fail |
|--------------|---------------------|------------------|-----------|
| Task | `/tasks/[slug]?preview_token=xxx` | Task card/details | |
| Guide | `/guides/[slug]?preview_token=xxx` | Guide with sections | |
| FAQ | `/faq?preview_token=xxx` | FAQ list with question | |
| Error | `/errors/[code]?preview_token=xxx` | Error details page | |

#### TC-PREV-003: Preview Without Token

**Objective**: Verify draft content is hidden without token

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Copy preview URL | URL noted | |
| 2 | Remove `?preview_token=xxx` from URL | URL without token | |
| 3 | Navigate to URL without token | Page loads | |
| 4 | Check for draft content | Draft content NOT visible | |
| 5 | Should see 404 or "Not found" | Content hidden from public | |

#### TC-PREV-004: Preview Banner Functionality

**Objective**: Test preview banner UI

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open preview URL | Preview banner visible | |
| 2 | Verify banner text | "PREVIEW MODE - This content is not published" | |
| 3 | Verify banner color | Yellow/amber background | |
| 4 | Click "Exit Preview" button | Banner disappears | |
| 5 | URL changes | `preview_token` removed | |
| 6 | Content hidden | Draft content no longer visible | |

#### TC-PREV-005: Preview After Edits

**Objective**: Verify preview shows latest changes

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open draft content in editor | Editor loads | |
| 2 | Generate preview in new tab | Preview visible | |
| 3 | **Keep preview tab open** | Don't close it | |
| 4 | In editor tab, change the title | New title entered | |
| 5 | Save draft | Changes saved | |
| 6 | Refresh preview tab | **Updated title visible** | |
| 7 | Changes appear immediately | No deploy required | |

---

## 7. Phase 4: Review Workflow

### 7.1 Submit Content for Review

#### TC-REV-001: Submit Draft for Review

**Objective**: Test submission workflow

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Author | Author dashboard | |
| 2 | Open draft content | Editor loads | |
| 3 | Verify current status | "Draft" | |
| 4 | Click "Submit for Review" | Confirmation dialog | |
| 5 | Confirm submission | Success message | |
| 6 | Verify status changed | Now "Pending Review" | |
| 7 | "Submit for Review" button disabled | Cannot re-submit | |

#### TC-REV-002: View in Reviews Dashboard

**Objective**: Verify item appears in review queue

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Reviewer | Reviewer dashboard | |
| 2 | Navigate to Reviews (`/reviews`) | Review dashboard loads | |
| 3 | Find submitted content | Card visible in pending list | |
| 4 | Card shows | Title, Type, Submitter, Date | |
| 5 | Action buttons visible | Preview, Compare, Approve, Request Changes | |

### 7.2 Review Actions

#### TC-REV-003: Approve Content

**Objective**: Test approval workflow

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | On Reviews dashboard | Pending items visible | |
| 2 | Click "Preview" on item | Preview opens in new tab | |
| 3 | Review content quality | Content displays correctly | |
| 4 | Return to Reviews dashboard | | |
| 5 | Click "Approve" | Confirmation dialog (optional) | |
| 6 | Confirm approval | Success message | |
| 7 | Item removed from pending | Not in pending list | |
| 8 | Check content status | Now "Approved" | |

#### TC-REV-004: Request Changes

**Objective**: Test rejection/changes workflow

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Find pending item | Item visible | |
| 2 | Click "Request Changes" | Modal opens with notes field | |
| 3 | Leave notes empty, click submit | Validation error: "Notes required" | |
| 4 | Enter feedback: `Please fix typo in step 3 and add more detail to step 5` | Notes entered | |
| 5 | Click "Submit" | Success message | |
| 6 | Item removed from pending | Not in review queue | |
| 7 | Check content status | Back to "Draft" | |
| 8 | **Login as Author** | | |
| 9 | Open the content | Feedback visible somewhere | |

#### TC-REV-005: Compare Versions in Review

**Objective**: Test diff viewer during review

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Find item with multiple versions | Item in review queue | |
| 2 | Click "Compare" | Diff viewer opens | |
| 3 | Shows comparison | Previous version vs current | |
| 4 | Changes highlighted | Added=green, removed=red, changed=blue | |
| 5 | Close diff viewer | Returns to review dashboard | |

### 7.3 Review Permissions

#### TC-REV-006: Self-Review Prevention

**Objective**: Author cannot approve own content

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Author | Author dashboard | |
| 2 | Create and submit content | Status = Pending Review | |
| 3 | Navigate to Reviews dashboard | Dashboard loads | |
| 4 | Find your own submission | Item visible (or not) | |
| 5 | Try to click Approve | Button disabled OR hidden | |
| 6 | Tooltip shows | "Cannot approve your own submission" | |

#### TC-REV-007: Role-Based Review Access

| Role | Can See Reviews | Can Approve | Can Request Changes |
|------|-----------------|-------------|---------------------|
| Author | Limited/No | No | No |
| Reviewer | Yes | Yes (others' content) | Yes |
| Publisher | Yes | Yes | Yes |
| Admin | Yes | Yes | Yes |

---

## 8. Phase 5: Publish Queue & Publishing

### 8.1 Publish Queue

#### TC-PUB-001: View Publish Queue

**Objective**: Verify publish queue displays approved content

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Publisher | Publisher dashboard | |
| 2 | Navigate to Publish (`/publish`) | Publish queue loads | |
| 3 | Only approved content shown | No draft or pending items | |
| 4 | Each item shows | Title, Type, Approved by, Date | |
| 5 | Checkboxes available | Can select items | |

#### TC-PUB-002: Single Item Publish

**Objective**: Publish one content item

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Check one item | Item selected | |
| 2 | Click "Publish Selected" | Confirmation dialog | |
| 3 | Confirm publish | Progress indicator | |
| 4 | Wait for completion | Success: "1 item published" | |
| 5 | Item removed from queue | Queue refreshes | |
| 6 | Check content status | Now "Published" | |
| 7 | **Check uno-app** | Content visible on public site | |

#### TC-PUB-003: Batch Publish

**Objective**: Publish multiple items at once

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Ensure 3+ approved items | Items in queue | |
| 2 | Click "Select All" | All items checked | |
| 3 | Counter shows | "Selected: X items" | |
| 4 | Click "Publish Selected" | Confirmation with warning | |
| 5 | Warning text | "This will make X items live" | |
| 6 | Confirm | Progress indicator | |
| 7 | All items publish | Success message with count | |
| 8 | Queue empty or reduced | Published items removed | |

### 8.2 Verify Published Content on uno-app

#### TC-PUB-004: Tasks Page Verification

**Objective**: Verify published task appears on /tasks

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Publish a Task | Task status = Published | |
| 2 | Open uno-app `/tasks` | Tasks page loads | |
| 3 | Find published task | Task card visible | |
| 4 | Click task | Task detail page opens | |
| 5 | Verify all fields | Title, description, steps display correctly | |
| 6 | **No preview banner** | Banner NOT shown (it's live) | |

#### TC-PUB-005: Guides Page Verification

**Objective**: Verify published guide appears on /guides

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Publish a Guide | Guide status = Published | |
| 2 | Open uno-app `/guides` | Guides page loads | |
| 3 | Find published guide | Guide card visible | |
| 4 | Click guide | Guide detail page opens | |
| 5 | Verify sections | All sections display with formatting | |
| 6 | Test section navigation | Can navigate between sections | |

#### TC-PUB-006: FAQ Page Verification

**Objective**: Verify published FAQ appears on /faq

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Publish an FAQ | FAQ status = Published | |
| 2 | Open uno-app `/faq` | FAQ page loads | |
| 3 | Find published FAQ | Question visible | |
| 4 | Click/expand FAQ | Answer reveals | |
| 5 | Verify rich text | Formatting preserved | |
| 6 | Verify category | Correct category displayed | |

#### TC-PUB-007: Errors Page Verification

**Objective**: Verify published error appears on /errors

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Publish an Error | Error status = Published | |
| 2 | Open uno-app `/errors` | Errors page loads | |
| 3 | Find published error | Error code visible | |
| 4 | Click error | Error detail page opens | |
| 5 | Verify fields | Code, message, cause, solution displayed | |

### 8.3 Real-Time Update Verification

#### TC-PUB-008: Immediate Visibility

**Critical Test**: Content appears without deployment

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | **Open uno-app in separate window** | Page loaded | |
| 2 | Note current content list | Count visible items | |
| 3 | In uno-admin, publish new content | Published successfully | |
| 4 | **Refresh uno-app** (F5) | Page reloads | |
| 5 | New content visible | **Immediately appears** | |
| 6 | No server restart needed | ✓ Verified | |

---

## 9. Phase 6: Version Control & History

### 9.1 Version History

#### TC-VER-001: View Version History

**Objective**: Access and view version history

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open content with multiple versions | Editor loads | |
| 2 | Find "Version History" section | Section visible | |
| 3 | Expand/view history | List of versions shown | |
| 4 | Each version shows | Version number, date, author, summary | |
| 5 | Current version marked | "Current" badge on active version | |

#### TC-VER-002: Version Details

**Objective**: View specific version information

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | View version list | Multiple versions shown | |
| 2 | Click "View" on older version | Version details displayed | |
| 3 | Shows change summary | Summary text visible | |
| 4 | Shows timestamp | Date/time of change | |
| 5 | Shows author | Who made the change | |

### 9.2 Version Diff Viewer

#### TC-VER-003: Compare Versions

**Objective**: View differences between versions

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click "View Diff" on a version | Diff modal opens | |
| 2 | Header shows | "Comparing vX vs vY" | |
| 3 | Field-by-field comparison | Each changed field shown | |
| 4 | Color coding correct | | |
| | - Added content | GREEN background | |
| | - Removed content | RED background | |
| | - Modified content | BLUE/highlighted | |
| 5 | Close modal | Click X or Escape | |

#### TC-VER-004: Diff with Translations

**Objective**: Verify diff shows all locales

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Edit content with translations | Add Spanish version | |
| 2 | Save as new version | v3 created | |
| 3 | View diff v2 vs v3 | Diff opens | |
| 4 | English changes shown | English section in diff | |
| 5 | Spanish changes shown | Spanish (es) section in diff | |
| 6 | Locale labels clear | "English", "Spanish" labels | |

### 9.3 Revert to Previous Version

#### TC-VER-005: Revert Content

**Objective**: Restore previous version content

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Publisher or Admin | Has revert permission | |
| 2 | Open content with 3+ versions | v1, v2, v3 exist | |
| 3 | Click "Revert" on v1 | Confirmation modal opens | |
| 4 | Modal shows warning | "This will create a new version with v1 content" | |
| 5 | Click "Cancel" | Modal closes, no changes | |
| 6 | Click "Revert" again | Modal opens | |
| 7 | Click "Confirm" | Processing... | |
| 8 | Success message | "Reverted to version 1" | |
| 9 | New version created | Now shows v4 | |
| 10 | Content matches v1 | Fields have v1 data | |

#### TC-VER-006: Revert Permission Check

**Objective**: Only authorized roles can revert

| Role | Can See Revert Button | Can Execute Revert |
|------|----------------------|-------------------|
| Author | No | No |
| Reviewer | No | No |
| Publisher | Yes | Yes |
| Admin | Yes | Yes |

Test each role:

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Author | Author dashboard | |
| 2 | Open content with versions | Editor loads | |
| 3 | Check version history | "Revert" buttons NOT visible | |
| 4 | Login as Publisher | Publisher dashboard | |
| 5 | Open same content | Editor loads | |
| 6 | Check version history | "Revert" buttons ARE visible | |

#### TC-VER-007: Revert Published Content

**Objective**: Revert updates published content

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open published content | Status = Published | |
| 2 | Note current title | "Title v3" | |
| 3 | Check v1 had different title | "Title v1" | |
| 4 | Revert to v1 | Success | |
| 5 | New version created | v4 with v1 content | |
| 6 | **Content still published** | Status unchanged | |
| 7 | Check uno-app | **Shows v1 title** (if re-published) | |

---

## 10. Phase 7: Public Frontend Verification

### 10.1 Tasks Page (`/tasks`)

#### TC-FRONT-001: Tasks List Page

**Objective**: Verify tasks page displays correctly

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to uno-app `/tasks` | Page loads | |
| 2 | Verify page title | "Tasks" or similar | |
| 3 | Published tasks display | Task cards visible | |
| 4 | Each card shows | Title, description, difficulty | |
| 5 | Draft tasks hidden | Not visible without preview token | |
| 6 | Cards clickable | Navigate to task detail | |

#### TC-FRONT-002: Task Detail Page

**Objective**: Verify task detail page

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click on a task card | Detail page loads | |
| 2 | URL correct | `/tasks/[slug]` | |
| 3 | Title displays | Task title shown | |
| 4 | Description displays | Full description | |
| 5 | Steps display | Numbered steps list | |
| 6 | Metadata displays | Difficulty, time estimate | |
| 7 | Navigation works | Can go back to list | |

### 10.2 Guides Page (`/guides`)

#### TC-FRONT-003: Guides List Page

**Objective**: Verify guides page displays correctly

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to uno-app `/guides` | Page loads | |
| 2 | Published guides display | Guide cards visible | |
| 3 | Each card shows | Title, summary | |
| 4 | Cards clickable | Navigate to guide detail | |

#### TC-FRONT-004: Guide Detail Page

**Objective**: Verify guide detail with sections

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click on a guide | Detail page loads | |
| 2 | Title and summary display | Header content visible | |
| 3 | Sections display | All sections visible | |
| 4 | Section titles work | Can see section headers | |
| 5 | Rich text formatting | Bold, italic, lists preserved | |
| 6 | Section navigation | Can jump to sections (if implemented) | |

### 10.3 FAQ Page (`/faq`)

#### TC-FRONT-005: FAQ Page

**Objective**: Verify FAQ accordion/list

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to uno-app `/faq` | Page loads | |
| 2 | FAQs displayed | Questions visible | |
| 3 | Click question | Answer expands | |
| 4 | Rich text in answer | Formatting preserved | |
| 5 | Click again | Answer collapses | |
| 6 | Multiple FAQs work | Each expands independently | |
| 7 | Categories displayed | FAQs grouped by category (if implemented) | |

### 10.4 Errors Page (`/errors`)

#### TC-FRONT-006: Errors List Page

**Objective**: Verify errors documentation page

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to uno-app `/errors` | Page loads | |
| 2 | Error codes display | List of error codes | |
| 3 | Error messages shown | Brief error descriptions | |
| 4 | Searchable (if implemented) | Can search by code | |

#### TC-FRONT-007: Error Detail Page

**Objective**: Verify error solution page

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click on error code | Detail page loads | |
| 2 | Error code displayed | Code prominently shown | |
| 3 | Error message displayed | Full message | |
| 4 | Cause section | Explains why error occurs | |
| 5 | Solution section | Step-by-step fix | |
| 6 | Platform tags | Shows affected platforms | |

---

## 11. Phase 8: Translation Management

### 11.1 Translation Editor

#### TC-TRANS-001: Access Translation Editor

**Objective**: View and navigate translation UI

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open content editor | Editor loads | |
| 2 | Find Translations section | Section visible | |
| 3 | Locale tabs visible | es, fr, ar, hi, pt, id, tl, sw | |
| 4 | English content shown | Original content as reference | |

#### TC-TRANS-002: Add Translation

**Objective**: Add Spanish translation

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Click "es" (Spanish) tab | Spanish fields appear | |
| 2 | English reference visible | Original text shown for reference | |
| 3 | Enter Spanish title | "Cómo restablecer tu contraseña" | |
| 4 | Enter Spanish description | Translation entered | |
| 5 | Translate other fields | All translatable fields filled | |
| 6 | Click "Save Draft" | Success message | |
| 7 | Reload page | Spanish translation preserved | |
| 8 | Translation coverage | Shows "1/8 locales" or similar | |

#### TC-TRANS-003: Multiple Languages

**Objective**: Add translations in multiple languages

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Add Spanish translation | Completed | |
| 2 | Click "fr" tab | French fields appear | |
| 3 | Add French translation | Fields filled | |
| 4 | Click "ar" tab | Arabic fields appear | |
| 5 | Verify RTL input | Text input right-to-left | |
| 6 | Add Arabic translation | Arabic text entered correctly | |
| 7 | Save all translations | Success | |
| 8 | Coverage updates | "3/8 locales" | |

#### TC-TRANS-004: Translation Persistence

**Objective**: Verify translations are saved correctly

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Add translations to multiple locales | Translations entered | |
| 2 | Save content | Success message | |
| 3 | Navigate away | Go to content list | |
| 4 | Return to editor | Open same content | |
| 5 | Check Spanish tab | Spanish text preserved | |
| 6 | Check French tab | French text preserved | |
| 7 | Check Arabic tab | Arabic text preserved with RTL | |

### 11.2 Translation Display on Frontend

#### TC-TRANS-005: View Translated Content

**Objective**: Verify translations display on uno-app

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Publish translated content | Status = Published | |
| 2 | Open uno-app | Default language (English) | |
| 3 | View content | English version displayed | |
| 4 | Change language to Spanish | Language selector (if available) | |
| 5 | Content updates | Spanish version displayed | |
| 6 | Change to Arabic | RTL layout applied | |

---

## 12. Phase 9: Advanced Features

### 12.1 Bulk Import

#### TC-IMP-001: Import Valid Content

**Objective**: Test bulk import functionality

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to Content | Content list | |
| 2 | Click "Import" button | Import page loads | |
| 3 | Prepare valid JSON file | See format below | |
| 4 | Upload file | File accepted | |
| 5 | Preview shows | Table with parsed items | |
| 6 | Click "Import" | Processing... | |
| 7 | Success summary | "Imported: X, Skipped: 0, Errors: 0" | |
| 8 | Check content list | New items visible | |

**Valid Import File Format**:
```json
[
  {
    "schema_id": "task",
    "slug": "imported-task-1",
    "data": {
      "title": "Imported Task 1",
      "description": "This was imported",
      "steps": ["Step 1", "Step 2"],
      "difficulty": "easy"
    }
  },
  {
    "schema_id": "faq",
    "slug": "imported-faq-1",
    "data": {
      "question": "Imported Question?",
      "answer": "Imported answer.",
      "category": "General"
    }
  }
]
```

#### TC-IMP-002: Import Error Handling

**Objective**: Test import validation

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create invalid JSON file | Missing required fields | |
| 2 | Upload file | File processed | |
| 3 | Preview shows errors | Red X on invalid rows | |
| 4 | Error messages | "Missing required field", "Invalid schema" | |
| 5 | Try to import | Only valid items imported | |

### 12.2 Content Scheduling

#### TC-SCHED-001: Schedule Future Publish

**Objective**: Set publish date/time

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open content editor | Editor loads | |
| 2 | Find Scheduling section | Section visible | |
| 3 | Enable scheduling toggle | Date/time fields appear | |
| 4 | Set "Publish At" | Future date/time selected | |
| 5 | Save content | Success message | |
| 6 | Content status | "Scheduled" | |
| 7 | **Wait for scheduled time** | (or adjust server time) | |
| 8 | Content auto-publishes | Status becomes "Published" | |

#### TC-SCHED-002: Schedule Unpublish

**Objective**: Set automatic unpublish

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Enable scheduling | Fields visible | |
| 2 | Set Publish At | Tomorrow 9:00 AM | |
| 3 | Set Unpublish At | Tomorrow 5:00 PM | |
| 4 | Save content | Success | |
| 5 | Verify validation | Unpublish must be after publish | |

### 12.3 Audit Log

#### TC-AUDIT-001: View Audit Trail

**Objective**: Track content changes

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Navigate to `/audit` | Audit log loads | |
| 2 | Table shows | Timestamp, Action, Content, Actor | |
| 3 | Create new content | | |
| 4 | Refresh audit log | "Create" action logged | |
| 5 | Edit content | | |
| 6 | Refresh audit log | "Update" action logged | |
| 7 | Publish content | | |
| 8 | Refresh audit log | "Publish" action logged | |

#### TC-AUDIT-002: Filter Audit Log

**Objective**: Test audit log filters

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Filter by Action: "Publish" | Only publish actions | |
| 2 | Filter by Actor | Only that user's actions | |
| 3 | Filter by Date Range | Actions within range | |
| 4 | Filter by Content ID | Actions for that content | |
| 5 | Clear Filters | All results shown | |

---

## 13. Phase 10: Error Scenarios & Edge Cases

### 13.1 Validation Errors

#### TC-ERR-001: Required Field Validation

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create new content | Editor opens | |
| 2 | Leave title empty | | |
| 3 | Click "Save Draft" | Validation error: "Title is required" | |
| 4 | Enter title | Validation passes | |
| 5 | Leave other required fields empty | Each shows error | |

#### TC-ERR-002: Slug Validation

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Enter slug with spaces | "my slug" | |
| 2 | Validation error | "Slug cannot contain spaces" | |
| 3 | Enter slug with special chars | "my@slug!" | |
| 4 | Validation error | "Invalid characters in slug" | |
| 5 | Enter valid slug | "my-valid-slug" | |
| 6 | Validation passes | Save succeeds | |

#### TC-ERR-003: Duplicate Slug

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Create content with slug "test-1" | Saved successfully | |
| 2 | Create another with same slug | Enter "test-1" | |
| 3 | Try to save | Error: "Slug already exists" | |

### 13.2 Permission Errors

#### TC-ERR-004: Unauthorized Actions

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Login as Author | Author role | |
| 2 | Try to access `/publish` | Access denied or buttons disabled | |
| 3 | Try to delete content | No delete button | |
| 4 | Try to revert version | No revert button | |

### 13.3 Network Errors

#### TC-ERR-005: Save with Network Error

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open content editor | Ready to edit | |
| 2 | Make changes | Title changed | |
| 3 | Disable network (DevTools) | Offline mode | |
| 4 | Click "Save Draft" | Loading indicator | |
| 5 | Error displayed | "Network error - please try again" | |
| 6 | Re-enable network | Online again | |
| 7 | Click "Save Draft" again | Saves successfully | |

### 13.4 Concurrent Editing

#### TC-ERR-006: Conflict Detection

| Step | Action | Expected Result | Pass/Fail |
|------|--------|-----------------|-----------|
| 1 | Open content in Browser A | Editor loads | |
| 2 | Open same content in Browser B | Editor loads | |
| 3 | Browser A: Change title | "Title from A" | |
| 4 | Browser A: Save | Success | |
| 5 | Browser B: Change title | "Title from B" | |
| 6 | Browser B: Save | Conflict warning (if implemented) | |
| 7 | Handle conflict | Overwrite or merge | |

---

## 14. Quick Reference Cards

### Status Flow Diagram
```
┌─────────┐     Submit      ┌────────────────┐
│  DRAFT  │────────────────►│ PENDING_REVIEW │
└─────────┘                 └────────────────┘
     ▲                              │
     │                    ┌─────────┴─────────┐
     │                    ▼                   ▼
     │            ┌──────────────┐     ┌──────────┐
     │            │ REQ_CHANGES  │     │ APPROVED │
     │            └──────────────┘     └──────────┘
     │                    │                   │
     └────────────────────┘                   │
                                              ▼
                                       ┌───────────┐
                                       │ PUBLISHED │
                                       └───────────┘
                                              │
                                              ▼
                                       ┌──────────┐
                                       │ ARCHIVED │
                                       └──────────┘
```

### Role Permissions Matrix

| Feature | Author | Reviewer | Publisher | Admin |
|---------|--------|----------|-----------|-------|
| Create content | ✓ | ✓ | ✓ | ✓ |
| Edit content | ✓ | ✓ | ✓ | ✓ |
| Submit for review | ✓ | ✓ | ✓ | ✓ |
| Review content | ✗ | ✓ | ✓ | ✓ |
| Approve/Reject | ✗ | ✓ | ✓ | ✓ |
| Publish content | ✗ | ✗ | ✓ | ✓ |
| Revert versions | ✗ | ✗ | ✓ | ✓ |
| Import content | ✗ | ✗ | ✓ | ✓ |
| Schedule content | ✗ | ✗ | ✓ | ✓ |
| Delete content | ✗ | ✗ | ✗ | ✓ |
| Manage schemas | ✗ | ✗ | ✓ | ✓ |
| Access settings | ✗ | ✗ | ✗ | ✓ |

### URL Quick Reference

| uno-admin Page | URL |
|----------------|-----|
| Dashboard | `/` |
| Content List | `/content` |
| Content Editor | `/content/:id` |
| New Content | `/content/new` |
| Import | `/content/import` |
| Schemas | `/schemas` |
| Schema Editor | `/schemas/:id` |
| Reviews | `/reviews` |
| Publish Queue | `/publish` |
| Audit Log | `/audit` |
| Settings | `/settings` |

| uno-app Page | URL |
|--------------|-----|
| Home | `/` |
| Tasks | `/tasks` |
| Task Detail | `/tasks/:slug` |
| Guides | `/guides` |
| Guide Detail | `/guides/:slug` |
| FAQ | `/faq` |
| Errors | `/errors` |
| Error Detail | `/errors/:code` |

---

## 15. Bug Report Template

```markdown
## Bug Report

**Bug ID**: CMS-BUG-XXX
**Reporter**: [Name]
**Date**: [Date]
**Severity**: Critical / High / Medium / Low
**Test Case**: [TC-XXX-XXX]

### Summary
[One-line description]

### Environment
- uno-admin URL: [URL]
- uno-app URL: [URL]
- Browser: Chrome 120 / Firefox / Safari / Edge
- OS: macOS 14 / Windows 11 / etc.
- User Role: Author / Reviewer / Publisher / Admin

### Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

### Expected Result
[What should happen]

### Actual Result
[What actually happened]

### Screenshots/Video
[Attach evidence]

### Console Errors
```
[Paste JavaScript console errors]
```

### Network Errors
[Any failed API requests from Network tab]

### Additional Notes
[Any other relevant information]
```

### Severity Definitions

| Severity | Definition | Example |
|----------|------------|---------|
| Critical | System crash, data loss, security issue | Cannot save any content, data deleted |
| High | Major feature completely broken | Publish button doesn't work |
| Medium | Feature degraded but workaround exists | Filter broken but search works |
| Low | Minor UI/cosmetic issue | Wrong icon, alignment off |

---

## 16. Sign-Off Checklist

### Phase Completion Checklist

#### Phase 1: Schema Management
- [ ] SCH-001: View Schema List
- [ ] SCH-002: View Schema Details
- [ ] SCH-003: Create Custom Schema
- [ ] SCH-004: Edit Schema Field Configuration

#### Phase 2: Content Creation
- [ ] TC-TASK-001: Create New Task
- [ ] TC-TASK-002: Edit Existing Task
- [ ] TC-TASK-003: All Field Types
- [ ] TC-GUIDE-001: Create Guide with Section Editor (modal + file upload)
- [ ] TC-GUIDE-002: Edit Section with Multiple Media (file upload)
- [ ] TC-GUIDE-003: Reference Fields
- [ ] TC-GUIDE-004: Section Reordering and Deletion (vertical arrows)
- [ ] TC-GUIDE-005: Image Upload Size Validation (500KB limit)
- [ ] TC-FAQ-001: Create FAQ
- [ ] TC-FAQ-002: Multiple Categories
- [ ] TC-ERROR-001: Create Error Doc

#### Phase 3: Content Preview
- [ ] TC-PREV-001: Basic Preview
- [ ] TC-PREV-002: All Content Types
- [ ] TC-PREV-003: No Token = Hidden
- [ ] TC-PREV-004: Banner Functionality
- [ ] TC-PREV-005: Preview After Edits

#### Phase 4: Review Workflow
- [ ] TC-REV-001: Submit for Review
- [ ] TC-REV-002: View Reviews Dashboard
- [ ] TC-REV-003: Approve Content
- [ ] TC-REV-004: Request Changes
- [ ] TC-REV-005: Compare Versions
- [ ] TC-REV-006: Self-Review Prevention
- [ ] TC-REV-007: Role-Based Access

#### Phase 5: Publishing
- [ ] TC-PUB-001: View Publish Queue
- [ ] TC-PUB-002: Single Item Publish
- [ ] TC-PUB-003: Batch Publish
- [ ] TC-PUB-004: Tasks Verification
- [ ] TC-PUB-005: Guides Verification
- [ ] TC-PUB-006: FAQ Verification
- [ ] TC-PUB-007: Errors Verification
- [ ] TC-PUB-008: Immediate Visibility

#### Phase 6: Version Control
- [ ] TC-VER-001: View Version History
- [ ] TC-VER-002: Version Details
- [ ] TC-VER-003: Compare Versions
- [ ] TC-VER-004: Diff with Translations
- [ ] TC-VER-005: Revert Content
- [ ] TC-VER-006: Revert Permissions
- [ ] TC-VER-007: Revert Published

#### Phase 7: Frontend Verification
- [ ] TC-FRONT-001: Tasks List
- [ ] TC-FRONT-002: Task Detail
- [ ] TC-FRONT-003: Guides List
- [ ] TC-FRONT-004: Guide Detail
- [ ] TC-FRONT-005: FAQ Page
- [ ] TC-FRONT-006: Errors List
- [ ] TC-FRONT-007: Error Detail

#### Phase 8: Translations
- [ ] TC-TRANS-001: Access Editor
- [ ] TC-TRANS-002: Add Translation
- [ ] TC-TRANS-003: Multiple Languages
- [ ] TC-TRANS-004: Persistence
- [ ] TC-TRANS-005: Frontend Display

#### Phase 9: Advanced Features
- [ ] TC-IMP-001: Valid Import
- [ ] TC-IMP-002: Error Handling
- [ ] TC-SCHED-001: Schedule Publish
- [ ] TC-SCHED-002: Schedule Unpublish
- [ ] TC-AUDIT-001: View Audit
- [ ] TC-AUDIT-002: Filter Audit

#### Phase 10: Error Scenarios
- [ ] TC-ERR-001: Required Fields
- [ ] TC-ERR-002: Slug Validation
- [ ] TC-ERR-003: Duplicate Slug
- [ ] TC-ERR-004: Unauthorized Actions
- [ ] TC-ERR-005: Network Errors
- [ ] TC-ERR-006: Concurrent Editing

### Final Sign-Off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| QA Lead | | | |
| Dev Lead | | | |
| Product Owner | | | |

---

## Appendix A: Test Data Templates

### Task Content Template
```json
{
  "schema_id": "task",
  "slug": "example-task",
  "data": {
    "title": "Example Task Title",
    "description": "Brief description of what this task accomplishes",
    "steps": [
      "First step description",
      "Second step description",
      "Third step description"
    ],
    "difficulty": "medium",
    "estimated_time": "10 minutes",
    "category": "Getting Started",
    "tags": ["beginner", "setup"]
  }
}
```

### Guide Content Template
```json
{
  "schema_id": "guide",
  "slug": "example-guide",
  "data": {
    "title": "Example Guide Title",
    "summary": "<p>Executive summary of what this guide covers</p>",
    "thumbnail": "data:image/png;base64,...",
    "video_url": "https://youtube.com/watch?v=...",
    "difficulty": "medium",
    "duration_minutes": 15,
    "prerequisites": ["Requirement 1", "Requirement 2"],
    "sections": [
      {
        "title": "Introduction",
        "summary": "<p>Welcome section overview</p>",
        "cover_images": ["data:image/png;base64,..."],
        "steps": [
          {
            "order": 1,
            "title": "Getting Started",
            "description": "<p>First step description with <strong>formatting</strong></p>",
            "images": ["data:image/png;base64,..."],
            "videos": []
          },
          {
            "order": 2,
            "title": "Next Step",
            "description": "<p>Continue with these instructions...</p>",
            "images": [],
            "videos": []
          }
        ]
      },
      {
        "title": "Main Content",
        "summary": "<p>The core content section</p>",
        "cover_images": [],
        "steps": [
          {
            "order": 1,
            "title": "Important Step",
            "description": "<p>Detailed instructions here</p>",
            "images": [],
            "videos": []
          }
        ]
      }
    ],
    "related_tasks": ["task-id-1", "task-id-2"]
  }
}
```

> **Note**: Images are stored as base64 data URLs. Maximum file size is 500KB per image to stay within API payload limits.

### FAQ Content Template
```json
{
  "schema_id": "faq",
  "slug": "example-faq",
  "data": {
    "question": "What is the example question?",
    "answer": "<p>This is the detailed answer with <strong>formatting</strong> support.</p>",
    "category": "General"
  }
}
```

### Error Content Template
```json
{
  "schema_id": "error",
  "slug": "e001-example-error",
  "data": {
    "error_code": "E001",
    "error_message": "Example error message",
    "cause": "<p>This error occurs when...</p>",
    "solution": "<p>To fix this error:</p><ol><li>Step 1</li><li>Step 2</li></ol>",
    "platform": ["web", "mobile"]
  }
}
```

---

*End of CMS Complete Testing Guide*
