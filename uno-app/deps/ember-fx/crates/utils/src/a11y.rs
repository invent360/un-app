//! Accessibility utilities for ember-fx.
//!
//! Provides helpers for focus management, keyboard navigation, and screen reader support.
//!
//! # Features
//!
//! - **Focus Trap** - Keep focus within a container (modals, drawers)
//! - **Keyboard Navigation** - Arrow key navigation for menus, listboxes
//! - **Screen Reader Text** - Visually hidden but accessible text
//! - **Live Regions** - Announce dynamic content changes
//! - **Skip Links** - Navigation shortcuts for keyboard users

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast};

/// CSS class for visually hidden but screen-reader accessible content.
pub const SR_ONLY_CLASS: &str = "fx-sr-only";

/// CSS for screen-reader only text (include in your stylesheet).
pub const SR_ONLY_CSS: &str = r#"
.fx-sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

.fx-sr-only-focusable:focus,
.fx-sr-only-focusable:active {
    position: static;
    width: auto;
    height: auto;
    overflow: visible;
    clip: auto;
    white-space: normal;
}
"#;

/// Selector for focusable elements.
pub const FOCUSABLE_SELECTOR: &str =
    "a[href], area[href], input:not([disabled]), select:not([disabled]), \
     textarea:not([disabled]), button:not([disabled]), iframe, object, embed, \
     [tabindex]:not([tabindex=\"-1\"]), [contenteditable], audio[controls], \
     video[controls], summary, [tabindex]:not([tabindex=\"-1\"])";

