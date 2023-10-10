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
            Series::from(vec!["A", "B", "C"]),
            Series::from(vec![0.7, 1.5, 1.9]),
        )
        .add_data(
            Series::from(vec!["A", "B", "C"]),
            Series::from(vec![0.3, 0.5, 0.9]),
        );

    let color = Color::from("#ff0000");
    let shift_degrees = 180.0;

    view! {
        <div class="mx-auto p-8">
            <h1>"Bar chart group example"</h1>
            <BarChartGroup chart=chart color=color shift_degrees=shift_degrees />
        </div>
    }
}
