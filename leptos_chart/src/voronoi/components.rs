use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView, Show};
use theta_chart::{color::Color, coord, delaunator::*};

/// Component ScatterChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.6"}
/// leptos_chart = {version = "0.3", features = ["Voronoi"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///    // fill the unit square with points
///    let mut rng = rand::thread_rng();
///    let vx = (0..100)
///        .map(|_| (96_f64 * rng.gen::<f64>() + 2_f64) )
///        .collect::<Vec<_>>();
///    let vy = (0..100)
///        .map(|_| (96_f64 * rng.gen::<f64>() + 2_f64) )
///        .collect::<Vec<_>>();
///    
///    let chart = Cartesian::new(
///        Series::from(vx).set_range(0., 100.),
///        Series::from(vy).set_range(0., 100.),
///    )
///    .set_view(720, 720, 3, 80, 80, 20);
///
///    let color = Color::from("#ff0000");
///
///    view! {
///      <div class="mx-auto p-8">
///        <h1>"Voronoi diagram example"</h1>
///        <Voronoi chart=chart.clone()/>
///      </div>
///
///      <div class="mx-auto p-8">
///        <h1>"Voronoi diagram with triangle example"</h1>
///        <Voronoi chart=chart delaunay=true color=color/>
///      </div>
///    }
/// }
/// ```
/// ## Set view for Voronoi
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
pub fn Voronoi(
    chart: coord::Cartesian,
    #[prop(default = Color::default())] color: Color,
    #[prop(default = false)] delaunay: bool,
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

    // For delaunay
    let triangle = triangle(xseries.clone(), yseries.clone());

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

        <clipPath id="imageclippath">
          <rect
            x="0"
            y=rec_chart.get_vector().get_y()
            height=rec_chart.get_height()
            width=rec_chart.get_width()
          ></rect>
        </clipPath>

        <g class="inner-chart" transform=translate_chart clip-path="url(#imageclippath)">
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

          // Point

          {
              let vector = rec_chart.get_vector();
              xsticks
                  .clone()
                  .into_iter()
                  .enumerate()
                  .map(|(index, data)| {
                      let x: f64 = xseries.scale(data.value) * vector.get_x();
                      let y: f64 = yseries.scale(ysticks[index].value) * vector.get_y();
                      let mut r = 2;
                      if delaunay {
                          r = 4;
                      }
                      view! { <circle cx=x cy=y r=r fill=color.to_string_hex()></circle> }
                  })
                  .collect::<Vec<_>>()
          }

          // For voronois

          {
              let vector = rec_chart.get_vector();
              triangle
                  .voronois
                  .clone()
                  .into_iter()
                  .enumerate()
                  .map(|(_index, data)| {
                      let mut points = "".to_string();
                      for vectex in data {
                          let px = xseries.scale(triangle.vertices[vectex].get_x())
                              * vector.get_x();
                          let py = yseries.scale(triangle.vertices[vectex].get_y())
                              * vector.get_y();
                          points.push_str(format!("{:.0},{:.0} ", px, py).as_str());
                      }
                      view! {
                        <polygon
                          points=points.clone()
                          class="voronoi"
                          fill="none"
                          stroke=color.shift_hue_degrees_index(120_f32, 2).to_string_hex()
                        ></polygon>
                      }
                  })
                  .collect::<Vec<_>>()
          }

          // For voronoi_edges

          {
              let vector = rec_chart.get_vector();
              triangle
                  .voronoi_edges
                  .clone()
                  .into_iter()
                  .enumerate()
                  .map(|(_index, data)| {
                      let x1 = xseries.scale(data.get_origin().get_x()) * vector.get_x();
                      let y1 = yseries.scale(data.get_origin().get_y()) * vector.get_y();
                      let vector_translate = data.get_vector().multiply(1000.);
                      let end = data.get_origin().translate(&vector_translate);
                      let x2 = xseries.scale(end.get_x()) * vector.get_x();
                      let y2 = yseries.scale(end.get_y()) * vector.get_y();
                      view! {
                        <line
                          x1=x1
                          y1=y1
                          x2=x2
                          y2=y2
                          stroke=color.shift_hue_degrees_index(120_f32, 2).to_string_hex()
                        ></line>
                      }
                  })
                  .collect::<Vec<_>>()
          }

          // For triangle
          <Show when=move || { delaunay } fallback=|| view! {}>

            {
                let vector = rec_chart.get_vector();
                triangle
                    .tuple_triangles
                    .iter()
                    .map(|data| {
                        let mut points = "".to_string();
                        let px1 = xseries.scale(xsticks[data[0]].value) * vector.get_x();
                        let py1 = yseries.scale(ysticks[data[0]].value) * vector.get_y();
                        let px2 = xseries.scale(xsticks[data[1]].value) * vector.get_x();
                        let py2 = yseries.scale(ysticks[data[1]].value) * vector.get_y();
                        let px3 = xseries.scale(xsticks[data[2]].value) * vector.get_x();
                        let py3 = yseries.scale(ysticks[data[2]].value) * vector.get_y();
                        points
                            .push_str(
                                format!(
                                    "{:.0},{:.0} {:.0},{:.0} {:.0},{:.0}",
                                    px1,
                                    py1,
                                    px2,
                                    py2,
                                    px3,
                                    py3,
                                )
                                    .as_str(),
                            );
                        view! {
                          <polygon
                            points=points.clone()
                            class="triangle"
                            fill="none"
                            stroke=color.shift_hue_degrees_index(120_f32, 1).to_string_hex()
                          ></polygon>
                        }
                    })
                    .collect::<Vec<_>>()
            }

          </Show>
        </g>
      </SvgChart>
    }
}
