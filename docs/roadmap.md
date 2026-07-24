# Cabochon roadmap

This roadmap translates `docs/cabochon-design.md` into an outcome-oriented
delivery sequence. It does not promise dates. Each phase carries a testable
GIST idea, each step is a workstream that tests part of that idea, and each
task is a review-sized execution unit.

The roadmap begins with contracts that would otherwise force rework. Later
phases deliver vertical slices: the currency object, the knowledge workspace,
cross-application embeds, and the formula tool. Architecture Decision Records
(ADRs) live under `docs/` when written.

## 1. Settle the contracts that make one object environment possible

Idea: if Cabochon settles identity, authority, transaction, and authoring-path
contracts before application work, each vertical slice can extend one object
environment instead of building a private framework.

### 1.1. Ratify stable identity and runtime boundaries

This step answers what remains stable when Cabochon runs under GNOME, KDE
Plasma, or its future shell. The outcome fixes crate ownership and prevents the
hosted runtime from becoming disposable scaffolding. See `cabochon-design.md`
§§3-6 and §14.

- [ ] 1.1.1. Record stable object identity and presentation separation in an
  ADR.
  - Define object lifetime, revision semantics, duplication, derivation, and
    presentation identity.
  - Success: the ADR resolves every decided identity item in
    `cabochon-design.md` §14.
- [x] 1.1.2. Record the hosted-runtime and future-shell boundary in an ADR.
  - Define which responsibilities remain in the runtime and which belong to a
    host or shell adapter.
  - Complete: [ADR 002](adr-002-hosted-runtime-boundary.md) keeps domain
    contracts independent of GNOME, KDE Plasma, and compositors.
- [ ] 1.1.3. Establish the workspace crate structure around those boundaries.
  - Requires 1.1.1 and 1.1.2.
  - Document every new abstraction's ownership and reuse policy in
    `cabochon-design.md` and `repository-layout.md` before implementation.
  - Success: `make check-fmt`, `make lint`, and `make test` exercise the same
    workspace structure used by later slices.

### 1.2. Decide discovery, authority, and mutation contracts

This step answers whether a tool can be discoverable without receiving ambient
authority and whether cross-object mutations can recover from provider failure.
Its outcome gates every tool-aware workflow. See `cabochon-design.md` §§4, 6.2,
6.4, 7.2-7.4, and 11-12.

- [ ] 1.2.1. Validate selector discovery and capability grants.
  - Requires 1.1.1.
  - Define grant scope, expiry, delegation, denial, and audit fields.
  - Start from [ADR 003](adr-003-runtime-security-boundary.md), which defines
    grant, denial, transaction, audit, portal, and export invariants.
  - Success: generated capability cases demonstrate that invocation authority
    never exceeds the presented grant.
- [ ] 1.2.2. Specify the transaction state machine and recovery contract.
  - Requires 1.1.1.
  - Model prepare, commit, abort, provider crash, and runtime restart.
  - Success: bounded exploration finds no execution that exposes a partially
    committed object set.
- [ ] 1.2.3. Implement contract-test harnesses for object providers.
  - Requires 1.2.1 and 1.2.2.
  - Cover descriptor discovery, selector invocation, capability denial,
    revision conflict, transaction recovery, and unknown-field preservation.
  - Success: later providers can prove compliance without importing runtime
    internals.

### 1.3. Compare developer authoring paths

This step answers which route gives Python- or JavaScript-aware developers a
rewarding first result while preserving a direct Rust path. The result is an
ADR, not three permanent runtimes. See `cabochon-design.md` §§6.1, 8.1, and 14.

- [ ] 1.3.1. Define one language-neutral currency-object contract fixture.
  - Requires 1.1.1 and 1.2.1.
  - Specify identity, base currency, presentations, selectors, and document
    insertion without committing to syntax.
  - Success: every candidate authoring path targets the same fixture.
