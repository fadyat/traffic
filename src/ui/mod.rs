use std::io::stdout;
use crossterm::{
    event::KeyEventKind,
    event,
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Frame,
    Terminal,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Chart, Dataset},
    widgets::{Axis, GraphType},
};
use crate::config::Config;
use crate::store::get_stored;
use crate::ui::app::App;

mod app;


fn ui(frame: &mut Frame, app: &App) {
    let binding = app.get_window()
        .iter()
        .map(|view| (view.index as f64, view.view.uniques as f64))
        .collect::<Vec<(f64, f64)>>();

    let datasets = vec![
        Dataset::default()
            .data(binding.as_slice())
            .graph_type(GraphType::Line)
    ];

    let xmin = app.get_window()
        .iter()
        .map(|view| view.index)
        .min()
        .unwrap_or(0);

    let xmax = app.get_window()
        .iter()
        .map(|view| view.index)
        .max()
        .unwrap_or(0);

    frame.render_widget(
        Chart::new(datasets)
            .block(
                Block::default()
                    .title("Repository traffic")
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center)
            )
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([xmin as f64, xmax as f64])
                // .labels(xlabels)
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 30.0])
                // .labels(ylabels)
            ),
        frame.size(),
    );
}


pub fn render_ui() -> Result<(), Box<dyn std::error::Error>> {
    let c = Config::new(".config/config.yaml".to_string())
        .expect("failed to initialize config");

    let stored_traffic = get_stored(&c.storage.state_path)
        .expect("failed to retrieve data from storage");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(
        CrosstermBackend::new(stdout())
    )?;
    terminal.clear()?;

    let app = App::new(stored_traffic);
    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}