//! Chart components for ember-fx.
//!
//! This module provides comprehensive chart visualization components:
//!
//! ## Time-Series Charts
//! - `LineChart`: Time-series line charts
//! - `AreaChart`: Stacked/layered area charts
//! - `SparklineChart`: Compact inline charts
//!
//! ## Category Charts
//! - `BarChart`: Horizontal bar charts
//! - `ColumnChart`: Vertical column charts
//! - `PieChart`: Pie and donut charts
//!
//! ## Statistical Charts
//! - `ScatterChart`: Scatter plots
//! - `BubbleChart`: Bubble charts with size dimension
//! - `BoxPlotChart`: Statistical box plots
//!
//! ## Financial Charts
//! - `CandlestickChart`: OHLC candlestick charts
//! - `RangeAreaChart`: Range area charts
//!
//! ## Advanced Charts
//! - `HeatmapChart`: Matrix heatmaps
//! - `TreemapChart`: Hierarchical treemaps
//! - `RadarChart`: Radar/spider charts
//! - `RadialBarChart`: Circular progress bars
//! - `GaugeChart`: Gauge/speedometer charts
//! - `FunnelChart`: Funnel/pyramid charts
//! - `TimelineChart`: Gantt-style timelines
//! - `SlopeChart`: Slope charts
//! - `PolarAreaChart`: Polar area charts
//! - `MixedChart`: Combined chart types
//!
//! # Example
//!
//! ```ignore
//! use ember_fx_components::chart::{LineChart, DataPoint, LineChartConfig};
//!
//! let data = vec![
//!     DataPoint::new(1716000000000, 45.0),
//!     DataPoint::new(1716003600000, 52.0),
//! ];
//!
//! view! {
//!     <LineChart data=Signal::derive(move || data.clone()) />
//! }
//! ```

mod types;
pub mod utils;
pub mod components;

mod line_chart;
mod area_chart;
mod sparkline;
mod bar_chart;
mod column_chart;
mod pie_chart;
mod scatter_chart;
mod bubble_chart;
mod heatmap_chart;
mod radialbar_chart;
mod gauge_chart;
mod candlestick_chart;
mod timeline_chart;
mod treemap_chart;
mod boxplot_chart;
mod radar_chart;
mod funnel_chart;
mod slope_chart;
mod polararea_chart;
mod rangearea_chart;
mod mixed_chart;

// Re-export types
pub use types::*;

// Re-export components
pub use line_chart::LineChart;
pub use area_chart::AreaChart;
pub use sparkline::SparklineChart;
pub use bar_chart::BarChart;
pub use column_chart::ColumnChart;
pub use pie_chart::PieChart;
pub use scatter_chart::ScatterChart;
pub use bubble_chart::BubbleChart;
pub use heatmap_chart::HeatmapChart;
pub use radialbar_chart::{RadialBarChart, RadialBarData};
pub use gauge_chart::{GaugeChart, GaugeTooltipData};
pub use candlestick_chart::CandlestickChart;
pub use timeline_chart::TimelineChart;
pub use treemap_chart::TreemapChart;
pub use boxplot_chart::{BoxPlotChart, BoxPlotTooltipData};
pub use radar_chart::{RadarChart, RadarTooltipData};
pub use funnel_chart::FunnelChart;
pub use slope_chart::SlopeChart;
pub use polararea_chart::PolarAreaChart;
pub use rangearea_chart::RangeAreaChart;
pub use mixed_chart::MixedChart;

// Re-export sub-components
pub use components::{XAxis, YAxis, ChartGrid, ChartLegend, ChartTooltip};
