use leptos::*;
use theta_chart::{coord::*};

// Wrap chart in SVG
#[cfg(any(doc, feature = "core"))]
#[component]
pub fn SvgChart(cx: Scope, cview: CView, children: Children) -> impl IntoView {
    // let outer = view.get_outer();
    // // For dev
    // // let inner = view.get_inner();
    let margin = cview.get_margin();
    let translate_chart = format!("translate({},{})", margin, margin);
    let vec_chart = cview.get_vector();
    view! { cx,
        <svg class="chart bg-secondary-80" width={vec_chart.get_x()} height={vec_chart.get_y()}>
            <rect  width={vec_chart.get_x()} height={vec_chart.get_y()} fill="#ffffff"></rect>
            <g class="inner-view" transform={translate_chart} >
                // For dev
                
                {children(cx)}
            </g>
            
        </svg>

    }
}
