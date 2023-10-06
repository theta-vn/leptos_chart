use crate::core::{SvgPolar, REM};
use leptos::{component, view, IntoView};
use theta_chart::{
    chart::{ScaleLabel, ScaleNumber},
    coord,
};

/// Component PieChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.5.1"}
/// leptos_chart = {version = "0.1.0", features = ["PieChart"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let chart = Polar::new(
///         Series::from(vec![1.0, 2.0, 3.]),
///         Series::from(vec!["A", "B", "C"])
///     )
///     .set_view(740, 540, 2, 200, 20);
///
///     view!{
///         <PieChart chart=chart />
///     }
/// }
/// ```
/// ## Set view for PieChart
/// ```ignore
///     ...
///     .set_view(740, 540, 2, 200, 20);
///     ...
/// ```
/// ## Arguments
/// - `width` : The width of SGV
/// - `height` : The height of SGV
/// - `position_label` : Positions for label
/// - `len_label` : Distance for label
/// - `margin` : Margin for actual chart
///
/// ## About position_label
/// - Top: 0
/// - Right: 1
/// - Bottom: 2
/// - Left: 3
///
#[allow(non_snake_case)]
#[component]
pub fn PieChart(chart: coord::Polar) -> impl IntoView {
    let pview = chart.get_view();

    // For processing SNumber
    let data = chart.get_data();
    let vec_arc = data.gen_pie();
    let series = data.series();

    // For processing SLabel
    let slabel = chart.get_label();

    // For Chart
    let circle_chart = pview.get_circle_chart();
    let translate_chart = format!(
        "translate({},{})",
        circle_chart.get_origin().get_x(),
        circle_chart.get_origin().get_y()
    );

    // For label
    let rec_label = pview.get_rec_label();
    let translate_label = format!(
        "translate({},{})",
        rec_label.get_origin().get_x(),
        rec_label.get_origin().get_y(),
    );

    view! {
        <SvgPolar pview={pview}>

            <g class="labels" transform={translate_label}>
                // For draw region of label
                {
                    #[cfg(feature = "debug")]
                    {
                        let vector = rec_label.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {
                            <circle id="origin" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#005bbe33;stroke-width:1" />
                            <path id="regionX" d=path  fill="#005bbe33" />
                        }
                    }
                }
                {
                    slabel.labels().into_iter().enumerate().map(|(index, label)|  {
                        let color = &slabel.colors()[index];
                        let py = index as f64 * 1.5 * REM;
                        view! {
                            <text x={1.5 * REM} y={py} dominant-baseline="text-before-edge">{format!("{}: {}",label, series[index])}</text>
                            <rect x={0} y={py + (1.5 - 1.0) * REM / 2.} width=REM height=REM fill={color.to_string_hex()}></rect>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </g>
            <g class="inner-chart" transform={translate_chart} >
                {
                    #[cfg(all(feature = "debug"))]
                    {
                        let radius = circle_chart.get_radius();
                        view! {
                            <circle id="origin" cx=0 cy=0 r=3 />
                            <circle id="circle" cx=0 cy=0 r=radius fill="#00ff0033"/>
                            <line x1="0" y1="0" x2=0 y2=-radius style="stroke:#00ff0033;stroke-width:2" />
                        }
                    }
                }

                {
                    vec_arc.into_iter().enumerate().map(|(index, data)|  {
                        let color = &slabel.colors()[index];
                        let radius = circle_chart.get_radius();
                        view! {
                            <path  fill={color.to_string_hex()} stroke="#ffffff" stroke-width="1" d={data.gen_path(radius)} />
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </g>
        </SvgPolar>
    }
}
