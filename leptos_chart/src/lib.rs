//! # A visualization library for leptos.
//!
//! The project provides chart types to draw for leptos.
//!
//! - [x] PieChart
//! - [x] BarChart
//! - [x] LineChart
//! - [ ] AreaChart
//! - [ ] Scatter Chart
//!
//! ## Examples and Usage
//! - [`PieChart`]
//! - [`BarChart`]
//! - [`LineChart`]
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

#[cfg(any(doc, feature = "LineChart"))]
mod linechart;
#[cfg(any(doc, feature = "LineChart"))]
pub use self::linechart::*;
