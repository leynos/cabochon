# Architectural decision record (ADR) 006: Make the GEM substrate a production boundary

## Status

Proposed.

## Date

2026-07-25.

## Context and problem statement

ADR 001 accepts a hosted runtime before a full desktop shell. ADR 002 keeps the
runtime independent of any shell. These decisions defer compositor ownership,
but they do not require Cabochon to defer its application and graphics model.
Without a normative Clerestory, Lapidary, Burin, and Escutcheon boundary,
Cabochon risks becoming only an object graph and tool-discovery framework
hosted inside other desktops, abandoning the Graphics Environment Manager (GEM)
separation between the application environment and device-independent graphics
that motivates the project.

## Decision drivers

- Preserve the GEM-derived split between application environment (Clerestory)
  and device-independent graphics (Lapidary) as law, not white-paper colour.
- Prevent Enfilade from expanding into the widget toolkit, drawing model, or
  layout engine by default.
- Keep hosted-mode work reusable: contracts proven under GNOME and KDE Plasma
  must survive a future Mullion shell unchanged.
- Ensure document, print, and export literacy is exercised by the proving
  ground rather than deferred with the desktop shell.

## Decision statement

In the context of proving Cabochon inside GNOME and KDE Plasma before building
a full shell, facing the risk that Enfilade dominates the design, we decided
for Clerestory, Lapidary, Burin, and Escutcheon as production contracts from
the hosted proving ground, and against treating UI, resources, graphics,
rendering, printing, and export as replaceable adapter details, to preserve
Cabochon's GEM-derived architecture while still deferring Mullion.

## Options considered

- Treat the GEM substrate as deferred adapters behind the object runtime.
  Rejected because the hosted product would validate only Enfilade, and later
  desktop work would inherit no proven application or graphics contract.
- Defer the whole platform until Mullion exists. Rejected by ADR 001: shell
  work must not precede product evidence.
- Make the substrate contracts production boundaries now while deferring only
  compositor ownership. Chosen.

## Decision outcome

Hosted Cabochon applications use Clerestory, Escutcheon, Lapidary, and Burin as
the application contract. Mullion remains deferred until the adoption decision
in the roadmap. Deferring the desktop shell never means deferring the Cabochon
application model.

## Supporting evidence

- `cabochon-design.md` §§1, 3, and 8 define the substrate thesis, the
  GEM-derived invariants, and the component responsibilities.
- `references/cabochon-white-paper.md` §§4-6 give Clerestory, Lapidary,
  Burin, Escutcheon, Mullion, Portal, and Enfilade equal conceptual weight.
- `roadmap.md` step 1.0 establishes the substrate contracts before
  Enfilade-specific behaviour enters the vertical slices.

## Consequences

- Hosted Cabochon applications use Clerestory and Lapidary rather than
  arbitrary host-native UI and rendering contracts.
- Enfilade coordinates identity, selectors, capabilities, relationships, and
  transactions, but it does not own widgets, scenes, layout, or rendering.
- The roadmap must prove at least one application through resource-defined UI
  and device-independent rendering before declaring the object model successful.
- Mullion remains deferred until adoption evidence justifies a full shell.
