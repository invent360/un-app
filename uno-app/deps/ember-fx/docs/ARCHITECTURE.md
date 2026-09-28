# ember-fx Architecture

This document describes the internal architecture of ember-fx, including the CSS loading mechanism, theme system, component patterns, and design system extensibility.

## Table of Contents

1. [Overview](#overview)
2. [CSS Loading Mechanism](#css-loading-mechanism)
   - [Theme Switching](#theme-switching)
   - [Why Both Attribute AND CSS Injection?](#why-both-attribute-and-css-injection)
3. [Theme System Architecture](#theme-system-architecture)
   - [CSS Variable Hierarchy](#css-variable-hierarchy)
   - [Theme Comparison: Light vs Dark](#theme-comparison-light-vs-dark)
4. [Component Architecture](#component-architecture)
5. [Design System Extension](#design-system-extension)
6. [Troubleshooting Guide](#troubleshooting-guide)

---

## Overview

ember-fx is a multi-crate Leptos component library with these core characteristics:

- **Workspace Structure**: 11 crates with clear separation of concerns
- **Dual-Path CSS**: Build-time compilation + runtime injection
- **Multi-Design System**: Ant, Material, Cupertino, DaisyUI, Prime, Flutter
- **Platform-Aware**: Auto-detects iOS/Android/Desktop for appropriate defaults
- **Reactive**: Built on Leptos signals for efficient updates

### Crate Dependency Graph

```
                    ┌─────────────┐
                    │   common    │  (enums, traits, constants)
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │    utils    │ │   macros    │ │   styles    │
    │ (DOM, CSS)  │ │ (proc-macro)│ │ (registry)  │
    └──────┬──────┘ └─────────────┘ └──────┬──────┘
           │                               │
           └───────────────┬───────────────┘
                           │
                    ┌──────▼──────┐
                    │    core     │  (ThemeProvider, Context)
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │   systems   │ │    icons    │ │ components  │
    │(Ant, Mat...)│ │  (heroicons)│ │ (Button...) │
    └─────────────┘ └─────────────┘ └──────┬──────┘
                                           │
                           ┌───────────────┼───────────────┐
                           │               │               │
                    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
                    │    tools    │ │   mobile    │ │    demos    │
                    │ (debugger)  │ │ (nav, swipe)│ │  (antd-01)  │
                    └─────────────┘ └─────────────┘ └─────────────┘
```

---

## CSS Loading Mechanism

ember-fx uses a **dual-path CSS strategy** combining build-time compilation with runtime injection.

### Build-Time Compilation

The `styles` crate compiles themes at build time via `build.rs`:

```
themes/
├── presets/           # JSON theme definitions
│   └── ant/
│       ├── dark.json
│       ├── light.json
│       └── glass.json
└── base/              # Component CSS files
    └── ant/
        ├── button.css
        ├── input.css
        └── modal.css
            ↓
        build.rs
            ↓
src/compiled/
├── ant/
│   ├── dark.css       # Minified theme CSS
│   ├── light.css
│   └── components.css # Concatenated component CSS
└── manifest.json      # Theme registry
```

#### JSON Theme Format

```json
{
  "name": "dark",
  "colorScheme": "dark",
  "colors": {
    "primary": "#1668dc",
    "primaryBg": "#111a2c",
    "primaryHover": "#3c89e8"
  },
  "radius": {
    "sm": "2px",
    "md": "6px",
    "lg": "8px"
  },
  "spacing": {
    "xs": "4px",
    "sm": "8px",
    "md": "16px"
  }
}
```

#### Generated CSS Output

```css
[data-theme="dark"] {
  color-scheme: dark;
  --fx-color-primary: #1668dc;
  --fx-color-primary-bg: #111a2c;
  --fx-color-primary-hover: #3c89e8;
  --fx-radius-sm: 2px;
  --fx-radius-md: 6px;
  --fx-spacing-xs: 4px;
  /* ... */
}
```

### Runtime CSS Injection

The `utils` crate provides runtime CSS management:

```rust
// crates/utils/src/css_loader.rs

/// Inject CSS into document head
pub fn inject_css(style_id: &str, css: &str) {
    // Creates/updates <style id="{style_id}">css</style>
}

/// Set data-theme attribute on <html>
pub fn apply_theme_attribute(theme: &str) {
    // <html data-theme="dark">
}

/// Inject base CSS variables with fallbacks
pub fn inject_base_css() {
    // :root { --fx-color-primary: oklch(58% 0.233 277); ... }
}
```

#### Style Element IDs

| ID | Purpose |
|----|---------|
| `ember-theme-css` | Theme-specific CSS variables |
| `ember-theme-components` | Component styling |
| `ember-theme-base` | Base CSS variables (fallbacks) |

### CSS Loading Flow

```
Application Start
       │
       ▼
┌──────────────────────────────────┐
│     ThemeProvider mounted        │
└──────────────────┬───────────────┘
                   │
       ┌───────────┴───────────┐
       │                       │
       ▼                       ▼
┌──────────────┐      ┌──────────────┐
│inject_base_  │      │Load saved    │
│css()         │      │preferences   │
│              │      │(localStorage)│
└──────┬───────┘      └──────┬───────┘
       │                     │
       └─────────┬───────────┘
                 ▼
┌────────────────────────────────────┐
│ apply_theme_attribute("dark")      │
│ → <html data-theme="dark">         │
└──────────────────┬─────────────────┘
                   │
                   ▼
┌────────────────────────────────────┐
│ ThemeRegistry::get_theme_css()     │
│ → Get embedded CSS from registry   │
└──────────────────┬─────────────────┘
                   │
                   ▼
┌────────────────────────────────────┐
│ inject_css(THEME_STYLE_ID, css)    │
│ → Insert <style> into <head>       │
└──────────────────┬─────────────────┘
                   │
                   ▼
┌────────────────────────────────────┐
│ inject_css(COMPONENTS_STYLE_ID,    │
│            component_css)          │
└────────────────────────────────────┘
```

### Theme Switching

When the user changes theme:

1. **Signal Update**: `theme_ctx.set_theme("light")`
2. **Persist**: Save to localStorage
3. **Reactive Effect**: Leptos effect triggered
4. **Attribute Update**: `<html data-theme="light">`
5. **CSS Update**: New theme CSS injected
6. **Cascade**: CSS variables update all components

No page reload required - pure CSS cascade handles the visual update.

#### Why Both Attribute AND CSS Injection?

You might wonder why we need both `apply_theme_attribute()` and `inject_css()`:

```rust
// Set attribute for CSS cascade
apply_theme_attribute(&theme_signal.get());     // → <html data-theme="dark">

// Inject theme-specific variables
if let Some(css) = ThemeRegistry::load_theme_css(...) {
    inject_css(THEME_STYLE_ID, &css);           // → ember-theme-css
}
```

**The injected CSS is "dormant" until the attribute matches:**

```css
/* This CSS does nothing until <html data-theme="dark"> exists */
[data-theme="dark"] { --fx-color-primary: #1668dc; }
```

So you need both:
1. The CSS rules (injected into `<head>`)
2. The attribute on `<html>` to activate them

#### Alternative: Direct `:root` Injection

You could inject CSS directly to `:root` without attribute selectors:

```css
:root { --fx-color-primary: #1668dc; }  /* No [data-theme] selector */
```

Then `apply_theme_attribute()` wouldn't be needed. **But this has downsides:**

| Aspect | With Attribute Selector | Without (direct `:root`) |
|--------|------------------------|------------------------|
| Multiple themes coexist | Yes - dark/light CSS can both be present | No - must replace all CSS |
| Theme switch cost | Change 1 attribute | Re-inject entire CSS block |
| Browser optimization | CSS cascade (fast) | DOM manipulation (slower) |
| Debugging | Inspect `data-theme` attr | No visible indicator |

#### Design Rationale

The attribute selector pattern enables a **hybrid approach**:

1. **Current behavior**: Inject only the active theme's CSS, switch by re-injecting
2. **Future optimization**: Pre-load all themes into one CSS file, switch instantly via attribute only

This is **future-proof**—you could pre-compile all themes into a static CSS file:

```css
/* All themes in one file */
[data-theme="dark"] { --fx-color-primary: #1668dc; ... }
[data-theme="light"] { --fx-color-primary: #1677ff; ... }
[data-theme="glass"] { --fx-color-primary: #3b82f6; ... }
```

Then theme switching becomes instant (just attribute change, zero JS/DOM cost).

---

## Theme System Architecture

### Core Components

#### ThemeContext

Central state management using Leptos signals:

```rust
// crates/core/src/context.rs

pub struct ThemeContext {
    theme: RwSignal<String>,           // "dark", "light", etc.
    design_system: RwSignal<DesignSystem>,  // Ant, Material, etc.
    platform: Platform,                 // Detected platform
}

impl ThemeContext {
    pub fn get_theme(&self) -> String;
    pub fn set_theme(&self, name: &str);
    pub fn is_dark(&self) -> bool;
    pub fn toggle(&self);
    pub fn get_design_system(&self) -> DesignSystem;
    pub fn get_class_prefix(&self) -> &'static str;
    pub fn get_component_class(&self, component: &str, variant: Option<&str>) -> String;
}
```

#### ThemeProvider

Leptos component that wraps your application:

```rust
// crates/core/src/provider.rs

#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_theme: Option<String>,
    #[prop(optional)] design_system: Option<DesignSystem>,
    #[prop(optional)] persist: Option<bool>,
    children: Children,
) -> impl IntoView {
    // 1. Detect platform
    // 2. Load saved preferences
    // 3. Create ThemeContext
    // 4. Inject CSS (Effects)
    // 5. Provide context to children
}
```

#### ThemeRegistry

Manages embedded, pre-compiled themes:

```rust
// crates/styles/src/registry.rs

pub struct CompiledTheme {
    pub name: &'static str,
    pub display_name: &'static str,
    pub design_system: &'static str,
    pub is_dark: bool,
    pub css: &'static str,  // Embedded via include_str!()
}

impl ThemeRegistry {
    pub fn get_theme_css(design_system: &str, theme: &str) -> Option<&'static str>;
    pub fn get_themes_for_system(design_system: &str) -> Vec<&'static CompiledTheme>;
    pub fn get_all_themes() -> HashMap<&'static str, Vec<&'static CompiledTheme>>;
    pub fn get_default_theme(design_system: &str) -> &'static str;
}
```

### Design System Enum

```rust
// crates/common/src/design_system.rs

pub enum DesignSystem {
    Ant,       // Enterprise web (default)
    Material,  // Android / Google
    Cupertino, // iOS / Apple
    DaisyUI,   // Tailwind-based
    Prime,     // PrimeReact-inspired
    Flutter,   // Cross-platform
    Auto,      // Platform-aware selection
}

impl DesignSystem {
    pub fn get_class_prefix(&self) -> &'static str;
    pub fn get_display_name(&self) -> &'static str;
    pub fn get_min_touch_target(&self) -> u32;
    pub fn uses_rounded_corners(&self) -> bool;  // predicate, no get_ prefix
}
```

### CSS Variable Hierarchy

ember-fx uses a three-layer CSS architecture where each layer has a distinct purpose and specificity level. This design enables theme switching without page reloads and provides graceful fallbacks.

```
┌─────────────────────────────────────────────┐
│ Layer 1: Base CSS (inject_base_css)         │
│ :root { --fx-color-primary: oklch(...); }   │
│ Fallback values, always present             │
└─────────────────────────────────────────────┘
                     ▲
                     │ overrides (higher specificity)
┌─────────────────────────────────────────────┐
│ Layer 2: Theme CSS ([data-theme="dark"])    │
│ [data-theme="dark"] {                       │
│   --fx-color-primary: #1668dc;              │
│ }                                           │
│ Theme-specific values                       │
└─────────────────────────────────────────────┘
                     ▲
                     │ consumes (var() references)
┌─────────────────────────────────────────────┐
│ Layer 3: Component CSS                      │
│ .fx-btn-ant {                               │
│   background: var(--fx-color-primary);      │
│ }                                           │
│ Design-system-specific component styles     │
└─────────────────────────────────────────────┘
```

---

#### Layer 1: Base CSS (Foundation Layer)

**Purpose**: Provide universal fallback values that ensure components always have valid CSS variable values, even if theme CSS fails to load.

**Selector**: `:root` (lowest specificity for variables)

**Utility Method**:
```rust
// crates/utils/src/css_loader.rs

/// Injects base CSS variables into the document head.
/// Called once during ThemeProvider initialization.
/// Creates <style id="ember-theme-base">
pub fn inject_base_css() {
    let base_css = r#"
        :root {
            /* Color primitives using oklch for perceptual uniformity */
            --fx-color-primary: oklch(58% 0.233 277);
            --fx-color-primary-hover: oklch(63% 0.233 277);
            --fx-color-primary-bg: oklch(25% 0.05 277);

            /* Semantic colors */
            --fx-color-success: oklch(65% 0.2 145);
            --fx-color-warning: oklch(75% 0.15 85);
            --fx-color-error: oklch(55% 0.25 27);

            /* Neutral palette */
            --fx-color-base-100: oklch(20% 0.02 260);
            --fx-color-base-200: oklch(25% 0.02 260);
            --fx-color-base-300: oklch(30% 0.02 260);
            --fx-color-base-content: oklch(95% 0.01 260);

            /* Typography */
            --fx-font-family: system-ui, -apple-system, sans-serif;
            --fx-font-size-base: 14px;
            --fx-line-height: 1.5;

            /* Spacing scale */
            --fx-spacing-xs: 4px;
            --fx-spacing-sm: 8px;
            --fx-spacing-md: 16px;
            --fx-spacing-lg: 24px;
            --fx-spacing-xl: 32px;

            /* Border radius */
            --fx-radius-sm: 2px;
            --fx-radius-md: 6px;
            --fx-radius-lg: 8px;
            --fx-radius-full: 9999px;

            /* Transitions */
            --fx-transition-fast: 0.1s ease;
            --fx-transition-normal: 0.2s ease;
            --fx-transition-slow: 0.3s ease;

            /* Shadows */
            --fx-shadow-sm: 0 1px 2px rgba(0,0,0,0.1);
            --fx-shadow-md: 0 4px 6px rgba(0,0,0,0.1);
            --fx-shadow-lg: 0 10px 15px rgba(0,0,0,0.1);
        }
    "#;
    inject_css("ember-theme-base", base_css);
}
```

**Key Characteristics**:
- Uses `:root` selector (equivalent to `html`, but semantic for variables)
- Employs `oklch()` color space for perceptual uniformity across light/dark
- Defines the complete design token vocabulary
- Acts as documentation of available variables
- Loaded once at app startup, never removed

**When It's Used**:
- Before any theme loads (prevents flash of unstyled content)
- As fallback when theme CSS variable is missing
- For variables that are theme-agnostic (spacing, transitions)

**Important**: Base CSS only provides defaults—it does **not** style the application. The `:root` selector has lower specificity than `[data-theme="x"]`, so theme values always win:

```css
:root {                                    /* ← ember-theme-base (always present) */
  --fx-color-primary: oklch(58% 0.233 277);   /* ← fallback value */
}

[data-theme="dark"] {                      /* ← ember-theme-css (overrides) */
  --fx-color-primary: #1668dc;                /* ← actual theme value wins */
}
```

**When base values are actually used:**
- App startup, before theme CSS injects (brief moment)
- If `ThemeRegistry::load_theme_css()` returns `None` (theme not found)
- For variables the active theme doesn't override (e.g., `--fx-transition-fast` may remain at base value if theme omits it)

---

#### Layer 2: Theme CSS (Customization Layer)

**Purpose**: Override base variables with theme-specific values. Each theme defines its own color palette and visual characteristics.

**Selector**: `[data-theme="themename"]` (attribute selector, higher specificity than `:root`)

**Utility Methods**:
```rust
// crates/utils/src/css_loader.rs

/// Sets the data-theme attribute on <html> element.
/// This activates the corresponding theme's CSS variables.
pub fn apply_theme_attribute(theme: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", theme);
            }
        }
    }
}

/// Injects theme-specific CSS into document head.
/// Creates/updates <style id="ember-theme-css">
pub fn inject_theme_css(css: &str) {
    inject_css("ember-theme-css", css);
}
```

```rust
// crates/styles/src/registry.rs

impl ThemeRegistry {
    /// Retrieves pre-compiled CSS for a specific theme.
    /// CSS is embedded at build time via include_str!()
    pub fn get_theme_css(design_system: &str, theme: &str) -> Option<&'static str> {
        match (design_system, theme) {
            ("ant", "dark") => Some(include_str!("compiled/ant/dark.css")),
            ("ant", "light") => Some(include_str!("compiled/ant/light.css")),
            ("ant", "glass") => Some(include_str!("compiled/ant/glass.css")),
            // ... other themes
            _ => None,
        }
    }
}
```

**Compiled Theme CSS Example** (generated from JSON preset):
```css
/* compiled/ant/dark.css */
[data-theme="dark"] {
    color-scheme: dark;

    /* Override primary color for dark theme */
    --fx-color-primary: #1668dc;
    --fx-color-primary-hover: #3c89e8;
    --fx-color-primary-bg: #111a2c;
    --fx-color-primary-border: #15325b;

    /* Dark neutral palette */
    --fx-color-base-100: #141414;
    --fx-color-base-200: #1f1f1f;
    --fx-color-base-300: #2a2a2a;
    --fx-color-base-content: rgba(255, 255, 255, 0.88);

    /* Component-specific overrides */
    --fx-input-bg: #1f1f1f;
    --fx-input-border: #424242;
    --fx-card-bg: #1f1f1f;
}

/* compiled/ant/light.css */
[data-theme="light"] {
    color-scheme: light;

    --fx-color-primary: #1677ff;
    --fx-color-primary-hover: #4096ff;
    --fx-color-primary-bg: #e6f4ff;
    --fx-color-primary-border: #91caff;

    /* Light neutral palette */
    --fx-color-base-100: #ffffff;
    --fx-color-base-200: #fafafa;
    --fx-color-base-300: #f5f5f5;
    --fx-color-base-content: rgba(0, 0, 0, 0.88);
}
```

**Key Characteristics**:
- Attribute selector `[data-theme="x"]` has higher specificity than `:root`
- Only overrides variables that differ from base (sparse override pattern)
- Includes `color-scheme` for native browser dark mode hints
- Multiple themes can coexist in CSS; only one is active via attribute
- Theme switching = changing one attribute (instant, no reload)

**CSS Specificity Explanation**:
```
:root                     → specificity: (0, 0, 1) - one pseudo-class
[data-theme="dark"]       → specificity: (0, 1, 0) - one attribute
```
The attribute selector wins, so theme values override base values.

---

#### Layer 3: Component CSS (Consumption Layer)

**Purpose**: Define component structure and styling using CSS variables. Components are design-system-aware but theme-agnostic.

**Selector**: `.fx-{component}-{design-system}` (class selectors)

**Utility Method**:
```rust
// crates/utils/src/css_loader.rs

/// Injects component CSS for the active design system.
/// Creates/updates <style id="ember-theme-components">
pub fn inject_component_css(css: &str) {
    inject_css("ember-theme-components", css);
}
```

**Component CSS Example**:
```css
/* themes/base/ant/button.css */

/* Base button structure */
.fx-btn-ant {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--fx-spacing-xs);

    /* Consume theme variables */
    font-family: var(--fx-font-family);
    font-size: var(--fx-font-size-base);
    line-height: var(--fx-line-height);

    padding: var(--fx-spacing-xs) var(--fx-spacing-md);
    border-radius: var(--fx-radius-md);
    border: 1px solid transparent;

    cursor: pointer;
    transition: all var(--fx-transition-normal);
}

/* Variant: Primary */
.fx-btn-ant-primary {
    background: var(--fx-color-primary);
    color: #fff;
    border-color: var(--fx-color-primary);
}

.fx-btn-ant-primary:hover:not(:disabled) {
    background: var(--fx-color-primary-hover);
    border-color: var(--fx-color-primary-hover);
}

/* Variant: Secondary/Default */
.fx-btn-ant-secondary {
    background: var(--fx-color-base-100);
    color: var(--fx-color-base-content);
    border-color: var(--fx-color-base-300);
}

/* Size variants */
.fx-btn-ant-sm {
    padding: var(--fx-spacing-xs) var(--fx-spacing-sm);
    font-size: 12px;
}

.fx-btn-ant-lg {
    padding: var(--fx-spacing-sm) var(--fx-spacing-lg);
    font-size: 16px;
}

/* State: Disabled */
.fx-btn-ant-disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
```

**Key Characteristics**:
- Uses `var()` to consume variables from Layers 1 & 2
- Never hardcodes colors (except pure white/black for contrast)
- Design-system suffix (`-ant`, `-material`) enables multiple systems
- Focuses on structure, layout, and behavior
- Theme-agnostic: same CSS works for all themes

---

#### How Variables Flow to UI Components

The following diagram shows the complete flow from variable definition to rendered component:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           APPLICATION STARTUP                               │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 1. ThemeProvider mounts                                                     │
│    ├── inject_base_css()              → <style id="ember-theme-base">       │
│    ├── apply_theme_attribute("dark")  → <html data-theme="dark">            │
│    ├── inject_theme_css(dark_css)     → <style id="ember-theme-css">        │
│    └── inject_component_css(ant_css)  → <style id="ember-theme-components"> │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 2. CSS Cascade Resolution (Browser)                                         │
│                                                                             │
│    :root { --fx-color-primary: oklch(58% 0.233 277); }     ← Layer 1 (base) │
│                          ↓ overridden by                                    │
│    [data-theme="dark"] { --fx-color-primary: #1668dc; }    ← Layer 2 (theme)│
│                          ↓ consumed by                                      │
│    .fx-btn-ant { background: var(--fx-color-primary); }    ← Layer 3 (comp) │
│                                                                             │
│    Computed value: background: #1668dc;                                     │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 3. Component Renders with Resolved Styles                                   │
│                                                                             │
│    <Button variant=Primary>"Click me"</Button>                              │
│                          ↓                                                  │
│    <button class="fx-btn-ant fx-btn-ant-primary">Click me</button>          │
│                          ↓                                                  │
│    Rendered with background: #1668dc (from dark theme)                      │
└─────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 4. Theme Switch (User clicks "Light Mode")                                  │
│                                                                             │
│    theme_ctx.set_theme("light")                                             │
│         ↓                                                                   │
│    apply_theme_attribute("light")  → <html data-theme="light">              │
│         ↓                                                                   │
│    Browser recalculates CSS variables:                                      │
│    [data-theme="light"] { --fx-color-primary: #1677ff; }                    │
│         ↓                                                                   │
│    All components using var(--fx-color-primary) update instantly            │
│         ↓                                                                   │
│    Button now has background: #1677ff (no re-render needed!)                │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

#### Concrete Example: Button Component Lifecycle

```rust
// In your application
view! {
    <Button variant=ButtonVariant::Primary size=ButtonSize::Large>
        "Submit"
    </Button>
}
```

**Step 1: Component generates class names**
```rust
// Inside Button component
let classes = "fx-btn-ant fx-btn-ant-primary fx-btn-ant-lg";
```

**Step 2: Browser resolves CSS**
```css
/* From Layer 3 (component CSS) */
.fx-btn-ant {
    padding: var(--fx-spacing-xs) var(--fx-spacing-md);  /* → 4px 16px */
    border-radius: var(--fx-radius-md);                  /* → 6px */
    transition: all var(--fx-transition-normal);         /* → 0.2s ease */
}

.fx-btn-ant-primary {
    background: var(--fx-color-primary);                 /* → #1668dc (dark) */
    color: #fff;
}

.fx-btn-ant-lg {
    padding: var(--fx-spacing-sm) var(--fx-spacing-lg); /* → 8px 24px */
}
```

**Step 3: Final computed styles**
```css
/* What the browser actually applies */
button.fx-btn-ant.fx-btn-ant-primary.fx-btn-ant-lg {
    display: inline-flex;
    padding: 8px 24px;           /* lg overrides base */
    border-radius: 6px;
    background: #1668dc;          /* from dark theme */
    color: #fff;
    transition: all 0.2s ease;
}
```

**Step 4: On theme switch to "light"**
```css
/* Only these values change, structure stays the same */
button.fx-btn-ant.fx-btn-ant-primary.fx-btn-ant-lg {
    /* ... same structure ... */
    background: #1677ff;          /* from light theme - instant update! */
}
```

---

#### Layer Comparison Summary

| Aspect | Layer 1: Base | Layer 2: Theme | Layer 3: Component |
|--------|---------------|----------------|-------------------|
| **Selector** | `:root` | `[data-theme="x"]` | `.fx-{comp}-{sys}` |
| **Specificity** | (0,0,1) | (0,1,0) | (0,1,0)+ |
| **Injection Method** | `inject_base_css()` | `inject_theme_css()` | `inject_component_css()` |
| **Style Element ID** | `ember-theme-base` | `ember-theme-css` | `ember-theme-components` |
| **When Loaded** | App startup (once) | Theme switch | App startup (once) |
| **Contains** | Fallback values | Theme overrides | Component structure |
| **Defines** | Token vocabulary | Visual identity | UI patterns |
| **Changes** | Never | On theme switch | Never |

---

#### Theme Comparison: Light vs Dark

The following comparison illustrates how the same CSS variable names resolve to different values based on the active theme.

##### JSON Presets Side-by-Side

| Property | Dark | Light |
|----------|------|-------|
| `colorScheme` | `"dark"` | `"light"` |
| **Primary** |
| `primary` | `#1668dc` | `#1677ff` |
| `primaryBg` | `#111a2c` | `#e6f4ff` |
| `primaryHover` | `#3c89e8` | `#4096ff` |
| `primaryActive` | `#1554ad` | `#0958d9` |
| **Base/Background** |
| `base-100` | `#141414` | `#ffffff` |
| `base-200` | `#1f1f1f` | `#fafafa` |
| `base-300` | `#2a2a2a` | `#f5f5f5` |
| `baseContent` | `rgba(255,255,255,0.85)` | `rgba(0,0,0,0.88)` |
| **Semantic** |
| `success` | `#49aa19` | `#52c41a` |
| `successBg` | `#162312` | `#f6ffed` |
| `warning` | `#d89614` | `#faad14` |
| `warningBg` | `#2b2111` | `#fffbe6` |
| `error` | `#dc4446` | `#ff4d4f` |
| `errorBg` | `#2c1618` | `#fff2f0` |
| **Border** |
| `border` | `#424242` | `#d9d9d9` |
| `borderSecondary` | `#303030` | `#f0f0f0` |
| **Text** |
| `text` | `rgba(255,255,255,0.85)` | `rgba(0,0,0,0.88)` |
| `textSecondary` | `rgba(255,255,255,0.65)` | `rgba(0,0,0,0.65)` |
| `textDisabled` | `rgba(255,255,255,0.25)` | `rgba(0,0,0,0.25)` |
| **Fill** |
| `fill` | `rgba(255,255,255,0.18)` | `rgba(0,0,0,0.15)` |
| **Shadows** |
| `shadow` | Heavy (`0.5` opacity) | Light (`0.03` opacity) |

##### Compiled CSS Comparison

**Dark Theme** (`[data-theme=dark]`):
```css
[data-theme=dark] {
  color-scheme: dark;

  /* Inverted: light text on dark backgrounds */
  --fx-color-base-100: #141414;
  --fx-color-base-200: #1f1f1f;
  --fx-color-base-300: #2a2a2a;
  --fx-color-base-content: #ffffffd9;        /* white @ 85% */
  --fx-color-text: #ffffffd9;

  /* Darker, saturated primary */
  --fx-color-primary: #1668dc;
  --fx-color-primary-bg: #111a2c;            /* very dark blue */
  --fx-color-primary-hover: #3c89e8;

  /* Semantic with dark backgrounds */
  --fx-color-success-bg: #162312;            /* dark green */
  --fx-color-error-bg: #2c1618;              /* dark red */
  --fx-color-warning-bg: #2b2111;            /* dark amber */

  /* Heavier shadows for depth */
  --fx-effect-shadow: 0 1px 2px 0 #00000080, 0 1px 6px -1px #0006, 0 2px 4px 0 #0006;

  /* Light fills on dark */
  --fx-color-fill: #ffffff2e;                /* white @ 18% */
  --fx-color-border: #424242;
}
```

**Light Theme** (`[data-theme=light]`):
```css
[data-theme=light] {
  color-scheme: light;

  /* Normal: dark text on light backgrounds */
  --fx-color-base-100: #fff;
  --fx-color-base-200: #fafafa;
  --fx-color-base-300: #f5f5f5;
  --fx-color-base-content: #000000e0;        /* black @ 88% */
  --fx-color-text: #000000e0;

  /* Brighter primary */
  --fx-color-primary: #1677ff;
  --fx-color-primary-bg: #e6f4ff;            /* light blue tint */
  --fx-color-primary-hover: #4096ff;

  /* Semantic with light backgrounds */
  --fx-color-success-bg: #f6ffed;            /* light green */
  --fx-color-error-bg: #fff2f0;              /* light red/pink */
  --fx-color-warning-bg: #fffbe6;            /* light yellow */

  /* Subtle shadows */
  --fx-effect-shadow: 0 1px 2px 0 #00000008, 0 1px 6px -1px #00000005, 0 2px 4px 0 #00000005;

  /* Dark fills on light */
  --fx-color-fill: #00000026;                /* black @ 15% */
  --fx-color-border: #d9d9d9;
}
```

##### Visual Impact on Button Component

The **same** component CSS applies to both themes:

```css
.fx-btn-ant-primary {
  background: var(--fx-color-primary);
  border-color: var(--fx-color-primary);
  color: var(--fx-color-primary-content);
}
```

**Resolved values:**

| Property | Dark Theme | Light Theme |
|----------|------------|-------------|
| `background` | `#1668dc` | `#1677ff` |
| `border-color` | `#1668dc` | `#1677ff` |
| `color` | `#fff` | `#fff` |
| hover `background` | `#3c89e8` | `#4096ff` |

##### Theme Switch Flow

```
User clicks "Light Mode"
         │
         ▼
theme_ctx.set_theme("light")
         │
         ▼
apply_theme_attribute("light")
         │
         ▼
<html data-theme="light">      ← attribute changes
         │
         ▼
Browser recalculates CSS:
  [data-theme=light] { ... }   ← now matches
  [data-theme=dark] { ... }    ← no longer matches
         │
         ▼
All var(--fx-*) references resolve to light values
         │
         ▼
Button background: #1668dc → #1677ff  (instant!)
```

No component re-renders needed—pure CSS cascade handles the visual update.

---

#### Why This Architecture?

1. **Instant Theme Switching**: Changing `data-theme` attribute triggers CSS cascade recalculation. No JavaScript re-renders, no DOM manipulation, no style recalculation per component.

2. **Graceful Degradation**: If theme CSS fails to load, base variables ensure components remain functional and styled.

3. **Separation of Concerns**:
   - Designers define tokens (Layer 1)
   - Themes customize appearance (Layer 2)
   - Developers build components (Layer 3)

4. **Bundle Size Optimization**: Component CSS is loaded once. Only small theme CSS changes on switch.

5. **Design System Flexibility**: Same component CSS works across all themes. New themes only need to define variable overrides.

### Platform Detection

```rust
// crates/core/src/platform.rs

pub enum Platform {
    IOS,      // → Cupertino
    Android,  // → Material
    MacOS,    // → Ant
    Windows,  // → Ant
    Linux,    // → Ant
    Web,      // → Ant (fallback)
}

impl Platform {
    pub fn detect() -> Self;
    pub fn default_design_system(&self) -> DesignSystem;
}
```

### Storage Keys

| Key | Purpose |
|-----|---------|
| `ember_theme` | Saved theme name |
| `ember_design_system` | Saved design system |

---

## Component Architecture

### File Structure

```
crates/components/src/button/
├── mod.rs          # Module declarations, re-exports
├── component.rs    # Main component implementation
└── types.rs        # ButtonVariant, ButtonSize enums
```

### Component Implementation Pattern

```rust
// crates/components/src/button/component.rs

#[component]
pub fn Button(
    // Variant/Size props (optional with defaults)
    #[prop(optional, into)]
    variant: Option<ButtonVariant>,

    #[prop(optional, into)]
    size: Option<ButtonSize>,

    // State props (booleans)
    #[prop(optional)]
    disabled: bool,

    #[prop(optional)]
    loading: bool,

    // Content props
    #[prop(optional, into)]
    icon: Option<String>,

    // Handler props
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,

    // Custom CSS
    #[prop(optional, into)]
    class: Option<String>,

    // Children
    children: Children,
) -> impl IntoView {
    // 1. Get theme context
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.get_class_prefix())
        .unwrap_or("ant");

    // 2. Resolve defaults
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();

    // 3. Build CSS classes
    let btn_prefix = format!("fx-btn-{}", design_system);
    let variant_class = variant.class(&btn_prefix);
    let size_class = size.class(&btn_prefix);

    // 4. Combine classes (reactive)
    let combined_class = move || {
        let mut parts = vec![btn_prefix.clone()];
        if !variant_class.is_empty() {
            parts.push(variant_class.clone());
        }
        if !size_class.is_empty() {
            parts.push(size_class.clone());
        }
        if disabled {
            parts.push(format!("{}-disabled", btn_prefix));
        }
        if let Some(ref custom) = class {
            parts.push(custom.clone());
        }
        parts.join(" ")
    };

    // 5. Render
    view! {
        <button
            class=combined_class
            disabled=disabled
            on:click=move |_| {
                if let Some(ref cb) = on_click {
                    cb.run(());
                }
            }
        >
            {children()}
        </button>
    }
}
```

### Type Definitions Pattern

```rust
// crates/components/src/button/types.rs

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
    Link,
}

impl ButtonVariant {
    /// Generate CSS class suffix
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Danger => "danger",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }

    /// Generate full CSS class
    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_suffix())
    }
}
```

### CSS Class Naming Convention

```
fx-{component}-{design_system}[-{variant}][-{size}][-{state}]

Examples:
- fx-btn-ant
- fx-btn-ant-primary
- fx-btn-ant-lg
- fx-btn-ant-disabled
- fx-btn-ant-primary fx-btn-ant-lg
- fx-input-material-filled
- fx-modal-cupertino-centered
```

### Component Categories

| Category | Components |
|----------|------------|
| `button` | Button, ButtonGroup |
| `input` | TextInput, TextArea, FormField |
| `selection` | Checkbox, Radio, Switch, Select |
| `notification` | Alert, Badge, Tag, Progress, Spinner |
| `layout` | Card, Modal, Drawer, Tabs, Collapse |
| `navigation` | Menu, Breadcrumb, Pagination, Steps |
| `data` | Avatar, Tooltip, Table, List |
| `visualization` | Empty, Result, Statistic, Timeline |
| `form_advanced` | Slider, DatePicker, Upload |

### Feature Flags

Each category can be enabled/disabled:

```toml
# Cargo.toml
[features]
default = ["button", "input", "notification"]
button = []
input = []
selection = []
notification = []
layout = []
# ...
```

---

## Design System Extension

### Adding a New Theme

1. **Create JSON preset** in `crates/styles/themes/presets/{system}/`:

```json
// themes/presets/ant/ocean.json
{
  "name": "ocean",
  "colorScheme": "dark",
  "colors": {
    "primary": "#0ea5e9",
    "primaryBg": "#0c4a6e",
    "primaryHover": "#38bdf8",
    "secondary": "#64748b",
    "accent": "#06b6d4",
    "base100": "#0f172a",
    "base200": "#1e293b",
    "base300": "#334155",
    "baseContent": "#f1f5f9"
  },
  "radius": {
    "sm": "4px",
    "md": "8px",
    "lg": "12px",
    "full": "9999px"
  },
  "spacing": {
    "xs": "4px",
    "sm": "8px",
    "md": "16px",
    "lg": "24px",
    "xl": "32px"
  }
}
```

2. **Rebuild** - The build script auto-detects new themes:

```bash
cargo build -p ember-fx-styles
```

3. **Update registry** (if not auto-detected) in `crates/styles/src/registry.rs`:

```rust
CompiledTheme {
    name: "ocean",
    display_name: "Ocean",
    design_system: "ant",
    is_dark: true,
    css: include_str!("compiled/ant/ocean.css"),
},
```

### Adding a New Design System

1. **Create preset directory**: `themes/presets/{new_system}/`

2. **Create base CSS directory**: `themes/base/{new_system}/`

3. **Add component CSS files** (button.css, input.css, etc.):

```css
/* themes/base/custom/button.css */
.fx-btn-custom {
    font-family: var(--fx-font-family);
    border-radius: var(--fx-radius-md);
    transition: var(--fx-transition-normal);
}

.fx-btn-custom-primary {
    background: var(--fx-color-primary);
    color: var(--fx-color-primary-content);
}

/* Size variants */
.fx-btn-custom-sm { padding: var(--fx-spacing-xs) var(--fx-spacing-sm); }
.fx-btn-custom-md { padding: var(--fx-spacing-sm) var(--fx-spacing-md); }
.fx-btn-custom-lg { padding: var(--fx-spacing-md) var(--fx-spacing-lg); }
```

4. **Add to DesignSystem enum** in `crates/common/src/design_system.rs`:

```rust
pub enum DesignSystem {
    // ... existing
    Custom,
}

impl DesignSystem {
    pub fn get_class_prefix(&self) -> &'static str {
        match self {
            // ... existing
            Self::Custom => "custom",
        }
    }
}
```

5. **Add feature flag** in workspace `Cargo.toml`:

```toml
[features]
custom = []
```

6. **Update ThemeRegistry** in `crates/styles/src/registry.rs`:

```rust
#[cfg(feature = "custom")]
static CUSTOM_THEMES: &[CompiledTheme] = &[
    CompiledTheme {
        name: "default",
        display_name: "Default",
        design_system: "custom",
        is_dark: false,
        css: include_str!("compiled/custom/default.css"),
    },
];
```

### Creating a New Component

1. **Create directory** in `crates/components/src/{category}/`:

```
crates/components/src/notification/
└── tooltip/
    ├── mod.rs
    ├── component.rs
    └── types.rs
```

2. **Define types** in `types.rs`:

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPlacement {
    pub fn as_suffix(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    pub fn class(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_suffix())
    }
}
```

3. **Implement component** in `component.rs`:

```rust
use leptos::prelude::*;
use crate::try_use_theme;
use super::types::TooltipPlacement;
use ember_fx_utils::a11y::generate_id;

#[component]
pub fn Tooltip(
    #[prop(into)]
    content: String,

    #[prop(optional, into)]
    placement: Option<TooltipPlacement>,

    #[prop(optional, into)]
    class: Option<String>,

    children: Children,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.get_class_prefix())
        .unwrap_or("ant");

    let placement = placement.unwrap_or_default();
    let tooltip_id = generate_id("tooltip");

    let tooltip_prefix = format!("fx-tooltip-{}", design_system);
    let placement_class = placement.class(&tooltip_prefix);

    let visible = RwSignal::new(false);

    view! {
        <div
            class=format!("{}-wrapper", tooltip_prefix)
            on:mouseenter=move |_| visible.set(true)
            on:mouseleave=move |_| visible.set(false)
            on:focus=move |_| visible.set(true)
            on:blur=move |_| visible.set(false)
        >
            <div aria-describedby=tooltip_id.clone()>
                {children()}
            </div>
            <Show when=move || visible.get()>
                <div
                    id=tooltip_id.clone()
                    role="tooltip"
                    class=format!("{} {}", tooltip_prefix, placement_class)
                >
                    {content.clone()}
                </div>
            </Show>
        </div>
    }
}
```

4. **Export in mod.rs**:

```rust
mod component;
mod types;

pub use component::Tooltip;
pub use types::TooltipPlacement;
```

5. **Add CSS** in `themes/base/{system}/tooltip.css`:

```css
.fx-tooltip-ant-wrapper {
    position: relative;
    display: inline-block;
}

.fx-tooltip-ant {
    position: absolute;
    padding: var(--fx-spacing-xs) var(--fx-spacing-sm);
    background: var(--fx-color-base-300);
    color: var(--fx-color-base-content);
    border-radius: var(--fx-radius-sm);
    font-size: 0.875rem;
    white-space: nowrap;
    z-index: 1000;
}

.fx-tooltip-ant-top {
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-bottom: var(--fx-spacing-xs);
}

/* ... other placements */
```

6. **Register in category mod.rs** and **lib.rs**.

---

## Troubleshooting Guide

### Theme Not Applying

**Symptoms**: Components appear unstyled or use wrong colors.

**Solutions**:

1. **Check ThemeProvider is wrapping your app**:
```rust
view! {
    <ThemeProvider>
        <App />
    </ThemeProvider>
}
```

2. **Verify data-theme attribute**:
```javascript
// In browser console
document.documentElement.getAttribute('data-theme')
// Should return "dark", "light", etc.
```

3. **Check CSS is injected**:
```javascript
// In browser console
document.getElementById('ember-theme-css')
// Should exist and contain CSS rules
```

4. **Inspect CSS variables**:
```javascript
getComputedStyle(document.documentElement)
    .getPropertyValue('--fx-color-primary')
// Should return a color value
```

### Components Missing Styles

**Symptoms**: Components render but look broken/unstyled.

**Solutions**:

1. **Check component CSS is injected**:
```javascript
document.getElementById('ember-theme-components')
```

2. **Verify feature flag is enabled**:
```toml
# Cargo.toml
[dependencies]
ember-fx-components = { version = "0.2", features = ["button", "input"] }
```

3. **Check class prefix matches design system**:
```rust
// Component should generate classes like:
// "fx-btn-ant" not "fx-btn-material"
```

### Theme Switch Not Working

**Symptoms**: `set_theme()` called but UI doesn't update.

**Solutions**:

1. **Check signal reactivity**:
```rust
// Use the context's set_theme method
let theme_ctx = use_theme();
theme_ctx.set_theme("light");  // Correct

// NOT: create a new signal
let theme = RwSignal::new("light");  // Wrong - not connected
```

2. **Verify Effect is running**:
```rust
// Add logging in ThemeProvider to debug
Effect::new(move |_| {
    log::info!("Theme changed to: {}", theme_signal.get());
    // ...
});
```

3. **Check localStorage**:
```javascript
localStorage.getItem('ember_theme')
// Should match current theme
```

### Platform Detection Issues

**Symptoms**: Wrong design system on mobile/desktop.

**Solutions**:

1. **Check user-agent detection**:
```javascript
navigator.userAgent
```

2. **Force design system**:
```rust
<ThemeProvider design_system=Some(DesignSystem::Material)>
```

3. **Debug platform detection**:
```rust
let platform = Platform::detect();
log::info!("Detected: {:?}", platform);
```

### Build Errors

**Symptoms**: Compilation fails with missing themes/CSS.

**Solutions**:

1. **Rebuild styles crate**:
```bash
cargo build -p ember-fx-styles
```

2. **Check JSON syntax** in theme presets

3. **Verify file paths** in `build.rs` match actual locations

4. **Check feature flags** are consistent across workspace

### CSS Variable Conflicts

**Symptoms**: Wrong colors, variables from other libraries.

**Solutions**:

1. **Use namespaced variables**: All ember-fx variables start with `--fx-`

2. **Check CSS specificity**: Theme selectors use `[data-theme="..."]`

3. **Inspect computed styles**:
```javascript
getComputedStyle(element).getPropertyValue('--fx-color-primary')
```

### Performance Issues

**Symptoms**: Slow theme switching, lag on page load.

**Solutions**:

1. **Use MinimalThemeProvider** if CSS is pre-compiled:
```rust
<MinimalThemeProvider>
```

2. **Check theme CSS size**:
```bash
ls -la crates/styles/src/compiled/ant/*.css
```

3. **Profile CSS injection**:
```rust
use ember_fx_tools::performance::measure_theme_switch;
let duration = measure_theme_switch("dark");
```

4. **Disable unused design systems**:
```toml
[features]
default = ["ant"]  # Only enable what you need
```

### SSR/Hydration Issues

**Symptoms**: Flash of unstyled content, hydration mismatches.

**Solutions**:

1. **Inject CSS server-side**: Include compiled CSS in `<head>`

2. **Set initial theme attribute**:
```html
<html data-theme="dark">
```

3. **Use `inject_base_css()` early**: Call before component rendering

4. **Match server/client theme**: Read from cookie or localStorage

### Accessibility Issues

**Symptoms**: Screen reader issues, keyboard navigation broken.

**Solutions**:

1. **Check ARIA attributes**: Use browser dev tools Accessibility panel

2. **Test keyboard navigation**: Tab through all interactive elements

3. **Verify focus trap** in modals:
```javascript
// Focus should stay within modal when Tab pressed
```

4. **Use a11y utilities**:
```rust
use ember_fx_utils::a11y::{FocusTrap, announce};
```

---

## Additional Resources

- [Component API Reference](./COMPONENTS.md)
- [Accessibility Guide](./ACCESSIBILITY.md)
- [Performance Guide](../tools/perf/PERFORMANCE.md)
- [Contributing Guide](./CONTRIBUTING.md)
