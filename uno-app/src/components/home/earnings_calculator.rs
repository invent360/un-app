//! Simple earnings calculator component
//!
//! Interactive calculator showing estimated monthly earnings based on
//! device types and task availability.

use leptos::prelude::*;
use crate::hooks::t;

/// Maximum devices per category
const MAX_DEVICES: u8 = 5;

/// Rate per task per day in USD
const RATE_PER_TASK_DAY: f64 = 0.10;

/// Days per month for calculation
const DAYS_PER_MONTH: f64 = 30.0;

/// User's share percentage (50%)
const USER_SHARE: f64 = 0.50;

/// Simple Earnings Calculator component
/// Allows users to estimate their monthly earnings based on device types
#[component]
pub fn EarningsCalculator() -> impl IntoView {
    // Device count signals
    let (android_play, set_android_play) = signal(0u8);
    let (android_apk, set_android_apk) = signal(1u8); // Default to 1 APK device
    let (iphone, set_iphone) = signal(0u8);
    let (pc, set_pc) = signal(0u8);

    // Include projected tasks toggle
    let (include_projected, set_include_projected) = signal(false);

    // Calculate earnings based on device counts and task availability
    // Current tasks:
    //   - Telemetry: All mobile devices (Play, APK, iPhone)
    //   - Ugrid: APK only
    // Projected tasks:
    //   - Entropy: All mobile devices (Play, APK, iPhone)
    //   - GPU Computing: Android (Play + APK) + PC
    let monthly_earnings = Signal::derive(move || {
        let play_count = android_play.get() as f64;
        let apk_count = android_apk.get() as f64;
        let ios_count = iphone.get() as f64;
        let pc_count = pc.get() as f64;
        let with_projected = include_projected.get();

        // Current tasks
        // Telemetry: 1 task on all mobile devices
        let telemetry_tasks = play_count + apk_count + ios_count;
        // Ugrid: 1 task on APK only
        let ugrid_tasks = apk_count;

        let current_tasks = telemetry_tasks + ugrid_tasks;

        // Projected tasks (if enabled)
        let projected_tasks = if with_projected {
            // Entropy: 1 task on all mobile devices
            let entropy_tasks = play_count + apk_count + ios_count;
            // GPU Computing: 1 task on Android + PC
            let gpu_tasks = play_count + apk_count + pc_count;
            entropy_tasks + gpu_tasks
        } else {
            0.0
        };

        let total_tasks = current_tasks + projected_tasks;
        let daily_pool = total_tasks * RATE_PER_TASK_DAY;
        let monthly_pool = daily_pool * DAYS_PER_MONTH;
        let user_share = monthly_pool * USER_SHARE;

        (user_share, current_tasks, projected_tasks)
    });

    // Current earnings (without projected)
    let current_earnings = Signal::derive(move || {
        let play_count = android_play.get() as f64;
        let apk_count = android_apk.get() as f64;
        let ios_count = iphone.get() as f64;

        let telemetry_tasks = play_count + apk_count + ios_count;
        let ugrid_tasks = apk_count;
        let current_tasks = telemetry_tasks + ugrid_tasks;

        let daily_pool = current_tasks * RATE_PER_TASK_DAY;
        let monthly_pool = daily_pool * DAYS_PER_MONTH;
        monthly_pool * USER_SHARE
    });

    view! {
        <section class="earnings-calculator">
            <div class="container">
                <h2 class="section-title">{move || t("home.calc.title")}</h2>
                <p class="section-subtitle">{move || t("home.calc.subtitle")}</p>

                <div class="calculator-grid">
                    // Device Selection Panel
                    <div class="calculator-devices">
                        <h3 class="panel-title">{move || t("home.calc.your_devices")}</h3>

                        <DeviceSelector
                            label_key="home.calc.android_play"
                            value=android_play
                            set_value=set_android_play
                            icon_class="device-icon device-icon-android"
                        />

                        <DeviceSelector
                            label_key="home.calc.android_apk"
                            value=android_apk
                            set_value=set_android_apk
                            icon_class="device-icon device-icon-android-apk"
                        />

                        <DeviceSelector
                            label_key="home.calc.iphone"
                            value=iphone
                            set_value=set_iphone
                            icon_class="device-icon device-icon-ios"
                        />

                        <DeviceSelector
                            label_key="home.calc.pc"
                            value=pc
                            set_value=set_pc
                            icon_class="device-icon device-icon-pc"
                        />

                        // Projected tasks toggle
                        <div class="projected-toggle">
                            <label class="toggle-label">
                                <input
                                    type="checkbox"
                                    class="toggle-checkbox"
                                    prop:checked=include_projected
                                    on:change=move |ev| {
                                        set_include_projected.set(event_target_checked(&ev));
                                    }
                                />
                                <span class="toggle-text">{move || t("home.calc.include_projected")}</span>
                            </label>
                        </div>
                    </div>

                    // Earnings Estimate Panel
                    <div class="calculator-estimate">
                        <h3 class="panel-title">{move || t("home.calc.your_estimate")}</h3>

                        <div class="estimate-card">
                            <div class="estimate-label">{move || t("home.calc.monthly_earnings")}</div>
                            <div class="estimate-amount">
                                {move || {
                                    let (earnings, _, _) = monthly_earnings.get();
                                    format!("${:.2}", earnings)
                                }}
                            </div>
                            <div class="estimate-note">
                                {move || {
                                    if include_projected.get() {
                                        t("home.calc.with_projected")
                                    } else {
                                        t("home.calc.current_tasks_only")
                                    }
                                }}
                            </div>
                        </div>

                        // Show current vs projected breakdown when projected is enabled
                        {move || {
                            if include_projected.get() {
                                let current = current_earnings.get();
                                view! {
                                    <div class="estimate-breakdown">
                                        <div class="breakdown-row">
                                            <span class="breakdown-label">{t("home.calc.current_tasks_only")}</span>
                                            <span class="breakdown-value">{format!("${:.2}", current)}</span>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}

                        // Task count info
                        <div class="task-summary">
                            {move || {
                                let (_, current, projected) = monthly_earnings.get();
                                let total = current + projected;
                                if total > 0.0 {
                                    view! {
                                        <div class="task-count">
                                            <span class="count-icon">"📊"</span>
                                            <span class="count-text">
                                                {format!("{:.0}", total)} " " {t("home.calc.active_tasks")}
                                            </span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="task-count empty">
                                            <span class="count-text">{t("home.calc.add_devices")}</span>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>

                // Disclaimer
                <p class="calculator-disclaimer">
                    {move || t("home.calc.disclaimer")}
                </p>
            </div>
        </section>
    }
}

/// Device count selector component
#[component]
fn DeviceSelector(
    /// Translation key for label
    label_key: &'static str,
    /// Current value
    value: ReadSignal<u8>,
    /// Setter for value
    set_value: WriteSignal<u8>,
    /// CSS class for the device icon
    icon_class: &'static str,
) -> impl IntoView {
    let decrement = move |_| {
        let current = value.get();
        if current > 0 {
            set_value.set(current - 1);
        }
    };

    let increment = move |_| {
        let current = value.get();
        if current < MAX_DEVICES {
            set_value.set(current + 1);
        }
    };

    let is_at_min = Signal::derive(move || value.get() == 0);
    let is_at_max = Signal::derive(move || value.get() >= MAX_DEVICES);

    view! {
        <div class="device-selector">
            <div class="device-info">
                <span class=icon_class></span>
                <span class="device-label">{move || t(label_key)}</span>
            </div>
            <div class="device-counter">
                <button
                    class="counter-btn decrement"
                    on:click=decrement
                    disabled=is_at_min
                    title="Decrease"
                >
                    "-"
                </button>
                <span class="counter-value">{move || value.get()}</span>
                <button
                    class="counter-btn increment"
                    on:click=increment
                    disabled=is_at_max
                    title="Increase"
                >
                    "+"
                </button>
            </div>
        </div>
    }
}
