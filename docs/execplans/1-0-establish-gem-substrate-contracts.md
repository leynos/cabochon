# Establish the GEM substrate contracts (roadmap 1.0)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan. Silence is not approval.

## Purpose / big picture

Cabochon is a document-centred application environment modelled on the Graphics
Environment Manager (GEM). GEM split the desktop into an Application
Environment Services (AES) layer for windows, menus, and dialogs, and a Virtual
Device Interface (VDI) for device-independent drawing. Cabochon keeps that
split under four names:

- Clerestory is the application environment (the AES analogue): windows,
  menus, commands, dialogs, focus, keyboard traversal, inspector panels,
  accessibility roles, and adaptation to a host desktop such as GNOME or KDE
  Plasma.
- Escutcheon is the resource system: textual, localizable descriptions of
  menus, strings, icons, command metadata, shortcuts, and dialog layout.
- Lapidary is the device-independent scene model (the VDI analogue): shapes,
  text, images, clipping, transforms, hit regions, invalidation, physical
  units, and output intent.
- Burin is the renderer: it turns one Lapidary scene into concrete output for
  every target, which this plan calls a workstation, following
  `docs/context.md`.

ADR 007 makes those four boundaries production contracts in hosted mode, before
any Enfilade (object graph) behaviour exists. This plan delivers roadmap step
1.0 and its four tasks, 1.0.1 to 1.0.4.

After this work, a developer can run a small application, `hello-window`,
inside an ordinary GNOME or KDE Plasma session. Its window, menu bar, commands,
shortcuts, strings, icon, About dialog, and inspector panel all come from
Escutcheon resource files rather than hand-built widgets. Its content is a
Lapidary scene that Burin draws on screen. The same scene can be exported, with
one command, to a PDF, a print-ready PDF, a PNG, a thumbnail, an accessibility
geometry description, clipboard payloads, and a hit-testing map. Automated
tests prove that every one of those outputs agrees with the scene's geometry,
text placement, clipping, and metadata within declared tolerances. Generated
scenes survive a save-and-load round trip unchanged. Redrawing after an edit
leaves no stale pixels and touches nothing outside the damaged region. None of
this depends on Enfilade.

Observable success, end to end:

1. `cargo run -p cabochon-hello-window -- run` opens a window titled
   "Hello, Cabochon" with a File, View, and Help menu bar. Ctrl+Q quits, F1
   opens the About dialog, Tab moves focus, and Orca or Accerciser sees a
   window, a menu bar, menu items, a dialog, and buttons with their names.
2. `cargo run -p cabochon-hello-window -- run --locale en-XA` shows the same
   user interface (UI) with pseudo-localized strings, without code changes.
3. `cargo run -p cabochon-hello-window -- export --out /tmp/hello` writes
   `content.pdf`, `content-print.pdf`, `content.png`, `thumbnail.png`,
   `accessibility.json`, `clipboard/`, and `hit-map.json`, all derived from one
   scene.
4. `make check-fmt`, `make lint`, and `make test` pass. `make kani` and
   `make verus` discharge the formal obligations listed in the verification
   plan. `make test-hosted` runs the hello-window application under a headless
   Wayland compositor and reports one window, one presented frame, and a
   published accessibility tree.

## Constraints

These are hard invariants. Violating any of them requires escalation, not a
workaround.

- C1, GEM-derived invariants. Every invariant in `docs/cabochon-design.md` §3
  holds: applications reach the desktop only through Clerestory, draw only
  through Lapidary scenes, render only through Burin, and take UI structure
  from Escutcheon. No substrate crate depends on, names, or anticipates an
  Enfilade type (ADR 007, ADR 008).
- C2, dependency direction. Dependencies point inwards. `cabochon-lapidary`
  depends on no other Cabochon crate. `cabochon-burin` and
  `cabochon-escutcheon` may depend only on `cabochon-lapidary`.
  `cabochon-clerestory` may depend only on `cabochon-lapidary` and
  `cabochon-escutcheon`. Only adapter and application crates may combine
  Clerestory with Burin. A contract test enforces this (see `OB-ARCH-DEPS`).
- C3, no ambient host coupling in contracts. Clerestory's domain types never
  mention winit, Wayland, X11, AccessKit, GTK, or Qt. Host types appear only in
  `cabochon-clerestory-hosted` (ADR 002).
- C4, integer geometry. Lapidary geometry uses integer English Metric Units
  (EMU) and fixed-point transforms. Floating-point arithmetic is permitted only
  in the named float-boundary modules of `cabochon-burin` and
  `cabochon-clerestory-hosted`. Each such function carries a reasoned
  `#[expect(clippy::float_arithmetic, reason = "...")]`, as the lint baseline in
  `docs/developers-guide.md` allows for genuine floating-point arithmetic.
- C5, repository lint and safety policy. `unsafe_code` stays `forbid`.
  `#[allow]` is never used. Every lint expectation is tightly scoped and
  reasoned. No `.expect()` or `.unwrap()` outside tests. No source file exceeds
  400 lines.
- C6, existing contracts. The root package `cabochon`, its binary, and
  `tests/cli_greeting.rs` keep working. The existing Makefile contract tests
  under `tests/` keep passing. In particular, the `test`, `lint-clippy`,
  `typecheck`, and `build` recipes gain no extra Cargo invocations, and
  `default-members` is not introduced.
- C7, no persisted Enfilade or document format. This plan defines a Lapidary
  scene serialization (version 1) and an Escutcheon resource schema (version
  1). Neither is a persisted user-document format, and neither may be presented
  as one.
- C8, no proof shortcuts. No `assume`, `admit`, `#[verifier::external_body]`,
  or trusted proof bodies are introduced to make a proof go through. An
  unproved theorem stays in this plan or the design documentation until it is
  proved.
- C9, tests do not mutate the process environment. Locale and display
  selection reach the application through explicit arguments or an injected
  `mockable::Env`, never through direct `std::env` access, which `clippy.toml`
  already forbids.
- C10, library instrumentation only. Library crates emit `tracing` events and
  `metrics` measurements but never install a subscriber or recorder. Only the
  `cabochon-hello-window` binary installs them.

## Tolerances (exception triggers)

These thresholds define the limits of autonomous action. Reaching one means
stop, record the situation in `Decision log`, and wait for direction.

- Scope: a milestone needs more than 3,500 net added lines of Rust (excluding
  test fixtures, fonts, and generated snapshots) or touches more than 70 files.
- Interface: a contract described in `Interfaces and dependencies` must change
  shape in a way that alters a downstream roadmap task (2.1.2 or 2.1.3).
- Dependencies: a dependency not listed in `Interfaces and dependencies` is
  needed, or a listed crate must move to a new major or pre-1.0 minor series.
- Parity: a measured cross-target disagreement exceeds a declared tolerance
  after three focused fix attempts. Tolerances are never loosened silently;
  loosening one is a decision that needs approval.
- Proof: a Verus obligation remains unproved after two working days of proof
  effort, or appears to need an `assume`, `admit`, or external body. Use the
  documented fallback (Kani or property testing) only after approval.
- Hosted environment: a headless Wayland compositor cannot be provisioned in
  continuous integration (CI) or locally, so `make test-hosted` cannot run.
- Upstream conflict: implementation evidence contradicts
  `docs/cabochon-design.md`, an ADR, or the roadmap. Set `Status: BLOCKED` and
  record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: the third-party rendering crates are all pre-1.0 (`vello_cpu` 0.3,
  `krilla` 0.8, `parley` 0.12, `hayro` 0.8) and churn roughly quarterly.
  Severity: medium. Likelihood: high. Mitigation: confine each crate to one
  Burin adapter module behind a Burin-owned trait, commit `Cargo.lock`, and
  make parity tests the upgrade gate.
- Risk: font-stack version skew. `krilla` pins `skrifa` 0.42 while
  `parley` and `vello_cpu` use a newer `skrifa`, so two font stacks get
  compiled. Severity: low. Likelihood: high. Mitigation: pass fonts between
  stacks only as bytes plus glyph identifiers; accept duplicate compilation and
  record the cost.
- Risk: anti-aliasing differences make raster-to-PDF pixel comparison flaky.
  Severity: medium. Likelihood: medium. Mitigation: compare geometry rather
  than pixels wherever possible. Pixel comparisons use coverage masks with
  declared intersection-over-union thresholds and exclude a one-pixel edge band.
- Risk: damage-clipped re-rendering in `vello_cpu` may not be bit-identical
  to a full re-render at damage-rectangle edges. Severity: medium. Likelihood:
  medium. Mitigation: snap damage outwards to whole device pixels and test
  bit-exactness first. If it fails, escalate with the measured difference
  before adopting a per-channel tolerance.
- Risk: Verus proofs over the hit-test and damage kernels prove harder than
  sketched, especially transform bounds, which need non-linear arithmetic.
  Severity: medium. Likelihood: medium. Mitigation: keep the proved kernels
  small and integer-only. Each obligation names a fallback (Kani bounded
  checking plus property tests) that needs approval before use.
- Risk: the Verus mirror drifts from the production kernel. Severity: high.
  Likelihood: medium. Mitigation: a drift-guard test compares marked kernel
  regions token by token (see `OB-PROOF-MIRROR`).
- Risk: `winit` 0.31 is in beta and changes the event loop API. Severity: low.
  Likelihood: high. Mitigation: use stable `winit` 0.30.13 and keep all `winit`
  code inside one adapter crate.
- Risk: CI runners may lack a headless Wayland compositor. Severity: medium.
  Likelihood: medium. Mitigation: install `weston` in a dedicated job and run
  `make test-hosted` there. Locally, `weston` comes from Extra Packages for
  Enterprise Linux (EPEL) on Rocky Linux 10.
- Risk: in-window menus and dialogs look less native than GTK or Qt menus.
  Severity: low. Likelihood: high. Mitigation: this is a deliberate GEM-style
  choice recorded in an ADR. Host adaptation covers window decorations, scale,
  input, and accessibility, which are what keep the application recognizable
  inside the host (`docs/terms-of-reference.md`).
- Risk: the workspace conversion collides with roadmap task 1.1.3, which owns
  "the workspace crate structure". Severity: medium. Likelihood: medium.
  Mitigation: this plan creates only the substrate crates that the design
  already names. 1.1.3 keeps ownership of the Enfilade and runtime crates.
  `Decision log` entry D1 asks the reviewer to confirm this split.
