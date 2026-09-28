# ember-fx Styling and Theming Architecture

## Overview

This document captures research findings on how ember-fx components and styles are applied to the antd-01 example application.

---

## 1. How ember-fx Components and Styles Are Applied to antd-01

### Entry Point Architecture

The application bootstraps in `examples/antd-01/src/main.rs:10-22`:

```rust
use ember_fx::{ThemeProvider, DesignSystem};

fn main() {
    mount_to_body(|| {
        view! {
            <ThemeProvider
                initial_theme="dark"
                design_system=DesignSystem::Ant
            >
                <App/>
            </ThemeProvider>
        }
    })
}
```

### Component Integration Pattern

**Cargo.toml dependency** (`examples/antd-01/Cargo.toml:18`):
```toml
ember-fx = { path = "../..", features = ["csr", "layouts", "panel", "input", "selection", "notification", "layout", "navigation", "data", "visualization", "form-advanced"] }
```

**Component Usage** (e.g., `examples/antd-01/src/pages/notification/alert.rs:4-5,20-24`):
```rust
use ember_fx::{Alert, AlertType};

<Alert
    alert_type=AlertType::Success
    message="Success! Your changes have been saved."
/>
```

---

## 2. Dual CSS Loading Mechanism

The antd-01 application uses **two completely separate CSS loading pathways** that serve different purposes:

### Path 1: Static CSS via Trunk (Build-Time)

**Source files:**
```
examples/antd-01/
├── index.html          ← Entry point with data-trunk directives
├── antd-pro.css        ← 457 lines of layout/design token CSS
└── styles.css          ← 1361 lines of app-specific layout CSS
```

**index.html directives:**
```html
<link data-trunk rel="css" href="antd-pro.css" />
<link data-trunk rel="css" href="styles.css" />
```

**What Trunk does at build time:**

1. **Parses** the `data-trunk rel="css"` attributes
2. **Copies** the CSS files to the `dist/` folder
3. **Adds content hash** to filenames for cache busting
4. **Replaces** the directives with standard `<link>` tags

**Resulting dist/index.html:7-8:**
```html
<link rel="stylesheet" href="/antd-pro-5c2b08c4735369fd.css" integrity="sha384-..."/>
<link rel="stylesheet" href="/styles-3ef3ec8d300a680.css" integrity="sha384-..."/>
```

**What these CSS files contain:**

| File | Size | Purpose |
|------|------|---------|
| `antd-pro.css` | 8.9 KB | Design tokens (`--fx-color-primary`, `--fx-sidebar-width`), component base styles (`.fx-sidebar`, `.fx-breadcrumb`, `.fx-panel`) |
| `styles.css` | 23.9 KB | App layout (`.app-layout`, `.app-header`, `.app-sidebar`), page containers, responsive breakpoints, settings drawer |

**These are loaded immediately by the browser** before any JavaScript/WASM executes.

---

### Path 2: Dynamic CSS via ThemeProvider (Runtime)

**When the WASM application starts**, the `ThemeProvider` component executes its effects and injects additional CSS dynamically.

**Execution flow in `src/themes/provider.rs:90-107`:**

```rust
Effect::new(move |_| {
    // STEP 1: Inject base CSS variables (fallbacks)
    inject_base_css();

    // STEP 2: Set data-theme attribute on <html>
    apply_theme_attribute(&theme_signal.get());  // Sets <html data-theme="dark">

    // STEP 3: Load theme-specific CSS from compiled registry
    if let Some(css) = ThemeRegistry::load_theme_css(
        &ctx.design_system().as_str(),  // "ant"
        &theme_signal.get(),             // "dark"
    ) {
        inject_css(THEME_STYLE_ID, &css);  // Creates <style id="ember-theme-css">
    }

    // STEP 4: Load component CSS for the design system
    let component_css = get_component_css(&ctx.design_system().as_str());
    if !component_css.is_empty() {
        inject_css(COMPONENTS_STYLE_ID, component_css);  // Creates <style id="ember-theme-components">
    }
});
```

**How `inject_css` works** (`src/themes/loader.rs:56-76`):

