# Developer Guide

This guide explains the contributor workflow for the generated Cabochon project.

## Local Workflow

Use `make all` as the public entrypoint for non-mutating formatting checks,
Markdown linting, spelling, Rust linting, and tests. It runs these gates in
that order, including when Make receives `-j`. `make lint` runs rustdoc,
Clippy, and Whitaker. `make test` prefers `cargo nextest run` and falls back to
`cargo test` when cargo-nextest is not available. `make audit` derives the Rust
workspace root with `cargo metadata`, logs workspace member manifests, and runs
`cargo audit` once from the workspace root. `make coverage` uses
`cargo llvm-cov` with `lld`.

The test suite runs once per pull request, in `ci.yml`'s coverage step. That
step runs the same tests `make test` runs except the doctests, which
`build-test` runs in a step of its own with
`cargo test --doc --workspace --all-features`. The repository used to carry an
`act-validation.yml` workflow that ran `make test WITH_ACT=1`, but nothing reads
`WITH_ACT` and no test is gated on Act, so that workflow ran the whole suite a
second time and was removed. The crate declares no features, so `make test`'s
`--all-features` selects the same tests as the coverage run's default.
`tests/workflow_suite_contract.rs` holds the split.

## Tooling

Run `make install-build-tools` before development builds. It installs the
toolchain and declared components from `rust-toolchain.toml` and a checksum
verified `mold` binary at the version in `tools/mold/VERSION`. Standard build,
test, lint, and typecheck targets run `make check-build-tools` before compiling
and report missing tools with an installation command. CI's pinned `setup-rust`
action installs the same `mold` version before any Linux suite job. Other jobs
may have tooling installed without selecting it as their backend.

On Linux, install `clang`, `lld`, `python3`, `uv`, and `cargo-audit` for the
full workflow. The local Markdown targets require `mdtablefix` 0.6.0 and
`markdownlint-cli2` 0.22.1; CI installs the pinned binary and uses the pinned
Markdown lint action. With `cargo-binstall` 1.22.0 available, install the
prebuilt `mdtablefix` binary through the same no-compile route as CI:

```sh
cargo binstall --no-confirm --locked --disable-strategies compile \
  --disable-telemetry --install-path "$HOME/.local/bin" mdtablefix@0.6.0
```

Check `mdtablefix --version` and put that bin directory on `PATH`. Run
`make install-markdownlint` to install `markdownlint-cli2` 0.22.1 through `bun`
into `BUILD_TOOLS_PREFIX/bin` (by default `~/.local/bin`), which Make places
first on `PATH`. `BUN` can select another compatible executable.
`make markdownlint` checks the same globs as CI, whose pinned action bundles
the same CLI version. MD010 checks code blocks; MD013 allows code-block lines
up to 120 columns.

`make spelling` runs the pinned `typos-config-builder` v0.1.3 gate over source
and prose. It regenerates `typos.toml` from the live shared dictionary and
`typos.local.toml` before checking spelling; CI runs the same target after
setting up `uv`. A successful release pin alone does not freeze the dictionary.

The repository owns `scripts/install-build-tools.sh` and
`scripts/check-build-tools.sh` solely as entrypoints for their corresponding
Make targets. `scripts/resolve-build-target.sh` is Make's private route reader:
for each Cargo invocation it gives command-line `--target` precedence over
`CARGO_BUILD_TARGET`, asks the pinned rustc for `target_os`, and refuses an
unclassifiable target. Make uses that result both for the flags it assigns and
for the linker preflight. The helper is not a general Cargo argument parser;
other local scripts should call Make targets rather than source these files. CI
provisions the same prerequisites through the pinned `setup-rust` action.
`rustup` lists the installed LLVM and Cranelift components without their
`-preview` suffixes; the preflight accounts for those display names.

### Security audit ignores

Security audit jobs may set `CARGO_AUDIT_IGNORES` for narrowly scoped RustSec
advisories that affect unused or tooling-only dependency paths. Keep each
ignore tied to a documented runtime impact analysis, and remove it when the
affected dependency leaves the graph or the project starts using the advised
runtime path.

### Release builds

`release.yml` runs on a `v*.*.*` tag push and on `workflow_dispatch`, and
builds six targets in one matrix, each leg with a `builder`.

