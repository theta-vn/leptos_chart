use crate::core::SvgPolar;
use leptos::{component, view, IntoView};
use theta_chart::{chart::ScaleNumber, color::Color, coord};

/// Component RadarChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.6"}
/// leptos_chart = {version = "0.3", features = ["RadarChart"]}
/// ```
///
/// ## Component
/// ```ignore
/// use leptos::*;
/// use leptos_chart::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///
///     let chart = Polar::new(
///         Series::from(vec![85.0, 55.0, 45., 60., 40.]),
///         Series::from(vec!["Reading", "Writing", "Listening", "Speaking", "React"]),
///     )
///     .set_view(740, 540, 2, 0, 20);
///     let color = Color::from("#ff0000");
///     view! {
///         <div class="mx-auto p-8">
///             // color is option
///             <RadarChart chart=chart color=color />
///         </div>
///     }
/// }
/// ```
/// ## Set view for RadarChart
/// ```ignore
///     ...
///     .set_view(740, 540, 2, 0, 20);
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
///
/// - Top: 0
/// - Right: 1
/// - Bottom: 2
/// - Left: 3
///
#[allow(non_snake_case)]
#[component]
pub fn RadarChart(
    chart: coord::Polar,
    #[prop(default = Color::default())] color: Color,
) -> impl IntoView {
    let pview = chart.get_view();

    // For processing SNumber
    let data = chart.get_data();
    let grid_radar = data.gen_radar_grid(data.series().len());
    let percents = data.to_percent_radar();

    // For processing SLabel
    let slabel = chart.get_label();

    // For Chart
    let circle_chart = pview.get_circle_chart();
    let translate_chart = format!(
        "translate({},{})",
        circle_chart.get_origin().get_x(),
        circle_chart.get_origin().get_y()
    );

    view! {
      <SvgPolar pview=pview>
        <g class="inner-chart" transform=translate_chart>

          {#[cfg(all(feature = "debug"))]
          {
              let radius = circle_chart.get_radius();
              view! { <circle id="circle" cx=0 cy=0 r=radius fill="#00ff0033"></circle> }
          }}

          // For grid

          {
              let grid_clone = grid_radar.clone();
              let radius = circle_chart.get_radius();
              grid_clone
                  .into_iter()
                  .enumerate()
                  .map(|(index, vector)| {
                      let point = vector.to_point();
                      let radius = radius * 1.1;
                      view! {
                        <line
                          x1="0"
                          y1="0"
                          x2=point.get_x() * radius
                          y2=point.get_y() * radius
                          style="stroke:#00000011;stroke-width:1"
                        ></line>
                        <text
                          x=point.get_x() * radius
                          y=point.get_y() * radius
                          dominant-baseline="middle"
                          text-anchor="middle"
                          opacity=0.5
                        >
                          {slabel.labels()[index].clone()}
                        </text>
                      }
                  })
                  .collect::<Vec<_>>()
          }

          {
              let mut line = "M".to_string();
              let radius = circle_chart.get_radius();
              let mut grid100 = "M".to_string();
              let mut grid050 = "M".to_string();
              for index in 0..percents.len() {
                  let vector = &grid_radar[index];
                  let point = vector.to_point();
                  let percent = percents[index];
                  grid100
                      .push_str(
                          format!(" {:.0},{:.0} ", point.get_x() * radius, point.get_y() * radius)
                              .as_str(),
                      );
                  grid050
                      .push_str(
                          format!(
                              " {:.0},{:.0} ",
                              point.get_x() * radius * 0.5,
                              point.get_y() * radius * 0.5,
                          )
                              .as_str(),
                      );
                  line.push_str(
                      format!(
                          " {:.0},{:.0} ",
                          point.get_x() * radius * percent,
                          point.get_y() * radius * percent,
                      )
                          .as_str(),
                  );
              }
              grid100.push_str(" Z");
              grid050.push_str(" Z");
              line.push_str(" Z");
              view! {
                <text
                  x=0
                  y=radius * -0.5
                  dominant-baseline="middle"
                  text-anchor="middle"
                  opacity=0.3
                >
                  50
                </text>
                <text
                  x=0
                  y=radius * -1.
                  dominant-baseline="middle"
                  text-anchor="middle"
                  opacity=0.3
                >
                  100
                </text>
                <text x=0 y=0 dominant-baseline="middle" text-anchor="middle" opacity=0.3>
                  0
                </text>
                <path d=grid100 stroke="#00000022" fill="none"></path>
                <path d=grid050 stroke="#00000011" fill="none"></path>
                <path
                  d=line
                  stroke=color.to_string_hex()
                  stroke-width="2"
                  fill=format!("{}33", color.to_string_hex())
                ></path>
              }
          }

        </g>
      </SvgPolar>
    }
}
