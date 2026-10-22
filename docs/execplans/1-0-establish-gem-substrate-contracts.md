# Establish the GEM substrate contracts (roadmap 1.0)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT

Implementation must not begin until a human reviewer explicitly approves this
plan. Silence is not approval. The reviewer accepted decisions D1, D6, D7, D8,
D10, D11, D12, D13, and D14 on 2026-10-22; approval of the plan as a whole is
still required before EP-M0 starts.

## Purpose / big picture

Cabochon is a document-centred application environment modelled on the Graphics
Environment Manager (GEM). GEM split the desktop into an Application
Environment Services (AES) layer for windows, menus, and dialogs, and a Virtual
Device Interface (VDI) for device-independent drawing. Cabochon keeps that
split under four names:

- Clerestory is the application environment (the AES analogue): windows,
  menus, commands, dialogs, focus, keyboard traversal, text input, inspector
  panels, accessibility roles, and adaptation to a host desktop such as GNOME
  or KDE Plasma.
- Escutcheon is the resource system: textual, localizable descriptions of
  menus, strings, icons, command metadata, shortcuts, dialog layout, and
  accessibility annotations.
- Lapidary is the device-independent scene model (the VDI analogue): shapes,
  text, images, clipping, transforms, layers, hit regions, invalidation,
  physical units, and output intent.
- Burin is the renderer: it turns one Lapidary scene into concrete output for
  every target, which `docs/context.md` calls a workstation.

ADR 0007 makes those four boundaries production contracts in hosted mode,
before any Enfilade (object graph) behaviour exists. This plan delivers roadmap
step 1.0 and its four tasks, 1.0.1 to 1.0.4.

After this work, a developer can run a small application, `hello-window`,
inside an ordinary GNOME or KDE Plasma session. Its window, menu bar, commands,
shortcuts, strings, icon, About dialog, and inspector panel all come from
Escutcheon resource files rather than hand-built widgets. Its content is a
Lapidary scene that Burin draws on screen. Typing a new greeting into the
inspector's text field, including through an input method, redraws only the
damaged part of the content. The same scene can be exported, with one command,
to a PDF, a print-ready PDF, a PNG, a thumbnail, an accessibility geometry
description, clipboard payloads, and a hit-testing map. Automated tests prove
that every one of those outputs agrees with the scene's geometry, text
placement, clipping, and metadata within declared tolerances. Generated scenes
survive a save-and-load round trip unchanged. Redrawing after an edit leaves no
stale pixels and touches nothing outside the damaged region. None of this
depends on Enfilade.

Observable success, end to end:

1. `cargo run -p cabochon-hello-window -- run` opens a window titled
   "Hello, Cabochon" with a File, View, and Help menu bar. Ctrl+Q quits, F1
   opens the About dialog, Tab moves focus, and Accerciser (an accessibility
   inspector) sees a window, a menu bar, menu items, a dialog, buttons, a text
   field, and the content canvas with its text, each with a name.
2. `cargo run -p cabochon-hello-window -- run --locale en-XA` shows the same
   user interface (UI) with pseudo-localized strings, and `--locale ar-XB`
   shows it mirrored for right-to-left text, without code changes.
3. `cargo run -p cabochon-hello-window -- export --out /tmp/hello` writes
   `content.pdf`, `content-print.pdf`, `content.png`, `thumbnail.png`,
   `accessibility.json`, `clipboard/`, `hit-map.json`, and `export.json` (a
   manifest recording fonts and fallbacks), all derived from one scene.
4. `cargo run -p cabochon-escutcheon --bin escutcheon-check -- <dir>` reports
   located diagnostics for a resource bundle, so an author who does not write
   Rust can validate resources.
5. `make check-fmt`, `make lint`, and `make test` pass. `make kani` and
   `make verus` discharge the formal obligations listed in the verification
   plan. `make test-hosted` runs the hello-window application under a headless
   Wayland compositor and reports one window, one presented frame, and one
   accessibility tree handed to AccessKit.

How to read identifiers in this plan: `RM-` names a roadmap task, `TDD-` a
technical-design requirement, `TOR-` a terms-of-reference measure, `EP-M` a
milestone of this plan, `OB-` a verification obligation, `A-` a trusted
external assumption (an axiom), `C` a constraint, and `D` a decision.

## Constraints

These are hard invariants. Violating any of them requires escalation, not a
workaround.

- C1, GEM-derived invariants. Every invariant in `docs/cabochon-design.md` §3
  holds: applications reach the desktop only through Clerestory, draw only
  through Lapidary scenes, render only through Burin, and take UI structure
  from Escutcheon. No substrate crate depends on or names an Enfilade type (ADR
  007, ADR 0008). Later Enfilade integration lives in bridge crates (decision
  D12).
- C2, dependency direction. Dependencies point inwards, counting normal
  (non-development) edges. `cabochon-lapidary` depends on no other Cabochon
  crate. `cabochon-burin` and `cabochon-escutcheon` may depend only on
  `cabochon-lapidary`. `cabochon-clerestory` may depend only on
  `cabochon-lapidary` and `cabochon-escutcheon`. Only adapter and application
  crates (`cabochon-clerestory-hosted`, `cabochon-hello-window`) may combine
  Clerestory with Burin. `OB-ARCH-DEPS` enforces this.
- C3, no third-party types in contracts. Public signatures of
  `cabochon-lapidary`, `cabochon-escutcheon`, and `cabochon-clerestory` never
  mention `winit`, Wayland, X11, AccessKit, GTK, Qt, `kurbo`, `parley`,
  `krilla`, `vello_cpu`, `kdl`, Fluent, `unic-langid`, or `miette` types.
  Burin's public signatures expose only Lapidary types and Burin-owned types.
  Host types appear only in `cabochon-clerestory-hosted` (ADR 0002).
- C4, integer geometry. Lapidary geometry uses integer English Metric Units
  (EMU) and fixed-point transforms. Floating-point arithmetic and lossy casts
  are permitted only in three named float-boundary modules:
  `cabochon_burin::float_boundary`,
  `cabochon_escutcheon::icon::float_boundary`, and
  `cabochon_clerestory_hosted::float_boundary`. Each such function carries a
  reasoned `#[expect(clippy::float_arithmetic, reason = "...")]` (or the
  matching cast lint), as the lint baseline in `docs/developers-guide.md`
  allows for genuine floating-point arithmetic.
- C5, repository lint, safety, and documentation policy. Public Rust items
  follow the style guide's doc-comment rules (`# Parameters`, `# Returns`,
  `# Errors`, and `# Examples` that run as doctests unless they genuinely
  cannot execute). `unsafe_code` stays `forbid`. `#[allow]` is never used.
  Every lint expectation is tightly scoped and reasoned. No `.expect()` or
  `.unwrap()` outside tests. No source file exceeds 400 lines.
- C6, existing contracts. The root package `cabochon`, its binary, and
  `tests/cli_greeting.rs` keep working. The existing contract tests under
  `tests/` keep passing. The `test`, `lint-clippy`, `typecheck`, and `build`
  recipes gain no extra Cargo invocations or prerequisites, and
  `default-members` is not introduced. Member crates declare no Cargo features,
  because every gate runs with `--all-features`. The root package keeps
  `rust-version = "1.85"`, and the `msrv` CI job
  (`cargo +1.85.0 check --lib --all-features --locked`) keeps passing after the
  workspace conversion.
- C7, no persisted user-document format. This plan defines a Lapidary scene
  document (version 1) and an Escutcheon resource schema (version 1). Neither
  is a persisted user-document format, and neither may be presented as one.
- C8, no proof shortcuts. No `assume`, `admit`, `assume_specification`,
  `#[verifier::external_body]`, `#[verifier(external_body)]`,
  `#[verifier::external]`, or trusted proof bodies are introduced to make a
  Verus proof go through. Kani harnesses may use `kani::assume` only to state
  an input domain that the production code also checks, and each use names that
  check in a comment. An unproved theorem stays in this plan or the design
  documentation until it is proved.
- C9, tests do not mutate the process environment. Locale and display
  selection reach the application through explicit arguments or an injected
  `mockable::Env`, never through direct `std::env` access, which `clippy.toml`
  already forbids. Child processes in end-to-end tests receive environment
  values through `Command::env`.
- C10, library instrumentation only. Library crates emit `tracing` events and
  `metrics` measurements but never install a subscriber or recorder. Only the
  `cabochon-hello-window` binary installs them.
- C11, shared host. Tests and Make recipes never kill processes they did not
  start. A recipe that starts `weston` records its process identifier and stops
  only that process.

## Tolerances (exception triggers)

These thresholds define the limits of autonomous action. Reaching one means
stop, record the situation in `Decision log`, and wait for direction.

- Scope: a milestone needs more than 3,000 net added lines of Rust (excluding
  test fixtures, fonts, and generated snapshots) or touches more than 60 files.
  Split the milestone and record the split rather than continuing.
- Interface: a contract in `Interfaces and dependencies` must change shape in a
  way that alters a downstream roadmap task (2.1.2 or 2.1.3).
- Dependencies: a dependency not listed in `Interfaces and dependencies` is
  needed, or a listed crate must move to a new major or pre-1.0 minor series.
- Parity: a measured cross-target disagreement exceeds a declared tolerance
  after three focused fix attempts. Tolerances are never loosened silently;
  loosening one is a decision that needs approval.
- Proof: a Verus obligation remains unproved after two working days of proof
  effort, or appears to need any construct banned by C8. Use the documented
  fallback (Kani or property testing) only after approval.
- Spikes: an EP-M0 or EP-M4a spike rejects its hypothesis (see `Plan of work`).
  Record the evidence and the proposed alternative, then wait.
- Hosted environment: a headless Wayland compositor cannot be provisioned in
  continuous integration (CI) or locally at the start of EP-M7b.
- Coverage: the coverage ratchet in `.github/workflows/ci.yml` fails because
  hosted-only code is unreachable without a display, and thinning the adapter
  does not resolve it.
- Upstream conflict: implementation evidence contradicts
  `docs/cabochon-design.md`, an ADR, or the roadmap. Set `Status: BLOCKED` and
  record the proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.

## Risks

- Risk: the third-party rendering crates are all pre-1.0 (`vello_cpu` 0.3,
  `krilla` 0.8, `parley` 0.12, `hayro` 0.8) and churn roughly quarterly.
  Severity: medium. Likelihood: high. Mitigation: confine each crate to one
  Burin adapter module behind a Burin-owned trait, commit `Cargo.lock`, add
  each dependency in its own commit so it reverts cleanly, and make parity
  tests the upgrade gate.
- Risk: `vello_cpu` selects single instruction, multiple data (SIMD) code
  paths at run time, and the Cranelift development backend implements only part
  of the `core::arch` intrinsics. Tests could then crash under Cranelift or
  diverge between the development host, CI runners, and the LLVM coverage
  build. Severity: high. Likelihood: medium. Mitigation: an EP-M0 spike renders
  a scene under `make test` and coverage; every test render goes through one
  testkit constructor that pins the scalar fallback level, single-threaded
  rendering, and one render mode. If Cranelift still fails, a per-package
  profile override compiles `vello_cpu` with LLVM (needs approval). The
  production SIMD path is an untested residual, recorded in
  `docs/developers-guide.md`.
- Risk: font-stack version skew. `krilla` pins `skrifa` 0.42 while `parley`
  and `vello_cpu` use a newer `skrifa`, so two font stacks get compiled.
  Severity: low. Likelihood: high. Mitigation: pass fonts between stacks only
  as bytes plus glyph identifiers, through Lapidary's `FontSet`.
- Risk: anti-aliasing differences make raster-to-PDF pixel comparison flaky.
  Severity: medium. Likelihood: medium. Mitigation: compare geometry rather
  than pixels wherever possible. Pixel comparisons require exact agreement
  outside a one-pixel edge band, plus a bounds check.
- Risk: damage-clipped re-rendering in `vello_cpu` may not be bit-identical to
  a full re-render at damage edges, because compositing a clip layer adds
  rounding. Severity: medium. Likelihood: medium. Mitigation: snap damage
  outwards to whole device pixels and run the bit-exactness spike at the start
  of EP-M3b. If it fails, escalate with the measured difference before adopting
  a per-channel tolerance.
- Risk: `krilla` may round glyph adjustments per glyph, accumulating error
  along a run, and converts quadratic curves to cubics. Severity: medium.
  Likelihood: medium. Mitigation: the EP-M4a spike measures the actual error
  before the vector tolerance is fixed, and parity compares on-curve points.
- Risk: `hayro` may not return right-to-left or ligated text in logical order.
  Severity: medium. Likelihood: medium. Mitigation: the EP-M4a spike checks
  this; text comparison normalizes to logical order and Unicode Normalization
  Form C (NFC).
- Risk: the Verus proofs prove harder than sketched, especially transform
  bounds and winding, which need non-linear arithmetic. Severity: medium.
  Likelihood: medium. Mitigation: keep the proved kernels small, total, and
  integer-only. Each obligation names a fallback that needs approval.
- Risk: the proved kernel drifts from the production code. Severity: high.
  Likelihood: medium. Mitigation: prefer a single-source kernel written inside
  `verus!` and compiled by `rustc` with ghost code erased (decision D11),
  decided by an EP-M0 spike; otherwise use a spec mirror with a parser-based
  drift guard (`OB-PROOF-MIRROR`).
- Risk: `winit` 0.31 is in beta and changes the event-loop API. Severity: low.
  Likelihood: high. Mitigation: use stable `winit` 0.30.13 (which
  `accesskit_winit` 0.34 supports) and keep all `winit` code inside one adapter
  crate.
- Risk: CI runners and the development host lack a headless Wayland
  compositor; `winit` silently falls back to X11 when no Wayland socket exists;
  no session bus means AccessKit publishes nowhere. Severity: medium.
  Likelihood: high. Mitigation: provisioning is a gated first step of EP-M7b
  (`weston` from Ubuntu packages in CI, Extra Packages for Enterprise Linux
  (EPEL) or a rootless Podman image locally); the hosted adapter forces the
  Wayland backend in tests; the test runs under `dbus-run-session`;
  `OB-CLR-HOST` checks the tree handed to AccessKit, and reading it back over
  the Assistive Technology Service Provider Interface (AT-SPI) is a named
  residual for roadmap 5.1.4.
- Risk: the coverage ratchet fails EP-M7b because the hosted event loop cannot
  run without a display. Severity: medium. Likelihood: medium. Mitigation: keep
  `cabochon-clerestory-hosted` thin, with key, scale, decoration, and
  accessibility mapping in pure, unit-tested functions; investigate whether the
  ratchet supports exclusions.
