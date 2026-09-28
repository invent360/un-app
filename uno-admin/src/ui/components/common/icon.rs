use leptos::prelude::*;

/// USD Coin icon component (gold dollar coin)
#[component]
pub fn UsdCoinIcon(
    #[prop(default = 16)] size: u32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 122.88 122.88"
            xmlns="http://www.w3.org/2000/svg"
            class=class
            style="display: inline-block; vertical-align: middle;"
        >
            <path fill="#fecb00" d="M61.44,0A61.46,61.46,0,1,1,18,18,61.21,61.21,0,0,1,61.44,0Z"/>
            <path fill="#d08b00" fill-rule="evenodd" d="M61.44,8.74a52.69,52.69,0,0,1,52.7,52.7c0,.31,0,.61,0,.92a50.86,50.86,0,1,0-51.77,51.77h-.92a52.7,52.7,0,0,1,0-105.4Z"/>
            <path fill="#db9300" fill-rule="evenodd" d="M63.28,12.41A50.87,50.87,0,1,1,12.41,63.28,50.87,50.87,0,0,1,63.28,12.41Z"/>
            <path fill="#b17600" d="M83.83,47.06,69.74,49.33a15.14,15.14,0,0,0-1.91-3.77A9.11,9.11,0,0,0,65,43.44V54.59q11.48,3.13,15.34,6.59A15.4,15.4,0,0,1,85.45,73a16.26,16.26,0,0,1-1.83,7.7A18.89,18.89,0,0,1,72.88,90,29.13,29.13,0,0,1,65,91.26v7.26H59.53V91.26a34.38,34.38,0,0,1-9-1.82,18,18,0,0,1-6-3.67,17.77,17.77,0,0,1-3.87-5,23.65,23.65,0,0,1-2-6.54l15.25-1.81a14.58,14.58,0,0,0,1.84,5.48,9.6,9.6,0,0,0,3.81,3V67.21a68.18,68.18,0,0,1-10.85-3.74,14.4,14.4,0,0,1-5.54-5.08,14.92,14.92,0,0,1-2.34-8.5,15.4,15.4,0,0,1,4.7-11.47c3.15-3.08,7.82-4.8,14-5.12V29.51H65V33.3q8.48.53,13,4.08a16.42,16.42,0,0,1,5.85,9.68Zm-24.3-3.81a7.1,7.1,0,0,0-3.38,2,4.33,4.33,0,0,0-1,2.81,4.64,4.64,0,0,0,1,2.94,7.08,7.08,0,0,0,3.37,2.1V43.25Zm5.49,38a8.73,8.73,0,0,0,4.7-2.42A5.58,5.58,0,0,0,71.21,75,5.16,5.16,0,0,0,70,71.69c-.84-1-2.49-2-4.93-2.83V81.23Z"/>
            <path fill="#fecb00" d="M81.19,45,67.1,47.22a14.53,14.53,0,0,0-1.91-3.68,9.19,9.19,0,0,0-2.81-2.08V52.38q11.48,3.07,15.34,6.47a14.92,14.92,0,0,1,5.09,11.61A15.77,15.77,0,0,1,81,78a18.66,18.66,0,0,1-4.68,5.75,18.87,18.87,0,0,1-6.07,3.35,30,30,0,0,1-7.85,1.22v7.12h-5.5V88.32a34.54,34.54,0,0,1-9-1.78,18.16,18.16,0,0,1-6-3.6A17.75,17.75,0,0,1,38,78a23.24,23.24,0,0,1-2-6.41l15.24-1.77a13.92,13.92,0,0,0,1.85,5.37,9.55,9.55,0,0,0,3.8,2.93V64.75A69.82,69.82,0,0,1,46,61.09a14.48,14.48,0,0,1-7.89-13.31,15,15,0,0,1,4.71-11.25q4.72-4.53,14-5V27.8h5.5v3.71q8.48.52,13,4A16.06,16.06,0,0,1,81.19,45ZM56.88,41.26a7.19,7.19,0,0,0-3.38,2,4.24,4.24,0,0,0-1,2.76,4.47,4.47,0,0,0,1,2.87,7.1,7.1,0,0,0,3.36,2.07V41.26Zm5.5,37.23a8.76,8.76,0,0,0,4.69-2.37,5.37,5.37,0,0,0,1.5-3.69,5.08,5.08,0,0,0-1.26-3.29,11.58,11.58,0,0,0-4.93-2.77V78.49Z"/>
        </svg>
    }
}

/// Icon names available in the dashboard
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IconName {
    Home,
    Analytics,
    Wallet,
    Rewards,
    Star,
    Settings,
    Moon,
    Sun,
    Menu,
    Order,
    User,
    Users,
    Revenue,
    ChevronDown,
    Plus,
    TrendUp,
    TrendDown,
    Search,
    Close,
    Server,
    Key,
    Document,
    CheckCircle,
    Rocket,
    Edit,
    Eye,
    ArrowLeft,
    ArrowBigLeftLine,
    Trash,
    ChevronRight,
    ChevronUp,
    Folder,
    Refresh,
    Clock,
    Calendar,
    Minus,
    Image,
    Play,
    Link,
    Upload,
    AlertCircle,
    Info,
    Coin,
    ChevronLeft,
    Store,
    Globe,
    Terminal,
    Copy,
    Download,
    Filter,
}

