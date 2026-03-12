# loco-logger-localtime

[Loco.rs](https://loco.rs) 的自定义日志初始化器，使用 [`tracing-appender-localtime`](https://crates.io/crates/tracing-appender-localtime) 替代标准 `tracing-appender`，使日志文件名和时间戳使用**本地时间**而非 UTC。

## 动机

Loco 内置的日志系统基于 `tracing-appender`，日志文件名中的时间戳使用 UTC 时间。对于需要本地时间的场景（如中国时区 UTC+8），文件名和日志时间与本地时间不一致，排查问题不够直观。

本库通过替换底层 appender 实现，使日志轮转文件名和日志行时间戳均采用本地时间。

## 安装

在你的 Loco 项目中添加依赖：

```toml
[dependencies]
loco-logger-localtime = { git = "https://github.com/ghjdegithub/loco-logger-localtime.git" }
```

## 用法

在 `src/app.rs` 中覆盖 `Hooks::init_logger`：

```rust
fn init_logger(ctx: &AppContext) -> Result<bool> {
    let _ = loco_logger_localtime::init::<App>(&ctx.config.logger);
    Ok(true)
}
```

然后可以从 `src/lib.rs` 中移除 `pub mod logger;` 以及对应的 `src/logger.rs` 文件（如果有的话）。

## 功能

- 日志文件名使用本地时间轮转（Minutely / Hourly / Daily / Never）
- 日志行时间戳使用本地时间（RFC 3339 格式）
- 支持 Compact、Pretty、Json 三种输出格式
- 支持非阻塞（non-blocking）文件写入
- 完全兼容 Loco 的 `config::Logger` 配置

## 许可证

MIT
