//! Task-based earnings section component
//!
//! Displays available tasks with their earnings rates and device compatibility,
//! replacing the tier-based ($5-75) pricing model.

use leptos::prelude::*;
use crate::hooks::t;

/// Task availability status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Currently available
    Current,
    /// Coming soon / projected
    Projected,
}

/// Supported device types for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Android via Play Store
    AndroidPlay,
    /// Android via direct APK
    AndroidApk,
    /// iPhone / iOS
    IPhone,
    /// PC / Desktop / Laptop
    PC,
}

impl DeviceType {
    /// Get display name for the device type
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceType::AndroidPlay => "Android",
            DeviceType::AndroidApk => "Android APK",
            DeviceType::IPhone => "iPhone",
            DeviceType::PC => "PC",
        }
    }

    /// Get CSS class for the device icon
    pub fn icon_class(&self) -> &'static str {
        match self {
            DeviceType::AndroidPlay => "device-icon-android",
            DeviceType::AndroidApk => "device-icon-android-apk",
            DeviceType::IPhone => "device-icon-ios",
            DeviceType::PC => "device-icon-pc",
        }
    }
}

/// Task information for display
#[derive(Debug, Clone)]
pub struct TaskItem {
    /// Unique identifier
    pub id: &'static str,
    /// Display name translation key
    pub name_key: &'static str,
    /// Rate per day in USD
    pub rate_per_day: f64,
    /// Current or projected
    pub status: TaskStatus,
    /// Compatible devices
    pub devices: &'static [DeviceType],
    /// Optional note (e.g., "APK only")
    pub note_key: Option<&'static str>,
}

/// Static task definitions - these represent the current task catalogue
const TASKS: &[TaskItem] = &[
    TaskItem {
        id: "telemetry",
        name_key: "home.tasks.telemetry_name",
        rate_per_day: 0.10,
        status: TaskStatus::Current,
        devices: &[DeviceType::AndroidPlay, DeviceType::AndroidApk, DeviceType::IPhone],
        note_key: None,
    },
    TaskItem {
        id: "ugrid",
        name_key: "home.tasks.ugrid_name",
        rate_per_day: 0.10,
        status: TaskStatus::Current,
        devices: &[DeviceType::AndroidApk],
        note_key: Some("home.tasks.apk_only"),
    },
    TaskItem {
        id: "entropy",
        name_key: "home.tasks.entropy_name",
        rate_per_day: 0.10,
        status: TaskStatus::Projected,
        devices: &[DeviceType::AndroidPlay, DeviceType::AndroidApk, DeviceType::IPhone],
        note_key: None,
    },
    TaskItem {
        id: "gpu",
        name_key: "home.tasks.gpu_name",
        rate_per_day: 0.10,
        status: TaskStatus::Projected,
        devices: &[DeviceType::AndroidPlay, DeviceType::AndroidApk, DeviceType::PC],
        note_key: None,
    },
];

/// Task-based earnings section
/// Replaces the tier-based ($5-75) section with actual task catalogue
#[component]
pub fn TaskBasedEarningsSection() -> impl IntoView {
    // Split tasks into current and projected
    let current_tasks: Vec<&TaskItem> = TASKS.iter().filter(|t| t.status == TaskStatus::Current).collect();
    let projected_tasks: Vec<&TaskItem> = TASKS.iter().filter(|t| t.status == TaskStatus::Projected).collect();

    view! {
        <section id="earnings" class="section earnings task-based-earnings">
            <div class="container">
                <h2 class="section-title">{move || t("home.tasks.section_title")}</h2>
                <p class="section-subtitle">{move || t("home.tasks.section_subtitle")}</p>

                // Current Tasks
                <div class="task-category">
                    <h3 class="task-category-title">{move || t("home.tasks.current")}</h3>
                    <div class="task-grid">
                        {current_tasks.into_iter().map(|task| {
                            view! { <TaskCard task=task /> }
                        }).collect_view()}
                    </div>
                </div>

                // Coming Soon Tasks
                <div class="task-category coming-soon">
                    <h3 class="task-category-title">{move || t("home.tasks.coming_soon")}</h3>
                    <div class="task-grid">
                        {projected_tasks.into_iter().map(|task| {
                            view! { <TaskCard task=task /> }
                        }).collect_view()}
                    </div>
                </div>

                // Stack note
                <div class="task-stack-note">
                    <span class="stack-icon">"💡"</span>
                    <span class="stack-text">{move || t("home.tasks.stack_note")}</span>
                </div>

                // Disclaimer
                <p class="earnings-disclaimer">
                    {move || t("home.tasks.disclaimer")}
                </p>
            </div>
        </section>
    }
}

/// Individual task card component
#[component]
fn TaskCard(task: &'static TaskItem) -> impl IntoView {
    let is_projected = task.status == TaskStatus::Projected;
    let rate_display = if is_projected {
        format!("~${:.2}", task.rate_per_day)
    } else {
        format!("${:.2}", task.rate_per_day)
    };

    view! {
        <div class="task-card" class:coming-soon=is_projected>
            <div class="task-card-header">
                <h4 class="task-name">{move || t(task.name_key)}</h4>
                {is_projected.then(|| view! {
                    <span class="task-badge coming-soon">{move || t("home.tasks.coming_soon_badge")}</span>
                })}
            </div>

            <div class="task-rate">
                <span class="rate-amount">{rate_display}</span>
                <span class="rate-period">{move || t("home.tasks.per_day")}</span>
            </div>

            <div class="task-devices">
                {task.devices.iter().map(|device| {
                    let icon_class = format!("device-icon {}", device.icon_class());
                    view! {
                        <span class="device-badge" title=device.display_name()>
                            <span class=icon_class></span>
                            {device.display_name()}
                        </span>
                    }
                }).collect_view()}
            </div>

            {task.note_key.map(|key| view! {
                <div class="task-note">
                    <span class="note-icon">"ℹ️"</span>
                    <span class="note-text">{move || t(key)}</span>
                </div>
            })}
        </div>
    }
}
