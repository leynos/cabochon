# Architectural decision record (ADR) 007: Keep Enfilade out of rendering authority

## Status

Accepted on 2026-08-04. Enfilade resolves presentation applicability, while
Cabochon display, print, export, thumbnail, clipboard, and accessibility
geometry cross the shared Lapidary and Burin boundary.

## Date

2026-07-25.

## Context and problem statement

Cabochon objects can expose multiple presentations, and live relationships can
derive values from other objects. Those capabilities need a stable graphics and
output boundary. If object providers define renderer-specific payloads or
toolkit-specific UI directly, Cabochon loses the GEM-derived separation between
semantic identity, application interaction, device-independent graphics, and
output devices.

## Decision drivers

- Keep presentations contract-testable rather than provider-private.
- Guarantee output parity: the same presentation must reach display, print,
  export, thumbnail, and accessibility targets through one scene model.
- Prevent the object graph from becoming an omni-runtime whose "presentation"
  means whatever the graph happens to produce.

## Decision statement

In the context of objects exposing multiple presentations through Enfilade,
facing the risk that providers smuggle renderer- or toolkit-specific state
through object descriptors, we decided that Enfilade may resolve object
identity, presentation applicability, selectors, capabilities, and
relationships, but presentations intended for Cabochon display, print, export,
thumbnailing, clipboard geometry, or accessibility geometry must cross through
Lapidary and Burin contracts, to preserve one device-independent output
boundary for every intent.

## Options considered

- Let providers ship private renderers for their presentations. Rejected
  because ordinary output intents would fragment across provider-specific code
  paths and become untestable as a platform contract.
- Route every presentation byte through Enfilade-defined payloads. Rejected
  because it makes the object graph the de facto drawing model, violating the
  GEM-derived invariants.
- Make Lapidary and Burin the mandatory crossing for Cabochon-rendered
  output while Enfilade resolves applicability. Chosen.

## Decision outcome

Enfilade resolves which presentation applies to an object and intent.
Clerestory, Lapidary, and Burin define how that presentation becomes an
interactive view, geometric scene, exported document, or print job.

## Supporting evidence

- `cabochon-design.md` §§3, 7.1, 8.3-8.4, 8.9, and 13.1 define the
  invariants, the presentation boundary, the scene and renderer contracts, and
  the conformance tests.
- ADR 006 establishes the substrate contracts this decision protects.

## Consequences

- Providers can offer presentation data, but they cannot require a private
  renderer for ordinary Cabochon output.
- Lapidary scenes become contract-testable artefacts.
- Burin can implement output parity tests across display, PDF, raster, and
  print-oriented targets.
- Accessibility geometry and hit testing follow the same scene model as
  visual output.
