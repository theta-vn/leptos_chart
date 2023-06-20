use leptos::{component, view, IntoView, Scope};
use theta_chart::{
    coord::{Axes, Rec}    
};

use crate::core::REM;

#[allow(non_snake_case)]
#[component]
pub fn XAxis(cx: Scope, region: Rec, axes: Axes) -> impl IntoView {
    let vector = region.get_vector();
    let mut mark_origin_y = REM as f64;
    let mut baseline = "text-before-edge";

    if vector.get_y() < 0. {
        mark_origin_y *= -1.;
        baseline = "text-after-edge";
    }

    view! {cx,
        <g class="stick"> // transform={translate}>
            <line x1="0" y1="0" x2=vector.get_x() y2="0" style="stroke:rgb(255,0,0)" />
            <line x1="0" y1="0" x2="0" y2={mark_origin_y} style="stroke:rgb(255,0,0)" />
                {
                    axes.sticks.into_iter().map(|stick| {
                        let dx = stick.value * vector.get_x();
                        view! {cx,
                            <line x1=dx y1="0" x2=dx y2={mark_origin_y/2.} style="stroke:rgb(255,0,0)" />
                            <text
                                y={mark_origin_y}
                                x={dx}
                                dominant-baseline={baseline}
                                text-anchor="middle"
                            >
                                {stick.label}
                            </text>
                        }
                    })
                    .collect::<Vec<_>>()
                }

        </g>
    }
}