- [ ] 1.3.2. Prototype the currency object through direct Rust and Objective
  Rust-shaped selector declarations.
  - Requires 1.3.1.
  - Measure authored concepts, steps to first presentation, diagnostics, and
    access to lower-level capabilities.
- [ ] 1.3.3. Prototype the same object through one trait-oriented dynamic
  language surface.
  - Requires 1.3.1.
  - Preserve the same identifiers and selector contracts as the Rust route.
  - Success: the dynamic object interoperates with the Rust provider fixture.
- [ ] 1.3.4. Select the initial authoring path in an ADR.
  - Requires 1.3.2 and 1.3.3.
  - Success: the decision records first-result evidence, rejected alternatives,
    and the route from accessible authoring to direct Rust.

## 2. Prove the object model with the currency exercise

Idea: if one currency object can retain identity across multiple presentations
and enter a document through both accessible and Rust authoring paths, the
developer model is coherent enough to support richer documents.

### 2.1. Deliver a persistent currency object end to end

This step answers whether identity, provider activation, persistence, and
presentation contracts form a usable loop. See `cabochon-design.md` §§6-9.

- [ ] 2.1.1. Implement object envelopes, revisions, and provider-owned payload
  storage.
  - Requires steps 1.1-1.3.
  - Preserve unknown fields and unknown object types through round trips.
  - Success: property-generated export/import cycles retain object identity and
    reachable links.
- [ ] 2.1.2. Implement the currency provider and two currency presentations.
  - Requires 2.1.1.
  - Use the authoring path selected by 1.3.4 and provide the equivalent direct
    Rust route.
  - Success: both routes produce objects satisfying the same provider contract.
- [ ] 2.1.3. Implement a minimal document host that inserts and reopens a
  currency embed.
  - Requires 2.1.2.
  - Success: reopening retains object identity and presentation choice rather
    than importing a rendered copy.

### 2.2. Make the first interaction inspectable and recoverable

This step answers whether the canonical exercise teaches the next layer rather
than hiding it. It also tests failure behaviour before richer providers arrive.
See `cabochon-design.md` §§8.1, 10, and 13.

- [ ] 2.2.1. Expose object identity, selectors, presentations, and capabilities
  in a developer inspector.
  - Requires 2.1.3.
  - Success: the inspector traces a currency embed back to its provider and
    declared selector contract.
- [ ] 2.2.2. Add bounded diagnostics for missing providers, denied
  capabilities, malformed payloads, and revision conflicts.
  - Requires 2.1.3.
  - Success: each failure preserves the last committed object and explains the
    unavailable operation.
- [ ] 2.2.3. Add an end-to-end currency exercise for hosted and direct-runtime
  modes.
  - Requires 2.2.1 and 2.2.2.
  - Exercise accessible authoring, direct Rust, provider restart, capability
    denial, and document reopen combinations.
  - Success: both modes expose the same object identity and outcome.

## 3. Deliver the personal knowledge proving ground

Idea: if notes can contain editable dataframes, Mermaid diagrams, and bitmap
objects without flattening them, Cabochon can provide useful individual value
before becoming a desktop environment.

### 3.1. Deliver interconnected notes with semantic embeds

This step answers whether documents can own structure while embedded objects
retain independent identity. The result informs editor and project boundaries.
See `cabochon-design.md` §§6.1, 6.3, 8.2, and 9.

- [ ] 3.1.1. Implement note documents, links, and the project tree.
  - Requires phase 2.
  - Preserve link identity across note renames and moves.
  - Success: a project exports, imports, and reopens with its note graph intact.
- [ ] 3.1.2. Implement the common embed envelope and presentation placeholder.
  - Requires 3.1.1.
  - Handle available, opaque, unavailable, stale, and malformed object states.
  - Success: provider removal never deletes or silently flattens an embed.
- [ ] 3.1.3. Add document editing transactions and undo across note and embed
  operations.
  - Requires 3.1.2.
  - Success: insertion, removal, move, and presentation change undo without
    changing the embedded object's identity.

### 3.2. Add focused dataframe, diagram, and bitmap editors