- Risk: bundled fonts add binary weight and licence obligations. Severity:
  low. Likelihood: certain. Mitigation: bundle only Open Font Licence (OFL)
  fonts, ship their licence text, and record checksums.

## Progress

- [x] (2026-10-18) Renamed the branch to
  `1-0-establish-gem-substrate-contracts`.
- [x] (2026-10-18) Reconnaissance: Wyvern agents surveyed the build and test
  infrastructure, the substrate requirements in the white paper, terms of
  reference, and ADRs, and the testing and documentation conventions.
- [x] (2026-10-18) External research with Firecrawl: rendering, text, PDF,
  windowing, accessibility, resource formats, localization, testing crates, and
  Verus tooling.
- [x] (2026-10-18) Drafted this ExecPlan.
- [ ] Community-of-experts design review and revision.
- [ ] Reviewer approval of the plan.
- [ ] EP-M0: workspace, tooling, and architecture guards.
- [ ] EP-M1: Lapidary scene model and serialization.
- [ ] EP-M2: Lapidary resolution, hit testing, and invalidation, with proofs.
- [ ] EP-M3: Burin raster-family workstations.
- [ ] EP-M4: Burin document-family workstations and cross-target parity.
- [ ] EP-M5: Escutcheon resource format and compiler.
- [ ] EP-M6: Clerestory application contract with the headless shell.
- [ ] EP-M7: hosted shell adapter and the hello-window application.
- [ ] EP-M8: documentation reconciliation and roadmap closure.

## Surprises & discoveries

- Observation: the white paper names GEM's AES, VDI, and `.RSC` resource
  files, but never VDI workstations, metafiles, or GEM's AES event calls.
  Evidence: Wyvern survey of `references/cabochon-white-paper.md`. Impact: the
  term "workstation" comes from `docs/context.md`, not the white paper.
  "Metafile" is not introduced; the serialized scene is called a "scene
  document".
- Observation: `krilla` supports PDF/A and PDF/UA but explicitly not PDF/X.
  Evidence: krilla source and documentation, October 2026. Impact:
  print-oriented output becomes a PDF/A-2b file with page boxes, an sRGB output
  intent, and a print ticket. PDF/X is deferred (decision D6).
- Observation: the repository's lint baseline denies `float_arithmetic` and
  the lossy cast lints, and `docs/developers-guide.md` already permits a
  reasoned `#[expect]` for genuine floating-point arithmetic. Evidence:
  `Cargo.toml` `[lints.clippy]`; `docs/developers-guide.md` "Lint baseline".
  Impact: integer geometry is the natural representation, and it also makes the
  Verus obligations tractable (decision D2).
- Observation: `docs/developers-guide.md` states that Cabochon is a single
  crate rather than a workspace. Evidence: "Lint baseline" section. Impact:
  EP-M0 must rewrite that paragraph and move the lint tables to
  `[workspace.lints]`.
- Observation: no Verus or Kani tooling exists in the repository. Kani is
  installed on the development host; Verus is not. Evidence: Wyvern survey;
  `command -v`. Impact: EP-M0 adds pinned prover tooling through
  `rust-prover-tools`.

## Decision log

Each decision below is proposed by the planning agent. Decisions D1, D6, D7,
D8, and D10 change or interpret upstream scope, and need explicit reviewer
acceptance with the plan.

- Decision D1: create the substrate crates now as Cargo workspace members
  under `crates/`, keeping the root package `cabochon`. Rationale: the design
  already names `cabochon-clerestory`, `cabochon-escutcheon`,
  `cabochon-lapidary`, and `cabochon-burin` (Figure 1), and constraint C2's
  dependency rule can only be enforced across crates. Roadmap 1.1.3 still owns
  the Enfilade, runtime, storage, and portal crates and the workspace-wide
  ownership policy. Alternative rejected: modules inside the root crate, which
  cannot stop `lapidary` from importing `clerestory`. Date/Author: 2026-10-18,
  planning agent. Needs acceptance.
- Decision D2: Lapidary measures length in integer EMU (`i64`, 914,400 per
  inch, 12,700 per point, 36,000 per millimetre, 9,525 per 96-dpi pixel) and
  transforms with a 16.16 fixed-point linear part plus an EMU translation.
  Rationale: every common unit converts exactly, scene round trips are exact,
  the lint baseline forbids float arithmetic, and Verus can reason about
  integers. Prior art: Office Open XML DrawingML (EMU) and FreeType's
  `FT_Matrix` (16.16). Date/Author: 2026-10-18, planning agent. To be recorded
  as ADR 009.
- Decision D3: Burin uses `vello_cpu` for raster output, `krilla` for PDF,
  `parley` and `fontique` for text layout, and `hayro` (test-only) to read PDF
  output back for parity checks. Rationale: these crates share the Linebender
  geometry and font stack. `krilla` writes tagged PDF and PDF/A. `hayro` can
  rasterize and interpret the PDFs that `krilla` writes. Alternative considered:
  `tiny-skia` (mature, but no text and a separate geometry stack). This is the
  hosted proving ground's software renderer; the deferred graphics processing
  unit (GPU) renderer decision in `docs/cabochon-design.md` §15 is unaffected.
  Date/Author: 2026-10-18, planning agent. To be recorded as ADR 011.
- Decision D4: Escutcheon resources use KDL 2.0 documents (the `kdl` crate)
  for structure, Project Fluent `.ftl` files for strings, and SVG files for
  icons, compiled into a typed, cross-referenced `ResourceBundle`. Rationale:
  KDL's node model of name, arguments, properties, and children fits menu and
  dialog trees, preserves comments and spans for diagnostics, and its
  specification is final. Fluent provides fallback chains, arguments,
  bidirectional isolation, and pseudo-localization. Alternatives considered:
  RON (concise, but weak diagnostics and formatting preservation), TOML
  (awkward for deep ordered trees), and Qt `.ui` or GtkBuilder XML
  (toolkit-specific). Date/Author: 2026-10-18, planning agent. To be recorded
  as ADR 010.
- Decision D5: Clerestory draws its own menu bar, menus, dialogs, and
  inspector panels inside the window as Lapidary scenes, and adapts to hosts
  through a `HostShell` port. The hosted adapter uses `winit` 0.30,
  `softbuffer`, and `accesskit_winit`. Rationale: GEM-style in-window furniture
  keeps one application contract for hosted and future Mullion modes
  (`docs/cabochon-design.md` §8.1). Native menus through `muda` need GTK on
  Linux and attach only to GTK windows. Date/Author: 2026-10-18, planning
  agent. To be recorded as ADR 012.
- Decision D6: "print-oriented output preparation" means a paginated PDF/A-2b
  file with explicit media, trim, and bleed boxes, an sRGB output intent, and a
  `PrintTicket` describing media, copies, and page range. Submitting jobs to
  the Common UNIX Printing System (CUPS) or the print portal remains with
  roadmap 5.1.2, and PDF/X is deferred. Rationale: `krilla` does not support
  PDF/X, and ADR 003 routes host printing through portals. Date/Author:
  2026-10-18, planning agent. Needs acceptance.
- Decision D7: Burin does not ship an SVG workstation in this step. Rationale:
  roadmap 1.0.4 omits SVG, and the design says "SVG where appropriate".
  Date/Author: 2026-10-18, planning agent. Needs acceptance.
- Decision D8: the minimum standard dialogs are alert, confirmation, and
  About, all resource-defined. File open and save dialogs are declared as
  host-delegated requests in the contract but implemented by the portal work in
  roadmap 5.1.2; the hello-window export command takes its output directory
  from the command line. Rationale: `docs/cabochon-design.md` §8.10 assigns
  file selection to the portal adapter, and 1.0 must not depend on it.
  Date/Author: 2026-10-18, planning agent. Needs acceptance.
- Decision D9: Lapidary owns resolution (flattening a scene into an ordered,
  integer display list with composed transforms, effective clips, and bounds),
  hit testing, and damage computation. Text shaping enters through a
  Lapidary-owned `TextLayout` port implemented by Burin's `parley` adapter.
  Every Burin workstation consumes the same `ResolvedScene`, so cross-target
  parity is structural and the tests check the backends. Date/Author:
  2026-10-18, planning agent.
- Decision D10: "physical units" on screen use a 96-dpi reference pixel scaled
  by the host's device scale, expressed as an integer in 1/120 units as in the
  Wayland `wp_fractional_scale_v1` protocol. Rationale: the host compositor
  does not report trustworthy physical dots per inch, and 1/120 steps represent
  every scale GNOME and KDE Plasma offer. Date/Author: 2026-10-18, planning
  agent. Needs acceptance.
- Decision D11: Verus proofs use a spec-mirror layout under `verus/`, with
  production kernel regions delimited by `// verus-mirror: begin <name>` and
  `// verus-mirror: end <name>` comments. A drift-guard test asserts that each
  mirrored region matches its production counterpart token for token.
  Rationale: Verus cannot import production modules, and a `#[path]` import
  would force Verus syntax into production code. Date/Author: 2026-10-18,
  planning agent.

## Outcomes & retrospective

Not started. Populate this section at each milestone boundary and at completion.

## Context and orientation

The repository is at `/home/leynos/.lody/repos/github---leynos---cabochon` in
the worktree used by this branch. Today it holds one Rust package, `cabochon`,
with `src/lib.rs` (a `greet()` function) and `src/main.rs` (which prints
`Hello from Cabochon!`). Everything else under `tests/` checks the build
standard: the Makefile, CI workflows, linker provisioning, and release routing.
There is no workspace, no substrate code, and no prover tooling.

The governing documents are:

- `docs/cabochon-design.md` (version 0.2, revised 2026-07-25), especially §3
  (GEM-derived invariants), §7.1 (presentations cross into Lapidary), §8.1 to
  §8.4 and §8.9 (component responsibilities), §11 (failure behaviour), §13.1
  (VDI conformance tests), and §14 (observability).
- `docs/adr-007-gem-substrate-boundary.md` and
  `docs/adr-008-enfilade-rendering-authority.md`, which make the substrate a
  production boundary and keep Enfilade out of rendering.
