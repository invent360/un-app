//! Suitability stepper state management for R5-13
//!
//! Manages the pre-claim suitability/eligibility flow:
//! 1. Country/Device - Detect country, verify device compatibility
//! 2. Connectivity - Check existing data/power situation
//! 3. Net Benefit - Show realistic earnings vs costs
//! 4. Terms & Consent - Accept terms, optional marketing
//!
//! State is persisted to localStorage for session resumption.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Suitability stepper stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub enum SuitabilityStage {
    /// Step 1: Country detection and device compatibility
    #[default]
    CountryDevice,
    /// Step 2: Connectivity/data situation
    Connectivity,
    /// Step 3: Net benefit calculator
    NetBenefit,
    /// Step 4: Terms acceptance and consent
    TermsConsent,
    /// Completed - ready to proceed to claim
    Complete,
}

impl SuitabilityStage {
    /// Get stage number (1-based)
    pub fn number(&self) -> u8 {
        match self {
            Self::CountryDevice => 1,
            Self::Connectivity => 2,
            Self::NetBenefit => 3,
            Self::TermsConsent => 4,
            Self::Complete => 5,
        }
    }

    /// Total number of user-facing stages (excluding Complete)
    pub fn total() -> u8 {
        4
    }

    /// Progress percentage (0-100)
    pub fn progress(&self) -> f64 {
        match self {
            Self::CountryDevice => 0.0,
            Self::Connectivity => 25.0,
            Self::NetBenefit => 50.0,
            Self::TermsConsent => 75.0,
            Self::Complete => 100.0,
        }
    }

    /// Get next stage
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::CountryDevice => Some(Self::Connectivity),
            Self::Connectivity => Some(Self::NetBenefit),
            Self::NetBenefit => Some(Self::TermsConsent),
            Self::TermsConsent => Some(Self::Complete),
            Self::Complete => None,
        }
    }

    /// Get previous stage
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::CountryDevice => None,
            Self::Connectivity => Some(Self::CountryDevice),
            Self::NetBenefit => Some(Self::Connectivity),
            Self::TermsConsent => Some(Self::NetBenefit),
            Self::Complete => Some(Self::TermsConsent),
        }
    }

    /// Display name for this stage
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::CountryDevice => "Location & Device",
            Self::Connectivity => "Connectivity",
            Self::NetBenefit => "Earnings Estimate",
            Self::TermsConsent => "Terms",
            Self::Complete => "Ready",
        }
    }

    /// Description for this stage
    pub fn description(&self) -> &'static str {
        match self {
            Self::CountryDevice => "Verify your location and device compatibility",
            Self::Connectivity => "Tell us about your internet connection",
            Self::NetBenefit => "See your estimated monthly earnings",
            Self::TermsConsent => "Review and accept the terms of service",
            Self::Complete => "You're ready to claim your license",
        }
    }
}

/// Device type for suitability check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    AndroidPhone,
    AndroidTablet,
    IPhone,
    IPad,
    Desktop,
    Other,
}

impl DeviceType {
    pub fn is_supported(&self) -> bool {
        matches!(self, Self::AndroidPhone | Self::AndroidTablet)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AndroidPhone => "Android Phone",
            Self::AndroidTablet => "Android Tablet",
            Self::IPhone => "iPhone",
            Self::IPad => "iPad",
            Self::Desktop => "Desktop/Laptop",
            Self::Other => "Other Device",
        }
    }
}

/// Connectivity quality for earnings estimate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectivityQuality {
    /// Good stable connection (WiFi + reliable mobile data)
    Good,
    /// Average connection (occasional drops)
    Average,
    /// Limited connection (data caps, frequent drops)
    Limited,
}

impl ConnectivityQuality {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Good => "Good (stable WiFi + mobile data)",
            Self::Average => "Average (occasional interruptions)",
            Self::Limited => "Limited (data caps or frequent drops)",
        }
    }

    /// Multiplier for earnings estimate
    pub fn earnings_multiplier(&self) -> f64 {
        match self {
            Self::Good => 1.0,
            Self::Average => 0.7,
            Self::Limited => 0.4,
        }
    }
}

/// Power situation for device availability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerSituation {
    /// Device can stay plugged in most of the time
    AlwaysOn,
    /// Device charged regularly but not always plugged in
    Regular,
    /// Power is limited or unreliable
    Limited,
}

impl PowerSituation {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AlwaysOn => "Always connected to power",
            Self::Regular => "Charged regularly",
            Self::Limited => "Limited power access",
        }
    }

    /// Hours per day estimate
    pub fn hours_per_day(&self) -> f64 {
        match self {
            Self::AlwaysOn => 20.0,
            Self::Regular => 12.0,
            Self::Limited => 6.0,
        }
    }
}

/// Persisted suitability answers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuitabilityAnswers {
    /// Detected/selected country code
    pub country_code: Option<String>,
    /// Detected/selected device type
    pub device_type: Option<DeviceType>,
    /// Connectivity quality
    pub connectivity: Option<ConnectivityQuality>,
    /// Power situation
    pub power: Option<PowerSituation>,
    /// Terms accepted
    pub terms_accepted: bool,
    /// Marketing consent
    pub marketing_consent: bool,
    /// Timestamp of last update
    pub updated_at: Option<String>,
}

impl SuitabilityAnswers {
    /// Calculate estimated monthly earnings in USD
    pub fn estimated_earnings(&self) -> Option<(f64, f64)> {
        let connectivity = self.connectivity?;
        let power = self.power?;

        // Base earnings: $2-5/month for full-time operation
        let base_min = 2.0;
        let base_max = 5.0;

        // Adjust for connectivity and power
        let hours_ratio = power.hours_per_day() / 24.0;
        let connectivity_mult = connectivity.earnings_multiplier();

        let min_earnings = base_min * hours_ratio * connectivity_mult;
        let max_earnings = base_max * hours_ratio * connectivity_mult;

        Some((min_earnings, max_earnings))
    }

