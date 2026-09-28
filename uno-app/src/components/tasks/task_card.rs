//! Task information card component

use leptos::prelude::*;

/// Task type information
#[derive(Debug, Clone, PartialEq)]
pub struct TaskInfo {
    pub name: String,
    pub description: String,
    pub icon: &'static str,
    pub min_earnings: f64,
    pub max_earnings: f64,
    pub requires_wifi: bool,
    pub requires_phone: bool,
    pub is_active: bool,
}

impl TaskInfo {
    pub fn earnings_display(&self) -> String {
        format!("${:.2} - ${:.2}/day", self.min_earnings, self.max_earnings)
    }
}

/// Task card component displaying task details
#[component]
pub fn TaskCard(task: TaskInfo) -> impl IntoView {
    let is_active = task.is_active;

    view! {
        <div class="task-card" class:inactive=!is_active>
            <div class="task-header">
                <span class="task-icon">{task.icon}</span>
                <h3 class="task-name">{task.name.clone()}</h3>
                {(!is_active).then(|| view! {
                    <span class="coming-soon-badge">"Coming Soon"</span>
                })}
            </div>

            <p class="task-description">{task.description.clone()}</p>

            <div class="task-meta">
                <div class="task-earnings">
                    <span class="label">"Earnings:"</span>
                    <span class="value">{task.earnings_display()}</span>
                </div>

                <div class="task-requirements">
                    {task.requires_wifi.then(|| view! {
                        <span class="requirement wifi" title="Works on WiFi">"WiFi"</span>
                    })}
                    {task.requires_phone.then(|| view! {
                        <span class="requirement phone" title="Works on Phone">"Phone"</span>
                    })}
                </div>
            </div>
        </div>
    }
}

