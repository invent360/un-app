//! Finance icons.
//!
//! This module provides SVG icons for finance, banking, and money.
//! Enable the `finance` feature to use these icons.

use leptos::prelude::*;

/// Finance icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FinanceIcon {
    // Money
    /// Dollar sign
    Dollar,
    /// Euro sign
    Euro,
    /// Pound sign
    Pound,
    /// Yen sign
    Yen,
    /// Coins
    Coins,
    /// Banknotes/Cash
    Cash,
    /// Money bag
    MoneyBag,
    /// Piggy bank
    PiggyBank,

    // Banking
    /// Bank building
    Bank,
    /// Vault/Safe
    Vault,
    /// Credit card
    CreditCard,
    /// ATM
    Atm,
    /// Receipt
    Receipt,
    /// Invoice
    Invoice,

    // Trading
    /// Chart line (trending)
    ChartLine,
    /// Chart bar
    ChartBar,
    /// Chart candlestick
    ChartCandle,
    /// Trending up
    TrendUp,
    /// Trending down
    TrendDown,
    /// Exchange/Swap
    Exchange,
    /// Wallet
    Wallet,

    // Business
    /// Calculator
    Calculator,
    /// Percentage
    Percent,
    /// Tax
    Tax,
    /// Budget
    Budget,
    /// Investment
    Investment,
    /// Dividend
    Dividend,
}

impl FinanceIcon {
    /// Returns the icon name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dollar => "Dollar",
            Self::Euro => "Euro",
            Self::Pound => "Pound",
            Self::Yen => "Yen",
            Self::Coins => "Coins",
            Self::Cash => "Cash",
            Self::MoneyBag => "Money Bag",
            Self::PiggyBank => "Piggy Bank",
            Self::Bank => "Bank",
            Self::Vault => "Vault",
            Self::CreditCard => "Credit Card",
            Self::Atm => "ATM",
            Self::Receipt => "Receipt",
            Self::Invoice => "Invoice",
            Self::ChartLine => "Line Chart",
            Self::ChartBar => "Bar Chart",
            Self::ChartCandle => "Candlestick Chart",
            Self::TrendUp => "Trending Up",
            Self::TrendDown => "Trending Down",
            Self::Exchange => "Exchange",
            Self::Wallet => "Wallet",
            Self::Calculator => "Calculator",
            Self::Percent => "Percentage",
            Self::Tax => "Tax",
            Self::Budget => "Budget",
            Self::Investment => "Investment",
            Self::Dividend => "Dividend",
        }
    }

    /// Parse a finance icon from its name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "dollar" | "usd" => Some(Self::Dollar),
            "euro" | "eur" => Some(Self::Euro),
            "pound" | "gbp" => Some(Self::Pound),
            "yen" | "jpy" => Some(Self::Yen),
            "coins" => Some(Self::Coins),
            "cash" | "banknotes" => Some(Self::Cash),
            "moneybag" | "money-bag" | "money bag" => Some(Self::MoneyBag),
            "piggybank" | "piggy-bank" | "piggy bank" => Some(Self::PiggyBank),
            "bank" => Some(Self::Bank),
            "vault" | "safe" => Some(Self::Vault),
            "creditcard" | "credit-card" | "credit card" => Some(Self::CreditCard),
            "atm" => Some(Self::Atm),
            "receipt" => Some(Self::Receipt),
            "invoice" => Some(Self::Invoice),
            "chartline" | "chart-line" | "line chart" => Some(Self::ChartLine),
            "chartbar" | "chart-bar" | "bar chart" => Some(Self::ChartBar),
            "chartcandle" | "candlestick" => Some(Self::ChartCandle),
            "trendup" | "trend-up" | "trending up" => Some(Self::TrendUp),
            "trenddown" | "trend-down" | "trending down" => Some(Self::TrendDown),
            "exchange" | "swap" => Some(Self::Exchange),
            "wallet" => Some(Self::Wallet),
            "calculator" => Some(Self::Calculator),
            "percent" | "percentage" => Some(Self::Percent),
            "tax" => Some(Self::Tax),
            "budget" => Some(Self::Budget),
            "investment" => Some(Self::Investment),
            "dividend" => Some(Self::Dividend),
            _ => None,
        }
    }

    /// Returns all available finance icons.
    pub fn all() -> &'static [FinanceIcon] {
        &[
            Self::Dollar, Self::Euro, Self::Pound, Self::Yen, Self::Coins, Self::Cash, Self::MoneyBag, Self::PiggyBank,
            Self::Bank, Self::Vault, Self::CreditCard, Self::Atm, Self::Receipt, Self::Invoice,
            Self::ChartLine, Self::ChartBar, Self::ChartCandle, Self::TrendUp, Self::TrendDown, Self::Exchange, Self::Wallet,
            Self::Calculator, Self::Percent, Self::Tax, Self::Budget, Self::Investment, Self::Dividend,
        ]
    }
}

