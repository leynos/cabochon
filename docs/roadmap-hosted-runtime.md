# Hosted-runtime roadmap

This roadmap delivers the executable foundation and the canonical currency
exercise. [The programme map](roadmap.md) defines gates G1-G3. References to D,
W, or E identify the other roadmaps linked there. Numeric dependencies without
a prefix refer to this file. Only phase 1 is the immediate delivery horizon.

The authority is [technical design](cabochon-design.md) §§3-13 and Architecture
Decision Records (ADRs) 002, 003, 007, and 008. A supported subset is an
implementation profile, not a replacement for those contracts. Later tasks must
be split further when the chosen adapter or storage implementation makes them
too large to review.

## 1. Open a real Cabochon application without Enfilade

Idea: a small resource-defined application can prove the Graphics Environment
Manager (GEM)-derived boundary before an object graph, language runtime, or
document suite exists.

Exit G1: a reproducible launch in one named Linux host environment opens the
application, handles commands and keyboard focus, exposes accessible roles,
and renders a scene without linking to Enfilade. A headless render checks the
same scene geometry. No claim of a complete toolkit is made.

### 1.1. Establish a runnable implementation profile

This step bounds the platform choices with one executable experiment rather
than inventing interfaces for every eventual component. See design §§3-6.

- [ ] 1.1.1. Record the first host environment and reproducible launch fixture.
  - Name the host version, display assumptions, supported architecture, build
    command, smoke-test command, and initial build and memory measurements.
  - Provide a development launcher; system installation is not required yet.
- [ ] 1.1.2. Spike one hosted window and rendering adapter behind the Cabochon
  boundaries, then record the choice and limitations.
  - Exercise text, a shape, input, and an offscreen target. Reuse existing
    implementation primitives rather than starting a compositor or GPU engine.
  - This chooses a hosted implementation, not the final Mullion renderer.
- [ ] 1.1.3. Establish only the modules or crates exercised by that application.
  - Requires 1.1.2. Record ownership and reuse in the design and repository
    layout before implementation; add a dependency-boundary test.
  - Existing formatting, lint, and test entrypoints exercise the result. Do
    not create placeholder crates for every box in the architecture diagram.

### 1.2. Deliver the smallest application and graphics contract

Requires step 1.1. The learning loop is a running application and a scene
fixture, not a complete application-framework specification. See design
§§8.1-8.4 and 13.1.

- [ ] 1.2.1. Load a window's title, strings, menu, and command metadata from
  Escutcheon resources and route commands through Clerestory.
  - Include keyboard traversal, focus, accessible names and roles, and a
    diagnostic for malformed resources. Defer a widget catalogue and designer.
- [ ] 1.2.2. Implement the scene operations needed by the fixture through
  Lapidary and Burin.
  - Bound the profile to rectangles, text runs, clipping, transforms, units,
    and hit regions. Reuse text shaping; do not invent an ASCII-only text API.
  - Tests preserve scene data and align visible and hit-test geometry. The
    initial damage region may be the entire window if declared explicitly.
- [ ] 1.2.3. Render the same fixture interactively and to an offscreen raster.
  - Compare geometry and text placement within recorded tolerances; test
    resize, repaint, clipping, and a renderer failure diagnostic.
  - No PDF, print service, semantic graph, or authoring-language comparison is
    needed to accept G1. Those obligations remain below.

## 2. Keep one currency object alive across presentations and restart

Idea: one durable, inspectable object can expose incorrect identity, authority,
and recovery assumptions before generalizing the runtime.

Requires G1. Exit G2: the Rust currency fixture has two presentations, can be
inserted into a minimal document and reopened, and survives the documented
failure cases. This is an engineering proof, not completed onboarding.

### 2.1. Persist one object safely

See design §§7.1 and 10. Settle the durable decisions as they become executable.

- [ ] 2.1.1. Record an identity ADR and executable envelope fixtures.
  - Cover lifetime, revision, duplicate versus derive, presentation identity,
    and deletion/reference behaviour. Separate identity from payload storage.
- [ ] 2.1.2. Compare the smallest viable storage adapter against reopen,
  unknown-field, unknown-type, and interrupted-write fixtures; record a choice.
  - Requires 2.1.1. Use one local store; do not add synchronization or multiple
    interchangeable production databases.
- [ ] 2.1.3. Implement envelope and provider-payload persistence for that store.
  - Requires 2.1.2. Round-trip generated fixtures without changing identity or
    dropping unknown data. Corrupt payloads leave recoverable siblings intact.

### 2.2. Admit only mutations with guarantees the runtime can uphold

Requires step 2.1. See design §§7.2, 7.4, 8.6-8.8, and 12-13, and ADR 003.

- [ ] 2.2.1. Implement selector descriptors and capability-checked dispatch.
  - Test target, selector, subject, access mode, expiry, and delegation scope.
    Missing, expired, incompatible, or over-broad authority must fail before
    the provider records any invocation. Discovery grants no authority.
- [ ] 2.2.2. Implement transaction-bound mutation in one provider's atomic
  storage domain, with revision checks and undo metadata.
  - Requires 2.2.1. Reject missing, unknown, or inactive transaction identifiers
    before provider dispatch. Model commit, abort, interruption, and restart;
    inject faults around durable writes. Committed or previous state is
    observable, never a fabricated success. Unsupported multi-provider writes
    fail before dispatch; E phase 2 owns their eventual protocol.
