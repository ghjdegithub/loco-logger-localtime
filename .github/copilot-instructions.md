# Copilot Instructions — loco-logger-localtime

## Project Overview

单文件 Rust 库，为 [Loco.rs](https://loco.rs) 提供**本地时间**日志初始化器。使用 `tracing-appender-localtime` 替代标准 `tracing-appender`，使日志文件名和时间戳采用本地时间而非 UTC。

- **公开 API**：唯一入口 `pub fn init<H: Hooks>(config: &config::Logger) -> Result<()>`
- **依赖**：loco-rs 0.16、tracing-subscriber 0.3（env-filter / json / local-time）、tracing-appender-localtime 0.2
- **状态**：`publish = false`，仅通过 git 依赖使用

## Build & Quality Commands

```bash
cargo build                # 编译
cargo clippy               # Lint（使用默认规则）
cargo fmt --check          # 格式检查
cargo doc --open           # 生成文档
```

> 当前无测试用例（`cargo test` 无可运行项）。

## Code Conventions

- **Rust Edition 2021**，遵循标准 rustfmt / clippy 默认配置
- 错误处理使用 `loco_rs::Result<T>` / `loco_rs::Error`
- 文档注释使用中文，代码中标识符使用英文
- 单文件结构 `src/lib.rs`，保持精简，不过度拆分模块
- 常量 `MODULE_WHITELIST` 硬编码日志过滤白名单

## Architecture Notes

- `OnceLock<WorkerGuard>` 静态存储非阻塞写入的 guard，防止提前 drop 导致日志丢失
- 日志层（Layer）按配置动态组合：文件 appender + stdout，叠加 `EnvFilter`
- 支持三种日志格式：Compact / Pretty / Json，均使用 `LocalTime::rfc_3339()`
- 轮转策略直接映射 `loco_rs::logger::Rotation` 枚举

## Key Pitfalls

- 强耦合 loco-rs 0.16 的 `config::Logger` 结构，升级 loco 版本时需验证兼容性
- `tracing-appender` 在 Cargo.toml 中是 `tracing-appender-localtime` 的别名，注意不要与标准 crate 混淆
