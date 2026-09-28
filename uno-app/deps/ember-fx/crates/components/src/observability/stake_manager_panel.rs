//! StakeManagerPanel Leptos component.
//!
//! A comprehensive staking management panel.

use leptos::prelude::*;
use leptos::callback::Callback;
use crate::try_use_theme;

/// Stake action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StakeAction {
    /// Stake tokens.
    Stake,
    /// Unstake tokens.
    Unstake,
    /// Restake rewards.
    Restake,
    /// Claim rewards.
    Claim,
}

impl StakeAction {
    /// Returns the label.
    pub fn as_label(&self) -> &'static str {
        match self {
            Self::Stake => "Stake",
            Self::Unstake => "Unstake",
            Self::Restake => "Restake",
            Self::Claim => "Claim",
        }
    }

    /// Returns the icon.
    pub fn as_icon(&self) -> &'static str {
        match self {
            Self::Stake => "🔒",
            Self::Unstake => "🔓",
            Self::Restake => "🔄",
            Self::Claim => "💰",
        }
    }
}

/// Validator/pool info for delegation.
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    /// Validator address/ID.
    pub id: String,
    /// Validator name.
    pub name: String,
    /// Commission rate (0-100).
    pub commission: f64,
    /// APY estimate.
    pub apy: f64,
    /// Total staked.
    pub total_staked: f64,
    /// Is active/online.
    pub is_active: bool,
    /// My delegation amount.
    pub my_delegation: Option<f64>,
}

impl ValidatorInfo {
    /// Create a new validator info.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        commission: f64,
        apy: f64,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            commission,
            apy,
            total_staked: 0.0,
            is_active: true,
            my_delegation: None,
        }
    }

    /// Set total staked.
    pub fn total_staked(mut self, amount: f64) -> Self {
        self.total_staked = amount;
        self
    }

    /// Set active status.
    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    /// Set my delegation.
    pub fn my_delegation(mut self, amount: f64) -> Self {
        self.my_delegation = Some(amount);
        self
    }
}

/// Stake action callback payload.
#[derive(Debug, Clone)]
pub struct StakeActionPayload {
    /// The action type.
    pub action: StakeAction,
    /// Amount (if applicable).
    pub amount: Option<f64>,
    /// Target validator (if applicable).
    pub validator_id: Option<String>,
}

