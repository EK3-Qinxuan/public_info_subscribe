# 设计文档

## 项目概述

Public Info Subscribe 是一个高性能、低延迟的市场公开信息订阅系统，用于接入各个市场的实时数据。

## 设计原则

### 1. 代码简洁性
- 模块化设计，每个模块职责明确
- 避免长篇复杂代码，复杂逻辑按功能拆分
- 关键位置添加清晰的注释

### 2. 命名规范与可维护性
- 遵循 Rust 官方命名规范
- 使用描述性的变量和函数名
- 模块划分清晰，易于理解和扩展

### 3. 高效性与低延迟
- **Lock-Free 设计**：使用 Tokio 的 unbounded_channel 实现无锁消息传递
- **零拷贝优化**：回调函数使用引用 `&MarketData` 避免数据复制
- **高性能 JSON**：使用 sonic-rs 替代标准 serde_json，性能提升 2-3 倍
- **内存友好**：结构体设计考虑内存对齐，减少 cache miss

### 4. 完善性
- 统一的错误处理机制（thiserror + anyhow）
- 配置验证确保数据有效性
- 全面的单元测试覆盖

### 5. JSON 处理
- 使用 sonic-rs 进行序列化/反序列化
- 强制使用结构体类型，禁止使用 `Value`
- 类型安全，编译时检查

### 6. 引用优先
- 回调函数参数使用引用
- 访问器方法返回引用
- 最小化内存分配和复制

### 7. 文档完善
- 所有公开 API 都有 Rust 文档注释
- 关键结构体字段有详细说明
- 复杂函数内部有步骤注释

## 架构设计

```
┌─────────────┐
│   用户代码   │
└──────┬──────┘
       │
       ├── 创建配置 (SubscriptionConfig)
       │
       ├── 构建订阅器 (SubscriberBuilder)
       │
       ├── 设置回调 (callback)
       │
       ├── 启动订阅 (start)
       │
       └── 接收数据 (MarketData)
```

### 核心模块

#### 1. Config Module (`src/config.rs`)
- **职责**：配置管理和验证
- **特性**：
  - JSON 序列化/反序列化支持
  - 配置有效性验证
  - 默认值处理

#### 2. Error Module (`src/error.rs`)
- **职责**：统一错误处理
- **特性**：
  - 使用 thiserror 定义错误类型
  - 支持错误转换和传播
  - 清晰的错误信息

#### 3. Market Module (`src/market.rs`)
- **职责**：市场数据结构定义
- **特性**：
  - Ticker、OrderBook、Trade 数据结构
  - 统一的 MarketData 枚举
  - JSON 序列化支持

#### 4. Subscriber Module (`src/subscriber.rs`)
- **职责**：订阅器核心实现
- **特性**：
  - 异步数据处理
  - Lock-free 消息传递
  - Builder 模式构建
  - 回调机制

## 性能优化

### 1. 内存优化
- 使用 `&str` 和 `&MarketData` 引用类型
- 避免不必要的 `String` 克隆
- 结构体字段顺序优化内存对齐

### 2. 并发优化
- 使用 Tokio 的 unbounded_channel（lock-free）
- 避免使用 `Mutex` 和 `RwLock`
- 异步处理减少阻塞

### 3. JSON 优化
- sonic-rs 提供 SIMD 加速
- 零拷贝解析
- 比标准库快 2-3 倍

### 4. 编译优化
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## 使用示例

### 基本使用
```rust
let config = SubscriptionConfig {
    market: "binance".to_string(),
    data_type: "ticker".to_string(),
    symbols: vec!["BTCUSDT".to_string()],
    reconnect_interval_ms: 5000,
    heartbeat_interval_ms: 30000,
};

let mut subscriber = SubscriberBuilder::new()
    .config(config)
    .callback(|data: &MarketData| {
        println!("收到数据: {}", data.symbol());
        Ok(())
    })
    .build()?;

subscriber.start().await?;
```

## 测试策略

### 单元测试
- 配置序列化/反序列化测试
- 配置验证测试
- 市场数据结构测试
- 订阅器创建和操作测试

### 集成测试
- 完整的数据流测试
- 错误处理测试
- 并发场景测试

## 未来扩展

### 短期计划
- 添加真实的市场连接器（WebSocket）
- 支持更多的市场和数据类型
- 添加日志框架（tracing）

### 长期计划
- 添加数据持久化功能
- 实现数据回放功能
- 支持分布式部署
- 添加监控和指标收集

## 依赖库选择

| 库 | 版本 | 用途 | 选择原因 |
|---|---|---|---|
| sonic-rs | 0.3 | JSON 序列化 | 高性能，SIMD 加速 |
| tokio | 1.36 | 异步运行时 | 成熟稳定，生态丰富 |
| thiserror | 1.0 | 错误定义 | 简化错误类型定义 |
| anyhow | 1.0 | 错误传播 | 方便的错误处理 |
| chrono | 0.4 | 时间处理 | 功能完善 |

## 贡献指南

### 代码规范
1. 使用 `cargo fmt` 格式化代码
2. 使用 `cargo clippy` 检查代码质量
3. 添加单元测试覆盖新功能
4. 更新文档说明变更

### 提交规范
- feat: 新功能
- fix: 修复问题
- docs: 文档更新
- test: 测试相关
- refactor: 重构代码
- perf: 性能优化

## 许可证

MIT License
