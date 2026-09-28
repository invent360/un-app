//! # ember-fx-macros
//!
//! Procedural and declarative macros for ember-fx.
//!
//! ## Macros
//!
//! - `clx!` - Compose CSS class names
//! - `variants!` - Define component variants (planned)
//! - `WriteCSSVars` - Generate CSS variable writers (planned)

use proc_macro::TokenStream;

mod clx;
mod write_css_vars;

/// Compose CSS class names with conditional support.
///
/// # Examples
///
/// Basic usage:
/// ```ignore
/// use ember_fx_macros::clx;
///
/// let classes = clx!("btn", "btn-primary");
/// assert_eq!(classes, "btn btn-primary");
/// ```
///
/// Conditional classes:
/// ```ignore
/// use ember_fx_macros::clx;
///
/// let is_active = true;
/// let is_disabled = false;
/// let classes = clx!(
///     "btn",
///     is_active => "active",
///     is_disabled => "disabled",
/// );
/// assert_eq!(classes, "btn active");
/// ```
#[proc_macro]
pub fn clx(input: TokenStream) -> TokenStream {
    clx::clx_impl(input)
}

/// Derive macro for generating CSS variable writers.
///
/// Generates methods to write struct fields as CSS custom properties.
///
/// # Example
///
/// ```ignore
/// use ember_fx_macros::WriteCSSVars;
///
/// #[derive(WriteCSSVars)]
/// pub struct ColorTheme {
///     color_primary: String,    // -> --colorPrimary
///     color_secondary: String,  // -> --colorSecondary
/// }
/// ```
#[proc_macro_derive(WriteCSSVars, attributes(css_var))]
pub fn write_css_vars(input: TokenStream) -> TokenStream {
    write_css_vars::write_css_vars_impl(input)
}