```rust
pub fn inject_css(style_id: &str, css_content: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            // Try to find existing style element, or create new one
            let style_el = document.get_element_by_id(style_id).or_else(|| {
                let el = document.create_element("style").ok()?;
                el.set_id(style_id);
                document.head()?.append_child(&el).ok()?;
                Some(el)
            });

            // Set the CSS content
            if let Some(el) = style_el {
                el.set_text_content(Some(css_content));
            }
        }
    }
}
```

**What gets injected at runtime:**

| Step | Style ID | Source | Content |
|------|----------|--------|---------|
| 1 | `ember-theme-base` | `inject_base_css()` | ~70 lines of CSS variable fallbacks (`:root { --fx-color-primary: oklch(58%...); }`) |
| 2 | `ember-theme-css` | `ThemeRegistry::load_theme_css("ant", "dark")` | Compiled `dark.css` theme (~50 CSS variables in `[data-theme=dark]` selector) |
| 3 | `ember-theme-components` | `get_component_css("ant")` | Compiled `components.css` (~2000+ lines of component styles) |

**The compiled CSS comes from embedded strings:**

```rust
// src/themes/registry.rs:103-111
CompiledTheme {
    name: "dark",
    display_name: "Dark",
    design_system: "ant",
    is_dark: true,
    css: include_str!("../compiled/ant/dark.css"),  // Embedded at compile time
}

// src/themes/registry.rs:171-174
#[cfg(feature = "ant")]
"ant" => include_str!("../compiled/ant/components.css"),
```

---

### Resulting DOM Structure

After both loading paths complete, the `<head>` contains:

```html
<head>
    <!-- Path 1: Trunk static CSS (loaded first, blocks rendering) -->
    <link rel="stylesheet" href="/antd-pro-5c2b08c4735369fd.css" />
    <link rel="stylesheet" href="/styles-3ef3ec8d300a680.css" />

    <!-- Path 2: ThemeProvider dynamic CSS (injected after WASM loads) -->
    <style id="ember-theme-base">
        :root {
            --fx-color-primary: oklch(58% 0.233 277);
            --fx-radius-md: 0.5rem;
            /* ... fallback variables ... */
        }
    </style>
    <style id="ember-theme-css">
        [data-theme=dark] {
            --fx-color-primary: #1668dc;
            --fx-color-base-100: #141414;
            /* ... theme-specific overrides ... */
        }
    </style>
    <style id="ember-theme-components">
        .fx-btn-ant { ... }
        .fx-btn-ant-primary { ... }
        .fx-panel { ... }
        /* ... all component styles ... */
    </style>
</head>

<html data-theme="dark">  <!-- Set by apply_theme_attribute() -->
```

---

### CSS Cascade Order (Specificity)

The loading order creates this cascade:

```
1. ember-theme-base     → :root fallbacks (lowest specificity)
2. ember-theme-css      → [data-theme=dark] overrides (attribute selector)
3. ember-theme-components → .fx-btn-ant-primary (class selectors)
4. antd-pro.css         → :root app tokens + .fx-* component styles
5. styles.css           → .app-layout, .page-container (app-specific)
```

Because **`antd-pro.css` and `styles.css` are loaded via `<link>` tags** (which typically come before injected `<style>` elements in terms of source order), and they use **higher-specificity selectors** or **later source order**, they can override ember-fx defaults.

Example override in `styles.css:1117-1121`:
```css
/* Override ember-fx built-in layout */
.fx-sidebar,
.fx-pro-layout-sidebar,
.fx-pro-layout-header {
    display: none !important;
}
```

---

### Why Two Paths?

| Path | Use Case |
|------|----------|
| **Trunk static CSS** | App-specific layouts, overrides, styles that don't change with theme |
| **ThemeProvider dynamic CSS** | Theme-switchable variables, design system components, reactive updates |

The **static path** ensures critical layout CSS is available immediately (no flash of unstyled content).

The **dynamic path** enables runtime theme switching—when you call `theme_ctx.set_theme("light")`, the Effect re-runs, updates `[data-theme]`, and re-injects the light theme CSS.

---

## 3. Inline Styling in antd-01

**Yes, antd-01 uses inline styling extensively**. The search revealed **63+ inline style usages** across component pages.

### Common Inline Style Patterns Found:

