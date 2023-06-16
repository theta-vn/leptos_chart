use leptos::*;
use leptos_chart::*;

fn main() {    
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {   

    let data_r = DataPie::default()        
        .set_view(800, 600, 0b0010, 200, 10)
        .set_data(vec![9.0, 50.0, 72.0])
        .set_label(vec!["Apples", "Bananas", "Cherries"]);

    let data_l = DataPie::default()        
        .set_view(800, 600, 0b1000, 200, 10)
        .set_data(vec![350.0, 200.0, 175.0])
        .set_label(vec!["Apples", "Bananas", "Cherries"]);

    let data_t = DataPie::default()        
        .set_view(600, 800, 0b0001, 200, 10)
        .set_data(vec![2.0, 1.0, 1.5])
        .set_label(vec!["Apples", "Bananas", "Cherries"]);

    let data_b = DataPie::default()        
        .set_view(600, 800, 0b0100, 200, 10)
        .set_data_u64(vec![22, 47, 15])
        .set_label(vec!["Apples", "Bananas", "Cherries"]);


    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Pie chart example with right label"</h1>
            <PieChart data=data_r />

            <h1>"Pie chart example with left label"</h1>
            <PieChart data=data_l />

            <h1>"Pie chart example with top label"</h1>
            <PieChart data=data_t />

            <h1>"Pie chart example with bottom label"</h1>
            <PieChart data=data_b />
        </div>
    }
}
