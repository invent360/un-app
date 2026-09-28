//! Cryptocurrency token icons.
//!
//! This module provides SVG icons for popular cryptocurrency tokens.
//! Enable the `crypto` feature to use these icons.

use leptos::prelude::*;

/// Cryptocurrency token identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenIcon {
    // Major cryptocurrencies
    /// Bitcoin (BTC)
    BTC,
    /// Ethereum (ETH)
    ETH,
    /// Solana (SOL)
    SOL,
    /// Polkadot (DOT)
    DOT,
    /// Cardano (ADA)
    ADA,
    /// Ripple (XRP)
    XRP,

    // Stablecoins
    /// USD Coin (USDC)
    USDC,
    /// Tether (USDT)
    USDT,
    /// Dai (DAI)
    DAI,

    // Wrapped tokens
    /// Wrapped Bitcoin (WBTC)
    WBTC,

    // Other tokens
    /// Ergo (ERG)
    ERG,
    /// Wrapped MTX (WMTX)
    WMTX,
}

impl TokenIcon {
    /// Returns the token symbol (e.g., "BTC", "ETH").
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::BTC => "BTC",
            Self::ETH => "ETH",
            Self::SOL => "SOL",
            Self::DOT => "DOT",
            Self::ADA => "ADA",
            Self::XRP => "XRP",
            Self::USDC => "USDC",
            Self::USDT => "USDT",
            Self::DAI => "DAI",
            Self::WBTC => "WBTC",
            Self::ERG => "ERG",
            Self::WMTX => "WMTX",
        }
    }

    /// Returns the full token name (e.g., "Bitcoin", "Ethereum").
    pub fn name(&self) -> &'static str {
        match self {
            Self::BTC => "Bitcoin",
            Self::ETH => "Ethereum",
            Self::SOL => "Solana",
            Self::DOT => "Polkadot",
            Self::ADA => "Cardano",
            Self::XRP => "XRP",
            Self::USDC => "USD Coin",
            Self::USDT => "Tether",
            Self::DAI => "Dai",
            Self::WBTC => "Wrapped BTC",
            Self::ERG => "Ergo",
            Self::WMTX => "Wrapped MTX",
        }
    }

    /// Returns the display string "{symbol} - {name}".
    pub fn display(&self) -> String {
        format!("{} - {}", self.symbol(), self.name())
    }

    /// Parse a token from its symbol string.
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol.to_uppercase().as_str() {
            "BTC" => Some(Self::BTC),
            "ETH" => Some(Self::ETH),
            "SOL" => Some(Self::SOL),
            "DOT" => Some(Self::DOT),
            "ADA" => Some(Self::ADA),
            "XRP" => Some(Self::XRP),
            "USDC" => Some(Self::USDC),
            "USDT" => Some(Self::USDT),
            "DAI" => Some(Self::DAI),
            "WBTC" => Some(Self::WBTC),
            "ERG" => Some(Self::ERG),
            "WMTX" => Some(Self::WMTX),
            _ => None,
        }
    }

    /// Returns all available token icons.
    pub fn all() -> &'static [TokenIcon] {
        &[
            Self::BTC, Self::ETH, Self::SOL, Self::DOT, Self::ADA, Self::XRP,
            Self::USDC, Self::USDT, Self::DAI, Self::WBTC, Self::ERG, Self::WMTX,
        ]
    }
}

// Token SVG constants - high-quality icons based on official brand assets
// Bitcoin - official orange with B symbol
const BTC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#F7931A"/><path fill="#fff" d="M23.2 14.1c.32-2.14-1.31-3.3-3.54-4.07l.72-2.9-1.77-.44-.7 2.82c-.47-.12-.94-.23-1.42-.34l.71-2.84-1.77-.44-.72 2.9c-.38-.09-.76-.17-1.13-.26l-2.44-.61-.47 1.89s1.31.3 1.28.32c.72.18.85.65.83 1.02l-.83 3.35c.05.01.11.03.18.06l-.18-.05-1.17 4.68c-.09.22-.31.54-.81.42.02.03-1.28-.32-1.28-.32l-.88 2.03 2.31.57c.43.11.85.22 1.26.33l-.73 2.93 1.77.44.72-2.9c.49.13.96.25 1.42.37l-.72 2.88 1.77.44.73-2.93c3.01.57 5.28.34 6.23-2.38.77-2.19-.04-3.45-1.62-4.28 1.15-.27 2.02-.97 2.25-2.5zm-4.03 5.65c-.55 2.19-4.24 1.01-5.44.71l.97-3.89c1.2.3 5.04.89 4.47 3.18zm.55-5.68c-.5 2-3.58.98-4.58.73l.88-3.53c1 .25 4.22.72 3.7 2.8z"/></svg>"##;

