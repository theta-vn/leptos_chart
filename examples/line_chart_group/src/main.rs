use leptos::*;
use leptos_chart::*;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    
    let chart = CartesianGroup::new()    
    .set_view(840, 640, 3, 50, 50, 20)
    // .add_data(
    //     Series::from(vec![0., 1.0, 2., 3., 4., 5.]),
    //     Series::from(vec![3.0, 1.0, 5., 8., 7., 2.]),
    // )
    // .add_data(
    //     Series::from(vec![0., 1.0, 2., 3., 4., 5.]),
    //     Series::from(vec![7.0, 8.0, 4., 4., 9., 4.]),
    // );
    .add_data(
        Series::from((vec!["1982", "1986", "2010", "2020", ], "%Y", "year")),
        Series::from(vec![3., 2.0, 1., 4.]),        
    )
    .add_data(
        Series::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year")),
        Series::from(vec![0., 1.0, 2., 3.]),        
    );
    

    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Line chart example"</h1>
            <LineChartGroup chart=chart />
        </div>
    }
}
