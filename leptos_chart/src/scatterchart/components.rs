use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView};
use theta_chart::{color::Color, coord};

/// Component ScatterChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.6"}
/// leptos_chart = {version = "0.3", features = ["ScatterChart"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let chart = Cartesian::new(
///         Series::from(vec![50,60,70,80,90,100,110,120,130,140,150]).set_range(40., 160.),
///         Series::from(vec![7,8,8,9,9,9,10,11,14,14,15]).set_range(6., 16.),
///     )
///     .set_view(820, 620, 3, 100, 100, 20);
///
///     let color = Color::from("#ff0000");
///     view!{
///         // color is option
///         <ScatterChart chart=chart color=color />
///     }
/// }
/// ```
/// ## Set view for ScatterChart
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
pub fn ScatterChart(
    chart: coord::Cartesian,
    #[prop(default = Color::default())] color: Color,
) -> impl IntoView {
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

    view! {
      <SvgChart cview=cview>
        <g class="axes">
          <g class="x-axis" transform=translate_xa>
            <XAxis region=rec_xa axes=axes_x/>
          </g>
          <g class="y-axis" transform=translate_ya>
            <YAxis region=rec_ya axes=axes_y/>
          </g>
        </g>
        <g class="inner-chart" transform=translate_chart>
          // For draw region of chart

          {#[cfg(feature = "debug")]
          {
              let vector = rec_chart.get_vector();
              let path = format!(
                  "M {},{} l {},{} l {},{} l {},{} Z",
                  0,
                  0,
                  vector.get_x(),
                  0,
                  0,
                  vector.get_y(),
                  -vector.get_x(),
                  0,
              );
              view! {
                <circle id="origin" cx="0" cy="0" r="3"></circle>
                <line
                  x1="0"
                  y1="0"
                  x2=vector.get_x()
                  y2=vector.get_y()
                  style="stroke:#00ff0033;stroke-width:2"
                ></line>
                <path id="region" d=path fill="#00ff0033"></path>
              }
          }}

          {
              let vector = rec_chart.get_vector();
              xsticks
                  .into_iter()
                  .enumerate()
                  .map(|(index, data)| {
                      let x: f64 = xseries.scale(data.value) * vector.get_x();
                      let y: f64 = yseries.scale(ysticks[index].value) * vector.get_y();
                      view! { <circle cx=x cy=y r="4" fill=color.to_string_hex()></circle> }
                  })
                  .collect::<Vec<_>>()
          }

        </g>
      </SvgChart>
    }
}
