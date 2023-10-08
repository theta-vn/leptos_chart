use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    
    let chart = CartesianGroup::new()    
    .set_view(840, 640, 3, 50, 50, 20)   
    .add_data(
        Series::from((vec!["1982", "1986", "2010", "2020", ], "%Y", "year")),
        Series::from(vec![3., 2.0, 1., 4.]),        
    )
    .add_data(
        Series::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year")),
        Series::from(vec![0., 1.0, 2., 3.]),        
    );

    view! {
        <div class="mx-auto p-8">
            <h1>"Line chart group example"</h1>
            <LineChartGroup chart=chart />
        </div>
    }
}
