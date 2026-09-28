//! Referrals page - Apply to become a referral agent

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::hooks::t;
use crate::api::submit_referral_application;

/// List of country codes for the dropdown
const COUNTRIES: &[(&str, &str)] = &[
    ("US", "United States"),
    ("GB", "United Kingdom"),
    ("CA", "Canada"),
    ("AU", "Australia"),
    ("DE", "Germany"),
    ("FR", "France"),
    ("ES", "Spain"),
    ("IT", "Italy"),
    ("BR", "Brazil"),
    ("MX", "Mexico"),
    ("AR", "Argentina"),
    ("CO", "Colombia"),
    ("CL", "Chile"),
    ("PE", "Peru"),
    ("NG", "Nigeria"),
    ("KE", "Kenya"),
    ("ZA", "South Africa"),
    ("GH", "Ghana"),
    ("TZ", "Tanzania"),
    ("UG", "Uganda"),
    ("IN", "India"),
    ("PK", "Pakistan"),
    ("BD", "Bangladesh"),
    ("PH", "Philippines"),
    ("ID", "Indonesia"),
    ("MY", "Malaysia"),
    ("TH", "Thailand"),
    ("VN", "Vietnam"),
    ("JP", "Japan"),
    ("KR", "South Korea"),
    ("CN", "China"),
    ("RU", "Russia"),
    ("PL", "Poland"),
    ("NL", "Netherlands"),
    ("BE", "Belgium"),
    ("PT", "Portugal"),
    ("SE", "Sweden"),
    ("NO", "Norway"),
    ("DK", "Denmark"),
    ("FI", "Finland"),
    ("AT", "Austria"),
    ("CH", "Switzerland"),
    ("IE", "Ireland"),
    ("NZ", "New Zealand"),
    ("AE", "United Arab Emirates"),
    ("SA", "Saudi Arabia"),
    ("EG", "Egypt"),
    ("TR", "Turkey"),
    ("IL", "Israel"),
];

/// Get flag image URL from flagcdn.com
fn get_flag_url(code: &str) -> String {
    format!("https://flagcdn.com/w40/{}.png", code.to_lowercase())
}

/// Get country name by code
fn get_country_name(code: &str) -> &'static str {
    COUNTRIES.iter()
        .find(|(c, _)| *c == code)
        .map(|(_, name)| *name)
        .unwrap_or("Select Country")
}