- Risk: CI time grows sharply, because the heavy dependency set compiles about
  five times per pull request (rustdoc, Clippy, Whitaker, coverage, and
  doctests) and property tests render many scenes. Severity: medium.
  Likelihood: high. Mitigation: per-suite proptest budgets, a nextest `render`
  test group, measured build times recorded at EP-M3a, and an
  optimization-level override for the heavy crates if the measurement justifies
  it.
- Risk: the workspace conversion breaks the Rust 1.85 `msrv` CI job, or the
  Rust-version-aware resolver (resolver 3) prefers older dependency releases
  because the root package declares `rust-version = "1.85"`. Cargo 1.85 must
  read every member manifest and the whole lockfile even though it checks only
  the root library. Severity: medium. Likelihood: medium. Mitigation: each
  member declares a `rust-version` no lower than the highest `rust-version`
  among its dependencies; EP-M0 runs the `msrv` command locally after the
  conversion and checks that `Cargo.lock` selects the newest releases the caret
  requirements allow. If the job cannot stay green, that is an upstream
  conflict to escalate, not a reason to weaken the job.
- Risk: `make audit` surfaces advisories (often "unmaintained") in the new
  dependency tree. Severity: low. Likelihood: high. Mitigation: each dependency
  commit runs `make audit`; any ignore is a reviewed decision.
- Risk: the workspace conversion collides with roadmap task 1.1.3, which owns
  "the workspace crate structure". Severity: medium. Likelihood: medium.
  Mitigation: create only the substrate crates the design already names, and
  document their ownership and reuse policy in EP-M0 (decision D1).
- Risk: bundled fonts add repository weight and licence obligations.
  Severity: low. Likelihood: certain. Mitigation: bundle only static subsets of
  Open Font Licence (OFL) fonts with their licence text and checksums.

## Progress

- [x] (2026-10-18) Renamed the branch to
  `1-0-establish-gem-substrate-contracts`.
- [x] (2026-10-18) Reconnaissance: Wyvern agents surveyed the build and test
  infrastructure, the substrate requirements in the white paper, terms of
  reference, and ADRs, and the testing and documentation conventions.
- [x] (2026-10-18) External research with Firecrawl: rendering, text, PDF,
  windowing, accessibility, resource formats, localization, testing crates, and
  Verus tooling.
- [x] (2026-10-18) Drafted the ExecPlan.
- [x] (2026-10-18) Community-of-experts review by four panels: structure and
  contracts, verification and proofs, failure modes and cost, and alternatives
  and viability. Verdicts: three "proceed with conditions" and one "revise".
- [x] (2026-10-18) Revised the plan to address every panel finding (see
  `Revision notes`).
- [x] (2026-10-22) The reviewer accepted decisions D1, D6, D7, D8, D10, D11,
  D12, D13, and D14.
- [ ] Reviewer approval of the plan as a whole.
- [ ] EP-M0: workspace, tooling, architecture guards, and feasibility spikes.
- [ ] EP-M1: Lapidary scene model and scene documents.
- [ ] EP-M2a: Lapidary resolution, device mapping, hit testing, damage, and
  accessibility nodes.
- [ ] EP-M2b: Verus proofs for the Lapidary kernels.
- [ ] EP-M3a: Burin foundations, text layout, raster, and thumbnail.
- [ ] EP-M3b: interactive surface and invalidation.
- [ ] EP-M3c: hit map, accessibility, clipboard geometry, and failure
  handling.
- [ ] EP-M4a: PDF read-back device and precision spike.
- [ ] EP-M4b: PDF and print workstations and the full parity suite.
- [ ] EP-M5: Escutcheon resource format and compiler.
- [ ] EP-M6a: Clerestory session, state machines, routing, and the headless
  shell.
- [ ] EP-M6b: Clerestory layout, furniture, text input, inspector, and
  accessibility tree.
- [ ] EP-M7a: the hello-window application under the headless shell.
- [ ] EP-M7b: the hosted shell adapter and its CI job.
- [ ] EP-M8: documentation reconciliation and roadmap closure.

## Surprises & discoveries

- Observation: the white paper names GEM's AES, VDI, and `.RSC` resource
  files, but never VDI workstations, metafiles, or GEM's AES event calls.
  Evidence: Wyvern survey of `references/cabochon-white-paper.md`. Impact:
  "workstation" comes from `docs/context.md`; "metafile" is not introduced, and
  the serialized scene is called a "scene document".
- Observation: `krilla` supports PDF/A and PDF/UA but explicitly not PDF/X.
  Evidence: `krilla` source and documentation, October 2026. Impact:
  print-oriented output is PDF/A-2b with page boxes, an sRGB output intent, and
  a print ticket (decision D6).
- Observation: the lint baseline denies `float_arithmetic` and the lossy cast
  lints, and `docs/developers-guide.md` already permits a reasoned `#[expect]`
  for genuine floating-point arithmetic. Impact: integer geometry is the
  natural representation and makes the Verus obligations tractable (decision
  D2).
- Observation: `docs/developers-guide.md` states that Cabochon is a single
  crate rather than a workspace. Impact: EP-M0 rewrites that paragraph and
  moves the lint tables to `[workspace.lints]`.
- Observation: no Verus or Kani tooling exists in the repository; Kani is
  installed on the development host and Verus is not. Impact: EP-M0 adds pinned
  prover tooling through `rust-prover-tools`.
- Observation: `vstd` and `verus_builtin_macros` are published on crates.io
  with dated pre-release versions (for example `0.0.0-2026-10-18-0147`).
  Evidence: crates.io API, 2026-10-18. Impact: a single-source kernel that
  `rustc` compiles with ghost code erased may be feasible (decision D11).
- Observation: `mockable` is at 3.0.0, while
  `docs/reliable-testing-in-rust-via-dependency-injection.md` cites `0.3`.
  Evidence: crates.io API. Impact: use `mockable` 3 and correct the guide in
  EP-M7a.
- Observation: `accesskit_winit` 0.34.1 requires `winit` ^0.30.5. Evidence:
  crates.io dependency metadata. Impact: `winit` 0.30.13 is compatible.
- Observation: running `make spelling` regenerates `typos.toml` with an
  unrelated change to a shared ignore pattern. Evidence: `git diff typos.toml`
  after the gate on 2026-10-18. Impact: per `AGENTS.md`, that regeneration
  belongs in a separate base pull request; pull request #51 carried the
  identical change and merged into `main` as `86f9791`. After this branch was
  rebased onto it on 2026-10-22, `make spelling` leaves `typos.toml` unchanged.
- Observation: pull request #65 adopted the shared documentation style guide.
  It moved every ADR to `docs/adrs/adr-nnnn-title-slug.md` with four-digit
  numbers, requires Rust doc comments with `# Parameters`, `# Returns`, and
  `# Errors` sections and runnable examples, and keeps repository-layout
  material out of the developers' guide. Evidence: `git diff 1f43018 0de2b57`
  on 2026-10-22. Impact: this plan's ADRs become ADR 0009 to ADR 0012 under
  `docs/adrs/`, its references to existing ADRs use the new paths, and
  constraint C5 adopts the doc-comment rules.
- Observation: pull request #51 gave the root package `rust-version = "1.85"`
  and added an `msrv` CI job that runs
  `cargo +1.85.0 check --lib --all-features --locked --manifest-path
  "$GITHUB_WORKSPACE/Cargo.toml"`
  from outside the checkout. It also moved the pinned nightly to
  `nightly-2026-10-11`, changed the Whitaker recipe from `env -u` to `unset`,
  and stopped Markdownlint flagging hard tabs in code blocks. Evidence:
  `git diff 42ca1cd origin/main` on 2026-10-22. Impact: the workspace
  conversion must keep the MSRV job green (constraint C6, EP-M0), and the
  `kani` recipe mirrors the current Whitaker recipe.

## Decision log

Each decision below was proposed by the planning agent. Decisions that change
or interpret upstream scope needed explicit reviewer acceptance; the reviewer
accepted all of them (D1, D6, D7, D8, D10, D11, D12, D13, and D14) on
2026-10-22.

- Decision D1: create the substrate crates now as Cargo workspace members
  under `crates/`, keeping the root package `cabochon`. EP-M0 records each
  crate's ownership, permitted callers, and reuse policy in
  `docs/cabochon-design.md` and `docs/repository-layout.md` before code is
  written, as roadmap 1.1.3 and `AGENTS.md` require for new abstractions.
  Rationale: the design already names `cabochon-clerestory`,
  `cabochon-escutcheon`, `cabochon-lapidary`, and `cabochon-burin` (Figure 1),
  and constraint C2 can only be enforced across crates. Roadmap 1.1.3 keeps the
  Enfilade, runtime, storage, and portal crates. Alternative rejected: modules
  inside the root crate, which cannot stop `lapidary` importing `clerestory`.
  Accepted by the reviewer on 2026-10-22.
- Decision D2: Lapidary measures length in integer EMU (`i64`, 914,400 per
  inch, 12,700 per point, 36,000 per millimetre, 9,525 per 96-dpi reference
  pixel), bounded to ±2^47, and transforms with a 16.16 fixed-point linear part
  plus an EMU translation. Transform application is checked: a result outside
  the EMU domain is a `GeometryOutOfRange` error, never a wrapped value.
  Decimal lengths such as `10.5pt` are parsed exactly or rejected. Rationale:
  every common unit converts exactly, round trips are exact, the lint baseline
  forbids float arithmetic, and Verus reasons well about integers. Prior art:
  Office Open XML DrawingML (EMU) and FreeType's `FT_Matrix` (16.16).
  Alternative considered: `kurbo` floating-point geometry, which removes
  conversion seams but loses exact round trips and makes proofs much harder. To
  be recorded as ADR 0009.
- Decision D3: Burin uses `vello_cpu` for raster output, `krilla` for PDF,
  `parley` and `fontique` for text layout, `png` for image decoding, and
  `hayro` (development only) to read PDF output back. Rationale: these crates
  share the Linebender geometry and font stack; `krilla` writes tagged PDF and
  PDF/A; `hayro` interprets what `krilla` writes. Alternative considered:
  `tiny-skia` (mature, but no text and a separate stack). This is the hosted
  proving ground's software renderer; the deferred graphics processing unit
  (GPU) renderer decision in `docs/cabochon-design.md` §15 is unaffected. To be
  recorded as ADR 0011.
- Decision D4: Escutcheon resources use KDL 2.0 documents (the `kdl` crate)
  for structure, Project Fluent `.ftl` files for strings, and SVG files for
  icons, compiled into a typed, cross-referenced `ResourceBundle`. Rationale:
  the white paper's own resource sketch
  (`menu main { item command("file.new") }`,
  `references/cabochon-white-paper.md` around line 568) is already KDL-shaped;
  KDL's node model fits menu and dialog trees, keeps spans for diagnostics, and
  its specification is final. Fluent provides fallback chains, arguments,
  bidirectional isolation, and pseudo-localization. Alternatives considered:
  RON (weak diagnostics and formatting preservation), TOML (awkward for deep
  ordered trees), and Qt `.ui` or GtkBuilder XML (toolkit-specific). To be
  recorded as ADR 0010.
