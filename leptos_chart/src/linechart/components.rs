use crate::{
    axes::{XAxis, YAxis},
    core::SvgChart,
};

use leptos::{component, view, IntoView, Scope};

use theta_chart::{chart::{Draw, ScaleNumber, ScaleTime, self, ScaleType}, coord};

#[allow(non_snake_case)]
#[component]
pub fn LineChart (cx: Scope, chart: coord::Chart) -> impl IntoView 
{
    // let chart = data.get_chart();
    log::debug!("CHART:::{:#?}", chart);
    let cview = chart.get_view();
    // let origin = view.get_origin();
    // let inner = view.get_inner();
    // let translate_axes = format!("translate({},{})", origin.get_x(), origin.get_y());
    // // Flip SVG vertically using an unusual viewbox
    // let translate_chart = format!(
    //     "translate({},{}) scale(1,-1)",
    //     origin.get_x(),
    //     inner.get_y()
    // );

    // // For processing x-axis
    // let time = chart.get_ax();
    // let intervale_x = inner.get_x();
    // let axes_x = time.gen_axes();

    // // For processing y-axis
    // let data_y = chart.get_ay();
    // let intervale_y = inner.get_y();
    // let axes_y = data_y.gen_axes();

    // For position
    let position_origin = cview.get_position_origin();


    // For Chart
    let rec_chart = cview.get_rec_chart();
    let translate_chart = format!("translate({},{})", rec_chart.get_origin().get_x(), rec_chart.get_origin().get_y());
    
    // For x-axis
    let rec_xa = cview.get_rec_x_axis();
    let translate_xa = format!("translate({},{})", rec_xa.get_origin().get_x(), rec_xa.get_origin().get_y());
    let series_x = chart.get_ax();
    let axes_x = series_x.gen_axes();
    // log::debug!("{:#?}", axes_x);

    // For y-axis
    let rec_ya = cview.get_rec_y_axis();
    let translate_ya = format!("translate({},{})", rec_ya.get_origin().get_x(), rec_ya.get_origin().get_y());
    let series_y = chart.get_ay();
    let axes_y = series_y.gen_axes();
    // log::debug!(&axes_x);

    // For chart
    let xseries = chart.get_ax();
    let yseries = chart.get_ay();

    

    let xsticks = xseries.to_stick();
    let ysticks = yseries.to_stick();

    view! { cx,

        <SvgChart cview={cview}>
          
            <g class="axes">
                <g class="x-axis" transform={translate_xa}>
                    // For draw region of x-axis                    
                    {
                        // let origin = rec_xa.get_origin();
                        let vector = rec_xa.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {cx,
                            <circle id="originX" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#ff000033;stroke-width:1" />    
                            <path id="regionX" d=path  fill="#ff000033" />
                        }                        
                    }
                    
                    <XAxis region=rec_xa axes=axes_x />
                </g>
                <g class="y-axis" transform={translate_ya}>
                    // For draw region of y-axis                    
                    {
                        // let origin = rec_ya.get_origin();
                        let vector = rec_ya.get_vector();
                        let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                        view! {cx,
                            <circle id="originY" cx="0" cy="0" r="3" />
                            <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#0000ff33;stroke-width:2" />      
                            <path id="regionY" d=path  fill="#0000ff33" />
                        }                        
                    }
                    <YAxis region=rec_ya axes=axes_y />
                </g>                
            </g>
            <g class="inner-chart"  transform={translate_chart}>
                // For draw region of chart
                {
                    // let origin = rec_ya.get_origin();
                    let vector = rec_chart.get_vector();
                    let path = format!("M {},{} l {},{} l {},{} l {},{} Z", 0, 0, vector.get_x(), 0, 0,vector.get_y(), -vector.get_x(), 0);
                    view! {cx,
                        <circle id="originY" cx="0" cy="0" r="3" />
                        <line x1="0" y1="0" x2=vector.get_x() y2=vector.get_y() style="stroke:#00ff0033;stroke-width:2" />      
                        <path id="regionY" d=path  fill="#00ff0033" />
                    }                        
                }
                {
                    let vector = rec_chart.get_vector();
                    let mut line = "M".to_string();
                    let point = xsticks.into_iter().enumerate().map(|(index, data)|  {
                        let x: f64 = xseries.scale(data.value) * vector.get_x();
                        let y: f64 = yseries.scale(ysticks[index].value) *vector.get_y();
                        line.push_str(format!(" {:.0},{:.0} ", x, y).as_str());
                        view! {cx,
                            <circle cx={x} cy={y}  r="2" stroke="black" stroke-width="1" fill="red" />
                        }
                    }).collect::<Vec<_>>();

                    view! {cx,
                        {point}
                        <path d={line} stroke="red" fill="none"/>
                    }
                }
            </g>
        </SvgChart>
    }
}
