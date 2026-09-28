# ember-fx Improvement Plan

> Based on comparative analysis with thaw and ui frameworks
> See: `lessons.md`, `framework-comparison.md`, `workspace-architecture.md`

---

## Workspace Migration (Primary Focus)

> Migrate from single-crate to 11-crate workspace architecture
> See: `workspace-architecture.md` for detailed specifications

### Target Structure

```text
ember-fx/
├── crates/
│   ├── core/          # ThemeContext, ThemeProvider, Platform
│   ├── common/        # DesignSystem enum, Size, Direction, traits
│   ├── utils/         # inject_css, ClassList, DOM helpers
│   ├── macros/        # WriteCSSVars, variants!, clx!
│   ├── icons/         # SVG icon system
│   ├── styles/        # CSS compilation, ThemeRegistry
│   ├── systems/       # Ant, Material, Cupertino implementations
│   ├── components/    # Button, Input, Modal, etc.
│   ├── tools/         # ThemeDebugger, PerformanceOverlay
│   └── mobile/        # MobileNavBar, SwipeContainer
├── demos/
│   ├── antd-01/       # Existing demo (moved from examples/)
│   └── showcase/      # New component showcase app
└── docs/
```

### Phase 1: Foundation (Week 1) ✅ COMPLETE

- [x] Create workspace root `Cargo.toml`
- [x] Create `crates/` directory structure
- [x] Extract `ember-fx-common` crate
  - [x] Move `DesignSystem` enum
  - [x] Create `Size`, `Direction`, `ThemeMode` enums
  - [x] Define `ComponentProps` trait
  - [x] Add shared constants
- [x] Extract `ember-fx-utils` crate
  - [x] Move `src/themes/loader.rs` → `utils/src/css_loader.rs`
  - [x] Create `ClassList` utility
  - [x] Create DOM helper functions
  - [x] Create scroll utilities
- [x] Extract `ember-fx-macros` crate
  - [x] Create proc macro crate structure
  - [x] Implement `WriteCSSVars` derive macro
  - [x] Implement `clx!` declarative macro
  - [ ] Implement `variants!` declarative macro (deferred to Phase 3)
- [x] Verify all crates compile independently
- [ ] Update CI to test workspace (pending)

### Phase 2: Core (Week 2) ✅ COMPLETE

- [x] Extract `ember-fx-core` crate
  - [x] Move `src/themes/context.rs` → `core/src/context.rs`
  - [x] Move `src/themes/provider.rs` → `core/src/provider.rs`
  - [x] Move `src/themes/platform.rs` → `core/src/platform.rs`
  - [ ] Add `Model`, `MaybeSignal` reactive types (deferred)
- [x] Extract `ember-fx-styles` crate
  - [x] Move `src/themes/registry.rs` → `styles/src/registry.rs`
  - [x] Move `src/compiled/` → `styles/src/compiled/`
  - [x] Move `themes/base/` → `styles/themes/`
  - [x] Move `build.rs` → `styles/build.rs`
  - [ ] Create CSS variable generation utilities (deferred)
- [x] Verify theme loading works end-to-end
- [x] Test theme switching functionality (4 platform tests pass)

### Phase 3: Components (Week 3) ✅ COMPLETE

- [x] Extract `ember-fx-systems` crate
  - [x] Create `ant/` module with theme, tokens, overrides
  - [x] Create `material/` module (stub)
  - [x] Create `cupertino/` module (stub)
  - [x] Define `DesignSystemImpl` trait
  - [x] Add feature flags per design system
- [x] Extract `ember-fx-components` crate
  - [x] Move all components from `src/components/`
  - [x] Organize by category (button, input, notification, etc.)
  - [x] Add feature flags per component category
  - [x] Update all imports to use workspace crates
- [x] Extract `ember-fx-icons` crate
  - [x] Create `Icon` component
  - [x] Create icon registry system
  - [x] Add heroicons set (lucide, phosphor as feature flags)
  - [x] Add feature flags per icon set
- [x] Move `examples/antd-01/` → `demos/antd-01/`
- [x] Update demo imports and dependencies
- [x] Verify demo compiles

### Phase 4: Extras (Week 4) ✅ COMPLETE

- [x] Create `ember-fx-tools` crate
  - [x] Implement `ThemeDebugger` component
  - [x] Implement `PerformanceOverlay` component
  - [x] Implement `ComponentInspector` component
  - [x] Implement `CSSVariableViewer` component
  - [x] Implement `A11yChecker` component
