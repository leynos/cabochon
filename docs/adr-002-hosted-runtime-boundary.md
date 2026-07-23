# Architectural decision record (ADR) 002: Keep the hosted runtime independent of the shell

## Status

Accepted on 2026-07-23. One hexagonal runtime contract will serve applications
hosted by another desktop and applications hosted by a future Cabochon shell.

## Date

2026-07-23.

## Context and problem statement

Cabochon must own semantic objects, selector dispatch, capabilities,
transactions, and persistence without taking over display composition. If those
contracts depend on GNOME, KDE Plasma, a compositor, renderer, storage engine,
or authoring language, the hosted product becomes throwaway scaffolding.

## Decision drivers

- Preserve semantic identity and behaviour across hosting environments.
- Keep infrastructure replaceable through explicit ports and adapters.
- Use host security and display facilities instead of duplicating them.
- Allow a future shell to add integration without creating another object
  system.

## Decision statement

In the context of running Cabochon under existing desktops and a possible
future shell, facing incompatible host and compositor facilities, we decided
for a per-user runtime with inward-facing domain contracts and host-specific
adapters, and against embedding the object model in applications, coupling it
to one host, or creating a second shell runtime, to achieve portable semantic
contracts, accepting explicit adapter and service-boundary complexity.

## Options considered

- Put the object model in each application. Rejected because identity,
  authority, and transactions would fragment across process boundaries.
- Bind the runtime to one desktop or compositor. Rejected because hosted
  adoption and later shell reuse require different infrastructure adapters.
- Build a separate runtime for the future shell. Rejected because applications
  and stored objects would acquire environment-specific semantics.

## Decision outcome

The runtime is a per-user session service and owns no top-level windows in
hosted mode. Domain contracts define object identity, selector dispatch,
capabilities, transactions, and persistence. Applications, renderers, storage,
portals, host desktops, and a future shell connect through adapters and do not
become domain dependencies.

## Supporting evidence

- `cabochon-design.md` §§3-7 define the trust boundary, topology, and component
  responsibilities.
- Wayland assigns rendering to clients and composition to the compositor; it
  does not provide Cabochon's semantic object model.
- XDG Desktop Portal provides a host-mediated boundary for supported resources.
- `terms-of-reference.md` §§1-3 require useful hosted applications before a
  desktop replacement proposition.

## Consequences

- Runtime ports use domain language and domain types, not host-specific types.
- Host, portal, rendering, storage, and shell implementations remain adapters.
- Crate extraction must preserve inward dependencies and prohibit adapters from
  coordinating directly around the runtime.
