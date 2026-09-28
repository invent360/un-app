//! Internationalization formatting utilities
//!
//! This module provides locale-aware formatting for:
//! - Numbers (with grouping separators)
//! - Currency (with proper symbols and positioning)
//! - Dates and times
//! - Pluralization
//!
//! # Usage
//!
//! ```ignore
//! use crate::locales::intl::{format_number, format_currency, format_date, pluralize};
//!
//! // Number formatting
//! let formatted = format_number(1234567.89, "en"); // "1,234,567.89"
//! let formatted = format_number(1234567.89, "fr"); // "1 234 567,89"
//!
//! // Currency formatting
//! let usd = format_currency(99.99, "USD", "en"); // "$99.99"
//! let eur = format_currency(99.99, "EUR", "fr"); // "99,99 €"
//!
//! // Pluralization
//! let msg = pluralize(1, "license", "licenses", "en"); // "1 license"
//! let msg = pluralize(5, "license", "licenses", "en"); // "5 licenses"
//! ```

use std::collections::HashMap;

// ============================================
// Locale Formatting Configuration
// ============================================

/// Locale-specific formatting configuration
#[derive(Debug, Clone)]
pub struct LocaleFormat {
    /// Decimal separator (. or ,)
    pub decimal_sep: char,
    /// Thousands grouping separator (, or . or space)
    pub group_sep: char,
    /// Grouping size (typically 3)
    pub group_size: usize,
    /// Currency symbol position (true = before, false = after)
    pub currency_prefix: bool,
    /// Space between currency symbol and number
    pub currency_space: bool,
    /// Date format pattern (DMY, MDY, YMD)
    pub date_order: DateOrder,
    /// Date separator
    pub date_sep: char,
    /// 24-hour time format
    pub time_24h: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateOrder {
    DMY, // Day-Month-Year (most of world)
    MDY, // Month-Day-Year (US)
    YMD, // Year-Month-Day (ISO, East Asia)
}

impl Default for LocaleFormat {
    fn default() -> Self {
        Self {
            decimal_sep: '.',
            group_sep: ',',
            group_size: 3,
            currency_prefix: true,
            currency_space: false,
            date_order: DateOrder::MDY,
            date_sep: '/',
            time_24h: false,
        }
    }
}

/// Get locale-specific formatting configuration
pub fn get_locale_format(locale: &str) -> LocaleFormat {
    match locale.to_lowercase().as_str() {
        // English (US default)
        "en" | "en-us" => LocaleFormat::default(),

        // English (UK/Commonwealth)
        "en-gb" | "en-au" | "en-nz" | "en-ie" => LocaleFormat {
            date_order: DateOrder::DMY,
            time_24h: true,
            ..Default::default()
        },

        // Spanish
        "es" => LocaleFormat {
            decimal_sep: ',',
            group_sep: '.',
            currency_prefix: false,
            currency_space: true,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // Portuguese
        "pt" | "pt-br" => LocaleFormat {
            decimal_sep: ',',
            group_sep: '.',
            currency_prefix: true,
            currency_space: true,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // French
        "fr" => LocaleFormat {
            decimal_sep: ',',
            group_sep: ' ',
            currency_prefix: false,
            currency_space: true,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // Arabic
        "ar" => LocaleFormat {
            decimal_sep: '٫', // Arabic decimal separator
            group_sep: '٬',   // Arabic thousands separator
            currency_prefix: false,
            currency_space: true,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // Hindi
        "hi" => LocaleFormat {
            decimal_sep: '.',
            group_sep: ',',
            group_size: 2, // Indian numbering: 1,00,000
            currency_prefix: true,
            currency_space: false,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
        },

        // Swahili
        "sw" => LocaleFormat {
            decimal_sep: '.',
            group_sep: ',',
            currency_prefix: true,
            currency_space: true,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // Tagalog (follows US conventions)
        "tl" => LocaleFormat::default(),

        // Indonesian
        "id" => LocaleFormat {
            decimal_sep: ',',
            group_sep: '.',
            currency_prefix: true,
            currency_space: false,
            date_order: DateOrder::DMY,
            date_sep: '/',
            time_24h: true,
            ..Default::default()
        },

        // Default to English
        _ => LocaleFormat::default(),
    }
}

// ============================================
// Number Formatting
// ============================================

/// Format a number according to locale conventions
///
/// # Arguments
/// * `value` - The number to format
/// * `locale` - The locale code (e.g., "en", "fr", "ar")
///
/// # Examples
/// ```ignore
/// format_number(1234567.89, "en");  // "1,234,567.89"
/// format_number(1234567.89, "fr");  // "1 234 567,89"
/// format_number(1234567.89, "hi");  // "12,34,567.89" (Indian grouping)
/// ```
pub fn format_number(value: f64, locale: &str) -> String {
    let fmt = get_locale_format(locale);
    format_number_with_config(value, &fmt)
}

/// Format a number with explicit configuration
pub fn format_number_with_config(value: f64, fmt: &LocaleFormat) -> String {
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    // Split into integer and decimal parts
    let int_part = abs_value.trunc() as u64;
    let dec_part = abs_value.fract();

    // Format integer part with grouping
    let int_str = format_integer_with_grouping(int_part, fmt.group_sep, fmt.group_size);

    // Format decimal part (if any)
    let result = if dec_part > 0.0 {
        let dec_str = format!("{:.2}", dec_part);
        // Remove "0." prefix
        let dec_digits = &dec_str[2..];
        format!("{}{}{}", int_str, fmt.decimal_sep, dec_digits)
    } else {
        int_str
    };

    if is_negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Format an integer with grouping separators
fn format_integer_with_grouping(value: u64, sep: char, group_size: usize) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let digits: Vec<char> = value.to_string().chars().collect();
    let len = digits.len();
    let mut result = String::with_capacity(len + len / group_size);

    for (i, &digit) in digits.iter().enumerate() {
        if i > 0 {
            let remaining = len - i;
            // Handle Indian grouping (first group of 3, then groups of 2)
            if group_size == 2 {
                if remaining == 3 || (remaining > 3 && (remaining - 3) % 2 == 0) {
                    result.push(sep);
                }
            } else if remaining % group_size == 0 {
                result.push(sep);
            }
        }
        result.push(digit);
    }

    result
}

/// Format an integer number (no decimals)
pub fn format_integer(value: i64, locale: &str) -> String {
    let fmt = get_locale_format(locale);
    let is_negative = value < 0;
    let abs_value = value.unsigned_abs();
    let formatted = format_integer_with_grouping(abs_value, fmt.group_sep, fmt.group_size);

    if is_negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

/// Format a percentage
pub fn format_percent(value: f64, locale: &str) -> String {
    let fmt = get_locale_format(locale);
    let formatted = format_number_with_config(value, &fmt);
    format!("{}%", formatted)
}

// ============================================
// Currency Formatting
// ============================================

/// Currency information
#[derive(Debug, Clone)]
pub struct CurrencyInfo {
    pub code: &'static str,
    pub symbol: &'static str,
    pub name: &'static str,
    /// Decimal places (typically 2, but some currencies use 0 or 3)
    pub decimals: u8,
}

/// Get currency information by code
pub fn get_currency_info(code: &str) -> CurrencyInfo {
    match code.to_uppercase().as_str() {
        "USD" => CurrencyInfo {
            code: "USD",
            symbol: "$",
            name: "US Dollar",
            decimals: 2,
        },
        "EUR" => CurrencyInfo {
            code: "EUR",
            symbol: "€",
            name: "Euro",
            decimals: 2,
        },
        "GBP" => CurrencyInfo {
            code: "GBP",
            symbol: "£",
            name: "British Pound",
            decimals: 2,
        },
        "INR" => CurrencyInfo {
            code: "INR",
            symbol: "₹",
            name: "Indian Rupee",
            decimals: 2,
        },
        "PHP" => CurrencyInfo {
            code: "PHP",
            symbol: "₱",
            name: "Philippine Peso",
            decimals: 2,
        },
        "KES" => CurrencyInfo {
            code: "KES",
            symbol: "KSh",
            name: "Kenyan Shilling",
            decimals: 2,
        },
        "TZS" => CurrencyInfo {
            code: "TZS",
            symbol: "TSh",
            name: "Tanzanian Shilling",
            decimals: 0,
        },
        "MXN" => CurrencyInfo {
            code: "MXN",
            symbol: "$",
            name: "Mexican Peso",
            decimals: 2,
        },
        "BRL" => CurrencyInfo {
            code: "BRL",
            symbol: "R$",
            name: "Brazilian Real",
            decimals: 2,
        },
        "ARS" => CurrencyInfo {
            code: "ARS",
            symbol: "$",
            name: "Argentine Peso",
            decimals: 2,
        },
        "SAR" => CurrencyInfo {
            code: "SAR",
            symbol: "﷼",
            name: "Saudi Riyal",
            decimals: 2,
        },
        "AED" => CurrencyInfo {
            code: "AED",
            symbol: "د.إ",
            name: "UAE Dirham",
            decimals: 2,
        },
        "EGP" => CurrencyInfo {
            code: "EGP",
            symbol: "E£",
            name: "Egyptian Pound",
            decimals: 2,
        },
        "IDR" => CurrencyInfo {
            code: "IDR",
            symbol: "Rp",
            name: "Indonesian Rupiah",
            decimals: 0,
        },
        "NGN" => CurrencyInfo {
            code: "NGN",
            symbol: "₦",
            name: "Nigerian Naira",
            decimals: 2,
        },
        _ => CurrencyInfo {
            code: "USD",
            symbol: "$",
            name: "US Dollar",
            decimals: 2,
        },
    }
}

/// Format a currency value according to locale conventions
///
/// # Arguments
/// * `value` - The monetary value
/// * `currency_code` - ISO 4217 currency code (e.g., "USD", "EUR")
/// * `locale` - The locale code for formatting
///
/// # Examples
/// ```ignore
/// format_currency(1234.56, "USD", "en");  // "$1,234.56"
/// format_currency(1234.56, "EUR", "fr");  // "1 234,56 €"
/// format_currency(1234.56, "INR", "hi");  // "₹1,234.56"
/// ```
pub fn format_currency(value: f64, currency_code: &str, locale: &str) -> String {
    let fmt = get_locale_format(locale);
    let currency = get_currency_info(currency_code);

    format_currency_with_config(value, &currency, &fmt)
}

/// Format currency with explicit configuration
pub fn format_currency_with_config(value: f64, currency: &CurrencyInfo, fmt: &LocaleFormat) -> String {
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    // Round to currency's decimal places
    let multiplier = 10_f64.powi(currency.decimals as i32);
    let rounded = (abs_value * multiplier).round() / multiplier;

    // Format the number
    let int_part = rounded.trunc() as u64;
    let int_str = format_integer_with_grouping(int_part, fmt.group_sep, fmt.group_size);

    let formatted_number = if currency.decimals > 0 {
        let dec_part = ((rounded - rounded.trunc()) * multiplier).round() as u64;
        let dec_str = format!("{:0>width$}", dec_part, width = currency.decimals as usize);
        format!("{}{}{}", int_str, fmt.decimal_sep, dec_str)
    } else {
        int_str
    };

    // Combine symbol and number
    let space = if fmt.currency_space { " " } else { "" };
    let result = if fmt.currency_prefix {
        format!("{}{}{}", currency.symbol, space, formatted_number)
    } else {
        format!("{}{}{}", formatted_number, space, currency.symbol)
    };

    if is_negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Format a currency range (e.g., "$10 - $50")
pub fn format_currency_range(min: f64, max: f64, currency_code: &str, locale: &str) -> String {
    let min_formatted = format_currency(min, currency_code, locale);
    let max_formatted = format_currency(max, currency_code, locale);

    // For ranges, we typically only show the symbol once
    let currency = get_currency_info(currency_code);
    let fmt = get_locale_format(locale);

    if fmt.currency_prefix {
        // Symbol is prefix: show "$10 - $50" (both have symbols for clarity)
        format!("{} - {}", min_formatted, max_formatted)
    } else {
        // Symbol is suffix: show "10 - 50 €"
        let min_num = format_number(min, locale);
        let max_num = format_number(max, locale);
        let space = if fmt.currency_space { " " } else { "" };
        format!("{} - {}{}{}", min_num, max_num, space, currency.symbol)
    }
}

// ============================================
// Date/Time Formatting
// ============================================

/// Month names by locale
pub fn get_month_name(month: u32, locale: &str, abbreviated: bool) -> &'static str {
    let months_en = if abbreviated {
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
    } else {
        ["January", "February", "March", "April", "May", "June",
         "July", "August", "September", "October", "November", "December"]
    };

    let months_es = if abbreviated {
        ["ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic"]
    } else {
        ["enero", "febrero", "marzo", "abril", "mayo", "junio",
         "julio", "agosto", "septiembre", "octubre", "noviembre", "diciembre"]
    };

    let months_fr = if abbreviated {
        ["janv", "févr", "mars", "avr", "mai", "juin", "juil", "août", "sept", "oct", "nov", "déc"]
    } else {
        ["janvier", "février", "mars", "avril", "mai", "juin",
         "juillet", "août", "septembre", "octobre", "novembre", "décembre"]
    };

    let months_ar = if abbreviated {
        ["يناير", "فبراير", "مارس", "أبريل", "مايو", "يونيو",
         "يوليو", "أغسطس", "سبتمبر", "أكتوبر", "نوفمبر", "ديسمبر"]
    } else {
        ["يناير", "فبراير", "مارس", "أبريل", "مايو", "يونيو",
         "يوليو", "أغسطس", "سبتمبر", "أكتوبر", "نوفمبر", "ديسمبر"]
    };

    let months_pt = if abbreviated {
        ["jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"]
    } else {
        ["janeiro", "fevereiro", "março", "abril", "maio", "junho",
         "julho", "agosto", "setembro", "outubro", "novembro", "dezembro"]
    };

    let months_id = if abbreviated {
        ["Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des"]
    } else {
        ["Januari", "Februari", "Maret", "April", "Mei", "Juni",
         "Juli", "Agustus", "September", "Oktober", "November", "Desember"]
    };

    let idx = (month.saturating_sub(1) as usize).min(11);

    match locale.to_lowercase().as_str() {
        "es" => months_es[idx],
        "fr" => months_fr[idx],
        "ar" => months_ar[idx],
        "pt" | "pt-br" => months_pt[idx],
        "id" => months_id[idx],
        _ => months_en[idx],
    }
}

/// Format a date according to locale conventions
///
/// # Arguments
/// * `year` - The year
/// * `month` - The month (1-12)
/// * `day` - The day of month
/// * `locale` - The locale code
///
/// # Examples
/// ```ignore
/// format_date(2024, 3, 15, "en");  // "3/15/2024" (US)
/// format_date(2024, 3, 15, "fr");  // "15/03/2024"
/// ```
pub fn format_date(year: i32, month: u32, day: u32, locale: &str) -> String {
    let fmt = get_locale_format(locale);

    match fmt.date_order {
        DateOrder::MDY => format!("{}{}{}{}{}", month, fmt.date_sep, day, fmt.date_sep, year),
        DateOrder::DMY => format!("{:02}{}{:02}{}{}", day, fmt.date_sep, month, fmt.date_sep, year),
        DateOrder::YMD => format!("{}{}{:02}{}{:02}", year, fmt.date_sep, month, fmt.date_sep, day),
    }
}

/// Format a date with month name
///
/// # Examples
/// ```ignore
/// format_date_long(2024, 3, 15, "en");  // "March 15, 2024"
/// format_date_long(2024, 3, 15, "fr");  // "15 mars 2024"
/// ```
pub fn format_date_long(year: i32, month: u32, day: u32, locale: &str) -> String {
    let month_name = get_month_name(month, locale, false);
    let fmt = get_locale_format(locale);

    match fmt.date_order {
        DateOrder::MDY => format!("{} {}, {}", month_name, day, year),
        DateOrder::DMY => format!("{} {} {}", day, month_name, year),
        DateOrder::YMD => format!("{} {} {}", year, month_name, day),
    }
}

/// Format a time
pub fn format_time(hour: u32, minute: u32, locale: &str) -> String {
    let fmt = get_locale_format(locale);

    if fmt.time_24h {
        format!("{:02}:{:02}", hour, minute)
    } else {
        let (display_hour, period) = if hour == 0 {
            (12, "AM")
        } else if hour < 12 {
            (hour, "AM")
        } else if hour == 12 {
            (12, "PM")
        } else {
            (hour - 12, "PM")
        };
        format!("{}:{:02} {}", display_hour, minute, period)
    }
}

/// Format a relative time (e.g., "2 hours ago", "in 3 days")
pub fn format_relative_time(seconds_diff: i64, locale: &str) -> String {
    let abs_diff = seconds_diff.abs();

    let (value, unit_singular, unit_plural) = if abs_diff < 60 {
        (abs_diff, "second", "seconds")
    } else if abs_diff < 3600 {
        (abs_diff / 60, "minute", "minutes")
    } else if abs_diff < 86400 {
        (abs_diff / 3600, "hour", "hours")
    } else if abs_diff < 2592000 {
        (abs_diff / 86400, "day", "days")
    } else if abs_diff < 31536000 {
        (abs_diff / 2592000, "month", "months")
    } else {
        (abs_diff / 31536000, "year", "years")
    };

    let unit = pluralize_unit(value, unit_singular, unit_plural, locale);

    // Get localized "ago" / "in" based on locale
    match locale.to_lowercase().as_str() {
        "es" => {
            if seconds_diff < 0 {
                format!("hace {} {}", value, unit)
            } else {
                format!("en {} {}", value, unit)
            }
        }
        "fr" => {
            if seconds_diff < 0 {
                format!("il y a {} {}", value, unit)
            } else {
                format!("dans {} {}", value, unit)
            }
        }
        "ar" => {
            if seconds_diff < 0 {
                format!("منذ {} {}", value, unit)
            } else {
                format!("خلال {} {}", value, unit)
            }
        }
        "pt" => {
            if seconds_diff < 0 {
                format!("há {} {}", value, unit)
            } else {
                format!("em {} {}", value, unit)
            }
        }
        _ => {
            if seconds_diff < 0 {
                format!("{} {} ago", value, unit)
            } else {
                format!("in {} {}", value, unit)
            }
        }
    }
}

// ============================================
// Pluralization
// ============================================

/// Pluralization rule categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

/// Get the plural category for a number in a given locale
///
/// This implements a subset of CLDR plural rules.
/// See: https://cldr.unicode.org/index/cldr-spec/plural-rules
pub fn get_plural_category(count: i64, locale: &str) -> PluralCategory {
    let abs_count = count.abs();

    match locale.to_lowercase().as_str() {
        // English, German, etc.: one/other
        "en" | "de" | "es" | "pt" | "it" | "nl" | "sw" | "tl" | "id" => {
            if abs_count == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }

        // French: one for 0 and 1
        "fr" => {
            if abs_count == 0 || abs_count == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }

        // Arabic: complex rules (zero, one, two, few, many, other)
        "ar" => {
            if abs_count == 0 {
                PluralCategory::Zero
            } else if abs_count == 1 {
                PluralCategory::One
            } else if abs_count == 2 {
                PluralCategory::Two
            } else {
                let mod100 = abs_count % 100;
                if (3..=10).contains(&mod100) {
                    PluralCategory::Few
                } else if (11..=99).contains(&mod100) {
                    PluralCategory::Many
                } else {
                    PluralCategory::Other
                }
            }
        }

        // Hindi: one for 0 and 1
        "hi" => {
            if abs_count == 0 || abs_count == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }

        // Default: English-like
        _ => {
            if abs_count == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        }
    }
}

/// Simple pluralization helper (singular/plural forms)
///
/// # Examples
/// ```ignore
/// pluralize(1, "license", "licenses", "en");  // "1 license"
/// pluralize(5, "license", "licenses", "en");  // "5 licenses"
/// ```
pub fn pluralize(count: i64, singular: &str, plural: &str, locale: &str) -> String {
    let category = get_plural_category(count, locale);
    let word = match category {
        PluralCategory::One => singular,
        _ => plural,
    };
    format!("{} {}", format_integer(count, locale), word)
}

/// Helper for pluralizing time units - returns owned String
fn pluralize_unit(value: i64, singular: &str, plural: &str, locale: &str) -> String {
    let category = get_plural_category(value, locale);

    // Return appropriate form based on plural category
    let unit = match (singular, category) {
        ("second", PluralCategory::One) => "second",
        ("second", _) => "seconds",
        ("minute", PluralCategory::One) => "minute",
        ("minute", _) => "minutes",
        ("hour", PluralCategory::One) => "hour",
        ("hour", _) => "hours",
        ("day", PluralCategory::One) => "day",
        ("day", _) => "days",
        ("month", PluralCategory::One) => "month",
        ("month", _) => "months",
        ("year", PluralCategory::One) => "year",
        ("year", _) => "years",
        // Fallback for unknown units
        (_, PluralCategory::One) => singular,
        _ => plural,
    };
    unit.to_string()
}

/// ICU-style message format with placeholders and pluralization
///
/// Supports:
/// - `{0}`, `{1}`, ... for positional arguments
/// - `{count, plural, one{...} other{...}}` for pluralization
///
/// # Examples
/// ```ignore
/// let msg = format_message(
///     "{count, plural, one{# license} other{# licenses}} remaining",
///     &[("count", "5")],
///     "en"
/// );
/// // Returns: "5 licenses remaining"
/// ```
pub fn format_message(pattern: &str, args: &[(&str, &str)], locale: &str) -> String {
    let mut result = pattern.to_string();

    // Create args map
    let args_map: HashMap<&str, &str> = args.iter().copied().collect();

    // Simple placeholder replacement: {name}
    for (key, value) in args {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }

    // Handle plural patterns: {name, plural, one{...} other{...}}
    // This is a simplified implementation
    let plural_re = regex::Regex::new(r"\{(\w+),\s*plural,\s*([^}]+)\}").ok();

    if let Some(re) = plural_re {
        while let Some(caps) = re.captures(&result) {
            let full_match = caps.get(0).map(|m| m.as_str()).unwrap_or("");
            let var_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let plural_forms = caps.get(2).map(|m| m.as_str()).unwrap_or("");

            if let Some(count_str) = args_map.get(var_name) {
                if let Ok(count) = count_str.parse::<i64>() {
                    let category = get_plural_category(count, locale);
                    let category_name = match category {
                        PluralCategory::Zero => "zero",
                        PluralCategory::One => "one",
                        PluralCategory::Two => "two",
                        PluralCategory::Few => "few",
                        PluralCategory::Many => "many",
                        PluralCategory::Other => "other",
                    };

                    // Extract the form for this category
                    let form_pattern = format!(r"{}{{([^}}]+)}}", category_name);
                    if let Ok(form_re) = regex::Regex::new(&form_pattern) {
                        if let Some(form_caps) = form_re.captures(plural_forms) {
                            let form_text = form_caps.get(1).map(|m| m.as_str()).unwrap_or("");
                            // Replace # with the actual number
                            let formatted_count = format_integer(count, locale);
                            let replacement = form_text.replace('#', &formatted_count);
                            result = result.replace(full_match, &replacement);
                            continue;
                        }
                    }

                    // Fallback to "other" form
                    let other_pattern = r"other\{([^}]+)\}";
                    if let Ok(other_re) = regex::Regex::new(other_pattern) {
                        if let Some(other_caps) = other_re.captures(plural_forms) {
                            let form_text = other_caps.get(1).map(|m| m.as_str()).unwrap_or("");
                            let formatted_count = format_integer(count, locale);
                            let replacement = form_text.replace('#', &formatted_count);
                            result = result.replace(full_match, &replacement);
                        }
                    }
                }
            }
        }
    }

    result
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_english() {
        assert_eq!(format_number(1234567.89, "en"), "1,234,567.89");
        assert_eq!(format_number(0.0, "en"), "0");
        assert_eq!(format_number(-1234.56, "en"), "-1,234.56");
    }

    #[test]
    fn test_format_number_french() {
        assert_eq!(format_number(1234567.89, "fr"), "1 234 567,89");
    }

    #[test]
    fn test_format_number_indian() {
        // Indian grouping: 12,34,567 (groups of 2 after first 3)
        assert_eq!(format_number(1234567.0, "hi"), "12,34,567");
    }

    #[test]
    fn test_format_currency_usd_english() {
        assert_eq!(format_currency(1234.56, "USD", "en"), "$1,234.56");
        assert_eq!(format_currency(0.0, "USD", "en"), "$0.00");
    }

    #[test]
    fn test_format_currency_eur_french() {
        assert_eq!(format_currency(1234.56, "EUR", "fr"), "1 234,56 €");
    }

    #[test]
    fn test_plural_category_english() {
        assert_eq!(get_plural_category(0, "en"), PluralCategory::Other);
        assert_eq!(get_plural_category(1, "en"), PluralCategory::One);
        assert_eq!(get_plural_category(2, "en"), PluralCategory::Other);
    }

    #[test]
    fn test_plural_category_arabic() {
        assert_eq!(get_plural_category(0, "ar"), PluralCategory::Zero);
        assert_eq!(get_plural_category(1, "ar"), PluralCategory::One);
        assert_eq!(get_plural_category(2, "ar"), PluralCategory::Two);
        assert_eq!(get_plural_category(5, "ar"), PluralCategory::Few);
        assert_eq!(get_plural_category(15, "ar"), PluralCategory::Many);
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "license", "licenses", "en"), "1 license");
        assert_eq!(pluralize(5, "license", "licenses", "en"), "5 licenses");
    }

    #[test]
    fn test_format_date() {
        assert_eq!(format_date(2024, 3, 15, "en"), "3/15/2024");
        assert_eq!(format_date(2024, 3, 15, "fr"), "15/03/2024");
    }
}
