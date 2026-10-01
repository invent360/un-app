//! Support Page for R5-13
//!
//! Provides support resources, ticket management, and exit flow.
//! Protected route - requires authentication.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use crate::hooks::{use_user, UserLoadState};

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
                            <p>"Loading support..."</p>
                        </div>
                    }.into_any()
                }
                UserLoadState::NotAuthenticated => {
                    view! {
                        <div class="support-unauthenticated">
                            <div class="auth-required-card">
                                <h2>"Sign In Required"</h2>
                                <p>"Please sign in to access support."</p>
                                <a href="/" class="btn-primary">"Go to Home"</a>
                            </div>
                        </div>
                    }.into_any()
                }
                UserLoadState::Error => {
                    view! {
                        <div class="support-error">
                            <p>"Error loading support. Please try again."</p>
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
                                    <p>"Please sign in to access support."</p>
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
                <h1>"Help & Support"</h1>
                <p class="support-subtitle">"Get help, track tickets, and manage your account"</p>
            </div>

            // Quick help section
            <QuickHelpSection />

            // Tab navigation
            <div class="support-tabs">
                <button
                    class=move || if active_section.get() == "tickets" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("tickets".to_string())
                >
                    "My Tickets"
                </button>
                <button
                    class=move || if active_section.get() == "faq" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("faq".to_string())
                >
                    "Common Questions"
                </button>
                <button
                    class=move || if active_section.get() == "account" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| set_active_section.set("account".to_string())
                >
                    "Account Actions"
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
                <ExitFlowModal is_open=show_exit_flow />
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
                        <h4>"Getting Started"</h4>
                        <p>"Setup guides and tutorials"</p>
                    </div>
                </a>
                <a href="/faq" class="quick-help-card">
                    <span class="help-icon">"❓"</span>
                    <div class="help-content">
                        <h4>"FAQ"</h4>
                        <p>"Frequently asked questions"</p>
                    </div>
                </a>
                <div class="quick-help-card">
                    <span class="help-icon">"💬"</span>
                    <div class="help-content">
                        <h4>"Live Chat"</h4>
                        <p>"Available 9am-5pm UTC"</p>
                    </div>
                </div>
                <a href="mailto:support@unetwork.io" class="quick-help-card">
                    <span class="help-icon">"📧"</span>
                    <div class="help-content">
                        <h4>"Email Support"</h4>
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
            <div class="ticket-header">
                <h3>"Support Tickets"</h3>
                <button class="btn-primary" on:click=move |_| show_new_ticket.set(true)>
                    "New Ticket"
                </button>
            </div>

            // Open tickets
            <div class="ticket-group">
                <h4 class="ticket-group-title">"Open Tickets" <span class="count">"("{open_tickets.len()}")"</span></h4>
                {if open_tickets.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>"No open tickets"</p>
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
                <h4 class="ticket-group-title">"Resolved Tickets" <span class="count">"("{closed_tickets.len()}")"</span></h4>
                {if closed_tickets.is_empty() {
                    view! {
                        <div class="empty-state">
                            <p>"No resolved tickets"</p>
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
                        <span class="unread-badge">"New"</span>
                    })}
                </div>
                <div class="ticket-subject">{ticket.subject}</div>
                <div class="ticket-meta">
                    <span class="ticket-date">"Created: "{ticket.created_at}</span>
                    <span class="ticket-updated">"Updated: "{ticket.updated_at}</span>
                </div>
            </div>
            <div class="ticket-status">
                <span class=status_class>{ticket.status}</span>
                <button class="btn-text view-btn">"View"</button>
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
            <h3>"Common Questions"</h3>
            <div class="faq-list">
                {faqs.into_iter().map(|(question, answer)| view! {
                    <details class="faq-item">
                        <summary class="faq-question">{question}</summary>
                        <div class="faq-answer">{answer}</div>
                    </details>
                }).collect::<Vec<_>>()}
            </div>
            <div class="faq-more">
                <a href="/faq" class="btn-secondary">"View All FAQs"</a>
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
            <h3>"Account Actions"</h3>

            // Balance breakdown
            <div class="balance-card">
                <h4>"Your Balance"</h4>
                <div class="balance-breakdown">
                    <div class="balance-row">
                        <span class="balance-label">"Earned (confirmed)"</span>
                        <span class="balance-value">"$12.50"</span>
                    </div>
                    <div class="balance-row">
                        <span class="balance-label">"Pending (processing)"</span>
                        <span class="balance-value pending">"$3.20"</span>
                    </div>
                    <div class="balance-row">
                        <span class="balance-label">"Paid out"</span>
                        <span class="balance-value paid">"$45.00"</span>
                    </div>
                    <div class="balance-row total">
                        <span class="balance-label">"Total lifetime"</span>
                        <span class="balance-value">"$60.70"</span>
                    </div>
                </div>
                <p class="balance-note">"Next payout scheduled for October 15, 2026"</p>
            </div>

            // Action cards
            <div class="action-cards">
                // Pause license
                <div class="action-card">
                    <div class="action-icon pause">"⏸"</div>
                    <div class="action-info">
                        <h4>"Pause License"</h4>
                        <p>"Temporarily stop earning. Your license remains active and you can resume anytime."</p>
                    </div>
                    <button class="btn-secondary">"Pause"</button>
                </div>

                // Request exit
                <div class="action-card warning">
                    <div class="action-icon exit">"🚪"</div>
                    <div class="action-info">
                        <h4>"Request Exit"</h4>
                        <p>"Permanently leave the program. Your pending balance will be paid out after processing."</p>
                    </div>
                    <button class="btn-warning" on:click=move |_| show_exit_flow.set(true)>
                        "Request Exit"
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
            <h4>"Exit Request Status"</h4>
            <div class="exit-timeline">
                <div class="timeline-step completed">
                    <span class="step-icon">"✓"</span>
                    <div class="step-content">
                        <span class="step-title">"Request Submitted"</span>
                        <span class="step-date">"Sept 25, 2026"</span>
                    </div>
                </div>
                <div class="timeline-step active">
                    <span class="step-icon">"•"</span>
                    <div class="step-content">
                        <span class="step-title">"Processing Balance"</span>
                        <span class="step-date">"In progress"</span>
                    </div>
                </div>
                <div class="timeline-step">
                    <span class="step-icon">"○"</span>
                    <div class="step-content">
                        <span class="step-title">"Final Payout"</span>
                        <span class="step-date">"Estimated: Oct 10, 2026"</span>
                    </div>
                </div>
            </div>
            <div class="exit-payout">
                <span class="payout-label">"Final payout amount:"</span>
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

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_is_submitting.set(true);

        // TODO: Submit to API
        // For now, just close after a delay
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            leptos::task::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(1000).await;
                is_open.set(false);
            });
        }
    };

    view! {
        <div class="modal-overlay" on:click=close_modal>
            <div class="modal-content new-ticket-modal" on:click=|ev| ev.stop_propagation()>
                <div class="modal-header">
                    <h3>"Create Support Ticket"</h3>
                    <button class="modal-close" on:click=close_modal>"×"</button>
                </div>

                <form class="ticket-form" on:submit=handle_submit>
                    <div class="form-group">
                        <label for="category">"Category"</label>
                        <select
                            id="category"
                            prop:value=move || category.get()
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_category.set(value);
                            }
                        >
                            <option value="general">"General Question"</option>
                            <option value="technical">"Technical Issue"</option>
                            <option value="payment">"Payment/Payout"</option>
                            <option value="account">"Account/License"</option>
                            <option value="other">"Other"</option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label for="subject">"Subject"</label>
                        <input
                            type="text"
                            id="subject"
                            placeholder="Brief description of your issue"
                            prop:value=move || subject.get()
                            on:input=move |ev| set_subject.set(event_target_value(&ev))
                            required=true
                            maxlength=100
                        />
                    </div>

                    <div class="form-group">
                        <label for="description">"Description"</label>
                        <textarea
                            id="description"
                            placeholder="Please provide details about your issue..."
                            rows=5
                            prop:value=move || description.get()
                            on:input=move |ev| set_description.set(event_target_value(&ev))
                            required=true
                        ></textarea>
                    </div>

                    <div class="form-note">
                        <span class="info-icon">"ℹ"</span>
                        <p>"We typically respond within 24-48 hours. For urgent issues, use live chat during business hours."</p>
                    </div>

                    <div class="form-actions">
                        <button type="button" class="btn-secondary" on:click=close_modal>
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="btn-primary"
                            disabled=move || is_submitting.get() || subject.get().is_empty() || description.get().is_empty()
                        >
                            {move || if is_submitting.get() { "Submitting..." } else { "Submit Ticket" }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

/// Exit flow modal with confirmation
#[component]
fn ExitFlowModal(is_open: RwSignal<bool>) -> impl IntoView {
    let (step, set_step) = signal(1u8);
    let (confirm_text, set_confirm_text) = signal(String::new());
    let (is_processing, set_is_processing) = signal(false);

    let close_modal = move |_| is_open.set(false);

    view! {
        <div class="modal-overlay" on:click=close_modal>
            <div class="modal-content exit-flow-modal" on:click=|ev| ev.stop_propagation()>
                <div class="modal-header">
                    <h3>"Request Exit"</h3>
                    <button class="modal-close" on:click=close_modal>"×"</button>
                </div>

                {move || match step.get() {
                    1 => view! {
                        <div class="exit-step">
                            <div class="warning-banner">
                                <span class="warning-icon">"⚠"</span>
                                <p>"This action is permanent. Please read carefully."</p>
                            </div>

                            <h4>"What happens when you exit?"</h4>
                            <ul class="exit-details">
                                <li>"Your license will be deactivated"</li>
                                <li>"Any pending balance will be processed for payout"</li>
                                <li>"Processing takes 7-14 business days"</li>
                                <li>"You cannot rejoin with the same license"</li>
                            </ul>

                            <div class="balance-summary">
                                <h4>"Your Current Balance"</h4>
                                <div class="summary-row">
                                    <span>"Confirmed earnings"</span>
                                    <span>"$12.50"</span>
                                </div>
                                <div class="summary-row">
                                    <span>"Pending (may change)"</span>
                                    <span>"$3.20"</span>
                                </div>
                                <div class="summary-row total">
                                    <span>"Estimated final payout"</span>
                                    <span>"$15.70"</span>
                                </div>
                            </div>

                            <div class="exit-actions">
                                <button class="btn-secondary" on:click=close_modal>
                                    "Cancel"
                                </button>
                                <button class="btn-warning" on:click=move |_| set_step.set(2)>
                                    "Continue"
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    2 => view! {
                        <div class="exit-step confirmation">
                            <h4>"Confirm Your Decision"</h4>
                            <p>"To confirm exit, type "<strong>"EXIT"</strong>" below:"</p>

                            <input
                                type="text"
                                class="confirm-input"
                                placeholder="Type EXIT"
                                prop:value=move || confirm_text.get()
                                on:input=move |ev| set_confirm_text.set(event_target_value(&ev))
                            />

                            <div class="exit-actions">
                                <button class="btn-secondary" on:click=move |_| set_step.set(1)>
                                    "Back"
                                </button>
                                <button
                                    class="btn-danger"
                                    disabled=move || confirm_text.get().to_uppercase() != "EXIT" || is_processing.get()
                                    on:click=move |_| {
                                        set_is_processing.set(true);
                                        // TODO: Submit exit request to API
                                        set_step.set(3);
                                    }
                                >
                                    {move || if is_processing.get() { "Processing..." } else { "Confirm Exit" }}
                                </button>
                            </div>
                        </div>
                    }.into_any(),
                    3 => view! {
                        <div class="exit-step complete">
                            <div class="success-icon">"✓"</div>
                            <h4>"Exit Request Submitted"</h4>
                            <p>"Your exit request has been received. We'll process your final payout within 7-14 business days."</p>
                            <p class="confirmation-number">"Confirmation: EXIT-2026092998"</p>

                            <div class="exit-actions">
                                <button class="btn-primary" on:click=close_modal>
                                    "Close"
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