- [ ] 2.2.3. Add the minimal per-user runtime process and provider test harness.
  - Requires 2.2.2. Compare an in-process fixture with versioned local
    inter-process calls; preserve identifiers across both paths.
  - Test activation, shutdown, reconnect, provider absence, denied dispatch,
    and restart. Record supported trust boundaries without claiming that a
    protocol test alone proves containment of a malicious native process.

### 2.3. Complete the engineering currency loop

Requires step 2.2. See design §§9.1 and 11. Keep values deterministic; the
teaching example does not need exchange-rate services or accounting features.

- [ ] 2.3.1. Implement the Rust currency provider with base-currency identity,
  fixed conversion fixtures, and two presentations through Lapidary and Burin.
  - Both presentations name the same object. Mutations use the dispatch path,
    not privileged fixture-only access.
- [ ] 2.3.2. Insert an existing currency object into a minimal resource-defined
  document host and reopen it with its presentation choice intact.
  - Requires 2.3.1. Creating an object and inserting a reference may be
    separate explicit operations; do not imply atomic multi-provider creation.
- [ ] 2.3.3. Add a narrow inspector and failure exercises to that document.
  - Requires 2.3.2. Show identity, revision, provider, selectors, presentation,
    and grant decision. Test missing provider, malformed payload, denied
    operation, revision conflict, and restart without losing committed content.

## 3. Complete the canonical exercise and make it installable

Idea: the small object loop should survive independent authors, real output
intents, and host boundaries before the workspace grows on top of it.

Requires G2. Exit G3 also requires D phase 2. The accepted accessible-authoring,
shared-rendering, and hosted-runtime commitments are not waived by the earlier
engineering demonstrations.

### 3.1. Complete the currency application and output profile

See design §§9.1 and 13.1 and ADRs 007-008. Extend the proven scene, not a
second renderer or an unbounded graphics feature set.

- [ ] 3.1.1. Add PDF-oriented export and scaled thumbnails for the same currency
  scene, including page units and the metadata needed to compare outputs.
  - Geometry, text placement, and clipping agree within declared tolerances.
- [ ] 3.1.2. Add print-oriented document preparation and clipboard geometry for
  the embedded currency scene.
  - Requires 3.1.1. Validate a deterministic print artefact and supported
    portal handoff; a hardware printer fleet is not an acceptance requirement.
- [ ] 3.1.3. Add accessible text and geometry extraction and verify scene hit
  regions against visual, clipboard, and accessibility geometry.
  - Requires 3.1.2. No extraction target may invent independent layout.
- [ ] 3.1.4. Add path, image, and text conformance fixtures for the profile.
  - Requires 3.1.3. Exercise the reused shaping backend with bidirectional
    text, font fallback, and grapheme-aware caret geometry. Record supported
    limits rather than implementing a new text engine.
- [ ] 3.1.5. Extend generated-scene tests across all delivered output targets.
  - Requires 3.1.4. Scenes round-trip and agree across interactive, PDF,
    raster, thumbnail, print, clipboard, accessibility, and hit-test outputs.
    Incremental damage tests detect stale pixels and out-of-damage drawing.
- [ ] 3.1.6. Complete the currency host's resource-defined application
  furniture.
  - Reuse Clerestory commands for open, save, export, print, and inspection.
    Cover needed dialogs, icons, shortcuts, localization hooks, and keyboard
    and accessibility metadata in the versioned Escutcheon resource profile.
  - This is the canonical application's exercised subset, not a full toolkit.

### 3.2. Protect a user's saved work

See design §§10-14. These are gates for accepting real project data, not polish
that follows implementation of all applications.

- [ ] 3.2.1. Export and import a complete owned project closure, including
  opaque objects and reachable links, without grants or host secrets.
  - Reopen the imported project with fresh authorization. Provider absence
    never authorizes deletion or flattening of an embed.
- [ ] 3.2.2. Implement one schema upgrade fixture with rollback-safe payload
  replacement and a documented backup and recovery procedure.
  - Requires 3.2.1. Interrupt the migration and runtime at each durable
    boundary; retain the last committed payload until migration commits.
- [ ] 3.2.3. Audit the supported trust boundary and bounded diagnostics.
  - Requires 3.2.2. Test malformed messages, denial, runtime/provider restart,
    and declared sandbox boundaries. Logs omit content and credentials; error
    fields and metric labels remain bounded. Unsupported provider execution
    modes are unavailable, not silently trusted.

### 3.3. Package and integrate the complete developer exercise

See design §§5-6, 8.10, and 9.1. Host integration is not dependent on live
tools.

- [ ] 3.3.1. Package application and runtime activation for the first host.
  - Requires step 3.2. Route supported file and print requests through portals;
    test cancellation and denial. Add no capture or notification feature
    without a workflow that needs it.
- [ ] 3.3.2. Add the second GNOME-like or KDE-like acceptance environment.
  - Requires 3.3.1. Test fresh install, first run, update, removal, and retained
    user data without altering host configuration. Exercise sandboxed and
    unsandboxed clients, supported portals, and provider upgrades.
- [ ] 3.3.3. Run the integrated currency exercise through both authoring paths.
  - Requires steps 3.1-3.2, 3.3.2, and D 2.2.2. Exercise hosted and direct
    runtime modes, provider restart, denial, reopen, and shared output evidence.
  - Record remaining limits and evidence for G3. A screenshot alone cannot
    establish object identity, cross-language interoperability, or recovery.
