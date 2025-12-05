//! Public Information Subscription Library
//!
//! 这个库提供了一个简洁、高效的接口来订阅各个市场的公开信息。
//! 设计原则：
//! - 低延迟优先，最小化锁的使用
//! - 使用引用减少内存复制
//! - 使用 sonic-rs 进行 JSON 序列化/反序列化
//! - 模块化设计，易于维护和扩展

pub mod config;
pub mod error;
pub mod market;
pub mod subscriber;

pub use error::{Error, Result};