- **Native macOS.** `x86_64-apple-darwin` builds on `macos-15-intel` and
  `aarch64-apple-darwin` on `macos-latest`, with
  `cargo +stable build --release --manifest-path <checkout>/Cargo.toml
  --target <target>`
  from the checkout's parent. `cross` has no Docker image for Apple targets;
  on a Linux runner it falls back to host cargo, which lacks the target and
  stops with E0463, so those legs could never build there.
- **Cross for the rest.** The Linux (`x86_64`, `aarch64`), Windows GNU and
  FreeBSD legs run
  `cross +stable build --release --manifest-path
  <checkout>/Cargo.toml --target <target>`
  from the checkout's parent on `ubuntu-latest`.
- **Linker for the x86_64 Linux leg.** `.cargo/config.toml` names `clang` as
  that triple's linker for the development build (with `mold`). The `cross`
  image has gcc and no clang, so the cross step sets
  `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc`. An environment value beats
  the configuration file and `cross` forwards `CARGO_*` variables into its
  container. The development configuration is untouched.
- **No cancellation.** `fail-fast` is off, so one failing leg cannot hide
  whether the others build.
- **Dry run.** A `workflow_dispatch` builds every leg and uploads the
  artefacts, then stops: the `release` job runs only for a tag push, or for a
  dispatch on a tag ref that sets `dry-run` to `false`. A branch dispatch
  therefore never publishes. Run the dispatch on a branch before tagging; it is
  the proof that every leg builds.

`tests/release_workflow.rs` holds these clauses: each is proved by a mutation
of a copy of the real workflow that the contract must refuse.

### Compiler cache (sccache)

The shared `setup-rust` action gives sccache a local-disk directory under
`runner.temp` on a GitHub-hosted runner. The directory is restored with
`actions/cache` on every event and saved only on a push to `main`, so a pull
request reads the cache and never writes one.

- **One shared lane.** `ci.yml`'s `build-test` and `coverage-main.yml`'s
  `coverage-upload` both set `sccache-cache-discriminator: coverage`. The
  action's default discriminator is the job ID, which would give the two jobs
  different lanes and leave the pull-request lane without a writer. Only
  `coverage-upload` runs on a push to `main`, so it is the writer and
  `build-test` is the reader. Keep the two values equal.
- **What it warms.** The lane holds the artefacts of the coverage build, so the
  coverage step of `build-test` restores from it. The lint step compiles a
  different graph and gains almost nothing from it; measured on frankie, lint
  hit 2.5 % and the coverage step hit 100 %.
- **`expect-cache: any`.** A GitHub-hosted job accepts whichever cache backend
  the runner offers, so the input is set explicitly.
- **`release.yml` disables it.** The release job builds with `cross` inside a
  container that receives neither `RUSTC_WRAPPER` nor `SCCACHE_PATH`, so
  sccache is switched off there with `use-sccache: 'false'`. The contract
  `tests/sccache_lane.rs` holds all three clauses (`expect-cache`, the shared
  discriminator, and the release switch) by action name and inputs, never by a
  revision.

## The build standard

Bare Cargo development commands, the public `make build`, `make test`, and
`make typecheck` targets, and the rustdoc and Clippy phases of `make lint` use
Cranelift and the parallel `rustc` frontend (`-Zthreads=8`). On Linux, these
development routes also use the `mold` linker (`-Clink-arg=-fuse-ld=mold`).
`.cargo/config.toml` supplies the dev-profile backend and flags when Cargo runs
from this checkout; Make restates the flags because its recipes assign
`RUSTFLAGS`. On x86_64 GNU/Linux, the configured linker launcher checks that
`ld.mold` on the selected `mold` binary's `PATH` directory is the same pinned
executable, then gives that directory to `clang` with `-B`. This prevents Clang
from silently choosing an older system `ld.mold`. The local binary installer
and CI's `setup-rust` action both add their verified binary directory to `PATH`.
`mold` is Linux-only; other platforms keep their platform linker. Install the
pinned `mold` binary and keep it on `PATH` before running Linux development
targets. Cargo selects one `rustflags` source rather than merging them, so
every source repeats the frontend flag.

The public `make dev-build` and `make dev-test` targets remain compatibility
aliases for `make build` and `make test`, so they inherit the same development
flags.

