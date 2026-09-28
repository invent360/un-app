//! Shared chart sub-components.
//!
//! This module provides reusable components for building charts:
//! - `XAxis` / `YAxis` - Axis rendering with ticks and labels
//! - `ChartGrid` - Background grid lines
//! - `ChartLegend` - Interactive legend
//! - `ChartTooltip` - Hover tooltip

mod axis;
mod grid;
mod legend;
mod tooltip;

pub use axis::{XAxis, YAxis, AxisProps};
pub use grid::{ChartGrid, GridProps};
pub use legend::{ChartLegend, LegendProps};
pub use tooltip::{ChartTooltip, TooltipProps};
