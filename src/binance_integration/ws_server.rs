use std::sync::Arc;

use binance::websockets::{WebSockets, WebsocketEvent};
use price_src_tcp_shared::{BidAsk, BidAskContract};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_context::AppContext;

pub fn setup_and_start_binance_ws(app: Arc<AppContext>, tickers: Vec<String>) {
    let mut endpoints: Vec<String> = Vec::new();

    for symbol in tickers.iter() {
        endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()));
    }

    println!(
        "Connecting to Binance websocket... Tickers: {:?}",
        endpoints
    );
    let mut web_socket: WebSockets<'_> = WebSockets::new(|event| {
        if !app.app_states.is_initialized() {
            print!("Handled message from binance, but skip. App is not initialized.");
            return Ok(());
        }

        handle_event(event, app.clone())
    });
    web_socket.connect_multiple_streams(&endpoints).unwrap();

    print!("Connected to binance websocket");
    if let Err(e) = web_socket.event_loop(&app.app_states.is_initialized) {
        println!("Error: {:?}", e);
    }

    web_socket.disconnect().unwrap();
}

fn handle_event(event: WebsocketEvent, app: Arc<AppContext>) -> Result<(), binance::errors::Error> {
    if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
        let time = depth_order_book.event_time;
        let symbol = depth_order_book.symbol;
        let bids = depth_order_book.bids.last();
        let asks = depth_order_book.asks.last();

        if bids.is_none() || asks.is_none() {
            return Ok(());
        }

        let to_publish = BidAskContract::BidAsk(BidAsk {
            date_time: price_src_tcp_shared::BidAskDateTime::Source(DateTimeAsMicroseconds::new(
                (time * 1000) as i64,
            )),
            id: symbol,
            bid: bids.unwrap().price,
            ask: asks.unwrap().price,
            source: "binance".into(),
        });

        let swawn = app.clone();
        tokio::spawn(async move {
            let mut write = swawn.connections.lock().await;
            for connection in write.values_mut() {
                connection.send(to_publish.clone()).await;
            }
        });
    }
    Ok(())
}
