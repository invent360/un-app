//! Wizard state management
//!
//! This module provides state management for the license claim wizard,
//! integrating with ember-fx's WizardStage trait for type-safe navigation.
//!
//! ## Wizard Flow
//!
//! The claim wizard follows a 5-stage journey:
//!
//! 1. **EconomicsReview** - Show 50/40/10 split transparency
//! 2. **Review** - Review license details and accept terms
//! 3. **Reserve** - Atomic reservation (license locked for user)
//! 4. **Claim** - Show license key after copy confirmation
//! 5. **WhatNext** - Guides, tasks, and next steps
//!
//! Post-claim touchpoints (D1, D3, D7, D30) are handled separately
//! based on claim timestamp.

use leptos::prelude::*;
use crate::types::{LicenseVariant, AvailabilityStatus};

/// Wizard stages - enhanced journey flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum ClaimWizardStage {
    /// Step 1: Economics transparency (50/40/10 split explanation)
    #[default]
    EconomicsReview,
    /// Step 2: Review license details and accept terms
    Review,
    /// Step 3: Atomic reservation in progress (license locked)
    Reserve,
    /// Step 4: Claim the license (shows license key after copy)
    Claim,
    /// Step 5: What's next - guides, tasks, warnings
    WhatNext,
}

/// Journey touchpoint stages (post-claim engagement)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JourneyTouchpoint {
    /// Day 1: Initial setup and activation
    D1,
    /// Day 3: First earnings check
    D3,
    /// Day 7: Week one review
    D7,
    /// Day 30: Monthly milestone
    D30,
}

impl JourneyTouchpoint {
    /// Get the day number for this touchpoint
    pub fn day(&self) -> u32 {
        match self {
            Self::D1 => 1,
            Self::D3 => 3,
            Self::D7 => 7,
            Self::D30 => 30,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::D1 => "Day 1: Getting Started",
            Self::D3 => "Day 3: First Earnings",
            Self::D7 => "Week 1: Review",
            Self::D30 => "Month 1: Milestone",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::D1 => "Set up your license and start earning",
            Self::D3 => "Check your first earnings and device status",
            Self::D7 => "Review your first week's performance",
            Self::D30 => "Celebrate your first month milestone",
        }
    }

    /// Calculate touchpoint from days since claim
    pub fn from_days_since_claim(days: u32) -> Option<Self> {
        match days {
            0..=1 => Some(Self::D1),
            2..=3 => Some(Self::D3),
            4..=7 => Some(Self::D7),
            8..=30 => Some(Self::D30),
            _ => None,
        }
    }

    /// Get all touchpoints in order
    pub fn all() -> Vec<Self> {
        vec![Self::D1, Self::D3, Self::D7, Self::D30]
    }
}

/// Implement ember-fx WizardStage trait for type-safe wizard navigation
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
impl ember_fx_components::form::WizardStage for ClaimWizardStage {
    fn display_name(&self) -> &'static str {
        match self {
            Self::EconomicsReview => "Economics",
            Self::Review => "Review",
            Self::Reserve => "Reserve",
            Self::Claim => "Claim",
            Self::WhatNext => "What Next",
        }
    }

    fn all_stages() -> Vec<Self> {
        vec![
            Self::EconomicsReview,
            Self::Review,
            Self::Reserve,
            Self::Claim,
            Self::WhatNext,
        ]
    }

    fn is_optional(&self) -> bool {
        false
    }
}

impl ClaimWizardStage {
    /// Get the stage number (1-based for display)
    pub fn number(&self) -> u8 {
        match self {
            Self::EconomicsReview => 1,
            Self::Review => 2,
            Self::Reserve => 3,
            Self::Claim => 4,
            Self::WhatNext => 5,
        }
    }

    /// Total number of stages
    pub fn total() -> u8 {
        5
    }

    /// Progress percentage (0-100)
    pub fn progress(&self) -> f64 {
        match self {
            Self::EconomicsReview => 0.0,
            Self::Review => 25.0,
            Self::Reserve => 50.0,
            Self::Claim => 75.0,
            Self::WhatNext => 100.0,
        }
    }

