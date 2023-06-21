// #[warn(dead_code)]

mod svg_chart;
pub use self::svg_chart::*;

mod svg_polar;
pub use self::svg_polar::*;

// Font size for text in SVG
#[cfg(any(doc, feature = "core"))]
pub(crate) const REM: f32 = 16.;

// Line-height for text in SVG
#[cfg(any(doc, feature = "core"))]
pub(crate) const LHEIGHT: f32 = 1.5;

#[cfg(any(doc, feature = "core"))]
pub use theta_chart::series::Series;
