use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    StaticSegment, WildcardSegment, ParamSegment,
};

use crate::context::{ThemeContextProvider, NavPositionContextProvider, UserRoleContextProvider};
use crate::state::provide_dashboard_state;
use crate::components::layout::AppShell;
use crate::pages::{
    HomePage, AnalyticsPage, SettingsPage, RewardsPage,
    NodeListPage, NodeDetailPage,
    LicenseListPage, LicenseDetailPage,
    AgentListPage, AgentDetailPage,
    ContentListPage, ContentEditorPage, ContentImportPage,
    ReviewsDashboardPage,
    PublishQueuePage,
    CmsLandingPage,
    AuditLogPage,
    SchemaListPage, SchemaEditorPage,
    HomePageEditor, FaqPageEditor,
    JobsListPage,
    MarketplaceListPage,
};

// ember-fx theme provider for Prime design system
// TODO: Re-enable when ember_fx_core dependency is added
// use ember_fx_core::{ThemeProvider, DesignSystem};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // ember-fx-styles: theme variables and component styles
        <Stylesheet id="ember-fx-themes" href="/styles/ember-fx-themes.css"/>
        <Stylesheet id="ember-fx-components" href="/styles/ember-fx-components.css"/>
        // uno-admin custom styles (Tailwind + app-specific)
        <Stylesheet id="leptos" href="/pkg/uno-admin.css"/>
        <Title text="UNO Admin"/>

        // ember-fx ThemeProvider with Prime design system
        // TODO: Re-enable ThemeProvider when ember_fx_core dependency is added
        <div class="dark">
            <ThemeContextProvider>
                <NavPositionContextProvider>
                    <UserRoleContextProvider>
                        <DashboardApp />
                    </UserRoleContextProvider>
                </NavPositionContextProvider>
            </ThemeContextProvider>
        </div>
    }
}

#[component]
fn DashboardApp() -> impl IntoView {
    // Provide dashboard state after theme context is available
    provide_dashboard_state();

    view! {
        <Router>
            <AppShell>
                <Routes fallback=move || view! { <NotFound /> }>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("marketplace") view=MarketplaceListPage />
                    <Route path=StaticSegment("analytics") view=AnalyticsPage />
                    <Route path=StaticSegment("rewards") view=RewardsPage />
                    <Route path=StaticSegment("nodes") view=NodeListPage />
                    <Route path=(StaticSegment("nodes"), ParamSegment("id")) view=NodeDetailPage />
                    <Route path=StaticSegment("licenses") view=LicenseListPage />
                    <Route path=(StaticSegment("licenses"), ParamSegment("id")) view=LicenseDetailPage />
                    <Route path=StaticSegment("agents") view=AgentListPage />
                    <Route path=(StaticSegment("agents"), ParamSegment("name")) view=AgentDetailPage />
                    <Route path=StaticSegment("cms") view=CmsLandingPage />
                    <Route path=StaticSegment("content") view=ContentListPage />
                    <Route path=(StaticSegment("content"), StaticSegment("home")) view=HomePageEditor />
                    <Route path=(StaticSegment("content"), StaticSegment("faq")) view=FaqPageEditor />
                    <Route path=(StaticSegment("content"), StaticSegment("import")) view=ContentImportPage />
                    <Route path=(StaticSegment("content"), ParamSegment("id")) view=ContentEditorPage />
                    <Route path=StaticSegment("schemas") view=SchemaListPage />
                    <Route path=(StaticSegment("schemas"), ParamSegment("id")) view=SchemaEditorPage />
                    <Route path=StaticSegment("reviews") view=ReviewsDashboardPage />
                    <Route path=StaticSegment("publish") view=PublishQueuePage />
                    <Route path=StaticSegment("audit") view=AuditLogPage />
                    <Route path=StaticSegment("jobs") view=JobsListPage />
                    <Route path=StaticSegment("settings") view=SettingsPage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </AppShell>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="flex flex-col items-center justify-center h-64 text-center">
            <h1 class="text-4xl font-bold text-slate-900 dark:text-white mb-2">"404"</h1>
            <p class="text-slate-500 dark:text-slate-400">"Page not found"</p>
            <a href="/" class="mt-4 text-primary-500 hover:text-primary-600">
                "Go back home"
            </a>
        </div>
    }
}
