use leptos::*;
use theta_chart::{chart::Draw, coord::*};

#[component]
pub fn SvgChart(cx: Scope, view: CView, children: Children) -> impl IntoView {
    let outer = view.get_outer();
    // let inner = view.get_inner();
    let margin = view.get_margin();
    let translate_chart = format!("translate({},{})", margin, margin);
    view! { cx,
        <svg class="chart bg-secondary-80" width={outer.get_x()} height={outer.get_y()}>
            <g class="chart-view" transform={translate_chart} >
                // TODO: for dev
                // <rect  width={inner.get_x()} height={inner.get_y()} fill="#00000033"></rect>
                {children(cx)}
            </g>
        </svg>

    }
}
