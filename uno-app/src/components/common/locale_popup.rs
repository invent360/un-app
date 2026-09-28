//! Locale selection popup for bilingual countries
//!
//! This component displays a modal popup when users from bilingual countries
//! (e.g., India with English/Hindi, Kenya with English/Swahili) first visit
//! the site, allowing them to choose their preferred language.

use leptos::prelude::*;
use crate::hooks::{t, set_locale, Locale, use_locale, hide_locale_popup};
use crate::locales;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_icons::{Flag, FlagIcon, FlagVariant};

/// Get the FlagIcon for a country code
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
fn get_flag_icon(country_code: &str) -> FlagIcon {
    FlagIcon::from_code(country_code).unwrap_or(FlagIcon::Un)
}

/// Locale popup for bilingual country language selection
#[component]
pub fn LocalePopup() -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        let ctx = use_locale();

        // Check if popup should be shown
        let show_popup = move || ctx.show_locale_popup.get();
        let bilingual_opts = move || ctx.bilingual_options.get();

        let on_select = move |locale_code: String| {
            if let Some(locale) = Locale::from_code(&locale_code) {
                set_locale(locale);
            }
            hide_locale_popup();
        };

        let on_close = move |_| {
            hide_locale_popup();
        };

        view! {
            {move || {
                if !show_popup() {
                    return view! {}.into_any();
                }

                let options = match bilingual_opts() {
                    Some(opts) => opts,
                    None => return view! {}.into_any(),
                };

                let country_code = options.country_code.clone();
                let primary_name = locales::locale_display_name(&options.primary_locale);
                let secondary_name = locales::locale_display_name(&options.secondary_locale);
                let primary_code = options.primary_locale.clone();
                let secondary_code = options.secondary_locale.clone();

                let flag_icon = get_flag_icon(&country_code);

                view! {
                    <div class="locale-popup-overlay" on:click=on_close>
                        <div class="locale-popup" on:click=|e| e.stop_propagation()>
                            <div class="locale-popup-header">
                                <Flag icon=flag_icon variant=FlagVariant::Rounded class="country-flag-svg" />
                                <h2>{move || t("locale.popup_title")}</h2>
                                <p class="subtitle">{move || t("locale.popup_subtitle")}</p>
                            </div>

                            <div class="locale-options">
                                <button
                                    class="locale-option-btn primary"
                                    on:click={
                                        let code = primary_code.clone();
                                        move |_| on_select(code.clone())
                                    }
                                >
                                    <span class="locale-name">{primary_name}</span>
                                    <span class="locale-code">{primary_code.to_uppercase()}</span>
                                </button>

                                <button
                                    class="locale-option-btn secondary"
                                    on:click={
                                        let code = secondary_code.clone();
                                        move |_| on_select(code.clone())
                                    }
                                >
                                    <span class="locale-name">{secondary_name}</span>
                                    <span class="locale-code">{secondary_code.to_uppercase()}</span>
                                </button>
                            </div>

                            <button class="locale-popup-close" on:click=on_close>
                                {move || t("common.close")}
                            </button>
                        </div>
                    </div>
                }.into_any()
            }}
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <div></div>
        }
    }
}
