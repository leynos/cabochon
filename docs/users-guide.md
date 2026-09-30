# User Guide

This guide explains how to use the generated Cabochon project after rendering
it from the template.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, `src/lib.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

Bare development Cargo commands and standard Make build, test and typecheck
targets use Cranelift and the parallel `rustc` frontend (`-Zthreads=8`). On
Linux, they link with the pinned `mold` binary. The Rustdoc and Clippy parts of
`make lint` use the same standard; Whitaker uses its installer-managed
toolchain.

Coverage uses LLVM and `lld`, while `make release` uses stable Cargo and the
production linker. Neither adds the development frontend or linker flags. See
the developer guide for local build-tool installation and routing details.

## Makefile Targets

The generated `Makefile` exposes these public targets:

- `make all` runs formatting checks, linting, tests and the workflow contract
  check below.
- `make check-fmt` verifies Rust formatting.
- `make lint` runs rustdoc, Clippy, and Whitaker with warnings denied.
- `make test` runs `cargo nextest run` when cargo-nextest is installed and
  falls back to `cargo test` otherwise. All projects also run doctests.
- `make build` builds the debug target.
- `make release` builds the release target.
- `make coverage` writes `lcov.info` using `cargo llvm-cov` and `lld`.
- `make audit` derives the Rust workspace root with `cargo metadata` and runs
  `cargo audit` once from that root.
- `make test-workflow-contracts` runs the shared CV-005 CodeScene contract
  (`cv005-contracts check`, pinned in the Makefile) over the repository's
  workflows. It needs `uv`, which fetches Python 3.13 itself.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux. See the developer guide for the local
build-tooling installs beyond these.
