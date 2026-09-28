//! DOM helper functions.
//!
//! Provides convenient wrappers around web-sys DOM APIs.

use web_sys::{Document, Element, Window};

/// Get the global window object.
#[must_use]
pub fn window() -> Option<Window> {
    web_sys::window()
}

/// Get the document object.
#[must_use]
pub fn document() -> Option<Document> {
    window().and_then(|w| w.document())
}

/// Get an element by its ID.
#[must_use]
pub fn get_element_by_id(id: &str) -> Option<Element> {
    document().and_then(|d| d.get_element_by_id(id))
}

/// Query for an element using a CSS selector.
#[must_use]
pub fn query_selector(selector: &str) -> Option<Element> {
    document().and_then(|d| d.query_selector(selector).ok().flatten())
}

/// Get the document's root element (`<html>`).
#[must_use]
pub fn document_element() -> Option<Element> {
    document().and_then(|d| d.document_element())
}

/// Get the document's body element.
#[must_use]
pub fn body() -> Option<web_sys::HtmlElement> {
    document().and_then(|d| d.body())
}

/// Set an attribute on the document element.
pub fn set_document_attribute(name: &str, value: &str) {
    if let Some(el) = document_element() {
        let _ = el.set_attribute(name, value);
    }
}

/// Get an attribute from the document element.
#[must_use]
pub fn get_document_attribute(name: &str) -> Option<String> {
    document_element().and_then(|el| el.get_attribute(name))
}

/// Remove an attribute from the document element.
pub fn remove_document_attribute(name: &str) {
    if let Some(el) = document_element() {
        let _ = el.remove_attribute(name);
    }
}