    /// Get next stage
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::EconomicsReview => Some(Self::Review),
            Self::Review => Some(Self::Reserve),
            Self::Reserve => Some(Self::Claim),
            Self::Claim => Some(Self::WhatNext),
            Self::WhatNext => None,
        }
    }

    /// Get previous stage
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::EconomicsReview => None,                    // First stage
            Self::Review => Some(Self::EconomicsReview),
            Self::Reserve => Some(Self::Review),
            Self::Claim => None,                              // Cannot go back after claim
            Self::WhatNext => None,                           // Cannot go back
        }
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.prev().is_some()
    }

    /// Check if this is a post-claim stage (shows success content)
    pub fn is_post_claim(&self) -> bool {
        matches!(self, Self::Claim | Self::WhatNext)
    }

    /// Check if this is the reservation stage
    pub fn is_reserving(&self) -> bool {
        matches!(self, Self::Reserve)
    }

    /// Check if this is the final stage
    pub fn is_final(&self) -> bool {
        matches!(self, Self::WhatNext)
    }

    /// Check if this is the first stage
    pub fn is_first(&self) -> bool {
        matches!(self, Self::EconomicsReview)
    }

    /// Check if this stage shows economics information
    pub fn shows_economics(&self) -> bool {
        matches!(self, Self::EconomicsReview)
    }

    /// Get stage description for accessibility
    pub fn description(&self) -> &'static str {
        match self {
            Self::EconomicsReview => "Review how earnings are split: 50% to you, 40% to platform, 10% to referrer",
            Self::Review => "Review your license details and accept terms",
            Self::Reserve => "Your license is being reserved exclusively for you",
            Self::Claim => "Claim your license and receive your license key",
            Self::WhatNext => "Get started with guides, tasks, and next steps",
        }
    }

    /// Get short title for progress display
    pub fn short_title(&self) -> &'static str {
        match self {
            Self::EconomicsReview => "Economics",
            Self::Review => "Terms",
            Self::Reserve => "Reserve",
            Self::Claim => "License",
            Self::WhatNext => "Setup",
        }
    }
}

/// Wizard state context
#[derive(Clone)]
pub struct ClaimWizardState {
    /// Current wizard stage
    pub stage: RwSignal<ClaimWizardStage>,
    /// Selected license variant (for display purposes)
    pub selected_variant: RwSignal<Option<LicenseVariant>>,
    /// Lease code entered by user
    pub lease_code: RwSignal<String>,
    /// Whether terms are accepted
    pub terms_accepted: RwSignal<bool>,
    /// Claimed license ID after successful claim
    pub claimed_license_id: RwSignal<Option<String>>,
    /// Claimed license key after successful claim
    pub claimed_license_key: RwSignal<Option<String>>,
    /// Whether claim is in progress
    pub is_claiming: RwSignal<bool>,
    /// Whether claim confirmation is in progress (when user copies the key)
    pub is_confirming: RwSignal<bool>,
    /// Whether the license has been confirmed (claim finalized)
    pub is_confirmed: RwSignal<bool>,
    /// Error message if claim failed
    pub error: RwSignal<Option<String>>,
    /// Whether wizard is open/visible
    pub is_open: RwSignal<bool>,
    /// Available license variants (loaded at app level)
    pub variants: RwSignal<Vec<LicenseVariant>>,
    /// Whether user has a referral code checkbox is checked
    pub has_referral: RwSignal<bool>,
    /// Referral code entered by user (optional)
    pub referral_code: RwSignal<String>,
    /// Referral code to apply during claim confirmation
    pub referral_code_to_apply: RwSignal<Option<String>>,
    /// Whether referral code is valid (None = not checked, Some(true/false) = result)
    pub referral_valid: RwSignal<Option<bool>>,
    /// Whether referral validation is in progress
    pub referral_checking: RwSignal<bool>,
    /// Referral name to display when valid
    pub referral_name: RwSignal<Option<String>>,
    /// Global availability status across all variants
    pub global_availability: RwSignal<AvailabilityStatus>,
    /// Whether variants are being loaded
    pub is_loading_variants: RwSignal<bool>,
}