| Pattern | Example Location | Usage |
|---------|------------------|-------|
| Flex layouts | `notification/alert.rs:19` | `style="display: flex; flex-direction: column; gap: 12px;"` |
| Grid layouts | `layout_components/card.rs:47` | `style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 16px;"` |
| Spacing/gaps | `data_display/avatar.rs:19` | `style="display: flex; gap: 16px; align-items: center;"` |
| Fixed dimensions | `notification/progress.rs:53` | `style="width: 120px; height: 120px;"` |
| Theme colors | `app.rs:132-138` | `style="background: #1890ff;"` for color picker buttons |
| Conditional visibility | `nav.rs:340-392` | `style=move \|\| { ... }` reactive inline styles |

### Inline Style Categories:

1. **Demo/Preview Containers**: Most inline styles are on `.demo-box` elements for layout control
2. **Color Swatches**: Settings drawer color options use inline backgrounds
3. **Reactive Styles**: Some navigation elements use reactive `move ||` style closures
4. **Bottom borders/padding**: Footer and section separators

---

## 4. Theme Application Mechanism

### 3-Layer CSS Variable Architecture

**Layer 1: Theme Variables** (set via `data-theme` attribute)

From `src/compiled/ant/dark.css`:
```css
[data-theme=dark] {
    color-scheme: dark;
    --fx-color-primary: #1668dc;
    --fx-color-base-100: #141414;
    --fx-color-text: #ffffffd9;
    /* ... 50+ variables ... */
}
```

**Layer 2: Base/Semantic Variables** (injected by `inject_base_css()`)

From `src/themes/loader.rs:101-149`:
```css
:root {
    --fx-color-primary: oklch(58% 0.233 277);
    --fx-radius-md: 0.5rem;
    --fx-spacing-md: 1rem;
    /* ... fallback values ... */
}
```

**Layer 3: Component CSS** (uses semantic variables)

From `themes/base/ant/button.css:38-42`:
```css
.fx-btn-ant-primary {
    background: var(--fx-color-primary, oklch(55% 0.22 250));
    color: var(--fx-color-primary-content, #fff);
}
```

### Theme Switching Flow

1. `ThemeContext.set_theme("dark")` updates the reactive signal (`src/themes/context.rs:79-82`)
2. Persists to localStorage: `LocalStorage::set(THEME_STORAGE_KEY, theme_name)`
3. Effect triggers `apply_theme_attribute(&theme)` (`src/themes/loader.rs:25-33`)
4. Sets `<html data-theme="dark">` on the document element
5. CSS `[data-theme=dark]` selectors automatically apply

---

## 5. ember-fx Code Primitives for Styling and Theming

### Core Primitives

| Primitive | Location | Purpose |
|-----------|----------|---------|
| `ThemeProvider` | `src/themes/provider.rs:41-125` | Context provider + CSS injection |
| `ThemeContext` | `src/themes/context.rs:34-175` | Reactive theme state management |
| `DesignSystem` | `src/themes/design_system.rs:20-51` | Design system enum (Ant, Material, etc.) |
| `ThemeRegistry` | `src/themes/registry.rs:24-165` | Compiled theme storage/lookup |
| `ThemeLoader` | `src/themes/loader.rs:185-217` | Component for CSS injection |

### CSS Utility Functions

| Function | Location | Purpose |
|----------|----------|---------|
| `inject_css(style_id, css)` | `src/themes/loader.rs:56-76` | Inject/update `<style>` element |
| `apply_theme_attribute(name)` | `src/themes/loader.rs:25-33` | Set `data-theme` on `<html>` |
| `inject_base_css()` | `src/themes/loader.rs:100-172` | Inject fallback variables |
| `remove_css(style_id)` | `src/themes/loader.rs:79-87` | Remove style element |

### Component Class Generation

From `src/themes/context.rs:163-169`:
```rust
pub fn component_class(&self, component: &str, variant: &str) -> String {
    let prefix = self.class_prefix(); // "ant", "material", etc.
    format!(
        "fx-{}-{} fx-{}-{}-{}",
        component, prefix, component, prefix, variant
    )
}
```