/// SVG Icon component
#[component]
pub fn Icon(
    #[prop(into)] name: IconName,
    #[prop(default = 24)] size: u32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let svg_content = match name {
        IconName::Home => view! {
            <path d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
        }.into_any(),
        IconName::Analytics => view! {
            <path d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
        }.into_any(),
        IconName::Wallet => view! {
            <path d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"/>
        }.into_any(),
        IconName::Rewards => view! {
            <path d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7"/>
        }.into_any(),
        IconName::Star => view! {
            <path d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
        }.into_any(),
        IconName::Settings => view! {
            <path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
            <path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
        }.into_any(),
        IconName::Moon => view! {
            <path d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
        }.into_any(),
        IconName::Sun => view! {
            <path d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
        }.into_any(),
        IconName::Menu => view! {
            <path d="M4 6h16M4 12h16M4 18h7"/>
        }.into_any(),
        IconName::Order => view! {
            <path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/>
        }.into_any(),
        IconName::User => view! {
            <path d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
        }.into_any(),
        IconName::Users => view! {
            <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2M9 7a4 4 0 100 8 4 4 0 000-8zM23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
        }.into_any(),
        IconName::Revenue => view! {
            <path d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
        }.into_any(),
        IconName::ChevronDown => view! {
            <path d="M19 9l-7 7-7-7"/>
        }.into_any(),
        IconName::Plus => view! {
            <path d="M12 4v16m8-8H4"/>
        }.into_any(),
        IconName::TrendUp => view! {
            <path d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
        }.into_any(),
        IconName::TrendDown => view! {
            <path d="M13 17h8m0 0v-8m0 8l-8-8-4 4-6-6"/>
        }.into_any(),
        IconName::Search => view! {
            <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
        }.into_any(),
        IconName::Close => view! {
            <path d="M6 18L18 6M6 6l12 12"/>
        }.into_any(),
        IconName::Server => view! {
            <rect x="2" y="3" width="20" height="6" rx="2" ry="2"/>
            <rect x="2" y="15" width="20" height="6" rx="2" ry="2"/>
            <circle cx="6" cy="6" r="1"/>
            <circle cx="6" cy="18" r="1"/>
        }.into_any(),
        IconName::Key => view! {
            <path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/>
        }.into_any(),
        IconName::Document => view! {
            <path d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
        }.into_any(),
        IconName::CheckCircle => view! {
            <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        }.into_any(),
        IconName::Rocket => view! {
            <path d="M13 10V3L4 14h7v7l9-11h-7z"/>
        }.into_any(),
        IconName::Edit => view! {
            <path d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
        }.into_any(),
        IconName::Eye => view! {
            <path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
            <path d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
        }.into_any(),
        IconName::ArrowLeft => view! {
            <path d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
        }.into_any(),
        IconName::ArrowBigLeftLine => view! {
            <path d="M12 15v3.586a1 1 0 0 1 -1.707 .707l-6.586 -6.586a1 1 0 0 1 0 -1.414l6.586 -6.586a1 1 0 0 1 1.707 .707v3.586h6v6h-6"/>
            <path d="M21 15v-6"/>
        }.into_any(),
        IconName::Trash => view! {
            <path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
        }.into_any(),
        IconName::ChevronRight => view! {
            <path d="M9 5l7 7-7 7"/>
        }.into_any(),
        IconName::Folder => view! {
            <path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/>
        }.into_any(),
        IconName::Refresh => view! {
            <path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
        }.into_any(),
        IconName::Clock => view! {
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 6v6l4 2"/>
        }.into_any(),
        IconName::Calendar => view! {
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
            <line x1="16" y1="2" x2="16" y2="6"/>
            <line x1="8" y1="2" x2="8" y2="6"/>
            <line x1="3" y1="10" x2="21" y2="10"/>
        }.into_any(),
        IconName::ChevronUp => view! {
            <path d="M5 15l7-7 7 7"/>
        }.into_any(),
        IconName::Minus => view! {
            <path d="M5 12h14"/>
        }.into_any(),
        IconName::Image => view! {
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
            <circle cx="8.5" cy="8.5" r="1.5"/>
            <path d="M21 15l-5-5L5 21"/>
        }.into_any(),
        IconName::Play => view! {
            <polygon points="5 3 19 12 5 21 5 3"/>
        }.into_any(),
        IconName::Link => view! {
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
        }.into_any(),
        IconName::Upload => view! {
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
        }.into_any(),
        IconName::AlertCircle => view! {
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="12"/>
            <line x1="12" y1="16" x2="12.01" y2="16"/>
        }.into_any(),
        IconName::Info => view! {
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
        }.into_any(),
        IconName::Coin => view! {
            <circle cx="12" cy="12" r="10" fill="currentColor" stroke="none"/>
            <text x="12" y="16" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e293b" stroke="none">"$"</text>
        }.into_any(),
        IconName::ChevronLeft => view! {
            <path d="M15 19l-7-7 7-7"/>
        }.into_any(),
        IconName::Store => view! {
            <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
            <path d="M9 22V12h6v10"/>
        }.into_any(),
        IconName::Globe => view! {
            <circle cx="12" cy="12" r="10"/>
            <path d="M2 12h20"/>
            <path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10 15.3 15.3 0 014-10z"/>
        }.into_any(),
        IconName::Terminal => view! {
            <polyline points="4 17 10 11 4 5"/>
            <line x1="12" y1="19" x2="20" y2="19"/>
        }.into_any(),
        IconName::Copy => view! {
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
        }.into_any(),
        IconName::Download => view! {
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
        }.into_any(),
        IconName::Filter => view! {
            <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
        }.into_any(),
    };

    view! {
        <svg
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
            {svg_content}
        </svg>
    }
}
