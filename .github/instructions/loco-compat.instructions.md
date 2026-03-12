---
applyTo: "**/Cargo.toml"
---

# Loco.rs 版本兼容性检查

当修改 `Cargo.toml` 中的依赖版本时，必须执行以下检查：

## loco-rs 升级检查

1. **验证 `config::Logger` 结构兼容性**：`init` 函数直接依赖 `loco_rs::config::Logger` 的字段（`file_appender`、`enable`、`format`、`level`、`override_filter`），升级 loco-rs 后确认这些字段未被重命名或移除。
2. **验证 `logger::Rotation` 枚举**：`Rotation` 的变体（Minutely / Hourly / Daily / Never）直接映射到 `tracing_appender::rolling::Rotation`，新增变体需要同步处理。
3. **验证 `logger::Format` 枚举**：`Format` 的变体（Compact / Pretty / Json）用于选择日志输出格式，新增变体需要同步处理。
4. **验证 `Hooks` trait**：`init` 的泛型约束 `H: Hooks` 使用了 `H::app_name()`，确认该方法签名未变更。

## tracing 生态升级检查

- `tracing-appender-localtime` 是 `tracing-appender` 的别名导入，确保版本与 `tracing-subscriber` 兼容。
- `tracing-subscriber` 的 `local-time` feature 是关键依赖，确认新版本仍提供 `LocalTime::rfc_3339()`。

## 快速验证命令

```bash
cargo build        # 编译通过
cargo clippy       # 无新警告
```
