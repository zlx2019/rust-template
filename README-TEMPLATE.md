# {{project-name}}

[![CI](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License: {{license}}](https://img.shields.io/badge/license-{{ license | replace: "-", "--" }}-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

> {{description}}

## Features

- 待补充

## 快速开始

### 1. 安装开发工具

项目通过 `rust-toolchain.toml` 锁定 Rust 版本，进入目录后 rustup 会自动安装对应工具链。另需安装以下工具（与 CI 检查保持一致）：

```bash
cargo install --locked cargo-deny     # 依赖安全 / 许可证审计
cargo install cargo-nextest --locked  # 测试运行器
cargo install typos-cli               # 拼写检查
cargo install git-cliff               # Changelog 生成
pip install pre-commit                # Git 提交前检查
```

### 2. 启用 pre-commit 钩子

```bash
pre-commit install
```

启用后每次 `git commit` 会自动运行格式化、Lint、测试等检查，全部通过才会提交成功。

### 3. 构建与运行

```bash
cargo run
```

## 开发

常用命令：

```bash
cargo nextest run    # 运行测试
cargo clippy         # 静态检查
cargo fmt            # 格式化
```

提交规范与完整开发流程见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

## 项目结构

分层架构与编码约定参见 [specs/SPEC.md](./specs/SPEC.md)。

## License

本项目采用 [{{license}}](./LICENSE) 许可证。
