//! Pagination types for API responses
//!
//! Provides cursor-based and offset-based pagination with standard response formats.

use serde::{Deserialize, Serialize};

/// Pagination query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    /// Sort field
    #[serde(default)]
    pub sort_by: Option<String>,
    /// Sort direction (asc/desc)
    #[serde(default)]
    pub sort_order: Option<String>,
}

fn default_page() -> i32 { 1 }
fn default_per_page() -> i32 { 20 }

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
            sort_by: None,
            sort_order: None,
        }
    }
}

impl PaginationParams {
    /// Create new pagination params
    pub fn new(page: i32, per_page: i32) -> Self {
        Self {
            page: page.max(1),
            per_page: per_page.clamp(1, 100),
            sort_by: None,
            sort_order: None,
        }
    }

    /// Get validated page number (minimum 1)
    pub fn page(&self) -> i32 {
        self.page.max(1)
    }

    /// Get validated per_page (clamped to 1-100)
    pub fn per_page(&self) -> i32 {
        self.per_page.clamp(1, 100)
    }

    /// Calculate SQL offset
    pub fn offset(&self) -> i64 {
        ((self.page() - 1) * self.per_page()) as i64
    }

    /// Calculate SQL limit
    pub fn limit(&self) -> i64 {
        self.per_page() as i64
    }

    /// Get sort direction (defaults to ASC)
    pub fn is_descending(&self) -> bool {
        self.sort_order
            .as_ref()
            .map(|s| s.to_lowercase() == "desc")
            .unwrap_or(false)
    }
}

/// Pagination metadata for responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// Current page number
    pub page: i32,
    /// Items per page
    pub per_page: i32,
    /// Total number of items
    pub total_items: i64,
    /// Total number of pages
    pub total_pages: i32,
    /// Has previous page
    pub has_prev: bool,
    /// Has next page
    pub has_next: bool,
}

impl PaginationMeta {
    /// Create pagination metadata from counts
    pub fn new(page: i32, per_page: i32, total_items: i64) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as i32;
        let page = page.max(1);

        Self {
            page,
            per_page,
            total_items,
            total_pages,
            has_prev: page > 1,
            has_next: page < total_pages,
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The data items
    pub data: Vec<T>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(data: Vec<T>, params: &PaginationParams, total_items: i64) -> Self {
        Self {
            data,
            pagination: PaginationMeta::new(params.page(), params.per_page(), total_items),
        }
    }

    /// Create an empty paginated response
    pub fn empty(params: &PaginationParams) -> Self {
        Self {
            data: Vec::new(),
            pagination: PaginationMeta::new(params.page(), params.per_page(), 0),
        }
    }
}

/// Cursor-based pagination for large datasets
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorParams {
    /// Cursor for next page (opaque string)
    pub cursor: Option<String>,
    /// Number of items to fetch
    #[serde(default = "default_cursor_limit")]
    pub limit: i32,
    /// Direction: forward or backward
    #[serde(default)]
    pub direction: Option<String>,
}

fn default_cursor_limit() -> i32 { 20 }

impl CursorParams {
    /// Get validated limit
    pub fn limit(&self) -> i32 {
        self.limit.clamp(1, 100)
    }

    /// Check if forward pagination
    pub fn is_forward(&self) -> bool {
        self.direction
            .as_ref()
            .map(|d| d.to_lowercase() != "backward")
            .unwrap_or(true)
    }

    /// Decode cursor (base64 encoded ID or timestamp)
    pub fn decode_cursor(&self) -> Option<i64> {
        self.cursor.as_ref().and_then(|c| {
            // Simple base64 decode to i64
            use std::str::FromStr;
            let decoded = String::from_utf8(
                base64_decode(c).ok()?
            ).ok()?;
            i64::from_str(&decoded).ok()
        })
    }
}

/// Cursor-based pagination response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorResponse<T> {
    /// The data items
    pub data: Vec<T>,
    /// Cursor for next page (None if no more data)
    pub next_cursor: Option<String>,
    /// Cursor for previous page (None if at start)
    pub prev_cursor: Option<String>,
    /// Whether there are more items
    pub has_more: bool,
}

impl<T> CursorResponse<T> {
    /// Create a new cursor response
    pub fn new(
        data: Vec<T>,
        next_cursor: Option<String>,
        prev_cursor: Option<String>,
        has_more: bool,
    ) -> Self {
        Self {
            data,
            next_cursor,
            prev_cursor,
            has_more,
        }
    }

    /// Create empty cursor response
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            next_cursor: None,
            prev_cursor: None,
            has_more: false,
        }
    }
}

/// Encode a value as cursor
pub fn encode_cursor(id: i64) -> String {
    base64_encode(id.to_string().as_bytes())
}

/// Simple base64 encode (no external dependency needed for basic use)
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;

        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }

    result
}

/// Simple base64 decode
fn base64_decode(data: &str) -> Result<Vec<u8>, ()> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn decode_char(c: u8) -> Result<u8, ()> {
        ALPHABET.iter().position(|&x| x == c).map(|p| p as u8).ok_or(())
    }

    let data = data.trim_end_matches('=');
    let mut result = Vec::new();

    for chunk in data.as_bytes().chunks(4) {
        if chunk.len() < 2 {
            break;
        }

        let b0 = decode_char(chunk[0])?;
        let b1 = decode_char(chunk[1])?;
        result.push((b0 << 2) | (b1 >> 4));

        if chunk.len() > 2 && chunk[2] != b'=' {
            let b2 = decode_char(chunk[2])?;
            result.push((b1 << 4) | (b2 >> 2));

            if chunk.len() > 3 && chunk[3] != b'=' {
                let b3 = decode_char(chunk[3])?;
                result.push((b2 << 6) | b3);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_defaults() {
        let params = PaginationParams::default();
        assert_eq!(params.page(), 1);
        assert_eq!(params.per_page(), 20);
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn test_pagination_params_clamping() {
        let params = PaginationParams::new(0, 500);
        assert_eq!(params.page(), 1);
        assert_eq!(params.per_page(), 100);
    }

    #[test]
    fn test_pagination_offset() {
        let params = PaginationParams::new(3, 10);
        assert_eq!(params.offset(), 20);
        assert_eq!(params.limit(), 10);
    }

    #[test]
    fn test_pagination_meta() {
        let meta = PaginationMeta::new(2, 10, 55);
        assert_eq!(meta.total_pages, 6);
        assert!(meta.has_prev);
        assert!(meta.has_next);
    }

    #[test]
    fn test_cursor_encode_decode() {
        let id = 12345i64;
        let cursor = encode_cursor(id);
        let params = CursorParams {
            cursor: Some(cursor),
            limit: 20,
            direction: None,
        };
        assert_eq!(params.decode_cursor(), Some(id));
    }
}
