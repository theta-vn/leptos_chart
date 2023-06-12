use theta_chart::{coord::*, series::*};

#[derive(Debug)]
pub struct DataBar(Chart<SLabel, SNumber>, bool);

impl DataBar {
    // pub fn new(data: Vec<f64>, label: Vec<&str>, view: Vec<u64>) -> Self {
    //     let linear = SNumber::new(data);
    //     let category = SLabel::from(label);
    //     let view = CView::from(view);
    //     let chart = Chart::default()
    //         .set_ay(linear.clone())
    //         .set_ax(category.clone())
    //         .set_view(view);
    //     Self(chart)
    // }

    pub fn get_chart(&self) -> Chart<SLabel, SNumber> {
        self.0.clone()
    }

    pub fn set_data(&self, data: Vec<f64>) -> Self {
        let linear = SNumber::new(data);
        let chart = &self.0;
        Self(chart.clone().set_ay(linear), self.1)
    }

    // pub fn set_data_i64(&self, data: Vec<i64>) -> Self {
    //     let linear = SNumber::from(data);
    //     let chart = &self.0;
    //     Self(chart.clone().set_ay(linear))
    // }

    // new(width: u64, height: u64, position_axes: u64, padding: u64, margin: u64)
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

    pub fn set_label(&self, label: Vec<&str>) -> Self {
        let category = SLabel::from(label);
        let chart = &self.0;
        Self(chart.clone().set_ax(category), self.1)
    }

    pub fn get_vertical(&self) -> bool {
        self.1
    }
}

impl Default for DataBar {
    fn default() -> Self {
        Self(Default::default(), true)
    }
}
