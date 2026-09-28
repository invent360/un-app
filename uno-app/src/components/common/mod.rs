//! Shared UI primitives
//!
//! This module provides common UI components, many of which are thin wrappers
//! around ember-fx components for consistency and rich feature support.

mod accordion;
mod button;
mod card;
mod copy_button;
mod design_system_switcher;
mod error_display;
mod icon;
mod language_selector;
mod loading;
mod locale_popup;
mod modal;
mod preview_banner;
mod testimonial_carousel;
mod theme_switcher;

// Core components (with ember-fx backing)
pub use accordion::{Accordion, AccordionItem, CollapseIconPosition};
pub use button::{Button, ButtonVariant, ButtonSize, ButtonShape};
pub use card::{Card, CardSize};
pub use loading::{LoadingSpinner, SkeletonLine, SkeletonCard, LoadingPage, SpinnerSize};
pub use modal::{Modal, ModalSize};

// Re-export ember-fx components for direct usage when more control is needed
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use accordion::{Collapse, CollapsePanel};
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use loading::{Spinner, Skeleton};

// Custom components (not ember-fx backed)
pub use copy_button::CopyButton;
pub use error_display::{
    ErrorMessage, ErrorToast, EmptyState, ConnectionError,
    ServerError, LoadingError, ErrorSeverity
};
pub use icon::{Icon, IconName};
pub use language_selector::LanguageSelector;
pub use locale_popup::LocalePopup;
pub use testimonial_carousel::{TestimonialCarousel, TestimonialData, StarRating};
pub use theme_switcher::ThemeSwitcher;
pub use design_system_switcher::DesignSystemSwitcher;
pub use preview_banner::PreviewBanner;
