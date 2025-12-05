# public_info_subscribe

便携化的各市场 WebSocket 公共频道 SDK 原型（Rust 工作区）。目标：统一订阅协议、可插拔交易所连接器、可复用的 SDK/CLI。

## 工作区结构

- `Cargo.toml`：工作区与共享依赖。
- `core` (`pis-core`)：抽象层。传输/编解码/归一化的 trait 与基础类型、回退策略、指标接口等。
- `connectors/binance` (`connector-binance`)：示例交易所连接器，展示如何实现 codec + normalizer。
- `sdk` (`pis-sdk`)：面向用户的高层封装，提供 `Client`/`ClientBuilder` 管理多个连接器。
- `cli` (`pis-cli`)：最小 CLI，使用空实现的传输（仅打印待发送的订阅请求）。
- `examples` / `benches`：预留目录，后续可补充示例与基准。

## 快速开始

```powershell
# 构建全量工作区
cargo build --workspace

# 运行演示 CLI（使用空传输，仅打印订阅消息）
cargo run -p pis-cli
```

## 扩展指引

1. 在 `connectors/<exchange>` 新建连接器 crate，实现：
	- `ExchangeConnector`（定义 endpoint 与默认订阅集）
	- `Codec`（订阅消息编码、入站帧解码）
	- `Normalizer`（将解码后的原始消息归一化为 `NormalizedEvent`）
2. 在 `sdk::ClientBuilder` 挂接新连接器特性，暴露便捷方法（如 `with_okx()`）。
3. 在 `Transport` 的具体实现中接入实际 WebSocket（`tokio-tungstenite` 等），并替换 CLI 中的空传输。
4. 为连接器添加 fixtures 与单元测试，验证编解码与归一化逻辑。

## 下一步建议

- 实现真实的 WebSocket transport，并加入心跳、重连、速率限制等策略。
- 补充更多连接器（OKX/Bybit/Huobi 等），并完善 `Normalizer` 以输出统一模型。
- 增加示例：将归一化事件写入 Kafka/文件、合并 orderbook、指标导出等。
