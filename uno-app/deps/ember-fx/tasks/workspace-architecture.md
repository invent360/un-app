# ember-fx Workspace Architecture Proposal

> Multi-crate structure for improved modularity, parallel compilation, and reusability

---

## Proposed Structure

```text
ember-fx/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── core/                     # Foundation: context, signals, reactivity
│   ├── common/                   # Shared types, traits, constants
│   ├── utils/                    # DOM utilities, helpers
│   ├── macros/                   # Procedural & declarative macros
│   ├── icons/                    # SVG icon system
│   ├── styles/                   # CSS compilation, theming engine
│   ├── systems/                  # Design systems (Ant, Material, etc.)
│   ├── components/               # UI component library
│   ├── tools/                    # Dev tools, debugging, profiling
│   └── mobile/                   # Mobile-specific components
├── demos/
│   ├── antd-01/                  # Ant Design demo
│   └── showcase/                 # Component showcase app
└── docs/
```

---

## Crate Responsibilities

### 1. `ember-fx-core`

**Purpose**: Foundation layer - context providers, reactive primitives, platform detection

**Current Files to Move**:

```text
src/themes/context.rs      → core/src/context.rs
src/themes/provider.rs     → core/src/provider.rs
src/themes/platform.rs     → core/src/platform.rs
```

**Public API**:

```rust
// core/src/lib.rs
pub use context::{ThemeContext, use_theme, try_use_theme};
pub use provider::{ThemeProvider, MinimalThemeProvider};
pub use platform::Platform;
pub use signals::{Model, MaybeSignal};
```

**Dependencies**: `leptos`, `web-sys`, `gloo-storage`

**Dependents**: All other crates

---

### 2. `ember-fx-common`

**Purpose**: Shared types, traits, enums, constants used across crates

**Contents**:

```rust
// common/src/lib.rs

// Design system enum
pub enum DesignSystem {
    Ant, Material, Cupertino, DaisyUI, Prime, Flutter, Auto
}

// Common component props
pub trait ComponentProps {
    fn class(&self) -> Option<&str>;
    fn disabled(&self) -> bool;
}

// Size variants (shared across components)
pub enum Size { Xs, Sm, Md, Lg, Xl }

// Direction
pub enum Direction { Ltr, Rtl }

// Theme mode
pub enum ThemeMode { Light, Dark, System }

// Constants
pub const THEME_STORAGE_KEY: &str = "ember_theme";
pub const STYLE_ID_PREFIX: &str = "ember-";
```

**Dependencies**: `serde` (for serialization)

**Dependents**: `core`, `components`, `systems`, `styles`

---

### 3. `ember-fx-utils`

**Purpose**: DOM utilities, class management, helper functions

**Current Files to Move**:

```text
src/themes/loader.rs       → utils/src/css_loader.rs
(new)                      → utils/src/class_list.rs
(new)                      → utils/src/dom.rs
(new)                      → utils/src/scroll.rs
```

**Public API**:

```rust
// utils/src/lib.rs
pub use css_loader::{inject_css, remove_css, apply_theme_attribute};
pub use class_list::ClassList;
pub use dom::{document, window, get_element_by_id};
pub use scroll::{get_scroll_parent, scroll_into_view};
```

**Dependencies**: `web-sys`, `wasm-bindgen`

**Dependents**: `core`, `components`, `styles`

---

### 4. `ember-fx-macros`

**Purpose**: Procedural and declarative macros for code generation

**Contents**:

```rust
// macros/src/lib.rs

// Proc macro: Generate CSS variable writers
#[proc_macro_derive(WriteCSSVars)]
pub fn write_css_vars(input: TokenStream) -> TokenStream { ... }

// Proc macro: Component boilerplate
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

// Declarative: Class composition with tw_merge
#[macro_export]
macro_rules! clx {
    ($($class:expr),* $(,)?) => { ... };
}

// Declarative: Component variants
#[macro_export]
macro_rules! variants {
    ($name:ident { $($body:tt)* }) => { ... };
}
```

**Dependencies**: `syn`, `quote`, `proc-macro2`

**Dependents**: `styles`, `components`

---

### 5. `ember-fx-icons`

**Purpose**: SVG icon system with multiple icon sets

**Structure**:

```text
icons/
├── src/
│   ├── lib.rs
│   ├── icon.rs           # Icon component
│   ├── registry.rs       # Icon lookup
│   └── sets/
│       ├── heroicons.rs
│       ├── lucide.rs
│       ├── phosphor.rs
│       └── custom.rs
└── assets/               # SVG source files
```

**Public API**:

```rust
// icons/src/lib.rs
pub use icon::Icon;
pub use registry::{IconSet, get_icon};

// Usage
view! { <Icon icon=icons::HeroCheck class="w-4 h-4" /> }
```

**Dependencies**: `leptos`

**Dependents**: `components`

**Feature Flags**:

```toml
[features]
heroicons = []
lucide = []
phosphor = []
all-icons = ["heroicons", "lucide", "phosphor"]
```

---

### 6. `ember-fx-styles`

