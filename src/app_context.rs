use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use my_tcp_sockets::{tcp_connection::SocketConnection, TcpServer};
use price_src_tcp_shared::{SourceFeedSerializer, BidAskContract};
use rust_extensions::ApplicationStates;
use tokio::sync::Mutex;

use crate::{SettingsModel, setup_and_start_binance_ws, setup_price_tcp_server, PriceRouterTcpServer};

pub struct AppContext{
    pub connections: Mutex<Vec<Arc<SocketConnection<BidAskContract, SourceFeedSerializer>>>>,
    pub settings: Arc<SettingsModel>,
    pub is_initialized: AtomicBool,
}

impl AppContext{
    pub fn new(settings: Arc<SettingsModel>) -> Self {
        Self {
            connections: Mutex::new(Vec::new()),
            settings: settings.clone(),
            is_initialized: AtomicBool::new(false),
        }
    }

    pub async fn setup_and_start(app: Arc<AppContext>, settings: Arc<SettingsModel>) {
        setup_and_start_binance_ws(app.clone(), settings.tickers_to_subscribe.clone());
        let tcp_server = setup_price_tcp_server(app.clone(), settings.clone());
        tcp_server.start().await;
        app.is_initialized.store(true, Ordering::Relaxed);
    }
}

impl ApplicationStates for AppContext {
    fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::Relaxed)
    }

    fn is_shutting_down(&self) -> bool {
        false
    }
}
