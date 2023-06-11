#[cfg(feature = "PieChart")]
use crate::core::SvgChart;

use leptos::{component, view, IntoView, Scope};

use theta_chart::chart::{Draw, ScaleLabel, ScaleNumber};

#[allow(non_snake_case)]
#[component]
pub fn PieChart(cx: Scope, data: crate::DataPie) -> impl IntoView {
    let chart = data.get_chart();
    let view = chart.get_view();
    let inner = view.get_inner();
    // Flip SVG vertically using an unusual viewbox
    let translate_chart = format!(
        "translate({},{}) scale(-1,-1)",
        inner.get_x(),
        inner.get_y()
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

    // For processing SLabel
    let slabel = chart.get_ay();

    view! { cx,

        <SvgChart view={view}>

            <g class="labels" transform={translate_label}>

                {
                    slabel.labels().into_iter().enumerate().map(|(index, label)|  {
                        let color = &slabel.colors()[index];
                        let py = index * 26;
                        view! {cx,
                            <text x="20" y={py} dominant-baseline="text-before-edge">{label}</text>
                            <rect x={0} y={py+4} width="1rem" height="1rem" fill={color.to_string_hex()}></rect>
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