- Decision D5: Clerestory owns the furniture model (commands, menus, dialogs,
  focus, modality, roles) and draws furniture inside the window as Lapidary
  scenes. It adapts to hosts through a `HostShell` port; the hosted adapter uses
  `winit` 0.30, `softbuffer`, and `accesskit_winit`. Menus are drawn within
  the window bounds because `winit` 0.30 offers no `xdg_popup`, so windows have
  a minimum size that fits the longest menu. Rationale: GEM-style in-window
  furniture keeps one application contract for hosted and Mullion modes
  (`docs/cabochon-design.md` §8.1). Alternatives considered: native menus
  through `muda` (needs GTK and GTK windows on Linux); GTK4 (pulls a second
  toolkit into a Wayland-native design); Slint, egui, or iced (each duplicates
  Lapidary with its own renderer); and Masonry as the hosted furniture renderer
  (gains text editing and IME from upstream, but adds a second render path
  outside Burin's parity tests). `FrameUpdate` stays at the scene level so the
  Masonry option remains open. To be recorded as ADR 0012.
- Decision D6: "print-oriented output preparation" means a paginated PDF/A-2b
  file with explicit media, trim, and bleed boxes, an sRGB output intent, and a
  `PrintTicket` describing media, copies, and page range. Submitting jobs to
  the Common UNIX Printing System (CUPS) or the print portal remains with
  roadmap 5.1.2; PDF/X is deferred. Rationale: `krilla` does not support PDF/X,
  and ADR 0003 routes host printing through portals. Accepted by the reviewer
  on 2026-10-22.
- Decision D7: Burin ships no SVG workstation in this step. Rationale: roadmap
  1.0.4 omits SVG and the design says "SVG where appropriate". EP-M8 records
  the deferral in `docs/cabochon-design.md` §15. Accepted by the reviewer on
  2026-10-22.
- Decision D8: the minimum standard dialogs are alert, confirmation, and
  About, all resource-defined. File open and save dialogs are not part of this
  contract; they arrive with the portal work in roadmap 5.1.2, and the
  hello-window export command takes its output directory from the command line.
  Rationale: `docs/cabochon-design.md` §8.10 assigns file selection to the
  portal adapter, and 1.0 must not depend on it. Accepted by the reviewer on
  2026-10-22.
- Decision D9: Lapidary owns resolution: flattening a scene, in scene space
  (EMU), into a `ResolvedScene` whose items carry composed transforms,
  effective clips, conservative bounds, and a group hierarchy. Lapidary also
  owns hit testing, damage, accessibility-node extraction, and the
  `DeviceMapping` that relates scene space to device pixels at a given scale.
  Text shaping enters through a Lapidary-owned `TextLayout` port that Burin's
  `parley` adapter implements. Every Burin workstation consumes the same
  `ResolvedScene`; Clerestory hit-tests and extracts accessibility nodes
  without Burin. Rationale: parity becomes structural, and C2 holds.
- Decision D10: screen output uses a 96-dpi reference pixel scaled by the
  host's device scale, expressed as an integer in 1/120 units as in the Wayland
  `wp_fractional_scale_v1` protocol. Device snapping snaps edges, not sizes, to
  avoid seams at 125% and 150%, and a scale change damages the whole window.
  Rationale: compositors do not report trustworthy physical dots per inch, and
  1/120 steps represent every scale GNOME and KDE Plasma offer. Headless
  `weston` reports integer scales only, so fractional scales are tested through
  the headless shell and remain a hosted residual for roadmap 5.1.3. Accepted
  by the reviewer on 2026-10-22.
- Decision D11: prove the Lapidary kernels from a single source if possible.
  The EP-M0 spike tests the hypothesis that a kernel module written inside
  `verus!`, depending on the crates.io `vstd` and `verus_builtin_macros`
  releases that match the pinned Verus binary, compiles under the repository's
  `rustc`, Clippy, and Whitaker gates with ghost code erased, and verifies under
  `make verus`. If the spike fails, fall back to a spec mirror under `verus/`
  with a parser-based drift guard (see `OB-PROOF-MIRROR`). Either way, kernels
  are total functions with no preconditions: checked arithmetic and early
  returns happen inside the proved region. Accepted by the reviewer on
  2026-10-22, including the production dependency on `vstd` if the spike
  succeeds.
- Decision D12: substrate crates never depend on Enfilade. Later integration,
  such as Clerestory presenting Enfilade-discovered tools
  (`docs/cabochon-design.md` §8.1) or Lapidary scenes embedding presentations,
  lives in bridge crates that depend on both. Figure 1 of the design draws
  `Clerestory --> Enfilade` and `Lapidary --> Enfilade`; EP-M8 proposes
  redrawing those edges through a bridge. Accepted by the reviewer on
  2026-10-22.
- Decision D13: drag-and-drop and host clipboard integration (Clerestory
  §8.1) are deferred. Burin's clipboard geometry workstation is delivered;
  copying to the host clipboard needs a host clipboard adapter that `winit`
  does not provide. EP-M8 proposes a roadmap task for it under step 5.1.
  Accepted by the reviewer on 2026-10-22.
- Decision D14: Clerestory includes a single-line text field with input
  method editor (IME) support in this step: `HostEvent` carries IME enable,
  preedit, commit, and disable events; `HostShell` exposes the IME cursor area;
  `Role::TextInput` exists. Rationale: the white paper calls input methods
  non-optional, and adding them after 2.1.3 would change `HostEvent` and the
  focus model. The hello-window inspector uses the field to edit the greeting,
  which also exercises invalidation end to end. Accepted by the reviewer on
  2026-10-22.
- Decision D15: font policy. Burin's `ParleyTextLayout` takes an explicit font
  collection. Furniture always uses the bundled fonts in `cabochon-fonts`.
  Content in `run` uses bundled fonts with system fallback; `export` uses
  bundled fonts only unless `--system-fonts` is passed, and records every font
  and fallback in `export.json`. Tests never use system fonts. A script that no
  available font covers is a `NoFontCovers` layout error, never silent
  missing-glyph boxes. Rationale: reproducible exports and tests, readable text
  in a real session.
- Decision D16: image resources are stored encoded (PNG only in this step)
  with a declared pixel size and a digest; Burin decodes them. Rationale:
  decoding failure becomes a real, testable render failure; PDF can embed the
  original bytes; scene documents stay small.
- Decision D17: a scene may embed another scene through
  `Node::Subscene { origin: OpaqueOrigin, .. }`. Resolved items, hit results,
  and accessibility nodes carry a `NodePath`, so identifiers from embedded
  scenes never collide. `OpaqueOrigin` is a bounded, opaque label, not an
  Enfilade type. Rationale: roadmap 2.1.3 must compose embed presentations into
  a document scene without changing the scene contract.
- Decision D18: the hello-window application is delivered as a stack of pull
  requests, one per milestone, with this plan's PR as the base. Rationale:
  about 15 milestones of up to 3,000 lines each are too much for one review.
- Decision D19: Escutcheon's canonical emitter (writing a bundle back to KDL)
  is deferred; nothing in roadmap 1.0 or 2.1 consumes it. Rationale: the
  alternatives panel judged it gold-plating.
- Decision D20: assertion style. Tests use `googletest` matchers
  (`expect_that!`, `elements_are!`, `matches_pattern!`) for structural and
  semantic assertions, `pretty_assertions::assert_eq!` for whole-value equality
  where a diff helps (round trips), and `insta` only for stable,
  reviewer-useful output summaries. `#[gtest]` is placed before `#[rstest]`.

## Outcomes & retrospective

Not started. Populate this section at each milestone boundary and at completion.

## Context and orientation

The repository is at `/home/leynos/.lody/repos/github---leynos---cabochon` in
the worktree used by this branch. Today it holds one Rust package, `cabochon`,
with `src/lib.rs` (a `greet()` function) and `src/main.rs` (which prints
`Hello from Cabochon!`). Everything under `tests/` checks the build standard:
the Makefile, CI workflows, linker provisioning, and release routing. There is
no workspace, no substrate code, and no prover tooling.

The governing documents are:

- `docs/cabochon-design.md` (version 0.2, revised 2026-07-25), especially §3
  (GEM-derived invariants), §7.1 (presentations cross into Lapidary), §8.1 to
  §8.4 and §8.9 (component responsibilities), §11 (failure behaviour), §13.1
  (VDI conformance tests), and §14 (observability).
- `docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md` and
  `docs/adrs/adr-0008-keep-enfilade-out-of-rendering-authority.md`, which make
  the substrate a production boundary and keep Enfilade out of rendering.
- `docs/adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md`,
  which keeps host types out of domain contracts.
- `docs/adrs/adr-0004-require-interoperable-authoring-paths.md`, which requires
  contracts that are neutral to the authoring language.
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
lint suite). `make test` runs `cargo nextest` (or `cargo test`) with
`--workspace --all-targets --all-features`, then the doctests. The Makefile
already passes `--workspace`, so new member crates join every gate
automatically. `.cargo/config.toml` selects the Cranelift backend, the parallel
frontend, and the pinned `mold` linker. Rust compilation on the development
host passes through a shared admission queue (`build-limits-rustc`), so builds
may wait for a slot; a waiting build is not hung.

Terms used throughout this plan:

- EMU (English Metric Unit): an integer length; 914,400 EMU make one inch.
- Fixed16: a signed 16.16 fixed-point number stored in an `i32`, where 65,536
  represents 1.0.
- Scene: a Lapidary tree of groups, shapes, text runs, images, hit regions,
  and embedded scenes, with page geometry, layers, and an output intent.
- Scene space: the scene's own coordinate system, measured in EMU.
- Device mapping: the translation and scale (in 1/120 units) that relate scene
  space to device pixels for one surface.
- Resolved scene: the ordered, flattened display list produced from a scene,
  in which every item carries its path, composed transform, effective clip, and
  conservative integer bounds, plus an index of group spans.
- Workstation: a Burin output target, such as interactive display, PDF,
  print, raster, thumbnail, accessibility extraction, clipboard, or hit-test
  map.
- Damage: the set of scene-space rectangles that must be redrawn after a scene
  changes; device snapping turns it into whole-pixel rectangles.
- Host shell: the desktop that owns windows and input in hosted mode, such as
  GNOME or KDE Plasma.
- Headless shell: a Clerestory adapter that runs applications without a
  display, recording frames, accessibility trees, and dispatched commands.
- IME (input method editor): host software that composes text, for example
  Chinese or Japanese characters, through intermediate "preedit" text before
  committing it.
- Spec mirror: a Verus copy of a production function, proved correct and kept
  identical to the production code by a drift-guard test.

Skills and references to load while implementing: `execplans` (this plan),
`rust-router` (then `rust-types-and-apis`, `rust-errors`, `rust-state-machines`
for focus, modality, menus, and text editing, and `arch-crate-design`),
`hexagonal-architecture` (ports and adapters), `rust-unit-testing` (`rstest`,
`googletest`, `pretty_assertions`, `insta`), `proptest`, `kani`, `verus`,
`rust-verification`, `arch-decision-records` for ADRs 0009 to 0012, `nextest`
for the test configuration, `en-gb-oxendict-style` for prose, `codegraph-mcp`
for navigation, and `firecrawl-mcp` for upstream documentation. Repository
references: `docs/rust-testing-with-rstest-fixtures.md`,
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
- ADRs: ADR 0002, ADR 0004, ADR 0007, and ADR 0008, all accepted.
- Roadmap: `RM-1.0.1`, `RM-1.0.2`, `RM-1.0.3`, and `RM-1.0.4`.
- Governing standards: `AGENTS.md`, `docs/documentation-style-guide.md`, and
  the lint baseline in `Cargo.toml` and `docs/developers-guide.md`.

Trace chains (upstream, then milestone, then acceptance evidence):

```plaintext
RM-1.0.3 -> TDD-LAP, TDD-VDI-RT -> EP-M1 -> lapidary tests/round_trip.rs, OB-LAP-ROUNDTRIP
RM-1.0.3 -> TDD-LAP, TDD-VDI-INVAL -> EP-M2a, EP-M2b, EP-M3b -> OB-LAP-DAMAGE, OB-LAP-SNAP
RM-1.0.3 -> TDD-VDI-INVAL -> EP-M3b -> OB-BUR-INVAL, OB-BUR-LOCALITY
RM-1.0.3 -> TDD-LAP (hit regions) -> EP-M2a, EP-M2b -> OB-LAP-HIT, OB-LAP-RESOLVE
RM-1.0.3, RM-1.0.4 -> TDD-BUR, TDD-PRES, TDD-VDI-PARITY, TDD-VDI-PROP -> EP-M3a..EP-M4b
    -> OB-BUR-PARITY-*, OB-BUR-NODROP
RM-1.0.4 -> TDD-FAIL-RENDER, TDD-VDI-FAIL -> EP-M3c -> OB-BUR-FAIL, output.feature
RM-1.0.4 -> TDD-VDI-TEXT -> EP-M3a, EP-M4b -> OB-TEXT-CARET, OB-TEXT-BIDI, OB-TEXT-FALLBACK, OB-BUR-PARITY-TEXT
RM-1.0.2 -> TDD-ESC, TDD-INV-4 -> EP-M5, EP-M7a -> OB-ESC-*, resources.feature, hello-window e2e
RM-1.0.1 -> TDD-CLR, TDD-INV-1, TDD-INV-6 -> EP-M6a, EP-M6b, EP-M7b -> OB-CLR-*, hello_window.feature, make test-hosted
TOR-SM-GEM -> RM-1.0.1..4 -> EP-M7a, EP-M7b -> hello-window e2e (run, export, locale), OB-CLR-HOST
TDD-INV-5, ADR-0007, ADR-0008 -> EP-M0 -> tests/substrate_dependency_contract.rs, OB-ARCH-DEPS
ADR-0004 -> EP-M5, EP-M8 -> escutcheon-check binary, schema references in docs
TDD-OBS -> EP-M3c, EP-M5, EP-M6a -> tracing and metrics assertions in each crate's tests
```

Gaps between the baseline and the target, and where each closes:

- No workspace or substrate crates: EP-M0.
- No scene model, units, layers, or scene documents: EP-M1.
- No resolution, hit testing, invalidation, or proofs: EP-M2a and EP-M2b.
- No renderer, outputs, or parity evidence: EP-M3a to EP-M4b.
- No resource format: EP-M5.
- No application contract, text input, or hosted adapter: EP-M6a to EP-M7b.
- No ADRs for units, resources, renderer, or furniture: ADRs 0009 to 0012,
  drafted in EP-M0 as `Proposed` and accepted at the milestone that implements
  each.

## Verification plan

This section names every obligation before the implementation is shaped around
it. For each obligation: the statement, the method, why that method is
proportionate, the domain, the artefact, the evidence, and the non-vacuity
argument. The lemma sketches are hypotheses about proof structure; the
implementation may show that some are unnecessary or need decomposing.

### Where formal verification is warranted

Verus is used for five small integer kernels in Lapidary whose correctness must
hold for scenes of any size, and on which every later application relies:
transform bounds, polygon winding with its bounding-box prefilter, topmost hit
scanning, damage soundness, and device snapping. These are contractual:
Clerestory routes pointer input through hit testing, and Burin's interactive
surface trusts damage and snapping to decide what to redraw. Rectangle algebra
appears only as supporting lemmas, valued mainly for overflow freedom. Kani
covers bounded arithmetic safety: unit conversion, decimal length parsing,
fixed-point overflow and floor identity, and the float-boundary domain.

Everything else uses example, parameterized, property, state-machine, snapshot,
behavioural, and end-to-end tests. Resource validation, focus and menu state
machines, layout, text editing, and cross-target parity depend on real
third-party behaviour that no proof could model, so tests against the real
libraries give the right assurance.

The relationship between proof and test is explicit. Verus establishes the
semantic properties of the kernels for all inputs. Single-source compilation
(or the drift guard) ties the proved code to the production code. Property
tests then check, through the public API, that resolution produces inputs the
kernels' theorems are about (`OB-LAP-RESOLVE`), that the production kernels
behave as proved (executable twins of each theorem), and that the real
rasterizer honours the locality assumption that connects damage, a statement
about bounds, to pixels (`OB-BUR-LOCALITY`). Removing any layer leaves a named
gap.

Non-vacuity mechanics: proptest has no built-in case classification, so every
property suite whose non-vacuity depends on case classes runs through a manual
`proptest::test_runner::TestRunner` with a fixed seed, accumulates per-class
counters, and asserts the minimum class counts after the run. Where a class
matters, generators construct it directly (for example, overlapping regions or
points on edges) rather than hoping to sample it.

Test budgets: pure Lapidary and Escutcheon suites run 256 cases. Render suites
run 32 cases with `max_shrink_time` of 30 seconds and belong to a nextest
`render` test group with a slow-test timeout, configured in
`.config/nextest.toml`. `PROPTEST_CASES` may raise budgets for deep runs.
Regression files under `proptest-regressions/` are committed.

### Axioms (trusted external assumptions)

- `A-RASTER-LOCALITY`: `vello_cpu` changes pixels only within a drawn item's
  conservative bounds (which include stroke outsets) dilated by one device
  pixel, and a pixel-aligned rectangular clip stops all writes outside it.
  Exercised by `OB-BUR-LOCALITY`.
- `A-RASTER-DET`: `vello_cpu`, pinned by the testkit constructor to its
  scalar path, single-threaded, is deterministic for identical input across
  machines.
- `A-PDF-PRECISION`: `krilla` writes geometry with the precision the EP-M4a
  spike measures, and `hayro` interprets `krilla`'s content streams faithfully.
  Exercised by `OB-BUR-PARITY-GEOM` and validated against handwritten PDFs in
  EP-M4a.