impl ClaimWizardState {
    /// Create new wizard state
    pub fn new() -> Self {
        Self {
            stage: RwSignal::new(ClaimWizardStage::EconomicsReview),
            selected_variant: RwSignal::new(None),
            lease_code: RwSignal::new(String::new()),
            terms_accepted: RwSignal::new(false),
            claimed_license_id: RwSignal::new(None),
            claimed_license_key: RwSignal::new(None),
            is_claiming: RwSignal::new(false),
            is_confirming: RwSignal::new(false),
            is_confirmed: RwSignal::new(false),
            error: RwSignal::new(None),
            is_open: RwSignal::new(false),
            variants: RwSignal::new(Vec::new()),
            has_referral: RwSignal::new(false),
            referral_code: RwSignal::new(String::new()),
            referral_code_to_apply: RwSignal::new(None),
            referral_valid: RwSignal::new(None),
            referral_checking: RwSignal::new(false),
            referral_name: RwSignal::new(None),
            global_availability: RwSignal::new(AvailabilityStatus::Available),
            is_loading_variants: RwSignal::new(false),
        }
    }

    /// Reset wizard to initial state
    pub fn reset(&self) {
        self.stage.set(ClaimWizardStage::EconomicsReview);
        self.selected_variant.set(None);
        self.lease_code.set(String::new());
        self.terms_accepted.set(false);
        self.claimed_license_id.set(None);
        self.claimed_license_key.set(None);
        self.is_claiming.set(false);
        self.is_confirming.set(false);
        self.is_confirmed.set(false);
        self.error.set(None);
        self.has_referral.set(false);
        self.referral_code.set(String::new());
        self.referral_code_to_apply.set(None);
        self.referral_valid.set(None);
        self.referral_checking.set(false);
        self.referral_name.set(None);
        // Don't reset global_availability - it's loaded once
        // Don't reset is_loading_variants - managed by variants loading
    }

    /// Set available variants (legacy - kept for compatibility)
    pub fn set_variants(&self, variants: Vec<LicenseVariant>) {
        self.variants.set(variants);
        self.is_loading_variants.set(false);
    }

    /// Set the global availability status directly
    pub fn set_availability(&self, available: bool) {
        if available {
            self.global_availability.set(AvailabilityStatus::Available);
        } else {
            // Could be AllClaimed or NoneInSystem - we'll use AllClaimed as default
            // since NoneInSystem should be rare in practice
            self.global_availability.set(AvailabilityStatus::AllClaimed);
        }
        self.is_loading_variants.set(false);
    }

    /// Set availability to "loading" state
    pub fn set_availability_loading(&self) {
        self.is_loading_variants.set(true);
    }

    /// Compute the global availability status from all variants (legacy)
    fn compute_global_availability(variants: &[LicenseVariant]) -> AvailabilityStatus {
        if variants.is_empty() {
            return AvailabilityStatus::NoneInSystem;
        }

        // Check if any variant has licenses available
        let any_available = variants.iter().any(|v| v.availability_status == AvailabilityStatus::Available);
        if any_available {
            return AvailabilityStatus::Available;
        }

        // Check if all variants have no licenses in system
        let all_none = variants.iter().all(|v| v.availability_status == AvailabilityStatus::NoneInSystem);
        if all_none {
            return AvailabilityStatus::NoneInSystem;
        }

        // Check if any has claimed licenses (meaning they exist but are claimed)
        let any_claimed = variants.iter().any(|v| v.availability_status == AvailabilityStatus::AllClaimed);
        if any_claimed {
            return AvailabilityStatus::AllClaimed;
        }

        // Otherwise, must be all expired
        AvailabilityStatus::AllExpired
    }

    /// Open the wizard and start availability check
    pub fn open(&self) {
        self.reset();
        // Set loading state - caller should check availability and call set_availability
        self.is_loading_variants.set(true);
        self.is_open.set(true);
    }

    /// Open wizard with a selected variant
    pub fn open_with_variant(&self, variant: LicenseVariant) {
        self.reset();
        self.selected_variant.set(Some(variant));
        self.is_open.set(true);
    }

    /// Close wizard
    pub fn close(&self) {
        self.is_open.set(false);
    }

