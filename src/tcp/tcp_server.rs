use std::{sync::Arc, net::SocketAddr};

use my_logger::MyLogger;
use my_tcp_sockets::TcpServer;
use price_src_tcp_shared::{SourceFeedSerializer, BidAskContract};

use crate::{app_context::AppContext, SettingsModel};

use super::Callback;

pub struct PriceRouterTcpServer{
    pub app_context:  Arc<AppContext>,
    pub tcp_server: TcpServer<BidAskContract, SourceFeedSerializer>
}

impl PriceRouterTcpServer{
    pub fn new(app: Arc<AppContext>, tcp_server: TcpServer<BidAskContract, SourceFeedSerializer>) -> Self {
        Self {
            app_context: app,
            tcp_server: tcp_server
        }
    }

    pub async fn start(&self) {
        self.tcp_server
            .start(self.app_context.clone(), Arc::new(SourceFeedSerializer::new),
        Arc::new(Callback::new(self.app_context.clone()))
    ).await;

    println!("TCP server started");
    }
}

pub fn setup_price_tcp_server(app: Arc<AppContext>, settings: Arc<SettingsModel>) -> PriceRouterTcpServer{
    let tcp_server : TcpServer<BidAskContract, SourceFeedSerializer> = TcpServer::new_with_logger(
        settings.service_name.clone(),
        SocketAddr::from(([0, 0, 0, 0], 8085)),
        Arc::new(MyLogger::to_concole())
    );


    return PriceRouterTcpServer{ app_context: app, tcp_server } ;
}