- `A-TEXT-DET`: `parley` with a fixed font collection and system fonts
  disabled produces identical glyph identifiers and positions for identical
  input.
- `A-KDL`, `A-FLUENT`, `A-USVG`, and `A-PNG`: those parsers conform to their
  specifications for the constructs used.
- `A-WINIT`: `winit` and `softbuffer` deliver input and present buffers as
  documented, and a headless `weston` session represents GNOME and KDE Plasma
  closely enough for a smoke test. Residual: GNOME-specific and KDE-specific
  behaviour, fractional scale, and AT-SPI read-back, which roadmap 5.1.3 and
  5.1.4 cover.
- `A-ACCESSKIT`: `accesskit_winit` publishes the supplied tree to AT-SPI
  faithfully.
- `A-VERUS`: the pinned Verus release and its Z3 solver are sound, and (if
  decision D11's single-source route is taken) `verus_builtin_macros` erases
  only ghost code.

### Lapidary obligations

`OB-LAP-UNITS`, exact and checked unit conversion and parsing.

- Statement: for every `i64` input, each `Emu` constructor (`from_points`,
  `from_millimetres`, `from_inches`, `from_reference_pixels`) returns `Ok(e)`
  with `e` exactly equal to the input times the unit factor when the product
  lies within ±`Emu::LIMIT` (2^47), and otherwise `Err(LengthOutOfRange)`.
  `Emu::parse` accepts a decimal with a unit suffix (`pt`, `mm`, `in`, `px`)
  exactly when the value is a whole number of EMU in range, and otherwise
  returns `InexactLength` or `LengthOutOfRange`. No panic or overflow occurs.
- Method: Kani harnesses (exhaustive for the constructors, whose multipliers
  are constants; bounded to 24-character inputs for `parse`), plus `rstest`
  boundary cases and a property test that `parse(format(e)) == e`.
- Artefact: `crates/cabochon-lapidary/src/units/kani.rs` (behind
  `#[cfg(kani)]`) and `crates/cabochon-lapidary/src/units/tests.rs`.
- Evidence: `make kani` reports every harness `VERIFICATION:- SUCCESSFUL`.
  Red step: the harness first runs against a constructor that multiplies
  without checking, and Kani reports an arithmetic overflow.
- Non-vacuity: harnesses `kani::cover!` both outcomes and the exact boundary
  values. Seeded fault: replacing `checked_mul` with `wrapping_mul` must fail.

`OB-LAP-FIXED`, fixed-point application is overflow-free and floors.

- Statement: `Fixed16::apply(c, e)` computes `floor(c × e / 2^16)` with an
  `i128` intermediate and an arithmetic right shift (never `/`, which truncates
  towards zero), never overflows for any `c` and any `e` in the EMU domain, and
  satisfies `r × 2^16 ≤ c × e < (r + 1) × 2^16`.
- Method: Kani with the CaDiCaL solver and a per-harness timeout; measured
  run times are recorded in `Artefacts and notes`. Monotonicity is not checked
  with Kani (symbolic multiplication makes it slow); Verus proves it in
  `OB-LAP-XFORM-BOUNDS`.
- Artefact: `crates/cabochon-lapidary/src/geometry/fixed/kani.rs`.
- Non-vacuity: `kani::cover!` for negative, zero, and positive coefficients
  and for results that round down. Seeded fault: using `/` instead of the shift
  must break the floor identity for negative products.

`OB-LAP-XFORM-BOUNDS`, transformed bounds are sound for the exact image.

- Statement: for every rectangle `r` and transform `t` whose result is in
  range, `transform_bounds(t, r)` returns a rectangle `[min, max)` such that,
  for every point `p` in `r`, the exact image satisfies
  `2^16 × min.x ≤ xx × p.x + xy × p.y + 2^16 × tx < 2^16 × max.x` (and likewise
  for `y`). Containment of the floored `apply(t, p)` follows as a corollary.
  The function evaluates the four inclusive corners (`max − 1`), takes the
  floor of the minimum exact image and the floor of the maximum exact image
  plus one, and returns `GeometryOutOfRange` when any value leaves the EMU
  domain (two terms of 2^46 plus a 2^47 translation can).
- Lemma sketch: (1) the exact image of each coordinate is a sum of two
  functions, each monotone in one input with direction given by the
  coefficient's sign; (2) so each coordinate attains its extremes over a
  rectangle at corners; (3) taking floor of the minimum and one more than the
  floor of the maximum yields a half-open interval containing every exact
  image; (4) the floored `apply` lies in the same interval.
- Method: Verus. Fallback, which needs approval: Kani over coordinates bounded
  to ±2^20 plus a property test over the full domain.
- Artefact: the transform kernel (D11 decides its location) and
  `verus/lapidary_transform.rs` or its single-source equivalent.
- Evidence: `make verus` passes. Red step: the postcondition is stated before
  the "plus one" step exists, and Verus reports the failure at the half-open
  boundary.
- Non-vacuity: witnesses for rotation-like transforms with mixed-sign
  coefficients. Seeded fault: evaluating only two corners (correct only for
  axis-aligned transforms) must fail.

`OB-LAP-WINDING`, polygon coverage and prefilter soundness.

- Statement: `polygon_covers(poly, p)` computes the non-zero winding number
  with half-open crossing (an edge from `a` to `b` crosses `p`'s scanline when
  `a.y ≤ p.y < b.y` or `b.y ≤ p.y < a.y`) and a strict left-of-edge test using
  exact `i128` cross products. If `p` lies outside the polygon's vertex bounds
  (half-open), the winding number is zero. A polygon built from an axis-aligned
  rectangle covers `p` exactly when `rect_contains` does.
- Lemma sketch: let `s(v)` be `[v.y ≤ p.y]`. (1) Cancellation: each edge
  contributes `s(a) − s(b)` to the signed crossing count, and these sum to zero
  round any closed polygon, including the wrap-around edge (induction over the
  vertex sequence). (2) Right side: if both endpoints satisfy `x ≤ p.x`, the
  strict test never counts the edge. (3) Left side: if both endpoints satisfy
  `x > p.x`, every crossing edge is counted with its sign, so the winding
  equals the cancelling sum from (1), which is zero. (4) Scanline: if `p.y`
  lies outside `[min.y, max.y)`, no edge crosses. (5) Rectangle equivalence,
  which ties hit rules to rectangle containment. Domain bound: coordinates
  below 2^62 in magnitude keep the cross products exact in `i128`; EMU's 2^47
  bound satisfies it.
- Method: Verus (non-linear steps use `nonlinear_arith` on the cross product
  `(b.x − a.x)(p.y − a.y) − (p.x − a.x)(b.y − a.y)`), plus a property test
  against an independent oracle that computes crossing x-coordinates as exact
  rationals.
- Artefact: the winding kernel and `verus/lapidary_winding.rs` or its
  single-source equivalent;
  `crates/cabochon-lapidary/tests/winding_properties.rs`.
- Non-vacuity: generators construct points on edges and vertices, scanlines
  through vertices, horizontal edges, a bow tie, and a pentagram (winding 2).
  Seeded fault: switching to the even-odd rule must fail on the pentagram.

`OB-LAP-HIT`, topmost hit testing.

- Statement: for a resolved hit list `h` in painting order and a scene point
  `p`, `hit_test(h, p)` returns `Some(i)` exactly when `i` is the greatest
  index with `covers(h[i], p)`, and `None` exactly when no entry covers `p`.
  `covers(e, p)` holds when `p` lies in the entry's effective clip (a rectangle
  intersected with a sequence of polygons, every one of which must cover `p`)
  and the entry's own polygon covers `p`.
- Lemma sketch: (1) the reverse scan's loop invariant is that no entry after
  the cursor covers `p`; (2) so the first covering entry found is the greatest;
  (3) skipping an entry whose bounds exclude `p` is sound by `OB-LAP-WINDING`'s
  prefilter lemma; (4) clip intersection is conjunction.
- Method: Verus; a property test comparing the production `hit_test` with an
  oracle that scans every entry and keeps the last hit (using the independent
  winding oracle); named `rstest` cases for nested clips, rotated transforms,
  overlapping regions, embedded scenes (path-valued results), and edges.
- Artefact: the hit kernel and `verus/lapidary_hit.rs` or its single-source
  equivalent; `crates/cabochon-lapidary/tests/hit_test_properties.rs`.
- Evidence: `make verus` and `make test` pass. Red step: the oracle property
  fails against a forward scan that returns the first hit.
- Non-vacuity: the runner's counters show at least 20% of cases with two or
  more covering regions and at least 10% with clip-excluded hits. Seeded fault:
  dropping the clip check must fail both the proof and the property.

`OB-LAP-DAMAGE`, damage soundness.

- Statement: for resolved scenes `old` and `new` and the damage `D` returned
  by `damage(old, new)`, every scene point `q` outside `D` is covered by the
  same sequence of paint items, in the same order, in both scenes.
- Algorithm: each item is interned to a `u32` key by full equality (never by
  digest) over its path, transform, clip, and paint payload. The kernel
  receives the key and bounds slices, finds the longest common prefix of length
  `pre` and then the longest common suffix of length `suf` with
  `pre + suf ≤ min(old.len, new.len)`, and adds the bounds of every item in the
  old middle and the new middle to `D`.
- Lemma sketch: (1) filtering a concatenation equals concatenating the
  filters; (2) if every middle item's bounds lie inside `D`, then for `q`
  outside `D` no middle item covers `q`; (3) prefix and suffix items are equal
  by key; (4) hence the filtered sequences are equal.
- Method: Verus over the kernel; property tests over generated edit sequences
  (insert, remove, move, recolour, retransform, change layer visibility)
  checking that identical scenes give empty damage, that a single insertion
  damages exactly the inserted item's bounds, that interning agrees with full
  equality, and an executable twin of the theorem: for random points `q` outside
  `D`, biased towards `D`'s edges, the covering sequences are equal.
- Artefact: the damage kernel and `verus/lapidary_damage.rs` or its
  single-source equivalent;
  `crates/cabochon-lapidary/tests/damage_properties.rs`.
- Non-vacuity: counters cover each edit kind. Seeded faults: omitting removed
  items' bounds must fail; letting the prefix and suffix overlap must fail on
  `old = [A, A]`, `new = [A]`, where `D` would wrongly be empty.

`OB-LAP-SNAP`, device snapping covers damage.

- Statement: for every scene rectangle `d`, device mapping `m`, and device
  pixel `px` that intersects `d` dilated by one device pixel, `px` lies inside
  `m.snap_out(d)`. Snapping snaps edges (floor the minimum, ceiling the
  maximum), never sizes.
- Method: Verus (small: floor and ceiling of integer quotients are monotone).
- Artefact: the snapping kernel and `verus/lapidary_snap.rs` or its
  single-source equivalent.
- Non-vacuity: witnesses at 100%, 125%, 150%, and 200% scale. Seeded fault:
  rounding both edges to nearest must fail.

`OB-LAP-RESOLVE`, resolution produces the inputs the theorems assume.

- Statement: `resolve` emits items in painting order; each item's transform
  is the composition of its ancestors' transforms; each effective clip is the
  intersection of its ancestors' clips; each item's bounds contain every
  flattened vertex, the stroke outset (half the stroke width times the larger
  of the mitre limit and 1.5, which exceeds the square-cap factor √2), glyph
  ink bounds rather than advance boxes, and image footprints; items in hidden
  layers are absent; embedded scenes contribute namespaced paths; resolution is
  deterministic.
- Method: a property test against a naive recursive oracle, plus named cases
  for every node kind, layer visibility, and embedded scenes.
- Artefact: `crates/cabochon-lapidary/tests/resolve_properties.rs`.
- Non-vacuity: generators include thick strokes with sharp mitre joins,
  rotated nested clips, and embedded scenes reusing node identifiers. Seeded
  fault: bounds that ignore the mitre extension must fail.

`OB-LAP-A11Y`, accessibility nodes. `ResolvedScene::accessibility_nodes()`
returns one node per text run and per annotated group, with bounds equal to the
resolved bounds, a role from the annotation (or `Text` for runs), the
annotation's label or the run's text as its name, and children nested by the
group-span index in painting order. Property test plus an `insta` snapshot of
the hello-window content.

`OB-LAP-ROUNDTRIP`, scene document round trip.

- Statement: for every valid scene `s`, `SceneDocument::decode(encode(s))`
  equals `s`; a document with an unknown major version is rejected with
  `SceneDecodeError::UnsupportedVersion`.
- Method: property test over generated scenes (depth up to 4, up to 24 nodes,
  every node kind including embedded scenes, layers, nested clips and
  transforms, left-to-right and right-to-left text runs, PNG images up to 16 by
  16 pixels, every output intent, and every colour intent); an `insta` snapshot
  of one canonical scene document; named unhappy cases.
- Artefact: `crates/cabochon-lapidary/tests/round_trip.rs`.
- Non-vacuity: counters show every node kind, output intent, and colour
  intent. Seeded fault: skipping the hit-region tag when encoding must fail.

`OB-LAP-VALID`, scene validation. Validation rejects duplicate node identifiers
within one scene, dangling image or layer references, coordinates outside the
EMU domain, degenerate transforms (zero determinant), image byte lengths over
the declared limit, and opaque origins longer than 64 bytes. Parameterized
`rstest` unhappy-path cases, plus a property test that every generated valid
scene passes.

### Text obligations (§13.1 text tests)

`OB-TEXT-CARET`, grapheme-aware caret geometry. Every caret stop that
`ParleyTextLayout` reports falls on a grapheme-cluster boundary, with
`unicode-segmentation` as the oracle, and caret stops are ordered visually
within each run. Named cases include a zero-width-joiner emoji sequence,
combining marks, and the Arabic lam-alef ligature; a property test covers
generated strings.

`OB-TEXT-BIDI`, bidirectional order. Visual run order for mixed-direction text
matches a `unicode-bidi` oracle, including mixed numerals. Named cases plus a
property test.

`OB-TEXT-FALLBACK`, font fallback. Text whose script the primary font lacks
uses the next registered font; a script that no registered font covers yields
`TextLayoutError::NoFontCovers` naming the run, never missing-glyph boxes.
Named cases.

### Burin obligations

Declared tolerances, defined as constants in
`crates/cabochon-burin/src/parity/tolerance.rs` and documented in the Burin
section of `docs/cabochon-design.md`:

- Vector targets (PDF and print): on-curve path points, clip vertices, image
  placements, and glyph origins within the tolerance the EP-M4a spike measures.
  The working target is 127 EMU (0.01 point); fixing the final value is a
  recorded decision.