- `docs/adr-002-hosted-runtime-boundary.md`, which keeps host types out of
  domain contracts.
- `docs/terms-of-reference.md` lines 275 to 279, the GEM substrate success
  measure: a developer can create a small resource-defined application that
  opens a window, exposes standard menus and commands, renders a Lapidary
  scene, exports the same scene through Burin, and integrates with
  accessibility and keyboard traversal, without writing host-specific code.
- `docs/roadmap.md` step 1.0 and its tasks 1.0.1 to 1.0.4. Tasks 2.1.2 and
  2.1.3 consume this work.
- `docs/context.md`, which defines Burin, Clerestory, Escutcheon, Lapidary,
  Surface, and Workstation.

Build and quality gates live in `Makefile`. `make check-fmt` checks Rust and
Markdown formatting. `make lint` runs rustdoc, Clippy, and Whitaker (a Dylint
suite). `make test` runs `cargo nextest` (or `cargo test`) with
`--workspace --all-targets --all-features`, then the doctests. The Makefile
already passes `--workspace`, so new member crates join every gate
automatically. `.cargo/config.toml` selects the Cranelift backend, the parallel
frontend, and the pinned `mold` linker.

Terms used throughout this plan:

- EMU (English Metric Unit): an integer length; 914,400 EMU make one inch.
- Fixed16: a signed 16.16 fixed-point number stored in an `i32`, where 65,536
  represents 1.0.
- Scene: a Lapidary tree of groups, shapes, text runs, images, and hit
  regions, with page geometry and an output intent.
- Resolved scene: the ordered, flattened display list produced from a scene,
  in which every item carries its composed transform, effective clip, and
  integer bounds.
- Workstation: a Burin output target, such as interactive display, PDF,
  print, raster, thumbnail, accessibility extraction, clipboard, or hit-test
  map.
- Damage: the set of rectangles that must be redrawn after a scene changes.
- Host shell: the desktop that owns windows and input in hosted mode, such as
  GNOME or KDE Plasma.
- Headless shell: a Clerestory adapter that runs applications without a
  display, recording frames and accessibility trees for tests.
- Spec mirror: a Verus copy of a production function, proved correct and kept
  identical to the production code by a drift-guard test.

Skills and references to load while implementing: `execplans` (this plan),
`rust-router` (then `rust-types-and-apis`, `rust-errors`, `rust-state-machines`
for focus and menu modes, and `arch-crate-design`), `hexagonal-architecture`
(ports and adapters), `rust-unit-testing` (`rstest`, `googletest`,
`pretty_assertions`, `insta`), `proptest`, `kani`, `verus`, `rust-verification`,
`arch-decision-records` for ADRs 009 to 012, `en-gb-oxendict-style` for prose,
`codegraph-mcp` for navigation, and `firecrawl-mcp` for upstream documentation.
Repository references: `docs/rust-testing-with-rstest-fixtures.md`,
`docs/reliable-testing-in-rust-via-dependency-injection.md`,
`docs/rust-doctest-dry-guide.md`,
`docs/complexity-antipatterns-and-refactoring-strategies.md`,
`docs/scripting-standards.md` (only if a helper script becomes necessary), and
`docs/documentation-style-guide.md` (ADR template, captions, and screen-reader
descriptions).

## Conformance basis

Upstream artefacts, at repository commit `42ca1cd` on `main`:

- Terms of reference: `docs/terms-of-reference.md`, last changed in commit
  `f9907ff`. Traced item `TOR-SM-GEM` is the GEM substrate success measure at
  lines 275 to 279.
- Technical design: `docs/cabochon-design.md` version 0.2, last changed in
  commit `386346b`. Traced items:
  - `TDD-INV-1` to `TDD-INV-7`: the seven invariants of §3.
  - `TDD-CLR`, `TDD-ESC`, `TDD-LAP`, and `TDD-BUR`: §8.1 to §8.4.
  - `TDD-PRES`: §8.9, every target consumes the same scene and output
    contracts.
  - `TDD-FAIL-RENDER`: the "Renderer failure" row of Table 2 in §11.
  - `TDD-VDI-RT`, `TDD-VDI-PARITY`, `TDD-VDI-PROP`, `TDD-VDI-INVAL`,
    `TDD-VDI-TEXT`, and `TDD-VDI-FAIL`: the six bullets of §13.1.
  - `TDD-OBS`: §14.
- ADRs: ADR 002, ADR 007, and ADR 008, all accepted.
- Roadmap: `RM-1.0.1`, `RM-1.0.2`, `RM-1.0.3`, and `RM-1.0.4`.
- Governing standards: `AGENTS.md`, `docs/documentation-style-guide.md`, and
  the lint baseline in `Cargo.toml` and `docs/developers-guide.md`.

Trace chains (upstream, then milestone, then acceptance evidence):

```plaintext
RM-1.0.3 -> TDD-LAP, TDD-VDI-RT -> EP-M1 -> lapidary::tests::round_trip (proptest), OB-LAP-ROUNDTRIP
RM-1.0.3 -> TDD-LAP, TDD-VDI-INVAL -> EP-M2 -> verus/lapidary_damage.rs, OB-LAP-DAMAGE; burin invalidation suite, OB-BUR-INVAL
RM-1.0.3 -> TDD-LAP (hit regions) -> EP-M2 -> verus/lapidary_hit.rs, OB-LAP-HIT
RM-1.0.3, RM-1.0.4 -> TDD-BUR, TDD-PRES, TDD-VDI-PARITY, TDD-VDI-PROP -> EP-M3, EP-M4 -> burin parity suite, OB-BUR-PARITY-*
RM-1.0.4 -> TDD-FAIL-RENDER, TDD-VDI-FAIL -> EP-M3 -> burin failure suite and output.feature, OB-BUR-FAIL
RM-1.0.4 -> TDD-VDI-TEXT -> EP-M3, EP-M4 -> burin text parity suite, OB-BUR-PARITY-TEXT
RM-1.0.2 -> TDD-ESC, TDD-INV-4 -> EP-M5, EP-M7 -> escutcheon resources.feature; hello-window e2e, OB-ESC-*
RM-1.0.1 -> TDD-CLR, TDD-INV-1, TDD-INV-6 -> EP-M6, EP-M7 -> clerestory hello_window.feature; make test-hosted, OB-CLR-*
TOR-SM-GEM -> RM-1.0.1..4 -> EP-M7 -> hello-window e2e (run, export, locale), OB-CLR-HOST
TDD-INV-5, ADR-007, ADR-008 -> EP-M0 -> tests/substrate_dependency_contract.rs, OB-ARCH-DEPS
TDD-OBS -> EP-M3, EP-M5, EP-M6 -> tracing and metrics assertions in each crate's tests
```

Gaps between the baseline and the target, and where each closes:

- No workspace or substrate crates: closed by EP-M0.
- No scene model, units, or serialization: closed by EP-M1.
- No hit testing or invalidation: closed by EP-M2.
- No renderer, output targets, or parity evidence: closed by EP-M3 and EP-M4.
- No resource format: closed by EP-M5.
- No application contract or hosted adapter: closed by EP-M6 and EP-M7.
- No ADRs for units, resources, renderer, or menus: ADRs 009 to 012, drafted
  in EP-M0 as `Proposed` and accepted at the milestone that implements each.

## Verification plan

This section names every obligation before the implementation is designed
around it. Obligations are grouped by crate. For each one: the statement, the
method, why that method is proportionate, the domain, the artefact, the
evidence, and the non-vacuity argument.

### Where formal verification is warranted

Verus is used for four integer kernels in Lapidary whose correctness must hold
for scenes of any size, and which every later application relies on: rectangle
algebra (supporting lemmas), transform bounds, topmost hit testing, and damage
soundness. These are contractual: Clerestory routes pointer input through hit
testing, and Burin's interactive workstation trusts damage to decide what to
redraw. Kani covers bounded arithmetic safety (unit conversion, fixed-point
multiplication, and the float-boundary domain), where exhaustive symbolic
checking of every 64-bit input is both feasible and stronger than sampling.
Everything else uses example, parameterized, property, state-machine, snapshot,
behavioural, and end-to-end tests: resource validation, focus and menu state
machines, and cross-target parity are checked against real third-party
behaviour that no proof could model.

The relationship between proof and test is explicit. Verus establishes the
semantic properties of the mirrored kernels for all inputs. The drift-guard
test ties the mirror to the production code. Property tests then check that the
production kernel, called through the real public API, satisfies the same
properties on generated scenes, and that real backends honour the axioms the
proofs rely on (for example, `A-RASTER-LOCALITY`). Removing any one layer
leaves a named gap.

### Axioms (trusted external assumptions)

- `A-RASTER-LOCALITY`: `vello_cpu` writes non-background pixels only within a
  fill or stroke's geometric bounds dilated by one device pixel, and a
  pixel-aligned rectangular clip stops all writes outside it. Exercised by
  `OB-BUR-COVER`.
- `A-RASTER-DET`: `vello_cpu`'s single-threaded 32-bit floating-point (f32)
  pipeline is deterministic for identical input on one machine.
- `A-PDF-PRECISION`: `krilla` writes coordinates with at least 0.01 point
  precision, and `hayro` interprets the content streams `krilla` produces
  faithfully. Exercised by `OB-BUR-PARITY-GEOM`.
- `A-TEXT-DET`: `parley` with a fixed, explicitly registered font collection
  and system fonts disabled produces identical glyph identifiers and positions
  for identical input.
- `A-KDL`, `A-FLUENT`, and `A-USVG`: the `kdl`, `fluent-bundle`, and `usvg`
  parsers conform to their specifications for the constructs the schema uses.
- `A-WINIT`: `winit` and `softbuffer` deliver input events and present
  buffers as documented, and a headless `weston` session represents GNOME and
  KDE Plasma closely enough for a smoke test. The residual gap, real
  GNOME-specific and KDE-specific behaviour, closes in roadmap 5.1.3.
- `A-ACCESSKIT`: `accesskit_winit` publishes the supplied tree to the
  Assistive Technology Service Provider Interface (AT-SPI) faithfully.
- `A-VERUS`: the pinned Verus release and its Z3 solver are sound.

### Lapidary obligations

`OB-LAP-UNITS`, exact and checked unit conversion.

