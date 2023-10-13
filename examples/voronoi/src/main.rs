use leptos::*;
use leptos_chart::*;
use rand::Rng;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    // fill the unit square with points
    let mut rng = rand::thread_rng();
    let vx = (0..100)
        .map(|_| (96_f64 * rng.gen::<f64>() + 2_f64) )
        .collect::<Vec<_>>();
    let vy = (0..100)
        .map(|_| (96_f64 * rng.gen::<f64>() + 2_f64) )
        .collect::<Vec<_>>();
    

    let chart = Cartesian::new(
        // Series::from(vec![0, 12, 8, 9, 0, 5, 9, 5]).set_range(-3., 14.),
        // Series::from(vec![2, 7, 7, 1, 12, 5, 12, 8]).set_range(-2., 15.),
        Series::from(vx).set_range(0., 100.),
        Series::from(vy).set_range(0., 100.),
    )
    .set_view(720, 720, 3, 80, 80, 20);

    let color = Color::from("#ff0000");

    view! {
      <div class="mx-auto p-8">
        <h1>"Voronoi diagram example"</h1>
        <Voronoi chart=chart.clone()/>
      </div>

      <div class="mx-auto p-8">
        <h1>"Voronoi diagram with triangle example"</h1>
        <Voronoi chart=chart delaunay=true color=color/>
      </div>
    }
}
