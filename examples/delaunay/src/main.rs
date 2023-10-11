use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Cartesian::new(
        Series::from(vec![1, 12, 7, 9, 2, 4, 9]).set_range(0., 13.),
        Series::from(vec![2, 4, 7, 1, 12, 4, 11]).set_range(0., 13.),
    )
    .set_view(720, 720, 3, 80, 80, 20);
    let color = Color::from("#0000ff");

    view! {
        <div class="mx-auto p-8">
            <h1>"Delaunay example"</h1>
            <Delaunay chart=chart color=color />
        </div>
    }
}
