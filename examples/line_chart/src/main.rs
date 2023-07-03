use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart = Cartesian::new(
        // Series::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year")),
        // Series::from((vec!["1982-04", "1986-02", "2017-02", "2020-05"], "%Y-%m", "month")),
        Series::from(vec![0., 1.0, 2., 3.]),
        Series::from((vec!["1982-03", "1982-07", "1983-02", "1983-04"], "%Y-%m", "month")),
        
    )
    .set_view(820, 620, 3, 100, 100, 20);

    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Line chart example"</h1>
            <LineChart chart=chart />
        </div>
    }
}
