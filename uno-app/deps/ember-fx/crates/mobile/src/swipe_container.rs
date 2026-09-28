//! Swipe gesture container component.

use leptos::prelude::*;

/// Direction of a swipe gesture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Container with swipe gesture detection.
///
/// Detects swipe gestures and calls the appropriate callback.
///
/// # Example
///
/// ```ignore
/// use ember_fx_mobile::{SwipeContainer, SwipeDirection};
///
/// view! {
///     <SwipeContainer
///         on_swipe=Callback::new(|dir: SwipeDirection| {
///             match dir {
///                 SwipeDirection::Left => log!("Swiped left"),
///                 SwipeDirection::Right => log!("Swiped right"),
///                 _ => {}
///             }
///         })
///     >
///         <div>"Swipe me!"</div>
///     </SwipeContainer>
/// }
/// ```
#[component]
pub fn SwipeContainer(
    /// Callback when a swipe is detected.
    on_swipe: Callback<SwipeDirection>,
    /// Minimum distance in pixels to trigger a swipe.
    #[prop(optional)]
    threshold: Option<f64>,
    /// Whether vertical swipes are enabled.
    #[prop(optional)]
    enable_vertical: Option<bool>,
    /// Whether horizontal swipes are enabled.
    #[prop(optional)]
    enable_horizontal: Option<bool>,
    /// Child content.
    children: Children,
) -> impl IntoView {
    let threshold = threshold.unwrap_or(50.0);
    let enable_vertical = enable_vertical.unwrap_or(false);
    let enable_horizontal = enable_horizontal.unwrap_or(true);

    let touch_start_x = RwSignal::new(0.0f64);
    let touch_start_y = RwSignal::new(0.0f64);
    let is_swiping = RwSignal::new(false);

    let on_touch_start = move |ev: web_sys::TouchEvent| {
        if let Some(touch) = ev.touches().item(0) {
            touch_start_x.set(touch.client_x() as f64);
            touch_start_y.set(touch.client_y() as f64);
            is_swiping.set(true);
        }
    };

    let on_touch_end = move |ev: web_sys::TouchEvent| {
        if !is_swiping.get() {
            return;
        }
        is_swiping.set(false);

        if let Some(touch) = ev.changed_touches().item(0) {
            let end_x = touch.client_x() as f64;
            let end_y = touch.client_y() as f64;

            let diff_x = end_x - touch_start_x.get();
            let diff_y = end_y - touch_start_y.get();

            let abs_diff_x = diff_x.abs();
            let abs_diff_y = diff_y.abs();

            // Determine if horizontal or vertical swipe
            if abs_diff_x > abs_diff_y && enable_horizontal {
                // Horizontal swipe
                if abs_diff_x >= threshold {
                    let direction = if diff_x > 0.0 {
                        SwipeDirection::Right
                    } else {
                        SwipeDirection::Left
                    };
                    on_swipe.run(direction);
                }
            } else if enable_vertical {
                // Vertical swipe
                if abs_diff_y >= threshold {
                    let direction = if diff_y > 0.0 {
                        SwipeDirection::Down
                    } else {
                        SwipeDirection::Up
                    };
                    on_swipe.run(direction);
                }
            }
        }
    };

    view! {
        <div
            class="fx-swipe-container"
            style="touch-action: pan-y;"
            on:touchstart=on_touch_start
            on:touchend=on_touch_end
        >
            {children()}
        </div>
    }
}
