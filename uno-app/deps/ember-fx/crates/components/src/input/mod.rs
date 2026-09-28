//! Input components for ember-fx.
//!
//! This module provides form input components with consistent styling,
//! validation states, and template integration.
//!
//! ## Components
//!
//! - [`TextInput`] - Single-line text input with label, placeholder, validation
//! - [`PasswordInput`] - Password field with show/hide toggle
//! - [`TextArea`] - Multi-line text input with auto-resize
//! - [`InputGroup`] - Wrapper for input + addons (prefix/suffix)
//! - [`InputAddon`] - Text/icon addon for InputGroup
//! - [`InputButton`] - Button addon for InputGroup
//! - [`FormField`] - Wrapper for any input with label, error, helper text
//! - [`InputNumber`] - Numeric input with increment/decrement steppers
//! - [`SearchInput`] - Search input with icon and optional button
//! - [`OtpInput`] - One-time password input with separate digit boxes
//!
//! ## Types
//!
//! - [`InputVariant`] - Visual variants (Outlined, Filled, Borderless)
//! - [`InputSize`] - Size variants (Sm, Md, Lg)
//! - [`ValidationState`] - Validation states (None, Success, Warning, Error)
//! - [`InputType`] - HTML input type attribute values
//! - [`StepperPosition`] - Stepper button position for InputNumber
//! - [`OtpLength`] - OTP digit count (4, 6, or 8)
//! - [`MaskType`] - Predefined mask patterns
//! - [`AutoCompleteOption`] - Option for autocomplete suggestions
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx::components::input::{TextInput, PasswordInput, ValidationState};
//!
//! let email = RwSignal::new(String::new());
//! let password = RwSignal::new(String::new());
//!
//! view! {
//!     <TextInput
//!         label="Email"
//!         placeholder="Enter your email"
//!         value=email
//!         validation=ValidationState::None
//!     />
//!     <PasswordInput
//!         label="Password"
//!         placeholder="Enter your password"
//!         value=password
//!     />
//! }
//! ```
//!
//! ## With InputGroup
//!
//! ```ignore
//! use ember_fx::components::input::{InputGroup, InputAddon, InputButton, TextInput};
//!
//! view! {
//!     <InputGroup>
//!         <InputAddon>"https://"</InputAddon>
//!         <TextInput placeholder="example.com" />
//!         <InputButton on_click=move |_| log::info!("Go!")>
//!             "Go"
//!         </InputButton>
//!     </InputGroup>
//! }
//! ```
//!
//! ## Numeric Input
//!
//! ```ignore
//! use ember_fx::components::input::{InputNumber, StepperPosition};
//!
//! let quantity = RwSignal::new(1.0);
//!
//! view! {
//!     <InputNumber
//!         value=quantity
//!         min=0.0
//!         max=100.0
//!         step=1.0
//!         stepper_position=StepperPosition::Right
//!     />
//! }
//! ```
//!
//! ## Search Input
//!
//! ```ignore
//! use ember_fx::components::input::SearchInput;
//!
//! let query = RwSignal::new(String::new());
//!
//! view! {
//!     <SearchInput
//!         value=query
//!         enter_button=true
//!         on_search=move |q| handle_search(q)
//!     />
//! }
//! ```
//!
//! ## OTP Input
//!
//! ```ignore
//! use ember_fx::components::input::{OtpInput, OtpLength};
//!
//! let otp = RwSignal::new(String::new());
//!
//! view! {
//!     <OtpInput
//!         value=otp
//!         length=OtpLength::Six
//!         on_complete=move |code| verify_otp(code)
//!     />
//! }
//! ```
//!
//! ## Template Integration
//!
//! Input components automatically use the template from context:
//!
//! ```ignore
//! use ember_fx::template::{TemplateProvider, InputTemplate};
//!
//! view! {
//!     <TemplateProvider input_template=InputTemplate::Material>
//!         <TextInput label="Name" />  // Uses Material structure
//!     </TemplateProvider>
//! }
//! ```

pub mod types;
pub mod text_input;
pub mod password_input;
pub mod textarea;
pub mod input_group;
pub mod form_field;
pub mod input_number;
pub mod search_input;
pub mod otp_input;
pub mod auto_complete;
pub mod input_mask;

// Re-export types
pub use types::{
    InputVariant,
    InputSize,
    ValidationState,
    InputType,
    StepperPosition,
    OtpLength,
    MaskType,
    AutoCompleteOption,
};

// Re-export components
pub use text_input::TextInput;
pub use password_input::PasswordInput;
pub use textarea::TextArea;
pub use input_group::{InputGroup, InputAddon, InputButton};
pub use form_field::{FormField, ReactiveFormField};
pub use input_number::InputNumber;
pub use search_input::{SearchInput, SearchButton};
pub use otp_input::OtpInput;
pub use auto_complete::AutoComplete;
pub use input_mask::InputMask;
