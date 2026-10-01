//! Support Page for R5-13
//!
//! Provides support resources, ticket management, and exit flow.
//! Protected route - requires authentication.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::{use_user, UserLoadState, t};
use crate::components::common::DevBanner;
use crate::api::support::{
    submit_ticket, initiate_exit,
    CreateTicketRequest, InitiateExitRequest,
};

/// Support ticket for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: String,
    pub number: String,
    pub subject: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub has_unread_reply: bool,
}

/// Exit request status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitRequest {
    pub status: String,
    pub requested_at: String,
    pub payout_status: Option<String>,
    pub net_payout_micros: Option<i64>,
    pub estimated_completion: Option<String>,
}

/// Support page component (protected)
#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <div class="support-page">
            <SupportContent />
        </div>
    }
}

/// Support content with authentication check
#[component]
fn SupportContent() -> impl IntoView {
    let user_ctx = use_user();

    view! {
        {move || {
            match user_ctx.state.get() {
                UserLoadState::Loading => {
                    view! {
                        <div class="support-loading">
                            <div class="loading-spinner"></div>
                            <p>{t("support.loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="support-unauthenticated">
                            <div class="auth-required-card">
                                <h2>{t("support.signin_required")}</h2>
                                <p>{t("support.signin_message")}</p>
                                <a href="/" class="btn-primary">{t("support.go_home")}</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="support-error">
                            <p>{t("support.error_loading")}</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::Loaded => {
                    match user_ctx.user.get() {
                        Some(profile) => {
                            view! { <AuthenticatedSupport user_id=profile.id.clone() /> }.into_any()
                        }
                        None => {
                            view! {
                                <div class="support-unauthenticated">
                                    <p>{t("support.signin_message")}</p>
                                    <a href="/" class="btn-primary">{t("support.go_home")}</a>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }
        }}
    }
}

/// Authenticated support view
#[component]
fn AuthenticatedSupport(user_id: String) -> impl IntoView {
    // Track which section is expanded
    let (active_section, set_active_section) = signal("tickets".to_string());

    // Track new ticket form visibility - use RwSignal for full read/write access
    let show_new_ticket = RwSignal::new(false);

    // Track exit flow visibility - use RwSignal for full read/write access
    let show_exit_flow = RwSignal::new(false);

    // Clone user_id for different closures
    let user_id_for_tabs = user_id.clone();
    let user_id_for_modal = user_id.clone();

    view! {
        <div class="support-authenticated">
            <div class="support-header">
                <h1>{t("support.help_title")}</h1>
                <p class="support-subtitle">{t("support.help_subtitle")}</p>
            </div>

            // Quick help section
            <QuickHelpSection />

            // Tab navigation
            <div class="support-tabs">
                <button
                    class=move || if active_section.get() == "tickets" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("tickets".to_string())
                >
                    {t("support.tab_my_tickets")}
                </button>
                <button
                    class=move || if active_section.get() == "faq" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("faq".to_string())
                >
                    {t("support.tab_common_questions")}
                </button>
                <button
                    class=move || if active_section.get() == "account" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("account".to_string())
                >
                    {t("support.tab_account_actions")}
                </button>
            </div>

            // Tab content
            <div class="support-content">
                {move || {
                    match active_section.get().as_str() {
                        "tickets" => view! {
                            <TicketSection
                                user_id=user_id_for_tabs.clone()
                                show_new_ticket=show_new_ticket
                            />
                        }.into_any(),
                        "faq" => view! {
                            <FaqSection />
                        }.into_any(),
                        "account" => view! {
                            <AccountActionsSection
                                show_exit_flow=show_exit_flow
                            />
                        }.into_any(),
                        _ => view! { <div></div> }.into_any(),
                    }
                }}
            </div>

            // New ticket modal
            {move || show_new_ticket.get().then(|| view! {
                <NewTicketModal
                    user_id=user_id_for_modal.clone()
                    is_open=show_new_ticket
                />
            })}

            // Exit flow modal
            {move || show_exit_flow.get().then(|| view! {
                <ExitFlowModal user_id=user_id.clone() is_open=show_exit_flow />
            })}
        </div>
    }
}

/// Quick help section with common resources
#[component]
fn QuickHelpSection() -> impl IntoView {
    view! {
        <div class="quick-help-section">
            <div class="quick-help-grid">
                <a href="/guides" class="quick-help-card">
                    <span class="help-icon">"📖"</span>
                    <div class="help-content">
                        <h4>{t("support.quick_getting_started")}</h4>
                        <p>{t("support.quick_getting_started_desc")}</p>
                    </div>
                </a>
                <a href="/faq" class="quick-help-card">
                    <span class="help-icon">"❓"</span>
                    <div class="help-content">
                        <h4>{t("support.quick_faq")}</h4>
                        <p>{t("support.quick_faq_desc")}</p>
                    </div>
                </a>
                <div class="quick-help-card">
                    <span class="help-icon">"💬"</span>
                    <div class="help-content">
                        <h4>{t("support.quick_live_chat")}</h4>
                        <p>{t("support.quick_live_chat_desc")}</p>
                    </div>
                </div>
                <a href="mailto:support@unetwork.io" class="quick-help-card">
                    <span class="help-icon">"📧"</span>
                    <div class="help-content">
                        <h4>{t("support.quick_email")}</h4>
                        <p>"support@unetwork.io"</p>
                    </div>
                </a>
            </div>
        </div>
    }
}

/// Ticket section with list and actions
#[component]
fn TicketSection(
    user_id: String,
    show_new_ticket: RwSignal<bool>,
) -> impl IntoView {
    // Mock tickets for now - would fetch from API
    let tickets: Vec<SupportTicket> = vec![
        SupportTicket {
            id: "t1".to_string(),
            number: "TKT-001234".to_string(),
            subject: "App not connecting".to_string(),
            status: "open".to_string(),
            created_at: "2026-09-28".to_string(),
            updated_at: "2026-09-29".to_string(),
            has_unread_reply: true,
        },
        SupportTicket {
            id: "t2".to_string(),
            number: "TKT-001198".to_string(),
            subject: "Payment not received".to_string(),
            status: "resolved".to_string(),
            created_at: "2026-09-20".to_string(),
            updated_at: "2026-09-22".to_string(),
            has_unread_reply: false,
        },
    ];

    let open_tickets: Vec<_> = tickets.iter().filter(|t| t.status != "resolved" && t.status != "closed").cloned().collect();
    let closed_tickets: Vec<_> = tickets.iter().filter(|t| t.status == "resolved" || t.status == "closed").cloned().collect();

    view! {
        <div class="ticket-section">
            <DevBanner
                title="Demo Mode"
                description="Ticket data shown below is for demonstration purposes."
            />
            <div class="ticket-header">
                <h3>{t("support.tickets_title")}</h3>
                <button class="btn-primary" on:click=move |_| show_new_ticket.set(true)>
                    {t("support.new_ticket")}
                </button>
            </div>

            // Open tickets
            <div class="ticket-group">
                <h4 class="ticket-group-title">{t("support.open_tickets")} <span class="count">"("{open_tickets.len()}")"</span></h4>
                {if open_tickets.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>{t("support.no_open_tickets")}</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="ticket-list">
                            {open_tickets.into_iter().map(|ticket| view! {
                                <TicketCard ticket=ticket />
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>

            // Closed tickets
            <div class="ticket-group">
                <h4 class="ticket-group-title">{t("support.resolved_tickets")} <span class="count">"("{closed_tickets.len()}")"</span></h4>
                {if closed_tickets.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>{t("support.no_resolved_tickets")}</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="ticket-list">
                            {closed_tickets.into_iter().map(|ticket| view! {
                                <TicketCard ticket=ticket />
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

/// Single ticket card display
#[component]
fn TicketCard(ticket: SupportTicket) -> impl IntoView {
    let status_class = match ticket.status.as_str() {
        "open" => "status-badge open",
        "pending" => "status-badge pending",
        "resolved" => "status-badge resolved",
        "closed" => "status-badge closed",
        _ => "status-badge",
    };

    view! {
        <div class="ticket-card">
            <div class="ticket-main">
                <div class="ticket-number">
                    {ticket.number.clone()}
                    {ticket.has_unread_reply.then(|| view! {
                        <span class="unread-badge">{t("support.badge_new")}</span>
                    })}
                </div>
                <div class="ticket-subject">{ticket.subject}</div>
                <div class="ticket-meta">
                    <span class="ticket-date">{t("support.created")}" "{ticket.created_at}</span>
                    <span class="ticket-updated">{t("support.updated")}" "{ticket.updated_at}</span>
                </div>
            </div>
            <div class="ticket-status">
                <span class=status_class>{ticket.status}</span>
                <button class="btn-text view-btn">{t("support.view")}</button>
            </div>
        </div>
    }
}

/// Common FAQ section
#[component]
fn FaqSection() -> impl IntoView {
    let faqs = vec![
        ("How do I start earning?", "Download the UNO app from the Play Store, enter your license key, and keep the app running in the background."),
        ("Why is my earnings estimate low?", "Earnings depend on your connectivity, device availability, and task supply in your region. Keep the app running continuously for best results."),
        ("When do I get paid?", "Payouts are processed monthly after you reach the minimum threshold. Check your dashboard for estimated payout dates."),
        ("Can I use multiple devices?", "Currently, each license supports one device. You can upgrade or add additional licenses if needed."),
        ("My app keeps stopping", "Make sure you've disabled battery optimization and enabled auto-start for the UNO app. See our setup guide for device-specific instructions."),
        ("How do I pause my account?", "Go to Account Actions in this support page and select 'Pause License'. You can resume at any time."),
    ];

    view! {
        <div class="faq-section">
            <h3>{t("support.common_questions")}</h3>
            <div class="faq-list">
                {faqs.into_iter().map(|(question, answer)| view! {
                    <details class="faq-item">
                        <summary class="faq-question">{question}</summary>
                        <div class="faq-answer">{answer}</div>
                    </details>
                }).collect::<Vec<_>>()}
            </div>
            <div class="faq-more">
                <a href="/faq" class="btn-secondary">{t("support.view_all_faqs")}</a>
            </div>
        </div>
    }
}

/// Account actions section (pause, exit)
#[component]
fn AccountActionsSection(
    show_exit_flow: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="account-actions-section">
            <h3>{t("support.account_actions")}</h3>

            // Balance breakdown
            <div class="balance-card">
                <h4>{t("support.your_balance")}</h4>
                <div class="balance-breakdown">
                    <div class="balance-row">
                        <span class="balance-label">{t("support.balance_earned")}</span>
                        <span class="balance-value">"$12.50"</span>
                    </div>
                    <div class="balance-row">
                        <span class="balance-label">{t("support.balance_pending")}</span>
                        <span class="balance-value pending">"$3.20"</span>
                    </div>
                    <div class="balance-row">
                        <span class="balance-label">{t("support.balance_paid")}</span>
                        <span class="balance-value paid">"$45.00"</span>
                    </div>
                    <div class="balance-row total">
                        <span class="balance-label">{t("support.balance_total")}</span>
                        <span class="balance-value">"$60.70"</span>
                    </div>
                </div>
                <p class="balance-note">{t("support.next_payout_note")}</p>
            </div>

            // Action cards
            <div class="action-cards">
                // Pause license
                <div class="action-card">
                    <div class="action-icon pause">"⏸"</div>
                    <div class="action-info">
                        <h4>{t("support.pause_license")}</h4>
                        <p>{t("support.pause_license_desc")}</p>
                    </div>
                    <button class="btn-secondary">{t("support.pause_btn")}</button>
                </div>

                // Request exit
                <div class="action-card warning">
                    <div class="action-icon exit">"🚪"</div>
                    <div class="action-info">
                        <h4>{t("support.request_exit")}</h4>
                        <p>{t("support.request_exit_desc")}</p>
                    </div>
                    <button class="btn-warning" on:click=move |_| show_exit_flow.set(true)>
                        {t("support.request_exit")}
                    </button>
                </div>
            </div>

            // Exit request status (if any)
            <ExitStatusCard />
        </div>
    }
}

/// Display exit request status if one exists
#[component]
fn ExitStatusCard() -> impl IntoView {
    // Mock - would come from API
    let has_exit_request = false;

    if !has_exit_request {
        return view! {}.into_any();
    }

    view! {
        <div class="exit-status-card">
            <h4>{t("support.exit_status_title")}</h4>
            <div class="exit-timeline">
                <div class="timeline-step completed">
                    <span class="step-icon">"✓"</span>
                    <div class="step-content">
                        <span class="step-title">{t("support.exit_step_submitted")}</span>
                        <span class="step-date">"Sept 25, 2026"</span>
                    </div>
                </div>
                <div class="timeline-step active">
                    <span class="step-icon">"•"</span>
                    <div class="step-content">
                        <span class="step-title">{t("support.exit_step_processing")}</span>
                        <span class="step-date">{t("support.in_progress")}</span>
                    </div>
                </div>
                <div class="timeline-step">
                    <span class="step-icon">"○"</span>
                    <div class="step-content">
                        <span class="step-title">{t("support.exit_step_payout")}</span>
                        <span class="step-date">"Estimated: Oct 10, 2026"</span>
                    </div>
                </div>
            </div>
            <div class="exit-payout">
                <span class="payout-label">{t("support.final_payout_amount")}</span>
                <span class="payout-amount">"$15.70"</span>
            </div>
        </div>
    }.into_any()
}

/// New ticket creation modal
#[component]
fn NewTicketModal(user_id: String, is_open: RwSignal<bool>) -> impl IntoView {
    let (subject, set_subject) = signal(String::new());
    let (category, set_category) = signal("general".to_string());
    let (description, set_description) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);

    let close_modal = move |_| is_open.set(false);

    let (submit_error, set_submit_error) = signal(Option::<String>::None);
    let (submit_success, set_submit_success) = signal(Option::<String>::None);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_is_submitting.set(true);
        set_submit_error.set(None);

        let request = CreateTicketRequest {
            category: category.get(),
            subject: subject.get(),
            description: description.get(),
        };

        leptos::task::spawn_local(async move {
            match submit_ticket(request).await {
                Ok(response) => {
                    if response.success {
                        if let Some(ticket_num) = response.ticket_number {
                            set_submit_success.set(Some(ticket_num));
                            // Close after showing success briefly
                            #[cfg(any(feature = "csr", feature = "hydrate"))]
                            {
                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                is_open.set(false);
                            }
                        }
                    } else {
                        set_submit_error.set(response.error.or(Some("Failed to create ticket".to_string())));
                        set_is_submitting.set(false);
                    }
                }
                Err(e) => {
                    set_submit_error.set(Some(e.to_string()));
                    set_is_submitting.set(false);
                }
            }
        });
    };

    view! {
        <div class="modal-overlay" on:click=close_modal>
            <div class="modal-content new-ticket-modal" on:click=|ev| ev.stop_propagation()>
                <div class="modal-header">
                    <h3>{t("support.create_ticket")}</h3>
                    <button class="modal-close" on:click=close_modal>"×"</button>
                </div>

                <form class="ticket-form" on:submit=handle_submit>
                    <div class="form-group">
                        <label for="category">{t("support.category_label")}</label>
                        <select
                            id="category"
                            prop:value=move || category.get()
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_category.set(value);
                            }
                        >
                            <option value="general">{t("support.category_general")}</option>
                            <option value="technical">{t("support.category_technical")}</option>
                            <option value="payment">{t("support.category_payment")}</option>
                            <option value="account">{t("support.category_account")}</option>
                            <option value="other">{t("support.category_other")}</option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="subject">{t("support.subject_label")}</label>
                        <input
                            type="text"
                            id="subject"
                            placeholder={t("support.subject_placeholder")}
                            prop:value=move || subject.get()
                            on:input=move |ev| set_subject.set(event_target_value(&ev))
                            required=true
                            maxlength=100
                        />
                    </div>

                    <div class="form-group">
                        <label for="description">{t("support.description_label")}</label>
                        <textarea
                            id="description"
                            placeholder={t("support.description_placeholder")}
                            rows=5
                            prop:value=move || description.get()
                            on:input=move |ev| set_description.set(event_target_value(&ev))
                            required=true
                        ></textarea>
                    </div>

                    {move || submit_error.get().map(|err| view! {
                        <div class="error-message">
                            <span class="error-icon">"✕"</span>
                            <p>{err}</p>
                        </div>
                    })}

                    {move || submit_success.get().map(|ticket_num| view! {
                        <div class="success-message">
                            <span class="success-icon">"✓"</span>
                            <p>{t("support.ticket_created")}" #{ticket_num}"</p>
                        </div>
                    })}

                    <div class="form-note">
                        <span class="info-icon">"ℹ"</span>
                        <p>{t("support.response_time_note")}</p>
                    </div>

                    <div class="form-actions">
                        <button type="button" class="btn-secondary" on:click=close_modal>
                            {t("common.cancel")}
                        </button>
                        <button
                            type="submit"
                            class="btn-primary"
                            disabled=move || is_submitting.get() || subject.get().is_empty() || description.get().is_empty()
                        >
                            {move || if is_submitting.get() { t("support.submitting") } else { t("support.submit_ticket") }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

/// Exit flow modal with confirmation
#[component]
fn ExitFlowModal(user_id: String, is_open: RwSignal<bool>) -> impl IntoView {
    let (step, set_step) = signal(1u8);
    let (confirm_text, set_confirm_text) = signal(String::new());
    let (is_processing, set_is_processing) = signal(false);
    let (exit_error, set_exit_error) = signal(Option::<String>::None);
    let (confirmation_number, set_confirmation_number) = signal(Option::<String>::None);

    // Get user context for license_id
    let user_ctx = use_user();

    let close_modal = move |_| is_open.set(false);

    view! {
        <div class="modal-overlay" on:click=close_modal>
            <div class="modal-content exit-flow-modal" on:click=|ev| ev.stop_propagation()>
                <div class="modal-header">
                    <h3>{t("support.request_exit")}</h3>
                    <button class="modal-close" on:click=close_modal>"×"</button>
                </div>

                {move || match step.get() {
                    1 => view! {
                        <div class="exit-step">
                            <div class="warning-banner">
                                <span class="warning-icon">"⚠"</span>
                                <p>{t("support.exit_warning")}</p>
                            </div>

                            <h4>{t("support.exit_what_happens")}</h4>
                            <ul class="exit-details">
                                <li>{t("support.exit_detail_1")}</li>
                                <li>{t("support.exit_detail_2")}</li>
                                <li>{t("support.exit_detail_3")}</li>
                                <li>{t("support.exit_detail_4")}</li>
                            </ul>

                            <div class="balance-summary">
                                <h4>{t("support.your_current_balance")}</h4>
                                <div class="summary-row">
                                    <span>{t("support.confirmed_earnings")}</span>
                                    <span>"$12.50"</span>
                                </div>
                                <div class="summary-row">
                                    <span>{t("support.pending_may_change")}</span>
                                    <span>"$3.20"</span>
                                </div>
                                <div class="summary-row total">
                                    <span>{t("support.estimated_final_payout")}</span>
                                    <span>"$15.70"</span>
                                </div>
                            </div>

                            <div class="exit-actions">
                                <button class="btn-secondary" on:click=close_modal>
                                    {t("common.cancel")}
                                </button>
                                <button class="btn-warning" on:click=move |_| set_step.set(2)>
                                    {t("support.continue")}
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    2 => view! {
                        <div class="exit-step confirmation">
                            <h4>{t("support.confirm_decision")}</h4>
                            <p>{t("support.confirm_exit_instruction")}</p>

                            <input
                                type="text"
                                class="confirm-input"
                                placeholder={t("support.type_exit")}
                                prop:value=move || confirm_text.get()
                                on:input=move |ev| set_confirm_text.set(event_target_value(&ev))
                            />

                            {move || exit_error.get().map(|err| view! {
                                <div class="error-message">
                                    <span class="error-icon">"✕"</span>
                                    <p>{err}</p>
                                </div>
                            })}

                            <div class="exit-actions">
                                <button class="btn-secondary" on:click=move |_| set_step.set(1)>
                                    {t("common.back")}
                                </button>
                                <button
                                    class="btn-danger"
                                    disabled=move || confirm_text.get().to_uppercase() != "EXIT" || is_processing.get()
                                    on:click=move |_| {
                                        set_is_processing.set(true);
                                        set_exit_error.set(None);

                                        // Get license_id from user context
                                        let license_id = user_ctx.user.get()
                                            .and_then(|u| u.license_id.clone());

                                        match license_id {
                                            Some(lid) => {
                                                let request = InitiateExitRequest {
                                                    license_id: lid,
                                                    exit_reason: Some("User requested voluntary exit".to_string()),
                                                };

                                                leptos::task::spawn_local(async move {
                                                    match initiate_exit(request).await {
                                                        Ok(response) => {
                                                            if response.success {
                                                                if let Some(conf_num) = response.confirmation_number {
                                                                    set_confirmation_number.set(Some(conf_num));
                                                                }
                                                                set_step.set(3);
                                                            } else {
                                                                set_exit_error.set(response.error.or(Some("Failed to process exit request".to_string())));
                                                                set_is_processing.set(false);
                                                            }
                                                        }
                                                        Err(e) => {
                                                            set_exit_error.set(Some(e.to_string()));
                                                            set_is_processing.set(false);
                                                        }
                                                    }
                                                });
                                            }
                                            None => {
                                                set_exit_error.set(Some("No license found. Please contact support.".to_string()));
                                                set_is_processing.set(false);
                                            }
                                        }
                                    }
                                >
                                    {move || if is_processing.get() { t("support.processing") } else { t("support.confirm_exit") }}
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    3 => view! {
                        <div class="exit-step complete">
                            <div class="success-icon">"✓"</div>
                            <h4>{t("support.exit_submitted")}</h4>
                            <p>{t("support.exit_submitted_message")}</p>
                            <p class="confirmation-number">
                                {t("support.confirmation")}" "
                                {move || confirmation_number.get().unwrap_or_else(|| "EXIT-PENDING".to_string())}
                            </p>

                            <div class="exit-actions">
                                <button class="btn-primary" on:click=close_modal>
                                    {t("common.close")}
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    _ => view! { <div></div> }.into_any(),
                }}
            </div>
        </div>
    }
}
