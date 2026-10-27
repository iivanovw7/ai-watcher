mod ai;
mod app;
mod config;
mod constants;
mod error;
mod models;
mod ui;
mod utils;

use app::run_app;
use config::AppConfig;
use error::Result;
use models::state::AppState;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(AppConfig::load()?);
    let state = Arc::new(Mutex::new(AppState::new()));

    let bg_state = Arc::clone(&state);
    let bg_config = Arc::clone(&config);

    tokio::spawn(async move {
        app::background_watcher_loop(bg_state, bg_config).await;
    });

    run_app(state).await
}