This step answers whether separate applications can edit one document's rich
objects through shared contracts rather than private file formats. See
`cabochon-design.md` §§5, 7.5, and 8.2.

- [ ] 3.2.1. Implement a bounded dataframe provider and focused editor.
  - Requires 3.1.2.
  - Cover typed scalar cells, rows, columns, selections, and document embeds.
  - Success: the same dataframe opens in the note and focused editor without
    conversion.
- [ ] 3.2.2. Implement a Mermaid source provider and focused logical-diagram
  editor.
  - Requires 3.1.2.
  - Preserve source text, render diagnostics, and semantic object identity.
  - Success: valid diagrams pass `make nixie`; invalid diagrams remain editable
    and produce source-located errors.
- [ ] 3.2.3. Implement a bitmap provider and focused paint editor.
  - Requires 3.1.2.
  - Bound initial operations to those needed to prove editable embedding.
  - Success: edits update every presentation of the same bitmap object.
- [ ] 3.2.4. Add cross-application end-to-end and combinatorial coverage.
  - Requires 3.2.1, 3.2.2, and 3.2.3.
  - Cover editor availability, provider restart, project reopen, undo, host
    desktop mode, and opaque-object combinations.
  - Success: no covered transition changes identity or silently drops content.

### 3.3. Validate individual value against representative knowledge work

This step answers whether the proving ground is useful rather than merely
architecturally interesting. Its evidence decides whether later slices deepen
the workspace or revisit the product premise. See `terms-of-reference.md` §§5-7
and `cabochon-design.md` §8.2.

- [ ] 3.3.1. Define representative note, dataframe, diagram, and bitmap
  workflows for evaluation.
  - Requires step 3.2.
  - Compare semantic continuity, application switching, and recovery with an
    established Obsidian workflow.
- [ ] 3.3.2. Instrument task completion, context switches, stale embeds, and
  provider failures without collecting document content.
  - Requires 3.3.1.
  - Success: evaluation data answers whether Cabochon's continuity claim holds.
- [ ] 3.3.3. Record the proving-ground outcome and scope decision.
  - Requires 3.3.2.
  - Success: an ADR accepts, narrows, or rejects the next vertical slice based
    on observed individual value.

## 4. Prove tool-aware cross-document behaviour

Idea: if a formula tool can operate at the point of use over values in notes,
dataframes, and diagrams without ambient authority, Cabochon's tool-aware model
offers value that an application-local plugin system cannot provide.

### 4.1. Settle live-relationship semantics

This step answers what users see when sources update, move, disappear, or lose
authorization. It closes the largest unresolved user contract before live data
ships. See `cabochon-design.md` §§6.3, 10, and 14.

- [ ] 4.1.1. Validate and refine the live-relationship state machine.
  - Requires 3.3.3.
  - Define current, refreshing, stale, broken, and denied states.
  - Treat recoverability as a transition back to current that preserves the
    last successful result, not as a standalone state.
  - Start from [ADR 005](adr-005-explicit-live-relationship-states.md).
  - Success: every source-change and failure transition refines the accepted
    safety states into one visible result.
- [ ] 4.1.2. Model refresh, move, denial, provider crash, and recovery
  sequences.
  - Requires 4.1.1.
  - Success: bounded exploration finds no silent transition from a valid value
    to an invented or unauthorized value.

### 4.2. Deliver formula discovery and live evaluation

This step answers whether selector-based applicability can produce a useful
point-of-use tool across object types. See `cabochon-design.md` §§6.2-6.4, 7.3,
and 8.3.

- [ ] 4.2.1. Define addressable scalar and currency selector contracts.
  - Requires 4.1.1.
  - Apply the contracts to currency objects, dataframe cells, note entities,
    and diagram elements.
- [ ] 4.2.2. Implement contextual tool queries and formula-tool presentation.
  - Requires 4.2.1.
  - Success: compatible selections expose the formula tool without a global
    services menu; incompatible selections do not.
