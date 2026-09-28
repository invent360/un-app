//! Wizard variant card component
//!
//! This module re-exports the unified VariantCard from features/license
//! and provides a convenience wrapper for the wizard context.

use leptos::prelude::*;
use crate::types::LicenseVariant;
use crate::features::license::{VariantCard as BaseVariantCard, VariantCardMode};

/// Variant card for wizard selection
///
/// This is a convenience wrapper that uses the compact mode and
/// accepts a selection callback.
#[component]
pub fn WizardVariantCard(
    variant: LicenseVariant,
    #[prop(into)] on_select: Callback<()>,
) -> impl IntoView {
    let variant_clone = variant.clone();

    // Wrap the callback to convert from LicenseVariant to ()
    let on_action = Callback::new(move |_: LicenseVariant| {
        on_select.run(());
    });

    view! {
        <BaseVariantCard
            variant=variant_clone
            mode=VariantCardMode::Compact
            on_action=on_action
        />
    }
}
