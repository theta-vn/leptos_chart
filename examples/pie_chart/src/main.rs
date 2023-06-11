use leptos::*;
use leptos_chart::*;

fn main() {    
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let data_n = DataPie::new(
        vec![9.0, 50.0, 72.0],
        vec!["Apple", "Banana", "Cream"],
        vec![800, 600, 0b0010, 200],
    );

    let data_d = DataPie::default()        
        .set_view(vec![800, 600, 0b1000, 200])
        .set_data(vec![350.0, 200.0, 175.0])
        .set_label(vec!["Apple", "Banana", "Cream"]);

    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Example pie chart"</h1>
            <PieChart data=data_n />
            <PieChart data=data_d />
        </div>
    }
}
