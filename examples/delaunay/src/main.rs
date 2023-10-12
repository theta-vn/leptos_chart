use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let chart = Cartesian::new(
        Series::from(vec![0, 12, 8, 9, 0, 5, 9, 5]).set_range(-3., 14.),
        Series::from(vec![2, 7, 7, 1, 12, 5, 12, 8]).set_range(-2., 15.),
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
