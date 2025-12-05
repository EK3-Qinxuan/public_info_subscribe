//! JSON 序列化示例
//!
//! 演示如何使用 sonic-rs 进行高性能 JSON 序列化和反序列化

use public_info_subscribe::{
    config::SubscriptionConfig,
    market::{OrderBookData, OrderBookLevel, TickerData},
    Result,
};

fn main() -> Result<()> {
    println!("=== JSON 序列化/反序列化示例 ===\n");

    // 示例 1: 配置序列化
    println!("1. 配置序列化示例");
    println!("-----------------");

    let config = SubscriptionConfig {
        market: "binance".to_string(),
        data_type: "ticker".to_string(),
        symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
        reconnect_interval_ms: 5000,
        heartbeat_interval_ms: 30000,
    };

    // 序列化为 JSON
    let json_str = config.to_json()?;
    println!("配置 JSON:\n{}\n", json_str);

    // 从 JSON 反序列化
    let parsed_config = SubscriptionConfig::from_json(&json_str)?;
    println!("解析后的配置:");
    println!("  市场: {}", parsed_config.market);
    println!("  数据类型: {}", parsed_config.data_type);
    println!("  交易对: {:?}\n", parsed_config.symbols);

    // 示例 2: Ticker 数据序列化
    println!("2. Ticker 数据序列化示例");
    println!("------------------------");

    let ticker = TickerData {
        symbol: "BTCUSDT".to_string(),
        last_price: 50000.0,
        high_24h: 51000.0,
        low_24h: 49000.0,
        volume_24h: 1000.0,
        timestamp: 1234567890, // 使用固定时间戳便于示例输出的可重现性
    };

    // 使用 sonic-rs 序列化（比标准 serde_json 更快）
    let ticker_json = sonic_rs::to_string(&ticker)?;
    println!("Ticker JSON:\n{}\n", ticker_json);

    // 反序列化
    let parsed_ticker: TickerData = sonic_rs::from_str(&ticker_json)?;
    println!("解析后的 Ticker:");
    println!("  交易对: {}", parsed_ticker.symbol);
    println!("  价格: ${:.2}", parsed_ticker.last_price);
    println!("  24h最高: ${:.2}", parsed_ticker.high_24h);
    println!("  24h最低: ${:.2}\n", parsed_ticker.low_24h);

    // 示例 3: 订单簿数据序列化
    println!("3. 订单簿数据序列化示例");
    println!("-----------------------");

    let orderbook = OrderBookData {
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
            OrderBookLevel {
                price: 2998.0,
                quantity: 20.0,
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
            OrderBookLevel {
                price: 3001.0,
                quantity: 5.5,
            },
        ],
        timestamp: 1234567890, // 使用固定时间戳便于示例输出的可重现性
    };

    // 序列化为美化的 JSON（用于展示）
    let orderbook_json = sonic_rs::to_string_pretty(&orderbook)?;
    println!("订单簿 JSON:\n{}\n", orderbook_json);

    // 反序列化
    let parsed_orderbook: OrderBookData = sonic_rs::from_str(&orderbook_json)?;
    println!("解析后的订单簿:");
    println!("  交易对: {}", parsed_orderbook.symbol);
    println!("  买盘档位数: {}", parsed_orderbook.bids.len());
    println!("  卖盘档位数: {}", parsed_orderbook.asks.len());
    println!("  最佳买价: ${:.2}", parsed_orderbook.bids[0].price);
    println!("  最佳卖价: ${:.2}\n", parsed_orderbook.asks[0].price);

    // 示例 4: 性能对比说明
    println!("4. sonic-rs 性能优势");
    println!("-------------------");
    println!("sonic-rs 是一个高性能的 JSON 库：");
    println!("  ✓ 比标准 serde_json 快 2-3 倍");
    println!("  ✓ 支持 SIMD 加速");
    println!("  ✓ 零拷贝解析");
    println!("  ✓ 低延迟场景的理想选择\n");

    println!("✅ 示例完成!");

    Ok(())
}
