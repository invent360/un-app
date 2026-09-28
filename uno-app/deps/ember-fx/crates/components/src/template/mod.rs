//! Template system for component structure.
//!
//! Templates control the **HTML structure** of components, separate from
//! their visual styling (themes). This separation allows:
//!
//! - Different layouts for the same component (floating label vs. standard)
//! - Design system-appropriate structure (Material vs. Ant)
//! - Layout-level template selection with component-level overrides
//!
//! ## Architecture
//!
//! ```text
//! Layout (sets default template)
//!     ↓
//! TemplateProvider (context)
//!     ↓
//! Components (use_input_template())
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx::template::{TemplateProvider, InputTemplate};
//!
//! // Layout sets default template
//! view! {
//!     <TemplateProvider input_template=InputTemplate::Material>
//!         <TextInput label="Name" />  // Uses Material structure
//!         <TextInput label="Email" template=InputTemplate::Standard />  // Override
//!     </TemplateProvider>
//! }
//! ```

pub mod types;
pub mod provider;

pub use types::{
    InputTemplate,
    ButtonTemplate,
    CardTemplate,
    TemplateVariant,
};

pub use provider::{
    TemplateProvider,
    TemplateContext,
    use_template,
    try_use_template,
    use_input_template,
    use_button_template,
    use_card_template,
};