// Ethereum - official diamond logo with gradient effect
const ETH_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#627EEA"/><path fill="#fff" fill-opacity=".6" d="M16.5 4v8.87l7.5 3.35z"/><path fill="#fff" d="M16.5 4L9 16.22l7.5-3.35z"/><path fill="#fff" fill-opacity=".6" d="M16.5 21.97v6.03L24 17.62z"/><path fill="#fff" d="M16.5 28V21.97L9 17.62z"/><path fill="#fff" fill-opacity=".2" d="M16.5 20.57l7.5-4.35-7.5-3.35z"/><path fill="#fff" fill-opacity=".6" d="M9 16.22l7.5 4.35v-7.7z"/></svg>"##;

// Solana - official gradient with 3 bars
const SOL_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><defs><linearGradient id="sol-g" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="#00FFA3"/><stop offset="100%" stop-color="#DC1FFF"/></linearGradient></defs><circle cx="16" cy="16" r="16" fill="url(#sol-g)"/><path fill="#fff" d="M9.5 20.5c.1-.2.3-.3.5-.3h13.6c.3 0 .5.4.3.6l-2.4 2.4c-.1.1-.3.2-.5.2H7.4c-.3 0-.5-.4-.3-.6l2.4-2.3zm0-12c.1-.1.3-.2.5-.2H23.6c.3 0 .5.4.3.6l-2.4 2.4c-.1.1-.3.2-.5.2H7.4c-.3 0-.5-.4-.3-.6l2.4-2.4zm13.6 6c-.1-.1-.3-.2-.5-.2H9c-.3 0-.5.4-.3.6l2.4 2.4c.1.1.3.2.5.2H25c.3 0 .5-.4.3-.6l-2.2-2.4z"/></svg>"##;

// Polkadot - official pink ellipses logo
const DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#E6007A"><path d="M12,0c2.39,0,4.328,1.127,4.328,2.517S14.39,5.034,12,5.034,7.672,3.907,7.672,2.517,9.61,0,12,0Zm0,18.966c2.39,0,4.328,1.127,4.328,2.517S14.39,24,12,24s-4.328-1.127-4.328-2.517S9.61,18.966,12,18.966ZM1.606,6C2.8,3.93,4.747,2.816,5.952,3.511s1.212,2.937.017,5.007S2.828,11.7,1.624,11.007.411,8.07,1.606,6Zm16.427,9.483c1.2-2.07,3.139-3.184,4.343-2.489s1.211,2.936.016,5.006-3.14,3.185-4.344,2.49S16.837,17.553,18.033,15.483ZM1.624,12.993c1.205-.7,3.15.419,4.346,2.489s1.187,4.311-.018,5.007S2.8,20.07,1.607,18,.42,13.689,1.624,12.993ZM18.049,3.512c1.2-.695,3.149.419,4.344,2.489s1.188,4.311-.016,5.007-3.148-.42-4.343-2.49S16.846,4.207,18.049,3.512Z"/></svg>"##;