An assigned `RUSTFLAGS` replaces the configuration's flags, so Make restates
the frontend and linker flags alongside warnings policy and any inherited
flags. It selects `mold` only for a Linux compilation target on a supported
Linux host, including when a Make caller uses `CARGO_BUILD_TARGET` or Cargo's
`--target` flag. The test, rustdoc, and Clippy invocations are evaluated
separately. Coverage explicitly selects LLVM for both dev and test profiles,
sets instrumentation flags without adding the development frontend or `mold`
linker flag, and uses `lld`. The local coverage and Whitaker routes remove
inherited `CARGO_ENCODED_RUSTFLAGS`, which would otherwise take precedence over
`RUSTFLAGS`. Both hosted coverage lanes fail before measurement if that encoded
variable is present; their explicit profile settings select LLVM. The pinned
coverage action then owns its instrumentation flags. The release Make target
clears `RUSTFLAGS`, invokes Cargo from outside the checkout with an absolute
manifest path, and fixes `CARGO_TARGET_DIR` to the checkout's `target/`
directory. The release recipe does not add the development frontend or linker
flags; this selects the LLVM release profile without loading development
configuration while keeping packaged output at its expected path. It selects
stable Cargo and removes inherited encoded flags and backend profile overrides;
CI's stable native and `cross` routes make the same exclusions. A direct nightly
`cargo build --release` from the repository root still gets the configured
frontend and Linux linker unless its caller excludes them. Stable Cargo rejects
the nightly-only profile key there; use the documented parent-directory and
absolute-manifest route for stable release builds. Whitaker runs from the
checkout's parent with an absolute manifest path under its installer-managed
toolchain. It removes inherited backend profile overrides and encoded Rust
flags. The temporary driver no longer inherits unstable profile settings, and
the repository check starts outside `.cargo/config.toml` discovery. Its own
`DYLINT_RUSTFLAGS=-D warnings` promotes suite findings to errors, so the lint
gate fails when a rolling lint reports a warning.

`tests/build_standard_contract.rs` checks the configured flags and evaluated
Make commands on Linux and macOS. `tests/build_backend_contract.rs` checks the
Cranelift default and the coverage, Whitaker, and release exclusions. CI
installs the linker through `setup-rust` before running the suite, including
the coverage action's indirect test route.

### Cranelift

The selected nightly reads `[profile.dev] codegen-backend = "cranelift"` from
`.cargo/config.toml`. Stable Cargo rejects that key even for a release-profile
build. The release workflow therefore runs stable Cargo or `cross` from the
checkout's parent directory with `--manifest-path` pointing into the checkout:
Cargo discovers configuration from its working directory, yet still builds this
manifest. Its empty `RUSTFLAGS` keeps the development frontend and linker out.
`tests/stable_cargo_config.rs` proves that stable Cargo refuses the profile
from the repository root, then builds the real release binary from the parent
working directory with the absolute manifest path. The full suite must pass
under the development backend on the integrated head; a successful single panic
test would not prove that contract.

## Coverage publication

Coverage has two workflows, and the split is a contract (concordat's CV-005,
`main-owned-codescene-coverage`), not a convention.
[ADR 006](adr-006-main-owns-coverage-publication.md) records the decision.
GitHub API read-back confirms the `codescene` environment admits deployments
from `main` alone. The token's location and removal from repository secrets
remain unverified pending owner evidence.

- `ci.yml` measures lld-linked lcov coverage on every pull request with the
  shared `generate-coverage` action, `with-ratchet: 'true'` and
  `publish-artefact: 'false'`. A drop against the ratchet baseline fails the
  pull request. The lane holds no CodeScene credential, has no upload step, and
  never contacts CodeScene.
