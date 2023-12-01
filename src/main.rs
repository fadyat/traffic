use crate::config::Config;

mod api;
mod cli;
mod config;
mod store;
mod ui;
mod merger;

fn main() {
    log4rs::init_file(".config/log4rs.yaml", Default::default()).unwrap();
    let c = Config::new(".config/config.yaml".to_string())
        .expect("failed to initialize config");

    let mode = if std::env::args().len() > 1 {
        cli::run_cli(&c)
    } else {
        log::set_max_level(log::LevelFilter::Off);
        ui::render_ui(&c)
    };

    if let Err(e) = mode {
        eprintln!("{}", e);
    }
}
