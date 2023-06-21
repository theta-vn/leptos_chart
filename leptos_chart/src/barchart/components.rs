use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView, Scope};
use theta_chart::coord;

/// Component BarChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.3.0"}
/// leptos_chart = {version = "0.0.2", features = ["BarChart"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App(cx: Scope) -> impl IntoView {
///     let chart_v = Cartesian::new(
///         Series::from(vec!["A", "B", "C"]),
///         Series::from(vec![1.0, 6.0, 9.]),
///     )
///     .set_view(820, 620, 3, 50, 50, 20);
///
///     view!{ cx,
///         <BarChart data=data />
///     }
/// }
/// ```
///
/// ## Set view for BarChart
///
/// ```ignore
///     ...
///     .set_view(820, 620, 3, 100, 100, 20);
///     ...
/// ```
///
/// ## Arguments
/// - `width` : The width of SGV
/// - `height` : The height of SGV
/// - `position_origin` : Positions for origin of chart xOy
/// - `height_x_axis` : Height x_axis
/// - `width_y_axis` : Width y_axis
/// - `margin` : Margin for actual chart
///
/// ## About position_axes
///
/// - Top Left: 0
/// - Top Right: 1
/// - Bottom Right: 2
/// - Bottom Left: 3
///
#[allow(non_snake_case)]
#[component]
pub fn BarChart(cx: Scope, chart: coord::Cartesian) -> impl IntoView {
    let cview = chart.get_view();

    // For Chart
    let rec_chart = cview.get_rec_chart();
    let translate_chart = format!(
        "translate({},{})",
        rec_chart.get_origin().get_x(),
        rec_chart.get_origin().get_y()
    );

    // For x-axis
    let rec_xa = cview.get_rec_x_axis();
    let translate_xa = format!(
        "translate({},{})",
        rec_xa.get_origin().get_x(),
        rec_xa.get_origin().get_y()
    );
    let series_x = chart.get_ax();
    let axes_x = series_x.gen_axes();
    // log::debug!("{:#?}", axes_x);

    // For y-axis
    let rec_ya = cview.get_rec_y_axis();
    let translate_ya = format!(
        "translate({},{})",
        rec_ya.get_origin().get_x(),
        rec_ya.get_origin().get_y()
    );
    let series_y = chart.get_ay();
    let axes_y = series_y.gen_axes();

    // For chart
    let xseries = chart.get_ax();
    let yseries = chart.get_ay();
    let xsticks = xseries.to_stick();
    let ysticks = yseries.to_stick();

    let mut x_is_label = true;
    match xseries {
        Series::Label(_) => (),
        _ => x_is_label = false,
    }

    view! { cx,

        <SvgChart cview={cview}>
            <g class="axes">
                <g class="x-axis" transform={translate_xa}>
                    // For draw region of x-axis
                    {
                        // let origin = rec_xa.get_origin();
                        let vector = rec_xa.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {cx,
                            <circle id="originX" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#ff000033;stroke-width:1" />
                            <path id="regionX" d=path  fill="#ff000033" />
                        }
                    }

                    <XAxis region=rec_xa axes=axes_x />
                </g>
                <g class="y-axis" transform={translate_ya}>
                    // For draw region of y-axis
                    {
                        // let origin = rec_ya.get_origin();
                        let vector = rec_ya.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {cx,
                            <circle id="originY" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#0000ff33;stroke-width:2" />
                            <path id="regionY" d=path  fill="#0000ff33" />
                        }
                    }
                    <YAxis region=rec_ya axes=axes_y />
                </g>
            </g>
            <g class="inner-chart"  transform={translate_chart}>
                // For draw region of chart
                {
                    // let origin = rec_ya.get_origin();
                    let vector = rec_chart.get_vector();
                    let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                    view! {cx,
                        <circle id="originY" cx="0" cy="0" r="3" />
                        <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#00ff0033;stroke-width:2" />
                        <path id="regionY" d=path  fill="#00ff0033" />
                    }
                }
                // {
                //     let vector = rec_chart.get_vector();
                //     let mut line = "M".to_string();
                //     let point = xsticks.clone().into_iter().enumerate().map(|(index, data)|  {
                //         let x: f64 = xseries.scale(data.value) * vector.get_x();
                //         let y: f64 = yseries.scale(ysticks[index].value) *vector.get_y();
                //         line.push_str(format!(" {:.0},{:.0} ", x, y).as_str());
                //         view! {cx,
                //             <circle cx={x} cy={y}  r="2" stroke="black" stroke-width="1" fill="red" />
                //         }
                //     }).collect::<Vec<_>>();

                //     view! {cx,
                //         {point}
                //         <path d={line} stroke="red" fill="none"/>
                //     }
                // }

                {
                    let vector = rec_chart.get_vector();
                    let color = Color::default();
                    if x_is_label {
                        let width_col = xseries.scale(0.9) * vector.get_x();
                        let style = format!("stroke:{};stroke-width:{}", color.to_string_hex() ,width_col.abs() as u64);
                        xsticks.into_iter().enumerate().map(|(index, data)|  {
                            let x: f64 = xseries.scale(data.value + 0.5) * vector.get_x();
                            let y: f64 = yseries.scale(ysticks[index].value) *vector.get_y();

                            view! {cx,
                                <line x1=x y1="0" x2=x y2=y style=style.clone() />
                            }
                        })
                        .collect::<Vec<_>>()
                    } else {
                        let width_col = yseries.scale(0.9) * vector.get_y();
                        let style = format!("stroke:{};stroke-width:{}", color.to_string_hex() ,width_col.abs() as u64);
                        xsticks.into_iter().enumerate().map(|(index, data)|  {
                            let x: f64 = xseries.scale(data.value) * vector.get_x();
                            let y: f64 = yseries.scale(ysticks[index].value +0.5) * vector.get_y();
                            view! {cx,
                                <line x1="0" y1=y x2=x y2=y style=style.clone() />
                            }
                        })
                        .collect::<Vec<_>>()
                    }

                }
            </g>
        </SvgChart>
    }
}
