# User Guide

This guide explains how to use the generated Cabochon project after rendering
it from the template.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, `src/lib.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

Debug builds use the standard LLVM backend. Coverage generation uses `lld`
instead because LLVM coverage tools expect LLVM-compatible linker behaviour.
See the developer guide for local build tooling and linker configuration.

## Makefile Targets

The generated `Makefile` exposes these public targets:

- `make all` runs formatting checks, linting, and tests.
- `make check-fmt` verifies Rust formatting.
- `make lint` runs rustdoc, Clippy, and Whitaker with warnings denied.
- `make test` runs `cargo nextest run` when cargo-nextest is installed and
  falls back to `cargo test` otherwise. All projects also run doctests.
- `make build` builds the debug target.
- `make release` builds the release target.
- `make coverage` writes `lcov.info` using `cargo llvm-cov` and `lld`.
- `make audit` derives the Rust workspace root with `cargo metadata` and runs
  `cargo audit` once from that root.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

Install `clang`, `lld`, `python3`, and `cargo-audit` before running the full
generated workflow locally on Linux. See the developer guide for the local
build-tooling installs beyond these.
