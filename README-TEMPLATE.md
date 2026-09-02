# {{project-name}}

[![CI](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{gh_username}}/{{project-name}}/actions/workflows/ci.yml)
[![License: {{license}}](https://img.shields.io/badge/license-{{ license | replace: "-", "--" }}-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.96.0%2B-orange.svg)](https://www.rust-lang.org)

> {{description}}

## Features

- Rust 2024 project skeleton with the toolchain version pinned in `rust-toolchain.toml`.
- Preconfigured formatting, Clippy, tests, dependency auditing, and spell checking.
- Preconfigured GitHub Actions CI and release workflows.
- Preconfigured pre-commit hooks that run the key quality checks before every commit.
- Preconfigured open source collaboration files: contribution guide, security policy, issue templates, and a pull request template.

## Quick Start

### 1. Install Development Tools

If Rust is not installed yet, install `rustup` first:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The project pins its Rust version in `rust-toolchain.toml`, so `rustup` installs the matching toolchain automatically once you enter the directory. Installing the following tools keeps local checks consistent with CI:

```bash
cargo install --locked cargo-deny
cargo install --locked cargo-nextest
cargo install --locked typos-cli
cargo install --locked git-cliff
pip install pre-commit
```

| Tool | Purpose |
|------|---------|
| `cargo-deny` | Checks dependency advisories, licenses, and duplicate dependencies |
| `cargo-nextest` | Runs Rust tests faster and with better reporting |
| `typos` | Checks spelling in code and documentation |
| `git-cliff` | Generates changelogs from Conventional Commits |
| `pre-commit` | Runs formatting, linting, tests, and audits before commits |

### 2. Enable Pre-Commit Hooks

```bash
pre-commit install
```

Once enabled, every `git commit` runs formatting, linting, and test checks. The commit only succeeds when all of them pass.

You can also run them against the whole repository manually:

```bash
pre-commit run --all-files
```

### 3. Build, Run, And Test

For an application project:

```bash
cargo run
```

For a library project:

```bash
cargo test
```

## Common Commands

| Command | Purpose |
|---------|---------|
| `cargo build --all-features` | Builds the project with all features enabled |
| `cargo run` | Runs an application project |
| `cargo test` | Runs tests with the standard Rust test runner |
| `cargo nextest run --all-features --no-tests pass` | Runs tests with nextest, tolerating an empty test set |
| `cargo fmt --all -- --check` | Checks code formatting |
| `cargo clippy --all-targets --all-features -- -D warnings` | Runs Clippy and treats warnings as errors |
| `cargo doc --no-deps --all-features` | Builds the documentation for this project |
| `cargo deny check` | Checks dependency advisories, licenses, and duplicate dependencies |
| `typos` | Checks spelling |

Before submitting changes, run the full local check set:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --all-features --no-tests pass
cargo deny check
typos
```

## Project Layout

```text
.
├── .github/                 # CI, release, issue, and pull request templates
├── examples/                # Runnable examples
├── fixtures/                # Test data
├── src/                     # Rust source entry point
├── tests/                   # Integration tests
├── Cargo.toml               # Package metadata, dependencies, lints, and profile config
├── CONTRIBUTING.md          # Contribution guide
├── SECURITY.md              # Security policy
├── deny.toml                # cargo-deny configuration
├── rust-toolchain.toml      # Pinned Rust toolchain
└── .pre-commit-config.yaml  # Local pre-commit checks
```

## Development Workflow

1. Create a feature branch from `main`.
2. Write the code, and add unit or integration tests for important behavior.
3. Run the local check set so the results match CI.
4. Commit using Conventional Commits, for example `feat: add config loader` or `fix: handle empty input`.
5. Open a pull request describing the change, the commands used to verify it, and any related issue.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the commit conventions and the full collaboration guide.

## Release

The release workflow is triggered by pushing a `v*` tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

It creates a GitHub Release and generates the changelog with `git-cliff`. Application projects also build multi-platform binaries and upload them to the Release. Once the `CARGO_REGISTRY_TOKEN` secret is configured, non-prerelease versions are published to crates.io.

## License

This project is licensed under [{{license}}](./LICENSE).
