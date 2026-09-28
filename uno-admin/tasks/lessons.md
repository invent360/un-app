# Lessons Learned

## 2026-06-05: License Filter Bug

### Issue
Filter dropdown was updating an unused signal (`filter_online`) instead of the actual filter signal (`status_filter`), causing:
1. Filter params not sent to backend
2. Results not shown in UI despite API returning data
3. UI filter state disconnected from actual filtering logic

### Root Causes
1. **Legacy signal not removed**: `filter_online` was a legacy signal that was never used but still being updated by the UI
2. **Signal naming confusion**: Two similar signals (`filter_online` vs `status_filter`) existed for the same purpose
3. **Missing reactive binding**: Dropdown `selected` attribute wasn't synced to filter state

### Fix Applied
1. Removed unused `filter_online` signal
2. Connected status dropdown to `status_filter` directly
3. Added `selected` attribute with reactive binding to sync UI state
4. Added filter signals to `paginated_resource` dependencies

### Pattern to Avoid
- **Don't leave dead code**: Remove unused signals/variables immediately
- **Name signals clearly**: Use consistent naming that matches their purpose
- **Verify signal connections**: When UI changes a signal, trace to verify the signal is actually used downstream
- **Test data flow**: After UI interaction, verify the correct API call is made with correct params

## 2026-06-05: Dual API Requests / Empty Table Bug

### Issue
License list page was making 2 API requests and showing "No licenses found" despite the API returning 26 results:
1. `get_paginated_licenses` - returned data correctly
2. `get_all_licenses_for_filtering` - called redundantly

### Root Causes
1. **Two Resource objects created**: Both `paginated_resource` and `all_licenses_resource` were unconditionally created, causing both to fire on mount
2. **Wrong data source used**: When any filter was active (`has_active_filters`), the UI used `all_licenses_resource` which fetched ALL licenses without filter params, then applied client-side filtering
3. **Redundant architecture**: `paginated_resource` already supported all filters via server-side filtering - the `all_licenses_resource` path was completely unnecessary

### Fix Applied
1. Removed `all_licenses_resource` and `has_active_filters` signal entirely
2. Simplified rendering to always use `paginated_resource`
3. Removed unused import `get_all_licenses_for_filtering`

### Pattern to Avoid
- **Don't create multiple Resource objects that do the same thing**: If server-side filtering is supported, use it exclusively
- **Avoid client-side filtering of server data**: It's inefficient and error-prone
- **One source of truth**: Choose either server-side or client-side filtering, not both
- **Verify Resource dependencies**: Ensure all filter state signals are in the Resource's dependency tuple so changes trigger refetch

## 2026-06-05: API Filter Parameter Serialization Bug

### Issue
Filter parameters were always being sent to the API even when "All" was selected, causing incorrect filtering:
- Selecting "All" for leased filter sent `"isLeased": false` instead of omitting the field
- API interprets `false` as "filter for NOT leased" rather than "no filter"

### Root Cause
`PaginatedLicenseRequest` used `bool` fields instead of `Option<bool>`:
```rust
// WRONG - always serializes, even when no filter wanted
pub is_leased: bool,  // false gets sent as "isLeased": false
```

### API Semantics (Correct Understanding)
| Parameter Value | API Behavior |
|-----------------|--------------|
| `true` | Filter for items WHERE field is true |
| `false` | Filter for items WHERE field is false |
| **field omitted** | No filter (show all) |

### Fix Applied
1. Changed `bool` to `Option<bool>` with `#[serde(skip_serializing_if = "Option::is_none")]`
2. Updated `new()` to default to `None` instead of `false`
3. Simplified `with_filters()` to pass `Option<bool>` values directly

### Pattern to Avoid
- **Understand API semantics first**: Don't assume `false` means "no filter" - test the actual behavior
- **Use Option<T> for optional parameters**: With `skip_serializing_if` to omit when None
- **Match serialization to API expectations**: If API expects field to be absent for "no filter", use Option
