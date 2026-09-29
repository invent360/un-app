//! Custom Actix extractors

pub mod auth;
pub mod geo;
pub mod locale;

pub use auth::{AuthenticatedUser, AuthError, get_authenticated_user};
pub use geo::{GeoLocation, country_name_from_code};
pub use locale::{RequestLocale, LocaleSource, SUPPORTED_LOCALES, DEFAULT_LOCALE, create_locale_cookie};
