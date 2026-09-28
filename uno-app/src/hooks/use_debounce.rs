//! Debounce hook for performance optimization

use leptos::prelude::*;
use std::time::Duration;

/// Hook that debounces a value, only updating after the specified delay
pub fn use_debounce<T>(
    value: ReadSignal<T>,
    delay_ms: u64,
) -> ReadSignal<T>
where
    T: Clone + Send + Sync + 'static,
{
    let (debounced, set_debounced) = signal(value.get());

    Effect::new(move |_| {
        let current = value.get();

        #[cfg(feature = "hydrate")]
        {
            use gloo_timers::future::TimeoutFuture;
            use leptos::task::spawn_local;

            let current_clone = current.clone();
            spawn_local(async move {
                TimeoutFuture::new(delay_ms as u32).await;
                set_debounced.set(current_clone);
            });
        }

        #[cfg(not(feature = "hydrate"))]
        {
            // On SSR, just set immediately
            set_debounced.set(current);
        }
    });

    debounced
}

/// Hook that throttles a callback, ensuring it's called at most once per interval
pub fn use_throttle<F>(callback: F, delay_ms: u64) -> impl Fn() + Clone
where
    F: Fn() + Clone + 'static,
{
    let last_called = StoredValue::new(std::time::Instant::now());
    let delay = Duration::from_millis(delay_ms);

    move || {
        let now = std::time::Instant::now();
        if now.duration_since(last_called.get_value()) >= delay {
            last_called.set_value(now);
            callback();
        }
    }
}

/// Debounced search input component helper
#[component]
pub fn DebouncedInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(default = 300)] delay_ms: u64,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let (local_value, set_local_value) = signal(value.get());

    // Sync local value with prop
    Effect::new(move |_| {
        set_local_value.set(value.get());
    });

    // Debounce the callback
    Effect::new(move |_| {
        let current = local_value.get();

        #[cfg(feature = "hydrate")]
        {
            use gloo_timers::future::TimeoutFuture;
            use leptos::task::spawn_local;

            spawn_local(async move {
                TimeoutFuture::new(delay_ms as u32).await;
                on_change.run(current);
            });
        }

        #[cfg(not(feature = "hydrate"))]
        {
            on_change.run(current);
        }
    });

    view! {
        <input
            type="text"
            class=class.unwrap_or_default()
            placeholder=placeholder.unwrap_or_default()
            prop:value=move || local_value.get()
            on:input=move |ev| {
                use leptos::wasm_bindgen::JsCast;
                if let Some(input) = ev.target() {
                    if let Some(input) = input.dyn_ref::<leptos::web_sys::HtmlInputElement>() {
                        set_local_value.set(input.value());
                    }
                }
            }
        />
    }
}
