use leptos::*;
use leptos_chart::*;

fn main() {    
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {    

    let data = DataLine::default()        
        .set_view(800, 600, 50, 15)
        // .set_time((vec!["1982-04-03", "1986-02-12", "2017-02-04", "2020-05-22"], "%Y-%m-%d", "date"))
        .set_time((vec!["1982-04-03", "1986", "2017", "2020"], "%Y", "year"))
        .set_data(vec![2.0, 0.0, 1.0]);        
    
    log::debug!("LINE DATA{:#?}", data);

    view! {cx,
        <div class="mx-auto p-8">            

            <h1>"Line chart example"</h1>
            <LineChart data=data />

        </div>
    }
}
