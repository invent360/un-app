//! CMS Layout with horizontal menubar

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::components::common::icon::{Icon, IconName};
use crate::state::CmsTab;

/// CMS horizontal menubar component
#[component]
pub fn CmsMenuBar() -> impl IntoView {
    let location = use_location();

    view! {
        <nav class="cms-menubar">
            <div class="cms-menubar-inner">
                {CmsTab::all().iter().map(|tab| {
                    let tab = *tab;
                    let path = tab.path();
                    let label = tab.label();
                    let pathname = location.pathname.clone();
                    let is_active = move || {
                        let current = pathname.get();
                        current.starts_with(path)
                    };

                    view! {
                        <a
                            href=path
                            class=move || format!(
                                "cms-menubar-item {}",
                                if is_active() { "active" } else { "" }
                            )
                        >
                            <CmsMenuIcon tab=tab />
                            <span>{label}</span>
                        </a>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </nav>
    }
}

/// Icon for CMS menu items
#[component]
fn CmsMenuIcon(tab: CmsTab) -> impl IntoView {
    let icon_name = match tab {
        CmsTab::Content => IconName::Document,
        CmsTab::Schemas => IconName::Settings,
        CmsTab::Reviews => IconName::CheckCircle,
        CmsTab::Publish => IconName::Rocket,
    };

    view! {
        <Icon name=icon_name size=18 />
    }
}
