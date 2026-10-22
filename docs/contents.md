# Documentation contents

[Documentation contents](contents.md) is the index for Cabochon's documentation
set.

## Project guides

- [User guide](users-guide.md) explains how to use the generated project and
  its public build and test commands.
- [Developer guide](developers-guide.md) explains the local workflow and
  implementation tooling for contributors.
- [Repository layout](repository-layout.md) explains the generated project's
  top-level files, directories, and ownership boundaries.
- [Terms of reference](terms-of-reference.md) defines Cabochon's problem
  space, audiences, scope boundaries, constraints, and unresolved questions.
- [Cabochon technical design](cabochon-design.md) defines the GEM-inspired
  application and graphics substrate, Enfilade object contracts, security
  model, hosted boundary, and verification invariants.
- [Cabochon context](context.md) defines the shared domain vocabulary used by
  the terms of reference and technical design.
- [Roadmap](roadmap.md) sequences Cabochon's delivery as GIST-oriented,
  user-facing vertical slices.
- [Documentation style guide](documentation-style-guide.md) defines the
  spelling, structure, Markdown, Architecture Decision Record (ADR), Request
  for Comments (RFC), migration-guide, and roadmap conventions used by this
  documentation set.

## Decision records

Decision records live under `docs/adrs/` as `adr-nnnn-title-slug.md`, per the
[ADR naming convention](documentation-style-guide.md#naming-convention).

- [ADR 0001: Adopt a runtime-first product scope](adrs/adr-0001-adopt-a-runtime-first-product-scope.md)
  records the hosted proving ground and the evidence gate for desktop work.
- [ADR 0002: Keep the hosted runtime independent of the shell](adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md)
  records the hexagonal runtime boundary shared by host desktops and a future
  Cabochon shell.
- [ADR 0003: Separate discovery, authority, and mutation](adrs/adr-0003-separate-discovery-authority-and-mutation.md)
  records capability, transaction, portal, audit, and export invariants.
- [ADR 0004: Require interoperable authoring paths](adrs/adr-0004-require-interoperable-authoring-paths.md)
  records the shared object contract for direct Rust and an accessible path.
- [ADR 0005: Expose live-relationship state explicitly](adrs/adr-0005-expose-live-relationship-state-explicitly.md)
  records the persisted dependency and failure-visibility contract.
- [ADR 0006: `main` owns coverage publication](adrs/adr-0006-main-owns-coverage-publication.md)
  records the split between the pull-request ratchet and the CodeScene
  publisher on `main`.
- [ADR 0007: Make the GEM substrate a production boundary](adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md)
  records Clerestory, Lapidary, Burin, and Escutcheon as production contracts
  in hosted mode while Mullion stays deferred.
- [ADR 0008: Keep Enfilade out of rendering authority](adrs/adr-0008-keep-enfilade-out-of-rendering-authority.md)
  records that Cabochon display, print, export, thumbnail, clipboard, and
  accessibility output crosses Lapidary and Burin contracts.

## Execution plans

- `execplans/` holds the task-level plans for roadmap step 1.0, which
  settles the Clerestory, Escutcheon, Lapidary, and Burin contracts. Implement
  them in this order:
  - [1.0.3: Define the minimum Lapidary scene contract](execplans/1-0-3-lapidary-scene-contract.md)
    converts the workspace, installs the prover tooling, and delivers the
    proved scene model.
  - [1.0.4: Define the minimum Burin output contract](execplans/1-0-4-burin-output-contract.md)
    renders one scene to every output and proves cross-target parity.
  - [1.0.2: Define the minimum Escutcheon resource format](execplans/1-0-2-escutcheon-resource-format.md)
    compiles localizable resources and can proceed in parallel with 1.0.4.
  - [1.0.1: Define the minimum Clerestory application contract](execplans/1-0-1-clerestory-application-contract.md)
    runs the resource-defined hello-window application in a host desktop.

## Rust reference material

- [Reliable testing in Rust via dependency injection](reliable-testing-in-rust-via-dependency-injection.md)
  explains how to keep tests deterministic by injecting environment, clock,
  filesystem, and other external dependencies.
- [Rust doctest Don't Repeat Yourself guide](rust-doctest-dry-guide.md)
  explains how to write maintainable, executable Rust documentation examples.
- [Rust testing with `rstest` fixtures](rust-testing-with-rstest-fixtures.md)
  explains fixture-based, parameterized, and asynchronous testing with `rstest`.

## Engineering practice

- [Complexity antipatterns and refactoring strategies](complexity-antipatterns-and-refactoring-strategies.md)
  explains cognitive complexity, the bumpy-road antipattern, and refactoring
  approaches for maintainable code.
- [Scripting standards](scripting-standards.md) explains the preferred Python
  scripting stack, command execution patterns, and test expectations for helper
  scripts.
