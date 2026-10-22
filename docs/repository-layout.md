# Repository layout

This document describes the generated Cabochon repository layout. It is the
canonical reference for where source code, tests, configuration, automation,
and long-lived documentation belong.

## Top-level tree

The tree below shows the generated repository structure. It is intentionally
compact and omits build output such as `target/`.

```plaintext
.
├── .cargo/
│   └── config.toml
├── .github/
│   ├── dependabot.yml
│   └── workflows/
│       ├── ci.yml
│       ├── coverage-main.yml
│       └── release.yml
├── docs/
│   ├── adrs/
│   │   ├── adr-0001-adopt-a-runtime-first-product-scope.md
│   │   └── ...
│   ├── contents.md
│   ├── developers-guide.md
│   ├── documentation-style-guide.md
│   ├── repository-layout.md
│   ├── users-guide.md
│   └── ...
├── scripts/
│   ├── check-build-tools.sh
│   ├── clang-pinned-mold.sh
│   ├── install-build-tools.sh
│   └── resolve-build-target.sh
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── ...
├── tools/
│   └── mold/
│       ├── SHA256SUMS
│       └── VERSION
├── AGENTS.md
├── Cargo.toml
├── LICENSE
├── Makefile
├── README.md
├── clippy.toml
├── codecov.yml
└── rust-toolchain.toml
```

## Path responsibilities

- `.cargo/config.toml`: Configures the default Cranelift development backend,
  parallel frontend, and Linux linker.
- `.github/dependabot.yml`: Configures automated dependency update checks.
- `.github/workflows/ci.yml`: Runs the generated project's continuous
  integration checks.
- `.github/workflows/coverage-main.yml`: Owns persistent main-branch coverage
  publication and declares the `codescene` environment. It exists. GitHub
  read-back confirms its deployment allow-list permits only `main` and
  administrator bypass is enabled. Secret placement there and removal from
  repository-level secrets remain unverified because GitHub returned HTTP 403
  for secret-name listings. Publisher run
  [38128492817](https://github.com/leynos/cabochon/actions/runs/38128492817)
  succeeded for coverage, ratchet-baseline saving, and CodeScene upload; this
  run does not establish the token's storage location.
- `.github/workflows/release.yml`: Builds and publishes binary release
  artefacts for the application flavour.
- `docs/`: Holds long-lived reference documentation, guides, style rules, and
  design material.
- `docs/contents.md`: Indexes the documentation set and should be updated when
  documentation files are added, renamed, or removed.
- `docs/users-guide.md`: Explains how to use the generated project and its
  public build and test commands.
- `docs/developers-guide.md`: Explains the contributor workflow and local
  tooling used to work on the generated project.
- `docs/repository-layout.md`: Documents the repository tree and path
  responsibilities.
- `docs/documentation-style-guide.md`: Defines the spelling, structure,
  Markdown, decision-record, and roadmap conventions used across the
  documentation set.
- `docs/adrs/`: Holds accepted Architecture Decision Records (ADRs) named
  `adr-nnnn-title-slug.md`, with the number zero-padded to four digits.

- `src/lib.rs`: Contains library support for application logic and doctested
  examples.
- `src/main.rs`: Contains the application entrypoint and top-level executable
  wiring.

- `tests/`: Holds integration and behavioural tests that exercise public
  behaviour.
- `scripts/*build-tools.sh`: Installs and checks the binary and toolchain
  prerequisites of the development standard. Make owns their invocation.
- `scripts/clang-pinned-mold.sh`: Checks the adjacent pinned `ld.mold` and
  selects its directory for Linux development links through `clang`.
- `scripts/resolve-build-target.sh`: Classifies each Make Cargo invocation's
  compilation target with the pinned rustc for flags and linker preflight.
- `tools/mold/`: Pins the supported Linux `mold` release and archive checksums.
- `AGENTS.md`: Provides repository-specific working instructions for agents and
  contributors.
- `Cargo.toml`: Defines package metadata, dependencies, lint policy, and Cargo
  configuration.
- `LICENSE`: Records the project licence text.
- `Makefile`: Provides the public build, lint, test, coverage, and
  documentation validation commands.
- `README.md`: Introduces the project and gives the shortest useful
  getting-started path.
- `clippy.toml`: Configures Clippy lint behaviour that is not expressed
  directly in `Cargo.toml`.
- `codecov.yml`: Configures coverage reporting behaviour.
- `rust-toolchain.toml`: Pins the Rust toolchain channel and required
  components.

## Ownership boundaries

- Keep generated source code under `src/`. Add modules below `src/` when a
  feature grows beyond a small entrypoint or crate root.
- Keep black-box integration tests and externally observable workflow tests
  under `tests/`.
- Keep reusable documentation under `docs/`. Update `docs/contents.md` whenever
  a documentation file is added, renamed, or removed.
- Keep build and validation entrypoints in `Makefile`; prefer adding or
  extending a Make target over documenting an ad hoc command.
- Keep continuous integration workflow changes under `.github/workflows/` and
  dependency-update policy under `.github/dependabot.yml`.
- Do not commit generated build output such as `target/`, coverage artefacts,
  or local editor state.

## Updating this document

Update this document when the repository gains a new top-level directory, a new
long-lived documentation category, a new workflow file, or a changed ownership
boundary that would otherwise make the tree misleading.
