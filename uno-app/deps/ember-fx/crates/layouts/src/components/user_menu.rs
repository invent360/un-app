//! User menu component for layout headers.

use leptos::prelude::*;
use crate::types::UserConfig;

/// User menu component that displays user avatar and name.
///
/// Shows the user's avatar (or initials fallback) and name in the header.
#[component]
pub fn UserMenu(
    /// User configuration.
    #[prop(into)]
    config: UserConfig,
) -> impl IntoView {
    view! {
        <div class="fx-user-menu">
            <div class="fx-user-avatar">
                {if let Some(ref url) = config.avatar_url {
                    view! {
                        <img src=url.clone() alt="Avatar" />
                    }.into_any()
                } else {
                    view! {
                        {config.avatar_text.clone().unwrap_or_else(|| "?".to_string())}
                    }.into_any()
                }}
            </div>
            {config.name.clone().map(|name| {
                view! {
                    <span class="fx-user-name">{name}</span>
                }
            })}
        </div>
    }
}
