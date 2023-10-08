use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart_v = Cartesian::new(
        Series::from(vec!["A", "B", "C"]),
        Series::from(vec![1.0, 6.0, 9.]),
    )
    .set_view(820, 620, 3, 50, 50, 20);

    let chart_h = Cartesian::new(
        Series::from(vec![0.7, 1.5, 1.9]),
        Series::from(vec!["A", "B", "C"]),
    )
    .set_view(820, 620, 3, 30, 30, 20);

    view! {
        <div class="mx-auto p-8">

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_v />

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_h />
        </div>
    }
}
