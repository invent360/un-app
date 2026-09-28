//! FAQ category tabs component

use leptos::prelude::*;

/// Category with display name and slug
#[derive(Debug, Clone, PartialEq)]
pub struct FaqCategoryInfo {
    pub slug: String,
    pub name: String,
    pub count: Option<usize>,
}

impl FaqCategoryInfo {
    pub fn new(slug: &str, name: &str) -> Self {
        Self {
            slug: slug.to_string(),
            name: name.to_string(),
            count: None,
        }
    }

    pub fn with_count(slug: &str, name: &str, count: usize) -> Self {
        Self {
            slug: slug.to_string(),
            name: name.to_string(),
            count: Some(count),
        }
    }
}

/// Category tabs component for filtering FAQ by category
#[component]
pub fn CategoryTabs(
    /// Available categories
    categories: Vec<FaqCategoryInfo>,
    /// Currently selected category (None = All)
    selected: ReadSignal<Option<String>>,
    /// Callback when category is selected
    on_select: impl Fn(Option<String>) + Send + Sync + 'static + Copy,
) -> impl IntoView {
    let is_all_selected = move || selected.get().is_none();

    view! {
        <div class="category-tabs">
            <button
                class="category-tab"
                class:active=is_all_selected
                on:click=move |_| on_select(None)
            >
                "All"
            </button>

            {categories.into_iter().map(|cat| {
                let slug = cat.slug.clone();
                let slug_for_check = cat.slug.clone();
                let is_selected = move || selected.get().as_ref() == Some(&slug_for_check);

                view! {
                    <button
                        class="category-tab"
                        class:active=is_selected
                        on:click=move |_| on_select(Some(slug.clone()))
                    >
                        {cat.name.clone()}
                        {cat.count.map(|c| view! {
                            <span class="category-count">{format!(" ({})", c)}</span>
                        })}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

/// Get default FAQ categories
pub fn get_default_categories() -> Vec<FaqCategoryInfo> {
    vec![
        FaqCategoryInfo::new("general", "General"),
        FaqCategoryInfo::new("earnings", "Earnings"),
        FaqCategoryInfo::new("setup", "Setup"),
        FaqCategoryInfo::new("security", "Security"),
    ]
}
