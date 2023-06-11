use crate::core::SvgChart;

use leptos::{component, view, IntoView, Scope};

use theta_chart::chart::{Draw, ScaleLabel, ScaleNumber};
const REM: f32 = 16.;
const LHEIGHT: f32 = 1.5;

#[allow(non_snake_case)]
#[component]
pub fn PieChart(cx: Scope, data: crate::DataPie) -> impl IntoView {
    let chart = data.get_chart();
    log::debug!("{:#?}", chart);
    let view = chart.get_view();
    let origin = view.get_origin();
    let inner = view.get_inner();
    // Flip SVG vertically using an unusual viewbox
    let translate_chart = format!(
        "translate({},{}) scale(-1,-1) ",
        inner.get_x() + origin.get_x() * 2.,
        inner.get_y() + origin.get_y() * 2.
    );

    let label_region = view.get_region_axes_first();

    let translate_label = format!(
        "translate({},{})",
        &label_region.get_origin().get_x() + 5.,
        label_region.get_origin().get_y() + 5.
    );

    // For processing SNumber
    let data = chart.get_ax();
    let center = view.get_center();
    let radius = view.get_radius();
    let vec_arc = data.gen_pie(center, radius - 5.);
    let series = data.series();
    

    // For processing SLabel
    let slabel = chart.get_ay();

    view! { cx,
        <SvgChart view={view}>
            <g class="labels" transform={translate_label}>
                {
                    slabel.labels().into_iter().enumerate().map(|(index, label)|  {
                        let color = &slabel.colors()[index];                        
                        let py = index as f32 * LHEIGHT * REM;
                        view! {cx,
                            <text x={LHEIGHT * REM} y={py} dominant-baseline="text-before-edge">{format!("{}: {}",label, series[index])}</text>
                            <rect x={0} y={py + (LHEIGHT - 1.0) * REM / 2.} width=REM height=REM fill={color.to_string_hex()}></rect>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </g>
            <g class="inner-chart" transform={translate_chart} >
                {
                    vec_arc.into_iter().enumerate().map(|(index, data)|  {
                        let color = &slabel.colors()[index];
                        view! {cx,
                            <path fill={color.to_string_hex()} stroke="#ffffff" stroke-width="1" d={data.gen_path()} />
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </g>
        </SvgChart>
    }
}
