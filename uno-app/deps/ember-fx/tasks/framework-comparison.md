# Comprehensive UI Framework Comparison

## ember-fx vs thaw vs ui (Rust/UI)

> Research conducted: May 2026
> Frameworks analyzed for Leptos/WASM ecosystem

---

## Executive Summary

| Framework | Philosophy | Components | Lines of Code | Maturity |
|-----------|-----------|------------|---------------|----------|
| **ember-fx** | Design-system agnostic theming library | 60+ | ~15,000 Rust | Beta (0.1.0) |
| **thaw** | Fluent Design System component library | 57 | ~14,929 Rust | Beta (0.5.0) |
| **ui** | Copy-paste component registry | 50+ (~398 demos) | ~20,000+ Rust | Stable (0.3.22) |

---

## I. MODULARITY

### ember-fx

**Architecture Pattern**: Multi-crate library with feature-gated components

**Structure**:
```
ember-fx/
├── src/
│   ├── components/      # Individual components
│   │   ├── button/
│   │   ├── input/
│   │   └── ...
│   ├── themes/          # Theming system
│   │   ├── provider.rs
│   │   ├── context.rs
│   │   ├── design_system.rs
│   │   ├── registry.rs
│   │   └── loader.rs
│   └── compiled/        # Pre-compiled CSS
└── themes/base/         # CSS source files
```

**Modularity Score: 8/10**

| Strength | Weakness |
|----------|----------|
| Feature flags per component category | Single crate (not workspace) |
| Design system abstraction (Ant, Material, etc.) | CSS compilation coupled to build.rs |
| Theming completely decoupled from components | No runtime component tree-shaking |
| `include_str!` embeds only enabled features | |

**Feature Flags**:
```toml
# Component granularity
button = ["core"]
panel = ["core"]
input = ["core"]
notification = ["core"]
layout = ["core", "template"]

# Design system granularity
ant = []
material = []
cupertino = []
daisyui = []
```

---

### thaw

**Architecture Pattern**: Workspace with specialized crates

**Structure**:
```
thaw/
├── thaw/                # Main component library
├── thaw_components/     # Shared internal components
├── thaw_macro/          # Procedural macros (WriteCSSVars)
├── thaw_utils/          # DOM utilities, class management
├── thaw_mobile/         # Mobile-specific components
└── demo/                # Documentation app
```

**Modularity Score: 9/10**

| Strength | Weakness |
|----------|----------|
| 6-crate workspace with clear boundaries | All 57 components in single thaw crate |
| Macro crate isolated from runtime | No per-component feature flags |
| Utils completely decoupled | Mobile components separate crate |
| Demo separate from library | |

**Workspace Dependencies**:
```toml
[workspace]
members = ["thaw", "thaw_components", "thaw_macro", "thaw_utils", "demo", "demo_markdown"]
```

**Component Internal Modularity**:
```
button/
├── mod.rs              # Exports
├── button.rs           # Implementation
├── types.rs            # Props, enums
└── button.css          # Scoped styles
```

---

### ui (Rust/UI)

**Architecture Pattern**: Workspace with copy-paste registry

**Structure**:
```
ui/
├── crates/
│   ├── leptos_ui/       # Core macros (clx!, void!, variants!)
│   ├── tw_merge/        # Tailwind class merging
│   ├── icons/           # SVG icon system
│   └── autoform/        # Form generation
├── app_crates/
│   ├── registry/        # Component library (92 files)
│   ├── app_domain/      # Domain logic
│   ├── app_config/      # Configuration
│   ├── app_routes/      # Routing
│   └── app_components/  # Shared components
├── app/                 # Main showcase application
└── server/              # Leptos Axum backend
```

**Modularity Score: 10/10**

| Strength | Weakness |
|----------|----------|
| 8+ workspace members with distinct purposes | Copy-paste means no versioned updates |
| Core utilities separate from UI components | Components not publishable as crate |
| Icons system multi-framework (Leptos, Dioxus) | Heavy app_crates coupling |
| tw_merge completely standalone | |

**Key Separation**:
- `leptos_ui` (macros) has zero dependencies on `registry`
- `tw_merge` is framework-agnostic (works with any Tailwind project)
- `icons` supports multiple Rust web frameworks

---

