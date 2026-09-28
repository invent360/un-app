//! Accordion/Collapse component - re-exports from ember-fx
//!
//! This module provides Collapse components from ember-fx with
//! full feature support (bordered, ghost styles, icon positioning, animations).

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::layout::{
    Collapse,
    CollapsePanel,
    CollapseIconPosition,
};

/// Accordion item data for backward compatibility.
#[derive(Clone)]
pub struct AccordionItem {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        }
    }
}

// For backward compatibility, provide an Accordion component that uses Collapse
#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
mod compat {
    use super::*;
    use leptos::prelude::*;

    /// Accordion component using ember-fx Collapse internally.
    ///
    /// For full control, use `Collapse` and `CollapsePanel` directly.
    #[component]
    pub fn Accordion(
        items: Vec<AccordionItem>,
        #[prop(optional)] allow_multiple: bool,
        #[prop(optional)] bordered: bool,
        #[prop(optional)] ghost: bool,
        #[prop(optional, into)] default_active_keys: Option<Vec<String>>,
    ) -> impl IntoView {
        let active_keys = RwSignal::new(default_active_keys.unwrap_or_default());
        let accordion_mode = !allow_multiple;

        view! {
            <Collapse
                active_keys=active_keys
                accordion=accordion_mode
                bordered=bordered
                ghost=ghost
            >
                {items.into_iter().map(|item| {
                    let id = item.id.clone();
                    let title = item.title.clone();
                    let content = item.content.clone();
                    view! {
                        <CollapsePanel key=id header=title>
                            <p>{content}</p>
                        </CollapsePanel>
                    }
                }).collect_view()}
            </Collapse>
        }
    }
}

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use compat::Accordion;

// Fallback for non-ember-fx builds
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
mod fallback {
    use super::AccordionItem;
    use leptos::prelude::*;

    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum CollapseIconPosition {
        #[default]
        Start,
        End,
    }

    #[component]
    pub fn Accordion(
        items: Vec<AccordionItem>,
        #[prop(optional)] allow_multiple: bool,
    ) -> impl IntoView {
        let active_keys = RwSignal::new(Vec::<String>::new());

        let toggle_item = move |id: String| {
            active_keys.update(|keys| {
                if keys.contains(&id) {
                    keys.retain(|i| i != &id);
                } else {
                    if !allow_multiple {
                        keys.clear();
                    }
                    keys.push(id);
                }
            });
        };

        view! {
            <div class="accordion">
                {items.into_iter().map(|item| {
                    let id = item.id.clone();
                    let id_for_click = id.clone();
                    let id_for_open = id.clone();
                    let id_for_icon = id.clone();
                    let id_for_content = id.clone();
                    let content = item.content.clone();

                    view! {
                        <div
                            class="accordion-item"
                            class:open=move || active_keys.get().contains(&id_for_open)
                        >
                            <button
                                class="accordion-header"
                                on:click=move |_| toggle_item(id_for_click.clone())
                            >
                                <span class="accordion-title">{item.title.clone()}</span>
                                <span class="accordion-icon">
                                    {move || if active_keys.get().contains(&id_for_icon) { "−" } else { "+" }}
                                </span>
                            </button>
                            {move || active_keys.get().contains(&id_for_content).then(|| view! {
                                <div class="accordion-content">
                                    <p>{content.clone()}</p>
                                </div>
                            })}
                        </div>
                    }
                }).collect_view()}
            </div>
        }
    }
}

#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
pub use fallback::*;
