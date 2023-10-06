use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Polar::new(
        Series::from(vec![1.0, 2.0, 5.]),
        Series::from(vec!["A", "B", "C"]),
    )
    .set_view(740, 540, 1, 200, 20);

    view! {
        <div class="mx-auto p-8">
            <h1>"Pie chart example with right label"</h1>
            <PieChart chart=chart />
        </div>
    }
}
