# Cabochon roadmap

This is the programme map for Cabochon, not a promise to build the entire
architecture before delivering anything useful. The detailed roadmaps below
share delivery gates; they are not four projects to start in parallel.

The [technical design](cabochon-design.md) and accepted Architecture Decision
Records (ADRs) remain authoritative. This revision changes delivery order and
task size, not the hosted-first product boundary, the Graphics Environment
Manager (GEM)-derived application model, or the requirement for interoperable
accessible and direct Rust creation of objects.

## Starting point and planning rules

The inspected baseline is commit `195b3424a2521f98f20e6aee08dcbf3feb956951`.
The library still contains the generated greeting in `src/lib.rs`; design
coverage must not be mistaken for implemented platform capability. No delivery
gate below is recorded as passed.

Plan against limited implementation and review capacity. Keep one delivery
step and, where useful, one bounded enabling spike active. A new document does
not imply a new team. Only the next step should have an execution plan; later
steps describe bounded outcomes and must be rechecked against evidence before
work starts. There are no calendar or version promises here.

Every admitted step needs an owner, an explicit scope limit, a runnable
acceptance example, and a place to record evidence. Execution issues and plans
must link back to the file-qualified task identifier. Split a task again when
it cannot be reviewed as one coherent change. Record measured effort after
completed steps rather than manufacturing estimates from the architecture.

## Roadmap ownership

| Roadmap                                                     | Owns                                                                                                          | Does not own                                                                   |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| [Hosted runtime (H)](roadmap-hosted-runtime.md)             | Executable GEM substrate, object persistence, authority, currency proof, output conformance, and installation | A full widget catalogue, a compositor, or all application types                |
| [Developer experience (D)](roadmap-developer-experience.md) | Comparable authoring experiments, one accessible path, shared conformance, and onboarding                     | Three permanent language implementations or a general-purpose language project |
| [Knowledge workspace (W)](roadmap-knowledge-workspace.md)   | Notes, one useful rich embed first, focused editors, and evidence of individual value                         | Obsidian feature parity or a desktop-publishing suite                          |
| [Conditional extensions (E)](roadmap-extensions.md)         | Live tools, multi-provider mutation, and separately gated shell exploration                                   | Prerequisites for the first hosted application                                 |

*Table 1: Delivery ownership. H, D, W, and E qualify identifiers in this map.*

For example, H 2.2.1 means task 2.2.1 in the hosted-runtime roadmap. Within a
child roadmap, unqualified identifiers refer to that file. Tasks are the single
source of completion status; the gates below summarize acceptance, not a
second checklist to maintain.

## Delivery gates

| Gate                             | Runnable result                                                                                                                         | Prerequisites           | Explicitly not a claim of                                   |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------- |
| G1: Hosted substrate             | Resource-defined window, commands, keyboard access, and a shared scene on one host, without Enfilade                                    | H phase 1               | A complete toolkit or object runtime                        |
| G2: Persistent currency proof    | Rust provider, two presentations, document insertion, reopen, denial, and restart                                                       | G1 and H phase 2        | Completed accessible onboarding or the full output contract |
| G3: Canonical developer exercise | Accessible and Rust routes share object semantics; the currency document exercises the required output paths and both host environments | H phase 3 and D phase 2 | The personal knowledge product                              |
| G4: First useful workspace       | Repeated real work with linked notes and one rich embed, plus an evidence-based scope decision                                          | G2 and W phase 1        | Completion of the agreed three-editor proving ground        |
| G5: Hosted proving ground        | Notes, dataframe, Mermaid, and bitmap workflows are recoverable, exportable, installable, and useful                                    | G3 and W phases 2-3     | Automatic authorization to build a shell                    |

*Table 2: Acceptance gates, not release dates or implied version numbers.*

G1 and G2 are engineering increments. They deliberately do not satisfy every
requirement of the canonical first developer exercise. G3 retains that full
commitment, including ADR 004's accessible authoring path and ADRs 007-008's
application and graphics boundaries. An internal workspace experiment may
start after G2; it must not be advertised as the completed hosted product.

G4 can precede G3 when early workflow evidence is useful, but it does not
cancel either audience's requirements. G5 requires both. Live tools may follow
G4 once their specific source providers exist. Shell implementation requires
an explicit decision based on G5, not just completion of a renderer spike.

## What is next, and what can wait

The immediate delivery step is H 1.1, followed by H 1.2. Establish a runnable
hosted application before expanding the object framework. H phase 2 then makes
one durable object useful. D phase 1 begins once the currency contract fixture
exists; it is not a prerequisite for opening a window or testing persistence.

Choose representative user and developer evaluation tasks before building the
corresponding experience. W 1.1.1 may be prepared early as an enabling study;
it must not trigger construction of three editors in parallel.

The following are not early prerequisites: general multi-provider commit,
automatic live formula propagation, complete host-integration matrices,
additional languages, advanced text editing, arbitrary document types, and
Mullion. Each has a later owner or an explicit non-goal. Installation, basic
accessibility, capability checks, and recovery are not deferred until after
these features.

## Invariants and progressive implementation

