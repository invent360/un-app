//! JobProgressPanel Leptos component.
//!
//! A panel displaying multiple job progress bars with status colors.

use leptos::prelude::*;
use crate::try_use_theme;

/// Job status determining the color of the progress bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JobStatus {
    /// Successful/Complete (green).
    Success,
    /// In progress/normal (blue).
    #[default]
    InProgress,
    /// Warning state (yellow/amber).
    Warning,
    /// Failed/error state (red).
    Error,
}

impl JobStatus {
    /// Get the bar color for this status.
    pub fn color(&self) -> &'static str {
        match self {
            Self::Success => "#22c55e",   // Green
            Self::InProgress => "#3b82f6", // Blue
            Self::Warning => "#d97706",    // Amber/Yellow
            Self::Error => "#ef4444",      // Red
        }
    }

    /// Get the text color for this status.
    pub fn text_color(&self) -> &'static str {
        match self {
            Self::Success => "#22c55e",
            Self::InProgress => "#60a5fa",
            Self::Warning => "#fbbf24",
            Self::Error => "#f87171",
        }
    }
}

/// A single job progress entry.
#[derive(Debug, Clone)]
pub struct JobProgress {
    /// Job name/label.
    pub name: String,
    /// Progress percentage (0-100).
    pub progress: f64,
    /// Job status.
    pub status: JobStatus,
}

impl JobProgress {
    /// Create a new job progress entry.
    pub fn new(name: impl Into<String>, progress: f64, status: JobStatus) -> Self {
        Self {
            name: name.into(),
            progress: progress.clamp(0.0, 100.0),
            status,
        }
    }

    /// Create a successful job.
    pub fn success(name: impl Into<String>, progress: f64) -> Self {
        Self::new(name, progress, JobStatus::Success)
    }

    /// Create an in-progress job.
    pub fn in_progress(name: impl Into<String>, progress: f64) -> Self {
        Self::new(name, progress, JobStatus::InProgress)
    }

    /// Create a warning job.
    pub fn warning(name: impl Into<String>, progress: f64) -> Self {
        Self::new(name, progress, JobStatus::Warning)
    }

    /// Create an error job.
    pub fn error(name: impl Into<String>, progress: f64) -> Self {
        Self::new(name, progress, JobStatus::Error)
    }
}

/// Configuration for the job progress panel.
#[derive(Debug, Clone)]
pub struct JobProgressConfig {
    /// Bar height in pixels.
    pub bar_height: u32,
    /// Gap between rows.
    pub row_gap: u32,
    /// Show percentage labels.
    pub show_percentage: bool,
    /// Label width in pixels.
    pub label_width: u32,
    /// Animate progress bars.
    pub animate: bool,
}

impl Default for JobProgressConfig {
    fn default() -> Self {
        Self {
            bar_height: 24,
            row_gap: 12,
            show_percentage: true,
            label_width: 100,
            animate: true,
        }
    }
}

/// JobProgressPanel component.
///
/// Displays a list of jobs with horizontal progress bars colored by status.
///
/// # Example
///
/// ```ignore
/// use ember_fx_components::observability::{JobProgressPanel, JobProgress, JobStatus};
///
/// let jobs = vec![
///     JobProgress::new("Cache Clear", 41.0, JobStatus::Warning),
///     JobProgress::new("Backup", 36.0, JobStatus::Warning),
///     JobProgress::new("Server Restart", 12.0, JobStatus::Error),
///     JobProgress::success("Deployment", 82.0),
/// ];
///
/// view! {
///     <JobProgressPanel
///         title="Job Completion status".to_string()
///         jobs=Signal::derive(move || jobs.clone())
///     />
/// }
/// ```
#[component]
pub fn JobProgressPanel(
    /// Panel title.
    #[prop(optional, into)]
    title: Option<String>,
    /// List of job progress entries.
    #[prop(into)]
    jobs: Signal<Vec<JobProgress>>,
    /// Configuration.
    #[prop(optional)]
    config: Option<JobProgressConfig>,
    /// Additional CSS classes.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme_ctx = try_use_theme();
    let design_system = theme_ctx
        .map(|ctx| ctx.class_prefix())
        .unwrap_or("ant");

    let config = config.unwrap_or_default();
    let bar_height = config.bar_height;
    let row_gap = config.row_gap;
    let show_percentage = config.show_percentage;
    let label_width = config.label_width;
    let animate = config.animate;

    let prefix = format!("fx-job-progress-{}", design_system);

    let combined_class = {
        let prefix = prefix.clone();
        let class = class.clone();
        move || {
            let mut parts = vec![prefix.clone()];
            if let Some(ref custom) = class {
                parts.push(custom.clone());
            }
            parts.join(" ")
        }
    };

    view! {
        <div
            class=combined_class
            style="background: var(--fx-color-bg-container, #1f1f1f); border-radius: 8px; padding: 16px;"
        >
            // Title
            {title.clone().map(|t| {
                view! {
                    <div style="color: var(--fx-color-text, #fff); font-size: 16px; font-weight: 500; margin-bottom: 16px; text-align: center;">
                        {t}
                    </div>
                }
            })}

            // Job list
            <div style=format!("display: flex; flex-direction: column; gap: {}px;", row_gap)>
                {move || {
                    jobs.get().into_iter().map(|job| {
                        let bar_color = job.status.color();
                        let text_color = job.status.text_color();
                        let progress = job.progress;
                        let name = job.name.clone();

                        let transition = if animate {
                            "transition: width 0.5s ease-out;"
                        } else {
                            ""
                        };

                        view! {
                            <div style="display: flex; align-items: center; gap: 12px;">
                                // Job name
                                <div style=format!(
                                    "width: {}px; color: var(--fx-color-text-secondary, #a0a0a0); font-size: 14px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                    label_width
                                )>
                                    {name}
                                </div>

                                // Progress bar container
                                <div style=format!(
                                    "flex: 1; height: {}px; background: var(--fx-color-bg-elevated, #2a2a2a); border-radius: 4px; overflow: hidden;",
                                    bar_height
                                )>
                                    // Progress fill
                                    <div style=format!(
                                        "height: 100%; width: {:.1}%; background: {}; border-radius: 4px; {}",
                                        progress, bar_color, transition
                                    )></div>
                                </div>

                                // Percentage
                                {show_percentage.then(|| {
                                    view! {
                                        <div style=format!(
                                            "min-width: 50px; color: {}; font-size: 16px; font-weight: 600; text-align: right;",
                                            text_color
                                        )>
                                            {format!("{:.0}%", progress)}
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}
