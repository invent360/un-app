//! Period toggle component for stats filtering

use leptos::prelude::*;

/// Time period for statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatsPeriod {
    Daily,
    #[default]
    Weekly,
    Monthly,
    AllTime,
}

impl StatsPeriod {
    pub fn label(&self) -> &'static str {
        match self {
            StatsPeriod::Daily => "Daily",
            StatsPeriod::Weekly => "Weekly",
            StatsPeriod::Monthly => "Monthly",
            StatsPeriod::AllTime => "All Time",
        }
    }

    pub fn all() -> &'static [StatsPeriod] {
        &[
            StatsPeriod::Daily,
            StatsPeriod::Weekly,
            StatsPeriod::Monthly,
            StatsPeriod::AllTime,
        ]
    }
}

/// Period toggle component
#[component]
pub fn PeriodToggle(
    #[prop(into)] selected: Signal<StatsPeriod>,
    #[prop(into)] on_change: Callback<StatsPeriod>,
) -> impl IntoView {
    view! {
        <div class="period-toggle">
            {StatsPeriod::all().iter().map(|period| {
                let p = *period;
                let is_active = move || selected.get() == p;
                view! {
                    <button
                        class="period-btn"
                        class:active=is_active
                        on:click=move |_| on_change.run(p)
                    >
                        {p.label()}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