A narrow implementation is allowed; a false guarantee is not. Each gate records
its supported scene operations, object operations, trust boundary, host
environment, and failure cases. Unsupported operations must fail explicitly.

- Clerestory, Escutcheon, Lapidary, and Burin are real boundaries from G1.
  Enfilade never becomes a private widget toolkit or renderer. Backend reuse is
  allowed behind those boundaries; it is not permission to bypass them.
- Every mutation is capability checked and transaction bound from its first
  implementation. Initially, a transaction may mutate only one provider's
  atomic storage domain. Unsupported cross-provider mutations are rejected
  before dispatch, not implemented as sequential best-effort writes.
- Identity, unknown data, and recoverable content survive persistence and
  export. Project exports never carry grants or host secrets. Access to host
  resources uses the supported portal boundary, not a development bypass.
- Keyboard interaction and accessible semantics accompany the first user
  interface (UI). Conformance expands with supported text and scene operations.
  All required currency output intents are exercised by G3; every new workspace
  feature extends that same contract rather than creating another renderer.

Use examples and contract tests for each delivered operation, property tests
for identity and authority invariants, and bounded state exploration for the
transaction and relationship protocols actually supported. Add high-risk
failure combinations when their components exist. Do not require the eventual
whole-platform Cartesian test matrix to merge the first application.

A gate review records the build and test evidence, supported limits, remaining
risks, user or developer observations where applicable, and a continue, narrow,
or stop decision. A failed value test is a reason to revise scope, not to add
more applications. New requirements enter a later gate unless they are needed
to make the current claim safe or true.

## Retained completed work

- [x] 1.1.2. Record the hosted-runtime and future-shell boundary in an ADR.
  - Historical task retained from the previous roadmap. [ADR
    002](adr-002-hosted-runtime-boundary.md) records the boundary; this does not
    mark its implementation complete.

## Allocation of the previous roadmap

Historical references in ADRs and discussions remain resolvable through this
map. Ranges include every task in the stated range. All previously unchecked
work remains uncompleted; relocation does not constitute implementation.

| Previous task identifiers | New owner                      | Treatment                                                                                        |
| ------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------ |
| 1.0.1-1.0.2               | H 1.1-1.2, H 3.1               | Minimum executable application and resource profile first; extend with the canonical exercise    |
| 1.0.3-1.0.4               | H 1.2, H 3.1                   | Scene and output tests grow together; full currency output evidence remains a G3 gate            |
| 1.1.1, 1.1.3              | H 2.1.1, H 1.1.3               | Record identity and introduce only exercised module or crate boundaries                          |
| 1.1.2                     | Retained above                 | Completed ADR, not completed runtime                                                             |
| 1.2.1                     | H 2.2.1                        | Denial and scoped grants precede provider dispatch                                               |
| 1.2.2                     | H 2.2.2, E phase 2             | Single-provider recovery first; multi-provider guarantees before admitting multi-provider writes |
| 1.2.3                     | H 2.2.3, D 2.1.2, E 2.2        | Grow a shared provider harness with the supported protocol                                       |
| 1.3.1-1.3.4               | D phase 1                      | Compare bounded prototypes against a working Rust baseline                                       |
| 2.1.1                     | H 2.1, H 3.2                   | Durable envelopes first; closure export and migration acceptance before G3                       |
| 2.1.2-2.1.3               | H 2.3, H 3.1, D phase 2        | Separate the engineering proof from the complete interoperable exercise                          |
| 2.2.1-2.2.3               | H 2.3.3, H 3.2-3.3, D 2.2      | Inspection and failure handling arrive with the first object                                     |
| 3.1.1-3.1.3               | W 1.1-1.2                      | Bound notes, identity-preserving embeds, and supported undo                                      |
| 3.2.1                     | W 2.1                          | Focused dataframe increment, not an entire spreadsheet                                           |
| 3.2.2                     | W 1.2.3                        | First proposed rich embed is source-edited Mermaid                                               |
| 3.2.3                     | W 2.2                          | Bounded bitmap editing after the first value test                                                |
| 3.2.4                     | W 3.1                          | Add combinations as each provider becomes real                                                   |
| 3.3.1-3.3.3               | W 1.1.1, W 1.3, W 3.2          | Define evaluation early; evaluate after one embed and again after the full proving ground        |
| 4.1.1-4.1.2               | E 1.1                          | Model visible live states before enabling refresh                                                |
| 4.2.1-4.2.4               | E 1.2                          | Two existing value sources and manual refresh before broader propagation                         |
| 5.1.1-5.1.4               | H 1.1.1, H 2.2.3, H 3.3, W 3.1 | Launch, activation, portals, packaging, and recovery move forward                                |
| 5.2.1-5.2.2               | W 3.2, E 3.1.1                 | Evidence precedes a separate accept, defer, or reject shell decision                             |
| 6.1.1-6.1.2               | E 3.1                          | Conditional shell exploration, not an automatic next phase                                       |
| 6.2.1-6.2.2               | E 3.2                          | Individual-benefit gate retained; organization-only work remains deferred                        |

*Table 3: Migration of existing obligations into the delivery roadmaps.*
