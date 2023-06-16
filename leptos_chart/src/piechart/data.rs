use theta_chart::{coord::*, series::*};

/// Support for creating and store data for Piechart
///
/// # Examples
///
/// ```ignore
/// let data = DataPie::default()        
///     .set_view(800, 600, 0b0010, 200, 10)
///     .set_data(vec![350.0, 200.0, 175.0])
///     .set_label(vec!["Apples", "Bananas", "Cherries"]);
/// ```
///
/// ## Set view for BarChart
/// ```ignore
///     ...
///     .set_view(800, 600, 0b0010, 200, 10)
///     ...
/// ```
/// ## Arguments
/// - `width` : The width of SGV
/// - `height` : The height of SGV
/// - `position_label` : Positions for label
/// - `padding` : Distance for label
/// - `margin` : Margin for actual chart
/// 
/// ## About position_label
/// |0b   |0   |0    |1    |0    |
/// |-----|----|-----|-----|-----|
/// |     |left|bottom|right|top|
/// 
/// =>  Label at right
#[derive(Debug)]
pub struct DataPie(Chart<SNumber, SLabel>);

impl DataPie {
    // pub fn new(data: Vec<f64>, label: Vec<&str>, view: Vec<u64>) -> Self {
    //     let linear = SNumber::new(data);
    //     let category = SLabel::from(label);
    //     let view = CView::from(view);
    //     let chart = Chart::default()
    //         .set_ax(linear.clone())
    //         .set_ay(category.clone())
    //         .set_view(view);
    //     Self(chart)
    // }

    /// Set data for PieChart with data is `Vec<f64>`
    pub fn set_data(&self, data: Vec<f64>) -> Self {
        let linear = SNumber::new(data);
        let chart = &self.0;
        Self(chart.clone().set_ax(linear))
    }

    /// Set data for PieChart with data is `Vec<u64>`
    pub fn set_data_u64(&self, data: Vec<u64>) -> Self {
        let linear = SNumber::from(data);
        let chart = &self.0;
        Self(chart.clone().set_ax(linear))
    }

    /// Set view for SVG of PieChart
    ///
    /// # Arguments
    /// - `width` : The width of SGV
    /// - `height` : The height of SGV
    /// - `position_label` : Positions for label
    /// - `padding` : Distance for axes
    /// - `margin` : Margin for actual chart
    pub fn set_view(
        &self,
        width: u64,
        height: u64,
        position_label: usize,
        padding: u64,
        margin: u64,
    ) -> Self {
        let view = CView::new(width, height, position_label, padding, margin);
        let chart = &self.0;
        Self(chart.clone().set_view(view))
    }

    /// Set labels for PieChart
    pub fn set_label(&self, label: Vec<&str>) -> Self {
        let category = SLabel::from(label);
        let chart = &self.0;
        Self(chart.clone().set_ay(category))
    }

    pub(crate) fn get_chart(&self) -> Chart<SNumber, SLabel> {
        self.0.clone()
    }
}

impl Default for DataPie {
    fn default() -> Self {
        Self(Default::default())
    }
}
