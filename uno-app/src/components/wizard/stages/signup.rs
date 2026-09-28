//! Sign-up stage component - create account in the app

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;
use crate::components::wizard::components::{StepCarousel, SlideConfig};

/// Sign-up stage - create account in the UNetwork app
#[component]
pub fn SignUpStage() -> impl IntoView {
    let state = use_wizard_state().expect("SignUpStage must be rendered within wizard context");

    // Dynamic carousel slides for SignUp step
    // Using placeholder images - replace with actual app screenshots in production
    let signup_slides = vec![
        SlideConfig::new("signup-1", "https://picsum.photos/seed/signup1/800/400")
            .title(t("wizard.signup.step1_title"))
            .description(t("wizard.signup.step1"))
            .alt("Open the UNetwork app"),
        SlideConfig::new("signup-2", "https://picsum.photos/seed/signup2/800/400")
            .title(t("wizard.signup.step2_title"))
            .description(t("wizard.signup.step2"))
            .alt("Enter your email"),
        SlideConfig::new("signup-3", "https://picsum.photos/seed/signup3/800/400")
            .title(t("wizard.signup.step3_title"))
            .description(t("wizard.signup.step3"))
            .alt("Create your password"),
    ];

    let state_for_back = state.clone();
    let state_for_next = state.clone();

    view! {
        <div class="wizard-stage wizard-signup">
            <div class="signup-carousel-container">
                <StepCarousel
                    slides=signup_slides
                    carousel_title=t("wizard.signup.title")
                    carousel_description=t("wizard.signup.subtitle")
                    show_thumbnails=true
                    loop_slides=true
                />
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
                        on:click=move |_| state_for_next.next_stage()
                    >
                        {move || t("common.next")}
                    </button>
                </div>
            </div>
        </div>
    }
}
