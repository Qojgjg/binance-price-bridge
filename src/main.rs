mod app_context;
mod binance_integration;
mod settings;
mod tcp;

pub use app_context::*;
pub use binance_integration::*;
pub use settings::*;
pub use tcp::*;

use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() {
    let settings = Arc::new(SettingsModel::load(".reachpay").await);
    let app = Arc::new(AppContext::new(settings.clone()));
    crate::app_context::setup_and_start(&app, settings.clone()).await;

    signal_hook::flag::register(
        signal_hook::consts::SIGTERM,
        app.app_states.is_shutting_down.clone(),
    )
    .unwrap();

    while !app.app_states.is_shutting_down() {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
