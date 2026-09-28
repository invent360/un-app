//! License variant display card
//!
//! This module re-exports the unified VariantCard from features/license
//! and provides a convenience wrapper for the license page context.

use leptos::prelude::*;
use crate::types::LicenseVariant;
use crate::features::license::{VariantCard as BaseVariantCard, VariantCardMode};
use crate::components::wizard::use_wizard_state;

/// Variant card for license page display
///
/// This is a convenience wrapper that automatically integrates with
/// the wizard context to open the claim wizard when clicked.
#[component]
pub fn VariantCard(variant: LicenseVariant) -> impl IntoView {
    let variant_clone = variant.clone();

    // Get the wizard state from context
    let wizard_state = use_wizard_state();

    // Handler to open the wizard with this variant
    let on_action = Callback::new(move |v: LicenseVariant| {
        if let Some(state) = wizard_state.as_ref() {
            state.open_with_variant(v);
        }
    });

    view! {
        <BaseVariantCard
            variant=variant_clone
            mode=VariantCardMode::Full
            on_action=on_action
        />
    }
}
