use leptos::*;
use leptos_chart::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn clear() {
    wasm_logger::init(wasm_logger::Config::default());
    // NO_HEADLESS=1 cargo test --test web --target wasm32-unknown-unknown
    let document = leptos::document();

    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    let chart_v = Cartesian::new(
        Series::from(vec!["A", "B", "C"]),
        Series::from(vec![1.0, 6.0, 9.]),
    )
    .set_view(820, 620, 3, 50, 50, 20);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <BarChart chart=chart_v /> },
    );

    let svg = test_wrapper.query_selector("svg").unwrap().unwrap();

    log::debug!("{:#?}", svg);
}