// Finance SVG constants
const DOLLAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v20"/><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg>"##;

const EURO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 10h12"/><path d="M4 14h9"/><path d="M19 6a7.7 7.7 0 0 0-5.2-2A8 8 0 1 0 19 18"/></svg>"##;

const POUND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M18 7a4 4 0 0 0-4-4 6 6 0 0 0-6 6v5"/><path d="M6 13h8"/><path d="M6 21h12"/><path d="M6 21c2-3 2-6 2-9"/></svg>"##;

const YEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V12"/><path d="M6 2l6 10 6-10"/><path d="M8 12h8"/><path d="M8 16h8"/></svg>"##;

const COINS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="8" cy="8" r="6"/><path d="M18.09 10.37A6 6 0 1 1 10.34 18"/><path d="M7 6h2v4"/><path d="M16 14h2v4"/></svg>"##;

const CASH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="12" cy="12" r="3"/><path d="M6 12h.01"/><path d="M18 12h.01"/></svg>"##;

const MONEY_BAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 3h6l-3 4-3-4z"/><path d="M6 7c0 0-4 3-4 9 0 4 4 6 10 6s10-2 10-6c0-6-4-9-4-9H6z"/><path d="M12 11v6"/><path d="M9 14h6"/></svg>"##;

const PIGGY_BANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 10a2 2 0 0 1 2 2 2 2 0 0 1-2 2"/><ellipse cx="12" cy="13" rx="8" ry="6"/><path d="M15 9c1-1 1-3 0-4"/><circle cx="15" cy="11" r="1"/><path d="M4 13c-1 0-2 1-2 2v2h3"/><path d="M17 17l1 4h-3l-1-4"/><path d="M10 17l-1 4h-3l1-4"/></svg>"##;

const BANK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 21h18"/><path d="M3 10h18"/><path d="M5 6l7-3 7 3"/><path d="M4 10v11"/><path d="M20 10v11"/><path d="M8 10v7"/><path d="M12 10v7"/><path d="M16 10v7"/></svg>"##;

const VAULT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="12" cy="12" r="4"/><path d="M12 8v8"/><path d="M8 12h8"/><path d="M18 21v2"/><path d="M6 21v2"/></svg>"##;

const CREDIT_CARD_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="5" width="20" height="14" rx="2"/><path d="M2 10h20"/><path d="M6 15h4"/><path d="M14 15h4"/></svg>"##;

const ATM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="2" width="18" height="20" rx="2"/><rect x="7" y="6" width="10" height="6" rx="1"/><path d="M7 16h2"/><path d="M11 16h2"/><path d="M15 16h2"/><path d="M7 19h10"/></svg>"##;

const RECEIPT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 2v20l2-1 2 1 2-1 2 1 2-1 2 1 2-1 2 1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1-2-1z"/><path d="M8 7h8"/><path d="M8 11h8"/><path d="M8 15h5"/></svg>"##;

const INVOICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M8 13h8"/><path d="M8 17h5"/><path d="M8 9h2"/></svg>"##;

const CHART_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><path d="M7 16l4-5 4 4 5-6"/></svg>"##;

const CHART_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><rect x="7" y="10" width="3" height="8"/><rect x="14" y="6" width="3" height="12"/></svg>"##;

const CHART_CANDLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9 5v4"/><rect x="7" y="9" width="4" height="6"/><path d="M9 15v4"/><path d="M17 3v3"/><rect x="15" y="6" width="4" height="8"/><path d="M17 14v4"/></svg>"##;

