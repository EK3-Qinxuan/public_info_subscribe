//! Configuration module
//!
//! 提供配置管理功能，使用结构体进行 JSON 反序列化

use crate::error::{Error, Result};
use sonic_rs::{Deserialize, Serialize};

/// 订阅配置
///
/// 包含订阅所需的所有配置项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionConfig {
    /// 市场名称（例如：binance, okex, huobi）
    pub market: String,

    /// 订阅的数据类型（例如：ticker, orderbook, trade）
    pub data_type: String,

    /// 交易对列表
    pub symbols: Vec<String>,

    /// 重连间隔（毫秒）
    #[serde(default = "default_reconnect_interval")]
    pub reconnect_interval_ms: u64,

    /// 心跳间隔（毫秒）
    #[serde(default = "default_heartbeat_interval")]
    pub heartbeat_interval_ms: u64,
}

/// 默认重连间隔：5秒
fn default_reconnect_interval() -> u64 {
    5000
}

/// 默认心跳间隔：30秒
fn default_heartbeat_interval() -> u64 {
    30000
}

impl SubscriptionConfig {
    /// 从 JSON 字符串创建配置
    ///
    /// # Arguments
    /// * `json` - JSON 格式的配置字符串
    ///
    /// # Returns
    /// * `Result<Self>` - 解析成功返回配置对象，失败返回错误
    pub fn from_json(json: &str) -> Result<Self> {
        sonic_rs::from_str(json).map_err(Error::Json)
    }

    /// 将配置序列化为 JSON 字符串
    ///
    /// # Returns
    /// * `Result<String>` - 序列化成功返回 JSON 字符串，失败返回错误
    pub fn to_json(&self) -> Result<String> {
        sonic_rs::to_string(self).map_err(Error::Json)
    }

    /// 验证配置的有效性
    ///
    /// # Returns
    /// * `Result<()>` - 配置有效返回 Ok，否则返回错误
    pub fn validate(&self) -> Result<()> {
        // 检查市场名称不为空
        if self.market.is_empty() {
            return Err(Error::Config("Market name cannot be empty".to_string()));
        }

        // 检查数据类型不为空
        if self.data_type.is_empty() {
            return Err(Error::Config("Data type cannot be empty".to_string()));
        }

        // 检查至少有一个交易对
        if self.symbols.is_empty() {
            return Err(Error::Config("Symbols list cannot be empty".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = SubscriptionConfig {
            market: "binance".to_string(),
            data_type: "ticker".to_string(),
            symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
            reconnect_interval_ms: 5000,
            heartbeat_interval_ms: 30000,
        };

        let json = config.to_json().unwrap();
        let deserialized: SubscriptionConfig = SubscriptionConfig::from_json(&json).unwrap();

        assert_eq!(config.market, deserialized.market);
        assert_eq!(config.data_type, deserialized.data_type);
        assert_eq!(config.symbols, deserialized.symbols);
    }

    #[test]
    fn test_config_validation() {
        let mut config = SubscriptionConfig {
            market: "binance".to_string(),
            data_type: "ticker".to_string(),
            symbols: vec!["BTCUSDT".to_string()],
            reconnect_interval_ms: 5000,
            heartbeat_interval_ms: 30000,
        };

        assert!(config.validate().is_ok());

        config.market = String::new();
        assert!(config.validate().is_err());
    }
}
