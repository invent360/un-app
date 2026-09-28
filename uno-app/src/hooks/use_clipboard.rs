//! Clipboard operations hook

use leptos::prelude::*;

/// Hook for clipboard copy functionality with feedback
pub fn use_clipboard() -> (ReadSignal<bool>, impl Fn(String) + Clone) {
    let (copied, set_copied) = signal(false);

    let copy = move |text: String| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen_futures::spawn_local;

            let set_copied = set_copied.clone();
            spawn_local(async move {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.write_text(&text);
                    if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                        set_copied.set(true);

                        // Reset after 2 seconds
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        set_copied.set(false);
                    }
                }
            });
        }

        #[cfg(not(feature = "hydrate"))]
        {
            let _ = text;
            let _ = set_copied;
        }
    };

    (copied, copy)
}