    /// Go to next stage
    pub fn next_stage(&self) {
        if let Some(next) = self.stage.get().next() {
            self.stage.set(next);
        }
    }

    /// Go to previous stage
    pub fn prev_stage(&self) {
        if let Some(prev) = self.stage.get().prev() {
            self.stage.set(prev);
        }
    }

    /// Go to specific stage
    pub fn go_to_stage(&self, stage: ClaimWizardStage) {
        self.stage.set(stage);
    }

    /// Select a variant
    pub fn select_variant(&self, variant: LicenseVariant) {
        self.selected_variant.set(Some(variant));
    }

    /// Start the reservation process (transition to Reserve stage)
    pub fn start_reservation(&self) {
        self.is_claiming.set(true);
        self.error.set(None);
        self.stage.set(ClaimWizardStage::Reserve);
    }

    /// Set reservation result (license reserved but not yet confirmed)
    pub fn set_reservation_success(&self, license_id: Option<String>, license_key: Option<String>, referral_code: Option<String>) {
        self.claimed_license_id.set(license_id);
        self.claimed_license_key.set(license_key);
        self.referral_code_to_apply.set(referral_code);
        self.is_claiming.set(false);
        self.is_confirmed.set(false);
        self.error.set(None);
        // Move from Reserve to Claim stage
        self.stage.set(ClaimWizardStage::Claim);
    }

    /// Set claim result (legacy - use set_reservation_success for new flow)
    pub fn set_claim_success(&self, license_id: Option<String>, license_key: Option<String>) {
        self.claimed_license_id.set(license_id);
        self.claimed_license_key.set(license_key);
        self.is_claiming.set(false);
        self.error.set(None);
        self.stage.set(ClaimWizardStage::Claim);
    }

    /// Proceed from EconomicsReview to Review
    pub fn accept_economics(&self) {
        self.stage.set(ClaimWizardStage::Review);
    }

    /// Set claim error
    pub fn set_claim_error(&self, error: String) {
        self.error.set(Some(error));
        self.is_claiming.set(false);
        self.is_confirming.set(false);
    }

    /// Start claiming process
    pub fn start_claiming(&self) {
        self.is_claiming.set(true);
        self.error.set(None);
    }

    /// Start confirming process (when user copies the key)
    pub fn start_confirming(&self) {
        self.is_confirming.set(true);
        self.error.set(None);
    }

    /// Set confirmation success (license claim finalized)
    pub fn set_confirm_success(&self) {
        self.is_confirming.set(false);
        self.is_confirmed.set(true);
        self.error.set(None);
    }

    /// Get current progress percentage
    pub fn progress_percentage(&self) -> f64 {
        self.stage.get().progress()
    }

    /// Check if wizard can proceed to next stage
    pub fn can_proceed(&self) -> bool {
        match self.stage.get() {
            ClaimWizardStage::EconomicsReview => true, // Always can proceed after viewing economics
            ClaimWizardStage::Review => {
                self.terms_accepted.get()
            }
            ClaimWizardStage::Reserve => false, // Auto-proceeds on success
            ClaimWizardStage::Claim => self.is_confirmed.get(), // Must confirm to proceed
            ClaimWizardStage::WhatNext => false, // Final stage
        }
    }

    /// Get the journey step number (1-10) for marketing plan tracking
    pub fn journey_step(&self) -> u8 {
        match self.stage.get() {
            ClaimWizardStage::EconomicsReview => 1,
            ClaimWizardStage::Review => 2,
            ClaimWizardStage::Reserve => 3,
            ClaimWizardStage::Claim => 4,
            ClaimWizardStage::WhatNext => 5,
        }
    }

    /// Check if wizard is in a loading/processing state
    pub fn is_processing(&self) -> bool {
        self.is_claiming.get() || self.is_confirming.get() || self.is_loading_variants.get()
    }
}

impl Default for ClaimWizardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide wizard context and return the state
pub fn provide_wizard_context() -> ClaimWizardState {
    let state = ClaimWizardState::new();
    provide_context(state.clone());
    state
}

/// Use wizard context (returns None if context not available)
pub fn use_wizard_state() -> Option<ClaimWizardState> {
    use_context::<ClaimWizardState>()
}
