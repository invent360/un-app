//! Button components.
//!
//! A comprehensive button component library following Ant Design patterns
//! with support for both legacy and modern API systems.
//!
//! ## Components
//!
//! - `Button` - Standard button with multiple styles, sizes, and states
//! - `ButtonGroup` - Groups buttons together with connected styling
//! - `IconButton` - Icon-only circular button
//! - `FloatButton` - Floating action button (FAB)
//! - `BackTop` - Scroll-to-top button
//! - `SplitButton` - Button with dropdown menu
//! - `SpeedDial` - Expanding FAB with actions
//!
//! ## API Systems
//!
//! ### Legacy API (Simple)
//! Use `variant` prop for quick button styling:
//! - `Primary`, `Secondary`, `Outline`, `Ghost`, `Danger`, `Link`, `Blue`, `Dashed`, `Text`
//!
//! ### Modern API (Flexible)
//! Use `color` + `style_variant` for full control:
//! - **Colors**: `Default`, `Primary`, `Danger`, + preset colors (Pink, Purple, Cyan, etc.)
//! - **Style Variants**: `Solid`, `Outlined`, `Dashed`, `Filled`, `Text`, `Link`
//!
//! ## Features
//!
//! - Multiple color + variant combinations
//! - Configurable sizes: Xs, Sm, Md, Lg, Xl
//! - Shapes: Default, Circle, Round
//! - States: Disabled, Loading, Ghost, Block
//! - Icon support with position (start/end)
//! - Link buttons (renders as `<a>`)
//! - Full accessibility support
//! - Theme-aware styling
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx_components::{
//!     Button, ButtonVariant, ButtonSize, ButtonShape,
//!     ButtonColor, ButtonStyleVariant, IconPosition,
//!     ButtonGroup, IconButton, FloatButton, BackTop,
//!     SplitButton, SplitButtonItem, SpeedDial, SpeedDialAction,
//! };
//!
//! view! {
//!     // Legacy API
//!     <Button variant=ButtonVariant::Primary>"Primary"</Button>
//!     <Button variant=ButtonVariant::Danger>"Delete"</Button>
//!
//!     // Modern API
//!     <Button color=ButtonColor::Primary style_variant=ButtonStyleVariant::Solid>"Solid"</Button>
//!     <Button color=ButtonColor::Danger style_variant=ButtonStyleVariant::Outlined>"Outlined"</Button>
//!
//!     // Shapes
//!     <Button shape=ButtonShape::Round>"Round"</Button>
//!     <Button shape=ButtonShape::Circle icon="+">"+"</Button>
//!
//!     // States
//!     <Button loading=true>"Processing..."</Button>
//!     <Button ghost=true>"Ghost"</Button>
//!     <Button block=true>"Full Width"</Button>
//!
//!     // Icons
//!     <Button icon="🔍" icon_position=IconPosition::Start>"Search"</Button>
//!
//!     // Link button
//!     <Button href="/home">"Go Home"</Button>
//! }
//! ```

mod types;
mod component;
mod button_group;
mod icon_button;
mod float_button;
mod split_button;
mod speed_dial;

pub use types::{
    // Modern API types
    ButtonColor, ButtonStyleVariant, IconPosition, HtmlButtonType, LoadingConfig,
    // Legacy types
    ButtonVariant, ButtonSize, ButtonShape,
    // Float button types
    FloatButtonPlacement,
    // Speed dial types
    SpeedDialDirection, SpeedDialAction,
};
pub use component::Button;
pub use button_group::{ButtonGroup, ButtonGroupContext, try_use_button_group};
pub use icon_button::IconButton;
pub use float_button::{FloatButton, FloatButtonGroup, BackTop};
pub use split_button::{SplitButton, SplitButtonItem};
pub use speed_dial::SpeedDial;
