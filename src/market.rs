//! Market data types and structures
//!
//! 定义了各种市场数据结构，使用 sonic-rs 进行序列化

use sonic_rs::{Deserialize, Serialize};

/// 市场数据类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// 行情快照
    Ticker,
    /// 订单簿
    OrderBook,
    /// 成交记录
    Trade,
}

/// Ticker 数据结构
///
/// 包含交易对的实时行情信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// 交易对符号
    pub symbol: String,

    /// 最新成交价
    pub last_price: f64,

    /// 24小时最高价
    pub high_24h: f64,

    /// 24小时最低价
    pub low_24h: f64,

    /// 24小时成交量
    pub volume_24h: f64,

    /// 时间戳（毫秒）
    pub timestamp: u64,
}

/// 订单簿档位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    /// 价格
    pub price: f64,

    /// 数量
    pub quantity: f64,
}

/// 订单簿数据结构
///
/// 包含买卖盘口数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookData {
    /// 交易对符号
    pub symbol: String,

    /// 买盘（bids）- 按价格降序排列
    pub bids: Vec<OrderBookLevel>,

    /// 卖盘（asks）- 按价格升序排列
    pub asks: Vec<OrderBookLevel>,

    /// 时间戳（毫秒）
    pub timestamp: u64,
}

/// 成交数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    /// 交易对符号
    pub symbol: String,

    /// 成交价格
    pub price: f64,

    /// 成交数量
    pub quantity: f64,

    /// 买方还是卖方主动成交
    pub is_buyer_maker: bool,

    /// 时间戳（毫秒）
    pub timestamp: u64,
}

/// 市场数据枚举
///
/// 统一的市场数据类型，便于处理不同类型的数据
#[derive(Debug, Clone)]
pub enum MarketData {
    /// Ticker 数据
    Ticker(TickerData),
    /// 订单簿数据
    OrderBook(OrderBookData),
    /// 成交数据
    Trade(TradeData),
}

impl MarketData {
    /// 获取数据的交易对符号
    pub fn symbol(&self) -> &str {
        match self {
            MarketData::Ticker(data) => &data.symbol,
            MarketData::OrderBook(data) => &data.symbol,
            MarketData::Trade(data) => &data.symbol,
        }
    }

    /// 获取数据的时间戳
    pub fn timestamp(&self) -> u64 {
        match self {
            MarketData::Ticker(data) => data.timestamp,
            MarketData::OrderBook(data) => data.timestamp,
            MarketData::Trade(data) => data.timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_data_serialization() {
        let ticker = TickerData {
            symbol: "BTCUSDT".to_string(),
            last_price: 50000.0,
            high_24h: 51000.0,
            low_24h: 49000.0,
            volume_24h: 1000.0,
            timestamp: 1234567890,
        };

        let json = sonic_rs::to_string(&ticker).unwrap();
        let deserialized: TickerData = sonic_rs::from_str(&json).unwrap();

        assert_eq!(ticker.symbol, deserialized.symbol);
        assert_eq!(ticker.last_price, deserialized.last_price);
    }

    #[test]
    fn test_market_data_accessors() {
        let ticker = MarketData::Ticker(TickerData {
            symbol: "BTCUSDT".to_string(),
            last_price: 50000.0,
            high_24h: 51000.0,
            low_24h: 49000.0,
            volume_24h: 1000.0,
            timestamp: 1234567890,
        });

        assert_eq!(ticker.symbol(), "BTCUSDT");
        assert_eq!(ticker.timestamp(), 1234567890);
    }
}