**Purpose**: CSS compilation, theme registry, style injection

**Current Files to Move**:

```text
src/themes/registry.rs     → styles/src/registry.rs
src/compiled/              → styles/src/compiled/
themes/base/               → styles/themes/
build.rs                   → styles/build.rs
```

**Structure**:

```text
styles/
├── src/
│   ├── lib.rs
│   ├── registry.rs        # Theme lookup
│   ├── compiler.rs        # CSS compilation
│   ├── variables.rs       # CSS variable generation
│   └── compiled/          # Pre-compiled CSS (generated)
│       ├── ant/
│       ├── material/
│       └── ...
├── themes/                # CSS source files
│   ├── base/
│   │   ├── ant/
│   │   ├── material/
│   │   └── ...
│   └── variants/
└── build.rs               # LightningCSS compilation
```

**Public API**:

```rust
// styles/src/lib.rs
pub use registry::{ThemeRegistry, CompiledTheme};
pub use variables::CSSVariables;

// Get theme CSS
let css = ThemeRegistry::load_theme_css("ant", "dark");
```

**Build Dependencies**: `lightningcss`, `walkdir`

**Dependents**: `core`, `systems`

---

### 7. `ember-fx-systems`

**Purpose**: Design system implementations (Ant, Material, Cupertino, etc.)

**Structure**:

```text
systems/
├── src/
│   ├── lib.rs
│   ├── ant/
│   │   ├── mod.rs
│   │   ├── theme.rs       # Ant color palette
│   │   ├── tokens.rs      # Design tokens
│   │   └── overrides.rs   # Component style overrides
│   ├── material/
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   └── tokens.rs
│   ├── cupertino/
│   └── ...
└── Cargo.toml
```

**Public API**:

```rust
// systems/src/lib.rs
pub use ant::AntDesignSystem;
pub use material::MaterialDesignSystem;
pub use cupertino::CupertinoDesignSystem;

pub trait DesignSystemImpl {
    fn name(&self) -> &'static str;
    fn class_prefix(&self) -> &'static str;
    fn default_theme(&self) -> &'static str;
    fn tokens(&self) -> &DesignTokens;
}
```

**Dependencies**: `ember-fx-common`, `ember-fx-styles`

**Dependents**: `components`

**Feature Flags**:

```toml
[features]
ant = []
material = []
cupertino = []
daisyui = []
prime = []
flutter = []
all-systems = ["ant", "material", "cupertino", "daisyui", "prime", "flutter"]
```

---

### 8. `ember-fx-components`

**Purpose**: UI component library

**Structure**:

```text
components/
├── src/
│   ├── lib.rs
│   ├── button/
│   │   ├── mod.rs
│   │   ├── button.rs
│   │   ├── button_group.rs
│   │   └── types.rs
│   ├── input/
│   ├── select/
│   ├── modal/
│   ├── card/
│   ├── table/
│   ├── notification/
│   │   ├── alert.rs
│   │   ├── badge.rs
│   │   ├── progress.rs
│   │   └── spinner.rs
│   ├── layout/
│   │   ├── flex.rs
│   │   ├── grid.rs
│   │   └── divider.rs
│   └── navigation/
│       ├── menu.rs
│       ├── breadcrumb.rs
│       └── tabs.rs
└── Cargo.toml
```

**Dependencies**:

- `ember-fx-core`
- `ember-fx-common`
- `ember-fx-utils`
- `ember-fx-macros`
- `ember-fx-icons`
- `ember-fx-systems`

**Feature Flags**:

```toml
[features]
# Component categories
button = []
input = []
select = []
modal = []
notification = []
layout = []
navigation = []
data = []
visualization = []

# Bundles
basic = ["button", "input", "notification"]
full = ["basic", "select", "modal", "layout", "navigation", "data", "visualization"]
```

---

### 9. `ember-fx-tools`

**Purpose**: Developer tools, debugging, profiling utilities

**Contents**:

```rust
// tools/src/lib.rs

// Theme debugger component
pub fn ThemeDebugger() -> impl IntoView { ... }

// Performance profiler
pub fn PerformanceOverlay() -> impl IntoView { ... }

// Component inspector
pub fn ComponentInspector() -> impl IntoView { ... }

// CSS variable viewer
pub fn CSSVariableViewer() -> impl IntoView { ... }

// Accessibility checker
pub fn A11yChecker() -> impl IntoView { ... }
```

**Dependencies**: `ember-fx-core`, `ember-fx-components`

**Feature Flags**:

```toml
[features]
default = []
dev-tools = []  # Only include in dev builds
```

---

### 10. `ember-fx-mobile`

**Purpose**: Mobile-specific components and utilities

**Contents**:

```rust
// mobile/src/lib.rs

// Mobile navigation
pub fn MobileNavBar() -> impl IntoView { ... }
pub fn MobileTabBar() -> impl IntoView { ... }

// Touch gestures
pub fn SwipeContainer() -> impl IntoView { ... }
pub fn PullToRefresh() -> impl IntoView { ... }

// Mobile-optimized components
pub fn MobileDrawer() -> impl IntoView { ... }
pub fn MobileSheet() -> impl IntoView { ... }

// Safe area handling
pub fn SafeAreaProvider() -> impl IntoView { ... }
```