- Statement: for every `i64` input, each `Emu` constructor
  (`from_points`, `from_millimetres`, `from_inches`, `from_reference_pixels`)
  returns `Ok(e)` with `e` exactly equal to the input times the unit factor
  when that product lies within `Emu::LIMIT` (±2^47), and otherwise returns
  `Err(LengthOutOfRange)`. No panic or overflow occurs.
- Method: Kani proof harnesses, plus named `rstest` cases at the boundaries.
- Rationale: the domain is all of `i64`; Kani checks it exhaustively, which
  sampling cannot.
- Artefact: `crates/cabochon-lapidary/src/units/kani.rs` (behind
  `#[cfg(kani)]`) and `crates/cabochon-lapidary/src/units/tests.rs`.
- Evidence: `make kani` reports every harness `VERIFICATION:- SUCCESSFUL`.
  Red step: the harness is written first against a constructor that multiplies
  without checking, and Kani reports an arithmetic overflow.
- Non-vacuity: harnesses `kani::cover!` both the `Ok` and `Err` outcomes and
  the exact boundary values ±`Emu::LIMIT`. Seeded fault: replacing
  `checked_mul` with `wrapping_mul` must fail verification.

`OB-LAP-FIXED`, fixed-point application is overflow-free and monotone.

- Statement: `Fixed16::apply(c, e)` computes `floor(c × e / 65,536)` using an
  `i128` intermediate, never overflows for any `c` and any `e` within the EMU
  domain, and is monotone non-decreasing in `e` when `c ≥ 0` and non-increasing
  when `c < 0`.
- Method: Kani for overflow freedom and the floor identity; the monotonicity
  property is also used by the Verus transform-bounds proof, which re-proves it
  over the mirror.
- Artefact: `crates/cabochon-lapidary/src/geometry/fixed/kani.rs`.
- Evidence: `make kani` success. Red step: a 64-bit intermediate overflows.
- Non-vacuity: `kani::cover!` for negative, zero, and positive coefficients
  and for results that round down. Seeded fault: rounding towards zero instead
  of down must break the floor identity for negative products.

`OB-LAP-RECT`, rectangle algebra (supporting lemmas).

- Statement: rectangles are half-open (`min ≤ p < max`) and normalized. For
  all rectangles `a` and `b` and points `p`: `intersect(a, b)` contains `p`
  exactly when both `a` and `b` contain `p`; `union_bounds(a, b)` contains both
  `a` and `b`; containment is transitive; an empty rectangle contains no point.
- Method: Verus lemmas over the mirror.
- Rationale: the hit and damage proofs depend on these facts for all inputs.
- Artefact: `verus/lapidary_rect.rs`.
- Evidence: `make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs` reports
  zero errors.
- Non-vacuity: each lemma has an inhabited antecedent; a proof-level witness
  (`assert(exists |p| rect_contains(r, p))` for a concrete non-empty `r`) shows
  the predicates are satisfiable. Seeded fault: making `rect_contains`
  inclusive of `max` must break the intersection lemma.

`OB-LAP-XFORM-BOUNDS`, transformed bounds are sound.

- Statement: for every rectangle `r` and transform `t`,
  `transform_bounds(t, r)` contains `apply(t, p)` for every point `p` in `r`.
  The function computes the minimum and maximum of the four transformed
  corners, then widens the maximum by one EMU to keep the result half-open.
- Lemma sketch: (1) `Fixed16::apply` is monotone in its argument with
  direction given by the coefficient's sign (from `OB-LAP-FIXED`); (2)
  therefore each output coordinate, a sum of two monotone terms plus a
  translation, attains its extremes over a rectangle at corners; (3) floor
  rounding preserves the ordering, so every interior image lies between the
  rounded corner extremes.
- Method: Verus. Fallback, which needs approval: Kani over coordinates bounded
  to ±2^20 plus a property test over the full domain.
- Artefact: `verus/lapidary_transform.rs`.
- Evidence: `make verus` passes. Red step: the lemma is stated before the
  widening step exists, and Verus reports the failing postcondition at the
  half-open boundary.
- Non-vacuity: witnesses for rotation-like transforms (mixed-sign
  coefficients). Seeded fault: using only two corners (the axis-aligned case)
  must fail.

`OB-LAP-HIT`, topmost hit testing.

- Statement: for a resolved hit list `h` (in painting order) and a device
  point `p`, `hit_test(h, p)` returns `Some(i)` exactly when `i` is the
  greatest index with `covers(h[i], p)`, and `None` exactly when no entry covers
  `p`. `covers(e, p)` holds when `p` lies in the entry's effective clip and
  inside its polygon by the non-zero winding rule, computed with exact `i128`
  cross products. The bounding-box prefilter is sound: if `covers(e, p)` then
  `p` lies in `bounds(e)`.
- Lemma sketch: (1) the reverse scan's loop invariant states that no entry
  after the cursor covers `p`; (2) the first covering entry found is therefore
  the greatest; (3) prefilter soundness: if `p` lies outside a polygon's vertex
  bounds, then no edge straddles `p`'s scanline to the right of `p`, so the
  crossing count is zero and the non-zero rule rejects `p`; (4) the effective
  clip is the intersection of ancestor clips (from `OB-LAP-RECT`).
- Method: Verus over the mirror for (1) to (4); a property test compares the
  production `hit_test` with a naive oracle (scan everything, keep the last
  hit) on generated scenes; named `rstest` cases cover nested clips,
  transforms, overlapping regions, and edges.
- Artefact: `verus/lapidary_hit.rs`;
  `crates/cabochon-lapidary/tests/hit_test_properties.rs`.
- Evidence: `make verus` passes; `make test` passes. Red step: the oracle
  property fails against a forward scan that returns the first hit.
- Non-vacuity: the generator's classification (via `prop_assert!` counters
  reported by the test) must show at least 20% of cases with two or more
  overlapping covering regions and at least 10% with clip-excluded hits. Seeded
  fault: dropping the clip check must fail both the proof and the oracle
  property.

`OB-LAP-DAMAGE`, damage soundness.

- Statement: for resolved scenes `old` and `new` and the damage `D` computed
  by `damage(old, new)`, every point `q` outside `D` is covered by the same
  sequence of paint items, in the same order, in both scenes. Paint at `q`
  therefore cannot differ.
- Algorithm: compare item sequences by stable node identifier and content
  from the front and from the back. The common prefix and suffix are unchanged.
  Every item in the old middle and in the new middle contributes its bounds to
  `D`.
- Lemma sketch: (1) filtering a concatenation equals concatenating the
  filters; (2) if every middle item's bounds lie inside `D`, then for `q`
  outside `D` no middle item covers `q`, so the filtered middles are empty; (3)
  prefix and suffix filters are equal because those items are equal; (4) hence
  the filtered sequences are equal.
- Method: Verus over the mirror; property tests over generated edit sequences
  (insert, remove, move, recolour, retransform) check identical scenes give
  empty damage and a single insertion damages exactly the inserted item's
  bounds.
- Artefact: `verus/lapidary_damage.rs`;
  `crates/cabochon-lapidary/tests/damage_properties.rs`.
- Evidence: `make verus` and `make test` pass. Red step: a damage function
  that reports only the new middle fails the proof for removals.
- Non-vacuity: edit generators must cover each edit kind, recorded by
  classification. Seeded fault: omitting removed items' bounds must fail.

`OB-LAP-ROUNDTRIP`, scene serialization round trip.

- Statement: for every valid scene `s`, `SceneDocument::decode(encode(s))`
  equals `s`; a document with an unknown major version is rejected with
  `SceneDecodeError::UnsupportedVersion`.
- Method: property test over generated scenes (depth up to 4, up to 24 nodes,
  all node kinds, nested clips and transforms, text runs with left-to-right and
  right-to-left scripts, images up to 16 by 16 pixels, every output intent);
  `insta` snapshot of one canonical scene document; named unhappy cases.
- Artefact: `crates/cabochon-lapidary/tests/round_trip.rs`.
- Non-vacuity: classification shows every node kind and output intent;
  seeded fault: skipping the hit-region tag during encoding must fail.

`OB-LAP-VALID`, scene validation. Validation rejects duplicate node
identifiers, dangling image references, coordinates outside `Emu::LIMIT`, and
degenerate transforms (zero determinant). Parameterized `rstest` unhappy-path
cases, plus a property test that every generated valid scene passes.

### Burin obligations

Declared tolerances, defined once as constants in
`crates/cabochon-burin/src/parity/tolerance.rs` and documented in the Burin
section of `docs/cabochon-design.md`:

- Vector targets (PDF and print): every path vertex, clip vertex, image
  placement, and glyph origin within 127 EMU (0.01 point) of the resolved scene.
- Raster targets (interactive, raster, thumbnail): item bounds within one
  device pixel at the render scale; filled-shape coverage masks with
  intersection over union of at least 0.98 for shapes at least 8 pixels across;
  glyph origins within one device pixel.
- Geometry-only targets (accessibility, clipboard geometry, hit map): exact
  EMU equality with the resolved scene, because no floating-point step is
  involved.
- Text content: exact Unicode equality per run between the scene, the
  accessibility extraction, and the PDF text extracted through `hayro`.
- Invalidation: bit-exact equality between an incrementally updated frame and
  a full re-render, and zero writes outside the snapped damage rectangles.
- Hit map: agreement with Lapidary's exact `hit_test` at every pixel centre
  more than one pixel (plus the curve-flattening tolerance of a quarter pixel)
  from any region or clip edge.

`OB-BUR-PARITY-GEOM` and `OB-BUR-PARITY-TEXT`, cross-target agreement. Every
workstation's output agrees with the shared `ResolvedScene` within the declared
tolerances. PDF and print output are read back through a `hayro-interpret`
`Device` implementation that records paths, clips, images, and glyph origins
with their transforms. Raster output is compared through coverage masks.
Method: a property test over generated scenes (the same generator as
`OB-LAP-ROUNDTRIP`, bounded to 12 nodes and depth 3), a fixed corpus of named
scenes (`rstest` with `#[files]`), and `insta` snapshots of a normalized,
target-independent summary per workstation (item kinds, rounded bounds, text,
and metadata), never raw bytes. Non-vacuity: classification must cover
clipping, rotated transforms, right-to-left text, images, and each output
intent; seeded fault: offsetting the PDF workstation's text by one point must
fail.

