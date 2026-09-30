//! Locale extraction from HTTP requests

use actix_web::{FromRequest, HttpRequest, dev::Payload, cookie::Cookie};
use std::future::{Ready, ready};

/// Supported locale codes
pub const SUPPORTED_LOCALES: &[&str] = &["en", "tl", "hi", "sw", "es", "pt", "fr", "ar", "id", "bn"];

/// Default locale
pub const DEFAULT_LOCALE: &str = "en";

/// Extracted locale from request
#[derive(Debug, Clone)]
pub struct RequestLocale {
    /// The determined locale code
    pub code: String,
    /// Source of the locale (for debugging)
    pub source: LocaleSource,
}

/// Source of locale detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocaleSource {
    /// From ?lang= query parameter
    QueryParam,
    /// From uno-locale cookie
    Cookie,
    /// From Accept-Language header
    AcceptLanguage,
    /// Default fallback
    Default,
}

impl RequestLocale {
    /// Create with default locale
    pub fn default_locale() -> Self {
        Self {
            code: DEFAULT_LOCALE.to_string(),
            source: LocaleSource::Default,
        }
    }

    /// Check if locale is RTL (right-to-left)
    pub fn is_rtl(&self) -> bool {
        self.code == "ar"
    }

    /// Get locale name for display
    pub fn display_name(&self) -> &'static str {
        match self.code.as_str() {
            "en" => "English",
            "tl" => "Tagalog",
            "hi" => "हिन्दी",
            "sw" => "Kiswahili",
            "es" => "Español",
            "pt" => "Português",
            "fr" => "Français",
            "ar" => "العربية",
            "id" => "Bahasa Indonesia",
            "bn" => "বাংলা",
            _ => "English",
        }
    }
}

impl Default for RequestLocale {
    fn default() -> Self {
        Self::default_locale()
    }
}

/// Extract locale from various sources in order of priority:
/// 1. Query parameter (?lang=es)
/// 2. Cookie (uno-locale=es)
/// 3. Accept-Language header
/// 4. Default to English
fn extract_locale(req: &HttpRequest) -> RequestLocale {
    // 1. Check query parameter
    if let Some(lang) = req.query_string()
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.split('=');
            if parts.next() == Some("lang") {
                parts.next()
            } else {
                None
            }
        })
    {
        if is_supported_locale(lang) {
            return RequestLocale {
                code: lang.to_lowercase(),
                source: LocaleSource::QueryParam,
            };
        }
    }

    // 2. Check cookie
    if let Some(cookie_header) = req.headers().get("Cookie") {
        if let Ok(cookies_str) = cookie_header.to_str() {
            for cookie_part in cookies_str.split(';') {
                let cookie_part = cookie_part.trim();
                if let Some(value) = cookie_part.strip_prefix("uno-locale=") {
                    let locale = value.trim();
                    if is_supported_locale(locale) {
                        return RequestLocale {
                            code: locale.to_lowercase(),
                            source: LocaleSource::Cookie,
                        };
                    }
                }
            }
        }
    }

    // 3. Check Accept-Language header
    if let Some(accept_lang) = req.headers().get("Accept-Language") {
        if let Ok(accept_str) = accept_lang.to_str() {
            if let Some(locale) = parse_accept_language(accept_str) {
                return RequestLocale {
                    code: locale.to_string(),
                    source: LocaleSource::AcceptLanguage,
                };
            }
        }
    }

    // 4. Default
    RequestLocale::default_locale()
}

/// Check if a locale code is supported
fn is_supported_locale(code: &str) -> bool {
    let code_lower = code.to_lowercase();
    SUPPORTED_LOCALES.contains(&code_lower.as_str())
}

/// Parse Accept-Language header and return the best matching locale
/// Format: "en-US,en;q=0.9,es;q=0.8"
fn parse_accept_language(header: &str) -> Option<&'static str> {
    // Parse language preferences with quality values
    let mut preferences: Vec<(&str, f32)> = header
        .split(',')
        .filter_map(|part| {
            let part = part.trim();
            let (lang, quality) = if let Some((lang, q)) = part.split_once(";q=") {
                (lang.trim(), q.parse().unwrap_or(0.0))
            } else {
                (part, 1.0)
            };

            // Extract primary language code (e.g., "en" from "en-US")
            let primary = lang.split('-').next()?;
            Some((primary, quality))
        })
        .collect();

    // Sort by quality (descending)
    preferences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Find first supported locale
    for (lang, _) in preferences {
        let lang_lower = lang.to_lowercase();
        for &supported in SUPPORTED_LOCALES {
            if lang_lower == supported {
                return Some(supported);
            }
        }
    }

    None
}

impl FromRequest for RequestLocale {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(extract_locale(req)))
    }
}

/// Helper to create a locale cookie
pub fn create_locale_cookie(locale: &str) -> Cookie<'static> {
    Cookie::build("uno-locale", locale.to_string())
        .path("/")
        .max_age(actix_web::cookie::time::Duration::days(365))
        .http_only(false) // Allow JS access for client-side reading
        .secure(false) // Set to true in production with HTTPS
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_locales() {
        assert!(is_supported_locale("en"));
        assert!(is_supported_locale("ES")); // Case insensitive
        assert!(is_supported_locale("tl"));
        assert!(!is_supported_locale("xx"));
        assert!(!is_supported_locale("de")); // German not supported
    }

    #[test]
    fn test_parse_accept_language() {
        assert_eq!(parse_accept_language("en-US,en;q=0.9"), Some("en"));
        assert_eq!(parse_accept_language("es-MX,es;q=0.9,en;q=0.8"), Some("es"));
        assert_eq!(parse_accept_language("tl-PH,tl;q=0.9"), Some("tl"));
        assert_eq!(parse_accept_language("de-DE,de;q=0.9"), None); // German not supported
    }

    #[test]
    fn test_rtl_detection() {
        let ar = RequestLocale { code: "ar".to_string(), source: LocaleSource::Default };
        assert!(ar.is_rtl());

        let en = RequestLocale { code: "en".to_string(), source: LocaleSource::Default };
        assert!(!en.is_rtl());
    }
}
