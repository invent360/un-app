//! Component traits organized by concern.
//!
//! Each submodule contains related traits that components can implement.
//! Traits are designed to be orthogonal - components mix and match as needed.

pub mod base;
pub mod size;
pub mod visual;
pub mod state;
pub mod interaction;
pub mod accessibility;

// Re-export all traits at this level for convenience
pub use base::{FxComponent, ComponentType};
pub use size::{Sizable, ComponentSize};
pub use visual::{Themed, Colored, Rounded, Bordered, ComponentColor, BorderRadius};
pub use state::{DisabledState, LoadingState};
pub use interaction::{Clickable, Focusable, Hoverable};
pub use accessibility::{Accessible, AriaRole};
