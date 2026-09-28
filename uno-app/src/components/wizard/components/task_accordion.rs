//! Task families accordion component

use leptos::prelude::*;
use crate::hooks::t;

/// Task family info for accordion
struct TaskFamily {
    name_key: &'static str,
    desc_key: &'static str,
    status_key: &'static str,
    earnings: &'static str,
    is_live: bool,
}

const TASK_FAMILIES: &[TaskFamily] = &[
    TaskFamily {
        name_key: "tasks.connection.name",
        desc_key: "tasks.connection.desc",
        status_key: "tasks.status.live",
        earnings: "$4 - $8",
        is_live: true,
    },
    TaskFamily {
        name_key: "tasks.connectivity.name",
        desc_key: "tasks.connectivity.desc",
        status_key: "tasks.status.live",
        earnings: "$6 - $12",
        is_live: true,
    },
    TaskFamily {
        name_key: "tasks.entropy.name",
        desc_key: "tasks.entropy.desc",
        status_key: "tasks.status.coming_soon",
        earnings: "$8 - $15",
        is_live: false,
    },
    TaskFamily {
        name_key: "tasks.cli.name",
        desc_key: "tasks.cli.desc",
        status_key: "tasks.status.coming_soon",
        earnings: "$10 - $20",
        is_live: false,
    },
    TaskFamily {
        name_key: "tasks.sms.name",
        desc_key: "tasks.sms.desc",
        status_key: "tasks.status.coming_soon",
        earnings: "$12 - $30",
        is_live: false,
    },
];

/// Task families accordion for educational content
#[component]
pub fn TaskAccordion() -> impl IntoView {
    let (open_index, set_open_index) = signal::<Option<usize>>(None);

    let toggle_item = move |index: usize| {
        set_open_index.update(|current| {
            if *current == Some(index) {
                *current = None;
            } else {
                *current = Some(index);
            }
        });
    };

    view! {
        <div class="task-accordion">
            <h3 class="accordion-title">{move || t("tasks.families.title")}</h3>
            <p class="accordion-subtitle">{move || t("tasks.families.subtitle")}</p>

            <div class="accordion-items">
                {TASK_FAMILIES.iter().enumerate().map(|(index, family)| {
                    let name_key = family.name_key;
                    let desc_key = family.desc_key;
                    let status_key = family.status_key;
                    let earnings = family.earnings;
                    let is_live = family.is_live;

                    view! {
                        <div
                            class="task-accordion-item"
                            class:open=move || open_index.get() == Some(index)
                            class:live=is_live
                        >
                            <button
                                type="button"
                                class="task-accordion-header"
                                on:click=move |_| toggle_item(index)
                            >
                                <div class="task-info">
                                    <span class="task-name">{move || t(name_key)}</span>
                                    <span class="task-status" class:live=is_live>
                                        {move || t(status_key)}
                                    </span>
                                </div>
                                <span class="accordion-icon">
                                    {move || if open_index.get() == Some(index) { "−" } else { "+" }}
                                </span>
                            </button>

                            {move || (open_index.get() == Some(index)).then(|| view! {
                                <div class="task-accordion-content">
                                    <p class="task-desc">{move || t(desc_key)}</p>
                                    <div class="task-earnings">
                                        <span class="earnings-label">{move || t("wizard.learn.earnings_potential")}</span>
                                        <span class="earnings-value">{earnings}{move || t("common.per_month")}</span>
                                    </div>
                                </div>
                            })}
                        </div>
                    }
                }).collect_view()}
            </div>

            // Maximize earnings tips
            <div class="maximize-tips">
                <h4 class="tips-title">{move || t("tasks.maximize.title")}</h4>
                <ul class="tips-list">
                    <li>{move || t("tasks.maximize.tip1")}</li>
                    <li>{move || t("tasks.maximize.tip2")}</li>
                    <li>{move || t("tasks.maximize.tip3")}</li>
                </ul>
            </div>
        </div>
    }
}
