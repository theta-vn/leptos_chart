use leptos::{component, view, IntoView, Scope};
use theta_chart::{get_bit_at, coord::Axes};

use crate::REM;

#[allow(non_snake_case)]
#[component]
pub fn YAxis(
    cx: Scope,
    len_x: f64,
    len_y: f64,
    position: usize,
    axes: Axes,
    intervale: f64,
    
) -> impl IntoView {
    let mut mark_origin_x = -REM as f64;
    let mut mark_origin_y = 0.;
    let mut translate = format!("translate({},{})", 0, 0);

    if get_bit_at(position, 1) {
        mark_origin_x = -REM as f64;
        translate = format!("translate({},{})", len_x, 0);
    }

    if get_bit_at(position, 2) {
        mark_origin_y = len_y
    }

    view! {cx,
        <g class="y-axis" transform={translate}>
            <line x1="0" y1="0" y2={len_y} x2="0" style="stroke:rgb(0,0,255)" />
            <line x1="0" y1={mark_origin_y} y2={mark_origin_y} x2={mark_origin_x} style="stroke:rgb(0,0,255)" />
                    {
                        axes.sticks.into_iter().map(|stick|  {
                            let dy = stick.value * intervale;
                            view! {cx,

                                <line x1="0" y1={dy} x2={mark_origin_x/2.} y2={dy} style="stroke:rgb(0,0,255)" />
                                <text
                                    y={dy}
                                    x={mark_origin_x}
                                    dominant-baseline="middle"
                                    text-anchor="end"
                                >
                                    {stick.label}
                                </text>
                                // <line x1="0" y1={dy} x2={-REM/2.} y2={dy} style="stroke:rgb(0,0,255)" />
            //                     <text x={-REM} y={dy}
            //                         dominant-baseline="middle"
            //                         text-anchor="end"
            //                     >
                            }
                        })
                        .collect::<Vec<_>>()
                    }

        </g>
    }
}
