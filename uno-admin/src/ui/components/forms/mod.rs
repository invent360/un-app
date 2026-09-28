//! Schema-driven form components
//!
//! These components dynamically render form fields based on schema definitions.

mod field_renderer;
mod text_field;
mod rich_text_field;
mod number_field;
mod boolean_field;
mod select_field;
mod list_field;
mod repeater_field;
mod media_field;
mod media_list_field;
mod gcs_media_upload;
mod reference_picker;
mod schema_form;
mod section_editor;
mod task_section_editor;
pub mod home_section_editor;
pub mod faq_question_editor;
pub mod guide_section_editor;

pub use field_renderer::FieldRenderer;
pub use text_field::TextField;
pub use rich_text_field::RichTextField;
pub use number_field::NumberField;
pub use boolean_field::BooleanField;
pub use select_field::{SelectField, MultiSelectField};
pub use list_field::ListField;
pub use repeater_field::RepeaterField;
pub use media_field::MediaField;
pub use media_list_field::MediaListField;
pub use gcs_media_upload::GcsMediaUploadList;
pub use reference_picker::{ReferencePicker, ReferenceListPicker};
pub use schema_form::{SchemaForm, ValidationError, validate_form_data};
pub use section_editor::SectionEditor;
pub use task_section_editor::{TaskSectionEditor, TaskContentEditor};
pub use home_section_editor::{HomeSectionEditor, HomeContentEditor};
pub use faq_question_editor::{FaqQuestionEditor, FaqContentEditor};
pub use guide_section_editor::{GuideSectionEditor, GuideContentEditor};
