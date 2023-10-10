use d3_delaunay_rs::delaunay::Delaunay;
use geo_types::Coord;
use leptos::*;
use leptos_chart::*;
use theta_chart::color::Color;
use theta_chart::coord::Cartesian;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let x_values = vec![50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150];
    let y_values = vec![7, 8, 8, 9, 9, 9, 10, 11, 14, 14, 15];
    let chart = Cartesian::new(
        Series::from(x_values.clone()).set_range(40., 160.),
        Series::from(y_values.clone()).set_range(6., 16.),
    )
    .set_view(820, 620, 3, 100, 100, 20);

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

    // Convert data series into a form compatible with the d3_delauany crate.
    let vector = rec_chart.get_vector();
    let points = x_values
        .iter()
        .zip(y_values)
        .map(|(x, y)| Coord {
            x: xseries.scale(*x as f64) * vector.get_x(),
            y: yseries.scale(y as f64) * vector.get_y(),
        })
        .collect::<Vec<Coord<f64>>>();

    let delaunay = Delaunay::new(&points);
    let d_delaunay = delaunay.render_to_string();

    view! {
        <div class="mx-auto p-8">
            <h1>"The delaunay triangulation of a set of points"</h1>
            <SvgChart cview >
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
                            <circle id="origin" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#00ff0033;stroke-width:2" />
                            <path id="region" d=path  fill="#00ff0033" />
                        }
                    }
                }

                <path class="delauany" d=d_delaunay stroke="green" fill="none"/>

                {
                    let color = Color::default();
                    let vector = rec_chart.get_vector();
                    xsticks.into_iter().enumerate().map(|(index, data)|  {
                        let x: f64 = xseries.scale(data.value) * vector.get_x();
                        let y: f64 = yseries.scale(ysticks[index].value) *vector.get_y();
                        view! {
                            <circle cx={x} cy={y}  r="4" fill=color.to_string_hex() />
                        }
                    }).collect::<Vec<_>>()
                }
              </g>
            </SvgChart>

        </div>
    }
}