// Cardano - official ADA logo
const ADA_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 497.32 459.54"><defs><style>.cls-1{fill:#0033ad;}</style></defs><g><g><path class="cls-1" d="M142,246.58a33.5,33.5,0,0,0,31.56,35.28h2A33.49,33.49,0,1,0,142,246.58Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M17.11,238a10.83,10.83,0,1,0,10.21,11.41A10.74,10.74,0,0,0,17.11,238Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M139.91,52.17a10.85,10.85,0,0,0-9.81-19.36,10.85,10.85,0,0,0,9.81,19.36Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M173.86,111.45A16.7,16.7,0,1,0,151.45,104,16.72,16.72,0,0,0,173.86,111.45Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M59.55,152.16a13.79,13.79,0,1,0-4-19.09h0A13.71,13.71,0,0,0,59.55,152.16Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M79.84,232a16.74,16.74,0,1,0,15.78,17.63h0A16.74,16.74,0,0,0,79.84,232Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M61.4,344.32A13.82,13.82,0,1,0,80,350.42h0a13.87,13.87,0,0,0-18.57-6.1Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M126.38,197.38a19.64,19.64,0,1,0-5.7-27.18,19.52,19.52,0,0,0,5.7,27.18Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M332.6,110.13a16.76,16.76,0,1,0-4.78-23.21h0a16.57,16.57,0,0,0,4.78,23.21Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M366.81,51.11A10.83,10.83,0,1,0,363.63,36a11.15,11.15,0,0,0,3.18,15.11Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M335.12,214.62a33.47,33.47,0,0,0-3.72,66.84h1.86a33.45,33.45,0,0,0,1.86-66.84Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M184.87,195a33.51,33.51,0,1,0,14.85-45.09A33.54,33.54,0,0,0,184.87,195Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M447.31,152.16a13.79,13.79,0,1,0-18.7-6.1,14,14,0,0,0,18.7,6.1Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M362.43,162.77a19.66,19.66,0,1,0,26.39,8.76A19.66,19.66,0,0,0,362.43,162.77Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M253,46.07a13.81,13.81,0,1,0-13-14.72,14,14,0,0,0,13,14.72Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M252.9,132.67A19.65,19.65,0,1,0,234.33,112a19.52,19.52,0,0,0,18.57,20.69Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M146.27,333.71A19.66,19.66,0,1,0,119.88,325,19.67,19.67,0,0,0,146.27,333.71Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M265.63,161.45a33.47,33.47,0,1,0,28-15.12A33.38,33.38,0,0,0,265.63,161.45Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M323.84,301.49A33.48,33.48,0,1,0,309,346.44h0a33.33,33.33,0,0,0,15-44.69C324,301.62,324,301.62,323.84,301.49Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M382.33,299.1a19.64,19.64,0,1,0,5.7,27.18,19.52,19.52,0,0,0-5.7-27.18Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M446.51,248.84a16.74,16.74,0,1,0-17.64,15.78h0A16.84,16.84,0,0,0,446.51,248.84Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M492.79,236.9A10.83,10.83,0,1,0,503,248.31,10.91,10.91,0,0,0,492.79,236.9Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M449.16,344.32a13.79,13.79,0,1,0,4,19.1,13.86,13.86,0,0,0-4-19.1Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M141.77,445.37a10.82,10.82,0,1,0,3,15h0A10.66,10.66,0,0,0,141.77,445.37Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M368.8,444.31a10.85,10.85,0,1,0,9.81,19.36,10.85,10.85,0,0,0-9.81-19.36Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M243.08,335a33.53,33.53,0,1,0-46.41,9.68A33.65,33.65,0,0,0,243.08,335Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M176.11,386.36a16.75,16.75,0,1,0,4.78,23.2h0A16.58,16.58,0,0,0,176.11,386.36Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M254.22,450.41a13.81,13.81,0,1,0,13,14.72,14,14,0,0,0-13-14.72Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M254.35,363.81a19.66,19.66,0,1,0,18.57,20.69,19.53,19.53,0,0,0-18.57-20.69Z" transform="translate(-5.69 -18.47)"/><path class="cls-1" d="M334.85,385a16.76,16.76,0,1,0,22.68,7.43A16.93,16.93,0,0,0,334.85,385Z" transform="translate(-5.69 -18.47)"/></g></g></svg>"##;

// XRP - official X logo with curved chevrons
const XRP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#23292F"/><path fill="#fff" d="M24.5 7h2.5l-6.2 6.1c-2.6 2.5-6.8 2.5-9.4 0L5 7h2.5l5.2 5.1c1.8 1.8 4.8 1.8 6.6 0L24.5 7zM7.5 25H5l6.2-6.1c2.6-2.5 6.8-2.5 9.4 0L27 25h-2.5l-5.2-5.1c-1.8-1.8-4.8-1.8-6.6 0L7.5 25z"/></svg>"##;

