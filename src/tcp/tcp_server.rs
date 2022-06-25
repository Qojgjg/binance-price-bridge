use std::{net::SocketAddr, sync::Arc};

use my_tcp_sockets::{tcp_connection::SocketConnection, TcpServer};
use price_src_tcp_shared::{BidAskContract, SourceFeedSerializer};

use crate::{app_context::AppContext, SettingsModel};

use super::Callback;

pub type TcpConnection = SocketConnection<BidAskContract, SourceFeedSerializer>;

pub struct PriceRouterTcpServer {
    pub app_context: Arc<AppContext>,
    pub tcp_server: TcpServer<BidAskContract, SourceFeedSerializer>,
}

impl PriceRouterTcpServer {
    pub fn new(
        app: Arc<AppContext>,
        tcp_server: TcpServer<BidAskContract, SourceFeedSerializer>,
    ) -> Self {
        Self {
            app_context: app,
            tcp_server: tcp_server,
        }
    }

    pub async fn start(&self) {
        self.tcp_server
            .start(
                Arc::new(SourceFeedSerializer::new),
                Arc::new(Callback::new(self.app_context.clone())),
                self.app_context.app_states.clone(),
                self.app_context.logger.clone(),
            )
            .await;

        println!("TCP server started");
    }
}

pub fn setup_price_tcp_server(
    app: Arc<AppContext>,
    settings: Arc<SettingsModel>,
) -> PriceRouterTcpServer {
    let tcp_server: TcpServer<BidAskContract, SourceFeedSerializer> = TcpServer::new(
        settings.service_name.clone(),
        SocketAddr::from(([0, 0, 0, 0], 8085)),
    );

    return PriceRouterTcpServer {
        app_context: app,
        tcp_server,
    };
}
