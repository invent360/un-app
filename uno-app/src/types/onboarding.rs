//! R3-13: Onboarding journey state machine
//!
//! Defines the states and transitions for user onboarding:
//! Landing → Eligibility → Economics → Account → Setup → Reservation → Activation → Active

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Progress within eligibility checks
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EligibilityProgress {
    /// Country check completed
    pub country_verified: bool,
    /// Device check completed
    pub device_verified: bool,
    /// Referral code validated (if provided)
    pub referral_validated: Option<bool>,
}

impl EligibilityProgress {
    pub fn is_complete(&self) -> bool {
        self.country_verified && self.device_verified
    }
}

/// Setup steps after account creation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupStep {
    /// Profile information
    Profile,
    /// Device configuration
    Device,
    /// Notification preferences
    Notifications,
    /// Final confirmation
    Confirmation,
}

impl SetupStep {
    pub fn next(&self) -> Option<SetupStep> {
        match self {
            SetupStep::Profile => Some(SetupStep::Device),
            SetupStep::Device => Some(SetupStep::Notifications),
            SetupStep::Notifications => Some(SetupStep::Confirmation),
            SetupStep::Confirmation => None,
        }
    }

    pub fn progress_percent(&self) -> u8 {
        match self {
            SetupStep::Profile => 25,
            SetupStep::Device => 50,
            SetupStep::Notifications => 75,
            SetupStep::Confirmation => 100,
        }
    }
}

/// Main onboarding state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum OnboardingState {
    /// Initial landing page
    Landing,

    /// Checking eligibility (country, device, referral)
    Eligibility {
        progress: EligibilityProgress,
        referral_code: Option<String>,
    },

    /// Reviewing economics/terms
    Economics {
        consent_version: Uuid,
        accepted_at: Option<DateTime<Utc>>,
    },

    /// Account creation/login
    Account {
        provider: String,
    },

    /// Post-account setup steps
    Setup {
        step: SetupStep,
    },

    /// License reserved, waiting for activation
    Reservation {
        license_id: Uuid,
        expires_at: DateTime<Utc>,
    },

    /// Activating license
    Activation {
        license_id: Uuid,
    },

    /// Fully onboarded and active
    Active {
        license_id: Uuid,
        activated_at: DateTime<Utc>,
    },

    /// Onboarding was paused (can resume)
    Paused {
        previous_state: Box<OnboardingState>,
        paused_at: DateTime<Utc>,
        reason: Option<String>,
    },

    /// Onboarding failed (cannot continue)
    Failed {
        error_code: String,
        error_message: String,
        failed_at: DateTime<Utc>,
    },
}

impl Default for OnboardingState {
    fn default() -> Self {
        Self::Landing
    }
}

impl OnboardingState {
    /// Check if this state can be resumed after disconnect
    pub fn can_resume(&self) -> bool {
        matches!(
            self,
            Self::Eligibility { .. }
                | Self::Economics { .. }
                | Self::Setup { .. }
                | Self::Reservation { .. }
                | Self::Paused { .. }
        )
    }

    /// Check if user can navigate back from this state
    pub fn can_go_back(&self) -> bool {
        matches!(
            self,
            Self::Eligibility { .. }
                | Self::Economics { .. }
                | Self::Account { .. }
                | Self::Setup { .. }
        )
    }

