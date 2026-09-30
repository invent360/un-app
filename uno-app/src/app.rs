//! Main application component and router configuration

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment, WildcardSegment,
};

use crate::routes::{
    ClaimPage, ContactPage, FaqPage, GuidesPage, HomePage, PreviewPage, ReferralsPage, TasksPage,
};

use crate::api::get_variants;
use crate::components::chatbot::ChatWidget;
use crate::components::common::LocalePopup;
use crate::components::layout::{Footer, Header};
use crate::components::wizard::{provide_wizard_context, ClaimWizard};
use crate::hooks::{provide_locale_context, provide_theme_context, t};
#[cfg(feature = "debug-routes")]
use crate::routes::DebugPage;

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
use ember_fx_components::{DesignSystem, ThemeProvider, ToastPlacement, ToastProvider};

/// Main application component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_locale_context();
    provide_theme_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/uno-app.css"/>
        <Link rel="icon" type_="image/png" href="/assets/favicon.png"/>
        <Meta name="description" content="UNO - Earn passive income by sharing your unused internet bandwidth"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Title text="UNO - License Distribution"/>

        <AppContent/>
    }
}

/// App content with conditional ember-fx wrapping
#[component]
fn AppContent() -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
    {
        view! {
            // ember-fx theme provider with full CSS injection and persistence
            // Uses dark-blue as initial theme (UNO signature), Ant design system
            <ThemeProvider initial_theme="dark-blue" design_system=DesignSystem::Ant persist=true>
                <ToastProvider placement=ToastPlacement::TopRight>
                    <AppRouter/>
                </ToastProvider>
            </ThemeProvider>
        }
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
    {
        view! {
            <AppRouter/>
        }
    }
}

/// Router component
#[component]
fn AppRouter() -> impl IntoView {
    // Provide wizard context at app level so it's available everywhere
    let wizard_state = provide_wizard_context();

    // Load variants at app level for the wizard
    let variants = Resource::new(|| (), |_| get_variants());

    // Update wizard state when variants are loaded
    let wizard_state_clone = wizard_state.clone();
    Effect::new(move |_| {
        if let Some(Ok(variant_list)) = variants.get() {
            wizard_state_clone.set_variants(variant_list);
        }
    });

    view! {
        <Router>
            <div class="app-wrapper">
                <Header />
                <main class="app-container">
                    <ApplicationRoutes/>
                </main>
                <Footer />
                <ChatWidget />
                <LocalePopup />

                // Global claim wizard modal - available on any page
                <ClaimWizard state=wizard_state.clone() />
            </div>
        </Router>
    }
}

/// Keep compile-time optional route definitions inside the same view macro.
/// A nested view expression becomes AnyView in Cargo Leptos development builds,
/// while Routes requires route definitions (AnyNestedRoute).
#[component]
fn ApplicationRoutes() -> impl IntoView {
    macro_rules! routes {
        ($($debug:tt)*) => {
            view! {
                    <Routes fallback=move || view! { <NotFound/> }>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=(StaticSegment("claim"), ParamSegment("id")) view=ClaimPage/>
                        <Route path=StaticSegment("tasks") view=TasksPage/>
                        <Route path=StaticSegment("guides") view=GuidesPage/>
                        <Route path=StaticSegment("faq") view=FaqPage/>
                        <Route path=StaticSegment("contact") view=ContactPage/>
                        <Route path=StaticSegment("referrals") view=ReferralsPage/>
                        <Route path=StaticSegment("preview") view=PreviewPage/>
                        $($debug)*
                        <Route path=WildcardSegment("any") view=NotFound/>
                    </Routes>
            }
        };
    }
    #[cfg(feature = "debug-routes")]
    {
        routes!(<Route path=StaticSegment("debug") view=DebugPage/> )
    }
    #[cfg(not(feature = "debug-routes"))]
    {
        routes!()
    }
}

/// 404 - Not Found page
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <p>{move || t("errors.page_not_found")}</p>
            <a href="/" class="btn-primary">{move || t("nav.home")}</a>
        </div>
    }
}
