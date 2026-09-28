//! Footer component

use leptos::prelude::*;
use crate::hooks::t;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer-content">
                <div class="footer-brand">
                    <a href="/" class="logo">
                        <img src="/assets/djed.png" alt="DJED Nodes" class="logo-img" />
                    </a>
                    <div class="footer-downloads">
                        <a href="https://apps.apple.com/gb/app/unity-network-app/id6755482738" target="_blank" rel="noopener noreferrer" class="footer-download-btn" aria-label="App Store">
                            <svg viewBox="0 0 24 24" fill="currentColor">
                                <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
                            </svg>
                        </a>
                        <a href="https://play.google.com/store/apps/details?id=io.unetwork.app&pcampaignid=web_share" target="_blank" rel="noopener noreferrer" class="footer-download-btn" aria-label="Google Play">
                            <svg viewBox="0 0 24 24" fill="currentColor">
                                <path d="M3 20.5v-17c0-.59.34-1.11.84-1.35L13.69 12l-9.85 9.85c-.5-.25-.84-.76-.84-1.35zm13.81-5.38L6.05 21.34l8.49-8.49 2.27 2.27zm3.35-4.31c.34.27.59.69.59 1.19s-.22.9-.57 1.18l-2.29 1.32-2.5-2.5 2.5-2.5 2.27 1.31zM6.05 2.66l10.76 6.22-2.27 2.27L6.05 2.66z"/>
                            </svg>
                        </a>
                        <a href="https://releases.unetwork.io/android/" target="_blank" rel="noopener noreferrer" class="footer-download-btn" aria-label="APK Download">
                            <svg viewBox="0 0 24 24" fill="currentColor">
                                <path d="M5 16c0 3.87 3.13 7 7 7s7-3.13 7-7v-4H5v4zM16.12 4.37l2.1-2.1-.82-.83-2.3 2.31C14.16 3.28 13.12 3 12 3s-2.16.28-3.09.75L6.6 1.44l-.82.83 2.1 2.1C6.14 5.64 5 7.68 5 10v1h14v-1c0-2.32-1.14-4.36-2.88-5.63zM9 9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z"/>
                            </svg>
                        </a>
                    </div>
                </div>

                <div class="footer-links">
                    <div class="footer-column">
                        <h4>{move || t("footer.quick_links")}</h4>
                        <a href="#how-it-works">{move || t("footer.how_it_works")}</a>
                        <a href="#earnings">{move || t("footer.earnings")}</a>
                        <a href="#faq">{move || t("footer.faq")}</a>
                    </div>
                    <div class="footer-column">
                        <h4>{move || t("footer.community")}</h4>
                        <a href="https://discord.gg/unetwork" target="_blank">"Discord"</a>
                        <a href="https://t.me/unetwork" target="_blank">"Telegram"</a>
                        <a href="https://twitter.com/unetwork" target="_blank">"Twitter"</a>
                    </div>
                    <div class="footer-column">
                        <h4>{move || t("footer.support")}</h4>
                        <a href="/contact">{move || t("footer.contact")}</a>
                        <a href="/privacy">{move || t("footer.privacy")}</a>
                        <a href="/terms">{move || t("footer.terms")}</a>
                    </div>
                </div>

                <div class="footer-bottom">
                    <p class="footer-copyright">
                        <img src="/assets/djed-icon.png" alt="" class="footer-djed-icon" />
                        {move || t("footer.copyright")}
                    </p>
                    <a href="https://unetwork.io" target="_blank" rel="noopener noreferrer" class="footer-powered-by">
                        <span>"Powered by"</span>
                        <img src="/assets/unetwork.png" alt="UNetwork" class="powered-by-logo" />
                    </a>
                </div>
            </div>
        </footer>
    }
}