`OB-BUR-HITMAP`, the hit-testing workstation. The hit map lists each hit
region's device-space polygon and effective clip (exact) and renders an
identifier raster. Away from edges the identifier raster agrees with
`hit_test`. Property test plus named cases.

`OB-BUR-A11Y`, accessibility extraction. Every text run and every annotated
group yields one node whose bounds equal the resolved bounds exactly, whose
role comes from the annotation (or `Text` for runs), and whose children follow
painting order. Property test plus an `insta` snapshot of the hello-window
content extraction.

`OB-BUR-CLIP`, clipboard geometry. The clipboard workstation produces a
geometry descriptor with exact scene bounds, a PNG whose pixel size equals the
bounds at the requested resolution (within one pixel), and a PDF whose media
box equals the bounds within the vector tolerance. Named cases.

`OB-BUR-INVAL`, invalidation. For any generated scene and edit sequence, the
interactive workstation's incremental frame equals a full re-render bit for
bit, and a sentinel-filled frame shows no writes outside the snapped damage.
Property test over edit sequences of length up to 8.

`OB-BUR-COVER`, raster locality (exercises `A-RASTER-LOCALITY`). For any
generated scene, every non-background pixel lies inside the snapped,
one-pixel-dilated bounds of some resolved item. This connects the Verus damage
proof, stated over bounds, to real pixels. Property test.

`OB-BUR-FAIL`, renderer failure. Each failure class (unsupported feature,
missing font, undecodable image, resource limit exceeded, backend error) yields
a bounded `RenderError` with a low-cardinality class, leaves the input scene's
digest unchanged, and never substitutes a rasterized copy for text. The
fallback policy returns a diagnostic scene that names the failed item. Method:
named `rstest` cases with a fault-injecting workstation supplied by dependency
injection, plus the behavioural feature below.

`OB-BUR-PRINT`, print preparation. Print output carries the media, trim, and
bleed boxes from the scene's print intent, an sRGB output intent, and passes
`krilla`'s PDF/A-2b validator. Named cases read back through `hayro`.

`OB-BUR-DET`, determinism. Rendering the same scene twice with the same fonts
produces byte-identical PNG and PDF output. Named cases; if `krilla` proves
non-deterministic, record it in `Surprises & discoveries` and compare
normalized content instead.

### Escutcheon obligations

`OB-ESC-REFS`, referential integrity. A compiled bundle contains no dangling
reference: every menu item's command, every command's label and tooltip string,
every icon, and every dialog control's command or string resolves. Method:
property test that generates valid bundles, then seeds exactly one dangling
reference and asserts compilation fails with a diagnostic naming the reference
and its source span; named `rstest` cases; an `insta` snapshot of the rendered
diagnostics for a multi-error file.

`OB-ESC-SHORTCUT`, shortcut uniqueness and syntax. No two commands share a
shortcut within one scope, and `Shortcut::parse(s.to_string()) == s`. Property
tests.

`OB-ESC-ROUNDTRIP`, canonical emission. `compile(emit(bundle)) == bundle` for
generated bundles. Property test.

`OB-ESC-L10N`, localization. Lookup walks the requested locale chain to the
base locale; the base locale must define every referenced key; the pseudo-locale
`en-XA` transforms every string; missing message arguments are reported, not
silently dropped. Named cases plus the behavioural feature.

`OB-ESC-VERSION`, schema versioning. A bundle declaring an unknown schema
version is rejected with a located diagnostic. Named case.

### Clerestory obligations

`OB-CLR-FOCUS`, focus and traversal. In every window, at most one element owns
focus, and only a focusable, enabled, visible element can own it. Tab followed
by Shift+Tab returns focus to where it started. A full Tab cycle visits every
focusable element exactly once. Method: `proptest-state-machine` with a
reference model, over operation sequences of length up to 32 (Tab, Shift+Tab,
enable, disable, show, hide, open dialog, close dialog, pointer focus).

`OB-CLR-MODAL`, modality. While a modal dialog is open, neither focus nor input
reaches elements outside it, and closing it restores the previous focus owner.
Same state machine.

`OB-CLR-MENU`, menu navigation. Opening, arrow navigation, activation, and
Escape never leave a menu open while focus sits outside it, and activation of a
disabled item dispatches nothing. Same state machine.

`OB-CLR-ROUTE`, command routing. A command reaches the first handler in the
responder chain (focused element, dialog, window, application) that accepts it.
A disabled command is never dispatched. A shortcut and the corresponding menu
item produce identical dispatch records. Named `rstest` cases plus a property
test over generated chains.

`OB-CLR-A11Y`, accessibility tree. Every interactive element has a role and a
non-empty accessible name, parent and child links are mutually consistent, and
the focused element appears in the tree. Property test over generated resource
bundles; the hosted adapter's conversion is checked by feeding its `TreeUpdate`
into `accesskit_consumer::Tree`.

`OB-CLR-HOST`, hosted conformance. The same scripted session run under the
headless shell and under the hosted shell (inside headless `weston`) yields the
same accessibility tree and the same command log. End-to-end test run by
`make test-hosted`.

### Architecture and proof-hygiene obligations

`OB-ARCH-DEPS`, dependency direction. `tests/substrate_dependency_contract.rs`
reads `cargo metadata` and asserts the dependency rules of constraint C2, that
no substrate crate or `cabochon-hello-window` depends on any crate whose name
contains `enfilade`, `runtime`, or `portal`, and that only
`cabochon-clerestory-hosted` depends on `winit`, `softbuffer`, or
`accesskit_winit`. Non-vacuity: the test also asserts that each expected
dependency edge exists, so an empty metadata read cannot pass.

`OB-PROOF-MIRROR`, proof mirror drift guard. `tests/verus_mirror_contract.rs`
extracts every `// verus-mirror: begin <name>` region from `crates/` and
`verus/`, strips comments and whitespace, and asserts each pair is
token-identical, that every production region has a mirror, and that `verus/`
contains no `assume(`, `admit(`, or `external_body`. Non-vacuity: it asserts at
least four mirrored regions exist.

`OB-PROVER-TOOLING`, prover tooling contract. Following the Verus on-ramp,
`tests/prover_tooling_contract.rs` checks that `tools/verus/VERSION`,
`tools/verus/SHA256SUMS`, and `tools/rust-prover-tools/REF` are well formed,
that the `verus`, `install-verus`, and `kani` recipes delegate through
`$(PROVER_TOOLS)` (or `cargo kani`) and contain no `curl`, `unzip`, or
`sha256sum`, and, with a recording fake supplied as
`PROVER_TOOLS_CONTRACT_TEST=1 PROVER_TOOLS=<fake>` on the `make` command line,
that `make verus` issues exactly
`prover-tools verus run --repo-root . --proof-file verus/lapidary_proofs.rs`.

## Plan of work

The work runs in stages. Stage A, understanding and proposal, is this plan.
Each milestone then follows Red-Green-Refactor: write the smallest failing
test, behavioural scenario, Kani harness, or Verus lemma statement first, and
run it to watch it fail for the expected reason; write the smallest
implementation that turns it green; then refactor with the focused tests and
the full gates rerun. Verification artefacts are written with the
implementation, not after it. Every milestone ends with `make check-fmt`,
`make lint`, and `make test` passing (run sequentially, ideally through the
`scrutineer` agent), plus `make kani` and `make verus` where the milestone says
so, and a commit.

### EP-M0: workspace, tooling, and architecture guards

Convert `Cargo.toml` into a workspace whose root package remains `cabochon`. Add
`[workspace] members = ["crates/*"]` and `resolver = "3"`. Move the
`[lints.*]` tables to `[workspace.lints.*]` unchanged, and add
`[lints] workspace = true` to the root and every member. Add
`[workspace.dependencies]` for every dependency in
`Interfaces and dependencies`, with caret requirements. Create the seven member
crates with `//!` module documentation, `publish = false`, and empty public
surfaces: `cabochon-lapidary`, `cabochon-burin`, `cabochon-escutcheon`,
`cabochon-clerestory`, `cabochon-clerestory-hosted`, `cabochon-hello-window`,
and `cabochon-substrate-testkit`.

Add the prover tooling: `tools/verus/VERSION` (pin the latest weekly release
when EP-M0 starts, currently `0.2026.10.04.426d8b0`), `tools/verus/SHA256SUMS`,
`tools/verus/README.md`, and `tools/rust-prover-tools/REF`. Add the Makefile
targets `install-verus`, `verus`, and `kani`, following
`~/.claude/skills/verus/references/project-on-ramp.md`. `make verus` runs
`verus/lapidary_proofs.rs`, which in EP-M0 is an explicit `FORMAL-SKIP` stub
that fails loudly if invoked with `REQUIRE_PROOFS=1`. `make kani` runs
`cargo kani --workspace`, preceded by `check-build-tools`. Add
`make test-hosted` as a stub that fails with a clear "not yet implemented"
message until EP-M7. None of these targets joins `make all`, `make test`, or
`make lint` (constraint C6). Add the new targets to `.PHONY` and to the
held-out list in `tests/build_standard_support/make.rs` if the build-standard
tests require it.

Write the guards first: `tests/substrate_dependency_contract.rs`
(`OB-ARCH-DEPS`), `tests/prover_tooling_contract.rs` (`OB-PROVER-TOOLING`), and
`tests/verus_mirror_contract.rs` (`OB-PROOF-MIRROR`, which passes vacuously
only in EP-M0 and gains its "at least four regions" assertion in EP-M2). Each
fails before the workspace and tooling exist.

Add one `rstest-bdd` smoke scenario in `cabochon-substrate-testkit` to prove
the behavioural harness is wired, and one `#[gtest]` plus `#[rstest]` test to
prove `googletest` composes with `rstest` (with `#[gtest]` placed first).

Draft ADRs 009 to 012 with status `Proposed`, using the template in
`docs/documentation-style-guide.md`. Update `docs/developers-guide.md` ("Lint
baseline" now describes `[workspace.lints]`; new sections on the workspace,
test conventions, and the proof inventory), `docs/repository-layout.md` (the
`crates/`, `verus/`, and `tools/verus/` directories), and `docs/contents.md`.

### EP-M1: Lapidary scene model and serialization