- [x] Create `ember-fx-mobile` crate
  - [x] Implement `MobileNavBar` component
  - [x] Implement `MobileTabBar` component
  - [x] Implement `SwipeContainer` component
  - [x] Implement `PullToRefresh` component
  - [x] Implement `SafeAreaProvider` component
- [ ] Create `demos/showcase/` crate (deferred - optional)
  - [ ] Set up Leptos app structure
  - [ ] Create component registry
  - [ ] Create theme switcher demo
  - [ ] Create component category pages
- [ ] Update all documentation (deferred)
- [x] Final integration testing
- [ ] Performance benchmarking (deferred)

---

## High Priority (Post-Migration)

### 1. Signal Optimization Patterns ✅ COMPLETE

- [x] Audit existing components for signal usage
- [x] Wrap computed props in `Memo::new()` where applicable
- [x] Replace `get()` with `get_untracked()` in event handlers
- [ ] Add `StoredValue` for immutable configuration (deferred - lower impact)
- [ ] Document signal best practices in component templates (deferred)

**Applied optimizations:**
- `slider.rs`: `percentage` calculation wrapped in `Memo::new()`
- `slider.rs`: Event handler guards use `get_untracked()`
- `date_picker.rs`: `current_year`, `current_month` use `get_untracked()` in handlers
- `time_picker.rs`: `selected_hour/minute/second/period` use `get_untracked()`
- `transfer.rs`: `selected_left`, `selected_right`, `target_keys` use `get_untracked()`
- `collapse.rs`: `active_keys` uses `get_untracked()` in click handler

**Reference** (from thaw):

```rust
// Before
let is_disabled = move || disabled.get() || loading.get();

// After
let is_disabled = Memo::new(move |_| disabled.get() || loading.get());
```

### 2. Stricter Clippy Rules ✅ COMPLETE

- [x] Add workspace-level clippy configuration to root `Cargo.toml`
- [x] Enable `unwrap_used = "deny"`
- [x] Enable `expect_used = "deny"`
- [x] Enable `panic = "deny"`
- [x] Enable `indexing_slicing = "deny"`
- [x] Enable `undocumented_unsafe_blocks = "deny"`
- [x] Fix all resulting clippy violations
- [ ] Add CI check for clippy compliance (deferred)

**Already configured in workspace Cargo.toml**:

```toml
[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"
undocumented_unsafe_blocks = "deny"
await_holding_lock = "deny"
```

**Fixed violations:**
- `validation.rs`: Replaced `parts[0]`/`parts[1]` indexing with iterator pattern
- `validation.rs`: Replaced `&value[8..]`/`&value[7..]` with `strip_prefix()`
- `design_system.rs`, `validation.rs`: Renamed `from_str` to `parse` (avoids `should_implement_trait`)
- `clx.rs`: Boxed large enum variant (reduces enum size)
- `registry.rs`: Removed unneeded `return` keyword
- `css_loader.rs`: Removed `let _ =` on unit value
- `class_list.rs`: Added `#[allow(clippy::should_implement_trait)]` for builder pattern `add` method
- `nav_bar.rs`, `tab_bar.rs`: Removed unnecessary `.clone()` on Copy types

### 3. Reduce Inline Styles in Demos ✅ COMPLETE

- [x] Audit all 63+ inline style usages in `demos/antd-01/src/`
- [x] Create utility classes for common patterns (flex layouts, gaps)
- [x] Add utility classes to `styles.css`
- [x] Replace inline `style="display: flex; gap: Xpx;"` with classes
- [x] Replace inline `style="width: Xpx; height: Xpx;"` with classes
- [x] Keep only truly dynamic styles as inline (reactive `move ||`)

**Results:**
- Reduced inline styles from 67 to 12 (82% reduction)
- Added 25+ utility classes to `styles.css` (u-flex-*, u-gap-*, u-size-*, etc.)
- Remaining 12 are appropriate: 7 unique color values, 5 reactive/nav-specific

---

## Medium Priority

### 4. Input Validation Layer ✅ COMPLETE

- [x] Create validation rule types in `common` crate
- [x] Add `ValidationRule` prop to Input component
- [x] Add reactive validation state to TextInput
- [x] Add validation examples to demos
- [x] Add `FormField` wrapper component
- [ ] Document validation patterns (deferred)

**Components:**
- `FormField` - Static wrapper with label, error, helper text
- `ReactiveFormField` - Reactive wrapper with Signal-based error state

