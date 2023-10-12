use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};
use leptos::{component, view, IntoView};
use theta_chart::{color::Color, coord, delaunator::*};

/// Component ScatterChart for leptos
///
/// # Examples
///
/// ## Cargo.toml
///
/// ```toml
/// [dependencies]
/// leptos = {version = "0.5"}
/// leptos_chart = {version = "0.2", features = ["ScatterChart"]}
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
pub fn Delaunay(
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

    // For delaunay

    let triangle = triangle(xseries.clone(), yseries.clone());
    log::debug!("{:#?}", triangle);
    let mut vec_trianle: Vec<(usize, usize, usize)> = [].to_vec();
    for index in (0..triangle.triangles.len()).step_by(3) {
        let p1_i = triangle.triangles[index];
        let p2_i = triangle.triangles[index + 1];
        let p3_i = triangle.triangles[index + 2];
        vec_trianle.push((p1_i, p2_i, p3_i))
    }

    let mut vec_hafledge: Vec<(usize, usize, usize)> = [].to_vec();
    for index in (0..triangle.halfedges.len()).step_by(3) {
        let e1_i = triangle.halfedges[index];
        let e2_i = triangle.halfedges[index + 1];
        let e3_i = triangle.halfedges[index + 2];
        vec_hafledge.push((e1_i, e2_i, e3_i))
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
                            <circle id="origin" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#00ff0033;stroke-width:2" />
                            <path id="region" d=path  fill="#00ff0033" />
                        }
                    }
                }
                // Point
                {
                    let vector = rec_chart.get_vector();
                    xsticks.clone().into_iter().enumerate().map(|(index, data)|  {
                        let x: f64 = xseries.scale(data.value) * vector.get_x();
                        let y: f64 = yseries.scale(ysticks[index].value) *vector.get_y();
                        view! {
                            <circle cx={x} cy={y}  r="4" fill=color.to_string_hex() />
                        }
                    }).collect::<Vec<_>>()
                }
                // For triangle
                {
                    let vector = rec_chart.get_vector();
                    vec_trianle.into_iter().enumerate().map(|(index, data)|  {
                        let mut points = "".to_string();
                        let px1 = xseries.scale(xsticks[data.0].value) * vector.get_x();
                        let py1 = yseries.scale(ysticks[data.0].value) * vector.get_y();
                        let px2 = xseries.scale(xsticks[data.1].value) * vector.get_x();
                        let py2 = yseries.scale(ysticks[data.1].value) * vector.get_y();
                        let px3 = xseries.scale(xsticks[data.2].value) * vector.get_x();
                        let py3 = yseries.scale(ysticks[data.2].value) * vector.get_y();

                        points.push_str(format!("{:.0},{:.0} {:.0},{:.0} {:.0},{:.0}", px1, py1, px2, py2, px3, py3).as_str());
                        let center = triangle.vertices[index].clone();
                        let cx = xseries.scale(center.get_x()) * vector.get_x();
                        let cy = yseries.scale(center.get_y()) * vector.get_y();
                        let halfedge = vec_hafledge[index];

                        log::debug!("Index: {:#?} Data: {:#?} Halfedge:{:#?}",index, data, halfedge);

                        view! {
                            // <polygon points={points.clone()} class="triangle" fill={format!("{}80", color.shift_hue_degrees_index(36., index).to_string_hex())} stroke={color.to_string_hex()}/>
                            <polygon points={points.clone()} class="triangle" fill="none" stroke={color.to_string_hex()}/>
                            <circle cx=cx cy=cy  r="4" fill={color.shift_hue_degrees_index(180., 1).to_string_hex()} />
                            // <text x=cx y=cy fill={color.shift_hue_degrees_index(36., index).to_string_hex()}>{format!("{}:({}-{}-{})", index, data.0, data.1, data.2)}</text>
                            // <text x=px1 y=py1 >{data.0}</text>
                            // <text x=px2 y=py2 >{data.1}</text>
                            // <text x=px3 y=py3 >{data.2}</text>
                        }
                    }).collect::<Vec<_>>()
                }
                // For voronol
                {
                    let vector = rec_chart.get_vector();
                    triangle.voronols.clone().into_iter().enumerate().map(|(index, data)|  {
                    
                    let mut points = "".to_string();
                    for vectex in data {
                        let px = xseries.scale(triangle.vertices[vectex].get_x()) * vector.get_x();
                        let py = yseries.scale(triangle.vertices[vectex].get_y()) * vector.get_y();
                        points.push_str(format!("{:.0},{:.0} ", px, py).as_str());
                    }                    
                    
                    view! {
                        // <polygon points={points.clone()} class="voronol" fill={format!("{}dd", color.shift_hue_degrees_index(36., index).to_string_hex())} stroke={color.to_string_hex()}/>
                        <polygon points={points.clone()} class="voronol" fill="none" stroke={color.shift_hue_degrees_index(180., 1).to_string_hex()}/>
                    }
                  }).collect::<Vec<_>>()
              }
            </g>
        </SvgChart>
    }
}
