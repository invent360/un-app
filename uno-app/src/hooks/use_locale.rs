//! Localization hook for managing translations
//!
//! This module provides reactive locale management using the modular locale files.
//! It supports automatic language detection, bilingual country popups, and
//! cookie-based persistence.

use leptos::prelude::*;
use crate::locales;

/// Supported locales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    En,     // English
    Tl,     // Tagalog
    Hi,     // Hindi
    Sw,     // Swahili
    Es,     // Spanish
    Pt,     // Portuguese
    Fr,     // French
    Ar,     // Arabic
    Id,     // Bahasa Indonesia
    Bn,     // Bengali/Bangla (Phase 9)
}

impl Locale {
    pub fn code(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Tl => "tl",
            Locale::Hi => "hi",
            Locale::Sw => "sw",
            Locale::Es => "es",
            Locale::Pt => "pt",
            Locale::Fr => "fr",
            Locale::Ar => "ar",
            Locale::Id => "id",
            Locale::Bn => "bn",
        }
    }

    pub fn name(&self) -> &'static str {
        locales::locale_display_name(self.code())
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "en" => Some(Locale::En),
            "tl" => Some(Locale::Tl),
            "hi" => Some(Locale::Hi),
            "sw" => Some(Locale::Sw),
            "es" => Some(Locale::Es),
            "pt" => Some(Locale::Pt),
            "fr" => Some(Locale::Fr),
            "ar" => Some(Locale::Ar),
            "id" => Some(Locale::Id),
            "bn" => Some(Locale::Bn),
            _ => None,
        }
    }

    pub fn all() -> &'static [Locale] {
        &[
            Locale::En, Locale::Tl, Locale::Hi, Locale::Sw,
            Locale::Es, Locale::Pt, Locale::Fr, Locale::Ar, Locale::Id, Locale::Bn,
        ]
    }

    /// Check if locale is RTL (right-to-left)
    pub fn is_rtl(&self) -> bool {
        locales::is_rtl(self.code())
    }
}

/// Bilingual country options for language selection popup
#[derive(Clone, Debug)]
pub struct BilingualOptions {
    pub country_code: String,
    pub primary_locale: String,
    pub secondary_locale: String,
}

/// Locale context for sharing current locale and translations
#[derive(Clone)]
pub struct LocaleContext {
    pub locale: RwSignal<Locale>,
    pub show_locale_popup: RwSignal<bool>,
    pub bilingual_options: RwSignal<Option<BilingualOptions>>,
}

/// Provide locale context to the app
pub fn provide_locale_context() {
    let locale = RwSignal::new(Locale::default());
    let show_locale_popup = RwSignal::new(false);
    let bilingual_options = RwSignal::new(None::<BilingualOptions>);

    provide_context(LocaleContext {
        locale,
        show_locale_popup,
        bilingual_options,
    });
}

/// Use the locale context
pub fn use_locale() -> LocaleContext {
    expect_context::<LocaleContext>()
}

/// Get translation for a key using the current locale
pub fn t(key: &str) -> String {
    let ctx = use_locale();
    let locale_code = ctx.locale.get().code();
    let translations = locales::get_translations(locale_code);

    translations
        .get(key)
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Fallback to English if key not found in current locale
            if locale_code != "en" {
                locales::get_translations("en")
                    .get(key)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| key.to_string())
            } else {
                key.to_string()
            }
        })
}

/// Set the current locale
pub fn set_locale(new_locale: Locale) {
    let ctx = use_locale();
    ctx.locale.set(new_locale);

    // Note: Cookie persistence for locale is handled server-side
    // The locale change is reactive and updates the UI immediately
}

/// Initialize locale from various sources (cookie, geo, browser preference)
pub fn initialize_locale_from_request(
    cookie_locale: Option<&str>,
    country_code: Option<&str>,
    accept_language: Option<&str>,
) -> (Locale, Option<BilingualOptions>) {
    // Priority 1: Cookie (user's previous choice)
    if let Some(code) = cookie_locale {
        if let Some(locale) = Locale::from_code(code) {
            return (locale, None);
        }
    }

    // Priority 2: Check if bilingual country
    if let Some(country) = country_code {
        if let Some((primary, secondary)) = locales::get_bilingual_options(country) {
            // Return primary locale but also signal for popup
            let bilingual = BilingualOptions {
                country_code: country.to_string(),
                primary_locale: primary.to_string(),
                secondary_locale: secondary.to_string(),
            };
            let locale = Locale::from_code(primary).unwrap_or_default();
            return (locale, Some(bilingual));
        }

        // Priority 3: Country-based locale
        let country_locale = locales::get_country_locale(country);
        if let Some(locale) = Locale::from_code(country_locale) {
            return (locale, None);
        }
    }

    // Priority 4: Accept-Language header
    if let Some(accept_lang) = accept_language {
        // Parse Accept-Language header (simplified)
        // Format: "en-US,en;q=0.9,es;q=0.8"
        for lang in accept_lang.split(',') {
            let lang_code = lang.split(';').next().unwrap_or("")
                .split('-').next().unwrap_or("")
                .trim();
            if let Some(locale) = Locale::from_code(lang_code) {
                return (locale, None);
            }
        }
    }

    // Default: English
    (Locale::default(), None)
}

/// Trigger the bilingual language selection popup
pub fn show_bilingual_popup(options: BilingualOptions) {
    let ctx = use_locale();
    ctx.bilingual_options.set(Some(options));
    ctx.show_locale_popup.set(true);
}

/// Hide the language selection popup
pub fn hide_locale_popup() {
    let ctx = use_locale();
    ctx.show_locale_popup.set(false);
}

/// Check if locale popup should be shown
pub fn should_show_locale_popup() -> bool {
    let ctx = use_locale();
    ctx.show_locale_popup.get()
}

/// Get the current bilingual options (if any)
pub fn get_bilingual_options() -> Option<BilingualOptions> {
    let ctx = use_locale();
    ctx.bilingual_options.get()
}

// ============================================
// Re-export intl formatting functions
// ============================================

/// Format a number according to the current locale
pub fn format_number(value: f64) -> String {
    let ctx = use_locale();
    crate::locales::intl::format_number(value, ctx.locale.get().code())
}

/// Format a currency value according to the current locale
pub fn format_currency(value: f64, currency_code: &str) -> String {
    let ctx = use_locale();
    crate::locales::intl::format_currency(value, currency_code, ctx.locale.get().code())
}

/// Format a date according to the current locale
pub fn format_date(year: i32, month: u32, day: u32) -> String {
    let ctx = use_locale();
    crate::locales::intl::format_date(year, month, day, ctx.locale.get().code())
}

/// Format time according to the current locale
pub fn format_time(hour: u32, minute: u32) -> String {
    let ctx = use_locale();
    crate::locales::intl::format_time(hour, minute, ctx.locale.get().code())
}

/// Get pluralized form of a word based on count
pub fn pluralize(count: i64, singular: &str, plural: &str) -> String {
    let ctx = use_locale();
    crate::locales::intl::pluralize(count, singular, plural, ctx.locale.get().code())
}

/// Format a message with placeholders
pub fn format_message(pattern: &str, args: &[(&str, &str)]) -> String {
    let ctx = use_locale();
    crate::locales::intl::format_message(pattern, args, ctx.locale.get().code())
}

/// Get the plural category for a count in the current locale
pub fn get_plural_category(count: i64) -> crate::locales::intl::PluralCategory {
    let ctx = use_locale();
    crate::locales::intl::get_plural_category(count, ctx.locale.get().code())
}