**Dependencies**: `ember-fx-core`, `ember-fx-components`

---

### 11. `demos/` (workspace members)

**Purpose**: Demo applications and component showcases

**Structure**:

```text
demos/
├── antd-01/               # Ant Design Pro layout demo
│   ├── src/
│   ├── Cargo.toml
│   └── index.html
└── showcase/              # Component showcase & documentation
    ├── src/
    │   ├── main.rs
    │   ├── app.rs
    │   ├── pages/
    │   │   ├── home.rs
    │   │   ├── components/
    │   │   └── themes/
    │   └── registry.rs    # Demo component registry
    ├── Cargo.toml
    └── index.html
```

---

## Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/common",
    "crates/utils",
    "crates/macros",
    "crates/icons",
    "crates/styles",
    "crates/systems",
    "crates/components",
    "crates/tools",
    "crates/mobile",
    "demos/antd-01",
    "demos/showcase",
]

[workspace.package]
version = "0.2.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/polkanight/ember-fx"

[workspace.dependencies]
# Internal crates
ember-fx-core = { path = "crates/core" }
ember-fx-common = { path = "crates/common" }
ember-fx-utils = { path = "crates/utils" }
ember-fx-macros = { path = "crates/macros" }
ember-fx-icons = { path = "crates/icons" }
ember-fx-styles = { path = "crates/styles" }
ember-fx-systems = { path = "crates/systems" }
ember-fx-components = { path = "crates/components" }
ember-fx-tools = { path = "crates/tools" }
ember-fx-mobile = { path = "crates/mobile" }

# External dependencies
leptos = "0.8"
web-sys = "0.3"
wasm-bindgen = "0.2"
gloo-storage = "0.3"
serde = { version = "1.0", features = ["derive"] }

# Build dependencies
lightningcss = "1.0.0-alpha.71"
walkdir = "2.5"

# Macro dependencies
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"

[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"
undocumented_unsafe_blocks = "deny"
```

---

## Dependency Graph

```text
                    ┌─────────────┐
                    │   common    │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
        ┌─────────┐  ┌─────────┐  ┌─────────┐
        │  utils  │  │  macros │  │  styles │
        └────┬────┘  └────┬────┘  └────┬────┘
             │            │            │
             └────────────┼────────────┘
                          │
                          ▼
                    ┌─────────┐
                    │   core  │
                    └────┬────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
         ▼               ▼               ▼
   ┌─────────┐     ┌─────────┐     ┌─────────┐
   │  icons  │     │ systems │     │  tools  │
   └────┬────┘     └────┬────┘     └─────────┘
        │               │
        └───────┬───────┘
                │
                ▼
          ┌───────────┐
          │components │
          └─────┬─────┘
                │
        ┌───────┴───────┐
        │               │
        ▼               ▼
   ┌─────────┐     ┌─────────┐
   │  mobile │     │  demos  │
   └─────────┘     └─────────┘
```

---

## Migration Strategy

### Phase 1: Foundation (Week 1)

- [ ] Create workspace structure
- [ ] Extract `common` crate
- [ ] Extract `utils` crate
- [ ] Extract `macros` crate
- [ ] Verify compilation

### Phase 2: Core (Week 2)

- [ ] Extract `core` crate
- [ ] Extract `styles` crate
- [ ] Migrate build.rs to styles crate
- [ ] Verify theme loading works

### Phase 3: Components (Week 3)

- [ ] Extract `systems` crate
- [ ] Extract `components` crate
- [ ] Extract `icons` crate
- [ ] Move `examples/antd-01` to `demos/antd-01`
- [ ] Verify demos work

### Phase 4: Extras (Week 4)

- [ ] Create `tools` crate
- [ ] Create `mobile` crate
- [ ] Create `demos/showcase` crate
- [ ] Update documentation
- [ ] Final testing

---

## Benefits

| Benefit | Description |
|---------|-------------|
| **Parallel Compilation** | Independent crates compile in parallel |
| **Selective Dependencies** | Apps only include what they need |
| **Clear Boundaries** | Each crate has single responsibility |
| **Easier Testing** | Test crates in isolation |
| **Reusability** | `utils`, `icons`, `macros` usable standalone |
| **Faster Dev Builds** | Change one crate, only rebuild dependents |

---

## Comparison to Other Frameworks

| Crate | thaw Equivalent | ui Equivalent |
|-------|-----------------|---------------|
| `core` | `thaw` (partial) | - |
| `common` | - | `app_domain` |
| `utils` | `thaw_utils` | - |
| `macros` | `thaw_macro` | `leptos_ui` |
| `icons` | (icondata_ai) | `icons` |
| `styles` | (embedded) | `tw_merge` |
| `systems` | (Fluent only) | - |
| `components` | `thaw` | `registry` |
| `tools` | - | - |
| `mobile` | `thaw_mobile` | - |
