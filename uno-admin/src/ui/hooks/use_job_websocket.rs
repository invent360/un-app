//! Job updates hook - uses polling for real-time updates

use leptos::prelude::*;

use crate::models::entity::SyncJobEntity;

/// Connection state (for UI indicator)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WsState {
    Connecting,
    Connected,
    Disconnected,
}

/// Polling interval in milliseconds
const POLL_INTERVAL_MS: u32 = 20000; // 20 seconds

/// Hook that polls for job updates
/// Returns (state, manual_refresh_callback)
#[cfg(target_arch = "wasm32")]
pub fn use_job_websocket(
    jobs: RwSignal<Vec<SyncJobEntity>>,
) -> (ReadSignal<WsState>, Callback<()>) {
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen_futures::spawn_local;
    use gloo_timers::callback::Interval;

    let (ws_state, set_ws_state) = signal(WsState::Connected);

    // Store interval handle to prevent multiple intervals
    let interval_handle: Rc<RefCell<Option<Interval>>> = Rc::new(RefCell::new(None));
    let interval_handle_clone = interval_handle.clone();

    // Only set up interval once using Effect::new with empty dependencies
    Effect::new(move |prev: Option<()>| {
        // Only run on first execution (when prev is None)
        if prev.is_some() {
            return;
        }

        // Create the polling interval
        let interval = Interval::new(POLL_INTERVAL_MS, move || {
            spawn_local(async move {
                match crate::handler::list_sync_jobs().await {
                    Ok(result) => {
                        jobs.set(result);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Poll failed: {}", e).into());
                    }
                }
            });
        });

        // Store the interval handle
        *interval_handle_clone.borrow_mut() = Some(interval);
    });

    // Manual refresh callback
    let refresh = Callback::new(move |_: ()| {
        set_ws_state.set(WsState::Connecting);
        spawn_local(async move {
            match crate::handler::list_sync_jobs().await {
                Ok(result) => {
                    jobs.set(result);
                    set_ws_state.set(WsState::Connected);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Refresh failed: {}", e).into());
                    set_ws_state.set(WsState::Disconnected);
                }
            }
        });
    });

    (ws_state, refresh)
}

/// Non-WASM stub
#[cfg(not(target_arch = "wasm32"))]
pub fn use_job_websocket(
    _jobs: RwSignal<Vec<SyncJobEntity>>,
) -> (ReadSignal<WsState>, Callback<()>) {
    let (ws_state, _) = signal(WsState::Disconnected);
    let refresh = Callback::new(move |_: ()| {});
    (ws_state, refresh)
}
