//! Setup Page for R5-13
//!
//! Guides users through app installation and setup process.
//! Shows OS-specific instructions and tracks progress.

use leptos::prelude::*;
use crate::hooks::{use_user, UserLoadState, t};

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
                            <p>{t("setup.loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="setup-unauthenticated">
                            <div class="auth-required-card">
                                <h2>{t("setup.signin_required")}</h2>
                                <p>{t("setup.signin_claim_first")}</p>
                                <a href="/" class="btn-primary">{t("setup.get_started")}</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="setup-error">
                            <p>{t("setup.error_loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            if profile.license_id.is_none() {
                                view! {
                                    <div class="setup-no-license">
                                        <h2>{t("setup.no_license_found")}</h2>
                                        <p>{t("setup.claim_license_first")}</p>
                                        <a href="/" class="btn-primary">{t("setup.claim_license")}</a>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <SetupWizard license_id=profile.license_id.unwrap() /> }.into_any()
                            }
                        }
                        None => {
                            view! {
                                <div class="setup-unauthenticated">
                                    <p>{t("setup.signin_continue")}</p>
                                    <a href="/" class="btn-primary">{t("setup.go_home")}</a>
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
                <h1>{t("setup.wizard_title")}</h1>
                <p class="setup-subtitle">{t("setup.wizard_subtitle")}</p>
            </div>

            // Progress indicator
            <div class="setup-progress">
                <SetupProgressStep
                    number=1
                    label=t("setup.step_download")
                    is_active=move || current_step.get() == 1
                    is_completed=move || completed_steps.get().get(0).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(1)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=2
                    label=t("setup.step_install")
                    is_active=move || current_step.get() == 2
                    is_completed=move || completed_steps.get().get(1).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(2)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=3
                    label=t("setup.step_configure")
                    is_active=move || current_step.get() == 3
                    is_completed=move || completed_steps.get().get(2).copied().unwrap_or(false)
                    on_click=move |_| go_to_step(3)
                />
                <div class="progress-connector"></div>
                <SetupProgressStep
                    number=4
                    label=t("setup.step_activate")
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
    #[prop(into)] label: String,
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
            <span class="step-label">{label.clone()}</span>
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
            <h2>{t("setup.download_title")}</h2>
            <p class="step-description">
                {t("setup.download_description")}
            </p>

            <div class="download-options">
                // Android download
                <div class="download-card primary">
                    <div class="platform-icon android">
                        <span>"🤖"</span>
                    </div>
                    <h3>{t("setup.android")}</h3>
                    <p>{t("setup.android_requirement")}</p>
                    <a
                        href="https://play.google.com/store/apps/details?id=com.uno.app"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn-primary download-btn"
                    >
                        {t("setup.download_play_store")}
                    </a>
                </div>

                // iOS coming soon
                <div class="download-card disabled">
                    <div class="platform-icon ios">
                        <span>"🍎"</span>
                    </div>
                    <h3>{t("setup.ios")}</h3>
                    <p>{t("setup.coming_soon")}</p>
                    <button class="btn-secondary download-btn" disabled=true>
                        {t("setup.app_store_coming_soon")}
                    </button>
                </div>
            </div>

            <div class="download-notice">
                <span class="info-icon">"ℹ"</span>
                <p>
                    {t("setup.download_notice")}
                </p>
            </div>

            <div class="step-actions">
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    {t("setup.downloaded_app")}
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
            <h2>{t("setup.install_title")}</h2>
            <p class="step-description">
                {t("setup.install_description")}
            </p>

            <div class="checklist">
                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>{t("setup.install_open_app")}</strong>
                        <p>{t("setup.install_open_app_desc")}</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>{t("setup.install_background")}</strong>
                        <p>{t("setup.install_background_desc")}</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>{t("setup.install_battery")}</strong>
                        <p>{t("setup.install_battery_desc")}</p>
                    </div>
                </div>

                <div class="checklist-item">
                    <span class="check-box">"☐"</span>
                    <div class="check-content">
                        <strong>{t("setup.install_autostart")}</strong>
                        <p>{t("setup.install_autostart_desc")}</p>
                    </div>
                </div>
            </div>

            <div class="permission-tips">
                <h4>{t("setup.why_permissions")}</h4>
                <p>
                    {t("setup.why_permissions_desc")}
                </p>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    {t("common.back")}
                </button>
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    {t("setup.completed_steps")}
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
            <h2>{t("setup.configure_title")}</h2>
            <p class="step-description">
                {t("setup.configure_description")}
            </p>

            <div class="license-display">
                <label>{t("setup.your_license_key")}</label>
                <div class="license-key">
                    <span class="key-value">{short_license}</span>
                    <button class="copy-btn" title={t("setup.copy_to_clipboard")}>
                        "📋"
                    </button>
                </div>
            </div>

            <div class="configure-instructions">
                <h4>{t("setup.in_uno_app")}</h4>
                <ol>
                    <li>{t("setup.configure_step1")}</li>
                    <li>{t("setup.configure_step2")}</li>
                    <li>{t("setup.configure_step3")}</li>
                    <li>{t("setup.configure_step4")}</li>
                </ol>
            </div>

            <div class="qr-code-section">
                <p>{t("setup.scan_qr_code")}</p>
                <div class="qr-placeholder">
                    <span>"[QR Code]"</span>
                </div>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    {t("common.back")}
                </button>
                <button class="btn-primary" on:click=move |_| on_complete.clone()()>
                    {t("setup.connected_license")}
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
            <h2>{t("setup.activate_title")}</h2>
            <p class="step-description">
                {t("setup.activate_description")}
            </p>

            <div class="success-icon">"🎉"</div>

            <div class="next-steps">
                <h4>{t("setup.what_happens_now")}</h4>
                <ul>
                    <li>{t("setup.next_step1")}</li>
                    <li>{t("setup.next_step2")}</li>
                    <li>{t("setup.next_step3")}</li>
                    <li>{t("setup.next_step4")}</li>
                </ul>
            </div>

            <div class="tips-card">
                <h4>{t("setup.tips_title")}</h4>
                <ul>
                    <li>{t("setup.tip1")}</li>
                    <li>{t("setup.tip2")}</li>
                    <li>{t("setup.tip3")}</li>
                    <li>{t("setup.tip4")}</li>
                </ul>
            </div>

            <div class="step-actions">
                <button class="btn-secondary" on:click=move |_| on_back.clone()()>
                    {t("common.back")}
                </button>
                <a href="/dashboard" class="btn-primary">
                    {t("setup.go_to_dashboard")}
                </a>
            </div>
        </div>
    }
}
