use plotly::common::Title;
use plotly::Plot;
use crate::gh_client::RepoView;

pub fn update(path: String, values: Vec<RepoView>) {
    let mut plot = Plot::new();
    let x = values.iter()
        .map(|view| view.timestamp.clone())
        .collect::<Vec<String>>();

    let unique_y = values.iter()
        .map(|view| view.uniques)
        .collect::<Vec<u64>>();

    let count_y = values.iter()
        .map(|view| view.count)
        .collect::<Vec<u64>>();

    plot.add_trace(plotly::Scatter::new(x.clone(), unique_y).name("uniques"));
    plot.add_trace(plotly::Scatter::new(x.clone(), count_y).name("count"));

    let layout = plotly::Layout::new()
        .title(Title::new("Repo views"))
        .x_axis(plotly::layout::Axis::new().title(Title::new("Date")))
        .y_axis(plotly::layout::Axis::new().title(Title::new("Views")));

    plot.set_layout(layout);
    plot.write_html(path);
}