# {{project-name}}

[![CI](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License: {{license}}](https://img.shields.io/badge/license-{{ license | replace: "-", "--" }}-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

> {{description}}

## 特性

- Rust 2024 项目骨架，使用 `rust-toolchain.toml` 锁定工具链版本。
- 预置格式化、Clippy、测试、依赖审计和拼写检查配置。
- 预置 GitHub Actions CI 与 Release 工作流。
- 预置 pre-commit 钩子，提交前自动运行关键质量检查。
- 预置开源协作文件：贡献指南、安全策略、Issue 模板和 PR 模板。

## 快速开始

### 1. 安装开发工具

如果还没有安装 Rust，请先安装 `rustup`：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

项目通过 `rust-toolchain.toml` 锁定 Rust 版本，进入目录后 `rustup` 会自动安装对应工具链。建议继续安装以下工具，让本地检查与 CI 保持一致：

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

| 工具 | 用途 |
|------|------|
| `cargo-deny` | 检查依赖安全公告、许可证和重复依赖 |
| `cargo-nextest` | 更快、更稳定地运行 Rust 测试 |
| `typos` | 检查代码和文档拼写 |
| `git-cliff` | 根据 Conventional Commits 生成 Changelog |
| `pre-commit` | 在提交前自动执行格式化、Lint、测试和审计 |

### 2. 启用 pre-commit 钩子

```bash
pre-commit install
```

启用后每次 `git commit` 会自动运行格式化、Lint、测试等检查，全部通过才会提交成功。

也可以手动对全量文件执行一次：

```bash
pre-commit run --all-files
```

### 3. 构建、运行和测试

如果这是应用项目：

```bash
cargo run
```

如果这是库项目：

```bash
cargo test
```

## 常用命令

| 命令 | 用途 |
|------|------|
| `cargo build --all-features` | 构建项目并启用所有 feature |
| `cargo run` | 运行应用项目 |
| `cargo test` | 使用标准 Rust 测试运行器 |
| `cargo nextest run --all-features --no-tests pass` | 使用 nextest 运行测试，允许暂时没有测试 |
| `cargo fmt --all -- --check` | 检查代码格式 |
| `cargo clippy --all-targets --all-features -- -D warnings` | 运行 Clippy，并将警告视为错误 |
| `cargo doc --no-deps --all-features` | 构建当前项目文档 |
| `cargo deny check` | 检查依赖安全公告、许可证和重复依赖 |
| `typos` | 检查拼写 |

提交前建议执行：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## 项目结构

```text
.
├── .github/                 # CI、Release、Issue 和 PR 模板
├── examples/                # 可运行示例
├── fixtures/                # 测试数据
├── src/                     # Rust 源码入口
├── tests/                   # 集成测试
├── Cargo.toml               # 包元数据、依赖、lint 和 profile 配置
├── CONTRIBUTING.md          # 贡献指南
├── SECURITY.md              # 安全策略
├── deny.toml                # cargo-deny 配置
├── rust-toolchain.toml      # Rust 工具链锁定
└── .pre-commit-config.yaml  # 本地提交前检查
```

## 开发流程

1. 从 `main` 切出功能分支。
2. 编写代码，并为重要行为补充单元测试或集成测试。
3. 运行提交前检查命令，确保本地结果与 CI 一致。
4. 使用 Conventional Commits 提交，例如 `feat: add config loader` 或 `fix: handle empty input`。
5. 创建 PR，并在描述中说明变更内容、验证命令和相关 issue。

提交规范与完整协作说明见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

## 发布

Release 工作流在推送 `v*` tag 时触发：

```bash
git tag v0.1.0
git push origin v0.1.0
```

工作流会创建 GitHub Release，并使用 `git-cliff` 生成 Changelog。应用项目会自动构建多平台二进制并上传到 Release。配置 `CARGO_REGISTRY_TOKEN` secret 后，非预发布版本会发布到 crates.io。

## License

本项目采用 [{{license}}](./LICENSE) 许可证。
