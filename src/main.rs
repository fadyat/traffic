use log::info;

mod api;
mod cli;
mod config;
mod store;
mod ui;

fn main() {
    log4rs::init_file(".config/log4rs.yaml", Default::default()).unwrap();

    let mode = if std::env::args().len() > 1 {
        cli::run_cli()
    } else {

        info!("\
            Welcome to the GitHub traffic viewer!\
            UI is not implemented yet, CLI will be used instead.\
        ");

        cli::run_cli()
        // ui::render_ui()
    };

    if let Err(e) = mode {
        eprintln!("{}", e);
    }
}
