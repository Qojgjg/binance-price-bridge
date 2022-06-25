use std::{collections::HashMap, sync::Arc};

use my_logger::MyLogger;
use tokio::sync::Mutex;

use crate::{
    setup_and_start_binance_ws, setup_price_tcp_server, AppStates, SettingsModel, TcpConnection,
};

pub struct AppContext {
    pub connections: Mutex<HashMap<i32, Arc<TcpConnection>>>,
    pub settings: Arc<SettingsModel>,
    pub app_states: Arc<AppStates>,
    pub logger: Arc<MyLogger>,
}

impl AppContext {
    pub fn new(settings: Arc<SettingsModel>) -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
            settings: settings.clone(),
            app_states: Arc::new(AppStates::new()),
            logger: Arc::new(MyLogger::to_console()),
        }
    }
}

pub async fn setup_and_start(app_ctx: &Arc<AppContext>, settings: Arc<SettingsModel>) {
    let app_for_spawn = app_ctx.clone();
    let settings_for_spawn = settings.clone();
    tokio::spawn(async move {
        setup_and_start_binance_ws(
            app_for_spawn.clone(),
            settings_for_spawn.tickers_to_subscribe.clone(),
        );
    });

    let tcp_server = setup_price_tcp_server(app_ctx.clone(), settings.clone());
    tcp_server.start().await;

    app_ctx.app_states.set_as_initialized();
}
