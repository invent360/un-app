//! Language selector dropdown component

use leptos::prelude::*;
use crate::hooks::{use_locale, set_locale, Locale};

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_icons::{Flag, FlagIcon, FlagVariant};

/// Map locale code to primary country flag
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
fn locale_to_flag(locale: Locale) -> FlagIcon {
    match locale {
        Locale::En => FlagIcon::Gb,  // British English
        Locale::Es => FlagIcon::Es,  // Spain
        Locale::Tl => FlagIcon::Ph,  // Philippines (Tagalog)
        Locale::Hi => FlagIcon::In,  // India (Hindi)
        Locale::Sw => FlagIcon::Ke,  // Kenya (Swahili)
        Locale::Pt => FlagIcon::Br,  // Brazil (Portuguese)
        Locale::Fr => FlagIcon::Fr,  // France
        Locale::Ar => FlagIcon::Sa,  // Saudi Arabia (Arabic)
        Locale::Id => FlagIcon::Id,  // Indonesia
        Locale::Bn => FlagIcon::Bd,  // Bangladesh (Bengali/Bangla)
    }
}

/// Language selector dropdown
#[component]
pub fn LanguageSelector() -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        let ctx = use_locale();
        let (is_open, set_is_open) = signal(false);

        let toggle_dropdown = move |_| {
            set_is_open.update(|open| *open = !*open);
        };

        let select_locale = move |locale: Locale| {
            set_locale(locale);
            set_is_open.set(false);
        };

        view! {
            <div class="language-selector">
                <button class="language-toggle" on:click=toggle_dropdown>
                    {move || {
                        let flag = locale_to_flag(ctx.locale.get());
                        view! {
                            <Flag icon=flag variant=FlagVariant::Circular class="language-flag language-flag--sm" />
                        }
                    }}
                    <span class="current-language">{move || ctx.locale.get().code().to_uppercase()}</span>
                    <span class="dropdown-arrow">{move || if is_open.get() { "▲" } else { "▼" }}</span>
                </button>

                {move || is_open.get().then(|| view! {
                    // Backdrop to capture clicks outside dropdown
                    <div class="language-backdrop" on:click=move |_| set_is_open.set(false)></div>
                    <div class="language-dropdown">
                        {Locale::all().iter().map(|locale| {
                            let loc = *locale;
                            let is_current = move || ctx.locale.get() == loc;
                            let flag = locale_to_flag(loc);

                            view! {
                                <button
                                    class="language-option"
                                    class:active=is_current
                                    on:click=move |_| select_locale(loc)
                                >
                                    <Flag icon=flag variant=FlagVariant::Circular class="language-flag language-flag--xs" />
                                    <span class="locale-code">{loc.code().to_uppercase()}</span>
                                    <span class="locale-name">{loc.name()}</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>
                })}
            </div>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <div class="language-selector">
                <span>"Language selector requires csr, hydrate, or ssr feature"</span>
            </div>
        }
    }
}
