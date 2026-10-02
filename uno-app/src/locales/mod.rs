//! Localization module with per-language translation files
//!
//! This module provides compile-time static translation maps using PHF (Perfect Hash Functions)
//! for O(1) lookup performance. Each language has its own file for maintainability.
//!
//! ## Features
//!
//! - **Static translations**: Compile-time PHF maps for fastest access
//! - **Lazy loading**: Optional dynamic loading for bundle size optimization
//! - **Bilingual support**: Country-specific language options
//!
//! ## Usage
//!
//! ```ignore
//! use crate::locales::{get_translations, lazy_loader};
//!
//! // Static (fastest)
//! let translation = get_translations("en").get("nav.home");
//!
//! // Lazy (smaller initial bundle)
//! lazy_loader::provide_lazy_translation_context();
//! let text = lazy_loader::lazy_t("nav.home");
//! ```

pub mod en;
pub mod es;
pub mod tl;
pub mod hi;
pub mod sw;
pub mod pt;
pub mod fr;
pub mod ar;
pub mod id;
pub mod bn;  // Bengali/Bangla - Phase 9
pub mod th;  // Thai - Phase 10
pub mod vi;  // Vietnamese - Phase 11
pub mod lazy_loader;
pub mod intl;

use phf::Map;

/// Get the translation map for a given locale code
pub fn get_translations(locale: &str) -> &'static Map<&'static str, &'static str> {
    match locale.to_lowercase().as_str() {
        "en" => &en::TRANSLATIONS,
        "es" => &es::TRANSLATIONS,
        "tl" => &tl::TRANSLATIONS,
        "hi" => &hi::TRANSLATIONS,
        "sw" => &sw::TRANSLATIONS,
        "pt" => &pt::TRANSLATIONS,
        "fr" => &fr::TRANSLATIONS,
        "ar" => &ar::TRANSLATIONS,
        "id" => &id::TRANSLATIONS,
        "bn" => &bn::TRANSLATIONS,
        "th" => &th::TRANSLATIONS,
        "vi" => &vi::TRANSLATIONS,
        _ => &en::TRANSLATIONS, // Fallback to English
    }
}

/// Bilingual countries with primary and secondary locale options
pub const BILINGUAL_COUNTRIES: &[(&str, &str, &str)] = &[
    // (country_code, primary_locale, secondary_locale)
    ("IN", "en", "hi"),  // India: English + Hindi
    ("KE", "en", "sw"),  // Kenya: English + Swahili
    ("PH", "en", "tl"),  // Philippines: English + Tagalog
    ("TZ", "sw", "en"),  // Tanzania: Swahili + English
    ("NG", "en", "yo"),  // Nigeria: English + Yoruba (future)
];

/// Country to default locale mapping for monolingual countries
pub const COUNTRY_LOCALES: &[(&str, &str)] = &[
    // English-speaking
    ("US", "en"), ("GB", "en"), ("AU", "en"), ("CA", "en"), ("NZ", "en"),
    ("IE", "en"), ("SG", "en"), ("ZA", "en"), ("GH", "en"), ("UG", "en"),

    // Spanish-speaking
    ("ES", "es"), ("MX", "es"), ("AR", "es"), ("CO", "es"), ("PE", "es"),
    ("CL", "es"), ("VE", "es"), ("EC", "es"), ("GT", "es"), ("CU", "es"),
    ("BO", "es"), ("DO", "es"), ("HN", "es"), ("PY", "es"), ("SV", "es"),
    ("NI", "es"), ("CR", "es"), ("PA", "es"), ("UY", "es"), ("PR", "es"),

    // Portuguese-speaking
    ("BR", "pt"), ("PT", "pt"), ("AO", "pt"), ("MZ", "pt"),

    // French-speaking
    ("FR", "fr"), ("BE", "fr"), ("CH", "fr"), ("SN", "fr"), ("CI", "fr"),
    ("CM", "fr"), ("MG", "fr"), ("ML", "fr"), ("BF", "fr"), ("NE", "fr"),

    // Arabic-speaking
    ("SA", "ar"), ("AE", "ar"), ("EG", "ar"), ("DZ", "ar"), ("MA", "ar"),
    ("IQ", "ar"), ("SD", "ar"), ("SY", "ar"), ("YE", "ar"), ("TN", "ar"),
    ("JO", "ar"), ("LY", "ar"), ("LB", "ar"), ("OM", "ar"), ("KW", "ar"),
    ("QA", "ar"), ("BH", "ar"),

    // Indonesian
    ("ID", "id"),

    // Bengali/Bangla-speaking
    ("BD", "bn"),  // Bangladesh

    // Thai
    ("TH", "th"),  // Thailand

    // Vietnamese
    ("VN", "vi"),  // Vietnam
];

/// Check if a country is bilingual and get its locale options
pub fn get_bilingual_options(country_code: &str) -> Option<(&'static str, &'static str)> {
    BILINGUAL_COUNTRIES.iter()
        .find(|(code, _, _)| *code == country_code)
        .map(|(_, primary, secondary)| (*primary, *secondary))
}

/// Get the default locale for a country
pub fn get_country_locale(country_code: &str) -> &'static str {
    // First check bilingual countries (return primary)
    if let Some((primary, _)) = get_bilingual_options(country_code) {
        return primary;
    }

    // Then check monolingual countries
    COUNTRY_LOCALES.iter()
        .find(|(code, _)| *code == country_code)
        .map(|(_, locale)| *locale)
        .unwrap_or("en") // Default to English
}

/// Get the display name for a locale code
pub fn locale_display_name(code: &str) -> &'static str {
    match code.to_lowercase().as_str() {
        "en" => "English",
        "es" => "Español",
        "tl" => "Tagalog",
        "hi" => "हिन्दी",
        "sw" => "Kiswahili",
        "pt" => "Português",
        "fr" => "Français",
        "ar" => "العربية",
        "id" => "Bahasa Indonesia",
        "bn" => "বাংলা",
        "th" => "ไทย",
        "vi" => "Tiếng Việt",
        _ => "English",
    }
}

/// Check if a locale is RTL (right-to-left)
pub fn is_rtl(locale: &str) -> bool {
    matches!(locale.to_lowercase().as_str(), "ar" | "he" | "fa" | "ur")
}

/// All supported locale codes
pub const SUPPORTED_LOCALES: &[&str] = &["en", "es", "tl", "hi", "sw", "pt", "fr", "ar", "id", "bn", "th", "vi"];