### Modularity Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **Crate Architecture** | Single crate | 6-crate workspace | 8+ crate workspace |
| **Feature Granularity** | Component categories | Rendering mode only | None (copy-paste) |
| **Theme/Component Separation** | Excellent | Excellent | Excellent |
| **Utility Isolation** | Embedded | Separate crate | Separate crates |
| **Build Decoupling** | CSS in build.rs | Runtime injection | Tailwind CLI external |

**Winner: ui** - Most granular separation with standalone utility crates

---

## II. PERFORMANCE

### ember-fx

**Build-Time Optimizations**:
```toml
[build-dependencies]
lightningcss = "1.0.0-alpha.71"  # CSS minification
walkdir = "2.5"

[profile.wasm-release]
opt-level = 'z'      # Size optimization
lto = true           # Link-time optimization
codegen-units = 1    # Maximum optimization
panic = "abort"      # Smaller binary
```

**Runtime Optimizations**:
- CSS embedded via `include_str!()` - zero network requests
- Theme CSS injected once per session
- `data-theme` attribute switching (instant, no re-injection)
- Component CSS deduplicated by style ID

**Performance Characteristics**:
| Metric | Value |
|--------|-------|
| CSS Injection | Once on mount + theme change |
| Theme Switching | Attribute-based (instant) |
| Component Rendering | Standard Leptos reactivity |
| Bundle Strategy | All-or-nothing per feature |

**Performance Score: 7/10**

---

### thaw

**Signal Optimizations**:
```rust
// Memoization prevents unnecessary re-renders
let only_icon = Memo::new(move |_| icon.with(|i| i.is_some()) && none_children);
let btn_disabled = Memo::new(move |_| disabled.get() || disabled_focusable.get());

// Untracked reads in event handlers
let on_click = move |e| {
    if btn_disabled.get_untracked() {  // No dependency tracking
        return;
    }
};
```

**CSS Optimizations**:
```rust
// Static CSS mounted once
mount_style("button", include_str!("./button.css"));

// Dynamic styles via RenderEffect
mount_dynamic_style(id, move || {
    theme.with(|t| t.write_css_vars(&mut css))
});
```

**Performance Characteristics**:
| Metric | Value |
|--------|-------|
| CSS Injection | Static once + dynamic per theme |
| Signal Tracking | Fine-grained with Memo/untracked |
| Component Rendering | Optimized with StoredValue |
| Class Management | HashSet-based deduplication |

**Unique Optimizations**:
- `StoredValue` for immutable config (zero-cost access)
- `RenderEffect` for minimal DOM updates
- GPU-accelerated CSS transitions via `leptos_transition_group`
- Scroll parent caching to avoid DOM traversals

**Performance Score: 9/10**

---

### ui (Rust/UI)

**Build-Time Optimizations**:
```toml
[profile.dev]
incremental = true
codegen-units = 256      # Parallel compilation (M1 optimized)
overflow-checks = false

[profile.dev.build-override]
opt-level = 3           # Dependencies fully optimized

[profile.server-dev]
codegen-backend = "cranelift"  # Faster dev compilation

[profile.release]
lto = true
opt-level = 'z'
codegen-units = 1
```

**Runtime Optimizations**:
```rust
// CSS hash-based cache busting (build.rs)
let css_hash = format!("{:x}", hasher.finish());
println!("cargo:rustc-env=CSS_HASH={css_hash}");
```

**tw_merge Performance**:
- Nom parser combinator for efficient class parsing
- Conflict resolution without string concatenation
- Zero allocation after initial parse

**Performance Characteristics**:
| Metric | Value |
|--------|-------|
| CSS Strategy | Tailwind JIT + external CLI |
| Build Speed | Cranelift for dev, LLVM for release |
| Class Merging | Parser-based, cached |
| Bundle Size | Tree-shaken by Tailwind v4 |

**Performance Score: 8/10**

---

### Performance Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **CSS Strategy** | Embedded + injected | Embedded + dynamic | External Tailwind |
| **Signal Optimization** | Basic Leptos | Memo + untracked | Basic Leptos |
| **Build Optimization** | LightningCSS | Standard | Cranelift + aggressive |
| **Theme Switching** | Attribute-based | CSS var injection | CSS variables |
| **Memory Efficiency** | Good | Excellent | Good |

**Winner: thaw** - Most sophisticated signal optimization and DOM efficiency

---

