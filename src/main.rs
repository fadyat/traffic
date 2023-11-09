mod cli;
mod config;
mod ui;
mod gh_client;
mod error;
mod store;
mod plot;

fn main() {
    let mode = if std::env::args().len() > 1 {
        cli::run_cli()
    } else {
        ui::render_ui()
    };

    if let Err(e) = mode {
        eprintln!("{}", e);
    }
}