- Raster targets (interactive, raster, thumbnail): item bounds within one
  device pixel at the render scale, and exact pixel agreement with the
  reference render outside a one-pixel band around every edge. Glyph origins
  are checked through the raster workstation's recording seam (the positions
  passed to the rasterizer after the float boundary), within one device pixel.
- Geometry-only targets (accessibility, clipboard geometry, hit map): exact
  EMU equality with the resolved scene, because no floating-point step is
  involved.
- Text content: exact equality, after normalizing to logical order and NFC,
  between the scene's runs, the accessibility extraction, and the PDF text read
  back through `hayro`.
- Invalidation: bit-exact equality between an incrementally updated frame and
  a full re-render, and zero writes outside the snapped damage, subject to the
  EP-M3b spike.
- Hit map: agreement with Lapidary's exact `hit_test` at every pixel centre
  more than one pixel (plus the curve-flattening tolerance of a quarter pixel)
  from any region or clip edge.

`OB-BUR-PARITY-GEOM` and `OB-BUR-PARITY-TEXT`, cross-target agreement. Every
workstation's output agrees with the shared `ResolvedScene` within the declared
tolerances. PDF and print output are read back through a `hayro-interpret`
`Device` implementation in the testkit that records paths, clips, images, and
glyph origins with their transforms. Method: a property test over generated
scenes (the round-trip generator, bounded to 12 nodes and depth 3), a fixed
corpus of named scenes (`rstest` with `#[files]`), and `insta` snapshots of a
normalized, target-independent summary per workstation (item kinds, rounded
bounds, text, and metadata), never raw bytes. Non-vacuity: counters cover
clipping, rotated transforms, right-to-left text, images, layers, embedded
scenes, and each output intent. Seeded faults: offsetting the PDF text by one
point must fail; shifting a raster shape by two pixels must fail.

`OB-BUR-NODROP`, no silent loss. For every workstation that can be read back,
the number of items of each kind (paths, glyph runs, images) in the output
equals the number in the resolved scene. Property test.

`OB-BUR-HITMAP`, the hit-testing workstation. The hit map lists each hit
region's path, scene-space polygon, and effective clip (exact) and renders an
identifier raster that agrees with `hit_test` away from edges. Property test
plus named cases.

`OB-BUR-A11Y`, the accessibility workstation serializes
`ResolvedScene::accessibility_nodes()` without altering it, and the PDF
workstation's structure tree follows the same nodes. Named cases.

`OB-BUR-CLIP`, clipboard geometry. The clipboard workstation produces a
geometry descriptor with exact scene bounds, a PNG whose pixel size equals the
bounds at the requested resolution (within one pixel), and a PDF whose media
box equals the bounds within the vector tolerance. Named cases.

`OB-BUR-INVAL`, invalidation. For any generated scene and edit sequence of
length up to 8, the interactive surface's incremental frame equals a full
re-render bit for bit, and a sentinel-filled frame shows no writes outside the
snapped damage. Property test.

`OB-BUR-LOCALITY`, raster locality (exercises `A-RASTER-LOCALITY`). For any
generated scene `S` and item `i`, render `S` and `S` with `i` removed or
recoloured; every pixel that differs lies within `i`'s snapped bounds dilated
by one pixel. This connects the damage proof, which is about bounds, to real
pixels. Property test. Non-vacuity: generators include thick strokes with sharp
mitre joins, square caps, combining marks, and colours distinct from the
background. Seeded fault: bounds that ignore the mitre extension must fail.

`OB-BUR-FLOAT`, the float boundary. Conversion to backend coordinates computes
device units of 1/256 pixel (or 1/256 point for PDF) in `i128`, checks that the
magnitude is below 2^24, and only then casts to `f32`, which is exact in that
range; the backend transform applies the constant scale 0.003_906_25 (exactly
1/256). Conversion from `parley`'s `f32` results back to EMU rounds to nearest
and rejects non-finite values. Kani proves the range check and the exactness of
the cast; named cases cover non-finite input.

`OB-BUR-FAIL`, renderer failure. Each failure class (unsupported feature,
undecodable image, image size mismatch, resource limit exceeded, backend error)
yields a bounded `RenderError` with a low-cardinality class, leaves the input
scene's digest unchanged, and never substitutes a rasterized copy for text.
`render_with_fallback` returns a diagnostic scene naming the failed item's
path. Method: named `rstest` cases with a fault-injecting workstation supplied
by dependency injection, plus the behavioural feature below. Layout failures
such as `NoFontCovers` arise in `resolve`, not in Burin, and are covered by
`OB-TEXT-FALLBACK`.

`OB-BUR-PRINT`, print preparation. Print output carries the media, trim, and
bleed boxes from the scene's print intent, an sRGB output intent, excludes
non-printable layers, and passes `krilla`'s PDF/A-2b validator. Named cases
read back through `hayro`.

`OB-BUR-DET`, determinism. With the creation date and document identifier
pinned, rendering the same scene twice with the same fonts produces
byte-identical PNG and PDF output. Named cases; if `krilla` is not
byte-deterministic, record it in `Surprises & discoveries` and compare
normalized content instead.

### Escutcheon obligations

`OB-ESC-REFS`, referential integrity. A compiled bundle contains no dangling
reference: every menu item's command (declared or built in), every command's
label, tooltip, and accessibility strings, every icon, and every dialog
control's command or string resolves. Method: a property test that generates
valid bundles, seeds exactly one dangling reference, and asserts compilation
fails with a diagnostic naming the reference and its source span; named
`rstest` cases; an `insta` snapshot of the rendered diagnostics for a
multi-error file.

`OB-ESC-SHORTCUT`, shortcut uniqueness and syntax. No two commands share a
shortcut within one scope, and `Shortcut::parse(s.to_string()) == s`. Shortcuts
name logical keys; matching falls back to the Latin layout's key for the same
physical key, so Ctrl+Q works on a Cyrillic layout. Property tests plus named
cases.

`OB-ESC-L10N`, localization. Lookup walks the requested locale chain to the
base locale; the base locale must define every referenced key; the
pseudo-locales `en-XA` (accented and lengthened) and `ar-XB` (right-to-left)
transform every string; missing message arguments are reported, not silently
dropped; bidirectional isolation marks are on in the UI and pinned in tests.
Named cases plus the behavioural feature.

`OB-ESC-DET`, determinism. Compilation is independent of directory listing
order, and diagnostics are sorted by source location. Property test with a
shuffling in-memory resource source.

`OB-ESC-VERSION`, schema versioning. A bundle declaring an unknown schema
version is rejected with a located diagnostic. Named case.

### Clerestory obligations

`OB-CLR-FOCUS`, focus and traversal. In every window, at most one element owns
focus, and only a focusable, enabled, visible element can own it. Tab followed
by Shift+Tab returns focus to where it started. A full Tab cycle visits every
focusable element exactly once. Method: `proptest-state-machine` with a
reference model, over operation sequences of length up to 32 (Tab, Shift+Tab,
enable, disable, show, hide, open dialog, close dialog, pointer focus, open
menu, close menu).

`OB-CLR-MODAL`, modality. While a modal dialog is open, neither focus nor input
reaches elements outside it, and closing it restores the previous focus owner.
Same state machine.

`OB-CLR-MENU`, menu navigation. Opening, arrow navigation, activation, and
Escape never leave a menu open while focus sits outside it, and activating a
disabled item dispatches nothing. Same state machine.

`OB-CLR-ROUTE`, command routing. A command reaches the first handler in the
responder chain (focused element, dialog, window, application) that accepts it.
A disabled command is never dispatched. A shortcut and the matching menu item
produce identical dispatch records. A host close request is dispatched as the
vetoable built-in `window.close` command. At session start, every command the
application declares must exist in the bundle, and the session fails fast
otherwise. Named `rstest` cases plus a property test over generated chains.

`OB-CLR-TEXT`, text editing. In the single-line text field, caret movement,
backspace, and delete move by grapheme cluster; preedit text is shown but never
committed without an IME commit event; disabling the IME discards preedit; the
IME cursor area reported to the host contains the caret.
`proptest-state-machine` over key and IME event sequences.

`OB-CLR-LAYOUT`, layout. Children lie inside their parent and do not overlap
their siblings; layout is deterministic; the `en-XA` pseudo-locale's lengthened
strings do not overflow the dialog's minimum size; right-to-left locales mirror
row order. Property test over generated dialog layouts plus `insta` snapshots
of the hello-window dialog layout in `en-GB`, `en-XA`, and `ar-XB`.

`OB-CLR-A11Y`, accessibility tree. Every interactive element has a role and a
non-empty accessible name; parent and child links are mutually consistent; the
focused element appears in the tree; the content canvas contains the scene's
accessibility nodes. Property test over generated resource bundles; the hosted
adapter's conversion is checked by feeding its `TreeUpdate` into
`accesskit_consumer::Tree`.

`OB-CLR-HOST`, hosted conformance. The same scripted session, using the same
`ParleyTextLayout` and bundled fonts, run under the headless shell and under
the hosted shell inside headless `weston`, yields the same accessibility tree
(as handed to AccessKit) and the same command log. End-to-end test run by
`make test-hosted`. Residual: reading the tree back over AT-SPI is deferred to
roadmap 5.1.4.

### Architecture and proof-hygiene obligations

`OB-ARCH-DEPS`, dependency direction and leakage.
`tests/substrate_dependency_contract.rs` runs
`cargo metadata --offline --locked`, considers only normal dependency edges,
and asserts constraint C2, that no substrate crate or `cabochon-hello-window`
depends on a crate whose name contains `enfilade`, `runtime`, or `portal`, and
an allowlist of third-party crates per contract crate: no `winit`, `softbuffer`,
`accesskit*`, `kurbo`, `parley`, `krilla`, `vello*`, `kdl`, `fluent*`,
`unic-langid`, or `miette` in `cabochon-lapidary` or `cabochon-clerestory`, and
only the hosted crate may depend on `winit`, `softbuffer`, or
`accesskit_winit`. Non-vacuity: the test also asserts that each expected edge
exists, so an empty metadata read cannot pass.

`OB-ARCH-FEATURES`, no member features. The existing
`the_crate_declares_no_features` test in `tests/workflow_suite_contract.rs` is
extended to read every workspace member's manifest.

`OB-PROOF-MIRROR`, proof hygiene. `tests/verus_mirror_contract.rs` asserts that
the proof sources contain none of the constructs banned by C8, that each proved
kernel region sits inside `verus!` and carries an `ensures` clause, and that at
least five kernel regions exist. If decision D11 falls back to spec mirrors, it
also parses both copies with `proc-macro2` token streams, erases ghost syntax
by a documented rule (remove `requires`, `ensures`, `decreases`, and loop
`invariant` clauses, and `proof { ... }` blocks), and asserts token identity of
each production region and its mirror.

`OB-PROVER-TOOLING`, prover tooling contract.
`tests/prover_tooling_contract.rs` checks that `tools/verus/VERSION`,
`tools/verus/SHA256SUMS`, and `tools/rust-prover-tools/REF` are well formed,
that the `verus` and `install-verus` recipes delegate through `$(PROVER_TOOLS)`
and contain no `curl`, `unzip`, or `sha256sum`, and, with a recording fake
supplied as `PROVER_TOOLS_CONTRACT_TEST=1 PROVER_TOOLS=<fake>` on the `make`
command line, that `make verus` issues exactly
`prover-tools verus run --repo-root . --proof-file verus/lapidary_proofs.rs`
with no extra arguments.

## Plan of work

The work runs in stages. Stage A, understanding and proposal, is this plan.
Each milestone then follows Red-Green-Refactor: write the smallest failing
test, behavioural scenario, Kani harness, or Verus lemma statement first, and
run it to watch it fail for the expected reason; write the smallest
implementation that turns it green; then refactor with the focused tests and
the full gates rerun. Verification artefacts are written with the
implementation, not after it. Every milestone ends with `make check-fmt`,
`make lint`, and `make test` passing (run sequentially, through the
`scrutineer` agent), plus `make kani` and `make verus` where the milestone says
so, and a commit. Each milestone is one stacked pull request (decision D18).

### EP-M0: workspace, tooling, architecture guards, and spikes

Workspace. Convert `Cargo.toml` to a workspace whose root package remains
`cabochon` and keeps its `rust-version = "1.85"`. Each member declares its own
`rust-version`, no lower than the highest `rust-version` among its
dependencies. After the conversion, run the `msrv` CI job's command locally
from outside the checkout and confirm it passes:

```bash
cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml"
```

The workspace stanza:

```toml
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.package]
edition = "2024"
license = "ISC"
repository = "https://github.com/leynos/cabochon"

[workspace.lints.rust]
# the existing [lints.rust] entries, unchanged, plus:
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)'] }

[lints]
workspace = true
```

Move the existing `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]`
tables to `[workspace.lints.*]` unchanged. Every member declares
`[lints] workspace = true` and `publish = false`. Add
`[workspace.dependencies]` entries, each with a caret requirement, in the
commit of the milestone that first uses them. Create the eight member crates
with `//!` module documentation and empty public surfaces: `cabochon-lapidary`,
`cabochon-burin`, `cabochon-escutcheon`, `cabochon-clerestory`,
`cabochon-clerestory-hosted`, `cabochon-hello-window`, `cabochon-fonts` (data
only: the bundled font bytes and their licences), and
`cabochon-substrate-testkit` (test support, consumed only from integration
tests under each crate's `tests/`, never from unit tests, so no crate's types
appear twice through a development-dependency cycle).

Prover tooling. Add `tools/verus/VERSION` (the latest weekly release when EP-M0
starts, currently `0.2026.10.04.426d8b0`), `tools/verus/SHA256SUMS`,
`tools/verus/README.md`, `tools/rust-prover-tools/REF` (`ref: <40-hex>`), and
`.verus/` in `.gitignore` before the first install, so
`mdtablefix --git --include-untracked` never rewrites Verus's bundled Markdown.
Add these Makefile targets, none of which joins `make all`, `make test`, or
`make lint`:

