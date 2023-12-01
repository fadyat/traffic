use crate::config::Config;
use anyhow::Result;

mod api;
mod cli;
mod config;
mod store;
mod ui;
mod merger;
mod sync;


#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file(".config/log4rs.yaml", Default::default()).unwrap();
    let c = Config::new(".config/config.yaml".to_string())
        .expect("failed to initialize config");

    let mode = if std::env::args().len() > 1 {
        cli::run_cli(&c).await
    } else {
        log::set_max_level(log::LevelFilter::Off);
        ui::render_ui(&c).await
    };

    match mode {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("error: {}", e);
            Err(e)
        }
    }
}
