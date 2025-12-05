# 贡献指南

感谢您对 Public Info Subscribe 项目的关注！本文档将帮助您了解如何为项目做出贡献。

## 开发环境设置

### 前置条件
- Rust 1.70 或更高版本
- Cargo（随 Rust 一起安装）

### 克隆仓库
```bash
git clone https://github.com/EK3-Qinxuan/public_info_subscribe.git
cd public_info_subscribe
```

### 构建项目
```bash
cargo build
```

### 运行测试
```bash
cargo test
```

## 代码规范

### 1. 代码风格
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 所有警告都应该被修复

```bash
# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy -- -D warnings
```

### 2. 命名规范
- 遵循 Rust 官方命名规范
- 函数和变量使用 snake_case
- 类型和 trait 使用 PascalCase
- 常量使用 SCREAMING_SNAKE_CASE

### 3. 文档注释
- 所有公开的 API 必须有文档注释
- 使用 `///` 进行文档注释
- 复杂的函数内部要有步骤注释

```rust
/// 创建新的订阅器
///
/// # Arguments
/// * `config` - 订阅配置
///
/// # Returns
/// * `Result<Self>` - 成功返回订阅器实例
pub fn new(config: SubscriptionConfig) -> Result<Self> {
    // 实现代码
}
```

### 4. 性能考虑
- 优先使用引用而非所有权转移
- 避免不必要的克隆
- 尽量减少锁的使用
- 考虑内存对齐和 cache 友好性

```rust
// ✅ 推荐：使用引用
pub fn process_data(data: &MarketData) -> Result<()> {
    // ...
}

// ❌ 不推荐：不必要的所有权转移
pub fn process_data(data: MarketData) -> Result<()> {
    // ...
}
```

### 5. 错误处理
- 使用 `Result` 类型处理可能失败的操作
- 使用 `thiserror` 定义自定义错误类型
- 提供清晰的错误信息

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),
}
```

### 6. JSON 处理
- 使用 sonic-rs 进行 JSON 序列化/反序列化
- 使用结构体类型，禁止使用 `Value`
- 为结构体派生 `Serialize` 和 `Deserialize`

```rust
use sonic_rs::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData {
    pub symbol: String,
    pub last_price: f64,
}
```

## 测试规范

### 1. 单元测试
- 每个公开函数都应该有对应的测试
- 测试函数使用 `#[test]` 标记
- 测试应该覆盖正常情况和边界情况

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = SubscriptionConfig {
            market: "binance".to_string(),
            // ...
        };
        assert!(config.validate().is_ok());
    }
}
```

### 2. 异步测试
- 使用 `#[tokio::test]` 标记异步测试

```rust
#[tokio::test]
async fn test_subscriber_creation() {
    let subscriber = Subscriber::new(config).await;
    assert!(subscriber.is_ok());
}
```

### 3. 测试覆盖率
- 目标：至少 80% 的代码覆盖率
- 使用 `cargo tarpaulin` 查看覆盖率

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 提交规范

### 提交信息格式
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响功能）
- `refactor`: 重构代码
- `test`: 添加或修改测试
- `perf`: 性能优化
- `chore`: 构建过程或辅助工具的变动

### 示例
```
feat(subscriber): add support for multiple callbacks

Add ability to register multiple callbacks for different data types.

Closes #123
```

## Pull Request 流程

### 1. Fork 项目
点击 GitHub 页面右上角的 "Fork" 按钮

### 2. 创建特性分支
```bash
git checkout -b feature/your-feature-name
```

### 3. 进行开发
- 编写代码
- 添加测试
- 更新文档

### 4. 确保代码质量
```bash
# 运行测试
cargo test

# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy -- -D warnings
```

### 5. 提交更改
```bash
git add .
git commit -m "feat: your feature description"
```

### 6. 推送到 Fork
```bash
git push origin feature/your-feature-name
```

### 7. 创建 Pull Request
- 访问 GitHub 上的原仓库
- 点击 "New Pull Request"
- 选择您的 Fork 和分支
- 填写 PR 描述
- 提交 PR

## Pull Request 检查清单

在提交 PR 之前，请确保：

- [ ] 代码遵循项目的代码规范
- [ ] 所有测试都通过
- [ ] 添加了必要的测试
- [ ] 更新了相关文档
- [ ] 运行了 `cargo fmt`
- [ ] 运行了 `cargo clippy` 并修复了所有警告
- [ ] PR 描述清楚说明了更改内容和原因
- [ ] 如果是新功能，添加了使用示例

## 代码审查

### 审查重点
1. **功能正确性**：代码是否实现了预期功能
2. **代码质量**：是否遵循最佳实践
3. **性能**：是否有性能问题
4. **安全性**：是否有安全隐患
5. **测试覆盖**：测试是否充分
6. **文档**：文档是否完善

### 响应审查意见
- 认真对待每一条审查意见
- 及时回复和修改
- 不确定的地方可以提出讨论

## 发布流程

项目维护者负责版本发布：

1. 更新版本号（Cargo.toml）
2. 更新 CHANGELOG.md
3. 创建 Git tag
4. 发布到 crates.io

## 获取帮助

如果您在贡献过程中遇到问题：

- 查看 [README.md](README.md) 和 [DESIGN.md](DESIGN.md)
- 在 GitHub Issues 中提问
- 联系项目维护者

## 行为准则

- 尊重他人
- 建设性地提出意见
- 保持友好和专业
- 欢迎新手贡献者

感谢您的贡献！🎉
