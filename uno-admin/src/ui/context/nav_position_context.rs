use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;

/// Navigation position options
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NavPosition {
    Top,
    #[default]
    Left,
    Bottom,
}

impl NavPosition {
    /// Get CSS class for body based on nav position
    pub fn body_class(&self) -> &'static str {
        match self {
            NavPosition::Top => "nav-position-top",
            NavPosition::Left => "nav-position-left",
            NavPosition::Bottom => "nav-position-bottom",
        }
    }

    /// Convert from string for storage
    pub fn from_str(s: &str) -> Self {
        match s {
            "top" => NavPosition::Top,
            "left" => NavPosition::Left,
            "bottom" => NavPosition::Bottom,
            _ => NavPosition::default(),
        }
    }

    /// Convert to string for storage
    pub fn to_str(&self) -> &'static str {
        match self {
            NavPosition::Top => "top",
            NavPosition::Left => "left",
            NavPosition::Bottom => "bottom",
        }
    }
}

/// Context for managing navigation position
#[derive(Clone, Copy)]
pub struct NavPositionContext {
    pub position: RwSignal<NavPosition>,
}

impl NavPositionContext {
    pub fn new() -> Self {
        // Always start with default for SSR/hydration consistency
        let position = RwSignal::new(NavPosition::default());
        Self { position }
    }

    /// Load saved position from localStorage (call after hydration)
    #[cfg(target_arch = "wasm32")]
    pub fn load_from_storage(&self) {
        if let Ok(stored) = gloo_storage::LocalStorage::get::<String>("unity_nav_position") {
            let saved_position = NavPosition::from_str(&stored);
            if saved_position != self.position.get() {
                self.position.set(saved_position);
                self.apply_body_class(saved_position);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn apply_body_class(&self, pos: NavPosition) {
        use web_sys::window;
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    let _ = body.class_list().remove_3(
                        "nav-position-top",
                        "nav-position-left",
                        "nav-position-bottom",
                    );
                    let _ = body.class_list().add_1(pos.body_class());
                }
            }
        }
    }

    pub fn set_position(&self, new_position: NavPosition) {
        self.position.set(new_position);

        // Save to localStorage on WASM
        #[cfg(target_arch = "wasm32")]
        {
            let _ = gloo_storage::LocalStorage::set("unity_nav_position", new_position.to_str());
        }

        // Apply CSS class to document
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        // Remove existing nav position classes
                        let _ = body.class_list().remove_3(
                            "nav-position-top",
                            "nav-position-left",
                            "nav-position-bottom",
                        );
                        // Add new class
                        let _ = body.class_list().add_1(new_position.body_class());
                    }
                }
            }
        }
    }

    pub fn cycle_position(&self) {
        let current = self.position.get();
        let next = match current {
            NavPosition::Top => NavPosition::Left,
            NavPosition::Left => NavPosition::Bottom,
            NavPosition::Bottom => NavPosition::Top,
        };
        self.set_position(next);
    }
}

/// Provider component for NavPositionContext
#[component]
pub fn NavPositionContextProvider(children: Children) -> impl IntoView {
    let context = NavPositionContext::new();
    provide_context(context);

    // Load saved position after hydration (client-side only)
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            context.load_from_storage();
        });
    }

    children()
}

/// Hook to get nav position context
pub fn use_nav_position() -> NavPositionContext {
    expect_context::<NavPositionContext>()
}
