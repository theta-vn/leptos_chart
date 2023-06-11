use theta_chart::{coord::*, series::*};

#[derive(Debug)]
pub struct DataPie(Chart<SNumber, SLabel>);

impl DataPie {
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

    pub fn set_data(&self, data: Vec<f64>) -> Self {
        let linear = SNumber::new(data);
        let chart = &self.0;
        Self(chart.clone().set_ax(linear))
    }

    pub fn set_view(&self, view: Vec<u64>) -> Self {
        let view = CView::from(view);
        let chart = &self.0;
        Self(chart.clone().set_view(view))
    }

    pub fn set_label(&self, label: Vec<&str>) -> Self {
        let category = SLabel::from(label);
        let chart = &self.0;
        Self(chart.clone().set_ay(category))
    }
}

impl Default for DataPie {
    fn default() -> Self {
        Self(Default::default())
    }
}
