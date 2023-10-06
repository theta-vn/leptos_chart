use leptos::*;
use theta_chart::coord::*;

// Wrap chart in SVG
#[cfg(any(doc, feature = "core"))]
#[component]
pub fn SvgChart(cview: CView, children: Children) -> impl IntoView {
    let margin = cview.get_margin();

    let translate_chart = format!("translate({},{})", margin, margin);
    let vec_chart = cview.get_vector();
    let view_box = format!("0 0 {} {}", vec_chart.get_x(), vec_chart.get_y());
    view! {
        <svg class="chart" viewBox=view_box>
            {
                #[cfg(feature = "debug")]
                {
                    view! {
                        <rect width={vec_chart.get_x()} height={vec_chart.get_y()} fill="#005bbe11"></rect>
                    }
                }
            }
            <g class="inner-view" transform={translate_chart} >

                {children()}
            </g>

        </svg>

    }
}
