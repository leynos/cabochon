# Developer-experience roadmap

This roadmap selects and supports one accessible authoring path alongside
Rust. [The programme map](roadmap.md) defines delivery gates and file-qualified
identifiers. H refers to the [hosted-runtime
roadmap](roadmap-hosted-runtime.md). Later work is conditional on the prototype
results; these tasks do not authorize three permanent language toolchains.

[ADR 004](adr-004-interoperable-authoring-paths.md) and [technical
design](cabochon-design.md) §§7.1, 9.1, and 15 remain authoritative. An early
Rust-only engineering fixture is not the first advertised developer experience.
G3 cannot pass without an accessible route over the same semantics.

## 1. Compare bounded authoring experiments against a working contract

Idea: a real provider fixture can reveal language-boundary and learning costs
more reliably than designing a new language before the runtime runs.

Entry: H 2.2 is complete. Evaluation criteria may be prepared earlier, but a
comparison must exercise the actual identity, dispatch, and transaction
contracts. Exit: an ADR selects the initial accessible path using comparable
evidence, or records that the available candidates do not yet meet the brief.

### 1.1. Freeze the comparison fixture and evaluation protocol

This step makes the comparison falsifiable. See design §9.1 and the
[terms of reference](terms-of-reference.md) §§4-5 and 7.

- [ ] 1.1.1. Publish a language-neutral currency fixture from the working Rust
  contract, including identity, revisions, selectors, grants, transactions,
  presentation selection, and document insertion.
  - Use fixed values and failure cases. No candidate may pass by using a
    separate object store or bypassing runtime authority checks.
- [ ] 1.1.2. Record the comparison rubric and a bounded prototype budget.
  - Requires 1.1.1. Measure authored concepts, setup steps, first-result effort,
    diagnostic quality, interoperability, and access to lower-level behaviour.
  - Name the intended Python- or JavaScript-familiar audience and the evidence
    needed to select a path before implementing the candidates.

### 1.2. Compare only the candidate surfaces needed by the fixture

Requires step 1.1 and H 2.3. See ADR 004. Each experiment must end with a result
and a limitations note; an inconclusive spike does not silently become a
compiler or language-runtime programme.

- [ ] 1.2.1. Measure the Rust baseline and implement one selector-declaration
  candidate with an Objective Rust-shaped surface for the fixture.
  - Do not implement unrelated syntax, a general compiler, or an IDE.
- [ ] 1.2.2. Implement one trait-oriented dynamic-language candidate against the
  same fixture and conformance tests.
  - Preserve all five shared semantics: identity, selectors, capabilities,
    transactions, and presentations. Prefer embedding existing machinery to
    constructing another language implementation.
- [ ] 1.2.3. Compare the candidates and record the selected accessible path.
  - Requires 1.2.1-1.2.2. Include failed cases, binding maintenance costs,
    rejected alternatives, and the route from first success to direct Rust.
  - A failed comparison pauses the G3 claim; it does not redefine the audience
    or remove accessible authoring without a separate product decision.

## 2. Make the selected path a usable, interoperable entry point

Idea: one small but supported authoring path is more valuable than several
impressive demonstrations that cannot exchange real objects or explain errors.

Requires phase 1. Exit: both authoring paths pass the shared fixture, and an
independent developer can complete and explain the exercise using the written
instructions. G3 additionally requires H phase 3's integration evidence.

### 2.1. Harden only the selected binding

See design §§7.1, 7.4, and 9.1. Keep Rust first class without duplicating object
identity, storage, security policy, or rendering contracts in the binding.

- [ ] 2.1.1. Implement the selected binding's resource, selector, argument,
  result, and error mappings for the currency fixture.
  - Preserve useful diagnostics at the language boundary. Unsupported values
    or operations fail explicitly; provider errors do not corrupt the runtime.
- [ ] 2.1.2. Extend the shared provider harness to both language
  implementations.
  - Requires 2.1.1. Test object creation in one path and reopen/invocation in
    the other, denial, inactive transactions, conflicts, restart, and unknown
    data preservation without importing runtime internals.
- [ ] 2.1.3. Package one minimal project template and both equivalent examples.
  - Requires 2.1.2. Reuse the runtime and application resources; do not ship a
    separate beginner framework or require an IDE to run the exercise.

### 2.2. Test onboarding before calling the exercise complete

Requires step 2.1. See terms of reference §7. Evidence may be a small observed
session and a written result; a telemetry platform is not needed.

- [ ] 2.2.1. Write and exercise the create, present, embed, inspect, and reopen
  tutorial for both paths, with one intentional capability-denial example.
  - Explain how the object retains identity, where resources and scenes live,
    and how to extend behaviour without bypassing the runtime.
- [ ] 2.2.2. Run the tutorial with an independent developer from the intended
  audience and repair blocking problems.
  - Requires 2.2.1 and H 3.1. Record completion steps, authored code, failure
    diagnoses, and understanding against the predeclared rubric. Include
    shared export and accessibility evidence, not only screen output.
  - Publish limitations and a continue or revise decision. H 3.3.3 owns the
    final hosted/direct-runtime and two-host integration gate.

## Deferred developer tooling

Visual resource construction, interactive previews beyond the fixture,
additional languages, sophisticated editor integration, and package ecosystems
remain candidates, not commitments. Admit one only when observed authoring
friction justifies it. Preserve the direct Rust and selected accessible routes
when improving the shared contract.
