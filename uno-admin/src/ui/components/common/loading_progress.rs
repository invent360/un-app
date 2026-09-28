//! Loading progress bar component with percentage indicator

use leptos::prelude::*;

/// Loading progress bar with steps and percentage
#[component]
pub fn LoadingProgress(
    #[prop(into)] _title: String,
    #[prop(default = 0)] current_step: usize,
    #[prop(default = 4)] total_steps: usize,
    #[prop(optional)] _steps: Option<Vec<String>>,
) -> impl IntoView {
    let percentage = if total_steps > 0 {
        (((current_step + 1) as f64 / total_steps as f64) * 100.0).min(100.0)
    } else {
        0.0
    };

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            // Progress bar with percentage inside - uniform pill shape
            <div class="relative h-9 bg-slate-200 dark:bg-slate-700" style="border-radius: 18px;">
                <div
                    class="absolute inset-y-0 left-0 bg-cyan-500 transition-all duration-300 flex items-center justify-center"
                    style={format!("width: {}%; border-radius: 18px;", percentage.max(20.0))}
                >
                    <span class="text-white text-sm font-medium">{format!("{:.0}%", percentage)}</span>
                </div>
            </div>
        </div>
    }
}

/// Simple indeterminate loading bar (when progress is unknown)
#[component]
pub fn LoadingBar(
    #[prop(into, optional)] message: Option<String>,
) -> impl IntoView {
    let msg = message.unwrap_or_else(|| "Loading...".to_string());

    view! {
        <div class="bg-white dark:bg-slate-800 rounded-2xl p-6 border border-slate-200 dark:border-slate-700 shadow-sm">
            // Progress bar with animated fill - uniform pill shape
            <div class="relative h-9 bg-slate-200 dark:bg-slate-700" style="border-radius: 18px;">
                <div class="absolute inset-y-0 left-0 w-1/2 bg-cyan-500 animate-pulse flex items-center justify-center" style="border-radius: 18px;">
                    <span class="text-white text-sm font-medium">{msg}</span>
                </div>
            </div>
        </div>
    }
}

/// Inline loading spinner with optional text
#[component]
pub fn LoadingSpinner(
    #[prop(into, optional)] message: Option<String>,
    #[prop(default = "md")] size: &'static str,
) -> impl IntoView {
    let (spinner_class, text_class) = match size {
        "sm" => ("w-4 h-4 border-2", "text-xs"),
        "lg" => ("w-8 h-8 border-4", "text-base"),
        _ => ("w-5 h-5 border-2", "text-sm"),
    };

    view! {
        <div class="flex items-center justify-center gap-3 py-4">
            <div class={format!("{} border-primary-500 border-t-transparent rounded-full animate-spin", spinner_class)}></div>
            {message.map(|msg| view! {
                <span class={format!("{} text-slate-500 dark:text-slate-400", text_class)}>{msg}</span>
            })}
        </div>
    }
}
