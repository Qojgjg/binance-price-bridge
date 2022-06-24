use std::{sync::Arc, time::Duration};

use binance_bridge::{SettingsModel, AppContext};
use my_settings_reader::SettingsModel;
use tokio::time::sleep;


#[tokio::main]
async fn main() {
    let settings = Arc::new(SettingsModel::load(".enonpay").await);
    let app = Arc::new(AppContext::new(settings.clone()));
    AppContext::setup_and_start(app.clone(), settings.clone()).await;

    loop {
        sleep(Duration::from_secs(5)).await;
    }
    
}