// USDC - official blue with dollar sign and circles
const USDC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#2775CA"/><path fill="#fff" d="M20.5 18.5c0-2-1.2-2.7-3.6-3-.8-.1-1.7-.3-2.3-.5-.5-.2-.8-.5-.8-1s.4-1 1.3-1c.8 0 1.2.3 1.4.9.1.2.2.3.4.3h1c.2 0 .4-.2.3-.4-.2-1-1-1.8-2.2-2v-1.2c0-.2-.2-.4-.5-.4h-.8c-.3 0-.5.2-.5.4v1.2c-1.5.2-2.5 1.2-2.5 2.5 0 1.9 1.2 2.6 3.5 2.9 1.8.3 2.4.6 2.4 1.4 0 .9-.8 1.3-1.7 1.3-1.2 0-1.6-.5-1.8-1.2 0-.2-.2-.3-.4-.3h-1c-.2 0-.4.2-.3.4.3 1.3 1.1 2 2.6 2.3v1.2c0 .2.2.4.5.4h.8c.3 0 .5-.2.5-.4v-1.2c1.6-.2 2.7-1.3 2.7-2.7z"/><path fill="#fff" d="M12.8 24.3c-4-1.4-6.2-5.7-4.7-9.8.7-2 2.3-3.6 4.3-4.3.2-.1.3-.3.3-.5v-.9c0-.2-.1-.4-.3-.4-.1 0-.1 0-.2 0-5 1.5-7.8 6.8-6.3 11.8 1 3.1 3.4 5.5 6.5 6.4.2.1.4 0 .5-.2 0-.1 0-.1 0-.2v-.9c0-.2-.1-.4-.1-.5-.1-.3-.1-.3 0-.5zm6.6-15.8c-.2-.1-.4 0-.5.2 0 .1 0 .1 0 .2v.9c0 .3.2.5.4.5 4 1.4 6.2 5.7 4.7 9.8-.7 2-2.3 3.6-4.3 4.3-.2.1-.3.3-.3.5v.9c0 .2.1.4.3.4.1 0 .1 0 .2 0 5-1.5 7.8-6.8 6.3-11.8-.9-3.2-3.4-5.6-6.5-6.5l-.3-.4z"/></svg>"##;

// USDT - official green Tether logo
const USDT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#26A17B"/><path fill="#fff" d="M17.9 17.9v-.1c-.1 0-1.1.1-2 .1-.7 0-1.5 0-1.9-.1v.1c-3.7-.2-6.5-.8-6.5-1.6s2.8-1.5 6.5-1.6v2.5c.4 0 1.2.1 2 .1.9 0 1.8 0 1.9-.1v-2.5c3.7.2 6.4.8 6.4 1.6 0 .8-2.7 1.4-6.4 1.6zm0-3.4v-2.3h5.2V9H8.9v3.2h5.2v2.3c-4.2.2-7.3 1-7.3 2.1s3.1 1.9 7.3 2.1v7.5h3.8v-7.5c4.1-.2 7.2-1 7.2-2.1s-3.1-1.9-7.2-2.1z"/></svg>"##;

// DAI - official golden/orange with D symbol and bars
const DAI_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#F5AC37"/><path fill="#fff" d="M17.8 9h-6.3v5.3H9v1.7h2.5V17.7H9v1.7h2.5V23h6.3c4.1 0 7.2-3.1 7.2-7s-3.1-7-7.2-7zm0 12.3h-4.5v-3.6h4.5c1.9 0 3.5-1.5 3.5-3.5h-8V12h8c-.2-2-1.8-3.3-3.5-3.3h-4.5V12h-1.8v2h1.8v1.7h-1.8v2h1.8v3.6h-1.8v-2h1.8v2h4.5c2 0 3.6-1.6 3.6-3.6s-1.6-3.4-3.6-3.4z"/><path fill="#fff" d="M9 14.3h2.5v1.7H9zm0 3.4h2.5v1.7H9z"/></svg>"##;

// WBTC - wrapped bitcoin with ring
const WBTC_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><circle cx="16" cy="16" r="16" fill="#201A2D"/><circle cx="16" cy="16" r="13" fill="none" stroke="#F7931A" stroke-width="1.5"/><path fill="#F7931A" d="M20.3 14.2c.2-1.5-.6-2.3-1.6-2.8l.3-1.3-.9-.2-.3 1.2c-.2 0-.5-.1-.7-.1l.3-1.2-.8-.2-.3 1.2c-.2 0-.4-.1-.5-.1l-1.2-.3-.2 1s.6.1.6.2c.3.1.4.3.4.5l-.4 1.7v.1l-.6 2.3c0 .1-.2.3-.4.2l-.6-.1-.4 1 1 .2c.2 0 .4.1.6.1l-.3 1.3.9.2.3-1.3c.2.1.4.1.7.2l-.3 1.2.8.2.3-1.3c1.4.3 2.4.2 2.9-1.1.4-1 0-1.6-.8-2 .6-.2.9-.6 1-1.3zm-1.9 2.6c-.3 1-1.9.5-2.5.3l.5-1.8c.6.2 2.3.4 2 1.5zm.2-2.7c-.2.9-1.6.4-2.1.3l.4-1.6c.5.1 2 .4 1.7 1.3z"/></svg>"##;

