use leptos::*;
use leptos_chart::*;

fn main() {    
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let data_r = DataPie::new(
        vec![9.0, 50.0, 72.0],
        vec!["Apple", "Banana", "Cream"],
        vec![800, 600, 0b0010, 200],
    );

    let data_l = DataPie::default()        
        .set_view(vec![800, 600, 0b1000, 200])
        .set_data(vec![350.0, 200.0, 175.0])
        .set_label(vec!["Apple", "Banana", "Cream"]);

    let data_t = DataPie::default()        
        .set_view(vec![600, 800, 0b0001, 200])
        .set_data(vec![2.0, 1.0, 1.5])
        .set_label(vec!["Apple", "Banana", "Cream"]);

    let data_b = DataPie::default()        
        .set_view(vec![600, 800, 0b0100, 200])
        .set_data(vec![2.0, 1.0, 1.5])
        .set_label(vec!["Apple", "Banana", "Cream"]);


    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Example pie chart label right"</h1>
            <PieChart data=data_r />

            <h1>"Example pie chart label left"</h1>
            <PieChart data=data_l />

            <h1>"Example pie chart label top"</h1>
            <PieChart data=data_t />

            <h1>"Example pie chart label bottom"</h1>
            <PieChart data=data_b />
        </div>
    }
}