```makefile
override PROVER_TOOLS_REF_FILE := tools/rust-prover-tools/REF
override PROVER_TOOLS_REF := $(shell awk '/^ref:/ { print $$2 }' $(PROVER_TOOLS_REF_FILE))
ifeq ($(shell printf '%s' '$(PROVER_TOOLS_REF)' | grep -Ec '^[0-9a-f]{40}$$'),0)
$(error PROVER_TOOLS_REF missing or malformed; expected "ref: <40-hex>" in $(PROVER_TOOLS_REF_FILE))
endif
override PROVER_TOOLS_SOURCE := git+https://github.com/leynos/rust-prover-tools.git@$(PROVER_TOOLS_REF)
PROVER_TOOLS_ORIGINS := $(origin PROVER_TOOLS_CONTRACT_TEST),$(origin PROVER_TOOLS)
ifeq ($(PROVER_TOOLS_ORIGINS),command line,command line)
else
override PROVER_TOOLS := $(UVX) --from "$(PROVER_TOOLS_SOURCE)" prover-tools
endif
VERUS_PROOF_FILE ?= verus/lapidary_proofs.rs

install-verus: ## Install the pinned Verus release
	$(PROVER_TOOLS) verus install --repo-root .

verus: ## Run the Verus proofs
	$(PROVER_TOOLS) verus run --repo-root . --proof-file "$(VERUS_PROOF_FILE)"

kani: ## Run the Kani harnesses for the Lapidary kernels
	cd "$(dir $(CURDIR))" && unset CARGO_ENCODED_RUSTFLAGS CARGO_PROFILE_DEV_CODEGEN_BACKEND \
		CARGO_PROFILE_TEST_CODEGEN_BACKEND CARGO_PROFILE_RELEASE_CODEGEN_BACKEND && \
		RUSTFLAGS="" cargo kani --manifest-path "$(CURDIR)/Cargo.toml" -p cabochon-lapidary
```

The `kani` recipe mirrors the `lint-whitaker` recipe in `Makefile` (run from
the parent directory and `unset` the inherited backend variables, so
`.cargo/config.toml`'s Cranelift and `mold` settings do not reach
`kani-compiler`). Until EP-M1, `verus/lapidary_proofs.rs` is an explicit
`FORMAL-SKIP` stub that prints `FORMAL-SKIP: no Verus proofs yet`. Add
`make test-hosted` as a stub that fails with "not yet implemented (EP-M7b)". Add
`verus`, `install-verus`, and `kani` to the held-out target list and
`test-hosted` to the development target list in
`tests/build_standard_support/make.rs`, and every new target to `.PHONY`.

Test infrastructure. Add `.config/nextest.toml` with a `render` test group and
slow-test timeouts. Add one `rstest-bdd` smoke scenario in the testkit and one
`#[gtest]` plus `#[rstest]` test to prove the harnesses compose.

Guards first. Write `tests/substrate_dependency_contract.rs` (`OB-ARCH-DEPS`),
`tests/prover_tooling_contract.rs` (`OB-PROVER-TOOLING`), and
`tests/verus_mirror_contract.rs` (`OB-PROOF-MIRROR`, whose minimum-region
assertion is enabled in EP-M2b), and extend `OB-ARCH-FEATURES`. Each fails
before the workspace and tooling exist.

Spikes (each records its hypothesis, command, and result in
`Artefacts and notes`; a rejected hypothesis triggers the spike tolerance):

1. S-VERUS: a toy kernel with `i128` multiplication, an arithmetic shift,
   slices, and a `while` loop verifies under the pinned Verus, and (decision
   D11) the same file, written inside `verus!` with the matching crates.io
   `vstd` and `verus_builtin_macros`, compiles and passes `make lint` and
   `make test` under the repository toolchain. Also records which Rust
   toolchain the pinned Verus requires and whether `rust-toolchain.toml`
   interferes.
2. S-KANI: `make kani` runs one trivial harness through the
   `build-limits-rustc` admission wrapper.
3. S-VELLO: `vello_cpu`, pinned to its scalar path, renders a rectangle under
   `make test` (Cranelift) and under `make coverage` (LLVM) with identical
   pixels.

Documentation. Draft ADRs 0009 to 0012 with status `Proposed`, as
`docs/adrs/adr-0009-<title-slug>.md` and so on, following the naming convention
and template in `docs/documentation-style-guide.md`. Record each crate's
ownership, permitted callers, and reuse policy in `docs/cabochon-design.md` (a
new subsection of §8) and `docs/repository-layout.md`. Update
`docs/developers-guide.md` ("Lint baseline" now describes `[workspace.lints]`;
new sections on the workspace, the dependency rule, test conventions and
budgets, and the prover tooling), and `docs/contents.md`. Pull request #51 has
already merged the `typos.toml` regeneration, so `make spelling` should leave
`typos.toml` unchanged; if a later dictionary change regenerates it again,
handle it as `AGENTS.md` directs.

### EP-M1: Lapidary scene model and scene documents

Implement, in this order, each behind red tests: `units` (`Emu`, constructors,
`Emu::parse`); `geometry` (`Point`, `Size`, `Rect`, `Fixed16`, checked
`Transform`, `Path` built from move, line, quadratic, cubic, and close segments,
`FillRule`); `paint` (`Colour` as sRGB with 16-bit channels and alpha, `Paint`,
`Stroke` with width, joins, caps, and a Fixed16 mitre limit); `text`
(`TextRun`, `FontRequest`, `TextDirection`, `LanguageTag`); `image`
(`ImageResource`: PNG bytes, declared pixel size, intrinsic physical size,
digest); `layer` (`LayerSpec` with name, visible, and printable flags); `scene`
(`Scene`, `Node`, `NodeId`, `Group`, `Shape`, `TextNode`, `ImageNode`,
`HitRegion`, `HitRegionId`, `Subscene`, `OpaqueOrigin`, `Annotation`,
`SemanticRole`, `PageGeometry`, `OutputIntent`, `ColourIntent`, `PrintIntent`);
`validate`; and `document` (`SceneDocument`, version 1, JSON through
`serde_json`, with image bytes in base64). Write the Kani harnesses for
`OB-LAP-UNITS` and `OB-LAP-FIXED` with the code they check. Put the scene
generator in the testkit. Write a prose reference for scene document version 1
for authors in other languages (ADR 0004).

### EP-M2a: resolution, device mapping, hit testing, damage, and accessibility nodes

Define the `TextLayout` port and `FontSet` in Lapidary, and a deterministic
`MonospaceTextLayout` fake in the testkit for Lapidary's own tests. Implement
`resolve(scene, layout)` producing a `ResolvedScene` in scene space: an ordered
list of `ResolvedItem`s (each with its `NodePath`, composed `Transform`,
effective clip of a rectangle plus a sequence of polygons, conservative bounds,
and paint payload), the group-span index, the used `FontSet`, and the hit list
(curves flattened with integer de Casteljau subdivision to a declared
tolerance). Implement `DeviceMapping` with `scene_to_device`,
`device_to_scene`, and `snap_out`. Implement `hit_test`, `hit_test_all`,
`damage`, and `accessibility_nodes`, with the five kernels in
`crates/cabochon-lapidary/src/kernel/` as plain, total Rust (no proofs yet, but
shaped for them: integer-only, slices, `while` loops, no preconditions). Write
the property tests and named cases for `OB-LAP-WINDING`, `OB-LAP-HIT`,
`OB-LAP-DAMAGE`, `OB-LAP-RESOLVE`, and `OB-LAP-A11Y`, and
`crates/cabochon-lapidary/tests/features/scene.feature`.

### EP-M2b: Verus proofs for the Lapidary kernels

Following decision D11's spike outcome, write the lemma statements first and run
`make verus` to see the expected open goals; then write the proofs for
`OB-LAP-XFORM-BOUNDS`, `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`, and
`OB-LAP-SNAP`, with rectangle algebra as supporting lemmas. Proof files live
under `verus/` (`lapidary_proofs.rs` includes `lapidary_rect.rs`,
`lapidary_transform.rs`, `lapidary_winding.rs`, `lapidary_hit.rs`,
`lapidary_damage.rs`, and `lapidary_snap.rs`), or, on the single-source route,
inside the kernel modules with `verus/lapidary_proofs.rs` as the entry point.
Enable `OB-PROOF-MIRROR`'s minimum-region assertion. Accept ADR 0009. Record
the proof inventory, and what each proof does not cover, in
`docs/developers-guide.md`. Add a CI workflow, `formal.yml`, triggered by
`workflow_dispatch` and a weekly schedule, that runs `make kani` and
`make verus`; it must satisfy the existing workflow contract tests.

### EP-M3a: Burin foundations, text layout, raster, and thumbnail

Add the bundled fonts to `cabochon-fonts` (static subsets of Noto Sans, Noto
Sans Arabic, and Noto Sans Hebrew, each with `OFL.txt` and a `SHA256SUMS`
entry). Implement `Workstation` with an associated `Request` type,
`RenderError`, the float-boundary module (`OB-BUR-FLOAT`), `ParleyTextLayout`
with an injected font collection (`OB-TEXT-CARET`, `OB-TEXT-BIDI`,
`OB-TEXT-FALLBACK`), PNG decoding, `RasterWorkstation`, and
`ThumbnailWorkstation`. Add the testkit render constructor that pins
determinism. Measure cold and warm build times and record them; propose an
optimization-level override for the heavy crates only if the measurement
justifies it.

### EP-M3b: interactive surface and invalidation

First run the bit-exactness spike (S-INVAL: a damage-clipped re-render equals a
full re-render for one recolour edit). Then implement `InteractiveSurface`, a
stateful surface with `redraw(&mut self, scene, damage, mapping)` that redraws
only snapped damage and records written spans through a `FrameSink`. Write
`OB-BUR-INVAL` and `OB-BUR-LOCALITY`.

### EP-M3c: hit map, accessibility, clipboard geometry, and failure handling

Implement `HitMapWorkstation`, `AccessibilityWorkstation`, the PNG and geometry
parts of `ClipboardWorkstation`, `render_with_fallback`, and the
fault-injection seam. Instrument rendering with a `tracing` span per
workstation and the metrics in `Interfaces and dependencies`. Write
`OB-BUR-HITMAP`, `OB-BUR-A11Y`, `OB-BUR-CLIP`, `OB-BUR-FAIL`, the raster half of
`OB-BUR-PARITY-*` and `OB-BUR-NODROP`, and the output behavioural feature's
failure scenario.

### EP-M4a: PDF read-back device and precision spike

Implement the `hayro-interpret` recording `Device` in the testkit and validate
it against handwritten PDFs whose geometry and text are known exactly. Run the
precision spike (S-PDF): render the corpus through `krilla`, read it back, and
measure the maximum on-curve and glyph-origin error and whether `hayro` returns
right-to-left and ligated text in logical order. Record the measured values and
propose the final vector tolerance for approval.

### EP-M4b: PDF and print workstations and the full parity suite

Implement `PdfWorkstation` (tagged PDF whose structure follows the
accessibility nodes, document metadata, real text drawn through `krilla`'s
positioned-glyph API from the shared layout, original PNG bytes, clips, and
non-printable layers retained but marked), `PrintWorkstation` (`PrintIntent`
page boxes, the PDF/A-2b validator, an sRGB output intent, a `PrintTicket`,
non-printable layers excluded), and the PDF part of `ClipboardWorkstation`.
Write `OB-BUR-PARITY-GEOM`, `OB-BUR-PARITY-TEXT`, the vector half of
`OB-BUR-NODROP`, `OB-BUR-PRINT`, `OB-BUR-DET`, and the remaining output feature
scenario. Accept ADR 0011. Mark roadmap tasks 1.0.3 and 1.0.4 done.

### EP-M5: Escutcheon resource format and compiler

Implement the `ResourceSource` port (a `cap_std` directory adapter and an
in-memory adapter), the KDL schema version 1 parser, the catalogue of built-in
commands (`dialog.ok`, `dialog.cancel`, `window.close`, `edit.copy`), the
Fluent localizer with locale-chain fallback and the `en-XA` and `ar-XB`
pseudo-locales, icon compilation from SVG to Lapidary paths through `usvg` (its
`text` feature disabled) in the float-boundary module, the shortcut grammar
with logical-key matching, accessibility annotations on commands and controls,
and validation with located diagnostics owned by Escutcheon (`Diagnostic` with
span, code, and message; no `miette` in the public API). Add the
`escutcheon-check` binary. Write the obligations and the behavioural feature.
Write a prose schema reference for authors in other languages. Accept ADR 0010.
Roadmap task 1.0.2 stays open until EP-M7a.

### EP-M6a: Clerestory session, state machines, routing, and the headless shell

Implement the `Application` trait, `ContentFrame`, `ViewContext`,
`CommandContext` (with `invalidate`), the `Session` that owns windows, menus,
dialogs, focus, modality, and command routing, the `HostEvent` enumeration, the
`HostShell` and `SessionWaker` ports, and the `HeadlessShell` adapter in
`cabochon_clerestory::shell::headless` (it has no infrastructure dependencies
and serves as the reference adapter). Write `OB-CLR-FOCUS`, `OB-CLR-MODAL`,
`OB-CLR-MENU`, and `OB-CLR-ROUTE`.

### EP-M6b: layout, furniture, text input, inspector, and accessibility tree

Implement the layout engine (rows, columns, labels, buttons, text fields,
separators, and property rows, measured through the `TextLayout` port, and
mirrored for right-to-left locales), the furniture scene builders that draw the
menu bar, menus, dialogs, and inspector through Lapidary, the single-line text
field with IME support, the inspector panel bound to an `InspectorModel`, and
the `AccessibilityTree`, which grafts the content scene's accessibility nodes
under the canvas. Write `OB-CLR-TEXT`, `OB-CLR-LAYOUT`, `OB-CLR-A11Y`, and
`hello_window.feature` against the headless shell. Accept ADR 0012.

### EP-M7a: the hello-window application under the headless shell

Implement `cabochon-hello-window`: resources under
`crates/cabochon-hello-window/resources/` (loaded from the bundle directory at
start, with an embedded copy as the default), the `run` and `export`
subcommands, `--locale`, `--shell headless`, `--script`,
`--exit-after-first-frame`, `--system-fonts`, a `tracing` subscriber writing to
standard error, and an environment reader through `mockable`. `export` refuses
a non-empty directory that it did not create unless `--force` is given, writes
into a temporary directory, and renames it into place. Write the end-to-end
tests for `run` (scripted, headless), `export`, and locale switching. Mark
roadmap task 1.0.2 done. Correct the `mockable` version in
`docs/reliable-testing-in-rust-via-dependency-injection.md`.

### EP-M7b: the hosted shell adapter and its CI job