// Ergo - official ERG logo
const ERG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 58.4 58.4"><style>.st0{fill-rule:evenodd;clip-rule:evenodd;}</style><g><g transform="translate(-70.000000, -35.000000)"><g transform="translate(70.000000, 34.000000)"><g transform="translate(0.000000, 0.830280)"><path class="st0" d="M11.2,47.8l18.3,7.4l18.2-7.8L55,29.1l-7.8-18.2L28.9,3.6l-18.2,7.8L3.4,29.7L11.2,47.8z M29.5,58.6c-0.2,0-0.4,0-0.6-0.1L9.3,50.6c-0.4-0.2-0.7-0.5-0.9-0.9L0.1,30.3c-0.2-0.4-0.2-0.8,0-1.3L8,9.5c0.2-0.4,0.5-0.7,0.9-0.9l19.4-8.3c0.4-0.2,0.8-0.2,1.3,0l19.6,7.9C49.5,8.3,49.8,8.6,50,9l8.3,19.4c0.2,0.4,0.2,0.8,0,1.3l-7.9,19.6c-0.2,0.4-0.5,0.7-0.9,0.9l-19.4,8.3C30,58.6,29.7,58.6,29.5,58.6z"/><polygon class="st0" points="33.4,29.1 25.6,37.7 38,37.7 38,41.5 20.5,41.5 20.5,37.7 28.3,29.1 20.5,21 20.5,17.3 38,17.3 38,21 25.8,21"/></g></g></g></g></svg>"##;