**Implemented Rules:**
- `required` - Non-empty validation
- `min_length` / `max_length` / `length` - Length constraints
- `email` - Email format validation
- `url` - URL format validation
- `numeric` / `integer` - Number validation
- `min` / `max` / `range` - Numeric range validation
- `pattern` - Pattern matching
- `custom` - Custom validation function

**Example API**:

```rust
<TextInput
    value=email
    rules=vec![
        ValidationRule::required("Email is required"),
        ValidationRule::email("Invalid email format"),
    ]
    validate_on=ValidateOn::Blur  // or Change, Submit
/>
```

### 5. E2E Test Coverage ✅ COMPLETE

- [x] Add Playwright dependency to project
- [x] Create `e2e/` directory structure
- [x] Write tests for core components:
  - [x] Button variants and states
  - [x] Input validation
  - [x] Theme switching
  - [x] Modal/Dialog behavior
- [x] Add visual regression tests
- [x] Integrate with CI pipeline

**Implementation:**
- `e2e/package.json` - Playwright dependencies
- `e2e/playwright.config.ts` - Test configuration with multi-browser support
- `e2e/tests/components/button.spec.ts` - Button variants, states, a11y
- `e2e/tests/components/input.spec.ts` - Input validation, states
- `e2e/tests/components/modal.spec.ts` - Modal open/close, focus management
- `e2e/tests/themes/switching.spec.ts` - Theme switching, persistence
- `e2e/fixtures/test-utils.ts` - Common test utilities
- `.github/workflows/e2e-tests.yml` - CI integration

**Run tests:**
```bash
cd e2e
npm install && npm run install:browsers
npm test
```

---

## Low Priority

### 6. Performance Profiling ✅ COMPLETE

- [x] Add build-time metrics collection
- [x] Measure CSS injection timing
- [x] Measure theme switching latency
- [x] Compare bundle sizes across feature combinations
- [x] Document performance characteristics
- [x] Add performance regression tests

**Implementation:**
- `tools/perf/scripts/build-metrics.sh` - Build time measurement
- `tools/perf/scripts/bundle-size.sh` - Bundle size analysis
- `crates/tools/src/performance.rs` - Runtime perf utilities
  - `PerfTimer` - Measure operation durations
  - `measure_css_injection()` - CSS injection timing
  - `measure_theme_switch()` - Theme switching latency
  - `benchmark()` - Run benchmarks with statistics
  - `PerformanceReport` - Live metrics display component
- `e2e/tests/performance/benchmarks.spec.ts` - Regression tests
- `tools/perf/PERFORMANCE.md` - Performance documentation

**Key metrics & thresholds:**
- Page load: < 3000ms
- Theme switch: < 50ms (P95)
- Modal open: < 200ms
- CSS injection: < 20ms

### 7. Accessibility Audit ✅ COMPLETE

- [x] Audit ARIA attributes across all components
- [x] Add keyboard navigation to interactive components
- [x] Add focus management to modals/drawers
- [ ] Test with screen readers (requires manual testing)
- [x] Document accessibility features
- [x] Add a11y tests to E2E suite

**Implementation:**
- `crates/utils/src/a11y.rs` - Comprehensive accessibility utilities
  - `FocusTrap` - Focus containment for modals/drawers
  - `RovingTabindex` - Keyboard navigation for component groups
  - `announce()` - Screen reader announcements
  - `navigate_list()` - Arrow key navigation helper
  - `is_activation_key()`, `is_dismiss_key()` - Key detection
  - `keys` module - Standard key constants
- `crates/components/src/layout/modal.rs` - Updated with:
  - `role="dialog"`, `aria-modal="true"`
  - `aria-labelledby` for title association
  - Focus trap (Tab cycling)
  - Escape key to close
- `crates/components/src/layout/drawer.rs` - Same improvements as Modal
- `docs/ACCESSIBILITY.md` - Full accessibility documentation
- `e2e/tests/accessibility/a11y.spec.ts` - Automated a11y tests

**Component Audit Results:**
- Excellent: Slider (full ARIA)
- Good: Button, Input, Checkbox, Radio, Switch, Collapse, Alert, Pagination, Breadcrumb, Steps, Tag
- Improved: Modal (dialog semantics, focus trap), Drawer (dialog semantics, focus trap)
- Needs future work: Select (combobox pattern), Menu (advanced keyboard), Tooltip (aria-describedby)

---

## Documentation Improvements

