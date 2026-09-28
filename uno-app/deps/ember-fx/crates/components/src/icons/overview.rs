//! Icons overview page.

use leptos::prelude::*;
use ember_fx_icons::{
    TokenIcon, get_token_svg,
    AnimalIcon, get_animal_svg,
    NatureIcon, get_nature_svg,
    FinanceIcon, get_finance_svg,
    CommerceIcon, get_commerce_svg,
    SocialIcon, get_social_svg,
};

/// Icons overview page showing all icon categories.
#[component]
pub fn IconsOverview() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <a href="/components" class="back-link">"<- Back to Components"</a>
                <h1 class="page-title">"Icons"</h1>
                <p class="page-subtitle">"A comprehensive collection of SVG icons for various use cases"</p>
            </header>

            <div class="component-nav">
                <a href="/components/icons/crypto" class="component-nav-item">"Crypto"</a>
                <a href="/components/icons/animals" class="component-nav-item">"Animals"</a>
                <a href="/components/icons/nature" class="component-nav-item">"Nature"</a>
                <a href="/components/icons/finance" class="component-nav-item">"Finance"</a>
                <a href="/components/icons/commerce" class="component-nav-item">"Commerce"</a>
                <a href="/components/icons/social" class="component-nav-item">"Social"</a>
            </div>

            <section class="demo-section">
                <h2>"Icon Categories"</h2>

                // Crypto tokens
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Crypto Tokens"</h3>
                        <a href="/components/icons/crypto" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Cryptocurrency and token icons for blockchain applications"</p>
                    <div class="icon-preview-grid">
                        {[TokenIcon::BTC, TokenIcon::ETH, TokenIcon::SOL, TokenIcon::DOT, TokenIcon::ADA, TokenIcon::XRP, TokenIcon::USDC, TokenIcon::WMTX]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_token_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.symbol()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>

                // Animals
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Animals"</h3>
                        <a href="/components/icons/animals" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Pet, farm, wildlife, and other animal icons"</p>
                    <div class="icon-preview-grid">
                        {[AnimalIcon::Dog, AnimalIcon::Cat, AnimalIcon::Horse, AnimalIcon::Bear, AnimalIcon::Dolphin, AnimalIcon::Butterfly]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_animal_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.name()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>

                // Nature
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Nature"</h3>
                        <a href="/components/icons/nature" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Plants, weather, farming, and landscape icons"</p>
                    <div class="icon-preview-grid">
                        {[NatureIcon::Tree, NatureIcon::Flower, NatureIcon::Sun, NatureIcon::Rain, NatureIcon::Mountain, NatureIcon::Tractor]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_nature_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.name()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>

                // Finance
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Finance"</h3>
                        <a href="/components/icons/finance" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Money, banking, trading, and business icons"</p>
                    <div class="icon-preview-grid">
                        {[FinanceIcon::Dollar, FinanceIcon::Bank, FinanceIcon::CreditCard, FinanceIcon::Wallet, FinanceIcon::ChartLine, FinanceIcon::PiggyBank]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_finance_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.name()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>

                // Commerce
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Commerce"</h3>
                        <a href="/components/icons/commerce" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Shopping, products, and e-commerce icons"</p>
                    <div class="icon-preview-grid">
                        {[CommerceIcon::Cart, CommerceIcon::ShoppingBag, CommerceIcon::Store, CommerceIcon::Package, CommerceIcon::Gift, CommerceIcon::Barcode]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_commerce_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.name()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>

                // Social
                <div class="icon-category-preview">
                    <div class="icon-category-header">
                        <h3>"Social"</h3>
                        <a href="/components/icons/social" class="view-all-link">"View all ->"</a>
                    </div>
                    <p class="icon-category-desc">"Communication, social actions, and profile icons"</p>
                    <div class="icon-preview-grid">
                        {[SocialIcon::Message, SocialIcon::Like, SocialIcon::Share, SocialIcon::Follow, SocialIcon::Globe, SocialIcon::Camera]
                            .into_iter()
                            .map(|icon| view! {
                                <div class="icon-preview-item">
                                    <span class="icon-preview-svg" inner_html=get_social_svg(icon)></span>
                                    <span class="icon-preview-name">{icon.name()}</span>
                                </div>
                            })
                            .collect_view()}
                    </div>
                </div>
            </section>
        </div>
    }
}
