# Developer Guide

This guide explains the contributor workflow for the generated Cabochon project.

## Local Workflow

Use `make all` as the public entrypoint for formatting, linting, and tests.
`make lint` runs rustdoc, Clippy, and Whitaker. `make test` prefers
`cargo nextest run` and falls back to `cargo test` when cargo-nextest is not
available. `make audit` derives the Rust workspace root with `cargo metadata`,
logs workspace member manifests, and runs `cargo audit` once from the workspace
root. `make coverage` uses `cargo llvm-cov` with `lld`.

GitHub Actions Act validation lives in `.github/workflows/act-validation.yml`.
The main `.github/workflows/ci.yml` workflow deliberately does not run
`make test WITH_ACT=1`; the separate Act workflow runs those slower
container-backed checks in parallel.

## Tooling

Debug builds use the standard LLVM backend. On Linux targets,
`.cargo/config.toml` configures clang to link with `mold` so debug builds link
quickly. Coverage generation uses `lld` because LLVM coverage tooling expects
LLVM-compatible linker behaviour.

`make dev-build` and `make dev-test` offer an opt-in accelerated path that
applies the Cranelift codegen backend alongside `mold`, via the fragment at
`tools/dev-fast/config.toml`. They require a nightly toolchain and are never
applied to release, coverage, or verification builds.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux.

### Security audit ignores

Security audit jobs may set `CARGO_AUDIT_IGNORES` for narrowly scoped RustSec
advisories that affect unused or tooling-only dependency paths. Keep each
ignore tied to a documented runtime impact analysis, and remove it when the
affected dependency leaves the graph or the project starts using the advised
runtime path.

## Lint baseline

`Cargo.toml`'s `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]` tables
are this repository's copy of the estate's phase 2 Rust baseline. Cabochon is
a single crate rather than a workspace, so the tables sit directly under
`[lints]` in the root manifest rather than under `[workspace.lints]` with
per-member `workspace = true` inheritance. `Cargo.toml` is authoritative for
the exact entries and levels; this section explains intent rather than
duplicating the list.

- Violations must be fixed. Where a fix is a genuine deferral, annotate the
  site with `#[expect(clippy::<lint>, reason = "...")]`, never `allow`: a
  fixed site's unfulfilled expectation then warns, so the backlog removes
  itself instead of rotting silently.
- `clippy.toml` carries the companion thresholds (cognitive complexity,
  argument count, function length, nesting) and the `disallowed-methods`
  list that bans direct `std::env` access. Reach for an injected environment
  reader instead of `std::env::var`/`var_os`/`vars`/`set_var`/`remove_var`.
- The pinned nightly toolchain in `rust-toolchain.toml` supplies `rustfmt`,
  `clippy`, and `rust-analyzer`, so `make lint`'s Clippy and rustdoc checks
  and editor tooling all run consistently for every contributor.
