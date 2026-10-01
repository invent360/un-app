//! Country and Device detection stage for R5-13

use leptos::prelude::*;
use crate::components::suitability::state::{SuitabilityState, DeviceType};

/// Country and device detection stage
#[component]
pub fn CountryDeviceStage(state: SuitabilityState) -> impl IntoView {
    let answers = state.answers;

    // Clone state for various uses
    let state_for_country = state.clone();
    let state_for_device1 = state.clone();
    let state_for_device2 = state.clone();
    let state_for_device3 = state.clone();
    let state_for_device4 = state.clone();
    let state_for_next = state.clone();
    let state_for_proceed = state.clone();

    // Detect device type from user agent (client-side only)
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let state_detect = state.clone();
        Effect::new(move |_| {
            if state_detect.answers.get().device_type.is_none() {
                if let Some(device) = detect_device_type() {
                    state_detect.set_device(device);
                }
            }
        });
    }

    let selected_device = Signal::derive(move || answers.get().device_type);
    let is_device_eligible = move || {
        answers.get().device_type.map(|d| d.is_supported()).unwrap_or(true)
    };
    let can_proceed = move || state_for_proceed.can_proceed();

    view! {
        <div class="suitability-stage country-device-stage">
            <h3>"Your Location & Device"</h3>
            <p class="stage-description">
                "We need to verify your location and device compatibility."
            </p>

            // Country selection
            <div class="form-group">
                <label class="form-label">"Your Country"</label>
                <select
                    class="form-select"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        if !value.is_empty() {
                            state_for_country.set_country(value);
                        }
                    }
                >
                    <option value="" disabled selected=move || answers.get().country_code.is_none()>
                        "Select your country"
                    </option>
                    <option value="NG" selected=move || answers.get().country_code.as_deref() == Some("NG")>"Nigeria"</option>
                    <option value="KE" selected=move || answers.get().country_code.as_deref() == Some("KE")>"Kenya"</option>
                    <option value="GH" selected=move || answers.get().country_code.as_deref() == Some("GH")>"Ghana"</option>
                    <option value="ZA" selected=move || answers.get().country_code.as_deref() == Some("ZA")>"South Africa"</option>
                    <option value="PH" selected=move || answers.get().country_code.as_deref() == Some("PH")>"Philippines"</option>
                    <option value="ID" selected=move || answers.get().country_code.as_deref() == Some("ID")>"Indonesia"</option>
                    <option value="IN" selected=move || answers.get().country_code.as_deref() == Some("IN")>"India"</option>
                    <option value="BD" selected=move || answers.get().country_code.as_deref() == Some("BD")>"Bangladesh"</option>
                    <option value="PK" selected=move || answers.get().country_code.as_deref() == Some("PK")>"Pakistan"</option>
                    <option value="BR" selected=move || answers.get().country_code.as_deref() == Some("BR")>"Brazil"</option>
                    <option value="OTHER" selected=move || answers.get().country_code.as_deref() == Some("OTHER")>"Other"</option>
                </select>
            </div>

            // Device selection
            <div class="form-group">
                <label class="form-label">"Your Device"</label>
                <div class="device-options">
                    <DeviceOption
                        device=DeviceType::AndroidPhone
                        selected=selected_device
                        on_select=move |d| state_for_device1.set_device(d)
                    />
                    <DeviceOption
                        device=DeviceType::AndroidTablet
                        selected=selected_device
                        on_select=move |d| state_for_device2.set_device(d)
                    />
                    <DeviceOption
                        device=DeviceType::IPhone
                        selected=selected_device
                        on_select=move |d| state_for_device3.set_device(d)
                    />
                    <DeviceOption
                        device=DeviceType::Desktop
                        selected=selected_device
                        on_select=move |d| state_for_device4.set_device(d)
                    />
                </div>
            </div>

            // Device eligibility warning
            {move || {
                if !is_device_eligible() {
                    view! {
                        <div class="eligibility-warning">
                            <span class="warning-icon">"!"</span>
                            <p>
                                "Currently, UNO only supports Android devices. "
                                "iOS and desktop support is coming soon."
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}

            // Navigation buttons
            <div class="stage-actions">
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

/// Device option button
#[component]
fn DeviceOption<F>(
    device: DeviceType,
    selected: Signal<Option<DeviceType>>,
    on_select: F,
) -> impl IntoView
where
    F: Fn(DeviceType) + 'static + Clone,
{
    let is_selected = move || selected.get() == Some(device);
    let is_supported = device.is_supported();
    let device_name = device.display_name();

    let icon = match device {
        DeviceType::AndroidPhone => "smartphone",
        DeviceType::AndroidTablet => "tablet",
        DeviceType::IPhone => "smartphone",
        DeviceType::Desktop => "monitor",
        _ => "help-circle",
    };

    let class = move || {
        let mut cls = "device-option".to_string();
        if is_selected() {
            cls.push_str(" selected");
        }
        if !is_supported {
            cls.push_str(" unsupported");
        }
        cls
    };

    view! {
        <button
            class=class
            on:click=move |_| on_select.clone()(device)
        >
            <span class="device-icon" data-icon=icon></span>
            <span class="device-name">{device_name}</span>
            {(!is_supported).then(|| view! {
                <span class="unsupported-badge">"Coming Soon"</span>
            })}
        </button>
    }
}

/// Detect device type from user agent
#[cfg(any(feature = "csr", feature = "hydrate"))]
fn detect_device_type() -> Option<DeviceType> {
    let window = web_sys::window()?;
    let navigator = window.navigator();
    let ua = navigator.user_agent().ok()?;
    let ua_lower = ua.to_lowercase();

    if ua_lower.contains("android") {
        if ua_lower.contains("mobile") {
            Some(DeviceType::AndroidPhone)
        } else {
            Some(DeviceType::AndroidTablet)
        }
    } else if ua_lower.contains("iphone") {
        Some(DeviceType::IPhone)
    } else if ua_lower.contains("ipad") {
        Some(DeviceType::IPad)
    } else if ua_lower.contains("windows") || ua_lower.contains("macintosh") || ua_lower.contains("linux") {
        Some(DeviceType::Desktop)
    } else {
        None
    }
}
