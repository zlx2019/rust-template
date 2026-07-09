# rust-template

[简体中文](./README.zh.md)

[![Template CI](https://github.com/zlx2019/rust-template/actions/workflows/template-ci.yml/badge.svg)](https://github.com/zlx2019/rust-template/actions/workflows/template-ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

A Rust project template for open source projects, powered by [cargo-generate](https://github.com/cargo-generate/cargo-generate). It creates a new Rust project with Rust 2024, CI, release automation, formatting, linting, tests, dependency auditing, spell checking, pre-commit hooks, and a practical starter layout for CLI tools, backend applications, and library crates.

## Features

- Rust 2024 project skeleton with either an `application` or `library` entry point.
- Optional dependency presets for common crates, including Tokio, axum, actix-web, anyhow, thiserror, tracing, serde_json, clap, and reqwest.
- Pinned Rust toolchain to reduce version drift across local machines and CI.
- Strict but practical lint defaults for `unsafe_code`, `missing_docs`, `unwrap_used`, `expect_used`, `panic`, and `dbg_macro`.
- GitHub Actions workflows for formatting, Clippy, documentation builds, nextest, cargo-deny, and typos.
- Release workflow for tag-based GitHub Releases, changelog generation, binary packaging for applications, and crates.io publishing.
- Open source collaboration files, including `CONTRIBUTING.md`, `SECURITY.md`, issue templates, a pull request template, and Dependabot configuration.

## Quick Start

### 1. Install Prerequisites

Install Rust and `cargo-generate`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked cargo-generate
```

If Rust is already installed, update the stable toolchain before generating a project:

```bash
rustup update stable
```

### 2. Generate A Project

Generate a new project from this GitHub template:

```bash
cargo generate zlx2019/rust-template
```

You can also provide the project name up front:

```bash
cargo generate zlx2019/rust-template --name my-app
```

The generator will ask for the following values:

| Option | Description |
|--------|-------------|
| `Github username` | Used for README badges and `Cargo.toml` homepage/repository links |
| `description` | Project description written to README and `Cargo.toml` |
| `license` | Open source license, one of `MIT`, `Apache-2.0`, or `GPL-3.0`; the matching `LICENSE` file is generated automatically |
| `project_type` | Choose `application` or `library`; the template keeps either `src/main.rs` or `src/lib.rs` |
| `ask_for_async` | Whether to enable the Tokio async runtime |
| `multi_choice` | Tokio features to enable, such as `full`, `rt-multi-thread`, `macros`, or `time` |
| `ask_for_web` | Whether to add web application dependencies |
| `choice` | Web framework choice, either `axum` or `actix-web` |
| `ask_common_libs` | Common crate choices, such as `anyhow`, `thiserror`, `tracing`, `serde_json`, `clap`, and `reqwest` |

### 3. Install Project Development Tools

```bash
cd my-app
```

The generated project pins its Rust version in `rust-toolchain.toml`. After entering the project directory, `rustup` will install the required toolchain when needed. Install the following tools to match the local workflow with CI:

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

Tool usage:

| Tool | Purpose |
|------|---------|
| `cargo-deny` | Checks dependency advisories, licenses, and duplicate dependencies |
| `cargo-nextest` | Runs Rust tests faster and with better reporting |
| `typos` | Checks spelling in code and documentation |
| `git-cliff` | Generates changelogs from Conventional Commits |
| `pre-commit` | Runs formatting, linting, tests, and audits before commits |

### 4. Enable Pre-Commit Checks

```bash
pre-commit install
```

After installation, each `git commit` runs the checks configured in `.pre-commit-config.yaml`. You can also run them against the whole repository manually:

```bash
pre-commit run --all-files
```

### 5. Build, Run, And Test

For application projects:

```bash
cargo run
```

For library projects:

```bash
cargo test
```

Before submitting changes, run the full local check set:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## Generated Project Layout

```text
.
├── .github/                 # CI, release, issue, and pull request templates
├── examples/                # Runnable examples
├── fixtures/                # Test data
├── src/                     # Rust source entry point
├── tests/                   # Integration tests
├── Cargo.toml               # Package metadata, dependencies, lints, and profile config
├── README.md                # README for the generated project
├── CONTRIBUTING.md          # Contribution guide
├── SECURITY.md              # Security policy
├── deny.toml                # cargo-deny configuration
├── rust-toolchain.toml      # Pinned Rust toolchain
└── .pre-commit-config.yaml  # Local pre-commit checks
```

## Maintaining This Template

This repository is itself a template. `Cargo.toml` and some files contain Liquid placeholders, so the repository cannot be built directly as a normal Rust crate. Template CI first expands the template with `cargo generate --path .`, then runs formatting, Clippy, documentation, tests, dependency auditing, and spell checking against the generated project.

To validate the template locally:

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

This project is licensed under [MIT](./LICENSE).
