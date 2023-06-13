use crate::{axes::YAxis, SvgChart, XAxis};

use leptos::{component, view, IntoView, Scope};

use theta_chart::{
    chart::{Draw, ScaleTime, ScaleType, ScaleNumber},
    color::Color,  
};

#[allow(non_snake_case)]
#[component]
pub fn LineChart(cx: Scope, data: crate::DataLine) -> impl IntoView {    
    let chart = data.get_chart();
    log::debug!("CHART:::{:#?}", chart);
    let view = chart.get_view();
    let origin = view.get_origin();
    let inner = view.get_inner();
    let translate_axes = format!("translate({},{})", origin.get_x(), origin.get_y());
    // Flip SVG vertically using an unusual viewbox
    let mut translate_chart = format!(
        "translate({},{}) scale(1,-1)",
        origin.get_x(),
        inner.get_y()
    );


    // For processing x-axis
    let time = chart.get_ax();
    let (distance_up_x, step_x, distance_down_x) = time.count_distance_step();    
    let interval_x = time.get_interval(inner.get_x());  
    let mut vec_string_x: Vec<String> = vec![];
    for index in 0..(distance_up_x + distance_down_x + 1) {
        vec_string_x.push(format!("{}", time.series()[index].format(time.get_format())));
    }

    // For processing y-axis
    let data_y = chart.get_ay();
    let interval_y = data_y.get_interval(inner.get_y());
    let (mut vec_string_y, step) = data_y.gen_sticks_label_step();    
    vec_string_y.reverse();
    

    // For position
    let position_axes = view.get_position_axes();

   

    view! { cx,

        <SvgChart view={view}>

            <g class="axes" transform={translate_axes}>
                <g class="x-axis">
                    <XAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} interval=interval_x sticks=vec_string_x scale_type=time.scale_type() />
                </g>
                <g class="y-axis">
                    <YAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} interval=interval_y sticks=vec_string_y scale_type=data_y.scale_type() />
                </g>
            </g>
            <g class="inner-chart" transform={translate_chart} >
                // {
                //     data.series().into_iter().enumerate().map(|(index, data)|  {
                //         view! {cx,
                //             <rect x={interval_label * index as f64 +  interval_label * 0.05} width={interval_label*0.9} height={data * interval_snumber / step} fill={Color::default().to_string_hex()} />
                //         }
                //     })
                //     .collect::<Vec<_>>()
                // }
            </g>
        </SvgChart>
    }
}
