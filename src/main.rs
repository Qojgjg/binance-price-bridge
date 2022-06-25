mod app_context;
mod binance_integration;
mod settings;
mod tcp;

pub use app_context::*;
pub use binance_integration::*;
pub use settings::*;
pub use tcp::*;

use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let settings = Arc::new(SettingsModel::load(".enonpay").await);
    let app = Arc::new(AppContext::new(settings.clone()));
    crate::app_context::setup_and_start(&app, settings.clone()).await;

    loop {
        sleep(Duration::from_secs(5)).await;
    }
}
