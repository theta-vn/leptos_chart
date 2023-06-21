use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart = Cartesian::new(
        Series::from(vec![0., 1.0, 2.]),
        Series::from(vec![3.0, 1.0, 5.]),
    )
    .set_view(820, 620, 3, 100, 100, 20);

    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Line chart example"</h1>
            <LineChart chart=chart />
        </div>
    }
}
