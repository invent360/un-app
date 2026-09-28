//! Progress bar showing claimed/remaining licenses
//!
//! Displays a visual indicator of license availability with
//! urgency levels based on remaining inventory.

use leptos::prelude::*;
use crate::hooks::t;

/// Urgency level based on claimed percentage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrgencyLevel {
    /// Low urgency (< 70% claimed)
    Low,
    /// Medium urgency (70-90% claimed)
    Medium,
    /// High urgency (> 90% claimed)
    High,
}

impl UrgencyLevel {
    /// Calculate urgency level from progress percentage
    pub fn from_progress(progress: f64) -> Self {
        if progress >= 90.0 {
            Self::High
        } else if progress >= 70.0 {
            Self::Medium
        } else {
            Self::Low
        }
    }

    /// Get CSS class for this urgency level
    pub fn class(&self) -> &'static str {
        match self {
            Self::Low => "urgency-low",
            Self::Medium => "urgency-medium",
            Self::High => "urgency-high",
        }
    }
}

/// Progress bar showing claimed/remaining licenses
#[component]
pub fn UrgencyBar(
    /// Progress percentage (0-100)
    progress: f64,
    /// Number of remaining licenses
    remaining: i32,
    /// Total number of licenses
    total: i32,
) -> impl IntoView {
    let urgency = UrgencyLevel::from_progress(progress);
    let progress_class = urgency.class();
    let claimed = total - remaining;
    let clamped_progress = progress.min(100.0);

    view! {
        <div class="urgency-bar">
            <div class="progress-container">
                <div
                    class=format!("progress-fill {}", progress_class)
                    style=format!("width: {}%", clamped_progress)
                />
            </div>
            <div class="progress-labels">
                <span class="claimed">{claimed} " " {move || t("licenses.claimed")}</span>
                <span class="remaining">{remaining} " " {move || t("licenses.left")}</span>
            </div>
        </div>
    }
}