Implement, in this order, each behind red tests: `units` (`Emu`, unit
constructors, `LengthOutOfRange`); `geometry` (`Point`, `Size`, `Rect`,
`Fixed16`, `Transform`, `Path` built from move, line, quadratic, cubic, and
close segments, `FillRule`); `paint` (`Colour` as sRGB with 16-bit channels and
alpha, `Paint`, `Stroke`); `text` (`TextRun`, `FontRequest`, `TextDirection`,
`LanguageTag`); `image` (`ImageResource` holding RGBA8 pixels, an intrinsic
physical size, and a content digest); `scene` (`Scene`, `Node`, `NodeId`,
`Group`, `Shape`, `TextNode`, `ImageNode`, `HitRegion`, `HitRegionId`,
`Annotation`, `SemanticRole`, `PageGeometry`, `OutputIntent`, `ColourIntent`,
`PrintIntent`); `validate` (`OB-LAP-VALID`); and `document` (`SceneDocument`
encode and decode, version 1, JSON through `serde_json`, with image pixels in
base64). Write the Kani harnesses for `OB-LAP-UNITS` and `OB-LAP-FIXED`
alongside the code they check. Put the scene generator (`proptest` strategies)
in `cabochon-substrate-testkit`, so Lapidary uses it only from its integration
tests under `crates/cabochon-lapidary/tests/`, never from unit tests (a
dev-dependency cycle would otherwise produce two incompatible copies of
Lapidary's types).

### EP-M2: Lapidary resolution, hit testing, and invalidation

Define the `TextLayout` port in `cabochon_lapidary::text::layout` and a
deterministic `MonospaceTextLayout` fake in the testkit (every grapheme
advances by a fixed amount). Implement `resolve(scene, layout)` producing a
`ResolvedScene`: an ordered `Vec<ResolvedItem>`, where each item carries its
`NodeId`, composed `Transform`, effective clip (a rectangle plus an optional
polygon), integer bounds, and paint payload; hit regions become device-space
polygons, and curves are flattened with integer de Casteljau subdivision to a
declared tolerance. Implement `hit_test`, `hit_test_all`, and `damage`. Place
the four proved kernels (rectangle algebra, transform bounds, hit scan with
prefilter, and prefix-suffix damage) in `crates/cabochon-lapidary/src/kernel/`,
each delimited by `verus-mirror` markers.

Write the Verus lemma statements first (`verus/lapidary_rect.rs`,
`verus/lapidary_transform.rs`, `verus/lapidary_hit.rs`, and
`verus/lapidary_damage.rs`, included by `verus/lapidary_proofs.rs`) and run
`make verus` to see the expected open goals. Then write the mirrors and proof
bodies, and the property tests. Add
`crates/cabochon-lapidary/tests/features/scene.feature`. Accept ADR 009.

### EP-M3: Burin raster-family workstations

Implement the `Workstation` trait and `RenderError`. Implement the `parley`
text adapter (`ParleyTextLayout`, implementing Lapidary's `TextLayout`) with an
injectable font collection, the float-boundary module
`cabochon_burin::float_boundary` (the only place EMU and Fixed16 values become
`f32` or `f64`, and the only place floats round back), and the workstations:
`RasterWorkstation` (PNG at a requested resolution), `ThumbnailWorkstation`
(fit within a maximum pixel box, preserving aspect ratio),
`InteractiveWorkstation` (a frame buffer with damage-aware redraw and a
`FrameSink` that records written spans), `HitMapWorkstation`,
`AccessibilityWorkstation`, and the PNG and geometry parts of
`ClipboardWorkstation`. Add `render_with_fallback` and the fault-injection
seam. Instrument rendering with a `tracing` span per workstation and the
metrics in `Interfaces and dependencies`. Bundle the test fonts in
`crates/cabochon-substrate-testkit/fonts/` (Noto Sans, Noto Sans Arabic, and
Noto Sans Hebrew, each with `OFL.txt` and a `SHA256SUMS` entry).

### EP-M4: Burin document-family workstations and parity

Implement `PdfWorkstation` (tagged PDF, document metadata, real text drawn
through `krilla`'s positioned-glyph API from the shared layout, images, and
clips), `PrintWorkstation` (`PrintIntent` page boxes, the PDF/A-2b validator,
an sRGB output intent, and a `PrintTicket`), and the PDF part of
`ClipboardWorkstation`. Implement the `hayro`-based read-back `Device` in the
testkit. Write the parity suite (`OB-BUR-PARITY-*`), the determinism cases, and
the output behavioural feature. Accept ADR 011. Mark roadmap tasks 1.0.3 and
1.0.4 done.

### EP-M5: Escutcheon resource format and compiler

Implement the resource source port (`ResourceSource`, with a `cap_std`
directory adapter and an in-memory adapter for tests), the KDL schema version 1
parser, the Fluent localizer with locale-chain fallback and the `en-XA`
pseudo-locale, icon compilation from SVG to Lapidary paths through `usvg` (with
its `text` feature disabled), the shortcut grammar, validation with located
diagnostics, and canonical emission. Add the behavioural feature and the
property suites. Accept ADR 010. Roadmap task 1.0.2 remains open until EP-M7
proves its success criterion.

### EP-M6: Clerestory application contract with the headless shell

Implement the `Application` trait (the driving side: command handling, command
state, content scene, and inspector source), the `Session` that owns windows,
menus, dialogs, focus, and command routing, the widget layout engine (rows,
columns, labels, buttons, separators, and property rows, measured through the
`TextLayout` port), the scene builders that draw furniture through Lapidary, the
`Role` enumeration and `AccessibilityTree`, the `HostShell` port, and the
`HeadlessShell` adapter with scripted input. Add the state-machine property
tests and the behavioural feature. Accept ADR 012.

### EP-M7: hosted shell adapter and the hello-window application

Implement `cabochon-clerestory-hosted`: the `winit` event loop, `softbuffer`
presentation of `InteractiveWorkstation` frames with damage, scale handling in
1/120 units, keyboard and pointer mapping, close requests, client-side
decorations where the compositor requires them, and `accesskit_winit`
publication. Implement `cabochon-hello-window` with its resources under
`crates/cabochon-hello-window/resources/`, the `run` and `export` subcommands,
`--locale`, `--shell headless|hosted`, `--script`, and
`--exit-after-first-frame`, a `tracing` subscriber writing to standard error,
and its end-to-end tests. Replace the `make test-hosted` stub with a recipe
that starts `weston --backend=headless`, runs the hosted smoke test, and fails
if `weston` is missing. Add a CI job that installs `weston` and runs
`make test-hosted`, keeping the existing workflow contract tests green. Mark
roadmap tasks 1.0.1 and 1.0.2 done.

### EP-M8: documentation reconciliation and roadmap closure

Reconcile every discovery with the upstream documents (see
`Outcomes & retrospective`): update `docs/cabochon-design.md` §§8.1 to 8.4 and
§13.1 with the contracts, tolerances, and proof inventory, update
`docs/users-guide.md` with the hello-window application, finish
`docs/developers-guide.md`, mark step 1.0 complete in `docs/roadmap.md`, and
set this plan's status to `COMPLETE`.

## Milestones and plateaus

### EP-M0

- Outcome: a workspace with seven empty member crates, workspace lints,
  prover tooling, and architecture guards; every existing gate still green.
- Requirements and gaps: prepares C1, C2, C6, `OB-ARCH-DEPS`,
  `OB-PROVER-TOOLING`, and `OB-PROOF-MIRROR`.
- Acceptance evidence: `make check-fmt`, `make lint`, and `make test` pass;
  `tests/substrate_dependency_contract.rs` passes; `make verus` prints
  `FORMAL-SKIP: no Verus proofs yet` and exits zero; `make kani` reports no
  harnesses and exits zero; `cargo tree -p cabochon` is unchanged.
- Conformance check: dependency rule encoded; no substrate behaviour yet; no
  Enfilade crates; the only public interfaces are empty crates; no persisted
  format; trace links current.
- Recovery: the conversion is one commit; revert it to return to the single
  crate.
- Remaining gaps: all substrate behaviour.
- Compatibility decision: none.

### EP-M1

- Outcome: Lapidary models scenes, units, and output intent, validates them,
  and round-trips them through scene documents.
- Requirements and gaps: `RM-1.0.3` (rectangles, paths, text runs, images,
  clipping and transforms as data, hit regions as data, physical units, and
  output intent); `TDD-VDI-RT`.
- Acceptance evidence: `OB-LAP-UNITS`, `OB-LAP-FIXED`, `OB-LAP-VALID`, and
  `OB-LAP-ROUNDTRIP` pass under `make test` and `make kani`.
- Conformance check: no floats; no dependency on other Cabochon crates; no
  host types; the scene document is labelled as a scene format, not a user
  document format; trace links current.
- Recovery: purely additive within `crates/cabochon-lapidary`; revert the
  milestone commits.
- Remaining gaps: resolution, hit testing, and invalidation.
- Compatibility decision: none.

### EP-M2

- Outcome: Lapidary resolves scenes, hit-tests them, and computes damage,
  with proved kernels.
- Requirements and gaps: `RM-1.0.3` (hit regions, invalidation); `TDD-LAP`;
  `TDD-VDI-INVAL` (semantic half).
- Acceptance evidence: `OB-LAP-RECT`, `OB-LAP-XFORM-BOUNDS`, `OB-LAP-HIT`,
  and `OB-LAP-DAMAGE` discharged by
  `make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs` with zero errors and
  no trusted assumptions; property tests and `scene.feature` pass under
  `make test`; `OB-PROOF-MIRROR` finds four regions.
- Conformance check: the `TextLayout` port is owned by Lapidary; no Burin
  dependency; proofs contain no shortcuts; trace links current.
- Recovery: revert the milestone; if a proof fails within tolerance, the
  fallback needs approval.
- Remaining gaps: real rendering.
- Compatibility decision: none.

### EP-M3

- Outcome: one scene renders to interactive, raster, thumbnail, hit-map,
  accessibility, and clipboard-geometry outputs, with failure handling.
- Requirements and gaps: `RM-1.0.4` (interactive rendering, raster export,
  thumbnailing, accessibility extraction, clipboard geometry, renderer failure);
  `TDD-VDI-INVAL` (pixel half); `TDD-VDI-FAIL`; `TDD-FAIL-RENDER`; `TDD-OBS`.
- Acceptance evidence: `OB-BUR-INVAL`, `OB-BUR-COVER`, `OB-BUR-HITMAP`,
  `OB-BUR-A11Y`, `OB-BUR-CLIP` (PNG and geometry), `OB-BUR-FAIL`, and the
  raster half of `OB-BUR-PARITY-*` pass under `make test`.
- Conformance check: Burin depends only on Lapidary; float arithmetic only in
  `float_boundary`; no private provider renderer path; trace links current.
- Recovery: revert the milestone; workstations are independent modules.
- Remaining gaps: PDF and print.
- Compatibility decision: none.

### EP-M4

- Outcome: the same scene also renders to PDF and print-oriented PDF, and the
  full cross-target parity suite passes.
- Requirements and gaps: completes `RM-1.0.3` and `RM-1.0.4`;
  `TDD-VDI-PARITY`; `TDD-VDI-PROP`; `TDD-VDI-TEXT` for shaping, bidirectional
  text, font fallback, and extraction.
- Acceptance evidence: `OB-BUR-PARITY-GEOM`, `OB-BUR-PARITY-TEXT`,
  `OB-BUR-PRINT`, `OB-BUR-DET`, and the output feature pass; roadmap 1.0.3 and
  1.0.4 marked done.
- Conformance check: print stops at preparation (decision D6); no SVG
  workstation (decision D7); trace links current.
- Recovery: revert the milestone.
- Remaining gaps: the resource format and the application contract.
- Compatibility decision: none.

### EP-M5

- Outcome: Escutcheon compiles resource bundles with located diagnostics,
  localization, icons, and shortcuts.
- Requirements and gaps: `RM-1.0.2` (format part); `TDD-ESC`; `TDD-INV-4`.
- Acceptance evidence: `OB-ESC-*` and `resources.feature` pass.
- Conformance check: Escutcheon depends only on Lapidary; no rendering code;
  resources are textual and versioned; trace links current.
- Recovery: revert the milestone.
- Remaining gaps: the hello-window proof of 1.0.2.
- Compatibility decision: none.

### EP-M6

- Outcome: Clerestory runs resource-defined applications under the headless
  shell with menus, commands, dialogs, focus, traversal, an inspector, and
  accessibility roles.
- Requirements and gaps: `RM-1.0.1` (all but the hosted adapter); `TDD-CLR`;
  `TDD-INV-1`; `TDD-INV-6`.
- Acceptance evidence: `OB-CLR-FOCUS`, `OB-CLR-MODAL`, `OB-CLR-MENU`,
  `OB-CLR-ROUTE`, `OB-CLR-A11Y`, and `hello_window.feature` pass under the
  headless shell.
- Conformance check: no host types in Clerestory; Clerestory does not depend
  on Burin; trace links current.
- Recovery: revert the milestone.
- Remaining gaps: the hosted adapter.
- Compatibility decision: none.

### EP-M7

- Outcome: the hello-window application runs under a real host compositor
  and exports its content to every workstation.
- Requirements and gaps: completes `RM-1.0.1` and `RM-1.0.2`; `TOR-SM-GEM`.
- Acceptance evidence: `OB-CLR-HOST` via `make test-hosted`; the
  hello-window end-to-end tests pass; roadmap 1.0.1 and 1.0.2 marked done.
- Conformance check: winit, softbuffer, and AccessKit appear only in the
  hosted crate; no Enfilade dependency; trace links current.
- Recovery: revert the milestone; the headless path from EP-M6 still works.
- Remaining gaps: documentation reconciliation.
- Compatibility decision: none.

### EP-M8

- Outcome: documentation and roadmap match the delivered contracts.
- Acceptance evidence: `make markdownlint`, `make nixie`, `make spelling`,
  `make check-fmt`, `make lint`, and `make test` pass; roadmap step 1.0 shows
  all four tasks done.
- Compatibility decision: none.

## Behavioural specifications

Each feature file is bound with `rstest-bdd` `#[scenario]` attributes.
Scenarios are written before the code that satisfies them, and each is run red
first.

`crates/cabochon-lapidary/tests/features/scene.feature`:

```gherkin
Feature: Lapidary scenes describe geometry independently of devices

  Scenario: The topmost hit region wins
    Given a scene with two overlapping hit regions "back" and "front"
    When the point inside both regions is hit-tested
    Then the result is "front"

  Scenario: Clipping hides hit regions
    Given a hit region inside a group clipped to its left half
    When a point in the right half of the region is hit-tested
    Then no hit region is reported

  Scenario: Editing one shape damages only that shape
    Given a resolved scene with three separated rectangles
    When the middle rectangle is recoloured
    Then the damage equals the middle rectangle's bounds
```

`crates/cabochon-burin/tests/features/output.feature`:

```gherkin
Feature: Burin renders one scene to every workstation

  Scenario: One scene, every output
    Given the hello-window content scene
    When it is rendered to every workstation
    Then each output's geometry matches the scene within its declared tolerance
    And the text extracted from the PDF equals the scene's text runs

  Scenario: A renderer failure is bounded and harmless
    Given a scene whose image cannot be decoded
    When it is rendered to the raster workstation
    Then the result is a "undecodable-image" render error naming the image node
    And the scene is unchanged
    And the fallback output is a diagnostic scene, not a flattened copy
```

`crates/cabochon-escutcheon/tests/features/resources.feature`:

```gherkin
Feature: Escutcheon compiles textual, localizable resources

  Scenario: A valid bundle compiles
    Given the hello-window resource bundle
    When it is compiled for locale "en-GB"
    Then the menu "File" contains the command "app.quit" with shortcut "Ctrl+Q"

  Scenario: A dangling command reference is located
    Given a bundle whose menu item names the undeclared command "file.frobnicate"
    When it is compiled
    Then compilation fails with a diagnostic at the menu item's line and column

  Scenario: Missing translations fall back to the base locale
    Given a bundle whose "fr" strings omit "about-title"
    When "about-title" is looked up for locale "fr"
    Then the base-locale text is returned and a fallback is recorded

  Scenario: The pseudo-locale transforms every string
    When the hello-window bundle is compiled for locale "en-XA"
    Then every visible string differs from its "en-GB" text
```

`crates/cabochon-clerestory/tests/features/hello_window.feature`:

```gherkin
Feature: Clerestory runs resource-defined applications

  Scenario: The window comes from resources
    Given the hello-window application under the headless shell
    When the session starts
    Then one window titled "Hello, Cabochon" is presented
    And its menu bar lists "File", "View", and "Help"

  Scenario: Shortcuts and menus dispatch the same command
    Given the hello-window application under the headless shell
    When the user presses "F1"
    And the user chooses "Help" then "About Hello, Cabochon"
    Then the dispatch log shows "app.about" twice with identical records

  Scenario: Dialogs are modal and restore focus
    Given the About dialog is open
    When the user presses "Tab" five times
    Then focus never leaves the dialog
    When the user presses "Escape"
    Then focus returns to the element that had it before the dialog opened

  Scenario: Furniture is accessible
    Given the hello-window application under the headless shell
    Then the accessibility tree contains a window, a menu bar, and menu items
    And every interactive element has a role and a non-empty name
```

## Concrete steps

Run every command from the worktree root. Long-running gates write through
`tee` so their output can be reviewed after truncation.

Milestone gate sequence (run sequentially; never in parallel with another gate
on the same worktree):

```bash
BRANCH=$(git branch --show-current)
make check-fmt 2>&1 | tee /tmp/check-fmt-cabochon-${BRANCH}.out
make lint 2>&1 | tee /tmp/lint-cabochon-${BRANCH}.out
make test 2>&1 | tee /tmp/test-cabochon-${BRANCH}.out
```

Formal gates, from EP-M1 (Kani) and EP-M2 (Verus):

```bash
make kani 2>&1 | tee /tmp/kani-cabochon-${BRANCH}.out
make install-verus
make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs 2>&1 | tee /tmp/verus-cabochon-${BRANCH}.out
```

Expected Verus result once EP-M2 is complete:

```plaintext
verification results:: <N> verified, 0 errors
```

Focused red and green loops, for example:

```bash
cargo nextest run -p cabochon-lapidary --test hit_test_properties
cargo nextest run -p cabochon-burin --test parity
```

Hosted smoke test, from EP-M7 (requires `weston`):

```bash
make test-hosted 2>&1 | tee /tmp/test-hosted-cabochon-${BRANCH}.out
```

Expected hosted output:

```plaintext
hello-window: shell=hosted windows=1 frames=1 accessibility_nodes=<N> exit=0
```

Hello-window export, from EP-M7:

```bash
cargo run -p cabochon-hello-window -- export --out /tmp/hello
ls /tmp/hello
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  hit-map.json  thumbnail.png
```

## Validation and acceptance

Quality criteria:

- Tests: `make test` passes, including every obligation's tests and every
  behavioural scenario; `make test-hosted` passes in its CI job.
- Verification: `make kani` discharges `OB-LAP-UNITS` and `OB-LAP-FIXED`;
  `make verus` discharges `OB-LAP-RECT`, `OB-LAP-XFORM-BOUNDS`, `OB-LAP-HIT`,
  and `OB-LAP-DAMAGE` with no trusted assumptions; the mirror and tooling
  contracts pass.
- Lint and formatting: `make check-fmt` and `make lint` pass with no new
  `#[expect]` outside the float-boundary modules.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Security: no new process-environment access, no `unsafe`, and
  `make audit` passes.

Red-Green-Refactor evidence is recorded in `Artefacts and notes` for each
milestone: the red command and its failure, the green command, and the refactor
gate run.

Acceptance by behaviour:

1. Running the hello-window `run` subcommand under GNOME or KDE Plasma shows
   the resource-defined window, menus, and dialogs, and Accerciser lists their
   roles and names.
2. Running `export` produces the seven outputs, and the parity suite proves
   they agree.
3. Changing a string in `resources/locales/en-GB/hello-window.ftl` changes the
   window title without recompiling Rust (the resources are loaded at start
   from the bundle directory, with an embedded copy as the default).

## Idempotence and recovery

Every step is additive and can be rerun. The workspace conversion in EP-M0 is
one commit and reverts cleanly. `make install-verus` is idempotent: it verifies
the checksum and reuses `.verus/<version>`, which is ignored by Git. Export
writes only inside the requested output directory and overwrites its own files.
If a gate fails halfway, fix the cause and rerun the whole gate sequence; never
skip a gate. Snapshot updates through `cargo insta review` happen only after
confirming the snapshot change reflects an intended contract change.

## Artefacts and notes

Record red, green, and refactor transcripts here per milestone as work proceeds.

## Interfaces and dependencies

These signatures are the contracts each milestone must leave in place. They are
pre-1.0 internal interfaces; no compatibility layer is owed to anyone.

### Workspace layout

```plaintext
crates/
├── cabochon-lapidary/          scene model, units, resolution, hit testing, damage
├── cabochon-burin/             workstations, text adapter, float boundary
├── cabochon-escutcheon/        resource schema, compiler, localization
├── cabochon-clerestory/        application contract, session, HostShell port, headless shell
├── cabochon-clerestory-hosted/ winit, softbuffer, and AccessKit adapter
├── cabochon-hello-window/      the hello-window application and its resources
└── cabochon-substrate-testkit/ generators, fonts, PDF read-back, tolerances (publish = false)
verus/                          Verus spec mirrors and proofs
tools/verus/                    pinned Verus version and checksums
tools/rust-prover-tools/REF     pinned prover-tools commit
```

### Lapidary (`cabochon-lapidary`)

```rust,no_run
pub struct Emu(i64);
impl Emu {
    pub const LIMIT: i64 = 1 << 47;
    pub const fn from_points(points: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_millimetres(mm: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_reference_pixels(px: i64) -> Result<Self, LengthOutOfRange>;
}

pub struct Fixed16(i32);
pub struct Transform { pub xx: Fixed16, pub xy: Fixed16, pub yx: Fixed16, pub yy: Fixed16, pub tx: Emu, pub ty: Emu }

pub struct Scene {
    pub page: PageGeometry,
    pub output_intent: OutputIntent,
    pub root: Group,
    pub images: ImageStore,
}

pub enum Node { Group(Group), Shape(Shape), Text(TextNode), Image(ImageNode), HitRegion(HitRegion) }

pub enum OutputIntent { Screen, Document, Print(PrintIntent) }

pub trait TextLayout {
    fn layout(&self, run: &TextRun) -> Result<ShapedRun, TextLayoutError>;
}

pub fn resolve(scene: &Scene, layout: &dyn TextLayout) -> Result<ResolvedScene, ResolveError>;

impl ResolvedScene {
    pub fn hit_test(&self, point: DevicePoint) -> Option<HitRegionId>;
    pub fn hit_test_all(&self, point: DevicePoint) -> Vec<HitRegionId>;
}

pub fn damage(old: &ResolvedScene, new: &ResolvedScene) -> Damage;

impl SceneDocument {
    pub fn encode(scene: &Scene) -> Result<String, SceneEncodeError>;
    pub fn decode(text: &str) -> Result<Scene, SceneDecodeError>;
}
```

### Burin (`cabochon-burin`)

```rust,no_run
pub trait Workstation {
    type Output;
    fn render(&self, scene: &ResolvedScene, request: &RenderRequest) -> Result<Self::Output, RenderError>;
}

pub enum RenderError {
    UnsupportedFeature { workstation: WorkstationKind, feature: FeatureKind },
    MissingFont { node: NodeId },
    UndecodableImage { node: NodeId },
    ResourceLimit { workstation: WorkstationKind, limit: LimitKind },
    Backend { workstation: WorkstationKind, class: BackendFailureClass },
}

pub struct ParleyTextLayout { /* font collection injected at construction */ }

pub fn render_with_fallback<W: Workstation>(
    workstation: &W,
    scene: &ResolvedScene,
    request: &RenderRequest,
) -> RenderOutcome<W::Output>;
```

Workstations: `InteractiveWorkstation`, `RasterWorkstation`,
`ThumbnailWorkstation`, `PdfWorkstation`, `PrintWorkstation`,
`AccessibilityWorkstation`, `ClipboardWorkstation`, and `HitMapWorkstation`.

Metrics, described with `describe_counter!` and `describe_histogram!`:
`burin_render_total{workstation, outcome}`,
`burin_render_failures_total{workstation, class}`, and
`burin_render_seconds{workstation}`. Labels are fixed enumerations.

### Escutcheon (`cabochon-escutcheon`)

```rust,no_run
pub trait ResourceSource {
    fn read(&self, path: &ResourcePath) -> Result<String, ResourceReadError>;
}

pub fn compile(source: &dyn ResourceSource, locale: &LocaleChain) -> Result<ResourceBundle, CompileDiagnostics>;

pub struct ResourceBundle { /* commands, menus, dialogs, inspectors, icons, strings, shortcuts */ }
pub struct CommandSpec { pub id: CommandId, pub label: StringKey, pub tooltip: Option<StringKey>,
                         pub icon: Option<IconId>, pub shortcuts: Vec<Shortcut>, pub enabled_by_default: bool }
```

Resource bundle layout for an application:

```plaintext
resources/
├── app.escutcheon.kdl        schema version, commands, menus, dialogs, inspectors, windows, icons
├── locales/en-GB/app.ftl     base locale (complete)
├── locales/fr/app.ftl        optional translations
└── icons/*.svg
```

Example KDL fragment:

```plaintext
escutcheon version=1
command "app.quit" label="cmd-quit" shortcut="Ctrl+Q"
command "app.about" label="cmd-about" shortcut="F1" icon="hello"
menu-bar {
    menu label="menu-file" { item command="app.quit" }
    menu label="menu-help" { item command="app.about" }
}
dialog "about" title="about-title" {
    column {
        label text="about-body"
        row align="end" { button command="dialog.ok" default=#true }
    }
}
```

Metric: `escutcheon_compile_total{outcome}`.

### Clerestory (`cabochon-clerestory`)

```rust,no_run
pub trait Application {
    fn handle_command(&mut self, command: &CommandId, context: &mut CommandContext) -> CommandOutcome;
    fn command_state(&self, command: &CommandId) -> CommandState;
    fn content(&self, window: WindowId) -> Option<Scene>;
    fn inspector(&self, window: WindowId) -> Option<InspectorModel>;
}

pub trait HostShell {
    fn open_window(&mut self, spec: &WindowSpec) -> Result<HostWindowId, ShellError>;
    fn present(&mut self, window: HostWindowId, frame: &FrameUpdate<'_>) -> Result<(), ShellError>;
    fn publish_accessibility(&mut self, window: HostWindowId, tree: &AccessibilityTree) -> Result<(), ShellError>;
    fn close_window(&mut self, window: HostWindowId) -> Result<(), ShellError>;
}

pub struct Session<A: Application> { /* windows, menus, dialogs, focus, routing */ }
impl<A: Application> Session<A> {
    pub fn handle(&mut self, event: HostEvent, shell: &mut dyn HostShell) -> Result<SessionStatus, SessionError>;
}

pub enum Role { Window, MenuBar, Menu, MenuItem, Dialog, Alert, Button, Label, Panel, Inspector, Canvas }
pub enum StandardDialog { Alert(DialogText), Confirm(DialogText), About }
```

`HeadlessShell` lives in `cabochon_clerestory::shell::headless` and records
windows, frames, accessibility trees, and the dispatch log. Metric:
`clerestory_command_dispatch_total{outcome}`.

### Dependencies

All requirements are caret requirements in `[workspace.dependencies]`;
`Cargo.lock` records the exact versions.

Runtime dependencies: `serde` 1, `serde_json` 1, `base64` 0.22, `thiserror` 2,
`tracing` 0.1, `metrics` 0.24, `kdl` 6.7, `fluent-bundle` 0.16, `fluent-pseudo`
0.3, `unic-langid` 0.9, `usvg` 0.48 (default features off), `parley` 0.12,
`fontique` 0.12, `vello_cpu` 0.3, `kurbo` 0.13, `krilla` 0.8, `winit` 0.30.13,
`softbuffer` 0.4.8, `accesskit` 0.25, `accesskit_winit` 0.34, `cap-std` 4,
`camino` 1, `clap` 4 (application only), and `tracing-subscriber` 0.3
(application only).

Development dependencies: `rstest` 0.26, `rstest-bdd` 0.6, `rstest-bdd-macros`
0.6, `googletest` 0.14, `pretty_assertions` 1.4, `insta` 1.49, `proptest` 1.11,
`proptest-state-machine` 0.9, `hayro` 0.8, `hayro-interpret` 0.8,
`accesskit_consumer` 0.39, and `mockable` 0.3 (for the application's
environment reader).

Verification tooling: Verus (pinned in `tools/verus/VERSION`), Kani (already
installed; `cargo kani`), and `rust-prover-tools` (pinned in
`tools/rust-prover-tools/REF`).

Patch versions above are the October 2026 releases reported by crates.io; use
the newest compatible release at the time each milestone starts and record any
deviation in `Decision log`.

## Documentation obligations

- ADR 009 (integer EMU geometry and fixed-point transforms), ADR 010 (KDL,
  Fluent, and SVG resources), ADR 011 (software renderer and workstation
  parity), and ADR 012 (in-window furniture and the host-shell port), each
  referenced from `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §§8.1 to 8.4 gain the contract summaries,
  tolerances, and proof inventory, with the proof-driven representation
  decisions (half-open integer rectangles, prefix-suffix damage, and
  topmost-wins scanning) stated as design facts.
- `docs/developers-guide.md` gains sections on the workspace, the dependency
  rule and its contract test, test conventions (`rstest`, `rstest-bdd`,
  `googletest`, `insta`, `proptest`), the float-boundary policy, the prover
  tooling, and the proof inventory with what each proof does not cover. It
  documents the reuse policy for the testkit: test-only, consumed only from
  integration tests, and never a production dependency.
- `docs/users-guide.md` gains a hello-window section covering `run`,
  `export`, and `--locale`.
- `docs/repository-layout.md` and `docs/contents.md` list every new directory
  and document, including this plan.
- `docs/roadmap.md` marks 1.0.1 to 1.0.4 done when their success criteria are
  proved.

## Revision notes

- 2026-10-18: initial draft, written from Wyvern reconnaissance and Firecrawl
  research. Awaiting the community-of-experts design review.
