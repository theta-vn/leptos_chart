//! # A visualization library for leptos.
//!
//! The project provides chart types to draw for leptos.
//!
//! - [x] PieChart
//! - [x] BarChart
//! - [x] LineChart
//! - [x] RadarChart
//! - [x] ScatterChart
//! - [x] BarChartGroup
//! - [x] LineChartGroup
//!
//! ## Examples and Usage
//! - [`PieChart`]
//! - [`BarChart`]
//! - [`LineChart`]
//! - [`RadarChart`]
//! - [`ScatterChart`]
//! - [`BarChartGroup`]
//! - [`LineChartGroup`]
//!
//! Check out the examples folder for helpful snippets of code, as well as minimal configurations that fit some of the most
//! popular chart types. For more explanation, see the crate documentation.

#![warn(missing_docs)]

#[cfg(any(doc, feature = "core"))]
mod core;
#[cfg(any(doc, feature = "core"))]
pub use self::core::Series;

#[cfg(any(doc, feature = "Axes"))]
mod axes;

#[cfg(any(doc, feature = "PieChart"))]
mod piechart;
#[cfg(any(doc, feature = "PieChart"))]
pub use self::piechart::*;

#[cfg(any(doc, feature = "BarChart"))]
mod barchart;
#[cfg(any(doc, feature = "BarChart"))]
pub use self::barchart::*;

#[cfg(any(doc, feature = "BarChartGroup"))]
mod barchart_group;
#[cfg(any(doc, feature = "BarChartGroup"))]
pub use self::barchart_group::*;

#[cfg(any(doc, feature = "LineChart"))]
mod linechart;
#[cfg(any(doc, feature = "LineChart"))]
pub use self::linechart::*;

#[cfg(any(doc, feature = "LineChartGroup"))]
mod linechart_group;
#[cfg(any(doc, feature = "LineChartGroup"))]
pub use self::linechart_group::*;

#[cfg(any(doc, feature = "ScatterChart"))]
mod scatterchart;
#[cfg(any(doc, feature = "ScatterChart"))]
pub use self::scatterchart::*;

#[cfg(any(doc, feature = "RadarChart"))]
mod radarchart;
#[cfg(any(doc, feature = "RadarChart"))]
pub use self::radarchart::*;
