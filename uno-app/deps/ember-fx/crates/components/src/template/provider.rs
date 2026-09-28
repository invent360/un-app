//! Template provider and context hooks.
//!
//! Provides Leptos context for template selection throughout the component tree.

use leptos::prelude::*;
use super::types::{InputTemplate, ButtonTemplate, CardTemplate};

/// Template context holding current template selections.
///
/// This context is provided by `TemplateProvider` and can be accessed
/// via the `use_template()` hook or specific hooks like `use_input_template()`.
#[derive(Clone, Copy, Debug)]
pub struct TemplateContext {
    input: RwSignal<InputTemplate>,
    button: RwSignal<ButtonTemplate>,
    card: RwSignal<CardTemplate>,
}

impl TemplateContext {
    /// Create a new template context with default templates.
    pub fn new() -> Self {
        Self {
            input: RwSignal::new(InputTemplate::default()),
            button: RwSignal::new(ButtonTemplate::default()),
            card: RwSignal::new(CardTemplate::default()),
        }
    }

    /// Create with specific initial templates.
    pub fn with_templates(
        input: InputTemplate,
        button: ButtonTemplate,
        card: CardTemplate,
    ) -> Self {
        Self {
            input: RwSignal::new(input),
            button: RwSignal::new(button),
            card: RwSignal::new(card),
        }
    }

    /// Get the current input template.
    pub fn input_template(&self) -> InputTemplate {
        self.input.get()
    }

    /// Get the input template signal for reactive updates.
    pub fn input_template_signal(&self) -> RwSignal<InputTemplate> {
        self.input
    }

    /// Set the input template.
    pub fn set_input_template(&self, template: InputTemplate) {
        self.input.set(template);
    }

    /// Get the current button template.
    pub fn button_template(&self) -> ButtonTemplate {
        self.button.get()
    }

    /// Get the button template signal for reactive updates.
    pub fn button_template_signal(&self) -> RwSignal<ButtonTemplate> {
        self.button
    }

    /// Set the button template.
    pub fn set_button_template(&self, template: ButtonTemplate) {
        self.button.set(template);
    }

    /// Get the current card template.
    pub fn card_template(&self) -> CardTemplate {
        self.card.get()
    }

    /// Get the card template signal for reactive updates.
    pub fn card_template_signal(&self) -> RwSignal<CardTemplate> {
        self.card
    }

    /// Set the card template.
    pub fn set_card_template(&self, template: CardTemplate) {
        self.card.set(template);
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Template provider component.
///
/// Wraps children with a template context that defines default templates
/// for components. Nested providers can override parent templates.
///
/// # Example
///
/// ```ignore
/// use ember_fx::template::{TemplateProvider, InputTemplate};
///
/// view! {
///     <TemplateProvider input_template=InputTemplate::Material>
///         // All inputs here use Material template by default
///         <TextInput label="Name" />
///
///         // Nested provider overrides for this section
///         <TemplateProvider input_template=InputTemplate::Floating>
///             <TextInput label="Email" />  // Uses Floating
///         </TemplateProvider>
///     </TemplateProvider>
/// }
/// ```
#[component]
pub fn TemplateProvider(
    /// Input template for child components.
    #[prop(optional, into)]
    input_template: Option<InputTemplate>,
    /// Button template for child components.
    #[prop(optional, into)]
    button_template: Option<ButtonTemplate>,
    /// Card template for child components.
    #[prop(optional, into)]
    card_template: Option<CardTemplate>,
    /// Child components.
    children: Children,
) -> impl IntoView {
    // Try to get parent context, or create new one
    let parent = use_context::<TemplateContext>();

    let ctx = if let Some(parent) = parent {
        // Inherit from parent, override with props
        TemplateContext::with_templates(
            input_template.unwrap_or_else(|| parent.input_template()),
            button_template.unwrap_or_else(|| parent.button_template()),
            card_template.unwrap_or_else(|| parent.card_template()),
        )
    } else {
        // No parent, use props or defaults
        TemplateContext::with_templates(
            input_template.unwrap_or_default(),
            button_template.unwrap_or_default(),
            card_template.unwrap_or_default(),
        )
    };

    provide_context(ctx);

    children()
}

/// Get the template context.
///
/// # Panics
///
/// Panics if no `TemplateProvider` is in the component tree.
pub fn use_template() -> TemplateContext {
    use_context::<TemplateContext>().expect(
        "TemplateContext not found. Wrap your app with <TemplateProvider>."
    )
}

/// Try to get the template context, returning None if not available.
pub fn try_use_template() -> Option<TemplateContext> {
    use_context::<TemplateContext>()
}

/// Get the current input template.
///
/// Returns the default template if no provider is in the tree.
///
/// # Example
///
/// ```ignore
/// let template = use_input_template();
/// match template.get() {
///     InputTemplate::Material => { /* render material style */ }
///     InputTemplate::Floating => { /* render floating style */ }
///     _ => { /* render standard style */ }
/// }
/// ```
pub fn use_input_template() -> Signal<InputTemplate> {
    if let Some(ctx) = use_context::<TemplateContext>() {
        Signal::derive(move || ctx.input_template())
    } else {
        Signal::derive(|| InputTemplate::default())
    }
}

/// Get the current button template.
///
/// Returns the default template if no provider is in the tree.
pub fn use_button_template() -> Signal<ButtonTemplate> {
    if let Some(ctx) = use_context::<TemplateContext>() {
        Signal::derive(move || ctx.button_template())
    } else {
        Signal::derive(|| ButtonTemplate::default())
    }
}

/// Get the current card template.
///
/// Returns the default template if no provider is in the tree.
pub fn use_card_template() -> Signal<CardTemplate> {
    if let Some(ctx) = use_context::<TemplateContext>() {
        Signal::derive(move || ctx.card_template())
    } else {
        Signal::derive(|| CardTemplate::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_context_default() {
        let ctx = TemplateContext::new();
        assert_eq!(ctx.input_template(), InputTemplate::Standard);
        assert_eq!(ctx.button_template(), ButtonTemplate::Standard);
        assert_eq!(ctx.card_template(), CardTemplate::Standard);
    }

    #[test]
    fn test_template_context_with_templates() {
        let ctx = TemplateContext::with_templates(
            InputTemplate::Material,
            ButtonTemplate::Icon,
            CardTemplate::Media,
        );
        assert_eq!(ctx.input_template(), InputTemplate::Material);
        assert_eq!(ctx.button_template(), ButtonTemplate::Icon);
        assert_eq!(ctx.card_template(), CardTemplate::Media);
    }
}
