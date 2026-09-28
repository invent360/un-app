# ember-fx + antd-01 Analysis Summary

> Quick reference for the comprehensive analysis in `research.md`

## Key Findings

### 1. Component & Style Application

| Mechanism | Description |
|-----------|-------------|
| **Entry Point** | `ThemeProvider` wraps app in `main.rs` with `DesignSystem::Ant` |
| **Dependencies** | Feature-gated via Cargo.toml (`features = ["csr", "layouts", ...]`) |
| **Component Import** | `use ember_fx::{Alert, AlertType};` |
| **Class Generation** | Components generate `fx-{component}-{design_system}-{variant}` classes |

### 2. Inline Styling

**YES - 63+ instances found**

| Category | Count | Example |
|----------|-------|---------|
| Flex layouts | ~40 | `style="display: flex; gap: 16px;"` |
| Grid layouts | ~5 | `style="display: grid; ..."` |
| Fixed dimensions | ~10 | `style="width: 120px; height: 120px;"` |
| Theme colors | 7 | `style="background: #1890ff;"` |
| Reactive styles | ~3 | `style=move \|\| { ... }` |

### 3. Theme Application

**3-Layer CSS Variable Architecture:**

```
Layer 1: [data-theme=dark] { --fx-color-primary: #1668dc; }  ← Theme-specific
Layer 2: :root { --fx-color-primary: oklch(58%...); }        ← Fallbacks
Layer 3: .fx-btn-ant { background: var(--fx-color-primary); } ← Components
```

**Switching:** `theme_ctx.set_theme("light")` → updates `<html data-theme="light">`

### 4. Styling Primitives

| Primitive | Purpose |
|-----------|---------|
| `ThemeProvider` | Context provider + CSS injection |
| `ThemeContext` | Reactive theme state (`use_theme()`) |
| `DesignSystem` | Enum: Ant, Material, Cupertino, etc. |
| `ThemeRegistry` | Compiled theme storage |
| `inject_css()` | DOM style injection |
| `apply_theme_attribute()` | Sets `data-theme` on `<html>` |

### 5. Configuration Sources

**Note:** No `tailwind.config.js` exists - ember-fx uses pure CSS variables.

| Source | Location | Purpose |
|--------|----------|---------|
| Theme source | `themes/base/ant/*.css` | Component CSS definitions |
| Compiled themes | `src/compiled/ant/*.css` | Minified, embedded CSS |
| App overrides | `examples/antd-01/styles.css` | Layout-specific styles |
| Design tokens | `examples/antd-01/antd-pro.css` | App-specific tokens |

### CSS Loading Flow

```
┌─────────────────────────────────────────────────────────────┐
│  BUILD TIME                                                  │
│  themes/base/ant/*.css → build.rs → src/compiled/ant/*.css  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  COMPILE TIME                                                │
│  include_str!() embeds CSS into WASM binary                 │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  RUNTIME (dual path)                                         │
│                                                              │
│  Path 1: Trunk                    Path 2: ThemeProvider     │
│  ├─ antd-pro.css                  ├─ ember-theme-base       │
│  └─ styles.css                    ├─ ember-theme-css        │
│  (static <link> tags)             └─ ember-theme-components │
│                                   (dynamic <style> injection)│
└─────────────────────────────────────────────────────────────┘
```

## Files

| File | Description |
|------|-------------|
| `tasks/research.md` | Full detailed analysis (497 lines) |
| `tasks/SUMMARY.md` | This quick reference |

## Open Questions

- [ ] Should inline styles be refactored to CSS classes?
- [ ] Is dual CSS loading causing FOUC?
- [ ] Should antd-01 use `MinimalThemeProvider`?