// WMTX - Official WMTX logo (yellow with black W design)
const WMTX_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"><path d="M0 0 C21.05869078 17.093197 34.79886766 40.7053379 37.67016602 67.89477539 C39.27022146 98.77393305 30.83633542 125.30445488 10.29907227 148.58227539 C-6.89091433 166.64728594 -31.21502295 178.32168367 -56.29858398 179.09790039 C-86.56059017 179.52643245 -112.3327929 170.06379505 -134.45483398 149.26977539 C-154.56156709 128.34569542 -162.8727353 102.08699046 -162.54467773 73.48461914 C-161.82683432 47.01358596 -149.07060052 22.49625417 -130.20483398 4.36743164 C-93.33684909 -27.34103906 -38.84324796 -30.57455002 0 0 Z " fill="#FEF432" transform="translate(162.329833984375,21.105224609375)"/><path d="M0 0 C-4.07770806 12.78499443 -8.53151521 25.42673965 -13.13763428 38.02947998 C-13.74818253 39.70164806 -14.35634602 41.37468903 -14.9619751 43.04864502 C-26.47606426 74.82956127 -26.47606426 74.82956127 -41.66796875 82.0546875 C-50.46605423 85.62108061 -60.67095788 85.06573285 -69.36328125 81.44921875 C-79.14086176 76.0751708 -84.53653324 68.39040029 -88 58 C-88.83768794 46.67061398 -88.08157966 37.57237646 -80.5625 28.625 C-72.12575993 19.74234344 -64.13863175 16.57344795 -52 16 C-53.87546231 23.2579356 -56.05444894 30.30947402 -58.65234375 37.33984375 C-58.99374893 38.26922256 -59.33515411 39.19860138 -59.68690491 40.15614319 C-60.40290338 42.09893103 -61.12146984 44.04077429 -61.8425293 45.98168945 C-62.94231594 48.95050036 -64.02502398 51.92520291 -65.10742188 54.90039062 C-65.80886027 56.79725826 -66.5112988 58.69375642 -67.21484375 60.58984375 C-67.53476791 61.47526352 -67.85469208 62.36068329 -68.18431091 63.27293396 C-69.84356494 67.68421342 -71.21083755 70.96625689 -75 74 C-69.55200186 73.56416015 -66.23877149 72.48521211 -62 69 C-57.41085072 63.33048849 -55.52619718 56.57580621 -53.1875 49.75 C-52.30764975 47.23026909 -51.42113559 44.71289374 -50.53515625 42.1953125 C-50.10477051 40.96844727 -49.67438477 39.74158203 -49.23095703 38.47753906 C-47.58818609 33.8365971 -45.88792577 29.22153413 -44.125 24.625 C-43.73284302 23.57650879 -43.73284302 23.57650879 -43.33276367 22.50683594 C-39.94577999 13.76553319 -34.80938189 7.31825666 -26.66796875 2.66796875 C-18.10507762 -1.0544555 -9.13596787 -0.36182051 0 0 Z " fill="#000000" transform="translate(117,66)"/><path d="M0 0 C1.44149414 0.02707031 1.44149414 0.02707031 2.91210938 0.0546875 C5.25499227 0.10131204 7.5956829 0.16671831 9.9375 0.25 C6.63521201 9.6814503 3.28846553 19.09667384 -0.07989502 28.50469971 C-0.76424469 30.4167044 -1.44790856 32.32895464 -2.13104248 34.24139404 C-4.46617752 40.77619117 -6.81639794 47.30530234 -9.18267822 53.82888794 C-9.97367765 56.01556506 -10.7604658 58.20377041 -11.54290771 60.39352417 C-12.65900688 63.51405056 -13.79062921 66.62854954 -14.92578125 69.7421875 C-15.25368149 70.66930038 -15.58158173 71.59641327 -15.91941833 72.55162048 C-19.75346235 82.95555991 -25.51640176 92.26471687 -35.5625 97.5703125 C-45.61351579 101.74849639 -55.69729779 101.56234149 -65.73828125 97.47265625 C-68.89751982 95.81073747 -70.83199035 94.01890854 -73.0625 91.25 C-72.49402344 91.16363281 -71.92554687 91.07726562 -71.33984375 90.98828125 C-65.64968156 89.95284272 -60.92877722 88.7480835 -57.0625 84.25 C-54.50311994 79.80290039 -52.80347066 75.05915124 -51.0625 70.25 C-50.70285156 69.25919434 -50.34320313 68.26838867 -49.97265625 67.24755859 C-47.84515887 61.35770418 -45.7460096 55.45802304 -43.65625 49.5546875 C-41.22288026 42.68988919 -38.76543596 35.83517952 -36.25 29 C-35.90155029 28.03143066 -35.55310059 27.06286133 -35.1940918 26.06494141 C-31.29252826 15.54009548 -26.40141781 7.15112727 -16.01953125 2.09375 C-10.65681185 -0.22319028 -5.7619981 -0.20850654 0 0 Z " fill="#000000" transform="translate(159.0625,49.75)"/></svg>"##;

/// Get the SVG content for a token icon.
#[must_use]
pub fn get_token_svg(token: TokenIcon) -> &'static str {
    match token {
        TokenIcon::BTC => BTC_SVG,
        TokenIcon::ETH => ETH_SVG,
        TokenIcon::SOL => SOL_SVG,
        TokenIcon::DOT => DOT_SVG,
        TokenIcon::ADA => ADA_SVG,
        TokenIcon::XRP => XRP_SVG,
        TokenIcon::USDC => USDC_SVG,
        TokenIcon::USDT => USDT_SVG,
        TokenIcon::DAI => DAI_SVG,
        TokenIcon::WBTC => WBTC_SVG,
        TokenIcon::ERG => ERG_SVG,
        TokenIcon::WMTX => WMTX_SVG,
    }
}

/// Token icon component for rendering cryptocurrency token icons.
///
/// # Example
///
/// ```ignore
/// use ember_fx_icons::{Token, TokenIcon};
///
/// view! {
///     <Token icon=TokenIcon::BTC class="w-6 h-6" />
/// }
/// ```
#[component]
pub fn Token(
    /// Token icon to render.
    icon: TokenIcon,
    /// CSS classes for the icon wrapper.
    #[prop(optional, into)]
    class: Option<String>,
    /// Accessible label for screen readers.
    #[prop(optional, into)]
    aria_label: Option<String>,
) -> impl IntoView {
    let svg_content = get_token_svg(icon);
    let class = class.unwrap_or_default();
    let label = aria_label.unwrap_or_else(|| icon.name().to_string());

    view! {
        <span
            class=format!("fx-token-icon {}", class)
            role="img"
            aria-label=label
            inner_html=svg_content
        />
    }
}
