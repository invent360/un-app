//! # ember-fx-utils
//!
//! DOM utilities and helper functions for ember-fx.
//!
//! This crate provides utilities for:
//! - CSS injection and management
//! - DOM manipulation helpers
//! - Class name composition
//! - Scroll utilities
//! - Accessibility helpers

mod css_loader;
mod class_list;
mod dom;
mod scroll;
pub mod a11y;

pub use css_loader::{
    inject_css, remove_css, has_css, inject_base_css,
    apply_theme_attribute, get_current_theme_attribute,
    THEME_STYLE_ID, COMPONENTS_STYLE_ID, BASE_STYLE_ID,
};
pub use class_list::ClassList;
pub use dom::{document, window, get_element_by_id, query_selector};
pub use scroll::{get_scroll_parent, scroll_into_view, ScrollBehavior};
pub use a11y::{
    FocusTrap, RovingTabindex, Orientation, AnnounceLevel,
    announce, generate_id, navigate_list, is_activation_key, is_dismiss_key,
    focus_first, focus_last, get_focusable_elements,
    keys, SR_ONLY_CLASS, SR_ONLY_CSS, FOCUSABLE_SELECTOR,
};
