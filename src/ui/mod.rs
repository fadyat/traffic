use crate::{
    ui::app::App,
    store::get_stored,
    config::Config,
    ui::r#type::BuildType,
};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{
        Axis,
        Block,
        Borders,
        Chart,
        Dataset,
    },
    Frame,
    Terminal,
};
use std::io::stdout;
use std::time::Duration;
use ratatui::widgets::GraphType;

mod app;
mod r#type;

fn ui(
    frame: &mut Frame,
    app: &App,
    build_type: &BuildType,
) {
    let window = app.get_window();
    let dataset_data = App::get_dataset(&window, build_type);

    frame.render_widget(
        Chart::new(vec![
            Dataset::default()
                .name(build_type.to_string())
                .graph_type(GraphType::Line)
                .data(&dataset_data)
        ])
            .block(
                Block::default()
                    .title("Repository traffic")
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center),
            )
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds(App::get_xbounds(&window)),
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds(App::get_ybounds(&window, build_type)),
            ),
        frame.size(),
    );
}

pub fn render_ui() -> Result<(), Box<dyn std::error::Error>> {
    let c = Config::new(".config/config.yaml".to_string()).expect("failed to initialize config");

    let stored_traffic =
        get_stored(&c.storage.state_path).expect("failed to retrieve data from storage");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(stored_traffic, 30);
    let mut btype = BuildType::Uniques;
    loop {
        terminal.draw(|frame| ui(frame, &app, &btype))?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('s') => btype = btype.toggle(),
                    event::KeyCode::Left => app.move_window(-1),
                    event::KeyCode::Right => app.move_window(1),
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
