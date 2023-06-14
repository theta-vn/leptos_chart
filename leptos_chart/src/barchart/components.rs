use crate::{axes::YAxis, SvgChart, XAxis};

use leptos::{component, view, IntoView, Scope};

use theta_chart::{
    chart::{Draw, ScaleLabel, ScaleNumber}, //, ScaleType},
    color::Color,
    // series::SLabel
};

#[allow(non_snake_case)]
#[component]
pub fn BarChart(cx: Scope, data: crate::DataBar) -> impl IntoView {
    let is_vertical = data.get_vertical();
    let chart = data.get_chart();
    // log::debug!("CHART:::{:#?}", chart);
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

    if !is_vertical {
        translate_chart = format!(
            "translate({},{}) rotate(90) scale(1,-1)",
            origin.get_x(),
            origin.get_y()
        );
    }

    // For processing SNumber
    let data = chart.get_ay();
    let mut len_snumber = inner.get_y();
    if !is_vertical {
        len_snumber = inner.get_x()
    };
    let intervale_snumber = data.get_intervale(len_snumber);    
    let axes_number = data.gen_axes();
    

    // For processing SLabel
    let slabel = chart.get_ax();
    let mut len_slabel = inner.get_x();
    if !is_vertical {
        len_slabel = inner.get_y()
    };
    let intervale_label = slabel.get_intervale(len_slabel);
    let axes_label = slabel.gen_axes();

    // For position
    let position_axes = view.get_position_axes();

    let mut intervale_x = intervale_label;
    let mut intervale_y = intervale_snumber;
    let mut axes_x = axes_label.clone();
    let mut axes_y = axes_number.clone();
    // log::debug!("{:#?}", &axes_y);
    axes_y = axes_y.reverse();
    // log::debug!("{:#?}", &axes_y);
    // let mut scale_type_x = slabel.scale_type();
    // let mut scale_type_y = data.scale_type();
    if !is_vertical {
        intervale_y = intervale_label;
        intervale_x = intervale_snumber;
        axes_y = axes_label.clone();
        axes_x = axes_number.clone();
        // scale_type_y = slabel.scale_type();
        
        // scale_type_x = data.scale_type();
    };

    view! { cx,

        <SvgChart view={view}>

            <g class="axes" transform={translate_axes}>
                <g class="x-axis">
                    <XAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} intervale=intervale_x axes=axes_x /> //scale_type=scale_type_x />
                </g>
                <g class="y-axis">
                    <YAxis len_x={inner.get_x()} len_y={inner.get_y()} position={position_axes} intervale=intervale_y axes=axes_y /> //scale_type=scale_type_y />
                </g>
            </g>
            <g class="inner-chart" transform={translate_chart} >
                {
                    data.series().into_iter().enumerate().map(|(index, data)|  {
                        view! {cx,
                            <rect x={intervale_label * index as f64 +  intervale_label * 0.05} width={intervale_label*0.9} height={data * intervale_snumber / axes_number.step} fill={Color::default().to_string_hex()} />
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </g>
        </SvgChart>
    }
}
