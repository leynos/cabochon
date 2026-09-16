# Architectural decision record (ADR) 001: Adopt a runtime-first product scope

## Status

Accepted on 2026-07-23. Cabochon will prove its object environment as a hosted
runtime and application set before investing in a full desktop shell.

## Date

2026-07-23.

## Context and problem statement

Cabochon serves people doing document-centred work and developers extending
that environment. Requiring either audience to replace an existing desktop
before seeing value would combine product validation with the cost and risk of
building a compositor, shell, toolkit integration, and complete application
suite.

## Decision drivers

- Demonstrate individual user value before increasing adoption cost.
- Test one object and tool model with both primary audiences.
- Keep the first proving ground small enough to falsify the product thesis.
- Preserve a route to a complete Cabochon desktop if evidence supports it.

## Decision statement

In the context of validating Cabochon's document and developer environment,
facing the cost of requiring a new desktop before either audience sees value,
we decided for a hosted runtime proven by a currency object and personal
knowledge workspace, and against beginning with a full desktop, a
developer-only framework, or a broad application suite, to achieve early
evidence from one coherent product surface, accepting that shell-level
differentiation is deferred.

## Options considered

- Build the full Cabochon desktop first. Rejected because shell work would
  dominate the unproven object and workflow proposition.
- Ship only a developer framework. Rejected because it would not test the
  promised value to people doing document-centred work.
- Pursue many document types at once. Rejected because breadth would weaken the
  currency-object and knowledge-workspace proofs.

## Decision outcome

The hosted runtime is the first product surface. The currency object proves the
developer contract; the personal knowledge workspace proves individual value.
Desktop work begins only after explicit adoption thresholds are met.

## Supporting evidence

- `terms-of-reference.md` §§1, 2, 4, 6, and 8 define the two audiences, hosted
  boundary, proving ground, and non-goals.
- `cabochon-design.md` §§1, 2, and 9 map both proofs onto one runtime model.
- `roadmap.md` phases 2-5 deliver evidence before phase 6 evaluates a desktop.
- GNOME and KDE Plasma provide host environments in which the proposition can
  be tested without controlling the shell.

## Consequences

- Hosted behaviour is a production boundary, not disposable scaffolding.
- Desktop-specific choices remain deferred until the hosted adoption decision.
- Initial scope excludes organization-only controls and additional document
  types unless evidence changes the product boundary through a later ADR.