## III. SECURITY

### ember-fx

**Security Patterns**:

| Pattern | Implementation |
|---------|----------------|
| DOM Manipulation | `web-sys` safe bindings only |
| CSS Injection | `set_text_content()` (XSS-safe) |
| Class Names | Static strings, no user input |
| Event Handlers | Typed callbacks only |

**Potential Concerns**:
- No explicit input sanitization layer
- Theme names from localStorage (trusted source)
- No CSP considerations documented

**Security Score: 7/10**

---

### thaw

**Security Patterns**:

| Pattern | Implementation |
|---------|----------------|
| DOM Manipulation | `web-sys` + Leptos abstractions |
| Content Injection | `set_text_content()` only |
| Class Management | `HashSet<Oco<'static, str>>` |
| ARIA Support | Built-in accessibility attributes |
| SSR Safety | Framework `<Style>` component |

```rust
// ClassList prevents injection
pub struct ClassList {
    value: RwSignal<HashSet<Oco<'static, str>>>,
}

// Event handlers are typed
#[prop(optional, into)] on_click: Option<BoxOneCallback<ev::MouseEvent>>
```

**SSR XSS Prevention**:
```rust
if #[cfg(feature = "ssr")] {
    use leptos_meta::Style;  // Framework-managed
} else {
    // Direct DOM only in CSR
}
```

**Security Score: 8/10**

---

### ui (Rust/UI)

**Security Patterns**:

| Pattern | Implementation |
|---------|----------------|
| No Unsafe | Zero `unsafe` blocks in core crates |
| Clippy Enforcement | `deny` on `unwrap`, `expect`, `panic` |
| Input Validation | `validator` crate integration |
| Password Fields | Enforced security attributes |
| Web-sys | Selective feature imports |

**Strict Clippy Rules** (workspace-wide):
```toml
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"
await_holding_lock = "deny"
undocumented_unsafe_blocks = "deny"
missing_safety_doc = "deny"
```

**Login Block Security** (enforced via tests):
```rust
assert!(source.contains("autocomplete=\"current-password\""));
assert!(source.contains("minlength=8"));
```

**Security Score: 10/10**

---

### Security Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **Unsafe Code** | None visible | None visible | Explicitly denied |
| **Input Validation** | None | Basic types | `validator` crate |
| **XSS Prevention** | text_content | text_content + SSR | text_content |
| **Clippy Enforcement** | Default | Default | Strict deny rules |
| **ARIA/A11y** | Basic | Comprehensive | Good |
| **Password Security** | N/A | N/A | Enforced in blocks |

**Winner: ui** - Most rigorous compile-time security enforcement

---

## IV. EXTENDABILITY & SEPARATION OF CONCERNS

### ember-fx

**Separation Architecture**:
```
┌─────────────────────────────────────────────┐
│           Application Layer                  │
│  (antd-01 example: pages, routing)          │
├─────────────────────────────────────────────┤
│           Theme Override Layer               │
│  (antd-pro.css, styles.css)                 │
├─────────────────────────────────────────────┤
│           Component Layer                    │
│  (Button, Alert, StatCard, etc.)            │
├─────────────────────────────────────────────┤
│           Theme System Layer                 │
│  (ThemeProvider, ThemeContext, Registry)    │
├─────────────────────────────────────────────┤
│           Design System Layer                │
│  (Ant, Material, Cupertino, DaisyUI)        │
└─────────────────────────────────────────────┘
```

**Extendability Mechanisms**:

1. **Custom Design Systems**:
```rust
pub enum DesignSystem {
    Ant,
    Material,
    Cupertino,
    DaisyUI,
    Prime,
    Flutter,
    Auto,  // Platform detection
}
```

2. **Theme Customization**:
- Add new themes to `src/compiled/{design_system}/`
- Register in `ThemeRegistry`
- CSS variables automatically cascade

3. **Component Overrides**:
- Local CSS can override any `fx-*` class
- `!important` for complete override control

**Score: 8/10**

---

### thaw

**Separation Architecture**:
```
┌─────────────────────────────────────────────┐
│           Application Layer                  │
│  (ConfigProvider wraps app)                 │
├─────────────────────────────────────────────┤
│           Component Layer                    │
│  (57 components with scoped CSS)            │
├─────────────────────────────────────────────┤
│           Theme Layer                        │
│  (Theme struct with CommonTheme + ColorTheme)│
├─────────────────────────────────────────────┤
│           Infrastructure Layer               │
│  (thaw_utils, thaw_macro, thaw_components)  │
└─────────────────────────────────────────────┘
```

