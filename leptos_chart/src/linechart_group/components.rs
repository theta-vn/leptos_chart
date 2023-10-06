use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView};
use theta_chart::{color::Color, coord, series::Series};

/// Component LineChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.5.1"}
/// leptos_chart = {version = "0.1.0", features = ["LineChartGroup"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let chart = CartesianGroup::new()
///         .set_view(840, 640, 3, 50, 50, 20)
///         .add_data(
///             Series::from((vec!["1982", "1986", "2010", "2020", ], "%Y", "year")),
///             Series::from(vec![3., 2.0, 1., 4.]),
///         )
///         .add_data(
///             Series::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year")),
///             Series::from(vec![0., 1.0, 2., 3.]),
///         );
///
///     view!{
///         <LineChartGroup chart=chart />
///     }
/// }
/// ```
/// ## Set view for LineChart
/// ```ignore
///     ...
///     .set_view(820, 620, 3, 100, 100, 20);
///     ...
/// ```
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
pub fn LineChartGroup(chart: coord::CartesianGroup) -> impl IntoView {
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
    let series_x_group = chart.get_ax_group();
    let axes_x = series_x_group.gen_axes();

    // For y-axis
    let rec_ya = cview.get_rec_y_axis();
    let translate_ya = format!(
        "translate({},{})",
        rec_ya.get_origin().get_x(),
        rec_ya.get_origin().get_y()
    );
    let series_y_group = chart.get_ay_group();

    let axes_y = series_y_group.gen_axes();

    // For chart
    let data = chart.get_data();
    let mut xseries: Vec<Series> = vec![];
    let mut yseries: Vec<Series> = vec![];
    for tup in data {
        xseries.push(tup.0);
        yseries.push(tup.1);
    }

    view! {

        <SvgChart cview={cview}>

            <g class="axes">
                <g class="x-axis" transform={translate_xa}>
                    <XAxis region=rec_xa axes=axes_x />
                </g>
                <g class="y-axis" transform={translate_ya}>
                    <YAxis region=rec_ya axes=axes_y />
                </g>
            </g>
            <g class="inner-chart"  transform={translate_chart}>
                // For draw region of chart
               {
                    #[cfg(feature = "debug")]
                    {
                        let vector = rec_chart.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {
                            <circle id="originY" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#00ff0033;stroke-width:2" />
                            <path id="regionY" d=path  fill="#00ff0033" />
                        }
                    }
                }
                {
                    let vector = rec_chart.get_vector();

                    xseries.into_iter().enumerate().map(|(index, datax)|  {
                        let mut color = Color::default();
                        if index !=0 {
                            color = color.shift_hue();
                        }

                        let xsticks = datax.to_stick();
                        let ysticks = yseries[index].to_stick();
                        let mut line = "M".to_string();
                        let point =  xsticks.iter().enumerate().map(|(i,d)|{
                            let x: f64 = series_x_group.scale(d.value) * vector.get_x();
                            let y: f64 = series_y_group.scale(ysticks[i].value) *vector.get_y();
                            line.push_str(format!(" {:.0},{:.0} ", x, y).as_str());
                            view! {
                                <circle cx={x} cy={y}  r="3"  fill=color.to_string_hex() />
                            }
                        }).collect::<Vec<_>>();

                        view! {
                            {point}
                            <path d={line} stroke={color.to_string_hex()} fill="none" stroke-width=2 />
                        }
                    }).collect::<Vec<_>>()

                }
            </g>
        </SvgChart>
    }
}
