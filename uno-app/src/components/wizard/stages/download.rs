//! Download stage component - download and install the app

use leptos::prelude::*;
use crate::hooks::t;
use crate::components::wizard::state::use_wizard_state;
use crate::components::wizard::components::{DownloadButtons, InstallGuide};

/// Download stage - download and install the UNetwork app
#[component]
pub fn DownloadStage() -> impl IntoView {
    let state = use_wizard_state().expect("DownloadStage must be rendered within wizard context");
    let (show_install, set_show_install) = signal(false);

    // Toggle install guide
    let on_toggle_install = move |_| {
        set_show_install.update(|v| *v = !*v);
    };

    let state_for_back = state.clone();
    let state_for_next = state.clone();

    view! {
        <div class="wizard-stage wizard-download">
            <h2 class="stage-title">{move || t("wizard.download.title")}</h2>
            <p class="stage-subtitle">{move || t("wizard.download.subtitle")}</p>

            // Download section
            <div class="download-section">
                <DownloadButtons />

                <button
                    type="button"
                    class="install-guide-toggle"
                    on:click=on_toggle_install
                >
                    {move || {
                        if show_install.get() {
                            t("wizard.success.hide_install_btn")
                        } else {
                            t("wizard.success.install_guide_btn")
                        }
                    }}
                </button>

                <Show
                    when=move || show_install.get()
                    fallback=|| ()
                >
                    <div class="install-guide-container">
                        <InstallGuide />
                    </div>
                </Show>
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
