//! Activate stage component - activate license and start earning

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;
use crate::components::wizard::components::{StepCarousel, SlideConfig};

/// Activate stage - activate license and start earning
#[component]
pub fn ActivateStage() -> impl IntoView {
    let state = use_wizard_state().expect("ActivateStage must be rendered within wizard context");

    // Dynamic carousel slides for Activate step
    // Using placeholder images - replace with actual app screenshots in production
    let activate_slides = vec![
        SlideConfig::new("activate-1", "https://picsum.photos/seed/activate1/800/400")
            .title(t("wizard.activate.step1_title"))
            .description(t("wizard.activate.step1"))
            .alt("Navigate to License settings"),
        SlideConfig::new("activate-2", "https://picsum.photos/seed/activate2/800/400")
            .title(t("wizard.activate.step2_title"))
            .description(t("wizard.activate.step2"))
            .alt("Enter your license key"),
        SlideConfig::new("activate-3", "https://picsum.photos/seed/activate3/800/400")
            .title(t("wizard.activate.step3_title"))
            .description(t("wizard.activate.step3"))
            .alt("Start earning"),
    ];

    let state_for_back = state.clone();
    let state_for_close = state.clone();

    view! {
        <div class="wizard-stage wizard-activate">
            <div class="activate-carousel-container">
                <StepCarousel
                    slides=activate_slides
                    carousel_title=t("wizard.activate.title")
                    carousel_description=t("wizard.activate.subtitle")
                    show_thumbnails=true
                    loop_slides=true
                />
            </div>

            // Completion message
            <div class="completion-message">
                <p class="completion-text">
                    {move || t("wizard.activate.completion_message")}
                </p>
            </div>

            // Actions
            <div class="wizard-footer">
                <div class="footer-left">
                    <button
                        type="button"
                        class="btn-secondary"
                        on:click=move |_| state_for_back.prev_stage()
                    >
                        {move || t("common.back")}
                    </button>
                </div>
                <div class="footer-right">
                    <button
                        type="button"
                        class="btn-primary"
                        on:click=move |_| state_for_close.close()
                    >
                        {move || t("wizard.success.done_btn")}
                    </button>
                </div>
            </div>
        </div>
    }
}