    /// Get the previous state for back navigation
    pub fn previous_state(&self) -> Option<OnboardingState> {
        match self {
            Self::Eligibility { .. } => Some(Self::Landing),
            Self::Economics { consent_version, .. } => Some(Self::Eligibility {
                progress: EligibilityProgress::default(),
                referral_code: None,
            }),
            Self::Account { .. } => Some(Self::Economics {
                consent_version: Uuid::nil(),
                accepted_at: None,
            }),
            Self::Setup { step } => {
                if *step == SetupStep::Profile {
                    Some(Self::Account { provider: String::new() })
                } else {
                    // Go to previous setup step
                    let prev_step = match step {
                        SetupStep::Device => SetupStep::Profile,
                        SetupStep::Notifications => SetupStep::Device,
                        SetupStep::Confirmation => SetupStep::Notifications,
                        SetupStep::Profile => return None,
                    };
                    Some(Self::Setup { step: prev_step })
                }
            }
            _ => None,
        }
    }

    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Active { .. } | Self::Failed { .. })
    }

    /// Check if user is earning (only in Active state with valid license)
    pub fn is_earning(&self) -> bool {
        matches!(self, Self::Active { .. })
    }

    /// Get progress percentage (0-100)
    pub fn progress_percent(&self) -> u8 {
        match self {
            Self::Landing => 0,
            Self::Eligibility { progress, .. } => {
                let mut pct = 10;
                if progress.country_verified { pct += 5; }
                if progress.device_verified { pct += 5; }
                pct
            }
            Self::Economics { accepted_at, .. } => {
                if accepted_at.is_some() { 30 } else { 25 }
            }
            Self::Account { .. } => 40,
            Self::Setup { step } => 40 + (step.progress_percent() as u8 / 4),
            Self::Reservation { .. } => 70,
            Self::Activation { .. } => 85,
            Self::Active { .. } => 100,
            Self::Paused { previous_state, .. } => previous_state.progress_percent(),
            Self::Failed { .. } => 0,
        }
    }

    /// Get the funnel stage name for analytics
    pub fn funnel_stage(&self) -> &'static str {
        match self {
            Self::Landing => "visit",
            Self::Eligibility { .. } => "view_info",
            Self::Economics { accepted_at: None, .. } => "economics_review",
            Self::Economics { accepted_at: Some(_), .. } => "terms_accepted",
            Self::Account { .. } => "start_claim",
            Self::Setup { .. } => "start_claim",
            Self::Reservation { .. } => "reserved",
            Self::Activation { .. } => "claimed",
            Self::Active { .. } => "activated",
            Self::Paused { previous_state, .. } => previous_state.funnel_stage(),
            Self::Failed { .. } => "failed",
        }
    }
}

/// Onboarding progress stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Option<Uuid>,
    pub state: OnboardingState,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl OnboardingProgress {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            session_id: None,
            state: OnboardingState::Landing,
            started_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn transition(&mut self, new_state: OnboardingState) {
        self.state = new_state;
        self.updated_at = Utc::now();

        if self.state.is_terminal() && matches!(self.state, OnboardingState::Active { .. }) {
            self.completed_at = Some(Utc::now());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onboarding_state_can_resume() {
        assert!(!OnboardingState::Landing.can_resume());
        assert!(OnboardingState::Eligibility {
            progress: EligibilityProgress::default(),
            referral_code: None,
        }.can_resume());
        assert!(OnboardingState::Setup { step: SetupStep::Profile }.can_resume());
        assert!(!OnboardingState::Active {
            license_id: Uuid::new_v4(),
            activated_at: Utc::now(),
        }.can_resume());
    }

    #[test]
    fn test_onboarding_progress_percent() {
        assert_eq!(OnboardingState::Landing.progress_percent(), 0);
        assert_eq!(OnboardingState::Active {
            license_id: Uuid::new_v4(),
            activated_at: Utc::now(),
        }.progress_percent(), 100);
    }

    #[test]
    fn test_setup_step_next() {
        assert_eq!(SetupStep::Profile.next(), Some(SetupStep::Device));
        assert_eq!(SetupStep::Confirmation.next(), None);
    }

    #[test]
    fn test_is_earning() {
        assert!(!OnboardingState::Reservation {
            license_id: Uuid::new_v4(),
            expires_at: Utc::now(),
        }.is_earning());

        assert!(OnboardingState::Active {
            license_id: Uuid::new_v4(),
            activated_at: Utc::now(),
        }.is_earning());
    }
}
