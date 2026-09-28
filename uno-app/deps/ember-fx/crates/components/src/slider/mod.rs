//! Slider components for selecting values from a range.
//!
//! This module provides slider components for selecting single values or ranges:
//!
//! - [`Slider`] - Single value slider with marks, tooltips, and keyboard navigation
//! - [`RangeSlider`] - Dual-handle slider for selecting a range of values
//!
//! ## Features
//!
//! - **Orientation**: Horizontal or vertical layout
//! - **Marks**: Labeled tick marks at specific values
//! - **Dots**: Step interval indicators
//! - **Tooltips**: Configurable value display on hover/drag
//! - **Keyboard Navigation**: Full accessibility with Arrow, PageUp/Down, Home/End keys
//! - **Touch Support**: Mobile-friendly drag interactions
//! - **Reverse**: Reverse direction for RTL or custom layouts
//! - **Range Config**: Draggable track, min/max range constraints, pushable handles
//! - **Semantic Styling**: Override classes/styles for any slider part
//!
//! ## Example
//!
//! ```ignore
//! use ember_fx_components::{Slider, RangeSlider, SliderMark, TooltipConfig};
//!
//! // Basic slider
//! let value = RwSignal::new(50.0);
//! view! { <Slider value=value /> }
//!
//! // Slider with marks
//! let marks = vec![
//!     SliderMark::new(0.0, "0%"),
//!     SliderMark::new(50.0, "50%"),
//!     SliderMark::new(100.0, "100%"),
//! ];
//! view! { <Slider value=value marks=marks dots=true /> }
//!
//! // Range slider
//! let range = RwSignal::new((20.0, 80.0));
//! view! { <RangeSlider value=range /> }
//!
//! // Vertical slider with always-visible tooltip
//! view! {
//!     <Slider
//!         value=value
//!         orientation=SliderOrientation::Vertical
//!         tooltip=TooltipConfig::always()
//!     />
//! }
//! ```

pub mod types;
pub mod slider;
pub mod range_slider;

// Re-export types
pub use types::{
    SliderSize,
    SliderOrientation,
    TooltipPlacement,
    TooltipVisibility,
    TooltipConfig,
    RangeConfig,
    SliderMark,
    SliderClassNames,
    SliderStyles,
};

// Re-export components
pub use slider::{Slider, SliderVariant};
pub use range_slider::RangeSlider;
