# public_info_subscribe

主要用作便携的接入各个市场的某些公开信息

## 特性

- 🚀 **高性能低延迟**：优先考虑低延迟，使用 lock-free 设计，最小化锁的使用
- 🔧 **模块化设计**：清晰的模块划分，易于维护和扩展
- 📦 **内存友好**：使用引用减少内存复制，空间换时间优化
- 🔒 **类型安全**：使用 sonic-rs 进行 JSON 序列化，强类型反序列化
- ⚡ **异步支持**：基于 Tokio 的异步运行时
- 📝 **完善文档**：关键结构体和函数都有详细的 Rust 标准注释

## 项目结构

```
src/
├── lib.rs          # 库入口
├── main.rs         # 应用入口和示例
├── config.rs       # 配置管理模块
├── error.rs        # 统一错误处理
├── market.rs       # 市场数据结构定义
└── subscriber.rs   # 订阅器核心实现
```

## 快速开始

### 安装依赖

```bash
cargo build
```

### 运行示例

```bash
cargo run
```

### 运行测试

```bash
cargo test
```

## 使用示例

```rust
use public_info_subscribe::{
    config::SubscriptionConfig,
    market::MarketData,
    subscriber::SubscriberBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = SubscriptionConfig {
        market: "binance".to_string(),
        data_type: "ticker".to_string(),
        symbols: vec!["BTCUSDT".to_string()],
        reconnect_interval_ms: 5000,
        heartbeat_interval_ms: 30000,
    };

    // 创建订阅器
    let mut subscriber = SubscriberBuilder::new()
        .config(config)
        .callback(|data: &MarketData| {
            println!("收到数据: {}", data.symbol());
            Ok(())
        })
        .build()?;

    // 启动订阅
    subscriber.start().await?;

    Ok(())
}
```

## 代码规范

本项目遵循以下代码规范：

1. **简洁性**：代码尽可能简洁，避免长篇复杂代码，复杂业务逻辑按功能拆分
2. **命名规范**：符合 Rust 命名规范和软件工程最佳实践
3. **高效性**：优先低延迟，少用锁，考虑 cache/内存友好
4. **完善性**：处理各种边界情况和错误情况
5. **JSON 处理**：使用 sonic-rs，用结构体反序列化而非 Value
6. **引用优先**：尽可能使用引用减少内存复制
7. **文档注释**：关键结构体字段和函数都有 Rust 标准注释

## 技术栈

- **sonic-rs**: 高性能 JSON 序列化/反序列化
- **tokio**: 异步运行时
- **thiserror**: 错误处理
- **anyhow**: 错误传播

## License

MIT