/// Custom country selector with flags
#[component]
fn CountrySelector(
    value: ReadSignal<String>,
    on_change: impl Fn(String) + 'static + Copy,
    disabled: Signal<bool>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let toggle_dropdown = move |_| {
        if !disabled.get() {
            set_is_open.update(|v| *v = !*v);
        }
    };

    let select_country = move |code: &'static str| {
        on_change(code.to_string());
        set_is_open.set(false);
    };

    // Close dropdown when clicking outside
    let close_dropdown = move |_| {
        set_is_open.set(false);
    };

    view! {
        <div class="country-selector" class:disabled=move || disabled.get()>
            // Selected value button
            <button
                type="button"
                class="country-selector-button"
                on:click=toggle_dropdown
                disabled=move || disabled.get()
            >
                {move || {
                    let code = value.get();
                    if code.is_empty() {
                        view! {
                            <span class="country-placeholder">{move || t("referrals.placeholder_country")}</span>
                            <svg class="dropdown-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="6 9 12 15 18 9"/>
                            </svg>
                        }.into_any()
                    } else {
                        view! {
                            <img
                                src=get_flag_url(&code)
                                alt=""
                                class="country-flag"
                            />
                            <span class="country-name">{get_country_name(&code)}</span>
                            <svg class="dropdown-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="6 9 12 15 18 9"/>
                            </svg>
                        }.into_any()
                    }
                }}
            </button>

            // Dropdown overlay (to close when clicking outside)
            {move || is_open.get().then(|| view! {
                <div class="country-selector-overlay" on:click=close_dropdown></div>
            })}

            // Dropdown list
            <div class="country-selector-dropdown" class:open=move || is_open.get()>
                {COUNTRIES.iter().map(|(code, name)| {
                    let code_static: &'static str = code;
                    let is_selected = {
                        let code_owned = code.to_string();
                        move || value.get() == code_owned
                    };
                    view! {
                        <button
                            type="button"
                            class="country-option"
                            class:selected=is_selected
                            on:click=move |_| select_country(code_static)
                        >
                            <img
                                src=get_flag_url(code)
                                alt=""
                                class="country-flag"
                            />
                            <span class="country-name">{*name}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Referrals page component with application form
#[component]
pub fn ReferralsPage() -> impl IntoView {
    // Form state
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (country_code, set_country_code) = signal(String::new());
    let (referral_code, set_referral_code) = signal(String::new());

    // UI state
    let (is_submitting, set_is_submitting) = signal(false);
    let (error_message, set_error_message) = signal::<Option<String>>(None);
    let (success_message, set_success_message) = signal::<Option<String>>(None);

    // Form validation
    let is_form_valid = move || {
        let u = username.get();
        let e = email.get();
        let c = country_code.get();
        let r = referral_code.get();

        !u.trim().is_empty()
            && u.len() >= 2
            && !e.trim().is_empty()
            && e.contains('@')
            && !c.is_empty()
            && !r.trim().is_empty()
            && r.len() >= 3
    };

    // Handle form submission
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if !is_form_valid() || is_submitting.get() {
            return;
        }

        let u = username.get();
        let e = email.get();
        let c = country_code.get();
        let r = referral_code.get();

        set_is_submitting.set(true);
        set_error_message.set(None);
        set_success_message.set(None);

        spawn_local(async move {
            match submit_referral_application(u, e, c, r).await {
                Ok(response) => {
                    if response.success {
                        set_success_message.set(Some(response.message));
                        // Clear form on success
                        set_username.set(String::new());
                        set_email.set(String::new());
                        set_country_code.set(String::new());
                        set_referral_code.set(String::new());
                    } else {
                        set_error_message.set(Some(response.message));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("An error occurred: {}", e)));
                }
            }
            set_is_submitting.set(false);
        });
    };

    let on_country_change = move |code: String| {
        set_country_code.set(code);
    };

    view! {
        <div class="page referrals-page">
            <header class="page-header">
                <h1>{move || t("referrals.page_title")}</h1>
                <p>{move || t("referrals.page_subtitle")}</p>
            </header>

            <section class="referrals-content">
                // Benefits section
                <div class="referrals-benefits">
                    <h2>{move || t("referrals.benefits_title")}</h2>
                    <ul class="benefits-list">
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            <span>{move || t("referrals.benefit_1")}</span>
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            <span>{move || t("referrals.benefit_2")}</span>
                        </li>
                        <li>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <polyline points="20 6 9 17 4 12"/>
                            </svg>
                            <span>{move || t("referrals.benefit_3")}</span>
                        </li>
                    </ul>
                </div>

                // Application form
                <div class="referrals-form-container">
                    <h2>{move || t("referrals.form_title")}</h2>

                    // Success message
                    {move || success_message.get().map(|msg| view! {
                        <div class="form-success">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                                <polyline points="22 4 12 14.01 9 11.01"/>
                            </svg>
                            <p>{msg}</p>
                        </div>
                    })}

                    // Error message
                    {move || error_message.get().map(|msg| view! {
                        <div class="form-error">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="15" y1="9" x2="9" y2="15"/>
                                <line x1="9" y1="9" x2="15" y2="15"/>
                            </svg>
                            <p>{msg}</p>
                        </div>
                    })}

                    <form class="referrals-form" on:submit=on_submit>
                        <div class="form-group">
                            <label for="username">{move || t("referrals.label_username")}</label>
                            <input
                                type="text"
                                id="username"
                                placeholder=move || t("referrals.placeholder_username")
                                prop:value=move || username.get()
                                on:input=move |ev| set_username.set(event_target_value(&ev))
                                disabled=move || is_submitting.get()
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label for="email">{move || t("referrals.label_email")}</label>
                            <input
                                type="email"
                                id="email"
                                placeholder=move || t("referrals.placeholder_email")
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                disabled=move || is_submitting.get()
                                required
                            />
                        </div>

                        <div class="form-group">
                            <label>{move || t("referrals.label_country")}</label>
                            <CountrySelector
                                value=country_code
                                on_change=on_country_change
                                disabled=Signal::derive(move || is_submitting.get())
                            />
                        </div>

                        <div class="form-group">
                            <label for="referral_code">{move || t("referrals.label_referral_code")}</label>
                            <input
                                type="text"
                                id="referral_code"
                                placeholder=move || t("referrals.placeholder_referral_code")
                                prop:value=move || referral_code.get()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev).to_uppercase();
                                    set_referral_code.set(value);
                                }
                                disabled=move || is_submitting.get()
                                maxlength="20"
                                required
                            />
                            <span class="form-hint">{move || t("referrals.hint_referral_code")}</span>
                        </div>

                        <button
                            type="submit"
                            class="btn-primary"
                            disabled=move || !is_form_valid() || is_submitting.get()
                        >
                            {move || {
                                if is_submitting.get() {
                                    t("referrals.submitting")
                                } else {
                                    t("referrals.submit")
                                }
                            }}
                        </button>
                    </form>
                </div>
            </section>
        </div>
    }
}
