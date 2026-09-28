//! Scroll utilities.
//!
//! Helpers for managing scroll behavior and finding scroll parents.

use web_sys::Element;

/// Scroll behavior options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBehavior {
    /// Instant scroll (no animation).
    #[default]
    Auto,
    /// Smooth animated scroll.
    Smooth,
}

impl ScrollBehavior {
    /// Convert to the web API string value.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Smooth => "smooth",
        }
    }
}

/// Find the nearest scrollable parent of an element.
///
/// Walks up the DOM tree to find the first ancestor with scrollable overflow.
#[must_use]
pub fn get_scroll_parent(element: &Element) -> Option<Element> {
    let window = crate::dom::window()?;
    let mut current = element.parent_element();

    while let Some(parent) = current {
        let style = window.get_computed_style(&parent).ok().flatten()?;

        let overflow_y = style.get_property_value("overflow-y").ok()?;
        let overflow_x = style.get_property_value("overflow-x").ok()?;

        if overflow_y == "auto" || overflow_y == "scroll" ||
           overflow_x == "auto" || overflow_x == "scroll" {
            return Some(parent);
        }

        current = parent.parent_element();
    }

    // Return document element as fallback
    crate::dom::document_element()
}

/// Scroll an element into view using the simple API.
pub fn scroll_into_view(element: &Element, _behavior: ScrollBehavior) {
    // Use the basic scroll_into_view_with_bool for compatibility
    // true = align to top, false = align to bottom
    element.scroll_into_view_with_bool(true);
}

/// Scroll to the top of the page.
pub fn scroll_to_top(_behavior: ScrollBehavior) {
    if let Some(window) = crate::dom::window() {
        // Use the simple scroll_to API
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}
