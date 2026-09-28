use leptos::prelude::*;
use leptos_router::components::{Route, Routes};
use leptos_router::{StaticSegment, WildcardSegment};

use crate::pages::{HomePage, AnalyticsPage, SettingsPage};

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes fallback=move || view! { <NotFound /> }>
            <Route path=StaticSegment("") view=HomePage />
            <Route path=StaticSegment("analytics") view=AnalyticsPage />
            <Route path=StaticSegment("settings") view=SettingsPage />
            <Route path=WildcardSegment("any") view=NotFound />
        </Routes>
    }
}

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
            <p>"Page not found"</p>
        </div>
    }
}