**Extendability Mechanisms**:

1. **Custom Themes**:
```rust
impl Theme {
    pub fn custom_light(brand_colors: &HashMap<i32, &str>) -> Self
    pub fn custom_dark(brand_colors: &HashMap<i32, &str>) -> Self
}
```

2. **Component References**:
```rust
#[prop(optional)] comp_ref: ComponentRef<ButtonRef>
// Allows imperative control of underlying DOM
```

3. **Locale Extension**:
```rust
pub trait LocaleExt {
    fn locale() -> Locale;
    fn today() -> &'static str;
    fn months() -> &'static [&'static str];
}
```

4. **Callback Abstraction**:
- `BoxOneCallback`, `ArcOneCallback` for custom event handling
- Prevents callback cloning overhead

**Score: 9/10**

---

### ui (Rust/UI)

**Separation Architecture**:
```
┌─────────────────────────────────────────────┐
│           Application Layer                  │
│  (app/, server/)                            │
├─────────────────────────────────────────────┤
│           Domain Layer                       │
│  (app_crates/app_domain, app_config)        │
├─────────────────────────────────────────────┤
│           Component Layer                    │
│  (app_crates/registry - copy-paste source)  │
├─────────────────────────────────────────────┤
│           Macro Layer                        │
│  (crates/leptos_ui - variants!, clx!)       │
├─────────────────────────────────────────────┤
│           Utility Layer                      │
│  (crates/tw_merge, crates/icons)            │
└─────────────────────────────────────────────┘
```

**Extendability Mechanisms**:

1. **variants! Macro**:
```rust
variants! {
    CustomButton {
        base: "my-base-classes",
        variants: {
            variant: { Primary: "...", Custom: "..." },
            size: { Sm: "...", Custom: "..." }
        },
        component: { element: button }
    }
}
```

2. **Copy-Paste Philosophy**:
- Components are yours to modify
- No upstream dependency lock-in
- Full source code control

3. **tw_merge Extensions**:
- Custom Tailwind class groups
- Conflict resolution rules

4. **Icon System**:
- Multi-framework support
- Animated icon variants
- Custom icon registration

**Score: 10/10**

---

### Extendability Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **Theme Customization** | Design system enum | Brand color maps | CSS variables |
| **Component Customization** | CSS override | Props + refs | Copy & modify |
| **New Components** | Add to src/components | Add to thaw/src | Add to registry |
| **Macro Extensibility** | None | WriteCSSVars | variants!, clx! |
| **i18n Extension** | None | LocaleExt trait | N/A |

**Winner: ui** - Copy-paste gives ultimate flexibility, macros enable rapid creation

---

## V. FLEXIBILITY

### ember-fx

**Flexibility Features**:

| Feature | Support |
|---------|---------|
| Multiple Design Systems | 6 (Ant, Material, Cupertino, DaisyUI, Prime, Flutter) |
| Theme Variants | 7 per design system |
| Rendering Modes | CSR, SSR, Hydrate |
| Platform Detection | iOS, Android, macOS, Windows, Web |
| CSS Override | Full cascade control |

**Design System Selection**:
```rust
<ThemeProvider
    initial_theme="dark"
    design_system=DesignSystem::Ant  // or Material, Cupertino, etc.
>
```

**Platform-Adaptive UI**:
```rust
impl Platform {
    pub fn default_design_system(&self) -> DesignSystem {
        match self {
            Platform::IOS | Platform::MacOS => DesignSystem::Cupertino,
            Platform::Android => DesignSystem::Material,
            _ => DesignSystem::Ant,
        }
    }
}
```

**Flexibility Score: 9/10**

---

### thaw

**Flexibility Features**:

| Feature | Support |
|---------|---------|
| Design Systems | Fluent Design only |
| Theme Variants | Light, Dark, Custom |
| Rendering Modes | CSR, SSR, Hydrate, Islands |
| Direction | LTR, RTL |
| Mobile Support | Dedicated thaw_mobile crate |

