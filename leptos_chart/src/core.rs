mod svg_chart;
pub use self::svg_chart::*;

mod svg_polar;
pub use self::svg_polar::*;

// Font size for text in SVG
#[cfg(any(doc, feature = "core"))]
pub(crate) const REM: f64 = 16.;

#[cfg(any(doc, feature = "core"))]
pub use theta_chart::series::Series;

#[cfg(any(doc, feature = "core"))]
pub use theta_chart::color::Color;