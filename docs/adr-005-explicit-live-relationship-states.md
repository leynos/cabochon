# Architectural decision record (ADR) 005: Expose live-relationship state explicitly

## Status

Accepted on 2026-07-23. A live relationship persists its dependency, preserves
its last successful result, and exposes freshness or failure state explicitly.

## Date

2026-07-23.

## Context and problem statement

Live formulas and cross-document embeds derive presentations from other
objects. Sources can change, move, disappear, become unauthorized, or lose
their provider. Replacing a valid result silently, presenting stale data as
current, or deleting the relationship would violate user trust.

## Decision drivers

- Preserve semantic identity and dependency information across failures.
- Distinguish freshness from availability and authorization.
- Never invent, erase, or expose a value because refresh failed.
- Leave refresh scheduling adaptable to later implementation evidence.

## Decision statement

In the context of derived cross-object presentations, facing source changes,
denial, and provider failure, we decided for persisted dependencies with
explicit current, refreshing, stale, broken, and denied states plus the last
successful result, and against silent recomputation, flattened copies, deletion
on failure, or stale values presented as current, to achieve inspectable and
recoverable behaviour, accepting visible state complexity and retained cached
data.

## Options considered

- Recompute silently and show only the latest attempt. Rejected because failure
  could replace trustworthy data with an error or invented value.
- Flatten the derived value into the containing document. Rejected because the
  source identity and update relationship would be lost.
- Delete broken or denied relationships. Rejected because temporary failure or
  policy change must not destroy user-owned structure.
- Block all display until refresh succeeds. Rejected because the last known
  value remains useful when clearly marked stale.

## Decision outcome

A live relationship stores source object identifiers, selector, arguments, last
successful source revisions, and last successful result. Its visible state is
current, refreshing, stale, broken, or denied. Refresh failure preserves the
last successful result and changes state; it never presents that value as
current. Recovery revalidates source identity, authorization, and revisions
before returning to current.

The exact refresh triggers, retry timing, source-move resolution, and
transition matrix remain implementation decisions to validate against the
formula slice. They may refine these states but may not weaken the preservation
and visibility contract.

## Supporting evidence

- `cabochon-design.md` §§7.3, 9.3, 11, and 13 require persisted dependencies,
  explicit staleness, authorization checks, and failure verification.
- `terms-of-reference.md` §§2, 6, and 9 identify live cross-document behaviour
  as a visible product promise rather than a storage detail.
- `roadmap.md` §4.1 requires model exploration across refresh, move, denial,
  provider crash, and recovery.

## Consequences

- Presentations need visible, accessible state indicators.
- Storage retains dependency metadata and the last successful result.
- Model-based tests must cover every refined transition without silent
  corruption or unauthorized disclosure.