const TREND_UP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 7l-8.5 8.5-5-5L2 17"/><path d="M16 7h6v6"/></svg>"##;

const TREND_DOWN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 17l-8.5-8.5-5 5L2 7"/><path d="M16 17h6v-6"/></svg>"##;

const EXCHANGE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 17h16l-4-4"/><path d="M20 7H4l4 4"/></svg>"##;

const WALLET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="6" width="20" height="14" rx="2"/><path d="M2 10h20"/><circle cx="16" cy="14" r="2"/></svg>"##;

const CALCULATOR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="2" width="16" height="20" rx="2"/><rect x="7" y="5" width="10" height="4"/><path d="M7 13h2"/><path d="M11 13h2"/><path d="M15 13h2"/><path d="M7 17h2"/><path d="M11 17h2"/><path d="M15 17h2"/></svg>"##;

const PERCENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 5L5 19"/><circle cx="6.5" cy="6.5" r="2.5"/><circle cx="17.5" cy="17.5" r="2.5"/></svg>"##;

const TAX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><path d="M9 13l6 6"/><circle cx="9.5" cy="13.5" r="1.5"/><circle cx="14.5" cy="18.5" r="1.5"/></svg>"##;

const BUDGET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M3 10h18"/><path d="M8 14h.01"/><path d="M12 14h.01"/><path d="M16 14h.01"/></svg>"##;

const INVESTMENT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v4"/><path d="M12 18v4"/><circle cx="12" cy="12" r="6"/><path d="M12 9v6"/><path d="M9 12h6"/></svg>"##;

const DIVIDEND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="8" r="6"/><path d="M12 5v6"/><path d="M9 8h6"/><path d="M12 14v4"/><path d="M8 22l4-4 4 4"/></svg>"##;

/// Get the SVG content for a finance icon.
#[must_use]
pub fn get_finance_svg(icon: FinanceIcon) -> &'static str {
    match icon {
        FinanceIcon::Dollar => DOLLAR_SVG,
        FinanceIcon::Euro => EURO_SVG,
        FinanceIcon::Pound => POUND_SVG,
        FinanceIcon::Yen => YEN_SVG,
        FinanceIcon::Coins => COINS_SVG,
        FinanceIcon::Cash => CASH_SVG,
        FinanceIcon::MoneyBag => MONEY_BAG_SVG,
        FinanceIcon::PiggyBank => PIGGY_BANK_SVG,
        FinanceIcon::Bank => BANK_SVG,
        FinanceIcon::Vault => VAULT_SVG,
        FinanceIcon::CreditCard => CREDIT_CARD_SVG,
        FinanceIcon::Atm => ATM_SVG,
        FinanceIcon::Receipt => RECEIPT_SVG,
        FinanceIcon::Invoice => INVOICE_SVG,
        FinanceIcon::ChartLine => CHART_LINE_SVG,
        FinanceIcon::ChartBar => CHART_BAR_SVG,
        FinanceIcon::ChartCandle => CHART_CANDLE_SVG,
        FinanceIcon::TrendUp => TREND_UP_SVG,
        FinanceIcon::TrendDown => TREND_DOWN_SVG,
        FinanceIcon::Exchange => EXCHANGE_SVG,
        FinanceIcon::Wallet => WALLET_SVG,
        FinanceIcon::Calculator => CALCULATOR_SVG,
        FinanceIcon::Percent => PERCENT_SVG,
        FinanceIcon::Tax => TAX_SVG,
        FinanceIcon::Budget => BUDGET_SVG,
        FinanceIcon::Investment => INVESTMENT_SVG,
        FinanceIcon::Dividend => DIVIDEND_SVG,
    }
}

/// Finance icon component.
#[component]
pub fn Finance(
    /// The finance icon to display.
    icon: FinanceIcon,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional aria-label for accessibility.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg = get_finance_svg(icon);
    let label = aria_label.unwrap_or_else(|| icon.name().to_string());

    view! {
        <span
            class=class
            role="img"
            aria-label=label
            inner_html=svg
        />
    }
}