- [ ] 4.2.3. Implement live formula evaluation through capability-checked
  transactions.
  - Requires 4.1.2 and 4.2.2.
  - Success: source updates follow the accepted state machine and denied reads
    reveal no value.
- [ ] 4.2.4. Add end-to-end capability, failure, and application-switching
  coverage.
  - Requires 4.2.3.
  - Exercise source type, host application, grant state, source move, provider
    crash, and runtime restart combinations.
  - Success: high-risk denial, crash, and restart triples preserve the security
    and transaction invariants.

## 5. Package the hosted runtime for real desktop use

Idea: if Cabochon installs, activates, and behaves like a good citizen under
GNOME and KDE Plasma, users can adopt its document environment before deciding
whether to adopt its desktop.

### 5.1. Integrate with host desktop boundaries

This step answers whether the runtime can rely on normal host facilities
without diluting Cabochon's object contracts. See `cabochon-design.md` §§4-5,
7.2, 10, and 13.

- [ ] 5.1.1. Implement per-user runtime activation, shutdown, and crash
  recovery.
  - Requires phase 4.
  - Success: applications reconnect after restart without losing committed
    objects or exposing prepared transactions.
- [ ] 5.1.2. Route supported host-resource requests through XDG portals.
  - Requires 5.1.1.
  - Cover file access, notifications, printing, and capture only when required
    by the proving-ground workflows.
- [ ] 5.1.3. Package Cabochon applications and runtime for isolated GNOME-like
  and KDE-like acceptance environments.
  - Requires 5.1.2.
  - Success: install, first run, update, and removal leave the host desktop's
    configuration intact.
- [ ] 5.1.4. Add hosted-environment end-to-end coverage.
  - Requires 5.1.3.
  - Exercise sandboxed and unsandboxed applications, portal denial, runtime
    restart, and provider upgrade combinations.

### 5.2. Establish the adoption decision

This step answers whether the hosted product has earned further investment in a
full desktop shell. See `terms-of-reference.md` §§6-8 and `cabochon-design.md`
§§8 and 14.

- [ ] 5.2.1. Define user-facing, operational, and strategic acceptance
  thresholds from hosted-product evidence.
  - Requires 5.1.4.
  - Keep individual benefit as the scope gate.
- [ ] 5.2.2. Record whether Cabochon proceeds to a full desktop shell.
  - Requires 5.2.1.
  - Success: an ADR accepts, defers, or rejects shell work without changing the
    hosted runtime contract.

## 6. Evaluate deferred extensions after the hosted promise

Idea: if the hosted runtime already provides trustworthy individual value,
Cabochon can evaluate broader work on evidence instead of letting desktop scope
consume the proving ground.

### 6.1. Evaluate a full Cabochon desktop

This step begins only if 5.2.2 accepts shell work. It tests whether compositor
ownership materially improves the document and tool experience. See
`cabochon-design.md` §§5 and 14.

- [ ] 6.1.1. Decide the compositor, renderer, and toolkit boundaries through
  ADR-backed spikes.
  - Requires 5.2.2 to accept shell work.
  - Compare reuse of the hosted runtime, protocol coverage, accessibility, and
    maintenance burden.
- [ ] 6.1.2. Deliver one shell-hosted knowledge-workspace acceptance slice.
  - Requires 6.1.1.
  - Success: the shell adds measurable user value without forking object,
    capability, or transaction contracts.

### 6.2. Evaluate broader document types and organization-only features

These items stay deferred until the agreed individual product works. See
`terms-of-reference.md` §6.2 and `cabochon-design.md` §2.2.

- [ ] 6.2.1. Rank additional document types by direct individual benefit.
  - Requires phase 5.
  - Reject candidates that do not strengthen semantic continuity.
- [ ] 6.2.2. Reconsider organization-only policy or fleet capabilities only
  after an individual use case exists.
  - Requires phase 5.
  - Success: no enterprise capability enters scope solely because an
    organization requested it.
