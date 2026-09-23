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
applies the Cranelift codegen backend alongside `mold`.
`tools/dev-fast/ config.toml` is what controls that repository-local opt-in
activation: Cargo only reads it when a command passes
`--config tools/dev-fast/config.toml` explicitly, so it never affects
`.cargo/config.toml`'s repository-wide defaults. `rust-toolchain.toml` retains
the `llvm-tools-preview` and `rustc-codegen-cranelift-preview` components so
both the accelerated path and `make coverage` have what they need
pre-installed. The dev-fast path requires a nightly toolchain and is never
applied to release, coverage, or verification builds.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux.

### Security audit ignores

Security audit jobs may set `CARGO_AUDIT_IGNORES` for narrowly scoped RustSec
advisories that affect unused or tooling-only dependency paths. Keep each
ignore tied to a documented runtime impact analysis, and remove it when the
affected dependency leaves the graph or the project starts using the advised
runtime path.

## Coverage publication

Coverage has two workflows, and the split is a contract (concordat's CV-005,
`main-owned-codescene-coverage`), not a convention.

- `ci.yml` measures lld-linked lcov coverage on every pull request with the
  shared `generate-coverage` action, `with-ratchet: 'true'` and
  `publish-artefact: 'false'`. A drop against the ratchet baseline fails the
  pull request. The lane holds no CodeScene credential, has no upload step, and
  never contacts CodeScene.
- `coverage-main.yml` runs on every push to `main` (and on dispatch). It
  measures the same source with the same action, format, output path and
  default baseline files, which writes the ratchet baseline every pull request
  compares against, then uploads the report to CodeScene in explicit upload
  mode. The upload step alone binds `CS_ACCESS_TOKEN`, its `if:` carries
  `github.ref == 'refs/heads/main'` as its own conjunct (a dispatch can name
  any branch), and the workflow's concurrency group never cancels a run in
  progress, so a burst of merges cannot abandon a baseline write; dispatches
  queue in a group of their own, so one cannot replace a pending push.

The reasons are both quiet failures: a pull request from a fork cannot read the
secret, so an upload there is silently skipped, and CodeScene accepts an upload
only for a branch it analyses, which a pull request head is not.

`tests/coverage_workflows.rs` enforces the split over every workflow a pull
request can reach, following local reusable-workflow calls transitively, and
over every other workflow too: only the publisher may hold the token, name the
CodeScene host, run the CLI or the uploader, or touch the retired
`CODESCENE_CLI_SHA256` variable. It drives each rule against breaching fixtures
under `tests/coverage_workflows/`. When adding a workflow, keep CodeScene,
`cs-coverage` and the token out of it unless it is the publisher; the contract
names the clause a change breaks.

## Lint baseline

`Cargo.toml`'s `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]` tables
are this repository's copy of the estate's phase 2 Rust baseline. Cabochon is a
single crate rather than a workspace, so the tables sit directly under
`[lints]` in the root manifest rather than under `[workspace.lints]` with
per-member `workspace = true` inheritance. `Cargo.toml` is authoritative for
the exact entries and levels; this section explains intent rather than
duplicating the list.

- Violations must be fixed. Where a fix is a genuine deferral, annotate the
  site with `#[expect(clippy::<lint>, reason = "...")]`, never `allow`: a fixed
  site's unfulfilled expectation then warns, so the backlog removes itself
  instead of rotting silently.
- `clippy.toml` carries the companion thresholds (cognitive complexity,
  argument count, function length, nesting) and the `disallowed-methods` list
  that bans direct `std::env` access. Reach for an injected environment reader
  instead of `std::env::var`/`var_os`/`vars`/`set_var`/`remove_var`.
- The pinned nightly toolchain in `rust-toolchain.toml` supplies `rustfmt`,
  `clippy`, and `rust-analyzer`, so `make lint`'s Clippy and rustdoc checks and
  editor tooling all run consistently for every contributor.
