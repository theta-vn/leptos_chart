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

    // // start by rendering our counter and mounting it to the DOM
    // // note that we start at the initial value of 10

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

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! {
            <div class="mx-auto p-8">
                <h1>"Bar chart group example"</h1>
                <BarChartGroup chart=chart />
            </div>
        },
    );


    let svg = test_wrapper.query_selector("svg").unwrap().unwrap();    
    
    log::debug!("{:#?}", svg.inner_html());
    // assert_eq!(
    //     div.outer_html(),
    //     // here we spawn a mini reactive system, just to render the
    //     // test case
    //     {
    //         // it's as if we're creating it with a value of 0, right?
    //         let (value, _set_value) = create_signal(0);

    //         // we can remove the event listeners because they're not rendered to HTML
    //         view! {
    //             <div>
    //                 <button>"Clear"</button>
    //                 <button>"-1"</button>
    //                 <span>"Value: " {value} "!"</span>
    //                 <button>"+1"</button>
    //             </div>
    //         }
    //         // the view returned an HtmlElement<Div>, which is a smart pointer for
    //         // a DOM element. So we can still just call .outer_html()
    //         .outer_html()
    //     }
    // );

    // // There's actually an easier way to do this...
    // // We can just test against a <SimpleCounter/> with the initial value 0
    // assert_eq!(test_wrapper.inner_html(), {
    //     let comparison_wrapper = document.create_element("section").unwrap();
    //     leptos::mount_to(
    //         comparison_wrapper.clone().unchecked_into(),
    //         || view! { <SimpleCounter initial_value=0 step=1/>},
    //     );
    //     comparison_wrapper.inner_html()
    // });


    
}
