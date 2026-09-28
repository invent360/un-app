//! Wizard progress bar component - Outlined style with vertical labels

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::ClaimWizardStage;

/// Progress bar for wizard stages - Outlined + Vertical Labels + Thick Lines
#[component]
pub fn WizardProgressBar(
    #[prop(into)] stage: Signal<ClaimWizardStage>,
) -> impl IntoView {
    let total_steps = ClaimWizardStage::total();

    view! {
        <div class="wizard-progress">
            <div class="progress-steps">
                {(1..=total_steps).map(|step| {
                    let step_label = match step {
                        1 => "wizard.progress.review",
                        2 => "wizard.progress.claim",
                        3 => "wizard.progress.what_next",
                        _ => "wizard.progress.step",
                    };

                    view! {
                        <div
                            class="progress-step"
                            class:completed=move || step < stage.get().number()
                            class:active=move || step == stage.get().number()
                        >
                            // Row with lines and circle
                            <div class="step-row">
                                // Left connector line
                                <div
                                    class="connector-line left"
                                    class:completed=move || step <= stage.get().number()
                                    class:hidden=move || step == 1
                                />

                                // Circle indicator
                                <div class="step-indicator">
                                    {move || {
                                        if step < stage.get().number() {
                                            // Completed - show checkmark
                                            view! {
                                                <svg viewBox="0 0 24 24" class="check-icon">
                                                    <path fill="currentColor" d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            // Current or pending - show number
                                            view! {
                                                <span class="step-number">{step}</span>
                                            }.into_any()
                                        }
                                    }}
                                </div>

                                // Right connector line
                                <div
                                    class="connector-line right"
                                    class:completed=move || step < stage.get().number()
                                    class:hidden=move || step == total_steps
                                />
                            </div>

                            // Label below
                            <span class="step-label">
                                {move || t(step_label)}
                            </span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
