//! Panel components for dashboard metrics and collapsible content.
//!
//! This module provides components for displaying key metrics with trends,
//! collapsible accordion panels, and related dashboard elements.
//!
//! ## Components
//!
//! - [`StatCard`] - Card for displaying key metrics with trends and charts
//! - [`Accordion`] - Collapsible content panels for FAQs and organized information
//! - [`Trend`] - Trend indicator with up/down arrows
//! - [`Field`] - Label/value field display
//!
//! ## Usage
//!
//! ```ignore
//! use ember_fx::components::{StatCard, Trend, Field, Accordion, AccordionItem};
//!
//! // StatCard example
//! view! {
//!     <StatCard
//!         title="Total Sales"
//!         value="$126,560"
//!         tooltip="Total sales this month"
//!     >
//!         <Trend flag=TrendFlag::Up value="12%" label="Week over week" />
//!     </StatCard>
//! }
//!
//! // Accordion example
//! let items = vec![
//!     AccordionItem::new("1", "Question 1", "Answer 1"),
//!     AccordionItem::new("2", "Question 2", "Answer 2"),
//! ];
//! view! {
//!     <Accordion items=items allow_multiple=true />
//! }
//! ```

mod types;
mod card;
mod trend;
mod field;
mod accordion;

pub use types::*;
pub use card::{StatCard, StatGroup};
pub use trend::Trend;
pub use field::{Field, InlineField};
pub use accordion::Accordion;
