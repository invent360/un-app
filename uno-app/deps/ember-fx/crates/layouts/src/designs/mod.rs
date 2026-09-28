//! Design system layout implementations.
//!
//! Each design system has its own module with layout components styled
//! according to that system's guidelines.
//!
//! Available design systems:
//! - **ant** - Ant Design styled layouts
//! - **mui** - Material Design (coming soon)
//! - **prime** - PrimeReact style (coming soon)

#[cfg(feature = "ant")]
pub mod ant;

// Future design systems:
// #[cfg(feature = "mui")]
// pub mod mui;
//
// #[cfg(feature = "prime")]
// pub mod prime;
