//! Connectivity stage for R5-13

use leptos::prelude::*;
use crate::components::suitability::state::{SuitabilityState, ConnectivityQuality, PowerSituation};

/// Connectivity and power situation stage
#[component]
pub fn ConnectivityStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;

    // Clone state for closures
    let state_for_conn1 = state.clone();
    let state_for_conn2 = state.clone();
    let state_for_conn3 = state.clone();
    let state_for_power1 = state.clone();
    let state_for_power2 = state.clone();
    let state_for_power3 = state.clone();
    let state_for_back = state.clone();
    let state_for_next = state.clone();
    let state_for_proceed = state.clone();

    let can_proceed = move || state_for_proceed.can_proceed();

    // Connectivity selection
    let is_conn_good = move || answers.get().connectivity == Some(ConnectivityQuality::Good);
    let is_conn_avg = move || answers.get().connectivity == Some(ConnectivityQuality::Average);
    let is_conn_limited = move || answers.get().connectivity == Some(ConnectivityQuality::Limited);

    // Power selection
    let is_power_always = move || answers.get().power == Some(PowerSituation::AlwaysOn);
    let is_power_regular = move || answers.get().power == Some(PowerSituation::Regular);
    let is_power_limited = move || answers.get().power == Some(PowerSituation::Limited);

    view! {
        <div class="suitability-stage connectivity-stage">
            <h3>"Your Connection"</h3>
            <p class="stage-description">
                "Help us estimate your potential earnings based on your connectivity."
            </p>

            // Connectivity quality
            <div class="form-group">
                <label class="form-label">"How stable is your internet connection?"</label>
                <div class="radio-options">
                    <label class=move || if is_conn_good() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="connectivity"
                            checked=is_conn_good
                            on:change=move |_| state_for_conn1.set_connectivity(ConnectivityQuality::Good)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Good (stable WiFi + mobile data)"</span>
                            <span class="radio-description">"WiFi at home + reliable mobile data when out"</span>
                        </div>
                    </label>
                    <label class=move || if is_conn_avg() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="connectivity"
                            checked=is_conn_avg
                            on:change=move |_| state_for_conn2.set_connectivity(ConnectivityQuality::Average)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Average (occasional interruptions)"</span>
                            <span class="radio-description">"Generally good but occasional interruptions"</span>
                        </div>
                    </label>
                    <label class=move || if is_conn_limited() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="connectivity"
                            checked=is_conn_limited
                            on:change=move |_| state_for_conn3.set_connectivity(ConnectivityQuality::Limited)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Limited (data caps or frequent drops)"</span>
                            <span class="radio-description">"Data caps, slow speeds, or frequent drops"</span>
                        </div>
                    </label>
                </div>
            </div>

            // Power situation
            <div class="form-group">
                <label class="form-label">"How often can you keep your device charged?"</label>
                <div class="radio-options">
                    <label class=move || if is_power_always() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="power"
                            checked=is_power_always
                            on:change=move |_| state_for_power1.set_power(PowerSituation::AlwaysOn)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Always connected to power"</span>
                            <span class="radio-description">"Device stays plugged in most of the day"</span>
                        </div>
                    </label>
                    <label class=move || if is_power_regular() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="power"
                            checked=is_power_regular
                            on:change=move |_| state_for_power2.set_power(PowerSituation::Regular)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Charged regularly"</span>
                            <span class="radio-description">"Charged in the morning and evening"</span>
                        </div>
                    </label>
                    <label class=move || if is_power_limited() { "radio-option selected" } else { "radio-option" }>
                        <input
                            type="radio"
                            name="power"
                            checked=is_power_limited
                            on:change=move |_| state_for_power3.set_power(PowerSituation::Limited)
                        />
                        <div class="radio-content">
                            <span class="radio-label">"Limited power access"</span>
                            <span class="radio-description">"Power outages or limited access to charging"</span>
                        </div>
                    </label>
                </div>
            </div>

            // Navigation buttons
            <div class="stage-actions">
                <button
                    class="btn-secondary"
                    on:click=move |_| state_for_back.prev_stage()
                >
                    "Back"
                </button>
                <button
                    class="btn-primary"
                    disabled=move || !can_proceed()
                    on:click=move |_| state_for_next.next_stage()
                >
                    "Continue"
                </button>
            </div>
        </div>
    }
}
