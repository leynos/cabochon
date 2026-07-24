# Architectural decision record (ADR) 003: Separate discovery, authority, and mutation

## Status

Accepted on 2026-07-23. Tool discovery grants no authority, and every provider
mutation requires both a matching capability and an active transaction.

## Date

2026-07-23.

## Context and problem statement

Applications and providers need to discover applicable operations without
receiving ambient access to user data. Mutations may span providers and must
not escape partially after denial, conflict, crash, or restart. Host resources
and exports introduce separate authority boundaries.

## Decision drivers

- Apply least authority at selector and target scope.
- Reject unauthorized or non-transactional work before provider dispatch.
- Make mutation outcomes auditable and recoverable.
- Avoid transferring host secrets or runtime grants with project data.

## Decision statement

In the context of invoking discoverable tools over user-owned objects, facing
untrusted clients, providers, content, and failure, we decided for explicit
capability grants plus mandatory active transactions for mutations, and against
ambient process authority, discovery-as-consent, or best-effort partial writes,
to achieve least-authority atomic behaviour, accepting broker, coordinator, and
audit overhead on every invocation.

## Options considered

- Treat discovery or registration as authority. Rejected because applicability
  queries would silently widen access.
- Trust an application or provider process for the whole session. Rejected
  because process identity does not express selector, target, access mode, or
  expiry.
- Permit mutations without transactions or accept partial multi-object success.
  Rejected because denial and failure could expose unaudited state.

## Decision outcome

The runtime separates registry queries from authority grants. Grants identify
the subject, target scope, selectors, access mode, expiry, and any delegated
authority. Before dispatch, the runtime validates selector applicability, grant
compatibility, and, for a mutating selector, an active transaction identifier.
Missing, expired, incompatible, unknown, or inactive authority fails without a
provider call or mutation.

Supported host-resource access crosses XDG portals. Audit events contain stable
object, selector, provider, decision, transaction, and bounded error fields but
exclude raw content, credentials, and unbounded paths. Project exports exclude
capability grants and host secrets.

## Supporting evidence

- `cabochon-design.md` §§4, 6.2, 6.4, 7.2-7.4, and 9-13 define the dispatch,
  transaction, portal, export, audit, and verification contracts.
- XDG Desktop Portal establishes a host-mediated permission boundary for
  sandboxed desktop resources.
- The currency, embed, and formula slices require cross-object behaviour where
  discovery and mutation authority cannot safely be conflated.

## Consequences

- A discoverable tool may still be denied when invoked.
- Dispatch tests must prove rejected mutations never reach a provider.
- The detailed multi-provider recovery state machine remains a separate
  implementation gate, but it cannot weaken these invariants.
