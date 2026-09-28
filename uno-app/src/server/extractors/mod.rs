//! Custom Actix extractors

pub mod geo;
pub mod locale;

pub use geo::{GeoLocation, country_name_from_code};
pub use locale::{RequestLocale, LocaleSource, SUPPORTED_LOCALES, DEFAULT_LOCALE, create_locale_cookie};
