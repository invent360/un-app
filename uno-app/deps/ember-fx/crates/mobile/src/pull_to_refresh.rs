//! Pull to refresh component.

use leptos::prelude::*;

/// Pull to refresh container.
///
/// Wraps content and provides pull-to-refresh functionality.
///
/// # Example
///
/// ```ignore
/// use ember_fx_mobile::PullToRefresh;
///
/// let is_refreshing = RwSignal::new(false);
///
/// view! {
///     <PullToRefresh
///         refreshing=is_refreshing
///         on_refresh=Callback::new(move |_| {
///             is_refreshing.set(true);
///             // Fetch data...
///             // is_refreshing.set(false);
///         })
///     >
///         <div>"Pull down to refresh"</div>
///     </PullToRefresh>
/// }
/// ```
#[component]
pub fn PullToRefresh(
    /// Whether the refresh is currently in progress.
    #[prop(into)]
    refreshing: Signal<bool>,
    /// Callback when pull-to-refresh is triggered.
    on_refresh: Callback<()>,
    /// Pull distance threshold to trigger refresh.
    #[prop(optional)]
    threshold: Option<f64>,
    /// Maximum pull distance.
    #[prop(optional)]
    max_distance: Option<f64>,
    /// Child content.
    children: Children,
) -> impl IntoView {
    let threshold = threshold.unwrap_or(80.0);
    let max_distance = max_distance.unwrap_or(120.0);

    let pull_distance = RwSignal::new(0.0f64);
    let touch_start_y = RwSignal::new(0.0f64);
    let is_pulling = RwSignal::new(false);

    let on_touch_start = move |ev: web_sys::TouchEvent| {
        if refreshing.get() {
            return;
        }
        if let Some(touch) = ev.touches().item(0) {
            touch_start_y.set(touch.client_y() as f64);
            is_pulling.set(true);
        }
    };

    let on_touch_move = move |ev: web_sys::TouchEvent| {
        if !is_pulling.get() || refreshing.get() {
            return;
        }
        if let Some(touch) = ev.touches().item(0) {
            let current_y = touch.client_y() as f64;
            let diff = current_y - touch_start_y.get();

            if diff > 0.0 {
                // Apply resistance to make pull feel natural
                let resistance = 0.5;
                let distance = (diff * resistance).min(max_distance);
                pull_distance.set(distance);
            }
        }
    };

    let on_touch_end = move |_: web_sys::TouchEvent| {
        if !is_pulling.get() {
            return;
        }
        is_pulling.set(false);

        let distance = pull_distance.get();
        if distance >= threshold && !refreshing.get() {
            on_refresh.run(());
        }

        pull_distance.set(0.0);
    };

    view! {
        <div
            class="fx-pull-to-refresh"
            style="position: relative; overflow: hidden;"
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
        >
            // Refresh indicator
            <div
                class="fx-ptr-indicator"
                style=move || {
                    let distance = if refreshing.get() { 56.0 } else { pull_distance.get() };
                    format!(
                        "position: absolute; top: 0; left: 0; right: 0; height: 56px; display: flex; align-items: center; justify-content: center; transform: translateY({}px); transition: {};",
                        distance - 56.0,
                        if is_pulling.get() { "none" } else { "transform 0.3s ease-out" }
                    )
                }
            >
                {move || {
                    if refreshing.get() {
                        view! {
                            <div
                                class="fx-ptr-spinner"
                                style="width: 24px; height: 24px; border: 2px solid var(--fx-color-primary, #1890ff); border-top-color: transparent; border-radius: 50%; animation: fx-spin 0.8s linear infinite;"
                            />
                        }.into_any()
                    } else {
                        let distance = pull_distance.get();
                        let progress = (distance / threshold).min(1.0);
                        let rotation = progress * 180.0;

                        view! {
                            <svg
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                style=format!(
                                    "width: 24px; height: 24px; color: var(--fx-text-secondary, #8c8c8c); transform: rotate({}deg); transition: transform 0.1s;",
                                    rotation
                                )
                            >
                                <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/>
                            </svg>
                        }.into_any()
                    }
                }}
            </div>

            // Content
            <div
                class="fx-ptr-content"
                style=move || {
                    let distance = if refreshing.get() { 56.0 } else { pull_distance.get() };
                    format!(
                        "transform: translateY({}px); transition: {};",
                        distance,
                        if is_pulling.get() { "none" } else { "transform 0.3s ease-out" }
                    )
                }
            >
                {children()}
            </div>
        </div>
    }
}
