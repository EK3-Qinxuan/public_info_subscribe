//! Public Info Subscribe - 市场公开信息订阅系统
//!
//! 这是一个轻量级、高性能的市场数据订阅系统
//! 支持多个市场的实时数据订阅

use public_info_subscribe::{
    config::SubscriptionConfig,
    market::{MarketData, TickerData},
    subscriber::SubscriberBuilder,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Public Info Subscribe System");
    println!("=============================\n");

    // 示例：创建订阅配置
    let config = SubscriptionConfig {
        market: "binance".to_string(),
        data_type: "ticker".to_string(),
        symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
        reconnect_interval_ms: 5000,
        heartbeat_interval_ms: 30000,
    };

    println!("Configuration:");
    println!("  Market: {}", config.market);
    println!("  Data Type: {}", config.data_type);
    println!("  Symbols: {:?}", config.symbols);
    println!();

    // 创建订阅器并设置回调
    let mut subscriber = SubscriberBuilder::new()
        .config(config)
        .callback(|data: &MarketData| {
            match data {
                MarketData::Ticker(ticker) => {
                    println!(
                        "[Ticker] {} - Price: {:.2}, Volume: {:.2}",
                        ticker.symbol, ticker.last_price, ticker.volume_24h
                    );
                }
                MarketData::OrderBook(orderbook) => {
                    println!(
                        "[OrderBook] {} - Bids: {}, Asks: {}",
                        orderbook.symbol,
                        orderbook.bids.len(),
                        orderbook.asks.len()
                    );
                }
                MarketData::Trade(trade) => {
                    println!(
                        "[Trade] {} - Price: {:.2}, Qty: {:.2}",
                        trade.symbol, trade.price, trade.quantity
                    );
                }
            }
            Ok(())
        })
        .build()?;

    // 启动订阅
    subscriber.start().await?;
    println!("Subscriber started successfully!\n");

    // 模拟接收数据
    println!("Simulating market data...\n");

    let ticker_data = MarketData::Ticker(TickerData {
        symbol: "BTCUSDT".to_string(),
        last_price: 50000.0,
        high_24h: 51000.0,
        low_24h: 49000.0,
        volume_24h: 1000.0,
        timestamp: chrono::Utc::now().timestamp_millis().max(0) as u64,
    });

    subscriber.push_data(ticker_data)?;

    // 等待处理
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    println!("\nSystem demonstration completed!");

    Ok(())
}
