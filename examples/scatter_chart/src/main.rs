use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Cartesian::new(
        Series::from(vec![50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150]).set_range(40., 160.),
        Series::from(vec![7, 8, 8, 9, 9, 9, 10, 11, 14, 14, 15]).set_range(6., 16.),
    )
    .set_view(820, 620, 3, 100, 100, 20);
    let color = Color::from("#ff0000");

    view! {
      <div class="mx-auto p-8">
        <h1>"Scatter chart example"</h1>
        <ScatterChart chart=chart color=color/>
      </div>
    }
}
