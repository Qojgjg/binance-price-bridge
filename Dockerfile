FROM rust:slim
COPY ./target/release/binance-bridge binance-bridge
ENTRYPOINT ["./binance-bridge"]
