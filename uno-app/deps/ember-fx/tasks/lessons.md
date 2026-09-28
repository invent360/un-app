# Lessons Learned: UI Framework Analysis

> Captured from comprehensive analysis of ember-fx, thaw, and ui frameworks
> Date: May 2026

---

## Key Insights

### 1. Modularity Patterns

**Lesson**: Workspace-based architecture with specialized crates scales better than single-crate with feature flags.

| Approach | Pros | Cons |
|----------|------|------|
| Single crate + features (ember-fx) | Simple dependency, easy versioning | All-or-nothing compilation, harder to test in isolation |
| Workspace crates (thaw, ui) | Clear boundaries, parallel compilation, reusable utilities | More complex dependency management |

**Best Practice**: Separate concerns into distinct crates:
- Core macros (compile-time)
- Runtime utilities (DOM, class management)
- Components (UI layer)
- Demo/docs (development only)

---

### 2. CSS Architecture Strategies

**Lesson**: Three viable approaches exist, each with trade-offs.

| Strategy | Framework | Build Complexity | Runtime Cost | Flexibility |
|----------|-----------|------------------|--------------|-------------|
| Embedded CSS (`include_str!`) | ember-fx, thaw | Medium (build.rs) | Zero network | Limited |
| CSS Variables + Rust structs | thaw | Low | Injection cost | Good |
| External Tailwind CLI | ui | High (Node.js) | None | Maximum |

**Best Practice**:
- Use CSS variables for theming (all three do this well)
- Embed static component CSS at compile time
- Keep theme-switching to attribute changes, not CSS re-injection

---

### 3. Performance Optimization Patterns

**Lesson**: Signal optimization is the biggest performance lever in Leptos apps.

**Patterns from thaw** (highest performance score):
```rust
// 1. Memoize derived values
let is_disabled = Memo::new(move |_| disabled.get() || loading.get());

// 2. Use untracked reads in event handlers
let on_click = move |e| {
    if is_disabled.get_untracked() { return; }  // No dependency
};

// 3. Use StoredValue for immutable config
let id = StoredValue::new(config_id);  // Zero-cost access
```

**Best Practice**:
- Wrap computed values in `Memo::new()`
- Use `get_untracked()` in callbacks
- Use `StoredValue` for config that never changes

---

### 4. Security Enforcement

**Lesson**: Compile-time enforcement is more reliable than runtime checks.

**ui framework's approach** (highest security score):
```toml
[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"
undocumented_unsafe_blocks = "deny"
```

**Best Practice**:
- Enforce safety rules at workspace level
- Use `validator` crate for input validation
- Selective `web-sys` feature imports (minimize API surface)
- Test security attributes in CI (e.g., password field requirements)

---

### 5. Theming Architecture

**Lesson**: Three-layer CSS variable architecture provides best balance.

```
Layer 1: [data-theme=X] { --color-primary: ...; }  ← Theme-specific
Layer 2: :root { --color-primary: fallback; }       ← Defaults
Layer 3: .component { color: var(--color-primary); } ← Consumption
```

**Best Practice**:
- Theme switching via `data-theme` attribute (instant, no re-render)
- Provide fallback values in `:root`
- Components only reference CSS variables, never hard-coded colors

---

### 6. Component Generation Patterns

**Lesson**: Macros can dramatically reduce boilerplate while maintaining type safety.

| Macro Type | Framework | Use Case |
|------------|-----------|----------|
| Proc macro (`#[derive]`) | thaw | Struct → CSS variables |
| Declarative macro (`macro_rules!`) | ui | Component + variants generation |
| None | ember-fx | Manual implementation |

**thaw's WriteCSSVars**:
```rust
#[derive(WriteCSSVars)]
pub struct ColorTheme {
    color_primary: String,  // → --colorPrimary
}
```

**ui's variants!**:
```rust
variants! {
    Button {
        base: "...",
        variants: { size: { Sm: "...", Lg: "..." } }
    }
}
// Generates: ButtonClass, ButtonSize enum, Button fn
```

**Best Practice**: Invest in code generation for:
- Repetitive component patterns
- CSS variable bridging
- Type-safe variant systems

---

### 7. Flexibility vs. Consistency Trade-off

**Lesson**: Design system lock-in is a spectrum.

| Framework | Lock-in Level | Flexibility | Consistency |
|-----------|---------------|-------------|-------------|
| ember-fx | Low (6 systems) | High | Medium |
| thaw | High (Fluent only) | Low | High |
| ui | None (copy-paste) | Maximum | User-dependent |

**Best Practice**:
- For enterprise/brand consistency: Higher lock-in (thaw)
- For agency/multi-client work: Lower lock-in (ember-fx, ui)
- Document design decisions regardless of approach

---

### 8. Documentation as Code

**Lesson**: Demo applications are better than markdown docs for UI frameworks.

| Framework | Documentation Approach |
|-----------|----------------------|
| ember-fx | Example app (antd-01) + inline comments |
| thaw | Dedicated demo crate + demo_markdown |
| ui | Registry showcase + Playwright E2E tests |

**Best Practice**:
- Maintain a living demo app that uses all components
- Test documentation examples in CI
- Use E2E tests to verify component behavior

---

## Recommendations for ember-fx

Based on this analysis, consider these improvements:

### High Priority

1. **Add signal optimization patterns**
   - Wrap computed props in `Memo::new()`
   - Use `get_untracked()` in event handlers

2. **Stricter clippy rules**
   ```toml
   [lints.clippy]
   unwrap_used = "deny"
   expect_used = "deny"
   ```

3. **Workspace refactor** (consider)
   - `ember-fx-core` (themes, context)
   - `ember-fx-components` (UI components)
   - `ember-fx-macro` (if adding code generation)

### Medium Priority

4. **Add input validation layer**
   - Integrate `validator` crate
   - Form-level validation components

5. **E2E test coverage**
   - Playwright tests for component behavior
   - Visual regression testing

### Low Priority

6. **Consider component generation macros**
   - Reduce boilerplate in new components
   - Ensure type-safe variant systems

---

## Anti-Patterns to Avoid

1. **Inline styles for layout** (found 63+ in antd-01)
   - Creates inconsistency
   - Harder to maintain
   - Use utility classes instead

2. **CSS re-injection on theme change**
   - Use attribute-based switching (`[data-theme=X]`)
   - Only inject once per session

3. **Untyped event handlers**
   - Always use typed callbacks
   - Prevents runtime errors

4. **All-in-one crate architecture**
   - Harder to test
   - Slower compilation
   - No reusability of utilities

---

## Summary Table

| Dimension | Winner | Key Lesson |
|-----------|--------|------------|
| Modularity | ui | Workspace > single crate |
| Performance | thaw | Signal optimization matters most |
| Security | ui | Compile-time enforcement |
| Extendability | ui | Macros + copy-paste ownership |
| Flexibility | ui | CSS-first approach |
| Architecture | thaw | Proc macros for bridging |

---

## References

- `tasks/framework-comparison.md` - Full 6-dimension comparison
- `tasks/research.md` - ember-fx deep dive
- `tasks/SUMMARY.md` - Quick reference
