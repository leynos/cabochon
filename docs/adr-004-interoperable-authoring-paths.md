# Architectural decision record (ADR) 004: Require interoperable authoring paths

## Status

Accepted on 2026-07-23. Cabochon will support direct Rust and at least one more
accessible authoring path over the same object and selector contracts.

## Date

2026-07-23.

## Context and problem statement

Cabochon's developer audience includes people familiar with Python or
JavaScript who may not know Rust and experienced systems developers who need
direct access. Choosing a syntax before prototyping could optimize the wrong
experience; allowing separate object systems would destroy interoperability.

## Decision drivers

- Produce a rewarding first visible result without requiring Rust knowledge.
- Preserve direct Rust access without treating it as a second-class escape.
- Keep identity, selectors, capabilities, and presentations language-neutral.
- Select the accessible language from comparable implementation evidence.

## Decision statement

In the context of serving both accessible and systems-level application
authors, facing different experience and control needs, we decided for direct
Rust plus an evidence-selected accessible path over one provider contract, and
against Rust-only authoring, a disconnected beginner runtime, or prematurely
selecting Objective Rust or a dynamic language, to achieve conceptual
continuity and interoperability, accepting the cost of bindings, fixtures, and
multiple authoring tools.

## Options considered

- Require Rust for all providers. Rejected because it excludes part of the
  stated developer audience before they can obtain a first result.
- Create a simplified, separate beginner object system. Rejected because
  objects and skills would not transfer to direct Rust development.
- Select Objective Rust or a dynamic language from prose. Rejected because the
  branch contains prior art and hypotheses, not comparative prototype evidence.

## Decision outcome

Every authoring path targets the same language-neutral currency-object fixture
and the same identity, selector, capability, transaction, and presentation
semantics. Direct Rust remains supported. Objective Rust-shaped declarations
and one trait-oriented dynamic-language route will be prototyped before a later
ADR selects the initial accessible implementation.

## Supporting evidence

- `terms-of-reference.md` §§2, 4, 5, and 8 define both developer experience
  levels and the shared-object assumption.
- `cabochon-design.md` §§2, 4, and 9.1 require interoperable accessible and
  direct Rust implementations.
- Étoilé LanguageKit supplies prior art for several languages sharing an object
  model; it does not determine Cabochon's language choice.
- `roadmap.md` §1.3 defines comparable fixtures, prototypes, and measurements.

## Consequences

- Language bindings cannot redefine object identity or selector semantics.
- The exact accessible language and syntax remain unresolved until spikes
  provide first-result, diagnostics, continuity, and interoperability evidence.
- Supporting two paths adds tooling and test cost to the initial developer
  exercise.