- `coverage-main.yml` runs on every push to `main` and on dispatch. It measures
  the same source with the same action, format, output path, and default
  baseline files. A push to `main` writes the ratchet baseline every pull
  request compares against; a dispatch reads it without advancing it. The lane
  then uploads the report to CodeScene in explicit upload mode. A check step
  reports whether the secret is set by evaluating
  `${{ secrets.CS_ACCESS_TOKEN != '' }}` into its output, and no step holds the
  token in its `env`, because the composite upload action would hand a step
  `env` to its nested `upload-artifact` and cache steps; the upload step passes
  the secret as its `access-token` input. The check runs earlier in the
  upload's own job, under no default shell, since a step's output is readable
  only there. The upload's `if:` is exactly
  `steps.codescene-token.outputs.available == 'true'` joined by `&&` to
  `github.ref == 'refs/heads/main'` (a dispatch can name any branch, and any
  further conjunct could only narrow, defeat, or invert the upload), and the
  workflow's concurrency group, exactly
  `${{ github.workflow }}-${{ github.ref }}` at every level, never cancels a
  run in progress and never overlaps two runs, so triggered runs (push and
  dispatch) upload in commit order and a burst of merges cannot abandon a
  baseline write. A manual re-run of an older `main` run is an operator action
  that republishes that commit's coverage and baseline until the next push
  supersedes it. The workflow answers exactly a push to `main` and
  `workflow_dispatch`, and the coverage selection both lanes run is pinned in
  the contract.

The selected coverage action saves the persistent ratchet baseline only on a
push to `main`. A permitted manual dispatch on `main` can upload coverage
without advancing that baseline; the next push updates it. There is no
`schedule` trigger.

The reasons are both quiet failures: a pull request from a fork cannot read the
secret, so an upload there is silently skipped, and CodeScene accepts an upload
only for a branch it analyses, which a pull request head is not.

Coverage currently measures 0%, and that figure is honest. The crate is still
the generated template stub, and no instrumented test exercises it: its one
library function is covered only by a doctest, which the coverage run does not
instrument, and the integration tests read workflow and Makefile files rather
than calling the crate. The ratchet is therefore inert until real code lands.
The first feature pull request adds tests that exercise the crate, and from
then on the baseline protects them.

`make test-workflow-contracts` enforces the split by running
`cv005-contracts check`, the shared contract library in `leynos/shared-actions`
(`packages/cv005-contracts`), from a full commit named by `CV005_CONTRACTS_REF`
in the Makefile; CI runs it in a "Check the CV-005 contracts" step. A fix to
the rules is therefore a pin bump. The target needs `uv`, which fetches the
Python 3.13 the library runs under. The repository's parameters are in
`.github/cv005.toml`: its `repository` name and the `[selection]` the baseline
measures, which the publisher's generator must carry and every pull-request
lane must match. The library's own suite proves each rule refuses the shape it
exists to refuse, so this repository keeps no copy of the readers or the
refusal cases.

The rule covers every workflow a pull request can reach, following local
reusable-workflow calls transitively, and every other workflow too: only the
publisher may hold the token, reach a secret by a computed name, name the
CodeScene host, run the CLI or the uploader, or touch the retired
`CODESCENE_CLI_SHA256` variable. Workflows are read strictly: a duplicate key,
or a workflow declaring both `on` and `true`, is refused rather than silently
resolved, and a reading failure exits 2 rather than passing. The pull-request
surface is seeded by every event that runs a workflow for a pull request, and a
workflow a push starts, or one it calls, may run a ratcheted coverage step only
behind `if: github.event_name == 'pull_request'`, so the publisher stays the
baseline's only writer. When adding a workflow, keep CodeScene, `cs-coverage`,
and the token out of it unless it is the publisher; the library names the
clause a change breaks.

## Lint baseline

`Cargo.toml`'s `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]` tables
apply the selected Concordat Rust baseline at revision
`902d034d9da8e7ca33a0d4032770519dd1609de2`, with the measured local extension
`missing_docs_in_private_items = "deny"`. Cabochon is a single crate rather
than a workspace, so the tables sit directly under `[lints]` in the root
manifest rather than under `[workspace.lints]` with per-member
`workspace = true` inheritance. `Cargo.toml` is authoritative for the exact
entries and levels; this section explains intent rather than duplicating the
list.

- Fix violations at source. Reserve a reasoned `#[expect]` for a proven
  macro-expansion artefact or genuine floating-point arithmetic; never use
  `#[allow]` or an expectation as a source-fix backlog.
- `clippy.toml` carries the companion thresholds (cognitive complexity,
  argument count, function length, nesting) and the `disallowed-methods` list
  that bans direct `std::env` access. Reach for an injected environment reader
  instead of `std::env::var`/`var_os`/`vars`/`set_var`/`remove_var`.
- The pinned nightly toolchain in `rust-toolchain.toml` supplies `rustfmt`,
  `clippy`, and `rust-analyzer`, so `make lint`'s Clippy and rustdoc checks and
  editor tooling all run consistently for every contributor.