First, provisioning: confirm `weston` is available in CI (Ubuntu package) and
locally (EPEL or a rootless Podman image); otherwise the hosted tolerance
triggers. Implement `cabochon-clerestory-hosted`: the `winit` event loop
(forcing the Wayland backend in tests), `softbuffer` presentation of
`InteractiveSurface` frames with damage, scale handling in 1/120 units, key,
pointer, and IME mapping, close requests, client-side decorations where the
compositor requires them, and `accesskit_winit` publication. Keep all mapping
in pure, unit-tested functions. Hosted tests carry
`#[ignore = "needs Wayland; run make test-hosted"]`. Replace the
`make test-hosted` stub with a recipe that creates a private `XDG_RUNTIME_DIR`,
starts `weston --backend=headless`, waits for its socket, runs the ignored
tests under `dbus-run-session` with `cargo nextest run --run-ignored only`, and
stops only the `weston` process it started. Add a CI job that installs `weston`
and `at-spi2-core`, sets up Rust through `setup-rust` with
`install-mold: 'true'`, and runs `make test-hosted`, keeping the existing
workflow contract tests green. Mark roadmap task 1.0.1 done.

### EP-M8: documentation reconciliation and roadmap closure

Reconcile every discovery with the upstream documents: update
`docs/cabochon-design.md` §§8.1 to 8.4 and §13.1 with the contracts,
tolerances, and proof inventory; propose the Figure 1 bridge edges (decision
D12) and the §15 deferrals (decisions D6, D7, D8, and D13); update
`docs/users-guide.md` with the hello-window application; finish
`docs/developers-guide.md`; propose the roadmap task for host clipboard and
drag-and-drop integration; mark step 1.0 complete in `docs/roadmap.md`; and set
this plan's status to `COMPLETE`.

## Milestones and plateaus

Every milestone's conformance check asks: are its assigned requirements
satisfied; is the design still followed; are upstream assumptions still valid;
has no unapproved public interface, dependency, trust boundary, or persisted
format appeared; and are trace links current. The entries below list only what
is specific to each milestone. Every milestone's recovery path is to revert its
commits (dependency additions are separate commits, so they revert cleanly) and
its compatibility decision is "none": every interface is pre-1.0 and internal.

- EP-M0. Outcome: a workspace with eight empty member crates, workspace
  lints, prover tooling, guards, and three spike results. Evidence: all gates
  pass; `make verus` prints the `FORMAL-SKIP` line; `make kani` runs the
  trivial harness; `cargo tree -p cabochon` is unchanged. Specific check:
  ownership and reuse policies are documented before code. Gaps: all substrate
  behaviour.
- EP-M1. Outcome: Lapidary models scenes, units, layers, embedded scenes,
  and output intent, validates them, and round-trips them. Requirements:
  `RM-1.0.3` (data model), `TDD-VDI-RT`. Evidence: `OB-LAP-UNITS`,
  `OB-LAP-FIXED`, `OB-LAP-VALID`, and `OB-LAP-ROUNDTRIP` pass under `make test`
  and `make kani`. Specific check: no floats; no dependency on other Cabochon
  crates; the scene document is labelled a scene format.
- EP-M2a. Outcome: Lapidary resolves, maps, hit-tests, damages, and extracts
  accessibility nodes. Requirements: `RM-1.0.3` (hit regions, invalidation),
  `TDD-LAP`. Evidence: `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`,
  `OB-LAP-RESOLVE`, and `OB-LAP-A11Y` property tests and `scene.feature` pass.
  Specific check: kernels are total and integer-only.
- EP-M2b. Outcome: the five kernels are proved. Evidence:
  `make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs` with empty
  `VERUS_FLAGS` reports zero errors and at least the verified-item count
  recorded at completion; `OB-PROOF-MIRROR` passes with five or more regions;
  `make test` passes. Specific check: no construct banned by C8.
- EP-M3a. Outcome: one scene renders to raster and thumbnail with real text.
  Requirements: `RM-1.0.4` (raster, thumbnail), `TDD-VDI-TEXT`. Evidence:
  `OB-BUR-FLOAT` (Kani), `OB-TEXT-*`, and raster parity cases pass. Specific
  check: floats only in the float boundary.
- EP-M3b. Outcome: damage-aware interactive redraw. Requirements:
  `RM-1.0.3` (invalidation), `RM-1.0.4` (interactive), `TDD-VDI-INVAL`.
  Evidence: S-INVAL result, `OB-BUR-INVAL`, and `OB-BUR-LOCALITY`.
- EP-M3c. Outcome: hit-map, accessibility, and clipboard-geometry outputs and
  bounded failure. Requirements: `RM-1.0.4` (accessibility, clipboard, renderer
  failure), `TDD-VDI-FAIL`, `TDD-FAIL-RENDER`, `TDD-OBS`. Evidence:
  `OB-BUR-HITMAP`, `OB-BUR-A11Y`, `OB-BUR-CLIP`, `OB-BUR-FAIL`, and the raster
  halves of parity and `OB-BUR-NODROP`.
- EP-M4a. Outcome: a validated PDF read-back device and a measured,
  approved vector tolerance. Evidence: handwritten PDF cases pass; S-PDF
  results recorded.
- EP-M4b. Outcome: PDF and print outputs and the full parity suite.
  Requirements: completes `RM-1.0.3` and `RM-1.0.4`, `TDD-VDI-PARITY`,
  `TDD-VDI-PROP`. Evidence: `OB-BUR-PARITY-*`, `OB-BUR-NODROP`, `OB-BUR-PRINT`,
  `OB-BUR-DET`, and `output.feature` pass; roadmap 1.0.3 and 1.0.4 marked done.
  Specific check: decisions D6 and D7 respected.
- EP-M5. Outcome: Escutcheon compiles bundles with located diagnostics,
  localization, icons, shortcuts, and accessibility annotations. Requirements:
  `RM-1.0.2` (format), `TDD-ESC`, `TDD-INV-4`, ADR 0004. Evidence: `OB-ESC-*`
  and `resources.feature` pass; `escutcheon-check` reports a located diagnostic
  for a broken fixture.
- EP-M6a. Outcome: the Clerestory session runs under the headless shell.
  Requirements: `RM-1.0.1` (windows, menus, commands, dialogs, focus,
  traversal), `TDD-CLR`, `TDD-INV-1`. Evidence: `OB-CLR-FOCUS`, `OB-CLR-MODAL`,
  `OB-CLR-MENU`, and `OB-CLR-ROUTE` pass. Specific check: no host types and no
  Burin dependency in Clerestory.
- EP-M6b. Outcome: furniture, text input, inspector, and accessibility tree.
  Requirements: `RM-1.0.1` (inspector panels, accessibility roles),
  `TDD-INV-6`. Evidence: `OB-CLR-TEXT`, `OB-CLR-LAYOUT`, `OB-CLR-A11Y`, and
  `hello_window.feature` pass.
- EP-M7a. Outcome: the hello-window application runs headless and exports.
  Requirements: completes `RM-1.0.2`; `TOR-SM-GEM` (all but the hosted window).
  Evidence: end-to-end tests pass; roadmap 1.0.2 marked done.
- EP-M7b. Outcome: the hello-window application runs under a real host
  compositor. Requirements: completes `RM-1.0.1` and `TOR-SM-GEM`. Evidence:
  `OB-CLR-HOST` via `make test-hosted` locally and in CI; roadmap 1.0.1 marked
  done. Specific check: `winit`, `softbuffer`, and AccessKit appear only in the
  hosted crate; coverage ratchet green.
- EP-M8. Outcome: documentation and roadmap match the delivered contracts.
  Evidence: `make markdownlint`, `make nixie`, `make spelling`,
  `make check-fmt`, `make lint`, and `make test` pass; roadmap step 1.0 shows
  all four tasks done.

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

  Scenario: Embedded scenes keep their identity
    Given a scene that embeds another scene containing hit region "price"
    When a point inside "price" is hit-tested
    Then the result path names the embedded scene and then "price"

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
    And the text read back from the PDF equals the scene's text runs

  Scenario: A renderer failure is bounded and harmless
    Given a scene whose PNG image bytes are corrupt
    When it is rendered to the raster workstation
    Then the result is an "undecodable-image" render error naming the image node
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

  Scenario: The pseudo-locales transform every string
    When the hello-window bundle is compiled for locales "en-XA" and "ar-XB"
    Then every visible string differs from its "en-GB" text
    And the "ar-XB" bundle declares right-to-left direction
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

  Scenario: Input methods compose text before committing it
    Given the inspector's greeting field has focus
    When the input method sends preedit "ni" and then commits "你"
    Then the field shows "你" and the content scene's greeting is updated
    And only the greeting's bounds are damaged

  Scenario: Furniture is accessible
    Given the hello-window application under the headless shell
    Then the accessibility tree contains a window, a menu bar, and menu items
    And every interactive element has a role and a non-empty name
    And the canvas contains the content scene's text
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

Formal gates, from EP-M1 (Kani) and EP-M2b (Verus):

```bash
make kani 2>&1 | tee /tmp/kani-cabochon-${BRANCH}.out
make install-verus
make verus VERUS_PROOF_FILE=verus/lapidary_proofs.rs 2>&1 | tee /tmp/verus-cabochon-${BRANCH}.out
```

Expected Verus result once EP-M2b is complete (the count is recorded then and
becomes the minimum for later runs):

```plaintext
verification results:: <N> verified, 0 errors
```

Focused red and green loops, for example:

```bash
cargo nextest run -p cabochon-lapidary --test hit_test_properties
cargo nextest run -p cabochon-burin --test parity
```

Hosted smoke test, from EP-M7b (requires `weston`, `dbus-run-session`, and
`at-spi2-core`):

```bash
make test-hosted 2>&1 | tee /tmp/test-hosted-cabochon-${BRANCH}.out
```

Expected hosted output:

```plaintext
hello-window: shell=hosted windows=1 frames=1 accessibility_nodes=<N> exit=0
```

Hello-window export, from EP-M7a:

```bash
cargo run -p cabochon-hello-window -- export --out /tmp/hello
ls /tmp/hello
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  export.json  hit-map.json  thumbnail.png
```

## Validation and acceptance

Quality criteria:

- Tests: `make test` passes, including every obligation's tests and every
  behavioural scenario; `make test-hosted` passes locally and in its CI job.
