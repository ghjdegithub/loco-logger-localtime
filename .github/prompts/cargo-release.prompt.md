---
description: "生成 crate 发布前的检查清单和必要变更"
---

# Cargo Release 检查清单

请为本 crate 生成发布到 crates.io 前的完整检查清单，并自动修复可以处理的项目。

## 检查项

### Cargo.toml 元数据
- [ ] 移除 `publish = false` 或改为 `publish = true`
- [ ] 添加 `license` 字段（如 `MIT` 或 `MIT OR Apache-2.0`）
- [ ] 添加 `repository` 字段指向 GitHub 仓库
- [ ] 添加 `keywords` 和 `categories` 字段
- [ ] 确认 `version` 是否需要更新

### 必要文件
- [ ] 添加 LICENSE 文件（与 Cargo.toml 中声明的许可证一致）
- [ ] 确认 README.md 已存在且内容准确
- [ ] 确认 CHANGELOG.md 是否需要创建

### 代码质量
- [ ] 运行 `cargo clippy` 无警告
- [ ] 运行 `cargo fmt --check` 无格式问题
- [ ] 运行 `cargo doc --no-deps` 文档无错误
- [ ] 公开 API 均有文档注释

### 发布验证
- [ ] 运行 `cargo package --list` 检查打包内容
- [ ] 运行 `cargo publish --dry-run` 模拟发布

请逐项检查并修复所有问题，最后给出汇总报告。