**Configuration Injection**:
```rust
<ConfigProvider
    theme=RwSignal::new(Theme::dark())
    dir=RwSignal::new(ConfigDirection::Rtl)
    locale=RwSignal::new(LocaleConfig::default())
>
```

**Component Variants**:
```rust
pub enum ButtonAppearance {
    Primary,
    Secondary,
    Subtle,
    Outline,
    Transparent,
}

pub enum ButtonSize { Small, Medium, Large }
pub enum ButtonShape { Rounded, Circular, Square }
```

**Flexibility Score: 7/10** (single design system limits flexibility)

---

### ui (Rust/UI)

**Flexibility Features**:

| Feature | Support |
|---------|---------|
| Design Systems | Tailwind-based (any design) |
| Theme Variants | CSS variables (unlimited) |
| Rendering Modes | CSR, SSR, Hydrate |
| Animation Library | 40+ custom animations |
| Icon Sources | 10,000+ icons, animated variants |

**CSS Variable System**:
```css
:root {
    --primary: oklch(0.205 0 0);
    --destructive: oklch(0.577 0.245 27.325);
    --success: oklch(0.65 0.16 145);
    /* Override anything */
}

.dark {
    --primary: oklch(0.985 0 0);
    /* Complete theme in CSS */
}
```

**Component Generation Flexibility**:
```rust
variants! {
    // Generate any component with any variants
    AnyComponent {
        base: "...",
        variants: { /* unlimited */ },
        component: { element: any_html_element }
    }
}
```

**Flexibility Score: 10/10**

---

### Flexibility Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **Design System Lock-in** | Low (6 options) | High (Fluent only) | None |
| **Theme Creation** | Add CSS files | Rust struct | CSS variables |
| **Component Variants** | CSS class-based | Rust enums | Macro-generated |
| **Platform Adaptation** | Built-in detection | Manual | N/A |
| **Animation Support** | Basic transitions | CSS + leptos_transition | 40+ keyframes |

**Winner: ui** - Most flexible due to CSS-first approach and macro generation

---

## VI. ARCHITECTURE & DESIGN QUALITY

### ember-fx

**Architectural Patterns**:

1. **Context Provider Pattern**:
```rust
<ThemeProvider> → provide_context(ThemeContext)
    ↓
Components use use_theme() → access context
```

2. **Registry Pattern**:
```rust
ThemeRegistry::load_theme_css(design_system, theme_name)
// Centralized theme management
```

3. **CSS Variable Architecture**:
```
[data-theme=dark] → Theme variables
         ↓
:root → Fallback variables
         ↓
.fx-btn-ant → Component consumption
```

**Design Quality Indicators**:
- Clear separation of theming and components
- Multiple design system support (rare in Leptos ecosystem)
- Build-time CSS optimization
- Documented CSS class conventions

**Architecture Score: 8/10**

---

### thaw

**Architectural Patterns**:

1. **Proc Macro Code Generation**:
```rust
#[derive(WriteCSSVars)]
pub struct ColorTheme {
    color_neutral_background_1: String,
    // → generates --colorNeutralBackground1
}
```

2. **Signal Composition**:
```rust
let theme = RwSignal::new(Theme::dark());
let ctx = ConfigInjection { theme, dir, locale, id };
provide_context(ctx);
```

3. **Component Reference Pattern**:
```rust
pub struct ButtonRef {
    element: NodeRef<html::Button>,
}
// Exposes imperative API when needed
```

4. **Callback Type Hierarchy**:
```
BoxCallback<()>        → No arguments
BoxOneCallback<T>      → Single argument
ArcOneCallback<T>      → Thread-safe variant
```

**Design Quality Indicators**:
- Professional-grade Fluent Design implementation
- Sophisticated signal optimization
- Complete accessibility support
- Well-documented component APIs

**Architecture Score: 9/10**

---

### ui (Rust/UI)

**Architectural Patterns**:

1. **Macro-Driven Component Generation**:
```rust
variants! { Button { ... } }
// Generates: ButtonClass, ButtonVariant, ButtonSize, Button fn
```

2. **Class Merging Pipeline**:
```rust
clx! {Card, div, "base", "override"}
    ↓
tw_merge!("base", "override")
    ↓
Conflict-resolved class string
```

3. **Copy-Paste Registry**:
```
registry/src/ui/button.rs  → Source of truth
    ↓ (copy)
your_project/src/components/button.rs  → Your code
```

