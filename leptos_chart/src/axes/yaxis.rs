use leptos::{component, view, IntoView, Scope};
use theta_chart::coord::{Axes, Rec};

use crate::core::REM;

#[allow(non_snake_case)]
#[component]
pub fn YAxis(cx: Scope, region: Rec, axes: Axes) -> impl IntoView {
    let vector = region.get_vector();
    let mut mark_origin_x = REM as f64;
    let mut text_anchor = "start";

    if vector.get_x() < 0. {
        mark_origin_x *= -1.;
        text_anchor = "end";
    }

    view! {cx,
       <g class="stick">
            <line x1="0" y1="0" x2="0" y2=vector.get_y() style="stroke:rgb(0,0,255)" />
            <line x1="0" y1="0" x2=mark_origin_x y2="0" style="stroke:rgb(0,0,255)" />

            {
                axes.sticks.into_iter().map(|stick| {
                    let dy = stick.value * vector.get_y();
                    view! {cx,
                        <line x1="0" y1=dy x2={mark_origin_x/2.} y2=dy style="stroke:rgb(255,0,0)" />
                        <text
                            y=dy
                            x=mark_origin_x
                            dominant-baseline="middle"
                            text-anchor=text_anchor
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
