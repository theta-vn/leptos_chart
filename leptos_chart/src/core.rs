// #[warn(dead_code)]

mod svg_chart;
pub use self::svg_chart::*;

// Font size for text in SVG
#[cfg(any(doc, feature = "core"))]
pub(crate) const REM: f32 = 16.;

// Line-height for text in SVG
#[cfg(any(doc, feature = "core"))]
pub(crate) const LHEIGHT: f32 = 1.5;