From `src/components/button/component.rs:92-95`:
```rust
let btn_prefix = format!("{}-btn", class_prefix);  // "ant-btn"
let variant_class = variant.class(&btn_prefix);    // "ant-btn-primary"
let size_class = size.class(&btn_prefix);          // "ant-btn-lg"
```

### CSS Class Naming Convention

Pattern: `fx-{component}-{design_system}-{variant}`

Examples:
- `.fx-btn-ant` (base button)
- `.fx-btn-ant-primary` (variant)
- `.fx-btn-ant-lg` (size)
- `.fx-panel` (component wrapper)
- `.fx-trend-color-success` (state)

---

## 6. Style/Theme Configuration

### Note: No tailwind.config.js exists in ember-fx

ember-fx uses a custom CSS variable system, not Tailwind.

### Actual Configuration Sources

**1. Theme Source Files** (`themes/base/ant/`):
```
_index.css      # Master import file
button.css      # Button component styles
card.css        # Card component styles
input.css       # Input component styles
notification.css    # Alert, Badge, Progress, etc.
layout.css      # Modal, Drawer, Tabs, etc.
navigation.css  # Menu, Breadcrumb, Steps, etc.
variants.css    # Theme variant modifiers
...
```

**2. Compiled Themes** (`src/compiled/ant/`):

| File | Description |
|------|-------------|
| `dark.css` | Dark theme variables |
| `light.css` | Light theme variables |
| `glass.css` | Glassmorphism theme |
| `skeumorph.css` | Bootstrap-like 3D effects |
| `shad.css` | shadcn/ui minimalist style |
| `mui.css` | Material UI style |
| `sky.css` | Dark theme with sky blue primary |
| `components.css` | All component styles combined |

**3. Build-time CSS Compilation**

From `Cargo.toml:84-87`:
```toml
[build-dependencies]
lightningcss = "1.0.0-alpha.71"
walkdir = "2.5"
```

The `build.rs` file compiles and minifies CSS at build time using LightningCSS.

### Configuration Connection to antd-01

**CSS Flow:**
```
themes/base/ant/*.css
    ↓ (build.rs compiles)
src/compiled/ant/components.css
src/compiled/ant/{theme}.css
    ↓ (include_str! embeds)
ThemeRegistry (runtime)
    ↓ (inject_css)
<style id="ember-theme-css">
<style id="ember-theme-components">
```

**antd-01 Overrides:**

`examples/antd-01/styles.css:6-20` defines **app-specific CSS variables**:
```css
:root {
    --sidebar-width: 208px;
    --sidebar-collapsed-width: 64px;
    --header-height: 56px;
    --sidebar-bg: #000000;
    --header-bg: #000000;
    --content-bg: #141414;
    --card-bg: #1f1f1f;
    /* ... */
}
```

`examples/antd-01/antd-pro.css:6-58` defines **fx-prefixed design tokens**:
```css
:root {
    --fx-color-primary: #1890ff;
    --fx-sidebar-width: 208px;
    --fx-header-height: 48px;
    /* ... */
}
```

**Override Pattern** (`examples/antd-01/styles.css:1115-1121`):
```css
/* Override ember-fx styles */
.fx-sidebar,
.fx-pro-layout-sidebar,
.fx-pro-layout-header {
    display: none !important;
}
```

---

## Summary

| Aspect | Implementation |
|--------|----------------|
| **Component Import** | Feature-gated Cargo dependencies + Rust imports |
| **CSS Delivery** | Compiled CSS embedded via `include_str!()`, injected at runtime |
| **Theme Switching** | `data-theme` attribute + CSS variable selectors |
| **Inline Styles** | Yes, 63+ instances for demo layouts and reactive styles |
| **Design System** | `DesignSystem::Ant` enum controls class prefix (`fx-btn-ant`) |
| **Customization** | Local CSS files override/extend ember-fx defaults |
| **Build System** | LightningCSS minification in `build.rs` |
| **No Tailwind** | Pure CSS variables, not Tailwind-based |

---

## Open Questions

- [ ] Should inline styles in demo pages be refactored to CSS classes?
- [ ] Is the dual CSS loading causing any FOUC (flash of unstyled content)?
- [ ] Should antd-01 use `MinimalThemeProvider` instead since it has static CSS?
