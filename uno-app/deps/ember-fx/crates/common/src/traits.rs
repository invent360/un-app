//! Common component traits.

/// Trait for components that accept common props.
///
/// This trait provides a standard interface for common component properties
/// like CSS classes and disabled state.
pub trait ComponentProps {
    /// Get the optional CSS class string.
    fn class(&self) -> Option<&str>;

    /// Check if the component is disabled.
    fn disabled(&self) -> bool;
}
