# . Gallery <!-- omit in toc -->

- [Graph types](#graph-types)
  - [Cloropleth](#cloropleth)
  - [Interactive Vconcat](#interactive-vconcat)
  - [Diverging Stacked Bar Chart](#diverging-stacked-bar-chart)
  - [Scatterplot](#scatterplot)
  - [Stacked Bar Graph](#stacked-bar-graph)
  - [Stock Graph](#stock-graph)
  - [Line with confidence Interval](#line-with-confidence-interval)
  - [Pie Chart](#pie-chart)
- [Data Sources](#data-sources)
  - [From ndarray](#from-ndarray)
  - [From csv](#from-csv)
  - [From url](#from-url)
  - [From JSON spec](#from-json-spec)
  - [From JSON spec with new Data](#from-json-spec-with-new-data)
  - [From nalgebra](#from-nalgebra)
  - [From rulinalg](#from-rulinalg)
- [Miscellaneous](#miscellaneous)
  - [Without using builders](#without-using-builders)

## Graph types

### [Cloropleth](./cloropleth_unemployment.rs)

```bash
cargo run --example cloropleth_unemployment
```

Display data from a csv and a tsv as a cloropleth on a geo projection.

![Cloropleth](./res/screens/cloropleth_unemployment.png)

### [Interactive Vconcat](./vconcat_interactive.rs)

```bash
cargo run --example vconcat_interactive
```

Display two charts that can be be interact together through selection

![vconcat_interact Visualization](./res/screens/vconcat_interact.png)

### [Diverging Stacked Bar Chart](./diverging_stacked_bar_chart.rs)

```bash
cargo run --example diverging_stacked_bar_chart
```

Display data from a json after doing some transformation on it as a stacked bar chart

![Diverging Stacked Bar Chart](./res/screens/diverging_stacked_bar_chart.png)

### [Scatterplot](./scatterplot.rs)

```bash
cargo run --example scatterplot
```

Loads data from a `csv`, deserializing to a `struct`, and display a colored scatterplot.

![scatterplot](./res/screens/scatterplot.png)

### [Stacked Bar Graph](./stacked_bar_chart.rs)

```bash
cargo run --example stacked_bar_chart
```

Loads data from an URL, displaying the aggregated count by type by month as a stacked bar graph.

![stacked_bar_chart](./res/screens/stacked_bar_chart.png)

### [Stock Graph](./stock_graph.rs)

```bash
cargo run --example stock_graph
```

Loads data from a `csv`, deserializing to a `struct`, and display the graph as a line.

![stock_graph](./res/screens/stock_graph.png)

### [Line with confidence Interval](./line_with_interval.rs)

```bash
cargo run --example line_with_interval
```

Loads data from an URL, displaying the aggregated mean by year with the variance on a two level layers as a line graph.

![line_with_interval](./res/screens/line_with_interval.png)

### [Pie Chart](./pie_chart.rs)

```bash
cargo run --example pie_chart
```

Loads data from an URL, displaying the aggregated mean by year with the variance on a two level layers as a line graph.

![pie_chart](./res/screens/pie_chart.png)

## Data Sources

### [From ndarray](./from_ndarray.rs)

```bash
cargo run --example from_ndarray
```

Loads data directly from a random [`ndarray::Array2`](https://docs.rs/ndarray/latest/ndarray/type.Array2.html).

![from_ndarray](./res/screens/from_ndarray.png)

### [From csv](./from_csv.rs)

```bash
cargo run --example from_csv
```

Loads data directly from a `csv` without deserializing to a `struct`.

![from_csv](./res/screens/stock_graph.png)

### [From url](./from_url.rs)

```bash
cargo run --example from_url
```

Loads data directly from an URL.

![from_url](./res/screens/stock_graph.png)

### [From JSON spec](./from_json_spec.rs)

```bash
cargo run --example from_json_spec
```

Load the chart directly from the JSON spec.

![from_json_spec](./res/screens/diverging_stacked_bar_chart.png)

### [From JSON spec with new Data](./from_mixed_json_rust.rs)

```bash
cargo run --example from_mixed_json_rust
```

Create a chart from existing json and add new data

![from_mixed_json_rust](./res/screens/mixed.png)

### [From nalgebra](./from_nalgebra.rs)

```rust
cargo run --example from_nalgebra --features nalgebra
```

Loads data directly from a random [`nalgebra::Matrix`](https://docs.rs/nalgebra/0.19.0/nalgebra/index.html).

![nalgebra](./res/screens/rulinalg_nalgebra.png)

### [From rulinalg](./from_ndarray.rs)

```rust
cargo run --example from_rulinalg --features rulinalg
```

Loads data directly from a random [`rulinalg::matrix::Matrix`](https://athemathmo.github.io/rulinalg/doc/rulinalg/matrix/struct.Matrix.html).

![rulinalg](./res/screens/rulinalg_nalgebra.png)

### [From nalgebra](./from_nalgebra.rs)

```rust
cargo run --example from_nalgebra --features nalgebra
```

Loads data directly from a random [`nalgebra::Matrix`](https://docs.rs/nalgebra/0.19.0/nalgebra/index.html).

<img src="./res/screens/rulinalg_nalgebra.png">

### [From rulinalg](./from_ndarray.rs)

```rust
cargo run --example from_rulinalg --features rulinalg
```

Loads data directly from a random [`rulinalg::matrix::Matrix`](https://athemathmo.github.io/rulinalg/doc/rulinalg/matrix/struct.Matrix.html).

<img src="./res/screens/rulinalg_nalgebra.png">

## Miscellaneous

### [Without using builders](./without_builders.rs)

```bash
cargo run --example without_builders
```

Build a graph without using the builders. This is not the recommended way as it's more verbose and expose more the
internal structures and types, but it's possible when wanting to avoid `Result`s produced by the Builders.

![without_builders](./res/screens/stock_graph.png)
