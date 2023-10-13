use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Cartesian::new(        
        Series::from(vec![0., 1.0, 2., 3.]),
        Series::from((vec!["1982-03", "1982-07", "1983-02", "1983-04"], "%Y-%m", "month")),        
    )
    .set_view(820, 620, 3, 100, 100, 20);
    let color = Color::from("#ff0000");

    view! {
      <div class="mx-auto p-8">
        <h1>"Line chart example"</h1>
        <LineChart chart=chart color=color/>
      </div>
    }
}
