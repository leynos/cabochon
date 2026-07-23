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
- [Cabochon technical design](cabochon-design.md) defines the runtime-first
  architecture, semantic object contracts, security model, and verification
  invariants.
- [Cabochon context](context.md) defines the shared domain vocabulary used by
  the terms of reference and technical design.
- [Roadmap](roadmap.md) sequences Cabochon's delivery as GIST-oriented,
  user-facing vertical slices.
- [Documentation style guide](documentation-style-guide.md) defines the
  spelling, structure, Markdown, Architecture Decision Record (ADR), Request
  for Comments (RFC), and roadmap conventions used by this documentation set.

## Decision records

- [ADR 001: Adopt a runtime-first product scope](adr-001-runtime-first-product-scope.md)
  records the hosted proving ground and the evidence gate for desktop work.
- [ADR 002: Keep the hosted runtime independent of the shell](adr-002-hosted-runtime-boundary.md)
  records the hexagonal runtime boundary shared by host desktops and a future
  Cabochon shell.
- [ADR 003: Separate discovery, authority, and mutation](adr-003-runtime-security-boundary.md)
  records capability, transaction, portal, audit, and export invariants.
- [ADR 004: Require interoperable authoring paths](adr-004-interoperable-authoring-paths.md)
  records the shared object contract for direct Rust and an accessible path.
- [ADR 005: Expose live-relationship state explicitly](adr-005-explicit-live-relationship-states.md)
  records the persisted dependency and failure-visibility contract.

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