4. **Block Composition**:
```rust
pub struct BlockEntry {
    title: &'static str,
    path: &'static str,
    view: fn() -> AnyView,
}
// Pre-built page sections as composable units
```

**Design Quality Indicators**:
- Modern Tailwind v4 with OKLch colors
- Type-safe variant generation
- Comprehensive test coverage (Playwright E2E)
- Clean workspace organization

**Architecture Score: 9/10**

---

### Architecture Comparison Summary

| Aspect | ember-fx | thaw | ui |
|--------|----------|------|-----|
| **Primary Pattern** | Context + Registry | Proc Macro + Signals | Declarative Macro + Copy |
| **Code Generation** | None | WriteCSSVars | variants!, clx!, void! |
| **State Management** | Leptos signals | Advanced signal composition | Basic Leptos signals |
| **CSS Architecture** | 3-layer variables | CSS vars via Rust structs | Tailwind + CSS vars |
| **Test Coverage** | Examples only | Demo app | E2E Playwright |
| **Documentation** | Inline + CLAUDE.md | Dedicated demo crate | Markdown + demo app |

---

## FINAL SCORES & RECOMMENDATIONS

### Overall Scores

| Dimension | ember-fx | thaw | ui |
|-----------|:--------:|:----:|:---:|
| **Modularity** | 8 | 9 | **10** |
| **Performance** | 7 | **9** | 8 |
| **Security** | 7 | 8 | **10** |
| **Extendability** | 8 | 9 | **10** |
| **Flexibility** | 9 | 7 | **10** |
| **Architecture** | 8 | **9** | 9 |
| **TOTAL** | **47/60** | **51/60** | **57/60** |

---

### Recommendation Matrix

| Use Case | Recommended Framework | Rationale |
|----------|----------------------|-----------|
| **Enterprise Fluent UI** | thaw | Native Fluent Design, comprehensive a11y |
| **Multi-platform adaptive UI** | ember-fx | 6 design systems, platform detection |
| **Maximum customization** | ui | Copy-paste ownership, macro flexibility |
| **Rapid prototyping** | ui | 398 demo variations, 31 pre-built blocks |
| **Performance-critical** | thaw | Best signal optimization, memoization |
| **Security-sensitive** | ui | Strictest compile-time enforcement |
| **Cross-design-system** | ember-fx | Only framework supporting multiple systems |
| **Mobile-first** | thaw | Dedicated thaw_mobile crate |

---

### Key Differentiators

**ember-fx**:
- **Unique**: Multi-design-system support (Ant, Material, Cupertino, etc.)
- **Best for**: Teams needing to switch design systems or target multiple platforms
- **Trade-off**: Less optimized signals, single-crate architecture

**thaw**:
- **Unique**: Professional Fluent Design with WriteCSSVars macro
- **Best for**: Enterprise apps requiring Microsoft Fluent aesthetic
- **Trade-off**: Locked to Fluent design language

**ui**:
- **Unique**: Copy-paste philosophy with macro-generated components
- **Best for**: Teams wanting full ownership and customization freedom
- **Trade-off**: No version-controlled upstream updates

---

## Appendix: File Reference

### ember-fx Key Files
| File | Purpose |
|------|---------|
| `src/themes/provider.rs` | ThemeProvider component |
| `src/themes/context.rs` | ThemeContext reactive state |
| `src/themes/registry.rs` | Compiled theme storage |
| `src/compiled/ant/dark.css` | Example compiled theme |
| `themes/base/ant/button.css` | Component CSS source |

### thaw Key Files
| File | Purpose |
|------|---------|
| `thaw/src/theme/mod.rs` | Theme struct definition |
| `thaw/src/theme/color.rs` | 130+ color definitions |
| `thaw_macro/src/lib.rs` | WriteCSSVars proc macro |
| `thaw_utils/src/class_list.rs` | Class management |
| `thaw/src/config_provider/mod.rs` | Context injection |

### ui Key Files
| File | Purpose |
|------|---------|
| `crates/leptos_ui/src/variants.rs` | variants! macro |
| `crates/tw_merge/tw_merge/src/lib.rs` | Class merging |
| `style/tailwind.css` | OKLch theme colors |
| `app_crates/registry/src/` | 92 component files |
| `app/src/__registry__/static_md_registry.rs` | Component registry |
