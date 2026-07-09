# rust-template

[English](./README.md)

[![Template CI](https://github.com/zlx2019/rust-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

一个面向开源项目的 Rust 项目模板，用于通过 [cargo-generate](https://github.com/cargo-generate/cargo-generate) 快速创建带有完整工程化配置的新项目。模板默认包含 Rust 2024、CI、Release、格式化、Lint、测试、依赖审计、拼写检查、pre-commit 和基础目录结构，适合作为 CLI、服务端应用或库项目的起点。

## 特性

- Rust 2024 项目骨架，支持生成 `application` 或 `library` 两种入口。
- 可选依赖模板：Tokio、axum、actix-web、anyhow、thiserror、tracing、serde_json、clap、reqwest 等常用 crate。
- 固定 Rust 工具链版本，减少不同机器和 CI 环境中的版本漂移。
- 预置严格但实用的 lint 规则：`unsafe_code`、`missing_docs`、`unwrap_used`、`expect_used`、`panic`、`dbg_macro`。
- 预置 GitHub Actions：格式化、Clippy、文档构建、nextest、cargo-deny、typos。
- 预置 Release 流程：基于 tag 创建 GitHub Release、生成 Changelog、应用项目打包多平台二进制、发布 crates.io。
- 预置开源协作文件：`CONTRIBUTING.md`、`SECURITY.md`、Issue 模板、PR 模板、Dependabot 配置。

## 快速开始

### 1. 安装基础工具

先安装 Rust 工具链和 `cargo-generate`：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

如果已经安装过 Rust，可以更新到最新稳定版后再生成项目：

```bash
rustup update stable
```

### 2. 生成项目

从 GitHub 模板生成新项目：

```bash
cargo generate zlx2019/rust-template
```

也可以指定项目名，减少交互步骤：

```bash
cargo generate zlx2019/rust-template --name my-app
```

生成过程中会依次询问以下信息：

| 选项 | 说明 |
|------|------|
| `Github username` | 用于生成 README 徽章、`Cargo.toml` 主页和仓库链接 |
| `description` | 项目简介，会写入 README 和 `Cargo.toml` |
| `license` | 开源许可证，支持 `MIT` / `Apache-2.0` / `GPL-3.0`，并自动生成对应 `LICENSE` 文件 |
| `project_type` | 选择 `application` 或 `library`，模板会保留对应的 `src/main.rs` 或 `src/lib.rs` |
| `ask_for_async` | 是否启用 Tokio 异步运行时 |
| `multi_choice` | 需要启用的 Tokio features，例如 `full`、`rt-multi-thread`、`macros`、`time` |
| `ask_for_web` | 是否生成 Web 项目依赖 |
| `choice` | Web 框架选择，支持 `axum` / `actix-web` |
| `ask_common_libs` | 常用基础库选择，如 `anyhow`、`thiserror`、`tracing`、`serde_json`、`clap`、`reqwest` 等 |

### 3. 进入项目并安装开发工具

```bash
cd my-app
```

生成后的项目会通过 `rust-toolchain.toml` 锁定 Rust 版本。进入目录后，`rustup` 会按需安装对应工具链。建议继续安装以下工具，让本地检查与 CI 保持一致：

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

工具用途：

| 工具 | 用途 |
|------|------|
| `cargo-deny` | 检查依赖安全公告、许可证和重复依赖 |
| `cargo-nextest` | 更快、更稳定地运行 Rust 测试 |
| `typos` | 检查代码和文档拼写 |
| `git-cliff` | 根据 Conventional Commits 生成 Changelog |
| `pre-commit` | 在提交前自动执行格式化、Lint、测试和审计 |

### 4. 启用提交前检查

```bash
pre-commit install
```

启用后，每次 `git commit` 会自动运行配置在 `.pre-commit-config.yaml` 中的检查。也可以手动对全量文件执行一次：

```bash
pre-commit run --all-files
```

### 5. 构建、运行和测试

应用项目：

```bash
cargo run
```

库项目：

```bash
cargo test
```

提交前建议执行完整检查：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## 生成后的目录结构

```text
.
├── .github/                 # CI、Release、Issue 和 PR 模板
├── examples/                # 可运行示例
├── fixtures/                # 测试数据
├── src/                     # Rust 源码入口
├── tests/                   # 集成测试
├── Cargo.toml               # 包元数据、依赖、lint 和 profile 配置
├── README.md                # 生成项目的自述文件
├── CONTRIBUTING.md          # 贡献指南
├── SECURITY.md              # 安全策略
├── deny.toml                # cargo-deny 配置
├── rust-toolchain.toml      # Rust 工具链锁定
└── .pre-commit-config.yaml  # 本地提交前检查
```

## 本模板如何维护

这个仓库本身也是模板项目，`Cargo.toml` 和部分文件包含 Liquid 占位符，不能直接当作普通 Rust crate 构建。模板 CI 会先用 `cargo generate --path .` 展开项目，再对生成结果执行格式化、Clippy、文档、测试、依赖审计和拼写检查。

本地验证模板时可以运行：

```bash
cargo generate --path . --name smoke-test --destination /tmp
cd /tmp/smoke-test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## License

本项目采用 [MIT](./LICENSE) 许可证。
