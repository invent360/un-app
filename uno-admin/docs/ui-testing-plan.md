# CMS UI Testing Plan - Manual Testing Guide

**Version**: 1.0
**Date**: May 30, 2026
**Testers**: UI/QA Team
**Applications**: uno-admin (Admin Dashboard), uno-app (Public Frontend)

---

## Table of Contents

1. [Test Environment Setup](#1-test-environment-setup)
2. [Test Data Requirements](#2-test-data-requirements)
3. [Feature Test Cases](#3-feature-test-cases)
   - [3.1 Content Management](#31-content-management)
   - [3.2 Version History & Revert](#32-version-history--revert)
   - [3.3 Version Diff Viewer](#33-version-diff-viewer)
   - [3.4 Preview System](#34-preview-system)
   - [3.5 Review Workflow](#35-review-workflow)
   - [3.6 Publish Queue](#36-publish-queue)
   - [3.7 Bulk Import](#37-bulk-import)
   - [3.8 Translation Editor](#38-translation-editor)
   - [3.9 Role-Based Access Control](#39-role-based-access-control)
   - [3.10 Content Scheduling](#310-content-scheduling)
   - [3.11 Audit Log](#311-audit-log)
4. [Cross-Browser Testing](#4-cross-browser-testing)
5. [Responsive Design Testing](#5-responsive-design-testing)
6. [Bug Report Template](#6-bug-report-template)
7. [Test Sign-Off Checklist](#7-test-sign-off-checklist)

---

## 1. Test Environment Setup

### URLs
| Environment | uno-admin URL | uno-app URL |
|-------------|---------------|-------------|
| Development | http://localhost:3001 | http://localhost:3000 |
| Staging | https://admin.staging.uno.network | https://staging.uno.network |
| Production | https://admin.uno.network | https://uno.network |

### Test Accounts
| Role | Username | Password | Permissions |
|------|----------|----------|-------------|
| Admin | admin@test.com | [provided] | Full access |
| Publisher | publisher@test.com | [provided] | Publish, Revert, Schedule |
| Reviewer | reviewer@test.com | [provided] | Review, Approve |
| Author | author@test.com | [provided] | Create, Edit only |

### Browser Requirements
- Chrome 120+ (Primary)
- Firefox 120+
- Safari 17+
- Edge 120+

---

## 2. Test Data Requirements

### Pre-requisites
Before testing, ensure the following test data exists:

| Data Type | Quantity | Details |
|-----------|----------|---------|
| Task content | 5+ items | Various statuses (draft, published, pending_review) |
| Guide content | 3+ items | With multiple versions |
| FAQ content | 3+ items | With translations |
| Error content | 2+ items | Basic entries |
| User accounts | 4 | One per role |

### Test Files for Import
Prepare these files in advance:

**valid-import.json**
```json
[
  {
    "content_type": "task",
    "slug": "test-import-1",
    "content": { "title": "Test Import 1", "description": "Description", "steps": ["Step 1"] }
  },
  {
    "content_type": "faq",
    "slug": "test-import-2",
    "content": { "title": "FAQ Import", "question": "Question?", "answer": "Answer." }
  }
]
```

**invalid-import.json** (for error testing)
```json
[
  { "content_type": "task", "slug": "", "content": {} },
  { "content_type": "invalid", "slug": "test", "content": { "title": "Test" } }
]
```

---

## 3. Feature Test Cases

### 3.1 Content Management

#### TC-CM-001: View Content List
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to `/content` | Content list page loads |
| 2 | Verify table headers | Shows: Title, Type, Status, Version, Updated, Actions |
| 3 | Verify pagination | Shows "Page X of Y", Previous/Next buttons work |
| 4 | Verify content count | Shows total count (e.g., "Showing 1-20 of 45") |

#### TC-CM-002: Filter Content by Type
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click Type dropdown | Shows: All, Task, Guide, FAQ, Error |
| 2 | Select "Task" | Only task content items displayed |
| 3 | Select "Guide" | Only guide content items displayed |
| 4 | Select "All" | All content types displayed |

#### TC-CM-003: Filter Content by Status
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click Status dropdown | Shows: All, Draft, Pending Review, Approved, Published, Archived |
| 2 | Select "Draft" | Only draft items shown, status badge is gray |
| 3 | Select "Published" | Only published items shown, status badge is green |
| 4 | Select "Pending Review" | Only pending items shown, status badge is yellow/orange |

#### TC-CM-004: Search Content
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Type "telemetry" in search | Results filter in real-time |
| 2 | Clear search | All content displayed again |
| 3 | Search with no results | Shows "No content found" message |

#### TC-CM-005: Create New Content
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click "New Content" button | Editor page opens with empty form |
| 2 | Select content type "Task" | Form fields update (Title, Description, Steps, Body) |
| 3 | Fill in required fields | No validation errors |
| 4 | Click "Save Draft" | Success message, redirects to editor with ID |
| 5 | Verify in content list | New item appears with status "Draft", version "v1" |

#### TC-CM-006: Edit Existing Content
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click Edit on existing content | Editor loads with populated fields |
| 2 | Modify title | Field updates |
| 3 | Add change summary | Text field accepts input |
| 4 | Click "Save Draft" | Success message, version increments |

#### TC-CM-007: Delete Content (Admin Only)
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Admin | Admin dashboard loads |
| 2 | Click delete on content item | Confirmation modal appears |
| 3 | Confirm deletion | Content removed from list |
| 4 | Login as Author | No delete button visible |

---

### 3.2 Version History & Revert

#### TC-VH-001: View Version History
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open content editor | Editor loads |
| 2 | Expand "Version History" section | Collapsible panel expands |
| 3 | Verify version list | Shows all versions (v1, v2, v3...) with dates |
| 4 | Current version highlighted | Active version has "current" badge |
| 5 | Each version shows | Change summary, date, author |

#### TC-VH-002: Revert to Previous Version
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Publisher/Admin | Has revert permission |
| 2 | Open content with 3+ versions | Version history shows v1, v2, v3 |
| 3 | Click "Revert" on v1 | Diff preview modal opens |
| 4 | Review changes shown | Side-by-side comparison displayed |
| 5 | Click "Confirm Revert" | Success message |
| 6 | Verify new version created | v4 created with v1's content |
| 7 | Content matches v1 | Fields contain v1 data |

#### TC-VH-003: Revert Button Visibility by Role
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Author | Open content editor |
| 2 | Check version history | NO "Revert" button visible |
| 3 | Login as Publisher | Open same content |
| 4 | Check version history | "Revert" buttons ARE visible |
| 5 | Login as Admin | "Revert" buttons ARE visible |

#### TC-VH-004: Revert Confirmation Modal
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click "Revert to v1" | Modal opens with warning |
| 2 | Modal shows | "This will create a new version with content from v1" |
| 3 | Click "Cancel" | Modal closes, no changes made |
| 4 | Click "Revert" again | Modal opens |
| 5 | Click "Confirm" | Revert executes, modal closes |

---

### 3.3 Version Diff Viewer

#### TC-DF-001: View Diff Between Versions
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open content with multiple versions | Version history visible |
| 2 | Click "View Diff" on a version | Diff modal opens |
| 3 | Modal header | Shows "Comparing vX vs vY" |
| 4 | Changes displayed | Field-by-field comparison shown |

#### TC-DF-002: Diff Color Coding
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | View diff with added content | Added fields have GREEN background |
| 2 | View diff with removed content | Removed fields have RED background |
| 3 | View diff with modified content | Modified fields have BLUE background |

#### TC-DF-003: Diff Shows All Locales
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Edit content with Spanish translation | Save as new version |
| 2 | View diff | Shows English changes |
| 3 | Scroll/expand | Shows Spanish (es) changes separately |
| 4 | Each locale labeled | Clear "English", "Spanish", etc. labels |

#### TC-DF-004: Close Diff Modal
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open diff modal | Modal is visible |
| 2 | Click X button | Modal closes |
| 3 | Press Escape key | Modal closes |
| 4 | Click outside modal | Modal closes (if implemented) |

---

### 3.4 Preview System

#### TC-PV-001: Generate Preview Token
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open content editor | Editor loads |
| 2 | Click "Preview" button | Preview opens in new tab |
| 3 | URL contains token | URL like `/tasks?preview_token=abc123...` |
| 4 | Preview banner visible | Amber/yellow banner at top |
| 5 | Banner text | "PREVIEW MODE - This content is not published" |

#### TC-PV-002: Preview Shows Draft Content
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Create new draft content | Title: "Preview Test Draft" |
| 2 | DO NOT publish | Status remains "draft" |
| 3 | Click Preview | Opens uno-app with preview token |
| 4 | Content visible | "Preview Test Draft" title shown |
| 5 | Visit same page without token | Content NOT visible (404 or hidden) |

#### TC-PV-003: Preview Banner Exit Button
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open preview URL | Preview banner visible |
| 2 | Click "Exit Preview" button | Redirects to regular page |
| 3 | Banner disappears | Normal page view |

#### TC-PV-004: Preview Token Expiration
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Generate preview token | Note the token |
| 2 | Wait 24+ hours (or use expired test token) | Token should expire |
| 3 | Access preview URL | Error: "Invalid or expired preview link" |

#### TC-PV-005: Preview Works on Tasks Page
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Create draft Task content | Save |
| 2 | Generate preview | Opens `/tasks?preview_token=xxx` |
| 3 | Task card visible | Draft task appears in task list |
| 4 | Card styling | Matches production styling |

#### TC-PV-006: Preview Works on Guides Page
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Create draft Guide content | Save |
| 2 | Generate preview | Opens `/guides?preview_token=xxx` |
| 3 | Guide visible | Draft guide appears |
| 4 | Accordion/sections work | Interactive elements function |

---

### 3.5 Review Workflow

#### TC-RW-001: Submit for Review
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open draft content | Status shows "draft" |
| 2 | Click "Submit for Review" | Confirmation appears |
| 3 | Confirm submission | Status changes to "pending_review" |
| 4 | Button disabled | Cannot submit again while pending |

#### TC-RW-002: View Review Dashboard
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to `/reviews` | Reviews dashboard loads |
| 2 | Pending reviews listed | Cards show title, type, submitter, date |
| 3 | Action buttons visible | Preview, Compare, Approve, Request Changes |

#### TC-RW-003: Approve Review
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Reviewer | Has review permission |
| 2 | Open Reviews dashboard | Pending items visible |
| 3 | Click "Approve" on item | Confirmation modal (optional) |
| 4 | Confirm approval | Item removed from pending list |
| 5 | Check content status | Status = "approved" |

#### TC-RW-004: Request Changes
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click "Request Changes" on pending review | Modal opens with notes field |
| 2 | Notes field required | Cannot submit empty |
| 3 | Enter feedback | "Please fix typo in step 3" |
| 4 | Submit | Review returned to author |
| 5 | Content status | Back to "draft" |

#### TC-RW-005: Self-Review Prevention
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Author | Create and submit content |
| 2 | Go to Reviews dashboard | Content visible (if same user can see) |
| 3 | Approve buttons | Should be disabled or hidden for own content |
| 4 | Tooltip/message | "Cannot approve your own submission" |

---

### 3.6 Publish Queue

#### TC-PQ-001: View Publish Queue
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to `/publish` | Publish queue page loads |
| 2 | Only approved content shown | No draft or pending items |
| 3 | Each item shows | Title, type, approved by, approved date |

#### TC-PQ-002: Select Items for Publish
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Check individual checkboxes | Items selected |
| 2 | Counter updates | "Selected: 3 items" |
| 3 | Click "Select All" | All items checked |
| 4 | Uncheck one | "Select All" becomes unchecked |

#### TC-PQ-003: Batch Publish
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Select 3 approved items | Counter shows "3 selected" |
| 2 | Click "Publish Selected" | Confirmation modal with warning |
| 3 | Warning text | "This will make selected content live" |
| 4 | Confirm | Progress indicator |
| 5 | Success | "3 items published" message |
| 6 | Items removed from queue | Queue refreshes |
| 7 | Content on public site | Visible on uno-app |

#### TC-PQ-004: Publish Permission Check
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Author | Navigate to `/publish` |
| 2 | Access denied OR | Queue visible but Publish button disabled |
| 3 | Login as Publisher | Publish button enabled |

---

### 3.7 Bulk Import

#### TC-BI-001: Access Import Page
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to `/content` | Content list page |
| 2 | Click "Import" button | Navigates to `/content/import` |
| 3 | Import page loads | File upload area visible |

#### TC-BI-002: Upload Valid JSON File
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click upload area | File picker opens |
| 2 | Select valid-import.json | File accepted |
| 3 | Preview table appears | Shows parsed content |
| 4 | Each row shows | Status (checkmark), Type, Slug, Title |
| 5 | Valid rows | Green checkmark |

#### TC-BI-003: Upload Invalid JSON File
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Select invalid-import.json | File processed |
| 2 | Preview table shows | Errors highlighted |
| 3 | Invalid rows | Red X with error message |
| 4 | Error messages | "Missing required field: slug", "Invalid content_type" |

#### TC-BI-004: Execute Import
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Upload valid file | Preview shows 2 valid items |
| 2 | Click "Import" button | Progress indicator |
| 3 | Import completes | Summary: "Imported: 2, Skipped: 0, Errors: 0" |
| 4 | Navigate to content list | New items visible with status "draft" |

#### TC-BI-005: Import Duplicate Handling
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Import file with existing slug | Process file |
| 2 | Preview shows warning | "Slug already exists" |
| 3 | Execute import | Duplicate skipped or updated (based on design) |
| 4 | Summary shows | Skipped count or update notice |

#### TC-BI-006: Import Permission Check
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Author | Go to content list |
| 2 | Import button | NOT visible or disabled |
| 3 | Login as Publisher | Import button visible and enabled |

---

### 3.8 Translation Editor

#### TC-TE-001: View Translation Editor
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open content editor | Editor loads |
| 2 | Scroll to Translations section | Translation editor visible |
| 3 | Locale tabs shown | es, fr, ar, hi, pt, id, tl, sw |

#### TC-TE-002: Switch Between Locales
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click "es" (Spanish) tab | Spanish fields displayed |
| 2 | Tab highlighted | Active tab has different style |
| 3 | Click "fr" (French) tab | French fields displayed |
| 4 | Previous input preserved | Spanish data still saved |

#### TC-TE-003: Edit Translation Fields
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Select Spanish tab | Spanish fields shown |
| 2 | Enter Spanish title | "Título en Español" |
| 3 | Enter Spanish description | Text accepted |
| 4 | English reference shown | Original English text visible for reference |

#### TC-TE-004: Save Translations
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Add Spanish and French translations | Fill in fields |
| 2 | Click "Save Draft" | Success message |
| 3 | Reload page | Translations preserved |
| 4 | Switch to Spanish tab | Previously entered text displayed |

#### TC-TE-005: Translation Coverage Display
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | View content with no translations | Coverage shows "0/8 locales" |
| 2 | Add Spanish translation | Coverage updates "1/8 locales" |
| 3 | Add French translation | Coverage updates "2/8 locales" |

#### TC-TE-006: RTL Language Support (Arabic)
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Select Arabic (ar) tab | Arabic fields shown |
| 2 | Input fields | Right-to-left text direction |
| 3 | Enter Arabic text | Text displays correctly |

---

### 3.9 Role-Based Access Control

#### TC-RBAC-001: Author Role Permissions
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Author | Dashboard loads |
| 2 | Navigate to content list | Content visible |
| 3 | Create new content | Allowed |
| 4 | Edit own content | Allowed |
| 5 | Submit for review | Allowed |
| 6 | Publish button | NOT visible |
| 7 | Revert button | NOT visible |
| 8 | Import button | NOT visible |
| 9 | Delete button | NOT visible |

#### TC-RBAC-002: Reviewer Role Permissions
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Reviewer | Dashboard loads |
| 2 | Access Reviews dashboard | Allowed |
| 3 | Approve reviews | Allowed |
| 4 | Request changes | Allowed |
| 5 | Publish button | NOT visible |
| 6 | Delete button | NOT visible |

#### TC-RBAC-003: Publisher Role Permissions
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Publisher | Dashboard loads |
| 2 | Publish content | Allowed |
| 3 | Revert versions | Allowed |
| 4 | Import content | Allowed |
| 5 | Schedule content | Allowed |
| 6 | Delete content | NOT visible |
| 7 | Manage users | NOT visible |

#### TC-RBAC-004: Admin Role Permissions
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Login as Admin | Dashboard loads |
| 2 | All content actions | Allowed |
| 3 | Delete content | Allowed |
| 4 | Access Settings | Allowed |
| 5 | Manage user roles | Allowed (if implemented) |

#### TC-RBAC-005: Role Switcher (Dev/Testing)
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to Settings | Settings page loads |
| 2 | Find "User Role (Development)" section | Role selector visible |
| 3 | Click "Author" button | Role changes |
| 4 | UI updates | Publish/Revert buttons disappear |
| 5 | Click "Admin" button | Role changes |
| 6 | UI updates | All buttons reappear |
| 7 | Refresh page | Role persists (localStorage) |

---

### 3.10 Content Scheduling

#### TC-SC-001: View Scheduling Section
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Open content editor | Editor loads |
| 2 | Find Scheduling section | Section visible |
| 3 | Toggle "Enable scheduling" | Datetime fields appear |

#### TC-SC-002: Set Publish At Time
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Enable scheduling | Fields visible |
| 2 | Click "Publish At" datetime picker | Date/time picker opens |
| 3 | Select future date/time | Value set |
| 4 | Schedule summary shows | "Publish at [date/time]" |

#### TC-SC-003: Set Unpublish At Time
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Enable scheduling | Fields visible |
| 2 | Set Publish At | Value set |
| 3 | Set Unpublish At (after Publish At) | Value set |
| 4 | Schedule summary shows | Both times displayed |

#### TC-SC-004: Validation - Unpublish Before Publish
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Set Publish At: Jan 15, 2026 10:00 | Value set |
| 2 | Set Unpublish At: Jan 10, 2026 10:00 | Should show error |
| 3 | Error message | "Unpublish time must be after publish time" |

#### TC-SC-005: Disable Scheduling
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Enable scheduling, set times | Values set |
| 2 | Uncheck "Enable scheduling" | Fields hide |
| 3 | Save content | No schedule saved |
| 4 | Reload | Scheduling disabled, no times |

---

### 3.11 Audit Log

#### TC-AL-001: Access Audit Log Page
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Navigate to `/audit` | Audit log page loads |
| 2 | Table displays | Timestamp, Action, Content, Actor, Details |

#### TC-AL-002: Filter by Action Type
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Click Action dropdown | Options: All, Create, Update, Publish, Revert, Delete, Review |
| 2 | Select "Publish" | Only publish actions shown |
| 3 | Select "Create" | Only create actions shown |

#### TC-AL-003: Filter by Actor
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Type username in Actor field | e.g., "admin" |
| 2 | Results filter | Only actions by that user |
| 3 | Clear field | All results shown |

#### TC-AL-004: Filter by Date Range
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Set "From Date" | Select date |
| 2 | Results filter | Only entries from that date onwards |
| 3 | Set "To Date" | Select date |
| 4 | Results filter | Only entries within range |

#### TC-AL-005: Filter by Content ID
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Enter content ID number | e.g., "42" |
| 2 | Results filter | Only actions for that content |
| 3 | Link to content | Clicking content slug navigates to editor |

#### TC-AL-006: Clear All Filters
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | Set multiple filters | Action, Actor, Date |
| 2 | Click "Clear Filters" | All filters reset |
| 3 | Full results displayed | All audit entries shown |

#### TC-AL-007: Pagination
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | View audit log with many entries | Pagination visible |
| 2 | Click "Next" | Page 2 loads |
| 3 | Click "Previous" | Page 1 loads |
| 4 | Page indicator | Shows "Page X" |

#### TC-AL-008: Action Badges Color Coding
| Step | Action | Expected Result |
|------|--------|-----------------|
| 1 | View Create action | GREEN badge |
| 2 | View Update action | BLUE badge |
| 3 | View Publish action | GREEN badge |
| 4 | View Revert action | YELLOW/AMBER badge |
| 5 | View Delete action | RED badge |

---

## 4. Cross-Browser Testing

Test the following critical flows in each browser:

### Browsers to Test
- [ ] Chrome 120+ (Windows)
- [ ] Chrome 120+ (macOS)
- [ ] Firefox 120+ (Windows)
- [ ] Firefox 120+ (macOS)
- [ ] Safari 17+ (macOS)
- [ ] Edge 120+ (Windows)

### Critical Flows per Browser
| Flow | Chrome | Firefox | Safari | Edge |
|------|--------|---------|--------|------|
| Login & Navigation | | | | |
| Create Content | | | | |
| Edit & Save Content | | | | |
| Version History | | | | |
| Preview System | | | | |
| Bulk Import | | | | |
| Translation Editor | | | | |

### Known Browser-Specific Issues to Check
- [ ] File upload drag-and-drop works
- [ ] Datetime picker renders correctly
- [ ] RTL text in Arabic fields
- [ ] Modal animations smooth
- [ ] Keyboard navigation works

---

## 5. Responsive Design Testing

### Breakpoints to Test
| Breakpoint | Width | Devices |
|------------|-------|---------|
| Mobile | 320px - 480px | iPhone SE, small Android |
| Tablet Portrait | 481px - 768px | iPad Mini portrait |
| Tablet Landscape | 769px - 1024px | iPad landscape |
| Desktop | 1025px - 1440px | Laptop |
| Large Desktop | 1441px+ | External monitor |

### Responsive Test Cases
| Component | Mobile | Tablet | Desktop |
|-----------|--------|--------|---------|
| Navigation sidebar | Collapses to hamburger | Collapses or mini | Full sidebar |
| Content list table | Horizontal scroll OK | Fits | Fits |
| Content editor | Stacked layout | Two columns | Two columns |
| Version history | Collapsible | Side panel | Side panel |
| Modals | Full screen OK | Centered | Centered |
| Action buttons | Stacked OK | Row | Row |

---

## 6. Bug Report Template

When reporting bugs, use this template:

```markdown
## Bug Report

**Bug ID**: BUG-XXX
**Reporter**: [Your Name]
**Date**: [Date]
**Severity**: Critical / High / Medium / Low
**Environment**: Development / Staging / Production

### Summary
[One-line description of the bug]

### Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

### Expected Result
[What should happen]

### Actual Result
[What actually happened]

### Screenshots/Video
[Attach screenshots or video recording]

### Browser/Device
- Browser: Chrome 120
- OS: macOS 14.2
- Screen Size: 1440x900

### Console Errors
```
[Paste any JavaScript console errors]
```

### Additional Notes
[Any other relevant information]
```

### Severity Definitions
| Severity | Definition | Example |
|----------|------------|---------|
| Critical | App crashes, data loss, security issue | Cannot save content, data deleted |
| High | Major feature broken, no workaround | Publish button doesn't work |
| Medium | Feature broken but workaround exists | Filter doesn't work, can use search |
| Low | Minor UI issue, cosmetic | Wrong icon, alignment off |

---

## 7. Test Sign-Off Checklist

### Pre-Release Sign-Off

#### Content Management
- [ ] TC-CM-001 to TC-CM-007 all pass
- [ ] No critical/high bugs open

#### Version History & Revert
- [ ] TC-VH-001 to TC-VH-004 all pass
- [ ] Revert creates new version correctly
- [ ] Role permissions enforced

#### Version Diff Viewer
- [ ] TC-DF-001 to TC-DF-004 all pass
- [ ] Color coding correct
- [ ] All locales shown

#### Preview System
- [ ] TC-PV-001 to TC-PV-006 all pass
- [ ] Preview banner visible
- [ ] Token expiration works

#### Review Workflow
- [ ] TC-RW-001 to TC-RW-005 all pass
- [ ] Status transitions correct
- [ ] Self-review prevented

#### Publish Queue
- [ ] TC-PQ-001 to TC-PQ-004 all pass
- [ ] Batch publish works
- [ ] Content goes live

#### Bulk Import
- [ ] TC-BI-001 to TC-BI-006 all pass
- [ ] Valid files import
- [ ] Invalid files show errors

#### Translation Editor
- [ ] TC-TE-001 to TC-TE-006 all pass
- [ ] All 8 locales work
- [ ] RTL support for Arabic

#### RBAC
- [ ] TC-RBAC-001 to TC-RBAC-005 all pass
- [ ] Each role has correct permissions
- [ ] No unauthorized access

#### Content Scheduling
- [ ] TC-SC-001 to TC-SC-005 all pass
- [ ] Datetime pickers work
- [ ] Validation prevents invalid schedules

#### Audit Log
- [ ] TC-AL-001 to TC-AL-008 all pass
- [ ] Filtering works
- [ ] Actions logged correctly

#### Cross-Browser
- [ ] All critical flows pass in Chrome
- [ ] All critical flows pass in Firefox
- [ ] All critical flows pass in Safari
- [ ] All critical flows pass in Edge

#### Responsive Design
- [ ] Mobile layout acceptable
- [ ] Tablet layout acceptable
- [ ] Desktop layout acceptable

### Sign-Off Approval

| Role | Name | Date | Signature |
|------|------|------|-----------|
| QA Lead | | | |
| Dev Lead | | | |
| Product Owner | | | |

---

## Appendix: Quick Reference

### Status Flow
```
draft → pending_review → approved → published
         ↓                  ↓
    changes_requested ←──────┘
```

### Keyboard Shortcuts (if implemented)
| Shortcut | Action |
|----------|--------|
| Ctrl+S | Save Draft |
| Ctrl+Enter | Submit for Review |
| Escape | Close Modal |

### Test Account Quick Login
```
Admin:     admin@test.com / [password]
Publisher: publisher@test.com / [password]
Reviewer:  reviewer@test.com / [password]
Author:    author@test.com / [password]
```
