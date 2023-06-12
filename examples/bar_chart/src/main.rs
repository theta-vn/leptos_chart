use leptos::*;
use leptos_chart::*;

fn main() {    
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // let data_n = DataBar::new(
    //     vec![74., 7., 46.],
    //     vec!["Apples", "Bananas", "Cherries"],
    //     vec![770, 570, 0b1100, 50, 10],
    // );

    let data_h = DataBar::default()        
        .set_view(800, 600, false, 50, 15)
        .set_data(vec![370.0, 200.0, 175.0])
        .set_label(vec!["A", "B", "C"]);
    
    // let data_t = DataPie::default()        
    //     .set_view(vec![600, 800, 0b0001, 200])
    //     .set_data(vec![2.0, 1.0, 1.5])
    //     .set_label(vec!["Apples", "Bananas", "Cherries"]);

    // let data_b = DataPie::default()        
    //     .set_view(vec![600, 800, 0b0100, 200])
    //     .set_data_u64(vec![22, 47, 15])
    //     .set_label(vec!["Apples", "Bananas", "Cherries"]);
    let data_v = DataBar::default()        
    .set_view(800, 600, true, 50, 10)
    .set_data(vec![370.0, 200.0, 175.0])
    .set_label(vec!["Apples", "Bananas", "Cherries"]);

    view! {cx,
        <div class="mx-auto p-8">
            // <h1>"Bar chart example"</h1>
            // <BarChart data=data_n />

            <h1>"Bar chart example"</h1>
            <BarChart data=data_h />


            <h1>"Bar chart example"</h1>
            <BarChart data=data_v />
        </div>
    }
}
