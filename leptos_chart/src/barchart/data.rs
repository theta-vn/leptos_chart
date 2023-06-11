use theta_chart::{coord::*, series::*};

pub struct DataPie(Chart<SNumber, SLabel>);

impl DataBar {
    pub fn new(data: Vec<f64>, label: Vec<&str>, view: Vec<u64>) -> Self {
        let linear = SNumber::new(data);
        let category = SLabel::from(label);
        let view = CView::from(view);
        let chart = Chart::default()
            .set_ax(linear.clone())
            .set_ay(category.clone())
            .set_view(view);
        Self(chart)
    }

    pub fn get_chart(&self) -> Chart<SNumber, SLabel> {
        self.0.clone()
    }
}
