//! Button component - re-exports from ember-fx
//!
//! This module provides Button components from ember-fx with
//! full feature support (loading states, variants, sizes, accessibility).

#[cfg(any(feature = "csr", feature = "hydrate", feature = "ssr"))]
pub use ember_fx_components::button::{
    Button,
    ButtonVariant,
    ButtonSize,
    ButtonShape,
    IconButton,
    ButtonGroup,
    FloatButton,
    SpeedDial,
    SplitButton,
};

// Re-export for backward compatibility: when ember-fx is not available, provide stubs
#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
mod fallback {
    use leptos::prelude::*;
    use leptos::ev;

    /// Button variant for fallback
    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum ButtonVariant {
        #[default]
        Primary,
        Secondary,
        Ghost,
        Outline,
        Danger,
        Link,
    }

    impl ButtonVariant {
        pub fn class(&self) -> &'static str {
            match self {
                ButtonVariant::Primary => "btn-primary",
                ButtonVariant::Secondary => "btn-secondary",
                ButtonVariant::Ghost => "btn-ghost",
                ButtonVariant::Outline => "btn-outline",
                ButtonVariant::Danger => "btn-danger",
                ButtonVariant::Link => "btn-link",
            }
        }
    }

    /// Button size for fallback
    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum ButtonSize {
        Xs,
        Sm,
        #[default]
        Md,
        Lg,
        Xl,
    }

    /// Button shape for fallback
    #[derive(Clone, Copy, Default, PartialEq)]
    pub enum ButtonShape {
        #[default]
        Default,
        Round,
        Circle,
    }

    /// Fallback Button component
    #[component]
    pub fn Button(
        #[prop(optional)] variant: ButtonVariant,
        #[prop(optional)] shape: ButtonShape,
        #[prop(optional)] disabled: bool,
        #[prop(optional, into)] class: Option<String>,
        #[prop(optional, into)] icon: Option<String>,
        #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
        children: Children,
    ) -> impl IntoView {
        let shape_class = match shape {
            ButtonShape::Round => "btn-round",
            ButtonShape::Circle => "btn-circle",
            ButtonShape::Default => "",
        };
        let class_name = format!("btn {} {} {}", variant.class(), shape_class, class.unwrap_or_default());

        let click_handler = move |e: ev::MouseEvent| {
            if let Some(cb) = on_click {
                cb.run(e);
            }
        };

        view! {
            <button class=class_name disabled=disabled on:click=click_handler>
                {icon.map(|i| view! { <span class="btn-icon" inner_html=i></span> })}
                {children()}
            </button>
        }
    }
}

#[cfg(not(any(feature = "csr", feature = "hydrate", feature = "ssr")))]
pub use fallback::*;
