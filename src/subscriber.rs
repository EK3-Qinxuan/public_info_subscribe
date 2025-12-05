//! Subscriber module
//!
//! 提供订阅市场数据的核心功能
//! 使用 lock-free 设计，优化低延迟场景

use crate::config::SubscriptionConfig;
use crate::error::{Error, Result};
use crate::market::MarketData;
use std::sync::Arc;
use tokio::sync::mpsc;

/// 数据回调函数类型
///
/// 使用引用减少内存复制
pub type DataCallback = Arc<dyn Fn(&MarketData) -> Result<()> + Send + Sync>;

/// 订阅器
///
/// 负责管理市场数据订阅的核心结构
pub struct Subscriber {
    /// 订阅配置
    config: SubscriptionConfig,

    /// 数据回调函数
    callback: Option<DataCallback>,

    /// 数据发送通道（用于异步处理）
    tx: Option<mpsc::UnboundedSender<MarketData>>,
}

impl Subscriber {
    /// 创建新的订阅器
    ///
    /// # Arguments
    /// * `config` - 订阅配置
    ///
    /// # Returns
    /// * `Result<Self>` - 成功返回订阅器实例
    pub fn new(config: SubscriptionConfig) -> Result<Self> {
        // 验证配置
        config.validate()?;

        Ok(Self {
            config,
            callback: None,
            tx: None,
        })
    }

    /// 设置数据回调函数
    ///
    /// # Arguments
    /// * `callback` - 接收到数据时调用的回调函数
    ///
    /// # Returns
    /// * `&mut Self` - 返回自身引用，支持链式调用
    pub fn with_callback<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&MarketData) -> Result<()> + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
        self
    }

    /// 启动订阅
    ///
    /// 这个方法会启动异步任务来处理数据接收
    ///
    /// # Returns
    /// * `Result<()>` - 成功返回 Ok，失败返回错误
    pub async fn start(&mut self) -> Result<()> {
        // Step 1: 创建无界通道用于数据传输
        let (tx, mut rx) = mpsc::unbounded_channel::<MarketData>();
        self.tx = Some(tx);

        // Step 2: 克隆回调函数以在异步任务中使用
        let callback = self.callback.clone();

        // Step 3: 启动数据处理任务
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                // 如果有回调函数，调用它
                if let Some(ref cb) = callback {
                    // 使用引用避免复制
                    if let Err(e) = cb(&data) {
                        eprintln!("Callback error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// 停止订阅
    ///
    /// 关闭数据通道，停止接收数据
    pub fn stop(&mut self) {
        self.tx = None;
    }

    /// 手动推送数据（用于测试或手动数据注入）
    ///
    /// # Arguments
    /// * `data` - 要推送的市场数据
    ///
    /// # Returns
    /// * `Result<()>` - 成功返回 Ok，失败返回错误
    pub fn push_data(&self, data: MarketData) -> Result<()> {
        if let Some(ref tx) = self.tx {
            tx.send(data)
                .map_err(|e| Error::Subscription(format!("Failed to send data: {}", e)))?;
        }
        Ok(())
    }

    /// 获取配置引用
    pub fn config(&self) -> &SubscriptionConfig {
        &self.config
    }
}

/// 订阅器构建器
///
/// 提供更灵活的订阅器构建方式
pub struct SubscriberBuilder {
    config: Option<SubscriptionConfig>,
    callback: Option<DataCallback>,
}

impl SubscriberBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            callback: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: SubscriptionConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置回调函数
    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&MarketData) -> Result<()> + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback));
        self
    }

    /// 构建订阅器
    pub fn build(self) -> Result<Subscriber> {
        let config = self
            .config
            .ok_or_else(|| Error::Config("Configuration is required".to_string()))?;

        let mut subscriber = Subscriber::new(config)?;
        subscriber.callback = self.callback;
        Ok(subscriber)
    }
}

impl Default for SubscriberBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::TickerData;

    #[tokio::test]
    async fn test_subscriber_creation() {
        let config = SubscriptionConfig {
            market: "binance".to_string(),
            data_type: "ticker".to_string(),
            symbols: vec!["BTCUSDT".to_string()],
            reconnect_interval_ms: 5000,
            heartbeat_interval_ms: 30000,
        };

        let subscriber = Subscriber::new(config);
        assert!(subscriber.is_ok());
    }

    #[tokio::test]
    async fn test_subscriber_with_callback() {
        let config = SubscriptionConfig {
            market: "binance".to_string(),
            data_type: "ticker".to_string(),
            symbols: vec!["BTCUSDT".to_string()],
            reconnect_interval_ms: 5000,
            heartbeat_interval_ms: 30000,
        };

        let mut subscriber = Subscriber::new(config).unwrap();
        subscriber.with_callback(|data| {
            println!("Received data for: {}", data.symbol());
            Ok(())
        });

        subscriber.start().await.unwrap();

        // 推送测试数据
        let ticker = MarketData::Ticker(TickerData {
            symbol: "BTCUSDT".to_string(),
            last_price: 50000.0,
            high_24h: 51000.0,
            low_24h: 49000.0,
            volume_24h: 1000.0,
            timestamp: 1234567890,
        });

        subscriber.push_data(ticker).unwrap();

        // 等待异步处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        subscriber.stop();
    }

    #[tokio::test]
    async fn test_subscriber_builder() {
        let config = SubscriptionConfig {
            market: "binance".to_string(),
            data_type: "ticker".to_string(),
            symbols: vec!["BTCUSDT".to_string()],
            reconnect_interval_ms: 5000,
            heartbeat_interval_ms: 30000,
        };

        let subscriber = SubscriberBuilder::new()
            .config(config)
            .callback(|data| {
                println!("Builder callback: {}", data.symbol());
                Ok(())
            })
            .build();

        assert!(subscriber.is_ok());
    }
}
