//! API request hook for making server function calls with loading states

use leptos::prelude::*;
use std::future::Future;

/// API request state
#[derive(Clone, Debug, PartialEq)]
pub enum ApiState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

impl<T> ApiState<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, ApiState::Loading)
    }

    pub fn is_success(&self) -> bool {
        matches!(self, ApiState::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ApiState::Error(_))
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            ApiState::Success(data) => Some(data),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            ApiState::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

/// Hook for making API requests with loading state management
pub fn use_api<T, E, F, Fut>(
    request_fn: F,
) -> (ReadSignal<ApiState<T>>, impl Fn() + Clone)
where
    T: Clone + Send + Sync + 'static,
    E: std::fmt::Display + 'static,
    F: Fn() -> Fut + Clone + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let (state, set_state) = signal(ApiState::<T>::Idle);

    let execute = move || {
        let request_fn = request_fn.clone();
        set_state.set(ApiState::Loading);

        leptos::task::spawn_local(async move {
            match request_fn().await {
                Ok(data) => set_state.set(ApiState::Success(data)),
                Err(e) => set_state.set(ApiState::Error(e.to_string())),
            }
        });
    };

    (state, execute)
}

/// Hook for making API requests that execute immediately
pub fn use_api_auto<T, E, F, Fut>(
    request_fn: F,
) -> ReadSignal<ApiState<T>>
where
    T: Clone + Send + Sync + 'static,
    E: std::fmt::Display + 'static,
    F: Fn() -> Fut + Clone + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let (state, execute) = use_api(request_fn);

    // Execute immediately
    Effect::new(move |_| {
        execute();
    });

    state
}
