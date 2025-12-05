//! Error types for the public info subscribe system
//!
//! 统一的错误处理类型，便于错误传播和处理

use thiserror::Error;

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, Error>;

/// 系统错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),

    /// 网络连接错误
    #[error("Network error: {0}")]
    Network(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Config(String),

    /// 订阅错误
    #[error("Subscription error: {0}")]
    Subscription(String),

    /// 通用错误
    #[error("Error: {0}")]
    Other(#[from] anyhow::Error),
}
