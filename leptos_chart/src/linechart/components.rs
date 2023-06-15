use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};

use leptos::{component, view, IntoView, Scope};

use theta_chart::chart::{Draw, ScaleNumber, ScaleTime};

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
    let translate_chart = format!(
        "translate({},{}) scale(1,-1)",
        origin.get_x(),
        inner.get_y()
    );

    // For processing x-axis
    let time = chart.get_ax();
    let intervale_x = inner.get_x();
    let axes_x = time.gen_axes();

    // For processing y-axis
    let data_y = chart.get_ay();
    let intervale_y = inner.get_y();
    let axes_y = data_y.gen_axes();

    // For position
    let position_axes = view.get_position_axes();

    view! { cx,

        <SvgChart view={view}>

            <g class="axes" transform={translate_axes}>
                <g class="x-axis">
                    <XAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} intervale=intervale_x axes=axes_x />
                </g>
                <g class="y-axis">
                    <YAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} intervale=intervale_y axes=axes_y />
                </g>
            </g>
            <g class="inner-chart" transform={translate_chart} >
                {
                    let mut line = "M".to_string();
                    let point = data_y.series().into_iter().enumerate().map(|(index, data)|  {
                        let x: f64 = time.scale(time.series()[index]) * intervale_x;
                        let y: f64 = data_y.scale(data) * intervale_y;
                        line.push_str(format!(" {:.0},{:.0} ", x, y).as_str());
                        view! {cx,
                            <circle cx={x} cy={y}  r="2" stroke="black" stroke-width="1" fill="red" />
                        }
                    }).collect::<Vec<_>>();

                    view! {cx,
                        {point}
                        <path d={line} stroke="red" fill="none"/>
                    }



                }
            </g>
        </SvgChart>
    }
}
