//! Custom Leptos hooks

mod use_api;
mod use_clipboard;
mod use_debounce;
mod use_lazy_load;
mod use_locale;
mod use_theme;
mod use_user;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
mod use_validation;

pub use use_api::{use_api, use_api_auto, ApiState};
pub use use_clipboard::use_clipboard;
pub use use_debounce::{use_debounce, use_throttle, DebouncedInput};
pub use use_lazy_load::{LazyImage, ResourceHints, Skeleton, SkeletonText, SkeletonCard};
pub use use_locale::{
    provide_locale_context, use_locale, t, set_locale,
    Locale, LocaleContext, BilingualOptions,
    initialize_locale_from_request, show_bilingual_popup,
    hide_locale_popup, should_show_locale_popup, get_bilingual_options,
    // Intl formatting functions
    format_number, format_currency, format_date, format_time,
    pluralize, format_message, get_plural_category,
};

// Re-export intl types for direct access
pub use crate::locales::intl::PluralCategory;
pub use use_theme::{
    provide_theme_context, use_theme, set_theme, current_theme,
    toggle_theme, initialize_theme, Theme, UnoThemeContext,
    // Design system support
    current_design_system, set_design_system, toggle_design_system,
    DesignSystem,
};

// Backward compatibility alias
pub type ThemeContext = UnoThemeContext;

// Re-export ember-fx types when features are enabled
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use use_theme::{EmberThemeContext, use_ember_theme, try_use_ember_theme};

// Re-export ember-fx toast hooks for convenience
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::{use_toast, try_use_toast, ToastItem, AlertType as ToastType};

// Re-export ember-fx form validation for convenience
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::{
    ValidationRule, ValidationResult, ValidateOn,
    validate_value, validate_all,
    FormField, ReactiveFormField, ValidationState,
};

// Application-specific validation utilities
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use use_validation::{
    email_rules, license_key_rules, username_rules, password_rules,
    use_form_validation, use_form_validation_all, FormState,
};

// R5-13: User session context
pub use use_user::{
    provide_user_context, use_user, is_authenticated, current_user, user_role,
    UserContext, UserProfile, UserLoadState, LicenseStatus, CohortStatus,
};
