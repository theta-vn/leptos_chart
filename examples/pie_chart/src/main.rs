use leptos::*;
use leptos_chart::*;

fn main() {    
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let data_pie = DataPie::new(
        vec![350.0, 200.0, 175.0],
        vec!["Apple", "Banana", "Cream"],
        vec![800, 600, 0b0010, 200],
    );

    view! {cx,
        <div class="mx-auto p-8">
            <h1>"Example pie chart"</h1>
            <PieChart data=data_pie />
        </div>
    }
}
