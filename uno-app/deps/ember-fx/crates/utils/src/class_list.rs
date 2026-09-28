//! Class name composition utilities.
//!
//! Provides a builder pattern for composing CSS class names dynamically.

use std::borrow::Cow;

/// A builder for composing CSS class names.
///
/// # Example
/// ```
/// use ember_fx_utils::ClassList;
///
/// let classes = ClassList::new()
///     .add("btn")
///     .add_if(true, "btn-primary")
///     .add_if(false, "btn-disabled")
///     .build();
///
/// assert_eq!(classes, "btn btn-primary");
/// ```
#[derive(Debug, Default, Clone)]
pub struct ClassList {
    classes: Vec<Cow<'static, str>>,
}

impl ClassList {
    /// Create a new empty class list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a class to the list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, class: impl Into<Cow<'static, str>>) -> Self {
        let class = class.into();
        if !class.is_empty() {
            self.classes.push(class);
        }
        self
    }

    /// Add a class conditionally.
    #[must_use]
    pub fn add_if(self, condition: bool, class: impl Into<Cow<'static, str>>) -> Self {
        if condition {
            self.add(class)
        } else {
            self
        }
    }

    /// Add a class from an optional value.
    #[must_use]
    pub fn add_option(self, class: Option<impl Into<Cow<'static, str>>>) -> Self {
        if let Some(c) = class {
            self.add(c)
        } else {
            self
        }
    }

    /// Add multiple classes at once.
    #[must_use]
    pub fn add_many(mut self, classes: impl IntoIterator<Item = impl Into<Cow<'static, str>>>) -> Self {
        for class in classes {
            self = self.add(class);
        }
        self
    }

    /// Build the final class string.
    #[must_use]
    pub fn build(self) -> String {
        self.classes.join(" ")
    }

    /// Check if the class list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }

    /// Get the number of classes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.classes.len()
    }
}

impl From<ClassList> for String {
    fn from(list: ClassList) -> Self {
        list.build()
    }
}

impl std::fmt::Display for ClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.classes.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_usage() {
        let classes = ClassList::new()
            .add("btn")
            .add("btn-primary")
            .build();
        assert_eq!(classes, "btn btn-primary");
    }

    #[test]
    fn test_conditional() {
        let classes = ClassList::new()
            .add("btn")
            .add_if(true, "active")
            .add_if(false, "disabled")
            .build();
        assert_eq!(classes, "btn active");
    }

    #[test]
    fn test_empty_class_ignored() {
        let classes = ClassList::new()
            .add("btn")
            .add("")
            .add("active")
            .build();
        assert_eq!(classes, "btn active");
    }

    #[test]
    fn test_option() {
        let classes = ClassList::new()
            .add("btn")
            .add_option(Some("active"))
            .add_option(None::<&str>)
            .build();
        assert_eq!(classes, "btn active");
    }
}