- Verification: `make kani` discharges `OB-LAP-UNITS`, `OB-LAP-FIXED`, and
  `OB-BUR-FLOAT`; `make verus` discharges `OB-LAP-XFORM-BOUNDS`,
  `OB-LAP-WINDING`, `OB-LAP-HIT`, `OB-LAP-DAMAGE`, and `OB-LAP-SNAP` with no
  construct banned by C8 and empty `VERUS_FLAGS`; the proof-hygiene and tooling
  contracts pass.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` outside the float-boundary modules except where a reason cites a
  proven macro-expansion artefact.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Security: no new process-environment access, no `unsafe`, and `make audit`
  passes, with any advisory ignore recorded as a decision.

Red-Green-Refactor evidence is recorded in `Artefacts and notes` for each
milestone: the red command and its failure, the green command, and the refactor
gate run.

Acceptance by behaviour:

1. Running the hello-window `run` subcommand under GNOME or KDE Plasma shows
   the resource-defined window, menus, dialogs, and inspector, and Accerciser
   lists their roles and names and the canvas text.
2. Typing into the inspector's greeting field, including through an input
   method, updates the content and redraws only the greeting.
3. Running `export` produces the eight outputs, and the parity suite proves
   they agree.
4. Changing a string in `resources/locales/en-GB/hello-window.ftl` changes the
   window title without recompiling Rust.

## Idempotence and recovery

Every step is additive and can be rerun. The workspace conversion in EP-M0 is
one commit and reverts cleanly. Each dependency addition is its own commit.
`make install-verus` is idempotent: it verifies the checksum and reuses
`.verus/<version>`, which Git ignores. `export` writes into a temporary
directory and renames it into place, and refuses foreign non-empty directories
without `--force`. `make test-hosted` stops only the `weston` it started, even
on failure. If a gate fails halfway, fix the cause and rerun the whole gate
sequence; never skip a gate. Snapshot updates through `cargo insta review`
happen only after confirming the change reflects an intended contract change.

## Artefacts and notes

Record spike results, measured run times, the final vector tolerance, the Verus
verified-item count, and red, green, and refactor transcripts here per
milestone as work proceeds.

## Interfaces and dependencies

These signatures are the contracts each milestone must leave in place. They are
pre-1.0 internal interfaces; no compatibility layer is owed to anyone.

### Workspace layout

```plaintext
crates/
├── cabochon-lapidary/          scene model, units, resolution, device mapping, hit testing, damage, accessibility nodes
├── cabochon-burin/             workstations, text layout adapter, float boundary
├── cabochon-escutcheon/        resource schema, compiler, localization, escutcheon-check binary
├── cabochon-clerestory/        application contract, session, HostShell port, headless shell
├── cabochon-clerestory-hosted/ winit, softbuffer, and AccessKit adapter
├── cabochon-hello-window/      the hello-window application and its resources
├── cabochon-fonts/             bundled OFL fonts and licences (data only)
└── cabochon-substrate-testkit/ generators, PDF read-back, render constructor, tolerances (tests only)
verus/                          Verus proof entry point (and mirrors, if decision D11 falls back)
tools/verus/                    pinned Verus version and checksums
tools/rust-prover-tools/REF     pinned prover-tools commit
.config/nextest.toml            test groups and timeouts
```

### Lapidary (`cabochon-lapidary`)

```rust,no_run
pub struct Emu(i64);
impl Emu {
    pub const LIMIT: i64 = 1 << 47;
    pub const fn from_points(points: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_millimetres(mm: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_inches(inches: i64) -> Result<Self, LengthOutOfRange>;
    pub const fn from_reference_pixels(px: i64) -> Result<Self, LengthOutOfRange>;
    pub fn parse(text: &str) -> Result<Self, LengthParseError>; // "10.5pt", "3mm", "1in", "12px"
}

pub struct Fixed16(i32);
pub struct Transform { pub xx: Fixed16, pub xy: Fixed16, pub yx: Fixed16, pub yy: Fixed16, pub tx: Emu, pub ty: Emu }

pub struct Scene {
    pub page: PageGeometry,
    pub output_intent: OutputIntent,
    pub colour_intent: ColourIntent,
    pub layers: Vec<LayerSpec>,
    pub root: Group,
    pub images: ImageStore,
}

pub enum Node {
    Group(Group),
    Shape(Shape),
    Text(TextNode),
    Image(ImageNode),
    HitRegion(HitRegion),
    Subscene(Subscene), // origin: OpaqueOrigin, transform, clip, scene: Arc<Scene>
}

pub enum OutputIntent { Screen, Document, Print(PrintIntent) }

pub trait TextLayout {
    fn layout(&self, run: &TextRun) -> Result<ShapedRun, TextLayoutError>;
    fn font_data(&self, key: FontKey) -> Option<FontData>;
}

pub struct ShapedRun {
    pub font: FontKey,
    pub glyphs: Vec<ShapedGlyph>,
    pub ink_bounds: Rect,
    pub caret_stops: Vec<CaretStop>,
}

pub fn resolve(scene: &Scene, layout: &dyn TextLayout) -> Result<ResolvedScene, ResolveError>;

pub struct DeviceMapping { pub origin: Point, pub scale: Scale120 }
impl DeviceMapping {
    pub fn scene_to_device(&self, point: Point) -> DevicePoint;
    pub fn device_to_scene(&self, point: DevicePoint) -> Point;
    pub fn snap_out(&self, rect: Rect) -> PixelRect;
}

impl ResolvedScene {
    pub fn hit_test(&self, point: Point) -> Option<HitPath>;
    pub fn hit_test_all(&self, point: Point) -> Vec<HitPath>;
    pub fn accessibility_nodes(&self) -> AccessibilityNodes;
    pub fn fonts(&self) -> &FontSet;
}

pub fn damage(old: &ResolvedScene, new: &ResolvedScene) -> Damage;

impl SceneDocument {
    pub fn encode(scene: &Scene) -> Result<String, SceneEncodeError>;
    pub fn decode(text: &str) -> Result<Scene, SceneDecodeError>;
}
```

`ResolveError` covers `GeometryOutOfRange`, `TextLayout(TextLayoutError)`
(including `NoFontCovers`), and `Invalid(ValidationError)`.

### Burin (`cabochon-burin`)

```rust,no_run
pub trait Workstation {
    type Request;
    type Output;
    fn render(&self, scene: &ResolvedScene, request: &Self::Request) -> Result<Self::Output, RenderError>;
}

pub struct InteractiveSurface { /* frame buffer and previous scene */ }
impl InteractiveSurface {
    pub fn redraw(
        &mut self,
        scene: &ResolvedScene,
        damage: &Damage,
        mapping: &DeviceMapping,
    ) -> Result<FrameReport, RenderError>;
    pub fn pixels(&self) -> &[u32];
}

pub enum RenderError {
    UnsupportedFeature { workstation: WorkstationKind, feature: FeatureKind },
    UndecodableImage { node: NodePath },
    ImageSizeMismatch { node: NodePath },
    ResourceLimit { workstation: WorkstationKind, limit: LimitKind },
    Backend { workstation: WorkstationKind, class: BackendFailureClass },
}

pub struct ParleyTextLayout { /* font collection injected at construction */ }
impl ParleyTextLayout {
    pub fn new(fonts: FontCollectionSpec) -> Result<Self, FontLoadError>;
}

pub fn render_with_fallback<W: Workstation>(
    workstation: &W,
    scene: &ResolvedScene,
    request: &W::Request,
) -> RenderOutcome<W::Output>;
```

Workstations: `RasterWorkstation`, `ThumbnailWorkstation`, `PdfWorkstation`,
`PrintWorkstation`, `AccessibilityWorkstation`, `ClipboardWorkstation`, and
`HitMapWorkstation`, plus the stateful `InteractiveSurface`.

Metrics, described with `describe_counter!` and `describe_histogram!`:
`burin_render_total{workstation, outcome}`,
`burin_render_failures_total{workstation, class}`, and
`burin_render_seconds{workstation}`. Labels are fixed enumerations.

### Escutcheon (`cabochon-escutcheon`)

```rust,no_run
pub trait ResourceSource {
    fn read(&self, path: &ResourcePath) -> Result<String, ResourceReadError>;
    fn list(&self, dir: &ResourcePath) -> Result<Vec<ResourcePath>, ResourceReadError>;
}

pub fn compile(source: &dyn ResourceSource, locales: &LocaleChain) -> Result<ResourceBundle, CompileDiagnostics>;

pub struct LocaleChain(Vec<LocaleTag>); // owned type; no unic-langid in the API
pub struct Diagnostic { pub file: ResourcePath, pub span: SourceSpan, pub code: DiagnosticCode, pub message: String }

pub struct ResourceBundle { /* commands, menus, dialogs, inspectors, windows, icons, strings, shortcuts */ }
pub struct CommandSpec {
    pub id: CommandId,
    pub label: StringKey,
    pub tooltip: Option<StringKey>,
    pub accessible_description: Option<StringKey>,
    pub icon: Option<IconId>,
    pub shortcuts: Vec<Shortcut>,
    pub enabled_by_default: bool,
}
```

Resource bundle layout for an application named `<app>`:

```plaintext
resources/
├── <app>.escutcheon.kdl        schema version, commands, menus, dialogs, inspectors, windows, icons
├── locales/en-GB/<app>.ftl     base locale (must be complete)
├── locales/fr/<app>.ftl        optional translations
└── icons/*.svg
```

Example fragment from `hello-window.escutcheon.kdl`. Identifiers with dots,
such as `app.quit`, are command identifiers. Hyphenated identifiers, such as
`menu-file`, are keys into the Fluent strings file. `#true` is KDL 2.0's
spelling of the boolean true. `dialog.ok` is a built-in command, so it is not
declared here.

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
    fn commands(&self) -> &[CommandId];
    fn handle_command(&mut self, command: &CommandId, context: &mut CommandContext) -> CommandOutcome;
    fn command_state(&self, command: &CommandId) -> CommandState;
    // ContentFrame { scene: Arc<Scene>, revision: ContentRevision }
    fn content(&self, view: &ViewContext) -> Option<ContentFrame>;
    fn inspector(&self, window: WindowId) -> Option<InspectorModel>;
}

pub enum HostEvent {
    Configured { window: HostWindowId, size: DeviceSize, scale: Scale120 },
    CloseRequested { window: HostWindowId },
    Key { window: HostWindowId, key: KeyEvent },          // logical key, physical key, modifiers, text
    Pointer { window: HostWindowId, event: PointerEvent },
    Ime { window: HostWindowId, event: ImeEvent },        // Enabled, Preedit { text, caret }, Commit(text), Disabled
    FocusChanged { window: HostWindowId, focused: bool },
    RedrawRequested { window: HostWindowId },
    Woken(WakeToken),
}

pub trait HostShell {
    fn open_window(&mut self, spec: &WindowSpec) -> Result<HostWindowId, ShellError>;
    fn present(&mut self, window: HostWindowId, frame: &FrameUpdate<'_>) -> Result<(), ShellError>;
    fn publish_accessibility(&mut self, window: HostWindowId, tree: &AccessibilityTree) -> Result<(), ShellError>;
    fn set_ime_cursor_area(&mut self, window: HostWindowId, area: PixelRect) -> Result<(), ShellError>;
    fn close_window(&mut self, window: HostWindowId) -> Result<(), ShellError>;
}

pub trait SessionWaker: Send + Sync {
    fn wake(&self, token: WakeToken);
}

pub struct FrameUpdate<'a> { pub scene: &'a ResolvedScene, pub damage: &'a Damage, pub mapping: &'a DeviceMapping }

pub struct Session<A: Application> { /* windows, menus, dialogs, focus, modality, routing */ }
impl<A: Application> Session<A> {
    pub fn handle(&mut self, event: HostEvent, shell: &mut dyn HostShell) -> Result<SessionStatus, SessionError>;
}

pub enum Role { Window, MenuBar, Menu, MenuItem, Dialog, Alert, Button, Label, TextInput, Panel, Inspector, Canvas }
pub enum StandardDialog { Alert(DialogText), Confirm(DialogText), About }
```

`FrameUpdate` carries a resolved scene, its damage, and the device mapping,
never pixels; the adapter rasterizes through Burin. `HeadlessShell` lives in
`cabochon_clerestory::shell::headless` and records windows, frames,
accessibility trees, IME cursor areas, and the dispatch log. Metric:
`clerestory_command_dispatch_total{outcome}`.

### Dependencies

All requirements are caret requirements in `[workspace.dependencies]`;
`Cargo.lock` records the exact versions. Versions are the October 2026
releases; use the newest compatible release when each milestone starts and
record any deviation in `Decision log`.

Runtime dependencies: `serde` 1, `serde_json` 1, `base64` 0.22, `thiserror` 2,
`tracing` 0.1, `metrics` 0.24, `kdl` 6.7, `fluent-bundle` 0.16, `fluent-pseudo`
0.3, `unic-langid` 0.9, `usvg` 0.48 (default features off), `png` 0.18,
`parley` 0.12, `fontique` 0.12, `vello_cpu` 0.3, `kurbo` 0.13, `krilla` 0.8,
`winit` 0.30.13, `softbuffer` 0.4.8, `accesskit` 0.25, `accesskit_winit` 0.34,
`cap-std` 4, `camino` 1, `clap` 4 (application only), `tracing-subscriber` 0.3
(application only), and `mockable` 3 (application only). If decision D11's
single-source route is taken, `vstd` and `verus_builtin_macros` at the dated
release matching `tools/verus/VERSION` (pre-release versions, so the
requirement is exact; this is the documented exception to caret requirements).

Development dependencies: `rstest` 0.26, `rstest-bdd` 0.6, `rstest-bdd-macros`
0.6, `googletest` 0.14, `pretty_assertions` 1.4, `insta` 1.49, `proptest` 1.11,
`proptest-state-machine` 0.9, `hayro` 0.8, `hayro-interpret` 0.8,
`accesskit_consumer` 0.39, `unicode-segmentation` 1, `unicode-bidi` 0.3, and
`proc-macro2` 1 (only if decision D11 falls back to spec mirrors).

Verification tooling: Verus (pinned in `tools/verus/VERSION`), Kani (already
installed; run through `make kani`), and `rust-prover-tools` (pinned in
`tools/rust-prover-tools/REF`).

## Documentation obligations

- ADRs under `docs/adrs/`, named `adr-nnnn-title-slug.md` per the style
  guide: ADR 0009 (integer EMU geometry and fixed-point transforms), ADR 0010
  (KDL, Fluent, and SVG resources), ADR 0011 (software renderer and workstation
  parity), and ADR 0012 (in-window furniture, the host-shell port, and the
  considered alternatives, including Masonry), each referenced from
  `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` gains a crate ownership and reuse subsection
  (EP-M0), and §§8.1 to 8.4 gain the contract summaries, tolerances, and proof
  inventory, with the proof-driven representation decisions stated as design
  facts: half-open integer rectangles, checked transforms, non-zero winding
  with half-open crossings, topmost-wins scanning, prefix and suffix damage
  over interned keys, and edge snapping.
- `docs/developers-guide.md` gains sections on the workspace, the dependency
  rule and its contract test, test conventions (`rstest`, `rstest-bdd`,
  `googletest`, `pretty_assertions`, `insta`, `proptest`, budgets, and the
  `render` test group), the float-boundary policy, the prover tooling, the
  proof inventory with what each proof does not cover, and the untested
  production SIMD path. It documents the reuse policy for the testkit
  (integration tests only, never a production dependency) and for
  `cabochon-fonts` (data only).
- Prose references for scene document version 1 and Escutcheon schema
  version 1, for authors working in other languages (ADR 0004).
- `docs/users-guide.md` gains a hello-window section covering `run`,
  `export`, `--locale`, `--system-fonts`, and `escutcheon-check`.
- `docs/repository-layout.md` and `docs/contents.md` list every new directory
  and document, including this plan.
- `docs/roadmap.md` marks 1.0.1 to 1.0.4 done when their success criteria are
  proved.

## Revision notes

- 2026-10-18: initial draft, written from Wyvern reconnaissance and Firecrawl
  research.
- 2026-10-18: revised after the community-of-experts review. The structure
  and contracts panel moved resolution into scene space with a Lapidary
  `DeviceMapping`, made transforms checked, added embedded scenes, layers,
  encoded images, a Lapidary-owned `FontSet`, Lapidary accessibility nodes,
  `HostEvent`, `SessionWaker`, `FrameUpdate`, a third-party leakage allowlist,
  and decision D12. The verification panel corrected the winding lemma
  (cancellation, right, left, and scanline cases), restated transform bounds
  over the exact image, replaced the raster coverage obligation with a
  differential locality test, and added `OB-LAP-SNAP`, `OB-LAP-RESOLVE`, the
  text obligations, `OB-BUR-NODROP`, `OB-BUR-FLOAT`, `OB-CLR-LAYOUT`,
  `OB-ESC-DET`, explicit non-vacuity mechanics, and the single-source Verus
  route (decision D11). The failure-modes panel added the `check-cfg` entry,
  the Kani recipe route, the held-out and development target lists,
  `.gitignore` for `.verus/`, rendering determinism, proptest budgets, hosted
  provisioning and isolation, safe export, and split four milestones into ten.
  The alternatives panel added text input with IME (decision D14), the font
  policy (decision D15), pseudo-locales for right-to-left layout, logical-key
  shortcuts, `escutcheon-check`, prose schema references, the Masonry
  alternative in ADR 0012, and deferred the canonical emitter (decision D19).
  The remaining work is unchanged in purpose; it is now sequenced as fifteen
  milestones and awaits approval.
- 2026-10-22: recorded the reviewer's acceptance of decisions D1, D6, D7, D8,
  D10, D11, D12, D13, and D14. No decision changed, so the remaining work is
  unchanged; EP-M0 still waits for approval of the plan as a whole.
- 2026-10-22: rebased onto `main` at `1f43018`, which includes pull requests
  #51 and #64. Recorded the merged `typos.toml` regeneration, the root package's
  `rust-version = "1.85"` and its `msrv` CI job (now part of constraint C6, a
  new risk, and an EP-M0 check), and the new nightly pin. Updated the `kani`
  recipe to mirror the current `lint-whitaker` recipe, and dropped the
  Markdownlint MD010 directives that the repository's new code-block policy
  makes unnecessary. The milestones are unchanged.
- 2026-10-22: rebased onto `main` at `0de2b57`, which adopts the shared
  documentation style guide (#65). Updated every ADR reference to the four-digit
  `docs/adrs/adr-nnnn-title-slug.md` scheme, so this plan's new ADRs are ADR
  0009 to ADR 0012; added the style guide's doc-comment rules to constraint C5;
  and recorded the change in `Surprises & discoveries`. The milestones are
  unchanged.
