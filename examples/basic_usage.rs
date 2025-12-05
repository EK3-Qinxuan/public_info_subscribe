//! 基本使用示例
//!
//! 演示如何创建和使用订阅器

use public_info_subscribe::{
    config::SubscriptionConfig,
    market::{MarketData, OrderBookData, OrderBookLevel, TickerData, TradeData},
    subscriber::SubscriberBuilder,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Public Info Subscribe - 基本使用示例 ===\n");

    // Step 1: 创建订阅配置
    let config = SubscriptionConfig {
        market: "binance".to_string(),
        data_type: "ticker".to_string(),
        symbols: vec![
            "BTCUSDT".to_string(),
            "ETHUSDT".to_string(),
            "BNBUSDT".to_string(),
        ],
        reconnect_interval_ms: 5000,
        heartbeat_interval_ms: 30000,
    };

    println!("配置信息:");
    println!("  市场: {}", config.market);
    println!("  数据类型: {}", config.data_type);
    println!("  交易对: {:?}\n", config.symbols);

    // Step 2: 创建订阅器并设置回调函数
    // 使用引用避免不必要的数据复制
    let mut subscriber = SubscriberBuilder::new()
        .config(config)
        .callback(|data: &MarketData| {
            // 根据不同的数据类型进行处理
            match data {
                MarketData::Ticker(ticker) => {
                    println!(
                        "📊 [行情] {} - 价格: ${:.2}, 24h高: ${:.2}, 24h低: ${:.2}, 成交量: {:.2}",
                        ticker.symbol,
                        ticker.last_price,
                        ticker.high_24h,
                        ticker.low_24h,
                        ticker.volume_24h
                    );
                }
                MarketData::OrderBook(orderbook) => {
                    println!(
                        "📖 [订单簿] {} - 买盘档位: {}, 卖盘档位: {}",
                        orderbook.symbol,
                        orderbook.bids.len(),
                        orderbook.asks.len()
                    );
                }
                MarketData::Trade(trade) => {
                    let side = if trade.is_buyer_maker { "卖" } else { "买" };
                    println!(
                        "💰 [成交] {} - 价格: ${:.2}, 数量: {:.4}, 方向: {}",
                        trade.symbol, trade.price, trade.quantity, side
                    );
                }
            }
            Ok(())
        })
        .build()?;

    // Step 3: 启动订阅器
    subscriber.start().await?;
    println!("✅ 订阅器启动成功!\n");

    // Step 4: 模拟接收不同类型的市场数据
    println!("模拟接收市场数据...\n");

    // 模拟 Ticker 数据
    let ticker = MarketData::Ticker(TickerData {
        symbol: "BTCUSDT".to_string(),
        last_price: 50000.0,
        high_24h: 51500.0,
        low_24h: 48900.0,
        volume_24h: 15234.56,
        timestamp: chrono::Utc::now().timestamp_millis().max(0) as u64,
    });
    subscriber.push_data(ticker)?;

    // 模拟 OrderBook 数据
    let orderbook = MarketData::OrderBook(OrderBookData {
        symbol: "ETHUSDT".to_string(),
        bids: vec![
            OrderBookLevel {
                price: 2999.0,
                quantity: 10.5,
            },
            OrderBookLevel {
                price: 2998.5,
                quantity: 15.2,
            },
        ],
        asks: vec![
            OrderBookLevel {
                price: 3000.0,
                quantity: 8.3,
            },
            OrderBookLevel {
                price: 3000.5,
                quantity: 12.1,
            },
        ],
        timestamp: chrono::Utc::now().timestamp_millis().max(0) as u64,
    });
    subscriber.push_data(orderbook)?;

    // 模拟 Trade 数据
    let trade = MarketData::Trade(TradeData {
        symbol: "BNBUSDT".to_string(),
        price: 450.5,
        quantity: 2.5,
        is_buyer_maker: false,
        timestamp: chrono::Utc::now().timestamp_millis().max(0) as u64,
    });
    subscriber.push_data(trade)?;

    // 等待数据处理完成
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    println!("\n✅ 示例运行完成!");

    Ok(())
}
