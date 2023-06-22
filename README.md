# A visualization library for leptos.

The project provides chart types to draw for leptos.


- [x] PieChart
- [x] BarChart
- [x] LineChart
- [ ] AreaChart
- [ ] Scatter Chart

# Examples and Usage

## PieChart

### `Cargo.toml`

```toml
leptos = {version = "0.3.0"}
leptos_chart = {version = "0.0.2", features = ["PieChart"]}
```

### `main.rs`

```rust
use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart = Polar::new(
        Series::from(vec![1.0, 2.0, 3.]),
        Series::from(vec!["A", "B", "C"]),
    )
    .set_view(740, 540, 1, 200, 20);

    view! {cx,
        <div class="mx-auto p-8" style="background:#00000077">
            <h1>"Pie chart example with right label"</h1>
            <PieChart chart=chart />
        </div>
    }
}
```

### Result

![PieChart](./examples/assets/pie_chart.png)

### With feature debug

![PieChart with debug](./examples/assets/pie_chart_debug.png)


## BarChart

### `Cargo.toml`

```toml
leptos = {version = "0.3.0"}
leptos_chart = {version = "0.0.2", features = ["BarChart"]}
```

### `main.rs`

```rust
use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart_v = Cartesian::new(
        Series::from(vec!["A", "B", "C"]),
        Series::from(vec![1.0, 6.0, 9.]),
    )
    .set_view(820, 620, 3, 50, 50, 20);

    let chart_h = Cartesian::new(
        Series::from(vec![0.7, 1.5, 1.9]),
        Series::from(vec!["A", "B", "C"]),
    )
    .set_view(820, 620, 3, 30, 30, 20);

    view! {cx,
        <div class="mx-auto p-8" style="background:#00000077">

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_v />

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_h />
        </div>
    }
}

```
### Result (debug)

![BarChartV](./examples/assets/bar_chart_v.png)

![BarChartH](./examples/assets/bar_chart_h.png)

## LineChart

### `Cargo.toml`

```toml
leptos = {version = "0.3.0"}
leptos_chart = {version = "0.0.2", features = ["LineChart"]}
```

### `main.rs`

```rust
use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart = Cartesian::new(
        Series::from(vec![0., 1.0, 2.]),
        Series::from(vec![3.0, 1.0, 5.]),
    )
    .set_view(820, 620, 3, 100, 100, 20);

    view! {cx,
        <div class="mx-auto p-8" style="background:#00000077">
            <h1>"Line chart example"</h1>
            <LineChart chart=chart />
        </div>
    }
}
```
### Result (debug)
![LineChart](./examples/assets/line_chart.png)


# Next Steps

This is still a work in progress, so the next steps are going to be implementing more views and improving on existing functionality.
