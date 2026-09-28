//! Android app download component

use leptos::prelude::*;

/// Play Store app URL
const PLAY_STORE_URL: &str = "https://play.google.com/store/apps/details?id=io.unetwork.app&pcampaignid=web_share";

/// Android download component with Play Store badge and instructions
#[component]
pub fn AndroidDownload(
    /// Optional license key to show in instructions
    #[prop(optional)]
    license_key: Option<String>,
) -> impl IntoView {
    let (show_instructions, set_show_instructions) = signal(false);

    let toggle_instructions = move |_| {
        set_show_instructions.update(|v| *v = !*v);
    };

    view! {
        <div class="android-download">
            <h3 class="download-title">"Download the App"</h3>

            <div class="download-badges">
                <a
                    href=PLAY_STORE_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="play-store-badge"
                    aria-label="Get it on Google Play"
                >
                    <img
                        src="/assets/images/google-play-badge.png"
                        alt="Get it on Google Play"
                        width="180"
                        height="53"
                    />
                </a>
            </div>

            <p class="download-note">
                "Available for Android 8.0 and above"
            </p>

            <button
                class="btn-secondary instructions-toggle"
                on:click=toggle_instructions
            >
                {move || if show_instructions.get() {
                    "Hide Installation Guide"
                } else {
                    "Show Installation Guide"
                }}
            </button>

            {move || show_instructions.get().then(|| {
                let key = license_key.clone();
                view! {
                    <div class="installation-guide">
                        <h4>"Installation Steps"</h4>
                        <ol class="steps-list">
                            <li>
                                <strong>"Download"</strong>
                                " - Tap the Google Play button above to download the UNO app"
                            </li>
                            <li>
                                <strong>"Install"</strong>
                                " - Open the downloaded app and allow any required permissions"
                            </li>
                            <li>
                                <strong>"Register"</strong>
                                " - Create your account or sign in with existing credentials"
                            </li>
                            <li>
                                <strong>"Activate"</strong>
                                " - Enter your license key when prompted"
                                {key.map(|k| view! {
                                    <div class="key-reminder">
                                        <span class="label">"Your key: "</span>
                                        <code class="key-value">{k}</code>
                                    </div>
                                })}
                            </li>
                            <li>
                                <strong>"Start Earning"</strong>
                                " - Keep the app running to maximize your earnings"
                            </li>
                        </ol>

                        <div class="tips-section">
                            <h5>"Tips for Maximum Earnings"</h5>
                            <ul class="tips-list">
                                <li>"Keep your phone connected to WiFi for better performance"</li>
                                <li>"Disable battery optimization for the UNO app"</li>
                                <li>"Run the app on multiple devices for higher earnings"</li>
                            </ul>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
