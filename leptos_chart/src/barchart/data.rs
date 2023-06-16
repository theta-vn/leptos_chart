use theta_chart::{coord::*, series::*};

/// # Support for creating and store data for BarChart
///
/// # Examples
///
/// ```ignore
/// let data = DataBar::default()        
///     .set_view(800, 600, true, 50, 10)
///     .set_data(vec![350.0, 200.0, 175.0])
///     .set_label(vec!["Apples", "Bananas", "Cherries"]);
/// ```
///
/// ## Set view for BarChart
/// ```ignore
///     ...
///     .set_view(800, 600, true, 50, 10)
///     ...
/// ```
/// ## Arguments
/// - `width` : The width of SGV
/// - `height` : The height of SGV
/// - `vertical` : Bar is vertical or horizontal
/// - `padding` : Distance for axes
/// - `margin` : Margin for actual chart
#[derive(Debug)]
pub struct DataBar(Chart<SLabel, SNumber>, bool);

impl DataBar {
    /// Set data for BarChart
    pub fn set_data(&self, data: Vec<f64>) -> Self {
        let linear = SNumber::new(data);
        let chart = &self.0;
        Self(chart.clone().set_ay(linear), self.1)
    }

    /// Set labels for BarChart
    pub fn set_label(&self, label: Vec<&str>) -> Self {
        let category = SLabel::from(label);
        let chart = &self.0;
        Self(chart.clone().set_ax(category), self.1)
    }

    /// Set view for SVG of BarChart
    /// # Arguments
    /// - `width` : The width of SGV
    /// - `height` : The height of SGV
    /// - `vertical` : Bar is vertical or horizontal
    /// - `padding` : Distance for axes
    /// - `margin` : Margin for actual chart
    pub fn set_view(
        &self,
        width: u64,
        height: u64,
        vertical: bool,
        padding: u64,
        margin: u64,
    ) -> Self {
        let mut position_axes = 0b1001;
        if vertical {
            position_axes = 0b1100
        };

        let view = CView::new(width, height, position_axes, padding, margin);
        let chart = &self.0;
        Self(chart.clone().set_view(view), vertical)
    }

    pub(crate) fn get_vertical(&self) -> bool {
        self.1
    }
    pub(crate) fn get_chart(&self) -> Chart<SLabel, SNumber> {
        self.0.clone()
    }
}

impl Default for DataBar {
    fn default() -> Self {
        Self(Default::default(), true)
    }
}