/// StakeManagerPanel component.
///
/// Comprehensive staking management interface.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{StakeManagerPanel, ValidatorInfo};
///
/// let validators = vec![
///     ValidatorInfo::new("val1", "Validator One", 5.0, 12.5)
///         .total_staked(1_000_000.0)
///         .my_delegation(500.0),
/// ];
///
/// view! {
///     <StakeManagerPanel
///         staked_balance=Signal::derive(move || 5000.0)
///         available_balance=Signal::derive(move || 2500.0)
///         pending_rewards=Signal::derive(move || 125.5)
///         validators=Signal::derive(move || validators.clone())
///     />
/// }
/// ```
#[component]
pub fn StakeManagerPanel(
    /// Currently staked balance.
    #[prop(into)]
    staked_balance: Signal<f64>,
    /// Available balance to stake.
    #[prop(into)]
    available_balance: Signal<f64>,
    /// Pending rewards.
    #[prop(optional, into)]
    pending_rewards: Option<Signal<f64>>,
    /// List of validators.
    #[prop(optional, into)]
    validators: Option<Signal<Vec<ValidatorInfo>>>,
    /// Unbonding period in days.
    #[prop(optional)]
    unbonding_days: Option<u32>,
    /// Token symbol.
    #[prop(optional, into)]
    token_symbol: Option<String>,
    /// Current APY.
    #[prop(optional, into)]
    current_apy: Option<Signal<f64>>,
    /// On stake action callback.
    #[prop(optional)]
    on_action: Option<Callback<StakeActionPayload>>,
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// Show validator list.
    #[prop(optional)]
    show_validators: Option<bool>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let title = title.unwrap_or_else(|| "Staking".to_string());
    let token_symbol = token_symbol.unwrap_or_else(|| "EMB".to_string());
    let unbonding_days = unbonding_days.unwrap_or(21);
    let show_validators = show_validators.unwrap_or(true);

    let prefix = format!("fx-stake-manager-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    // Format token amount
    let format_tokens = move |val: f64| -> String {
        if val >= 1_000_000.0 {
            format!("{:.2}M", val / 1_000_000.0)
        } else if val >= 1000.0 {
            format!("{:.2}K", val / 1000.0)
        } else {
            format!("{:.2}", val)
        }
    };

    // Action button click handler
    let handle_action = {
        let on_action = on_action.clone();
        move |action: StakeAction| {
            if let Some(cb) = &on_action {
                cb.run(StakeActionPayload {
                    action,
                    amount: None,
                    validator_id: None,
                });
            }
        }
    };

    view! {
        <div class=combined_class>
            // Header
            <div class=format!("{}-header", prefix)>
                <span class=format!("{}-icon", prefix)>"🔒"</span>
                <span class=format!("{}-title", prefix)>{title}</span>
                {current_apy.map(|apy| {
                    view! {
                        <span class=format!("{}-apy", prefix)>
                            {move || format!("{:.1}% APY", apy.get())}
                        </span>
                    }
                })}
            </div>

            // Balance overview
            {
                let staked_sym = token_symbol.clone();
                let avail_sym = token_symbol.clone();
                let rewards_sym = token_symbol.clone();
                view! {
                    <div class=format!("{}-balances", prefix)>
                        <div class=format!("{}-balance-card", prefix)>
                            <span class=format!("{}-balance-label", prefix)>"Staked"</span>
                            <span class=format!("{}-balance-value {}-staked", prefix, prefix)>
                                {move || format!("{} {}", format_tokens(staked_balance.get()), staked_sym)}
                            </span>
                        </div>
                        <div class=format!("{}-balance-card", prefix)>
                            <span class=format!("{}-balance-label", prefix)>"Available"</span>
                            <span class=format!("{}-balance-value {}-available", prefix, prefix)>
                                {move || format!("{} {}", format_tokens(available_balance.get()), avail_sym)}
                            </span>
                        </div>
                        {pending_rewards.map(|pr| {
                            let sym = rewards_sym.clone();
                            view! {
                                <div class=format!("{}-balance-card", prefix)>
                                    <span class=format!("{}-balance-label", prefix)>"Rewards"</span>
                                    <span class=format!("{}-balance-value {}-rewards", prefix, prefix)>
                                        {move || format!("{} {}", format_tokens(pr.get()), sym)}
                                    </span>
                                </div>
                            }
                        })}
                    </div>
                }
            }

            // Quick actions
            <div class=format!("{}-actions", prefix)>
                {[StakeAction::Stake, StakeAction::Unstake, StakeAction::Restake, StakeAction::Claim]
                    .into_iter()
                    .map(|action| {
                        let handle = handle_action.clone();
                        let btn_class = format!("{}-action-btn", prefix);
                        view! {
                            <button
                                class=btn_class
                                on:click=move |_| handle(action)
                            >
                                <span class=format!("{}-action-icon", prefix)>
                                    {action.as_icon()}
                                </span>
                                <span class=format!("{}-action-label", prefix)>
                                    {action.as_label()}
                                </span>
                            </button>
                        }
                    })
                    .collect_view()}
            </div>

            // Unbonding notice
            <div class=format!("{}-notice", prefix)>
                <span class=format!("{}-notice-icon", prefix)>"⏱️"</span>
                <span class=format!("{}-notice-text", prefix)>
                    {format!("Unbonding period: {} days", unbonding_days)}
                </span>
            </div>

            // Validator list
            {move || {
                if show_validators {
                    validators.as_ref().map(|vals| {
                        view! {
                            <div class=format!("{}-validators", prefix)>
                                <div class=format!("{}-validators-header", prefix)>
                                    "Validators"
                                </div>
                                <div class=format!("{}-validators-list", prefix)>
                                    {vals.get().into_iter().map(|v| {
                                        let status_color = if v.is_active {
                                            "var(--fx-color-success, #52c41a)"
                                        } else {
                                            "var(--fx-color-text-secondary, #8c8c8c)"
                                        };

                                        view! {
                                            <div class=format!("{}-validator", prefix)>
                                                <div class=format!("{}-validator-info", prefix)>
                                                    <span
                                                        class=format!("{}-validator-status", prefix)
                                                        style=format!("background: {};", status_color)
                                                    />
                                                    <span class=format!("{}-validator-name", prefix)>
                                                        {v.name.clone()}
                                                    </span>
                                                </div>
                                                <div class=format!("{}-validator-stats", prefix)>
                                                    <span class=format!("{}-validator-apy", prefix)>
                                                        {format!("{:.1}% APY", v.apy)}
                                                    </span>
                                                    <span class=format!("{}-validator-commission", prefix)>
                                                        {format!("{:.1}% fee", v.commission)}
                                                    </span>
                                                </div>
                                                {v.my_delegation.map(|d| {
                                                    let sym = token_symbol.clone();
                                                    view! {
                                                        <div class=format!("{}-validator-delegation", prefix)>
                                                            {format!("{} {}", format_tokens(d), sym)}
                                                        </div>
                                                    }
                                                })}
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
