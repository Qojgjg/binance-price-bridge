use std::sync::Arc;

use my_tcp_sockets::{SocketEventCallback, ConnectionEvent};
use price_src_tcp_shared::{SourceFeedSerializer, BidAskContract};

use crate::app_context::AppContext;

pub struct Callback{
    app: Arc<AppContext>
}

impl Callback {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app: app }
    }
}


#[async_trait::async_trait]
impl SocketEventCallback<BidAskContract, SourceFeedSerializer> for Callback{
    async fn handle(&self, connection_event: ConnectionEvent<BidAskContract, SourceFeedSerializer>){
        match connection_event{
            ConnectionEvent::Connected(connection) => {
                let mut write = self.app.connections.lock().await;
                println!("handled new connection");
                write.push(connection);
                
            },
            ConnectionEvent::Disconnected(connection) => {
                println!("Disconnected");
            },
            ConnectionEvent::Payload{connection, payload} => {
                println!("Received payload from {:?}", payload);
            }
        }
    }
}