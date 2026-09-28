//! Simple markdown text parser for guide descriptions.
//!
//! Supports:
//! - `\n` for line breaks
//! - `**text**` for bold text
//! - `- item` for list items

use leptos::prelude::*;

/// Parses simple markdown-like text and renders it as HTML.
///
/// Supported syntax:
/// - `\n` - Line break
/// - `**text**` - Bold text
/// - `- text` - Bullet point (at start of line)
///
/// # Example
///
/// ```ignore
/// view! {
///     <MarkdownText text="Hello **world**!\n- Item 1\n- Item 2" />
/// }
/// ```
#[component]
pub fn MarkdownText(
    /// The text to parse and render.
    #[prop(into)]
    text: String,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let parsed = parse_markdown(&text);

    let class_str = class.unwrap_or_default();

    view! {
        <span class=class_str inner_html=parsed />
    }
}

/// Parses markdown-like text into HTML.
pub fn parse_markdown(text: &str) -> String {
    let mut result = String::new();

    // First, handle literal \n sequences (escaped newlines in the source)
    let text = text.replace("\\n", "\n");

    // Split by newlines to handle line-by-line processing
    let lines: Vec<&str> = text.split('\n').collect();
    let mut in_list = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Check if this is a list item
        if trimmed.starts_with("- ") {
            if !in_list {
                result.push_str("<ul class=\"fx-markdown-list\">");
                in_list = true;
            }
            let item_content = parse_inline(&trimmed[2..]);
            result.push_str(&format!("<li>{}</li>", item_content));
        } else {
            // Close list if we were in one
            if in_list {
                result.push_str("</ul>");
                in_list = false;
            }

            // Parse inline formatting
            let parsed_line = parse_inline(trimmed);

            if !parsed_line.is_empty() {
                result.push_str(&parsed_line);

                // Add line break if not the last line and next line is not empty
                if i < lines.len() - 1 {
                    let next_trimmed = lines.get(i + 1).map(|s| s.trim()).unwrap_or("");
                    if !next_trimmed.is_empty() && !next_trimmed.starts_with("- ") {
                        result.push_str("<br/>");
                    }
                }
            }
        }
    }

    // Close any open list
    if in_list {
        result.push_str("</ul>");
    }

    result
}

/// Parses inline formatting (bold, etc.)
fn parse_inline(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for ** (bold)
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            // Find closing **
            if let Some(end) = find_closing_marker(&chars, i + 2, "**") {
                let bold_text: String = chars[i + 2..end].iter().collect();
                result.push_str(&format!("<strong>{}</strong>", escape_html(&bold_text)));
                i = end + 2;
                continue;
            }
        }

        // Regular character - escape HTML
        result.push_str(&escape_html_char(chars[i]));
        i += 1;
    }

    result
}

/// Finds the position of a closing marker.
fn find_closing_marker(chars: &[char], start: usize, marker: &str) -> Option<usize> {
    let marker_chars: Vec<char> = marker.chars().collect();
    let marker_len = marker_chars.len();

    let mut i = start;
    while i + marker_len <= chars.len() {
        let mut matches = true;
        for (j, mc) in marker_chars.iter().enumerate() {
            if chars[i + j] != *mc {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Escapes HTML special characters.
fn escape_html(text: &str) -> String {
    text.chars().map(escape_html_char).collect()
}

/// Escapes a single HTML character.
fn escape_html_char(c: char) -> String {
    match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#39;".to_string(),
        _ => c.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text() {
        assert_eq!(parse_markdown("Hello world"), "Hello world");
    }

    #[test]
    fn test_bold() {
        assert_eq!(parse_markdown("Hello **world**"), "Hello <strong>world</strong>");
    }

    #[test]
    fn test_newline() {
        assert_eq!(parse_markdown("Line 1\\nLine 2"), "Line 1<br/>Line 2");
    }

    #[test]
    fn test_list() {
        let result = parse_markdown("- Item 1\\n- Item 2");
        assert!(result.contains("<ul"));
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
    }
}
