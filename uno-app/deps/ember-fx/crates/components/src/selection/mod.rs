//! Selection components module.
//!
//! Provides selection components like Checkbox, Radio, Switch, Select,
//! TreeSelect, Cascader, and Mentions.
//!
//! ## Components
//!
//! - [`Checkbox`] / [`CheckboxGroup`] - Boolean selection
//! - [`Radio`] / [`RadioGroup`] - Single selection from options
//! - [`Switch`] - Toggle switch
//! - [`Select`] - Single selection dropdown
//! - [`MultiSelect`] - Multiple selection dropdown
//! - [`TreeSelect`] - Hierarchical tree dropdown
//! - [`Cascader`] - Multi-level cascading dropdown
//! - [`Mentions`] - @mention textarea input
//!
//! ## Types
//!
//! - [`SelectionSize`] - Component sizes (Sm, Md, Lg)
//! - [`SelectOption`] - Option item for Select
//! - [`TreeSelectNode`] - Node for TreeSelect
//! - [`CascaderOption`] - Option for Cascader
//! - [`CascaderTrigger`] - Cascader expansion trigger
//! - [`MentionTrigger`] - Mention trigger configuration
//! - [`MentionOption`] - Mention suggestion option

mod types;
mod checkbox;
mod radio;
mod switch;
mod select;
mod multi_select;
mod tree_select;
mod cascader;
mod mentions;
mod tria_state;

pub use types::*;
pub use checkbox::{Checkbox, CheckboxGroup};
pub use radio::{Radio, RadioGroup};
pub use switch::Switch;
pub use select::Select;
pub use multi_select::MultiSelect;
pub use tree_select::TreeSelect;
pub use cascader::Cascader;
pub use mentions::Mentions;
pub use tria_state::{TriaState, TriaStateOption};
