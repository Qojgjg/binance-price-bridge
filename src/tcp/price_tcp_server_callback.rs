use std::sync::Arc;

use my_tcp_sockets::{ConnectionEvent, SocketEventCallback};
use price_src_tcp_shared::{BidAskContract, SourceFeedSerializer};

use crate::app_context::AppContext;

pub struct Callback {
    app: Arc<AppContext>,
}

impl Callback {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app: app }
    }
}

#[async_trait::async_trait]
impl SocketEventCallback<BidAskContract, SourceFeedSerializer> for Callback {
    async fn handle(
        &self,
        connection_event: ConnectionEvent<BidAskContract, SourceFeedSerializer>,
    ) {
        match connection_event {
            ConnectionEvent::Connected(connection) => {
                let mut write_access = self.app.connections.lock().await;
                println!("New connection {}", connection.id);
                write_access.insert(connection.id, connection);
            }
            ConnectionEvent::Disconnected(connection) => {
                let mut write_access = self.app.connections.lock().await;
                write_access.remove(&connection.id);
                println!("Disconnected {}", connection.id);
            }
            ConnectionEvent::Payload {
                connection,
                payload,
            } => {
                if payload.is_ping() {
                    connection.send(BidAskContract::Pong).await;
                }
                println!("Received payload from {:?}", payload);
            }
        }
    }
}