/// Get all focusable elements within a container.
#[cfg(target_arch = "wasm32")]
pub fn get_focusable_elements(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let node_list = container.query_selector_all(FOCUSABLE_SELECTOR).ok();

    node_list
        .map(|list| {
            (0..list.length())
                .filter_map(|i| list.item(i))
                .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
                .filter(|el| {
                    // Filter out hidden elements
                    let style = web_sys::window()
                        .and_then(|w| w.get_computed_style(el).ok())
                        .flatten();

                    style
                        .map(|s| {
                            s.get_property_value("display").ok() != Some("none".to_string())
                                && s.get_property_value("visibility").ok()
                                    != Some("hidden".to_string())
                        })
                        .unwrap_or(true)
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_focusable_elements(_container: &web_sys::Element) -> Vec<web_sys::Element> {
    Vec::new()
}

/// Focus the first focusable element in a container.
#[cfg(target_arch = "wasm32")]
pub fn focus_first(container: &web_sys::Element) {
    let elements = get_focusable_elements(container);
    if let Some(first) = elements.first() {
        if let Some(html_el) = first.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn focus_first(_container: &web_sys::Element) {}

/// Focus the last focusable element in a container.
#[cfg(target_arch = "wasm32")]
pub fn focus_last(container: &web_sys::Element) {
    let elements = get_focusable_elements(container);
    if let Some(last) = elements.last() {
        if let Some(html_el) = last.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn focus_last(_container: &web_sys::Element) {}

/// Key codes for keyboard navigation.
pub mod keys {
    pub const ENTER: &str = "Enter";
    pub const SPACE: &str = " ";
    pub const ESCAPE: &str = "Escape";
    pub const TAB: &str = "Tab";
    pub const ARROW_UP: &str = "ArrowUp";
    pub const ARROW_DOWN: &str = "ArrowDown";
    pub const ARROW_LEFT: &str = "ArrowLeft";
    pub const ARROW_RIGHT: &str = "ArrowRight";
    pub const HOME: &str = "Home";
    pub const END: &str = "End";
    pub const PAGE_UP: &str = "PageUp";
    pub const PAGE_DOWN: &str = "PageDown";
}

/// Direction for keyboard navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    Next,
    Previous,
    First,
    Last,
}

/// Navigate through a list of elements using keyboard.
///
/// Returns the index of the element to focus, or None if no navigation should occur.
pub fn navigate_list(
    key: &str,
    current_index: usize,
    total_items: usize,
    wrap: bool,
) -> Option<usize> {
    if total_items == 0 {
        return None;
    }

    match key {
        keys::ARROW_DOWN | keys::ARROW_RIGHT => {
            if current_index + 1 < total_items {
                Some(current_index + 1)
            } else if wrap {
                Some(0)
            } else {
                None
            }
        }
        keys::ARROW_UP | keys::ARROW_LEFT => {
            if current_index > 0 {
                Some(current_index - 1)
            } else if wrap {
                Some(total_items - 1)
            } else {
                None
            }
        }
        keys::HOME => Some(0),
        keys::END => Some(total_items.saturating_sub(1)),
        _ => None,
    }
}

/// Check if a key event should activate an element (Enter or Space).
pub fn is_activation_key(key: &str) -> bool {
    key == keys::ENTER || key == keys::SPACE
}

/// Check if a key event should close/dismiss (Escape).
pub fn is_dismiss_key(key: &str) -> bool {
    key == keys::ESCAPE
}

/// Generate a unique ID for accessibility associations.
#[cfg(target_arch = "wasm32")]
pub fn generate_id(prefix: &str) -> String {
    use js_sys::Math;
    let random = (Math::random() * 1_000_000.0) as u32;
    format!("{}-{}", prefix, random)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}-{}", prefix, id)
}

/// Announce a message to screen readers using a live region.
#[cfg(target_arch = "wasm32")]
pub fn announce(message: &str, politeness: AnnounceLevel) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Find or create the announcer element
            let announcer_id = "fx-announcer";
            let announcer = document
                .get_element_by_id(announcer_id)
                .or_else(|| {
                    document.create_element("div").ok().map(|el| {
                        el.set_id(announcer_id);
                        let _ = el.set_attribute("aria-live", politeness.as_str());
                        let _ = el.set_attribute("aria-atomic", "true");
                        let _ = el.set_attribute("class", SR_ONLY_CLASS);

                        if let Some(body) = document.body() {
                            let _ = body.append_child(&el);
                        }
                        el
                    })
                });

            if let Some(el) = announcer {
                // Clear and set to trigger announcement
                el.set_text_content(Some(""));
                // Use setTimeout to ensure the clear is processed
                let message = message.to_string();
                let closure = Closure::once(Box::new(move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(el) = document.get_element_by_id(announcer_id) {
                                el.set_text_content(Some(&message));
                            }
                        }
                    }
                }) as Box<dyn FnOnce()>);

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    100,
                );
                closure.forget();
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn announce(_message: &str, _politeness: AnnounceLevel) {}

/// Politeness level for screen reader announcements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnnounceLevel {
    /// Polite - announce when convenient (default)
    #[default]
    Polite,
    /// Assertive - announce immediately, interrupting
    Assertive,
}

impl AnnounceLevel {
    /// Get the aria-live attribute value.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// Focus trap state for modals and dialogs.
#[derive(Debug, Clone)]
pub struct FocusTrap {
    /// ID of the container element.
    pub container_id: String,
    /// Element that had focus before the trap was activated.
    pub previous_focus: Option<String>,
    /// Whether the trap is currently active.
    pub active: bool,
}

impl FocusTrap {
    /// Create a new focus trap for a container.
    pub fn new(container_id: impl Into<String>) -> Self {
        Self {
            container_id: container_id.into(),
            previous_focus: None,
            active: false,
        }
    }

    /// Activate the focus trap.
    #[cfg(target_arch = "wasm32")]
    pub fn activate(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Save current focus
                if let Some(active) = document.active_element() {
                    self.previous_focus = active.id().into();
                    if self.previous_focus.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
                        // Generate ID if element doesn't have one
                        let id = generate_id("fx-prev-focus");
                        let _ = active.set_id(&id);
                        self.previous_focus = Some(id);
                    }
                }

                // Focus first element in container
                if let Some(container) = document.get_element_by_id(&self.container_id) {
                    focus_first(&container);
                }

                self.active = true;
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate the focus trap and restore previous focus.
    #[cfg(target_arch = "wasm32")]
    pub fn deactivate(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Restore previous focus
                if let Some(ref prev_id) = self.previous_focus {
                    if let Some(el) = document.get_element_by_id(prev_id) {
                        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = html_el.focus();
                        }
                    }
                }
            }
        }

        self.active = false;
        self.previous_focus = None;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn deactivate(&mut self) {
        self.active = false;
        self.previous_focus = None;
    }

    /// Handle Tab key to trap focus within the container.
    #[cfg(target_arch = "wasm32")]
    pub fn handle_tab(&self, shift_key: bool) -> bool {
        if !self.active {
            return false;
        }

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(container) = document.get_element_by_id(&self.container_id) {
                    let focusable = get_focusable_elements(&container);
                    if focusable.is_empty() {
                        return false;
                    }

                    let active = document.active_element();
                    let active_index = active.as_ref().and_then(|a| {
                        focusable.iter().position(|el| {
                            el.is_same_node(Some(a))
                        })
                    });

                    match (shift_key, active_index) {
                        // Shift+Tab on first element -> go to last
                        (true, Some(0)) | (true, None) => {
                            if let Some(last) = focusable.last() {
                                if let Some(html_el) = last.dyn_ref::<web_sys::HtmlElement>() {
                                    let _ = html_el.focus();
                                    return true;
                                }
                            }
                        }
                        // Tab on last element -> go to first
                        (false, Some(i)) if i == focusable.len() - 1 => {
                            if let Some(first) = focusable.first() {
                                if let Some(html_el) = first.dyn_ref::<web_sys::HtmlElement>() {
                                    let _ = html_el.focus();
                                    return true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        false
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_tab(&self, _shift_key: bool) -> bool {
        false
    }
}

/// Roving tabindex manager for keyboard navigation in component groups.
#[derive(Debug, Clone)]
pub struct RovingTabindex {
    /// Current focused index.
    pub current_index: usize,
    /// Total number of items.
    pub total_items: usize,
    /// Whether to wrap around at boundaries.
    pub wrap: bool,
    /// Orientation (horizontal or vertical).
    pub orientation: Orientation,
}

/// Orientation for roving tabindex.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
    Both,
}

impl RovingTabindex {
    /// Create a new roving tabindex manager.
    pub fn new(total_items: usize) -> Self {
        Self {
            current_index: 0,
            total_items,
            wrap: true,
            orientation: Orientation::Both,
        }
    }

    /// Set the orientation.
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set whether to wrap around.
    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Handle a key event and return the new index if navigation occurred.
    pub fn handle_key(&mut self, key: &str) -> Option<usize> {
        let should_handle = match self.orientation {
            Orientation::Horizontal => {
                key == keys::ARROW_LEFT || key == keys::ARROW_RIGHT
            }
            Orientation::Vertical => {
                key == keys::ARROW_UP || key == keys::ARROW_DOWN
            }
            Orientation::Both => {
                matches!(key, keys::ARROW_LEFT | keys::ARROW_RIGHT | keys::ARROW_UP | keys::ARROW_DOWN)
            }
        };

        if !should_handle && key != keys::HOME && key != keys::END {
            return None;
        }

        let new_index = navigate_list(key, self.current_index, self.total_items, self.wrap)?;
        self.current_index = new_index;
        Some(new_index)
    }

    /// Get the tabindex value for an item at the given index.
    pub fn tabindex_for(&self, index: usize) -> i32 {
        if index == self.current_index {
            0
        } else {
            -1
        }
    }

    /// Update the total number of items.
    pub fn set_total(&mut self, total: usize) {
        self.total_items = total;
        if self.current_index >= total && total > 0 {
            self.current_index = total - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigate_list() {
        // Arrow down
        assert_eq!(navigate_list(keys::ARROW_DOWN, 0, 5, false), Some(1));
        assert_eq!(navigate_list(keys::ARROW_DOWN, 4, 5, false), None);
        assert_eq!(navigate_list(keys::ARROW_DOWN, 4, 5, true), Some(0));

        // Arrow up
        assert_eq!(navigate_list(keys::ARROW_UP, 2, 5, false), Some(1));
        assert_eq!(navigate_list(keys::ARROW_UP, 0, 5, false), None);
        assert_eq!(navigate_list(keys::ARROW_UP, 0, 5, true), Some(4));

        // Home/End
        assert_eq!(navigate_list(keys::HOME, 3, 5, false), Some(0));
        assert_eq!(navigate_list(keys::END, 1, 5, false), Some(4));
    }

    #[test]
    fn test_roving_tabindex() {
        let mut roving = RovingTabindex::new(5);

        assert_eq!(roving.tabindex_for(0), 0);
        assert_eq!(roving.tabindex_for(1), -1);

        assert_eq!(roving.handle_key(keys::ARROW_DOWN), Some(1));
        assert_eq!(roving.tabindex_for(0), -1);
        assert_eq!(roving.tabindex_for(1), 0);

        assert_eq!(roving.handle_key(keys::END), Some(4));
        assert_eq!(roving.current_index, 4);
    }

    #[test]
    fn test_is_activation_key() {
        assert!(is_activation_key(keys::ENTER));
        assert!(is_activation_key(keys::SPACE));
        assert!(!is_activation_key(keys::ESCAPE));
    }
}
