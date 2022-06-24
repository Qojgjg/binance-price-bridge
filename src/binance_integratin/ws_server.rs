use std::sync::{Arc, atomic::Ordering};

use binance::websockets::{WebSockets, WebsocketEvent};
use price_src_tcp_shared::{BidAskContract, BidAsk};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_context::AppContext;


pub fn setup_and_start_binance_ws(app: Arc<AppContext>, tickers: Vec<String>) {
    let mut endpoints: Vec<String> = Vec::new();

    for symbol in tickers.iter() {
        endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()));
    }
 
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event| {
        if !app.is_initialized.load(Ordering::Relaxed) {
            print!("Handled message from binance but skip. App is not initialized.");
            return Ok(());
        }

        handle_event(event, app.clone())
    });
    web_socket.connect_multiple_streams(&endpoints).unwrap();
    print!("Connected to binance websocket");
}

fn handle_event(event: WebsocketEvent, app: Arc<AppContext>) -> Result<(),binance::errors::Error >{

    if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
        let time = depth_order_book.event_time;
        let symbol = depth_order_book.symbol;
        let bids = depth_order_book.bids.last();
        let asks = depth_order_book.asks.last();

        if bids.is_none() || asks.is_none() {
            return Ok(());
        }

        let to_publish = BidAskContract::BidAsk(BidAsk{
            date_time: price_src_tcp_shared::BidAskDateTime::Source(DateTimeAsMicroseconds::new((time * 1000) as i64)),
            id: symbol,
            bid: bids.unwrap().price,
            ask: asks.unwrap().price,
            source: "binance".into()
        });

        let swawn = app.clone();
        tokio::spawn(async move {
            let mut write = swawn.connections.lock().await;
            for connection in write.iter_mut() {
                connection.send(to_publish.clone()).await;
            }
        });
    }
    Ok(())
}

