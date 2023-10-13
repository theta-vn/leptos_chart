use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Polar::new(
        Series::from(vec![85.0, 55.0, 45., 60., 40.]),
        Series::from(vec!["Reading", "Writing", "Listening", "Speaking", "React"]),
    )
    .set_view(740, 540, 1, 0, 20);
    let color = Color::from("#ff0000");
    view! {
      <div class="mx-auto p-8">
        <h1>"Radar chart example"</h1>
        <RadarChart chart=chart color=color/>
      </div>
    }
}
