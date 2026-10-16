# Conditional-extensions roadmap

These are separately admitted extensions, not a fourth project to start now.
[The programme map](roadmap.md) defines the evidence gates and H, D, and W
identifiers. Completion of one extension does not automatically authorize the
others. Each phase needs its own scope review and execution plan before work.

The authority remains [technical design](cabochon-design.md) §§7, 9.3, 12-15,
[ADR 003](adr-003-runtime-security-boundary.md), [ADR
005](adr-005-explicit-live-relationship-states.md), and the individual-benefit
gate in the [terms of reference](terms-of-reference.md).

## 1. Add one useful cross-document live tool

Idea: a narrowly scoped formula can test point-of-use discovery and live
relationships without first implementing general cross-provider mutation.

Entry: G4 supports the workflow, G3's runtime is available, and W 2.1 supplies
the dataframe source. Start with currency objects and dataframe cells, not
addressable scalars in every note and diagram. Exit: a formula works across
those two existing types, exposes failure state, and never reads without
current authority. Other source types require a separate supported increment.

### 1.1. Validate the smallest live-relationship state machine

See design §7.3 and ADR 005. Scope the model before writing refresh machinery.

- [ ] 1.1.1. Specify current, refreshing, stale, broken, and denied states for
  one formula and its persisted source references and revisions.
  - Include manual refresh, source move, deletion, denial, provider failure,
    and recovery. Retain the last successful result under its access policy;
    retention is not permission to reveal a value after access is denied.
- [ ] 1.1.2. Explore the bounded transition model and publish counterexamples
  or evidence for the chosen protocol.
  - Requires 1.1.1. No transition may invent a result, silently change a source
    identity, or expose an unauthorized value. Record bounds and assumptions.

### 1.2. Deliver manual evaluation before automatic propagation

Requires step 1.1. See design §§7.2-7.4 and 9.3.

- [ ] 1.2.1. Define scalar and currency selector fixtures for the two source
  types and expose the formula command on compatible selections.
  - Applicability queries remain separate from authority. Incompatible
    selections do not offer an operation they cannot execute.
- [ ] 1.2.2. Implement deterministic, manually refreshed formula evaluation.
  - Requires 1.2.1. Read sources with current grants and recorded revisions;
    commit the formula result through one provider's transaction. Detect
    changed inputs rather than claiming a coherent snapshot of unrelated reads.
  - Bound expressions and units. Defer custom functions, cyclic dependencies,
    large dependency graphs, and multi-provider write effects.
- [ ] 1.2.3. Add automatic refresh for the same bounded, acyclic relationships.
  - Requires 1.2.2. Test source updates and moves, revocation, provider crash,
    and restart, including high-risk combinations. Preserve last successful
    state under the applicable access policy and expose staleness explicitly.
- [ ] 1.2.4. Evaluate the workflow and admit additional source types only when
  the same capability and state contracts are demonstrably reusable.
  - Requires 1.2.3. Note entities and diagram elements are later increments,
    not hidden prerequisites for the first formula. Record the scope decision.

## 2. Support multi-provider mutation only for an admitted workflow

Idea: a concrete compound edit can justify the substantial cost of a durable
coordinator. A read-only-source formula does not itself justify that cost.

Entry: G3 and an accepted workflow that cannot be expressed safely using the
existing atomic domain. This phase is not a prerequisite for phase 1's
single-provider result commits. Until accepted, cross-provider writes remain
unsupported and are rejected before any participant mutates.

### 2.1. Establish the recovery protocol before provider implementation

See design §§7.4, 8.8, and 13 and ADR 003.

- [ ] 2.1.1. Specify participant eligibility, revision validation, prepare,
  commit, abort, durable decisions, and restart recovery for two providers.
  - Include capabilities, undo information, provider disappearance, and the
    reader-visible state. Reject participants that cannot meet the protocol.
- [ ] 2.1.2. Explore denial, conflict, crash, and restart interleavings in a
  bounded model and record its assumptions and implementation obligations.
  - Requires 2.1.1. No execution may expose a partially committed object set.
    Ambiguous recovery stops implementation until the protocol is narrowed or
    a necessary proof obligation is discharged; testing is not an excuse to
    claim unsupported atomicity.

### 2.2. Implement and integrate one compound operation

Requires step 2.1. Split storage and recovery changes into review-sized units
in the execution plan; this heading is not a single implementation task.

- [ ] 2.2.1. Implement the coordinator's durable decision record and recovery
  reader against two synthetic participants.
  - Mirror the model's state transitions and fault-inject durable boundaries.
- [ ] 2.2.2. Add prepare, commit, abort, and recovery adapters for the
  workflow's two real providers, with the shared provider harness.
  - Requires 2.2.1. Unknown, incompatible, or unavailable participants cannot
    enter an operation that the runtime presents as atomic.
- [ ] 2.2.3. Integrate the single admitted compound edit and its undo policy.
  - Requires 2.2.2. Exercise denial, crash, restart, and conflict over local
    inter-process boundaries. Show either a committed result or an explicit
    unavailable/recovery state; never label partial writes as success.

## 3. Evaluate a shell and other scope extensions independently

Idea: hosted success should buy an informed choice, not an obligation to finish
a desktop environment. Organization-only demand does not override individual
value. See design §15 and terms of reference §§6-8.

### 3.1. Decide whether Mullion adds value the host cannot provide

Entry: G5 and its recorded product evidence. Formula completion is required
only when the proposed shell use case depends on it.

- [ ] 3.1.1. Record an accept, defer, or reject decision for a specific shell
  experiment, with a user-visible improvement and maintenance budget.
  - A deferred or rejected decision is a valid outcome. Hosted contracts and
    support must remain useful without Mullion.
- [ ] 3.1.2. If accepted, compare compositor and GPU implementation options in
  bounded spikes and record an ADR before choosing production dependencies.
  - Requires 3.1.1 to accept the experiment. Measure runtime reuse, protocol
    coverage, accessibility, recovery, and maintenance burden.
- [ ] 3.1.3. Deliver one shell-hosted acceptance workflow using the unchanged
  application, graphics, object, capability, and transaction contracts.
  - Requires 3.1.2. Demonstrate the predeclared improvement over hosted mode.
    Failure returns work to the hosted roadmap; it does not authorize a
    rewrite of every layer or a general desktop feature backlog.

### 3.2. Admit additional scope only through an individual use case

Entry: G5. This step is independent of the shell decision.

- [ ] 3.2.1. Rank proposed additional document types by observed individual
  benefit, semantic continuity, and implementation and maintenance cost.
  - Keep them outside the initial proving ground unless a scope decision
    explicitly admits one with its own acceptance slice.
- [ ] 3.2.2. Reconsider organization-only policy or fleet features only after
  identifying a direct individual use case and recording a scope decision.
  - Organizational demand alone is insufficient. No organizational feature
    becomes an implicit dependency of hosted use or exploratory development.
