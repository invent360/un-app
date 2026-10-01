//! Setup Page for R5-13
//!
//! Guides users through app installation and setup process.
//! Shows OS-specific instructions and tracks progress.

use leptos::prelude::*;
use crate::hooks::{use_user, UserLoadState};

/// Setup page component (protected)
#[component]
pub fn SetupPage() -> impl IntoView {
    view! {
        <div class="setup-page">
            <SetupContent />
        </div>
    }
}

/// Setup content with authentication check
#[component]
fn SetupContent() -> impl IntoView {
    let user_ctx = use_user();

    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! {
                        <div class="setup-loading">
                            <div class="loading-spinner"></div>
                            <p>"Loading..."</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="setup-unauthenticated">
                            <div class="auth-required-card">
                                <h2>"Sign In Required"</h2>
                                <p>"Please sign in and claim a license first."</p>
                                <a href="/" class="btn-primary">"Get Started"</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="setup-error">
                            <p>"Error loading setup. Please try again."</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            if profile.license_id.is_none() {
                                view! {
                                    <div class="setup-no-license">
                                        <h2>"No License Found"</h2>
                                        <p>"You need to claim a license before setting up the app."</p>
                                        <a href="/" class="btn-primary">"Claim License"</a>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <SetupWizard license_id=profile.license_id.unwrap() /> }.into_any()
                            }
                        }
                        None => {
                            view! {
                                <div class="setup-unauthenticated">
                                    <p>"Please sign in to continue."</p>
                                    <a href="/" class="btn-primary">"Go Home"</a>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Setup wizard with steps
#[component]
fn SetupWizard(license_id: String) -> impl IntoView {
    // Track current step
    let (current_step, set_current_step) = signal(1u8);

    // Track completed steps (persisted to localStorage)
    let (completed_steps, set_completed_steps) = signal(vec![false; 4]);

    // Load saved progress on mount
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(data)) = storage.get_item("uno_setup_progress") {
                        if let Ok(steps) = serde_json::from_str::<Vec<bool>>(&data) {
                            set_completed_steps.set(steps);
                        }
                    }
                }
            }
        });
    }

    // Save progress when steps complete
    let save_progress = move |steps: Vec<bool>| {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(data) = serde_json::to_string(&steps) {
                        let _ = storage.set_item("uno_setup_progress", &data);
                    }
                }
            }
        }
        set_completed_steps.set(steps);
    };

    let mark_step_complete = move |step: usize| {
        let mut steps = completed_steps.get();
        if step < steps.len() {
            steps[step] = true;
            save_progress(steps);
        }
    };

    let go_to_step = move |step: u8| {
        set_current_step.set(step);
    };

    view! {
        <div class="setup-wizard">
            <div class="setup-header">
                <h1>"Set Up Your UNO App"</h1>
                <p class="setup-subtitle">"Follow these steps to start earning"</p>
            </div>

            // Progress indicator
            <div class="setup-progress">
                <SetupProgressStep
                    number=1
                    label="Download"
                    is_active=move || current_step.get() == 1
                    is_completed=move || completed_steps.get().get(0).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(1)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=2
                    label="Install"
                    is_active=move || current_step.get() == 2
                    is_completed=move || completed_steps.get().get(1).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(2)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=3
                    label="Configure"
                    is_active=move || current_step.get() == 3
                    is_completed=move || completed_steps.get().get(2).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(3)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=4
                    label="Activate"
                    is_active=move || current_step.get() == 4
                    is_completed=move || completed_steps.get().get(3).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(4)
                />
            </div>

            // Step content
            <div class="setup-content">
                {move || {
                    match current_step.get() {
                        1 => view! {
                            <DownloadStep
                                on_complete=move || {
                                    mark_step_complete(0);
                                    set_current_step.set(2);
                                }
                            />
                        }.into_any(),
                        2 => view! {
                            <InstallStep
                                on_back=move || set_current_step.set(1)
                                on_complete=move || {
                                    mark_step_complete(1);
                                    set_current_step.set(3);
                                }
                            />
                        }.into_any(),
                        3 => view! {
                            <ConfigureStep
                                license_id=license_id.clone()
                                on_back=move || set_current_step.set(2)
                                on_complete=move || {
                                    mark_step_complete(2);
                                    set_current_step.set(4);
                                }
                            />
                        }.into_any(),
                        4 => view! {
                            <ActivateStep
                                on_back=move || set_current_step.set(3)
                                on_complete=move || {
                                    mark_step_complete(3);
                                }
                            />
                        }.into_any(),
                        _ => view! { <div></div> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}

/// Progress step indicator
#[component]
fn SetupProgressStep<F>(
    number: u8,
    label: &'static str,
    #[prop(into)] is_active: Signal<bool>,
    #[prop(into)] is_completed: Signal<bool>,
    on_click: F,
) -> impl IntoView
where
    F: Fn(()) + 'static + Clone,
{
    let class = move || {
        let mut cls = "progress-step".to_string();
        if is_active.get() {
            cls.push_str(" active");
        }
        if is_completed.get() {
            cls.push_str(" completed");
        }
        cls
    };

    view! {
        <button class=class on:click=move |_| on_click.clone()(())>
            <div class="step-number">
                {move || if is_completed.get() {
                    view! { <span>"✓"</span> }.into_any()
                } else {
                    view! { <span>{number}</span> }.into_any()
                }}
            </div>
            <span class="step-label">{label}</span>
        </button>
    }
}

/// Step 1: Download the app
#[component]
fn DownloadStep<F>(on_complete: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    view! {
        <div class="setup-step-content download-step">
            <h2>"Download the UNO App"</h2>
            <p class="step-description">
                "Get the official UNO app from the Play Store to start earning."
            </p>

            <div class="download-options">
                // Android download
                <div class="download-card primary">
                    <div class="platform-icon android">
                        <span>"🤖"</span>
                    </div>
                    <h3>"Android"</h3>
                    <p>"Requires Android 8.0 or later"</p>
                    <a
                        href="https://play.google.com/store/apps/details?id=com.uno.app"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn-primary download-btn"
                    >
                        "Download from Play Store"
                    </a>
                </div>

                // iOS coming soon
                <div class="download-card disabled">
                    <div class="platform-icon ios">
                        <span>"🍎"</span>
                    </div>
                    <h3>"iOS"</h3>
                    <p>"Coming Soon"</p>
                    <button class="btn-secondary download-btn" disabled=true>
                        "App Store - Coming Soon"
                    </button>
                </div>
            </div>

            <div class="download-notice">
                <span class="info-icon">"ℹ"</span>
                <p>
                    "Only download from official sources. Never install from unknown links."
                </p>
            </div>

            <div class="step-actions">
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    "I've Downloaded the App"
                </button>
            </div>
        </div>
    }
}

/// Step 2: Install and permissions
#[component]
fn InstallStep<F1, F2>(on_back: F1, on_complete: F2) -> impl IntoView
where
    F1: Fn() + 'static + Clone,
    F2: Fn() + 'static + Clone,
{
    view! {
        <div class="setup-step-content install-step">
            <h2>"Install & Grant Permissions"</h2>
            <p class="step-description">
                "Complete these steps to enable the app to earn for you."
            </p>

            <div class="checklist">
                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>"Open the UNO app"</strong>
                        <p>"Launch the app after installation completes"</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>"Allow background activity"</strong>
                        <p>"Enable the app to run in the background for continuous earning"</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>"Disable battery optimization"</strong>
                        <p>"Go to Settings → Apps → UNO → Battery → Don't optimize"</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>"Enable auto-start (if available)"</strong>
                        <p>"Some devices require this for apps to start after reboot"</p>
                    </div>
                </div>
            </div>

            <div class="permission-tips">
                <h4>"Why these permissions?"</h4>
                <p>
                    "UNO uses your unused bandwidth to complete network tasks. "
                    "These permissions ensure the app can run reliably and earn for you even when you're not using your phone."
                </p>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    "Back"
                </button>
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    "I've Completed These Steps"
                </button>
            </div>
        </div>
    }
}

/// Step 3: Configure with license
#[component]
fn ConfigureStep<F1, F2>(license_id: String, on_back: F1, on_complete: F2) -> impl IntoView
where
    F1: Fn() + 'static + Clone,
    F2: Fn() + 'static + Clone,
{
    let short_license = if license_id.len() > 16 {
        format!("{}...", &license_id[..16])
    } else {
        license_id.clone()
    };

    view! {
        <div class="setup-step-content configure-step">
            <h2>"Connect Your License"</h2>
            <p class="step-description">
                "Link your license to the app to start earning."
            </p>

            <div class="license-display">
                <label>"Your License Key"</label>
                <div class="license-key">
                    <span class="key-value">{short_license}</span>
                    <button class="copy-btn" title="Copy to clipboard">
                        "📋"
                    </button>
                </div>
            </div>

            <div class="configure-instructions">
                <h4>"In the UNO App:"</h4>
                <ol>
                    <li>"Open the app and go to Settings"</li>
                    <li>"Tap 'Enter License Key'"</li>
                    <li>"Paste your license key"</li>
                    <li>"Tap 'Activate'"</li>
                </ol>
            </div>

            <div class="qr-code-section">
                <p>"Or scan this QR code with the app:"</p>
                <div class="qr-placeholder">
                    <span>"[QR Code]"</span>
                </div>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    "Back"
                </button>
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    "I've Connected My License"
                </button>
            </div>
        </div>
    }
}

/// Step 4: Activation and earning
#[component]
fn ActivateStep<F1, F2>(on_back: F1, on_complete: F2) -> impl IntoView
where
    F1: Fn() + 'static + Clone,
    F2: Fn() + 'static + Clone,
{
    view! {
        <div class="setup-step-content activate-step">
            <h2>"You're All Set!"</h2>
            <p class="step-description">
                "Your device is now configured to earn with UNO."
            </p>

            <div class="success-icon">"🎉"</div>

            <div class="next-steps">
                <h4>"What happens now?"</h4>
                <ul>
                    <li>"The app will start processing tasks automatically"</li>
                    <li>"Keep your device connected to WiFi when possible"</li>
                    <li>"Check your dashboard to track earnings"</li>
                    <li>"Complete D1, D7, and D30 milestones to maximize rewards"</li>
                </ul>
            </div>

            <div class="tips-card">
                <h4>"Tips for Maximum Earnings"</h4>
                <ul>
                    <li>"Keep the app running in the background"</li>
                    <li>"Connect to WiFi instead of mobile data"</li>
                    <li>"Keep your device charged or plugged in"</li>
                    <li>"Don't force-close the app"</li>
                </ul>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    "Back"
                </button>
                <a href="/dashboard" class="btn-primary">
                    "Go to Dashboard"
                </a>
            </div>
        </div>
    }
}