    /// Check if all required fields are filled
    pub fn is_complete(&self) -> bool {
        self.country_code.is_some()
            && self.device_type.is_some()
            && self.connectivity.is_some()
            && self.power.is_some()
            && self.terms_accepted
    }

    /// Check if device is eligible
    pub fn is_device_eligible(&self) -> bool {
        self.device_type.map(|d| d.is_supported()).unwrap_or(false)
    }
}

/// Storage key for localStorage
const STORAGE_KEY: &str = "uno_suitability_answers";

/// Suitability stepper state
#[derive(Clone)]
pub struct SuitabilityState {
    /// Current stage
    pub stage: RwSignal<SuitabilityStage>,
    /// User answers
    pub answers: RwSignal<SuitabilityAnswers>,
    /// Whether stepper is open
    pub is_open: RwSignal<bool>,
    /// Error message
    pub error: RwSignal<Option<String>>,
    /// Whether checking eligibility
    pub is_checking: RwSignal<bool>,
}

impl SuitabilityState {
    /// Create new suitability state
    pub fn new() -> Self {
        Self {
            stage: RwSignal::new(SuitabilityStage::CountryDevice),
            answers: RwSignal::new(SuitabilityAnswers::default()),
            is_open: RwSignal::new(false),
            error: RwSignal::new(None),
            is_checking: RwSignal::new(false),
        }
    }

    /// Load state from localStorage
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    pub fn load_from_storage(&self) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(data)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(answers) = serde_json::from_str::<SuitabilityAnswers>(&data) {
                        self.answers.set(answers);
                        // Determine stage based on completed answers
                        self.restore_stage();
                    }
                }
            }
        }
    }

    /// Save state to localStorage
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    pub fn save_to_storage(&self) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let mut answers = self.answers.get();
                answers.updated_at = Some(chrono::Utc::now().to_rfc3339());
                if let Ok(data) = serde_json::to_string(&answers) {
                    let _ = storage.set_item(STORAGE_KEY, &data);
                }
            }
        }
    }

    /// Restore stage based on completed answers
    fn restore_stage(&self) {
        let answers = self.answers.get();

        if answers.terms_accepted {
            self.stage.set(SuitabilityStage::Complete);
        } else if answers.connectivity.is_some() && answers.power.is_some() {
            self.stage.set(SuitabilityStage::TermsConsent);
        } else if answers.country_code.is_some() && answers.device_type.is_some() {
            self.stage.set(SuitabilityStage::Connectivity);
        } else {
            self.stage.set(SuitabilityStage::CountryDevice);
        }
    }

    /// Open the stepper
    pub fn open(&self) {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        self.load_from_storage();
        self.is_open.set(true);
    }

    /// Close the stepper
    pub fn close(&self) {
        self.is_open.set(false);
    }

    /// Go to next stage
    pub fn next_stage(&self) {
        if let Some(next) = self.stage.get().next() {
            self.stage.set(next);
            #[cfg(any(feature = "csr", feature = "hydrate"))]
            self.save_to_storage();
        }
    }

    /// Go to previous stage
    pub fn prev_stage(&self) {
        if let Some(prev) = self.stage.get().prev() {
            self.stage.set(prev);
        }
    }

    /// Update country code
    pub fn set_country(&self, code: String) {
        self.answers.update(|a| a.country_code = Some(code));
    }

    /// Update device type
    pub fn set_device(&self, device: DeviceType) {
        self.answers.update(|a| a.device_type = Some(device));
    }

    /// Update connectivity
    pub fn set_connectivity(&self, quality: ConnectivityQuality) {
        self.answers.update(|a| a.connectivity = Some(quality));
    }

    /// Update power situation
    pub fn set_power(&self, power: PowerSituation) {
        self.answers.update(|a| a.power = Some(power));
    }

    /// Set terms accepted
    pub fn set_terms_accepted(&self, accepted: bool) {
        self.answers.update(|a| a.terms_accepted = accepted);
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        self.save_to_storage();
    }

    /// Set marketing consent
    pub fn set_marketing_consent(&self, consent: bool) {
        self.answers.update(|a| a.marketing_consent = consent);
    }

    /// Check if can proceed to next stage
    pub fn can_proceed(&self) -> bool {
        let answers = self.answers.get();
        match self.stage.get() {
            SuitabilityStage::CountryDevice => {
                answers.country_code.is_some() && answers.device_type.is_some()
            }
            SuitabilityStage::Connectivity => {
                answers.connectivity.is_some() && answers.power.is_some()
            }
            SuitabilityStage::NetBenefit => true, // Info only
            SuitabilityStage::TermsConsent => answers.terms_accepted,
            SuitabilityStage::Complete => true,
        }
    }

    /// Reset state
    pub fn reset(&self) {
        self.stage.set(SuitabilityStage::CountryDevice);
        self.answers.set(SuitabilityAnswers::default());
        self.error.set(None);
        self.is_checking.set(false);

        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item(STORAGE_KEY);
                }
            }
        }
    }
}

impl Default for SuitabilityState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide suitability context
pub fn provide_suitability_context() -> SuitabilityState {
    let state = SuitabilityState::new();
    provide_context(state.clone());
    state
}

/// Use suitability context
pub fn use_suitability_state() -> Option<SuitabilityState> {
    use_context::<SuitabilityState>()
}
