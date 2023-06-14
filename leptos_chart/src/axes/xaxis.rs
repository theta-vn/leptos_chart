use leptos::{component, view, IntoView, Scope};
use theta_chart::{get_bit_at, coord::Axes};

use crate::REM;

#[allow(non_snake_case)]
#[component]
pub fn XAxis(
    cx: Scope,
    len_x: f64,
    len_y: f64,
    position: usize,
    axes: Axes,
    intervale: f64,
    
) -> impl IntoView {
    let mut mark_origin_x = 0.;
    let mut mark_origin_y = REM as f64;
    let mut baseline = "text-before-edge";
    let mut translate = format!("translate({},{})", 0, len_y);

    if get_bit_at(position, 0) {
        mark_origin_y = -REM as f64;
        baseline = "text-after-edge";
        translate = format!("translate({},{})", 0, 0);
    }

    if get_bit_at(position, 1) {
        mark_origin_x = len_x
    }

    view! {cx,
        <g class="x-axis" transform={translate}>
            <line x1="0" y1="0" x2={len_x} y2="0" style="stroke:rgb(255,0,0)" />
            <line x1={mark_origin_x} y1="0" x2={mark_origin_x} y2={mark_origin_y} style="stroke:rgb(255,0,0)" />
                {                    
                    axes.sticks.into_iter().map(|stick| {
                        let dx = stick.value * intervale;
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
