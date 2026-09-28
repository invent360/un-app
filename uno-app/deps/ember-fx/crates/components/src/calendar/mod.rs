//! Calendar components module.
//!
//! Provides date and time picker components including single pickers,
//! range pickers, and combined date-time pickers.
//!
//! ## Components
//!
//! - [`DatePicker`] - Single date selection
//! - [`TimePicker`] - Single time selection
//! - [`DateRangePicker`] - Date range selection (start/end)
//! - [`TimeRangePicker`] - Time range selection (start/end)
//! - [`DateTimePicker`] - Combined date and time selection
//!
//! ## Types
//!
//! - [`DatePickerMode`] - Date picker modes (Date, Week, Month, Year)
//! - [`DatePickerSize`] - Size variants
//! - [`TimePickerSize`] - Time picker size variants
//! - [`TimeFormat`] - Time format (12h/24h, with/without seconds)
//! - [`DateRangeValue`] - Date range value container
//! - [`TimeRangeValue`] - Time range value container
//! - [`DateTimeValue`] - Combined date-time value container
//! - [`DateRangePreset`] - Quick selection presets
//!
//! ## Example
//!
//! ```ignore
//! use ember_fx::components::calendar::{
//!     DatePicker, TimePicker, DateRangePicker,
//!     DateRangeValue, DateRangePreset,
//! };
//!
//! let date = RwSignal::new(None::<String>);
//! let time = RwSignal::new(None::<String>);
//! let range = RwSignal::new(DateRangeValue::default());
//!
//! view! {
//!     <DatePicker value=date placeholder="Select date" />
//!     <TimePicker value=time use_12_hours=true />
//!     <DateRangePicker
//!         value=range
//!         presets=vec![
//!             DateRangePreset::today("2024-01-15"),
//!             DateRangePreset::last_7_days("2024-01-08", "2024-01-15"),
//!         ]
//!     />
//! }
//! ```

mod types;
mod date_picker;
mod time_picker;
mod date_range_picker;
mod time_range_picker;
mod date_time_picker;

pub use types::*;
pub use date_picker::DatePicker;
pub use time_picker::TimePicker;
pub use date_range_picker::DateRangePicker;
pub use time_range_picker::TimeRangePicker;
pub use date_time_picker::DateTimePicker;
