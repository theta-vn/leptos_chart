use theta_chart::{coord::*, series::*};

// #[derive(Debug)]
// pub struct DataLine(X);
// 
// impl DataLine {
//     pub fn get_chart(&self) -> Chart<STime, SNumber> {
//         self.0.clone()
//     }

//     pub fn set_time(&self, data: (Vec<&str>, &str, &str)) -> Self {
//         let time = STime::from(data);
//         let chart = &self.0;
//         Self(chart.clone().set_ax(time))
//     }

//     pub fn set_data(&self, data: Vec<f64>) -> Self {
//         let linear = SNumber::new(data);
//         let chart = &self.0;
//         Self(chart.clone().set_ay(linear))
//     }

//     pub fn set_view(&self, width: u64, height: u64, padding: u64, margin: u64) -> Self {
//         let position_axes = 0b1100;

//         let view = CView::new(width, height, position_axes, padding, margin);
//         let chart = &self.0;
//         Self(chart.clone().set_view(view))
//     }
// }

// impl Default for DataLine {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }
