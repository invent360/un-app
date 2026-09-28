//! Overlay components.
//!
//! This module provides overlay components that display on top of the page content:
//!
//! - [`Modal`] - Standard modal dialog
//! - [`ConfirmModal`] - Confirmation dialogs with type-specific icons
//!
//! # Example
//!
//! ```ignore
//! use ember_fx::components::overlay::{Modal, ConfirmModal, ConfirmType};
//!
//! // Basic modal
//! let modal_open = RwSignal::new(false);
//! view! {
//!     <Modal
//!         open=Signal::derive(move || modal_open.get())
//!         title="My Modal"
//!         on_close=Callback::new(move |_| modal_open.set(false))
//!     >
//!         <p>"Modal content here"</p>
//!     </Modal>
//! }
//!
//! // Confirm dialog
//! let confirm_open = RwSignal::new(false);
//! view! {
//!     <ConfirmModal
//!         open=Signal::derive(move || confirm_open.get())
//!         confirm_type=ConfirmType::Success
//!         title="Operation Successful"
//!         content="Your changes have been saved."
//!         on_close=Callback::new(move |_| confirm_open.set(false))
//!     />
//! }
//! ```

mod types;
mod icons;
mod modal;
mod confirm;

// Re-export types
pub use types::{
    ConfirmType,
    ModalSize,
    ModalAnimation,
    ConfirmButtonConfig,
    ConfirmOptions,
};

// Re-export icons
pub use icons::{
    InfoCircleFilled,
    CheckCircleFilled,
    ExclamationCircleFilled,
    CloseCircleFilled,
    ConfirmIcon,
    CloseOutlined,
};

// Re-export components
pub use modal::Modal;
pub use confirm::ConfirmModal;