> Architecture documentation complete. See `docs/ARCHITECTURE.md`

### 8. Component Documentation

- [ ] Add JSDoc-style comments to all public APIs
- [ ] Create component API reference
- [ ] Add usage examples for each component
- [ ] Document prop types and defaults
- [ ] Add "when to use" guidance

### 9. Architecture Documentation ✅ COMPLETE

- [x] Document CSS loading mechanism (dual path)
- [x] Document theme system architecture
- [x] Create component creation guide
- [x] Document design system extension process
- [x] Add troubleshooting guide

**Implementation:**
- `docs/ARCHITECTURE.md` - Comprehensive architecture documentation covering:
  - Crate dependency graph and workspace structure
  - CSS loading mechanism (build-time compilation + runtime injection)
  - Theme system architecture (ThemeContext, ThemeProvider, ThemeRegistry)
  - CSS variable hierarchy (3-layer system)
  - Component implementation patterns with code examples
  - CSS class naming conventions
  - Design system extension guide (adding themes, design systems, components)
  - Complete troubleshooting guide with solutions for common issues

---

## Completed

- [x] Comprehensive framework analysis (see `framework-comparison.md`)
- [x] ember-fx architecture research (see `research.md`)
- [x] Lessons learned documentation (see `lessons.md`)
- [x] Workspace architecture design (see `workspace-architecture.md`)

---

## Notes

### Dependencies to Add

```toml
# For validation (ember-fx-components)
validator = { version = "0.20", features = ["derive"] }

# For macros (ember-fx-macros)
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
```

### Estimated Effort

| Task | Effort | Impact |
| ---- | ------ | ------ |
| Phase 1: Foundation | 1 week | Critical |
| Phase 2: Core | 1 week | Critical |
| Phase 3: Components | 1 week | Critical |
| Phase 4: Extras | 1 week | High |
| Signal optimization | 2-3 days | High |
| Clippy rules | 1 day | High |
| Inline style cleanup | 2 days | Medium |
| Validation layer | 3-4 days | Medium |
| E2E tests | 1 week | Medium |

### Review Checkpoints

- [x] After Phase 1: Verify foundation crates compile
- [x] After Phase 2: Verify theme system works
- [x] After Phase 3: Verify all demos work
- [x] After Phase 4: Full integration review
- [x] After High Priority: Review performance metrics
- [x] After Medium Priority: Review code quality metrics
- [x] After Low Priority: Accessibility audit complete

---

## Ant Design Modal System (Overlay Module) ✅ COMPLETE

**Date:** 2026-05-14

Implemented comprehensive Ant Design modal system with confirm dialog variants.

### Files Created

**Styles:**
- `crates/styles/themes/base/ant/modal.css` - Comprehensive modal CSS with confirm dialogs

**Components:**
- `crates/components/src/overlay/types.rs` - ConfirmType, ModalSize, ModalAnimation enums
- `crates/components/src/overlay/icons.rs` - SVG icons (Info, Check, Exclamation, Close)
- `crates/components/src/overlay/modal.rs` - Enhanced Modal component
- `crates/components/src/overlay/confirm.rs` - ConfirmModal component
- `crates/components/src/overlay/mod.rs` - Module exports

**Showcase:**
- `demos/showcase/src/pages/overlay/mod.rs` - Module exports
- `demos/showcase/src/pages/overlay/overview.rs` - Overview page
- `demos/showcase/src/pages/overlay/modal.rs` - Comprehensive modal demos

### Files Modified

- `crates/styles/themes/base/ant/_index.css` - Added modal.css import
- `crates/components/src/lib.rs` - Added overlay module with feature gate
- `crates/components/Cargo.toml` - Added overlay feature
- `demos/showcase/Cargo.toml` - Added overlay feature
- `demos/showcase/src/pages/mod.rs` - Added overlay module
- `demos/showcase/src/nav.rs` - Added Overlay nav category
- `demos/showcase/src/routes.rs` - Added overlay routes

### Features Implemented

- **Modal component** with title, footer, size variants, keyboard support (ESC), focus trap
- **ConfirmModal component** with 5 types: Info, Success, Warning, Error, Confirm
- **Size variants**: Small (400px), Default (520px), Large (800px), XL (1000px), Fullscreen
- **Confirm loading state** for async operations
- **Danger confirmation** with red OK button
- **Full accessibility**: ARIA attributes, keyboard navigation, focus management
- **Responsive design** and reduced motion support
