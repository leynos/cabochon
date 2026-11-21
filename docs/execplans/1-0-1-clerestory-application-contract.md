# Define the minimum Clerestory application contract (roadmap 1.0.1)

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & discoveries`, `Decision log`,
`Outcomes & retrospective`, `Conformance basis`, and `Verification plan` must
be kept up to date as work proceeds.

Status: DRAFT (refined 2026-11-21 against the Cabochon UI guidelines; expert
review applied; awaiting user approval)

Branch: `1-0-1-clerestory-application-contract`

Roadmap item: `1.0.1` in `docs/roadmap.md`.

The reviewer approved an earlier revision of this plan on 2026-11-04. That
approval does not cover this revision, which changes the acceptance surface,
adds formal proof milestones, and depends on amendments to two sibling plans.
Implementation must not begin until the user approves this revision, the pull
request carrying it has merged, and the 1.0.3, 1.0.4, and 1.0.2 plans
(`docs/execplans/1-0-3-lapidary-scene-contract.md`,
`docs/execplans/1-0-4-burin-output-contract.md`, and
`docs/execplans/1-0-2-escutcheon-resource-format.md`) have been implemented and
merged.

## Purpose / big picture

Cabochon models its application environment on the Graphics Environment Manager
(GEM), the 1980s desktop whose Application Environment Services (AES) gave
every program the same windows, menus, and dialogs from resource files.
Clerestory is Cabochon's AES analogue. It owns windows, menus, commands,
standard dialogs, focus, keyboard traversal, text input, inspector panels,
accessibility roles, and adaptation to a host desktop such as GNOME or KDE
Plasma. It builds on Escutcheon (resources), Lapidary (scenes), and Burin
(rendering), which the three earlier task plans of roadmap step 1.0 deliver.
Architectural decision record (ADR) 0007 makes all four production contracts in
hosted mode, and the roadmap requires them to work without Enfilade, Cabochon's
semantic object layer.

Today the repository is a single crate, `cabochon`, whose `src/main.rs` prints
a greeting. Nothing draws a window. After this plan, a developer can run a
small application, `hello-window`, inside an ordinary GNOME or KDE Plasma
session. The Cabochon UI guidelines call this program "the smallest application
that proves the Cabochon application contract" (`book/36-hello-window.md` in
`leynos/cabochon-ui-guidelines`). Its window, menu bar, commands, shortcuts,
strings, icon, status line, and dialogs all come from Escutcheon resources
rather than hand-built widgets. Its document is one Lapidary scene drawn by
Burin. Every control is reachable by keyboard and described to assistive
technology. The same scene exports to every Burin output with one command.

Clerestory's contract is wider than `hello-window`'s surface. The roadmap names
inspector panels, and the reviewer accepted a text field with input-method
support (decision D14). The guidelines place neither in `hello-window`'s
document, so a second, test-only application, the contract probe, exercises
them. Nothing in the contract goes unproved, and `hello-window` stays "entirely
explained by the contract".

Observable success:

1. `cargo run -p cabochon-hello-window -- run` opens one window. Its menu bar
   reads File, View, Help. File holds Export, Print, and Quit. View holds
   Command palette, Zoom in, Zoom out, Actual size, and Dark appearance. Help
   holds About and Diagnostics. The document shows a greeting, a short
   explanation, a primary Export button, and a secondary About button. A status
   line sits at the bottom. Control+Q quits, Control+K opens and closes the
   command palette, F1 describes the focused control in the status line, Alt
   focuses the menu bar, and Tab visits every control once per cycle.
2. `--locale en-XA` shows pseudo-localized strings and `--locale ar-XB` shows
   the furniture mirrored for right-to-left text, with no code changes and no
   clipped labels.
3. `cargo run -p cabochon-hello-window -- review --out "$(mktemp -d)"` writes
   the conformance bundle: `content.pdf`, `content-print.pdf`, `content.png`,
   `thumbnail.png`, `accessibility.json`, `clipboard/`, `hit-map.json`, and
   `export.json` (the manifest).
4. `make test` passes, including `furniture.feature` and
   `hello_window.feature` under the headless shell. `make verus` proves the
   focus kernel. `make test-hosted` runs the application under a headless
   Wayland compositor and prints:

   ```plaintext
   hello-window: shell=hosted windows=1 frames=1 accessibility_nodes=<N> exit=0
   ```

5. Roadmap tasks 1.0.1 and 1.0.2 are marked done (decision T1).

How to read identifiers: `RM-` roadmap task; `TDD-` technical-design element
(`TDD-CLR` is `docs/cabochon-design.md` §8.1, `TDD-INV-n` is invariant `n` of
§3, `TDD-FAIL` is §11, `TDD-OBS` is §14); `TOR-SM-GEM` is the terms of
reference's substrate success measure; `UIG-` a UI-guidelines rule, followed by
the book's own checklist identifier where one exists; `REQ-` a requirement this
plan derives; `GAP-` a baseline-to-target gap; `EP-M` milestone; `OB-`
verification obligation; `INV-` invariant; `L-` lemma; `AXIOM-` trusted
assumption; `C` constraint; `D` decision inherited from the reviewed step-level
plan; `T` decision made when the step plan was split into task plans; `R`
decision made in this refinement; `DEP-` a recorded departure from the
guidelines; and `AM-` an amendment to a sibling plan.

## Constraints

- C1, GEM-derived invariants. Applications reach the desktop only through
  Clerestory, draw only through Lapidary scenes, render only through Burin, and
  take user-interface structure from Escutcheon (`docs/cabochon-design.md` §3,
  invariants 1 to 4). No crate in this plan names an Enfilade type (invariant
  5, decision D12).
- C2, dependency direction, enforced by allow-lists counting normal and build
  edges (decision T13). `OB-ARCH-DEPS` rows: `cabochon-clerestory` may depend
  only on `cabochon-lapidary`, `cabochon-escutcheon`, `thiserror`, `tracing`,
  `metrics`, and `unicode-normalization`, plus `vstd` and
  `verus_builtin_macros` if the 1.0.3 plan adopted its single-source proof
  route (decision D11); `cabochon-clerestory-headless` only on
  `cabochon-clerestory`, `cabochon-lapidary`, and `thiserror`;
  `cabochon-clerestory-hosted` on `cabochon-clerestory`, `cabochon-burin`,
  `cabochon-lapidary`, `winit`, `softbuffer`, `accesskit`, `accesskit_winit`,
  `thiserror`, and `tracing`; and `cabochon-hello-window` on the three
  Clerestory crates, `cabochon-escutcheon`, `cabochon-burin`,
  `cabochon-lapidary`, `cabochon-fonts`, `clap`, `eyre`, `serde`, `serde_json`,
  `tracing`, `tracing-subscriber`, `metrics`, `metrics-util`, `mockable`,
  `cap-std`, and `camino`. Only the hosted and application crates combine
  Clerestory with Burin in normal edges, and only the hosted crate reaches
  `winit`, `softbuffer`, or AccessKit. Development edges are not restricted by
  these rows.
- C3, module containment (`OB-ARCH-IMPORTS` rows). In Clerestory, `kernel`
  imports nothing from the rest of the crate, and `vstd` appears only under
  `kernel`. In the hosted crate, `winit` and `softbuffer` appear only under
  `event_loop` and `present`, AccessKit only under `accessibility`, and float
  or cast lint expectations only under `float_boundary`. In the application,
  `clap` appears only under `cli`, `tracing-subscriber` and `metrics-util` only
  under `telemetry`, `eyre` only in `main.rs`, and `serde` and `serde_json`
  only under `export::wire`, whose private data-transfer types write
  `export.json` (decision T15). The script and journal files are line-oriented
  text handled by handwritten code in the application's `script` and
  `journal_file` modules, so they need no serializer.
- C4, no third-party types in Clerestory's public interface: no `winit`,
  Wayland, X11, AccessKit, GTK, or Qt types (ADR 0002's consequence that host
  concerns stay behind adapter ports). `OB-ARCH-PUBAPI` row:
  `cabochon-clerestory`'s public interface may name only `std`, `core`, `alloc`,
  `cabochon_lapidary`, and `cabochon_escutcheon`. The adapters and the
  application are composition crates and are exempt.
- C5, integer geometry. Floating-point arithmetic appears only in
  `cabochon_clerestory_hosted::float_boundary` (scale factors and pointer
  positions from `winit`), with reasoned, item-scoped lint expectations.
- C6, repository policy as stated in `AGENTS.md` and enforced by `Cargo.toml`
  and `clippy.toml`: no file over 400 lines, cognitive complexity at most 9,
  functions at most 70 lines and four parameters, no `unsafe`, no `unwrap` or
  `expect` outside tests, a `//!` comment on every module, Rustdoc with an
  example on every public item, a message on every assertion, and en-GB Oxford
  spelling.
- C7, existing contracts keep passing: the `msrv` continuous integration (CI)
  job, the workflow contract tests under `tests/`, and the build-standard
  contracts. Member crates declare no Cargo features.
- C8, no proof shortcuts. Verus sources contain no `assume`, `admit`,
  `#[verifier::external_body]`, `#[verifier::external]`, or trusted proof body,
  and no axiom introduced for a theorem in this plan. The 1.0.3 plan's
  `OB-PROOF-MIRROR` guard enforces this textually.
- C9, tests never mutate the process environment. Locale, display, and
  appearance reach the application through arguments or an injected
  `mockable::Env`; child processes in end-to-end tests receive environment
  values through `Command::env`. No `serial_test`, shared lock, or `temp_env`.
- C10, only the `cabochon-hello-window` binary installs a `tracing`
  subscriber or a `metrics` recorder. Library crates emit and never install.
- C11, shared host. Recipes never kill processes they did not start. Rust
  compilation always runs on the host under its compile admission, never inside
  a container.
- C12, the UI guidelines bind user experience, not platform scope. Where the
  book and the roadmap, design document, or an ADR disagree about scope, the
  programme documents win (the book's own rule, `book/appendix-references.md`
  "Contract basis"). Every applicable **must** rule this plan does not meet is
  listed as a `DEP-` departure in `Decision log`.
- C13, this plan's implementation does not edit the sibling plan documents.
  The amendments `AM-ESC-1` and `AM-BUR-1` reach those documents through their
  own plan-revision pull request (decision R16). Code those plans delivered may
  be changed here where a milestone says so.
- C14, implementation begins only after the user approves this revision.

If satisfying the objective requires violating a constraint, stop, record the
conflict in `Decision log`, and escalate.

## Tolerances (exception triggers)

Comparison base: the commit on `main` from which the first implementation
branch is cut, recorded in `Artefacts and notes` at the start of EP-M0. Limits
are net lines (additions minus deletions), measured per category with:

```bash
git diff --numstat "$BASE"...HEAD -- <category paths>
```

Each category is enforced independently; spare capacity in one never offsets
another. ExecPlan files under `docs/execplans/` are excluded from every count.

- Code: Rust under `crates/*/src`, `crates/*/tests`, `tests/`, and `verus/`,
  plus `Makefile` recipes and `scripts/`. Limit: 3,300 net lines in any one
  milestone and 20,000 for the plan. Generated `*.snap` files and
  `proptest-regressions/` are excluded.
- Configuration: `Cargo.toml` manifests, `.config/nextest.toml`,
  `.github/workflows/`, `typos.local.toml`, and handwritten resource bundles
  (`*.kdl`, `*.ftl`, `*.svg`). Limit: 1,200 net lines for the plan.
  `Cargo.lock` and the generated `typos.toml` are excluded.
- Documentation: `docs/` except `docs/execplans/`, and `README.md`. Limit:
  1,500 net lines for the plan.
- Files: more than 70 non-ExecPlan files touched in one milestone.
- Interface: a Lapidary, Escutcheon, or Burin contract must change beyond
  `AM-ESC-1` and `AM-BUR-1`, or a Clerestory contract changes in a way that
  alters roadmap tasks 2.1.2 and 2.1.3.
- Dependencies: a dependency not listed in `Interfaces and dependencies` is
  needed, or a listed crate must move to a new major or pre-1.0 minor series.
- Proof: a lemma group in EP-M2 is not proved after two working days and
  three restructurings of the kernel, or proving it needs a construct C8 bans.
  The escalation must offer the named fallback, an exhaustive small-scope test
  of the same statement, for the user to accept or refuse; it is never adopted
  silently.
- Hosted environment: a headless Wayland compositor cannot be provisioned in
  CI or locally during EP-M0.
- Coverage: the coverage ratchet fails because hosted-only code cannot run
  without a display, although the display-bound modules are within their
  300-line cap.
- Upstream conflict: evidence contradicts `docs/cabochon-design.md`, an ADR,
  the roadmap, or a **must** rule of the UI guidelines that `Decision log` does
  not already record as a departure. Set `Status: BLOCKED` and record the
  proposed deviation.
- Iterations: a red test cannot be turned green after five attempts with
  distinct hypotheses.
- Ambiguity: two readings of a guideline rule lead to different observable
  behaviour and `Decision log` does not choose between them.

The initial budgets are justified in `Context and orientation`.

## Risks

- Risk: the guidelines' `hello-window` needs resource features the approved
  Escutcheon plan does not define. Severity: high. Likelihood: certain.
  Mitigation: `AM-ESC-1` is to land in the 1.0.2 plan before that plan is
  implemented (decision R16); if 1.0.2 was implemented without it, EP-M0 adds
  the features to Escutcheon as an additive change in its own pull request.
- Risk: scope. The refined surface adds a status line, command palette, zoom,
  appearance, and diagnostics to an already large plan. Severity: high.
  Likelihood: high. Mitigation: each addition has a minimal, stated semantics;
  ten milestones with per-milestone budgets, delivered as stacked pull requests
  (decision R15); exclusions and departures are explicit.
- Risk: headless `weston` implements neither the `text-input-v3` protocol nor
  fractional scaling, and may offer no keyboard seat, so real key, pointer, and
  input-method events cannot be exercised under the hosted adapter in CI.
  Severity: medium. Likelihood: high. Mitigation: `OB-CLR-HOST` claims only
  what it can show (decision R13); `winit` event mapping is verified by pure
  unit tests and the manual check; the residual is named for roadmap 5.1.4.
- Risk: AccessKit's Linux adapter activates only when an assistive technology
  has announced itself on the session bus, so a hosted test may publish no
  tree. Severity: medium. Likelihood: high. Mitigation: `OB-HOST-A11Y` checks
  the conversion with `accesskit_consumer`, independent of activation; EP-M0
  probes activation; reading the tree back over the Assistive Technology
  Service Provider Interface (AT-SPI) stays a residual.
- Risk: a hosted test leaks `weston` or D-Bus processes on the shared host,
  or flakes when a runner image changes. Severity: medium. Likelihood: medium.
  Mitigation: the runner script's process-group, deadline, and readiness rules
  in EP-M8; a pinned runner image; both `weston` versions recorded.
- Risk: `winit` 0.30 has no popup surfaces, so menus, sheets, and the palette
  are drawn inside the window. Severity: low. Likelihood: certain. Mitigation:
  menus flip or scroll inside the window, and a test proves the menu bar and
  widest sheet fit the declared minimum window in every shipped locale.
- Risk: `winit` 0.31 (beta) changes the event-loop interface and
  `accesskit_winit` 0.34 supports only `winit` 0.30. Severity: low. Likelihood:
  high. Mitigation: stable `winit` 0.30.13, confined to the hosted crate behind
  the `HostShell` port.
- Risk: the Verus proofs need induction over a rank function and over the
  frame stack, and later milestones may force kernel changes that reopen them.
  Severity: medium. Likelihood: medium. Mitigation: EP-M0 checks the statements
  are expressible; the proof tolerance is time-boxed with a named fallback; any
  milestone that touches `kernel/` reruns `make verus`; a pull-request path
  filter runs the proofs in CI.
- Risk: the coverage ratchet fails because the hosted event loop cannot run
  without a display. Severity: medium. Likelihood: medium. Mitigation: the
  display-bound modules (`event_loop`, `present`) are capped at 300 lines
  together, with all mapping in pure, unit-tested functions; EP-M0 measures the
  ratchet's behaviour on an ignored test.
- Risk: `rstest-bdd` 0.6 declares a minimum supported Rust version of 1.88,
  above the root's 1.85. Severity: low. Likelihood: certain. Mitigation: it is
  a development dependency, and the `msrv` job checks only the root library;
  confirm in EP-M0.
- Risk: whole-tree accessibility republication, relayout on every event, and
  full-window damage on a scale change are linear in the number of controls.
  Severity: low. Likelihood: certain. Mitigation: acceptable at tens of nodes;
  revisit at roadmap 2.1 or beyond 1,000 nodes; the tree digest is recomputed
  only when the tree changes.
- Risk: the sibling plans still describe Cranelift as the development
  backend, while `main` disabled it on 2026-11-14. Severity: low. Likelihood:
  certain. Mitigation: this plan depends on neither backend; it records the
  drift for the 1.0.3 plan's owner.

## Progress

- [x] (2026-10-18) Step-level plan drafted, reviewed by four
  community-of-experts panels, and revised (commit `616a8fa`).
- [x] (2026-10-22) Reviewer accepted decisions D1, D6, D7, D8, D10, D11, D12,
  D13, and D14 on the step-level plan.
- [x] (2026-10-23) Decomposed into four task-level plans; this is the last.
- [x] (2026-11-04) Reviewer approval of the earlier revision.
- [x] (2026-11-21) Refined against the UI guidelines, current tooling, and the
  proof-sketch-first verification policy.
- [x] (2026-11-21) Reviewed by a six-lens expert panel (verdicts: revise,
  revise, proceed with conditions) and revised.
- [ ] User approval of this revision: decisions R1 to R17, departures
  `DEP-1` to `DEP-7`, and amendments `AM-ESC-1` and `AM-BUR-1`.
- [ ] `AM-ESC-1` and `AM-BUR-1` merged into the sibling plans.
- [ ] The 1.0.3, 1.0.4, and 1.0.2 plans implemented and merged.
- [ ] EP-M0: feasibility probes and prerequisite check.
- [ ] EP-M1: focus kernel, routing, session, and the headless shell.
- [ ] EP-M2: Verus proofs of the focus kernel.
- [ ] EP-M3: measures, theme, layout, and view arithmetic.
- [ ] EP-M4: window, menus, status line, sheets, and the accessibility tree.
- [ ] EP-M5: text input, command palette, inspector panel.
- [ ] EP-M6: the hello-window application: surface, resources, and export.
- [ ] EP-M7: the hello-window application: view state, diagnostics, telemetry.
- [ ] EP-M8: the hosted shell adapter and its CI job.
- [ ] EP-M9: documentation reconciliation and step closure.

## Surprises & discoveries

- Observation: `accesskit_winit` 0.34.2 requires `winit` ^0.30.5 and does not
  support the 0.31 betas. Evidence: crates.io metadata, 2026-11-21. Impact:
  `winit` 0.30.13 is the only compatible stable release.
- Observation: `muda` (native menus) needs GTK on Linux and attaches only to
  GTK windows. Impact: supports in-window menus (decision D5).
- Observation: `mockable` is at 3.0.0, while
  `docs/reliable-testing-in-rust-via-dependency-injection.md` cites `0.3` and
  recommends `temp_env`, which `AGENTS.md` forbids. Impact: use `mockable` 3
  and correct the guide in EP-M7.
- Observation: the UI guidelines (`leynos/cabochon-ui-guidelines`, commit
  `9852939`, 2026-11-21) define `hello-window` differently from the earlier
  revision of this plan: a read-only document with no inspector, a status line,
  and Print, Command palette, Zoom, Appearance, and Diagnostics commands; F1
  means help for the focused object, not About. Evidence:
  `book/36-hello-window.md`; `book/18-focus-keyboard.md` "Keys that must work".
  Impact: decisions R2 to R9.
- Observation: the book cites the earlier revision of this plan, at
  `de40019`, as part of its own contract basis, and says platform documents win
  a conflict. Evidence: `book/appendix-references.md` "Contract basis". Impact:
  adopting chapter 36 is a choice this plan makes (decision R1), not something
  the book can impose.
- Observation: the book's profile table makes "Inspector and project window"
  a semantic-workspace capability, while roadmap 1.0.1 lists inspector panels.
  Impact: decision R3.
- Observation: headless `weston` 13 to 15 lacks `text-input-v3` and
  `fractional-scale-v1`. Evidence: wayland.app protocol support tables,
  2026-11-21. Impact: the hosted residuals.
- Observation: AccessKit's Unix adapter builds no tree until
  `org.a11y.Status` reports an enabled client, and Accerciser does not set that
  flag at start-up. Evidence: AccessKit discussion 502. Impact: `OB-HOST-A11Y`
  and the manual check's instructions.
- Observation: `winit` refuses to create an event loop off the main thread on
  Linux, and the test harness runs tests on worker threads. Impact: every
  hosted test starts the application as a child process.
- Observation: the workflow contract tests refuse a raw `cargo nextest` line
  in CI but accept `make test-hosted`, treat it as a suite job (the name
  contains `make test`), and so require a Linux runner, a pinned `setup-rust`
  with `install-mold: 'true'` and `expect-cache: any`, and the enforced
  `cargo-nextest` install step. Evidence: `tests/workflow_suite_contract.rs`,
  `tests/build_backend_contract/`. Impact: EP-M8's CI job.
- Observation: `docs/scripting-standards.md` makes Python with `uv` the
  baseline for new scripts. Impact: the hosted-test runner is Python.
- Observation: `docs/roadmap.md` task 5.1.3 is packaging; automated
  GNOME-like and KDE-like acceptance is task 5.1.4. Impact: residuals cite
  5.1.4.
- Observation: Lapidary's `resolve` takes one `TextLayout`. Impact: the
  session holds one layout, and the font policy lives in the font collection
  and the family names furniture requests (decision D15 as restated).

## Decision log

Inherited from the reviewed step-level plan (accepted ones say so):

- Decision D5: Clerestory owns the furniture model (commands, menus, dialogs,
  focus, modality, roles) and draws furniture inside the window as Lapidary
  scenes. It adapts to hosts through a `HostShell` port; the hosted adapter uses
  `winit` 0.30, `softbuffer`, and `accesskit_winit`. Considered: native menus
  through `muda` (needs GTK), GTK4 (a second toolkit), Slint, egui, or iced
  (each duplicates Lapidary), and Masonry as the hosted furniture renderer (a
  second render path outside Burin's parity tests). `FrameUpdate` stays at the
  scene level so the Masonry option remains open. Recorded as ADR 0012.
- Decision D8 (accepted 2026-10-22): standard dialogs are alert,
  confirmation, and About. File dialogs wait for roadmap 5.1.2; export takes
  its directory from the command line.
- Decision D10 (accepted 2026-10-22): device scale in 1/120 units; snapping
  applies to edges; a scale change damages the whole window.
- Decision D11 (accepted 2026-10-22, 1.0.3 plan): Verus kernels are written
  once and compiled into production if the 1.0.3 plan's S-VERUS spike
  succeeded; otherwise a Verus copy is kept token-identical by a drift guard.
  This plan follows whichever route 1.0.3 delivered; EP-M0 records which.
- Decision D12 (accepted 2026-10-22): Enfilade integration lives in bridge
  crates; EP-M9 proposes redrawing the design's Figure 1 accordingly.
- Decision D13 (accepted 2026-10-22): drag-and-drop and host clipboard
  integration are deferred; EP-M9 proposes a roadmap task for them under step
  5.1.
- Decision D14 (accepted 2026-10-22): a single-line text field with input
  method editor (IME) support is in scope.
- Decision D15 (restated): font policy. Furniture always requests the bundled
  families from `cabochon-fonts`, which every font collection contains. A
  `--fonts bundled|bundled+system` option selects the collection: `run`
  defaults to `bundled+system`, and `export` and `review` to `bundled`. The
  manifest records every font and fallback. Tests pass `--fonts bundled`.
- Decision D20: assertion style (`googletest` matchers, `pretty_assertions`
  for whole values, `insta` for stable summaries; `#[gtest]` before
  `#[rstest]`, because the reverse order registers the test twice).

Task-level decisions (2026-10-23):

- Decisions T13, T14, and T15 (recorded in the 1.0.3 plan): the architecture
  is enforced by tested guards with allow-lists. `OB-ARCH-DEPS`
  (`tests/substrate_dependency_contract.rs`) checks each crate's normal and
  build dependencies. `OB-ARCH-LINTS` (`tests/lint_policy_contract.rs`) checks
  that each member inherits the workspace lints. `OB-ARCH-IMPORTS`
  (`tests/architecture_import_contract.rs`) parses sources with `syn` and
  checks which modules may import which paths. `OB-ARCH-PUBAPI`
  (`tests/public_api_contract.rs`) checks, with the `public-api` crate, which
  crates a public interface may name. Rule tables live in
  `tests/architecture/rules.rs`. A plan adds a row per crate and, for each row,
  a seeded violation in a fixture that must fail. `serde` is confined to
  private data-transfer types.
- Decision T1: this plan runs last and marks roadmap tasks 1.0.1 and 1.0.2
  done, because the hello-window application proves both success criteria.
- Decision T5: EP-M6 moves the hello-window bundle from
  `crates/cabochon-escutcheon/tests/fixtures/hello-window/` to
  `crates/cabochon-hello-window/resources/` with `git mv`, and repoints every
  reference, so one copy exists. Under `AM-ESC-1` that bundle already has the
  chapter 36 surface.
- Decision T7: in-app Export writes to the directory given by
  `run --export-dir`, because file dialogs are deferred (decision D8).
- Decision T8 (amended by R15): one pull request per task plan.
- Decision T9: ADR numbers are reserved: 0009 for the 1.0.3 plan, 0010 for
  1.0.2, 0011 for 1.0.4, and 0012 for this plan; 0013 for the 1.0.3 plan's
  Cranelift contingency. Any other pull request needing an ADR takes 0014 or
  later.
- Decision T11: the hello-window binary embeds its resource bundle with
  `include_str!` and loads it through Escutcheon's `MemorySource` by default;
  `--resources DIR` loads a directory through the `cap_std` adapter instead.
- Decision T12: behavioural scenarios split by crate: `furniture.feature`
  drives the contract probe in Clerestory's tests, and `hello_window.feature`
  drives the real application.

Refinement decisions (2026-11-21, author: planning agent; all await approval):

- Decision R1: the UI guidelines at commit `9852939` join the conformance
  basis at the book's substrate profile. Rules are cited by chapter file and
  heading, and by checklist identifier where `book/40-checklist.md` gives one.
  Rationale: the user introduced the book as a required reference. Adopting it
  is this plan's choice; precedence follows C12. Rejected: keeping the earlier
  surface and recording departures, which would leave the reference application
  contradicting the reference text on six points.
- Decision R2: `hello-window`'s surface is the one in
  `book/36-hello-window.md`: three menus, a read-only document with a greeting,
  an explanation, and Export and About buttons, and a status line. The window
  title is the document's name, a resource string. Edit, Window, New, Open, and
  Save are absent as the book's recorded fixture exemptions.
- Decision R3: Clerestory's contract areas that `hello-window` does not show
  (inspector panel, a bound text field, alert and confirmation sheets, a
  destructive action, pane cycling) are exercised by a second resource-defined
  application, the contract probe, authored by this plan under
  `crates/cabochon-clerestory/tests/fixtures/contract-probe/`. Rationale: the
  roadmap requires inspector panels and decision D14 requires text input, while
  the book forbids `hello-window` from adding features. The generic inspector
  panel is furniture: a titled pane of property rows bound to an
  `InspectorModel`. The book's Inspector, with identity and authority blocks,
  is semantic-workspace scope and is excluded.
- Decision R4: command semantics follow the book's command record
  (`book/10-menus.md` "Command records"). Each command has one record: a
  semantic identifier, localized label, input requirement, availability with a
  single resource-defined reason, optional shortcut, and menu location.
  Availability has three values: available, unavailable (shown disabled, still
  reachable by arrow keys, its reason announced), and inapplicable (absent).
  Menu items, buttons, palette results, and shortcuts are views of the record
  and dispatch identically.
- Decision R5: Export and Print are distinct records. In-app Export
  (`file.export`) writes one screen-oriented `content.pdf` and its manifest
  entry to the `--export-dir` directory, and is unavailable, with its reason,
  when none was given. The `export` subcommand does the same without a window.
  The `review` subcommand writes the eight-artefact conformance bundle (`UIG`
  `CON-04`: an ordinary export yields one file; the bundle comes from "the
  review command"). Print (`file.print`) is present in File and takes its
  availability from the shell's declared capabilities; neither adapter in this
  plan can print, so it is unavailable with the reason "printing is not
  available on this host". The print target itself is proved by
  `content-print.pdf` in the bundle. No print request operation is added to the
  port now, because the desktop print portal is asynchronous and belongs to
  roadmap 5.1.2; a synchronous stub would have to be broken later. See `DEP-1`.
- Decision R6: the command palette follows `book/23-command-palette.md` for
  commands. Control+K opens it over the document and closes it when open. It is
  one tab stop: the query field keeps focus while Up and Down move the result
  selection. Matching is case-insensitive, ignores diacritics, and matches
  space-separated words in any order; a leading `>` is accepted. An empty query
  shows the last ten commands under a Recent heading. Tab completes the word
  under the caret from the visible results, or dismisses when no completion is
  offered. Escape clears a completion first and then closes. Unavailable
  results are listed with their reason and never run. The list is exposed as a
  list box with an active option. Objects, tools, and places are excluded, and
  the `@` and `#` prefixes return nothing, because no such results exist
  without Enfilade.
- Decision R7: View is flat: Command palette, Zoom in (Control+=), Zoom out
  (Control+-), Actual size (Control+0), and Dark appearance, a check item. Zoom
  steps through 50, 75, 100, 125, 150, 200, 300, and 400 per cent and scales
  only the document plane; arrow and page keys pan when the document has focus,
  and the wheel scrolls. The initial appearance comes from the host when it
  reports one, else light. Rationale: the book never uses submenus and itself
  names **Dark appearance** as a toggle (`book/10-menus.md` "Wording").
  Rejected: Zoom and Appearance submenus, which add nested menus and radio
  items for no rule.
- Decision R8: F1 dispatches the built-in `help.context`, which shows the
  focused control's resource-defined help string in the status line, or a "no
  help is available" message. A help surface with documentation is later work.
  About sits in Help with no shortcut. Diagnostics (`help.diagnostics`) opens
  an information sheet showing the application and renderer versions, shell
  adapter, locale chain, font collection, command-dispatch counters, and the
  last successful and last failed operation with times from an injected clock.
- Decision R9: dialogs are document-modal sheets drawn inside their window.
  Each declares a purpose (information, input, confirmation, recovery) and
  whether its confirming action is destructive; initial focus and the Return
  default follow `book/22-dialogs.md` "Defaults and focus". Actions sit on the
  trailing edge, safe action first, on every host.
- Decision R10: focus is one state machine per window, held entirely in a
  proved kernel. The proof sketch fixed these representation choices before any
  interface was written. The owner and every remembered control are stable stop
  identifiers, never indices, so inserting or removing a control cannot
  silently move focus. The owner is one optional identifier, not a flag per
  control. The kernel owns the stops, the frame stack, the owner, and the
  transition function, and derives eligibility itself; the session supplies
  only each stop's scope and whether it is operable. Sheets push modal frames;
  menu-bar activation and the palette push transient frames, which Tab leaves.
  Menu items, palette rows, panes, and tab stops share one cyclic-successor
  function. Rationale: keyboard reachability, the absence of keyboard traps,
  and never losing focus are promises every later application inherits; they
  must hold for any number of controls and any nesting depth; and the kernel is
  small, pure, and stable. Rejected: property tests alone, which sample; and a
  proof of the successor function alone, which would leave the risky part (the
  frame stack and edits) unproved.
- Decision R11: Clerestory draws no document header in this plan. The single
  visible title and the window controls belong to the frame the host side
  supplies: the compositor's where it offers server-side decoration, otherwise
  the frame `winit` draws. Clerestory passes the title to the shell and exposes
  it in the accessible name. See `DEP-2`.
- Decision R12: Clerestory keeps its own `Role` and `States` vocabulary,
  aligned with AccessKit's names, and reports accessible bounds and stable node
  identifiers. A window's accessible name carries its role, modality, and
  attachment (`UIG` `WIN-06`). A composite exposes its roving cursor as the
  focused descendant. The hosted adapter maps roles with an exhaustive `match`;
  content nodes map from Lapidary's `SemanticRole`. The accessibility order
  equals the focus order. Assistive-technology actions (focus, activate, set
  value) arrive as host events.
- Decision R13: the session writes journal entries to an injected sink
  (dispatches, focus changes, announcements, and a digest of each published
  accessibility tree in logical units). The application writes the journal file
  in two sections: `[session]`, which is compared between shells, and `[host]`
  (frames presented, shell name, metrics snapshot), which is not. Under the
  hosted adapter the script's steps are injected as host events at the session
  boundary from inside the event loop, because headless `weston` cannot
  synthesize input. `OB-CLR-HOST` therefore proves window creation, configure
  and scale handling, presentation, loop integration, and that the same session
  behaves identically behind both adapters. It does not prove `winit`'s key,
  pointer, or IME mapping, which pure unit tests and the manual check cover.
- Decision R14: the application binary installs a `metrics-util` debugging
  recorder, registers only counters and gauges with fixed labels, and shows a
  snapshot in Diagnostics and, once at exit, in the journal's `[host]` section.
  Rationale: `AGENTS.md` requires new telemetry to be exercised through the
  application's collection path, and the design's session service does not
  exist yet.
- Decision R15: the plan is delivered as a stack of pull requests, one per
  milestone, each a coherent plateau, and the last marks the roadmap entries
  done. Rationale: one pull request of about 17,000 lines cannot be reviewed.
  Amends T8 for this plan. Rejected: two separate task plans (a headless
  contract and an application), which would need a roadmap change; the stack
  gives the same review units.
- Decision R16: `AM-ESC-1` and `AM-BUR-1` are delivered to the sibling plans
  by a separate plan-revision pull request, raised once this revision is
  approved and before the 1.0.2 and 1.0.4 plans are implemented. If either
  sibling is implemented first, EP-M0 makes the same change to its delivered
  code as an additive pull request at the bottom of this plan's stack.
- Decision R17: the headless shell is its own crate,
  `cabochon-clerestory-headless`. Rationale: an adapter inside the application
  model's crate would need an import rule to keep it out, and would ship in
  every dependant.
- Amendment `AM-ESC-1` (1.0.2 plan, schema version 1, all additive):
  1. `command` gains `input` (`none` or `required`, which drives the
     ellipsis), `unavailable` (the string key of its single reason), `help`
     (a string key), and `checkable`.
  2. Controls gain a `help` string key.
  3. `dialog` gains `purpose`; `button` gains `destructive`.
  4. A `window` node carries `title`, `min-size`, and status-line string keys.
  5. A `pane` node names a titled group of controls, for F6.
  6. The built-in commands become `dialog.ok`, `dialog.cancel`,
     `window.close`, `app.quit` (Control+Q), `help.context` (F1),
     `view.command-palette` (Control+K), and `view.next-pane` (F6). A bundle
     labels and places them; it cannot rebind them, and no other command may
     use those keys or Tab, Enter, Space, or Escape.
  7. The plan's example fragment drops `app.about shortcut="F1"`.
  8. The 1.0.2 fixture is authored to the chapter 36 surface (decision R2);
     Escutcheon keeps a small `controls` fixture for the text field and
     property row.
- Amendment `AM-BUR-1` (1.0.4 plan): Burin exposes its renderer version and
  declared parity tolerances as constants so the manifest can record them (`UIG`
  `CON-05`). Fallback if refused: the application records the crate version
  only, as a departure.

Departures from applicable **must** rules of the guidelines (all await the
user's confirmation):

- `DEP-1` (`book/36-hello-window.md`, Print "opens the print surface"): Print
  is present but unavailable under both adapters (decision R5), until roadmap
  5.1.2 delivers the portal.
- `DEP-2` (`UIG` `WIN-07`, decoration profiles): only a host-framed window is
  supported; where `winit` draws the frame, the book would call the profile
  client-decorated and expect Clerestory's document header (decision R11).
  EP-M0 records which frame each target host shows. Owner: roadmap 6.1.2.
- `DEP-3` (`UIG` `FBK-04`, operations over 200 milliseconds announce
  themselves): command handlers are synchronous, so no frame can be shown
  mid-operation. Export reports its outcome on completion; EP-M6 records the
  measured export time, and more than 200 milliseconds on the development host
  triggers escalation. Asynchronous commands and the progress zone belong to
  roadmap 2.1.
- `DEP-4` (`UIG` `LOC-04`, locale date formats): the status line and
  Diagnostics show times in ISO 8601 form, because Escutcheon schema version 1
  has no date formatting.
- `DEP-5` (`book/25-errors-recovery.md` "Diagnostics", copyable bundle):
  omitted until the host clipboard exists (decision D13).
- `DEP-6` (`book/09-windows.md` "Menu bar", the More menu): not implemented;
  a test proves the menu bar fits the declared minimum width in every shipped
  locale, and session start rejects a bundle that does not fit in the active
  one.
- `DEP-7` (`book/06-document-centre.md`, the plane scrolls): keyboard and
  wheel panning only; scroll bars wait for a scrolling container control.

Product questions the user should confirm alongside the decisions: F1 help in
the status line (R8); the primary Export button being unavailable when `run` is
started without `--export-dir` (R5); `review` being a command-line subcommand
only (R5); and dark appearance being supported at all (R7).

## Outcomes & retrospective

Not started.

## Context and orientation

Inspected base: `main` at commit `d295397` (2026-11-21). At that revision the
repository is one package, `cabochon` (`Cargo.toml`, `src/lib.rs`,
`src/main.rs`), with no `[workspace]`, no `crates/` directory, and no
`.config/` directory. Integration tests under `tests/` check the build
standard, the Makefile, and the CI workflows.

When implementation starts, the three earlier plans will have changed that. The
workspace will contain `cabochon-lapidary` (scenes, `resolve`, hit testing,
`damage`, `DeviceMapping`, accessibility nodes, the `TextLayout` port),
`cabochon-burin` (`ParleyTextLayout`, every output, `InteractiveSurface`,
`render_with_fallback`), `cabochon-fonts` (bundled font data),
`cabochon-escutcheon` (`compile`, `ResourceBundle`, `Localizer`,
`MemorySource`, built-in commands, the `escutcheon-check` binary, and a fixture
bundle), `cabochon-substrate-testkit` (`MonospaceTextLayout`, generators), the
architecture guards described under decisions T13 to T15,
`.config/nextest.toml`, and prover tooling. That tooling is
`make install-verus` and `make verus` (Verus proofs under `verus/`, run through
the pinned `prover-tools` command), `make install-kani` and `make kani` (Kani
0.67.0 harnesses), the guard `tests/verus_mirror_contract.rs`
(`OB-PROOF-MIRROR`: no banned construct, every kernel region inside `verus!`
with an `ensures` clause, and on the copy route token identity with production
code), `tests/prover_tooling_contract.rs`, and a `formal.yml` workflow that
runs both provers weekly and on dispatch. EP-M0 confirms each of these.

Gate commands that exist today: `make check-fmt`, `make lint` (Rustdoc, Clippy,
Whitaker), `make test` (nextest plus doctests, with `-D warnings`),
`make typecheck`, `make markdownlint`, `make spelling`, `make nixie`, and
`make audit`. `make test-hosted` does not exist; EP-M8 adds it. CI
(`.github/workflows/ci.yml`) runs on `ubuntu-latest`: an `msrv` job, and a
`build-test` job whose coverage step runs the suite under a ratchet.

Governing documents:

- `docs/cabochon-design.md` §3 (invariants), §6 (the runtime owns no
  top-level windows in hosted mode), §8.1 (Clerestory's responsibilities,
  identical under the hosted adapter and the future Mullion shell), §8.9, §11
  (a renderer failure yields a bounded diagnostic), §13, and §14 (spans and
  low-cardinality metrics; libraries install nothing).
- `docs/adrs/adr-0002-keep-the-hosted-runtime-independent-of-the-shell.md` and
  `docs/adrs/adr-0007-make-the-gem-substrate-a-production-boundary.md`.
- `docs/terms-of-reference.md`, "Success measures": "a developer can create a
  small resource-defined Cabochon application that opens a window, exposes
  standard menus and commands, renders a Lapidary scene, exports the same scene
  through Burin, and integrates with accessibility and keyboard traversal
  without writing host-desktop-specific code." The next measure says the
  hosted-product hypothesis is validated only by a workflow that also exercises
  Enfilade; this plan does not claim it.
- `docs/context.md` and `references/cabochon-white-paper.md` (background).
- `docs/roadmap.md` tasks 1.0.1 and 1.0.2.
- The UI guidelines,
  <https://github.com/leynos/cabochon-ui-guidelines/blob/main/book/README.md>,
  at commit `9852939`: chapters 5, 6, 9, 10, 11, 12, 14, 17, 18, 19, 22, 23,
  25, 36, 37, 38, 39, and 40.
- Testing and style guides: `docs/rust-testing-with-rstest-fixtures.md`
  (fixtures run per test; never hold a display or bus handle in a `#[once]`
  fixture, which is never dropped), `docs/rust-doctest-dry-guide.md` (doctests
  see only public items; use a hidden fallible `main`),
  `docs/reliable-testing-in-rust-via-dependency-injection.md` (inject the
  environment and the clock),
  `docs/complexity-antipatterns-and-refactoring-strategies.md` (guard clauses
  and extracted predicates keep state machines under the complexity limit),
  `docs/scripting-standards.md` (Python with `uv`, tests under
  `scripts/tests/`), and `docs/documentation-style-guide.md` (ADR naming and
  sections; doc-comment layout).

Terms:

- Host shell: the desktop that owns windows and input (GNOME or KDE Plasma).
  In this plan "hosted" always describes the shell adapter, not the runtime.
- Headless shell: an adapter that runs applications without a display,
  recording frames and IME cursor areas.
- Furniture: standard application chrome: menu bar, menus, status line,
  sheets, the palette, and inspector panels.
- Sheet: a dialog attached to, and drawn inside, one window.
- Logical unit (lu): the guidelines' interface length at 100 per cent scale;
  Clerestory converts it to device pixels through the 1/120 scale.
- Tab stop (stop): a control that Tab can reach. A composite (a menu, the
  palette's list) is one stop; arrow keys move a roving cursor inside it.
- Scope: the set of stops one surface owns. A frame is an open surface's
  entry on the focus stack. A modal frame (a sheet) keeps Tab inside its scope;
  a transient frame (the menu bar, the palette) is left by Tab.
- Focus-return chain: where focus goes when a surface closes: the control
  that opened it, else the surviving context, else a safe fallback.
- Responder chain: the order in which a command is offered to handlers:
  focused element, sheet, window, application.
- IME (input method editor): host software that composes text through
  intermediate "preedit" text before committing it.
- Conformance bundle: the eight artefacts the `review` subcommand writes.
- Kernel: a small, total function or state machine proved with Verus.

Exclusions, each with the predicate that makes the owning guideline
inapplicable or the roadmap task that owns the work: file dialogs (decision D8,
roadmap 5.1.2); host clipboard and drag-and-drop (D13); tool strip and masthead
(optional bands, absent); splitters and collapsible panes (panes here are
fixed); context menus (no object under the pointer has commands of its own);
the state chip (no live relationship exists without Enfilade); access keys (the
book makes them optional); palette objects, tools, and places (R6); the book's
Inspector identity and authority blocks (R3); undo (the document is read-only,
and the probe's field commits directly to its model); window geometry
persistence and per-user activation (roadmap 5.1.1); more than one window per
session; and motion (no finite transition is drawn, so reduced motion is
satisfied without a host preference). Departures from applicable rules are the
`DEP-` items in `Decision log`.

Budget survey. No comparable implemented plan exists: the four step-1.0 plans
are approved and unimplemented, and their revision notes record no budget
change. Each used one trigger, 3,000 net Rust lines or 60 files per milestone,
with no configuration or documentation limit. The expert review judged the
first draft's two largest milestones under-budgeted, so both were split. This
plan estimates, in net lines of code by milestone: EP-M0 none committed; EP-M1
2,800; EP-M2 1,200 of proof; EP-M3 2,000; EP-M4 2,700; EP-M5 2,500; EP-M6
2,200; EP-M7 1,800; EP-M8 2,000; EP-M9 100. The total, 17,300, plus 15 per cent
contingency gives the 20,000 limit, and the per-milestone limit of 3,300 gives
the largest milestone 18 per cent. Configuration (four manifests, two bundles
in three locales, one CI job, nextest entries) is estimated at 950 lines.
Documentation (ADR 0012, a design section, users' and developers' guide
sections, layout and contents entries) is estimated at 1,200 lines.

Skills to load: `execplans`, `rust-router` (then `rust-state-machines` for
focus, menus, and text editing, `rust-types-and-apis`, `rust-errors`, and
`domain-cli-and-daemons` for the application), `hexagonal-architecture`,
`rust-verification` and `verus` (EP-M0, EP-M2), `rust-unit-testing`, `proptest`,
`nextest`, `github-stacks`, `arch-decision-records`, `en-gb-oxendict`,
`codegraph-mcp`, and `firecrawl-mcp` for `winit`, `softbuffer`, and AccessKit
documentation.

## Conformance basis

Upstream revisions inspected: `leynos/cabochon` `main` at `d295397` for the
design document, terms of reference, roadmap, and ADRs 0001 to 0008;
`leynos/cabochon-ui-guidelines` at `9852939`. The book records `de40019` as the
platform revision it was checked against; no document cited here changed between
`de40019` and `d295397`. ADR 0012 does not exist yet; this plan writes it. No
supply-chain or security standard beyond `make audit` governs this work.

Derived requirements:

- `REQ-WINDOW`: a session opens one document window from resources, with one
  visible title, a declared minimum size, a role-bearing accessible name, and
  vetoable close (`RM-1.0.1`, `TDD-CLR`, `UIG` `WIN-06`).
- `REQ-COMMAND`: every command is one record; menus, buttons, palette, and
  shortcuts dispatch it identically (`RM-1.0.1`, `UIG` `CTL-12`).
- `REQ-MENU`: an in-window menu bar with state marks, disabled-but-reachable
  items, type-ahead, and the guideline keys (`RM-1.0.1`).
- `REQ-DIALOG`: alert, confirmation, and About sheets with purpose-scoped
  defaults (`RM-1.0.1`, decision D8).
- `REQ-FOCUS`: one focus owner whenever any control is eligible; Tab reaches
  every eligible stop; traps only in modal surfaces, which Escape always
  leaves; focus returns through the chain; a visible ring that changes no layout
  (`RM-1.0.1`, `UIG` `CTL-02`, `CTL-05`).
- `REQ-KEYS`: the routing decision table of `book/18-focus-keyboard.md` and
  its "keys that must work".
- `REQ-TEXT`: a single-line text field with IME composition (decision D14).
- `REQ-INSPECTOR`: a generic inspector panel bound to a model (`RM-1.0.1`).
- `REQ-STATUS`: a status line that is silent by default, expires success
  messages, keeps problems until dismissed, and is a polite live region.
- `REQ-A11Y`: roles, names, states, bounds, and order for every control
  (`RM-1.0.1`, `TOR-SM-GEM`, `UIG` accessibility).
- `REQ-L10N`: `en-XA` and `ar-XB` run without clipping or reversed order, and
  every control stays reachable at 200 per cent scale (`UIG` `CON-07`,
  `LOC-07`).
- `REQ-HOST`: the same session runs behind the headless and hosted adapters
  with no host type in the application (`RM-1.0.1`, `TDD-INV-6`, ADR 0002).
- `REQ-APP`: `hello-window` matches chapter 36 and takes its interface from
  resources (`RM-1.0.2`, `UIG` `CON-01` to `CON-06`).
- `REQ-OBS`: dispatch is traced and counted through the application's own
  collection path (`TDD-OBS`).
- `REQ-FAIL`: a renderer, resolution, or presentation failure leaves the
  session running with a bounded diagnostic (`TDD-FAIL`).

Gaps at the base revision: `GAP-NO-CLERESTORY` (no crate, port, or adapter
exists), `GAP-NO-APP` (no resource-defined application), and `GAP-NO-HOSTED-CI`
(no CI job has a display).

```plaintext
RM-1.0.1 -> REQ-FOCUS -> EP-M1, EP-M2 -> OB-CLR-FOCUS-KERNEL, OB-CLR-FOCUS, OB-CLR-MODAL
RM-1.0.1 -> REQ-COMMAND, REQ-KEYS -> EP-M1 -> OB-CLR-ROUTE, OB-CLR-KEYS
RM-1.0.1 -> REQ-L10N -> EP-M3 -> OB-CLR-LAYOUT, OB-CLR-VIEW
RM-1.0.1 -> REQ-WINDOW, REQ-MENU, REQ-DIALOG, REQ-STATUS -> EP-M4 -> OB-CLR-WINDOW, OB-CLR-MENU, OB-CLR-DIALOG, OB-CLR-STATUS
RM-1.0.1 -> REQ-A11Y -> EP-M4, EP-M8 -> OB-CLR-A11Y, OB-HOST-A11Y
RM-1.0.1 -> REQ-TEXT, REQ-INSPECTOR -> EP-M5 -> OB-CLR-TEXT, OB-CLR-PALETTE, OB-CLR-INSPECTOR, furniture.feature
RM-1.0.1 -> TDD-INV-6, ADR-0002 -> REQ-HOST -> EP-M8 -> OB-CLR-HOST, make test-hosted
RM-1.0.2 -> REQ-APP -> EP-M6 -> OB-APP-RESOURCES, OB-APP-SURFACE, OB-APP-EXPORT, hello_window.feature
TOR-SM-GEM -> RM-1.0.1..4 -> EP-M6, EP-M7, EP-M8 -> OB-APP-EXPORT, OB-APP-E2E, OB-CLR-HOST
TDD-OBS -> REQ-OBS -> EP-M1, EP-M7 -> OB-CLR-OBS
TDD-FAIL -> REQ-FAIL -> EP-M1, EP-M6 -> OB-CLR-FAIL
GAP-NO-CLERESTORY -> EP-M1; GAP-NO-APP -> EP-M6; GAP-NO-HOSTED-CI -> EP-M8
```

## Verification plan

Clerestory's obligations fall into three kinds. The focus rules are a
transition system whose guarantees must hold for every window layout and every
nesting depth; they get a formal proof. The key-routing rules are a finite
table; they get an exhaustive table test. Everything else is orchestration,
layout, or integration, and gets property tests against independent oracles,
named cases, snapshots, behavioural scenarios, and end-to-end tests.

### Abstract model

The focus model is written before the implementation and independently of it.

- A window has a finite sequence `stops` in reading order. Each stop has an
  identifier, unique in the window, a scope, and a flag `operable` (visible and
  enabled).
- `frames` is a non-empty sequence. Each frame has a scope, a kind (base,
  modal, or transient), and an ordered list of return candidates, which are
  stop identifiers. The bottom frame, and only it, is the base frame. No scope
  appears twice. The active scope is the top frame's.
- A stop is eligible when it is operable and its scope is the active scope.
  The mask is the sequence of eligibility flags.
- `owner` is an optional stop identifier.
- `succ(mask, from)`: let `n` be the mask's length. When `from` is an index
  `i < n`, the result is `(i + d) mod n` for the least `d` in `1..=n` whose bit
  is true, or nothing when no bit is true. When `from` is absent or out of
  range, the result is the least true index, or nothing. `pred` is the mirror
  image.
- `rehome(p)` is the first eligible index at or after position `p`,
  cyclically, or nothing.

Transitions. Each is total; a rejected transition leaves the state unchanged.

- `Next` and `Previous` (Tab and Shift+Tab). Under a transient top frame they
  first perform `Close` on it. They then set the owner to the stop at `succ` or
  `pred` of the owner's index.
- `Focus(id)` sets the owner when that stop is eligible, and is otherwise
  rejected. Pointer and assistive-technology focus use it.
- `SetOperable(id, flag)`, `Insert(position, stop)`, and `Remove(id)` edit
  `stops`. `Insert` is rejected for a duplicate identifier or a position past
  the end.
- `Open(scope, kind, entry)` is rejected when the kind is base, the scope is
  already on the stack, or no stop in that scope is operable. Otherwise it
  pushes a frame whose return candidates are the current owner followed by any
  context candidates the session supplies, and sets the owner to the first
  eligible entry candidate, else the first eligible stop.
- `Close(scope)` is rejected when the scope is the base frame's or is not on
  the stack. Otherwise it removes that frame and every frame above it. The
  owner becomes the first eligible return candidate of the removed frame; else,
  if its first candidate still exists, the stop at `succ` of that candidate's
  index; else the first eligible stop.
- Normalization ends every transition. If the owner is not eligible, the
  owner becomes `succ` of its index when its stop still exists, and `rehome(p)`
  when it was removed from position `p`. If there is no owner and some stop is
  eligible, the owner becomes the first eligible stop.

A roving cursor (the items of a menu, the rows of the palette, the panes of a
window) is a separate `(mask, cursor)` pair using the same `succ` and `pred`. A
menu's mask keeps disabled items and drops separators. Replacing the mask, as
the palette does on each keystroke, moves the cursor to the first true bit.

### Invariants and lemmas

- `INV-FOCUS-WF` (`REQ-FOCUS`): stop identifiers are distinct; `frames` is
  non-empty, its bottom frame is the only base frame, and its scopes are
  distinct; the owner is absent or names an eligible stop; and the owner is
  absent exactly when no stop is eligible. One owner at most holds by
  representation; "focus is never lost" is the last clause.
- `INV-ROVING`: a cursor is absent or an in-range true bit, and is absent
  exactly when the mask has no true bit.
- `L-INIT-WF`: the initial state (one base frame, owner the first eligible
  stop if any) satisfies `INV-FOCUS-WF`.
- `L-STEP-PRESERVES`: every transition, accepted or rejected, preserves
  `INV-FOCUS-WF`. With `L-INIT-WF` the invariant holds after every sequence.
- `L-RING-TOTAL`: `succ` returns nothing exactly when the mask has no true
  bit; any index it returns is in range and true. Likewise `pred`.
- `L-RING-NEAREST`: `succ` satisfies the definition in `Abstract model`,
  which determines it uniquely. In particular a mask whose only true bit is `i`
  gives `succ(mask, i) = i`.
- `L-RING-INVERSE`: for a true bit `i`, `pred(mask, succ(mask, i)) = i` and
  `succ(mask, pred(mask, i)) = i`.
- `L-RING-CYCLE` (keyboard reachability): if the mask has `k ≥ 1` true bits
  and `i` is one, then for every true bit `j` there is exactly one `t` in
  `0..k` with `succ` applied `t` times to `i` giving `j`, and `succ` applied
  `k` times gives `i`. One Tab cycle therefore visits every eligible stop
  exactly once.
- `L-SCOPE-CONFINE`: under a base or modal top frame, `Next` and `Previous`
  leave `frames` unchanged and produce an eligible owner. Traversal cannot
  leave a modal surface, and by `L-RING-CYCLE` reaches all of it.
- `L-SCOPE-ESCAPE` (no keyboard trap): when the stack is deeper than one,
  `Close` on the top frame's scope is accepted and shortens the stack; a stack
  of depth `d` reaches the base frame after `d − 1` closes; and under a
  transient top frame `Next` also shortens it.
- `L-SCOPE-RESTORE` (focus-return chain): suppose the owner is `o`, a frame
  is opened, and it is later closed, and in between no transition removes `o`,
  makes it inoperable, or changes the frames beneath. Then the owner is `o`
  again. If `o` was made inoperable but still exists, the owner is the next
  eligible stop after it. Nested opens unwind in order to `o`.
- `L-EDIT-STABLE`: `Insert`, and `Remove` or `SetOperable` applied to a stop
  other than the owner, never change an existing owner.
- `INV-ROUTE` (`REQ-COMMAND`): a dispatch reaches the first responder in the
  chain that accepts it, an unavailable command is never delivered, and a
  record's dispatch is the same from every view.
- `INV-LAYOUT` (`REQ-L10N`): children lie inside their parent, siblings do
  not overlap, and layout is a function of its inputs.
- `INV-A11Y` (`REQ-A11Y`): every operable control has a role and a non-empty
  name unique within its group; parent and child links agree; node identifiers
  are stable across republication; the tree's pre-order of operable nodes
  equals the focus order; the focus owner, or its roving cursor's item, is the
  tree's focus.
- `INV-TEXT` (`REQ-TEXT`): the caret rests only on caret stops; preedit text
  is never committed without a commit event; the reported IME cursor area
  contains the caret.
- `INV-VIEW`: the zoom level is one of the declared steps, and the pan
  offset lies between zero and the zoomed document's overflow on each axis.

### Axioms (trusted, not verified here)

- `AXIOM-VERUS`: the pinned Verus and Z3 are sound, and on the single-source
  route `verus_builtin_macros` erases only ghost code (the 1.0.3 plan's
  `A-VERUS`). Depends on it: `OB-CLR-FOCUS-KERNEL`.
- `AXIOM-LAP-HIT`: Lapidary's hit testing returns the topmost covering
  region (the 1.0.3 plan's `OB-LAP-HIT`). Depends on it: pointer routing.
- `AXIOM-LAP-UNITS`: Lapidary's `DeviceMapping` converts between scene and
  device units without overflow (the 1.0.3 plan's Kani obligations). Depends on
  it: logical-unit conversion, which Clerestory delegates and does not repeat.
- `AXIOM-LAP-CARET`: the `TextLayout` port reports caret stops at grapheme
  cluster boundaries. Depends on it: `OB-CLR-TEXT`. Checked at the boundary by
  a named case with a combining sequence and an emoji sequence under
  `ParleyTextLayout`.
- `AXIOM-BUR-INVAL`: Burin redraws no pixel outside the damage it is given
  (the 1.0.4 plan's `OB-BUR-INVAL`). Depends on it: redraw.
- `AXIOM-WINIT`: `winit` and `softbuffer` deliver input and present buffers
  as documented, and headless `weston` represents GNOME and KDE Plasma closely
  enough for a smoke test. Residual: host-specific behaviour, real input, IME,
  fractional scale, and AT-SPI read-back (roadmap 5.1.4).
- `AXIOM-ACCESSKIT`: `accesskit_winit` publishes the supplied tree to AT-SPI
  faithfully once activated.

### Methods per obligation

Property suites share these mechanics. Each runs through a manual
`proptest::test_runner::TestRunner` with a fixed seed, accumulates per-class
counters, and asserts minimum counts after the run, because `proptest` has no
built-in classification. Generators construct the important classes directly. A
suite runs 256 cases; state-machine suites use up to 32 operations per case, a
monospace `TextLayout`, and a bounded shrink budget. Regression files under
`proptest-regressions/` are committed. A seeded fault is a temporary edit,
applied to see the named check fail and then reverted with `git restore`; its
transcript goes in `Artefacts and notes`, and no fault is committed.

- Obligation `OB-CLR-FOCUS-KERNEL`: `L-INIT-WF`, `L-STEP-PRESERVES`, the four
  `L-RING` lemmas, the three `L-SCOPE` lemmas, `L-EDIT-STABLE`, and
  `INV-ROVING`, over `crates/cabochon-clerestory/src/kernel/`, which contains
  the whole focus state and its transition function. Method: Verus proof.
  Rationale: the properties quantify over all layouts, stack depths, and
  operation sequences; the kernel is pure, and the proved object is the
  specified one. Domain: all states satisfying the invariant and all
  transitions; no transition has a precondition. Artefact:
  `verus/clerestory_proofs.rs`, or the kernel modules themselves on the
  single-source route. Evidence: `make verus`; red first by running it with the
  lemma statements and empty proof bodies and recording the open goals;
  discharged when it reports every function verified with no error and
  `OB-PROOF-MIRROR` passes. Non-vacuity: `L-INIT-WF` shows the invariant is
  inhabited; a proof function exhibits concrete states reaching each accepted
  and each rejected transition, a two-deep stack, a close whose first candidate
  is inoperable, and, for `L-SCOPE-RESTORE` and `L-RING-CYCLE`, a state
  satisfying every hypothesis; seeded faults: scanning from `from` instead of
  `from + 1` must fail `L-RING-NEAREST` on the single-bit mask, and omitting
  normalization after `SetOperable` must fail `L-STEP-PRESERVES`. Residual: the
  session code that reports scopes and operability to the kernel.
- Obligation `OB-CLR-FOCUS` (`INV-FOCUS-WF` at the public interface, and an
  executable twin of each lemma). Method: `proptest-state-machine` driving
  `Session` through host events against a reference model written from
  `Abstract model` over plain vectors and linear search, sharing no code with
  the kernel; plus named cases. Rationale: it ties the proved kernel to what
  the session really reports. Domain: windows of 0 to 24 stops with sheets, the
  palette, and menu activation; Tab, Shift+Tab, enable, disable, show, hide,
  add, remove, pointer focus, and open and close of each surface. Artefact:
  `crates/cabochon-clerestory/tests/focus_machine.rs`. Evidence:
  `cargo nextest run -p cabochon-clerestory focus_machine`; red against a
  session whose Tab ignores scope. Non-vacuity: at least 20 per cent of cases
  with a modal frame open, 10 per cent closing over an inoperable invoker, 10
  per cent with an insertion or removal before the owner, and 5 per cent with
  no eligible stop; seeded fault: restoring focus to the invoker without
  checking that it is operable.
- Obligation `OB-CLR-MODAL`: while a sheet is open, no key or pointer event
  reaches a control outside it, and Escape always closes it. Method: the same
  state machine with an event-delivery oracle, plus the "Dialogs are modal"
  scenario. Non-vacuity: at least 20 per cent of cases deliver a pointer event
  outside an open sheet; seeded fault: hit testing the window before the sheet.
- Obligation `OB-CLR-KEYS` (`REQ-KEYS`): the pure function
  `route_key(context, modal, key)` returns the disposition the guideline's
  routing table gives. The domain is six focus contexts (composition, text
  field with completion, text field, composite, button, plain) by two modal
  states by ten keys (Tab, Shift+Tab, Enter, Space, Escape, arrow, F1, F6, Alt,
  other chord): 120 cells. Method: an `rstest` table with one case per
  normative cell, plus a totality test that iterates the three enumerations and
  asserts every cell has an expected value. Artefact:
  `crates/cabochon-clerestory/src/key_routing/tests.rs`. Non-vacuity: the
  totality test fails when a variant is added without rows; seeded fault:
  letting Enter activate the default while a button has focus.
- Obligation `OB-CLR-ROUTE` (`INV-ROUTE`). Method: a property test over
  generated responder chains and availability, with an oracle that scans the
  chain independently; named cases for the vetoable `window.close`, for session
  start failing when the application declares a command the bundle lacks, and
  for an assistive-technology activation dispatching the same record as a key.
  A proof is not used: the specification of first-match dispatch is the same
  fold as its implementation, so a proof would restate it. Non-vacuity: at
  least 30 per cent of chains have two or more accepting responders and 20 per
  cent an unavailable command; seeded fault: offering the command to the
  application first.
- Obligation `OB-CLR-OBS` (`REQ-OBS`): dispatch emits a span and increments
  `clerestory_command_dispatch_total` with the fixed `outcome` labels
  `dispatched`, `unavailable`, `unhandled`, and `vetoed`. Method: a named case
  with a local recorder in Clerestory; and, in `OB-APP-E2E`, a non-zero counter
  read through the binary's own recorder.
- Obligation `OB-CLR-FAIL` (`REQ-FAIL`): a shell whose `present` fails leaves
  the session running, marks the window for retry, journals the failure, and
  counts it in `clerestory_shell_error_total`; a content scene that fails to
  resolve is replaced by a bounded diagnostic scene; a lost window ends the
  session cleanly. Method: named cases with a failing fake shell and a failing
  application. Seeded fault: propagating the presentation error with `?`.
- Obligation `OB-CLR-LAYOUT` (`INV-LAYOUT`, `REQ-L10N`). Method: property
  test over generated layout trees with a geometric oracle that checks
  containment and disjointness directly from the output rectangles; `insta`
  snapshots of the probe's sheets in `en-GB`, `en-XA`, and `ar-XB`, as text
  summaries of named rectangles in logical units, with no paths, times, or font
  metrics; named cases for the band heights (menu bar 29 lu, status line 26
  lu), control targets (28, 32, 36 lu), sheet width (420 to 560 lu), the 512 by
  384 lu floor, 200 per cent scale, the focus ring (3 lu at 3 lu offset,
  changing no layout bounds, muted when the window is inactive), and the
  contrast ratios of both themes computed from the colour table. A proof is not
  used: the layout engine will change with later profiles and is not yet a
  stable kernel. Non-vacuity: at least 30 per cent of trees nest three levels
  deep and 20 per cent have a child whose natural size exceeds its share;
  seeded fault: dropping the gap between children from the extent sum.
- Obligation `OB-CLR-VIEW` (`INV-VIEW`): zoom steps and pan clamping. Method:
  property test over viewport sizes, document sizes, zoom steps, and pan
  requests, with an oracle computed in 128-bit integers; named cases at the
  extremes. Kani is not used: unit conversion is delegated to Lapidary's
  already-checked `DeviceMapping`, and what remains is a clamp. Seeded fault:
  clamping to the document size instead of the overflow.
- Obligation `OB-CLR-WINDOW` (`REQ-WINDOW`): the window's title, minimum
  size, and accessible name come from resources; a close request is dispatched
  as `window.close` and can be vetoed; session start rejects a bundle whose
  menu bar or widest sheet exceeds the minimum size in the active locale; no
  document header is drawn. Method: named cases.
- Obligation `OB-CLR-MENU` (`REQ-MENU`): Alt opens a transient frame on the
  menu bar; arrows reach disabled items and skip separators; a letter jumps to
  the next item starting with it; Enter on a disabled item dispatches nothing
  and announces its reason; Escape, and Tab, close the menu and return focus; a
  check item shows its state; a menu taller than the space below flips or
  scrolls and stays inside the window. Method: the focus state machine extended
  with menu operations, named cases, and a property on generated menu sizes and
  window heights. Seeded fault: skipping disabled items.
- Obligation `OB-CLR-DIALOG` (`REQ-DIALOG`): for each purpose and
  destructive flag, initial focus and the Enter default are those of
  `book/22-dialogs.md`; Escape performs the safe exit; opening a sheet while
  one is open is rejected. Method: an `rstest` table over purpose,
  destructiveness, and key. Seeded fault: making the destructive button the
  Enter default.
- Obligation `OB-CLR-STATUS` (`REQ-STATUS`): the status line is empty by
  default and never shows "Ready"; a success message clears when its wake
  arrives; a problem stays until dismissed; `help.context` shows the focused
  control's help string; each message is announced politely once. Method: named
  cases with the headless shell's injected clock.
- Obligation `OB-CLR-A11Y` (`INV-A11Y`). Method: property test over
  generated bundles and operation sequences, comparing the tree with the focus
  model and with the previous tree's identifiers; named cases for states
  (unavailable, checked, expanded, read-only, modal) and for the window's
  accessible name. Seeded fault: emitting the tree in paint order.
- Obligation `OB-CLR-TEXT` (`INV-TEXT`). Method: `proptest-state-machine`
  over key and IME events against a reference model on a vector of grapheme
  clusters; named cases for the routing rows (Enter commits the field's own
  action, Escape reverts the draft). Non-vacuity: at least 25 per cent of cases
  contain a preedit that is cancelled; seeded fault: committing preedit on
  focus loss.
- Obligation `OB-CLR-PALETTE`: the result list is exactly the applicable
  commands matching the query under decision R6's rules, in menu order;
  unavailable results carry their reason and never run; Tab completes or
  dismisses; Control+K toggles; the recent list holds at most ten. Method:
  property test with an independent matching oracle; named cases, including a
  query with diacritics and one in `en-XA`. Seeded fault: dropping unavailable
  commands.
- Obligation `OB-CLR-INSPECTOR` (`REQ-INSPECTOR`): a panel shows one
  property row per model entry; committing a field calls the application once
  with the key and text; a rejected commit keeps the draft and shows the
  reason; F6 moves focus between the document and the panel. Method: named
  cases and the inspector scenarios.
- Obligation `OB-APP-RESOURCES` (`REQ-APP`): the application builds its
  window, menus, sheets, strings, shortcuts, and icon only from the compiled
  bundle, loaded through `MemorySource` by default; changing a string in a copy
  of the bundle passed with `--resources` changes the presented title without
  recompiling. Method: end-to-end test with `assert_cmd`.
- Obligation `OB-APP-SURFACE` (`REQ-APP`, `REQ-L10N`): the presented menus,
  items, buttons, and status line are exactly those of
  `book/36-hello-window.md`, and the menu bar and About sheet fit the minimum
  window in every shipped locale. Method: a named test comparing the journal's
  first accessibility tree with a table transcribed from the chapter; `insta`
  layout summaries in the three locales. Seeded fault: adding an Edit menu.
- Obligation `OB-APP-EXPORT`: `review` writes all eight artefacts from one
  scene revision into a staging directory beside the target and renames it into
  place; with `--force` it first renames the old directory aside and removes it
  only after the new one is in place; `export` writes one file and its manifest
  entry; both refuse a non-empty directory they did not create unless `--force`
  is given; a failure injected after the seventh artefact leaves the previous
  output intact and no staging directory behind; an in-app export that fails
  reports a problem in the status line and the journal; the manifest records
  intents, scene revision, versions, tolerances, fonts and substitutions, and
  which identity fields apply; `en-XA` and `ar-XB` also produce the bundle.
  Method: end-to-end tests; `insta` snapshot of the manifest with versions and
  times normalized.
- Obligation `OB-APP-E2E`: a scripted headless session exercising menus,
  shortcuts, the palette with IME text, About, Diagnostics, zoom, pan, dark
  appearance, the unavailable Print, Tab traversal, and quit produces the
  expected journal and exit status in `en-GB`, `en-XA`, and `ar-XB`. Method:
  end-to-end test; the script names labels by string key, so it is
  locale-independent.
- Obligation `OB-HOST-A11Y`: the hosted adapter's conversion of an
  `AccessibilityTree` to an AccessKit `TreeUpdate`, fed to
  `accesskit_consumer::Tree`, preserves roles, names, states, bounds, order,
  and focus, and the role mapping covers every `Role`. Method: unit tests in
  the hosted crate; no display needed.
- Obligation `OB-CLR-HOST` (`REQ-HOST`): with the window size, scale,
  appearance, clock, and fonts fixed, the script (without IME steps) run behind
  the headless adapter and behind the hosted adapter inside headless `weston`
  yields identical `[session]` journal sections; the hosted run reports the
  configured size it was given, presents at least one frame, and shows one
  title. Method: end-to-end test run by `make test-hosted`, starting the
  application as a child process. Non-vacuity: the test first asserts the
  section holds at least one dispatch, one focus change, and one tree digest,
  and that the configured size matched the request, so two empty or mismatched
  runs cannot pass. Scope of the claim: decision R13.

### Formal verification decision

Verus is used for the focus kernel (decision R10) and nowhere else. A proof
adds a guarantee over all layouts and all operation sequences that sampling
cannot give, for the property that users of keyboards and screen readers depend
on most. It leaves unverified the session code that reports scopes and
operability, which `OB-CLR-FOCUS` covers through the public interface. Kani is
not used, for the reason beside `OB-CLR-VIEW`. Command routing, layout, text
editing, diagnostics formatting, the command-line interface, and adapter
plumbing carry no proof obligation, for the reasons given beside each.

### Behavioural specification

The runner is `rstest-bdd`: each scenario binds with
`#[scenario(path = "...", name = "...")]`, and step functions take fixtures by
parameter name. The command is `make test`.

`crates/cabochon-clerestory/tests/features/furniture.feature` (EP-M4 and EP-M5;
drives the contract probe; `REQ-FOCUS`, `REQ-MENU`, `REQ-DIALOG`, `REQ-TEXT`,
`REQ-INSPECTOR`, `REQ-A11Y`):

```gherkin
Feature: Clerestory furniture behaves consistently

  Scenario: Every view of a command dispatches the same record
    Given the contract probe under the headless shell
    When the user opens the palette, types the label of "help.about", and presses "Enter"
    And the user presses "Escape"
    And the user chooses "Help" then "About"
    Then the journal shows "help.about" twice with identical records

  Scenario: Dialogs are modal and restore focus
    Given the probe's greeting field has focus
    And the About sheet is open
    When the user presses "Tab" five times
    Then focus never leaves the sheet
    When the user presses "Escape"
    Then focus returns to the greeting field

  Scenario: Focus falls back when the invoker becomes unavailable
    Given a confirmation sheet opened from the "Reset" button
    When the application disables "Reset" and the user presses "Escape"
    Then focus moves to the next operable control after "Reset"

  Scenario: A destructive confirmation defaults to the safe action
    Given a destructive confirmation sheet is open
    When the user presses "Enter"
    Then the journal shows "dialog.cancel" and no destructive command

  Scenario: Input methods compose text before committing it
    Given the probe's greeting field has focus
    When the input method sends preedit "ni" and then commits "你"
    And the user presses "Enter"
    Then the application receives the greeting "你" once
    And only the greeting's bounds are damaged

  Scenario: A disabled menu item is reachable and inert
    Given the probe with "Export" unavailable
    When the user opens "File" and presses "Down" until "Export" is highlighted
    Then the item's reason is announced
    When the user presses "Enter"
    Then nothing is dispatched and the menu stays open

  Scenario: Tab leaves the menu bar
    Given the probe's greeting field has focus
    When the user presses "Alt" and then "Tab"
    Then no menu is open and focus is on the control after the greeting field

  Scenario: F6 cycles panes
    Given the probe with focus in the document
    When the user presses "F6"
    Then focus is in the inspector panel

  Scenario: Furniture is accessible
    Given the contract probe under the headless shell
    Then every operable control has a role, a unique non-empty name, and bounds
    And the accessibility order equals the Tab order
```

`crates/cabochon-hello-window/tests/features/hello_window.feature` (EP-M6 and
EP-M7; `REQ-APP`, `REQ-L10N`):

```gherkin
Feature: The hello-window application is defined by its resources

  Scenario: The window matches the reference surface
    Given the hello-window application under the headless shell
    When the session starts
    Then the menu bar lists "File", "View", and "Help" and no other menu
    And "File" lists "Export", "Print", and "Quit"
    And the document shows a primary "Export" button and an "About" button
    And a status line is present and empty

  Scenario: Strings change without recompiling
    Given a copy of the resource bundle whose document title is "Bonjour"
    When the application starts with that bundle through --resources
    Then the presented window is titled "Bonjour"

  Scenario: Export says why it is unavailable
    Given the hello-window application started without an export directory
    When the user opens "File" and highlights "Export"
    Then the announced reason is the bundle's "no export directory" string

  Scenario: Print is present and honest
    Given the hello-window application under the headless shell
    When the user opens "File" and highlights "Print"
    Then "Print" is listed, unavailable, and announces that this host cannot print

  Scenario: A successful export is reported and then cleared
    Given the hello-window application with an export directory
    When the user activates the "Export" button
    Then the directory contains "content.pdf" and its manifest entry
    And the status line reports the export
    When five seconds pass
    Then the status line is empty

  Scenario: The interface survives the pseudo-locales
    Given the hello-window application with locale "ar-XB"
    Then the menu bar starts at the right edge
    And no label is clipped
```

## Plan of work

Red-Green-Refactor throughout: write the failing test or lemma statement, see
it fail for the stated reason, make it pass with the smallest change, then
tidy. Each milestone ends with `make check-fmt`, `make lint`, and `make test`
passing, run sequentially through the `scrutineer` agent, then a commit and its
own stacked pull request (decision R15). Any milestone that changes a file under
`kernel/` also ends with `make verus`. Each dependency addition is its own
commit followed by `make audit`.

### EP-M0: feasibility probes and prerequisite check (prototype)

This milestone resolves named uncertainties and commits no production code,
except under step 1's fallback. Probes live in a scratch directory outside the
repository and are deleted afterwards; transcripts go in `Artefacts and notes`.

1. Prerequisites. Confirm the deliverables listed in `Context and orientation`;
   record the delivered names of the Lapidary, Burin, and Escutcheon types this
   plan consumes, and which D11 proof route 1.0.3 took; confirm `AM-ESC-1` and
   `AM-BUR-1` were delivered. Go: all present. If an amendment is missing from
   delivered code, apply decision R16's fallback as the first pull request of
   the stack. If a deliverable is missing, set `Status: BLOCKED` and name it.
2. Hosted provisioning. In a throwaway crate, open a `winit` 0.30 window
   under `weston --backend=headless` with a private runtime directory,
   `WAYLAND_DISPLAY` set, and `DISPLAY` unset, and present one `softbuffer`
   frame. Record: whether a frame is presented locally and on the pinned
   `ubuntu-24.04` runner; both `weston` versions and the accepted flag
   spellings; whether the compositor or `winit` draws the frame (for `DEP-2`);
   whether a seat exists and keys can be synthesized; whether the AccessKit
   activation handler runs after `org.a11y.Status.IsEnabled` is set inside
   `dbus-run-session`; and how the coverage ratchet treats an ignored test's
   uncovered lines. Go: a frame is presented in both places. No-go: the hosted
   tolerance triggers. The other answers adjust EP-M8's assertions only.
3. Proof shape. Write the kernel state, `succ`, and the statements of
   `L-RING-NEAREST`, `L-RING-CYCLE`, `L-STEP-PRESERVES`, and `L-SCOPE-RESTORE`
   in a scratch Verus file and run the pinned Verus. Expected: open goals, with
   no error about unsupported constructs. Record the wall time. Go: the
   statements are accepted. No-go: the proof tolerance applies.
4. Tooling. Confirm `rstest-bdd` builds with the workspace's `rstest`, the
   `msrv` job is unaffected, and `#[gtest]` before `#[rstest]` compiles.

### EP-M1: focus kernel, routing, session, and the headless shell

Assigned: `REQ-FOCUS`, `REQ-COMMAND`, `REQ-KEYS`, `REQ-OBS`, `REQ-FAIL`,
`GAP-NO-CLERESTORY`; decisions R4, R10, R13, R17.

Create `crates/cabochon-clerestory` and `crates/cabochon-clerestory-headless`,
and add their rows to the architecture guards, running each seeded violation
red first. Write, in this order:

1. `kernel::ring` (`succ`, `pred`, the roving cursor) and `kernel::focus`
   (stops, frames, owner, the transition function, normalization), shaped for
   proof as the 1.0.3 plan shapes its kernels: slices and vectors, `while`
   loops, no preconditions. Red: unit tests for the cases named in
   `Abstract model`, including insertion before the owner and removal of the
   owner.
2. `OB-CLR-FOCUS`'s reference model and state machine, at first against the
   kernel directly. Red: it fails against a transition that ignores scope.
3. `key_routing::route_key` with `OB-CLR-KEYS`.
4. `command` (records, availability, responder chain, dispatch) with
   `OB-CLR-ROUTE` and `OB-CLR-OBS`.
5. The `Application` trait, `Session`, `HostEvent`, the `HostShell` and
   `JournalSink` ports, and the two-phase `handle` (compute the transition and
   its effects, then apply the effects) with `OB-CLR-FAIL`.
6. `HeadlessShell`, with an injected clock and a wake queue.
7. `OB-CLR-FOCUS` and `OB-CLR-MODAL` through `Session`, with placeholder
   surfaces.

Draft ADR 0012 as `docs/adrs/adr-0012-<title-slug>.md` with status Proposed,
and record the crates' ownership and reuse policy in `docs/developers-guide.md`.

### EP-M2: Verus proofs of the focus kernel

Assigned: `REQ-FOCUS`; `OB-CLR-FOCUS-KERNEL`.

Add `verus/clerestory_proofs.rs`. Write every lemma statement first and run
`make verus` to record the open goals. Prove, in this order, the ring lemmas and
`INV-ROVING`; `L-INIT-WF` and `L-STEP-PRESERVES`; then `L-SCOPE-CONFINE`,
`L-SCOPE-ESCAPE`, `L-SCOPE-RESTORE`, and `L-EDIT-STABLE`. Supporting lemmas,
triggers, and collection reasoning are worked out here, against the real
verification conditions, and not before. Change the `verus` recipe to take a
list of proof files (`VERUS_PROOF_FILES`, one `--proof-file` argument each),
extend `tests/prover_tooling_contract.rs` to expect both files, extend
`OB-PROOF-MIRROR` to count the Clerestory kernel regions, and add a
`pull_request` trigger to `.github/workflows/formal.yml` filtered to `verus/**`
and `crates/*/src/kernel/**`, keeping the workflow contract tests green. Apply
and revert the two seeded faults. If a proof forces a kernel change, update
`Abstract model`, the reference model, and `OB-CLR-FOCUS` in the same commit.
Record the proof inventory, and what it does not cover, in
`docs/developers-guide.md` and the design document's §8.1 draft.

### EP-M3: measures, theme, layout, and view arithmetic

Assigned: `REQ-L10N`; decisions D10, R7.

Implement `measure` (the guideline lengths as named constants in logical units,
each citing its chapter, converted through Lapidary's `DeviceMapping`), `theme`
(light and dark colour roles from `book/12-palette.md`, including the focus
ring), `view` (zoom steps and pan clamping), and the layout engine (`column`,
`row` with `align`, `label`, `button`, `text-field`, `separator`, `spacer`,
`property-row`, and `pane`; measured through the `TextLayout` port; mirrored
for right-to-left locales). Author the contract probe bundle. Write
`OB-CLR-LAYOUT` and `OB-CLR-VIEW`. Clerestory's unit tests use a small monospace
`TextLayout` in the crate's own test support; integration tests use the
testkit's `MonospaceTextLayout`; snapshots and features use Burin's
`ParleyTextLayout` with bundled fonts, a development-only edge.

### EP-M4: window, menus, status line, sheets, and the accessibility tree

Assigned: `REQ-WINDOW`, `REQ-MENU`, `REQ-DIALOG`, `REQ-STATUS`, `REQ-A11Y`;
decisions R8 (`help.context`), R9, R11, R12.

Implement the furniture scene builders for the menu bar, menus with state
marks, the status line, sheets, and the focus ring; menu-bar activation,
type-ahead, and flipping; purpose-scoped sheet defaults; status messages with
expiry through `request_wake`; and `accessibility::AccessibilityTree`, grafting
the content scene's nodes under the document node. Write `OB-CLR-WINDOW`,
`OB-CLR-MENU`, `OB-CLR-DIALOG`, `OB-CLR-STATUS`, `OB-CLR-A11Y`, and the menu,
dialog, and accessibility scenarios of `furniture.feature`.

### EP-M5: text input, command palette, inspector panel

Assigned: `REQ-TEXT`, `REQ-INSPECTOR`, `REQ-COMMAND`; decisions R3, R6, D14.

Implement the single-line text field (caret movement by caret stop, selection,
preedit, commit, revert, IME cursor area and allowance), the command palette
over it (matching through `unicode-normalization`, completion, the recent
list), the inspector panel bound to an `InspectorModel`, and F6 pane cycling
over the roving cursor. Write `OB-CLR-TEXT`, `OB-CLR-PALETTE`,
`OB-CLR-INSPECTOR`, and the remaining scenarios of `furniture.feature`.

### EP-M6: the hello-window application: surface, resources, and export

Assigned: `REQ-APP`, `REQ-FAIL`, `GAP-NO-APP`; decisions R2, R5, T5, T7, T11.

Create `crates/cabochon-hello-window` and add its rows to the architecture
guards. Move the bundle (decision T5) and embed it. Implement the `run`,
`export`, and `review` subcommands; `--locale`, `--shell headless`, `--script`,
`--journal`, `--size`, `--scale`, `--clock`, `--exit-after-first-frame`,
`--export-dir`, `--resources`, and `--fonts`; the document scene and its two
buttons; export and review with staging; the manifest; the script reader and
the journal writer; and environment and clock ports through `mockable`. `main`
returns `eyre::Result`; every other function returns a typed error. Write
`OB-APP-RESOURCES`, `OB-APP-SURFACE`, `OB-APP-EXPORT`, and the first five
scenarios of `hello_window.feature`. Record the export time for `DEP-3`.

### EP-M7: the hello-window application: view state, diagnostics, telemetry

Assigned: `REQ-APP`, `REQ-OBS`; decisions R7, R8, R14.

Implement zoom and pan, dark appearance and `--appearance`, Diagnostics, the
`tracing` subscriber writing to standard error, and the metrics recorder. Write
`OB-APP-E2E` and the remaining scenario. Correct the `mockable` version and the
`temp_env` advice in
`docs/reliable-testing-in-rust-via-dependency-injection.md`.

### EP-M8: the hosted shell adapter and its CI job

Assigned: `REQ-HOST`, `REQ-A11Y`, `GAP-NO-HOSTED-CI`; decisions R11, R13.

Create `crates/cabochon-clerestory-hosted` and add its rows to the architecture
guards: the `winit` event loop (`ApplicationHandler`), `softbuffer`
presentation of `InteractiveSurface` frames with damage, scale handling in
1/120 units, key, pointer, wheel, and IME mapping, close requests, host
appearance, wake deadlines, script injection, and `accesskit_winit` publication
with action requests. All mapping is in pure, unit-tested functions;
`event_loop` and `present` together stay under 300 lines. Write `OB-HOST-A11Y`.
Hosted end-to-end tests start the binary with `assert_cmd` and carry
`#[ignore = "needs Wayland; run make test-hosted"]`.

Add `scripts/run_hosted_tests.py`, a `uv` script following
`docs/scripting-standards.md`, with tests under `scripts/tests/` that use a
fake compositor command. Given a command after `--`, it:

1. creates a runtime directory with `mkdtemp` (mode 0700, prefix
   `cabochon-hosted-`);
2. starts `dbus-run-session` in a new process group, and inside it
   `weston --backend=headless --no-config --width=800 --height=600` with a
   socket name unique to the run;
3. waits up to 10 seconds for the socket, checking that `weston` is still
   alive, and prints `weston`'s log if it is not;
4. runs the command with `WAYLAND_DISPLAY` and `XDG_RUNTIME_DIR` set and
   `DISPLAY` removed from the child's environment, under a 10-minute deadline;
5. on every exit path sends TERM, then KILL after 5 seconds, to that process
   group only, and removes the runtime directory. It touches no other process
   or directory.

Locally, `weston` comes from distribution packages; where they are missing, a
rootless Podman container may run `weston` alone with the runtime directory
mounted, and never `cargo` (C11).

Add the `test-hosted` target. It depends on `check-build-tools`, fails with a
clear message when `cargo-nextest` is absent, and has one recipe line that sets
`RUSTFLAGS` in the same inherited form as the `test` recipe and runs:

```bash
uv run scripts/run_hosted_tests.py -- cargo nextest run --run-ignored only \
  -p cabochon-clerestory-hosted -p cabochon-hello-window
```

Add `test-hosted` to `DEVELOPMENT_TARGETS` in
`tests/build_standard_support/make.rs` and to `.PHONY`. Add a `hosted` group to
`.config/nextest.toml` (`max-threads = 1`, a 60-second slow timeout,
terminating after three periods). Add a CI job `hosted` on the pinned runner
image that installs `weston`, `dbus-daemon`, `at-spi2-core`, and
`libxkbcommon0`; sets up Rust through the pinned `setup-rust` action with
`install-mold: 'true'`, `install-clang-lld: 'true'`, `expect-cache: any`, and
its own `sccache-cache-discriminator`; installs `cargo-nextest` with the
install step the build-backend contract enforces; and runs `make test-hosted`.
Keep every workflow contract test green. Write `OB-CLR-HOST`. Accept ADR 0012.

### EP-M9: documentation reconciliation and step closure

Update `docs/cabochon-design.md` §8.1 with the Clerestory contract summary, the
focus model, and the proof inventory; propose the Figure 1 bridge edges
(decision D12); add the §15 rows for ADR 0012 and for the deferrals and
departures. Replace the template text in `docs/users-guide.md` with a
hello-window section (`run`, `export`, `review`, `--locale`, `--fonts`,
`--resources`, a keyboard reference, and `escutcheon-check`). Finish
`docs/developers-guide.md` (the session model, the adapters, the contract
probe, the script and journal formats, `make test-hosted` and its provisioning,
the hosted residuals, and the proof inventory). Update
`docs/repository-layout.md` and `docs/contents.md`. Record the guidelines
checklist result for `hello-window` in the pull request: each applicable
**must** with its evidence, each `DEP-` departure, and each inapplicable rule
with its false predicate. Propose the roadmap task for host clipboard and
drag-and-drop under step 5.1; mark roadmap tasks 1.0.1 and 1.0.2 done;
reconcile every discovery with the conformance basis; and set this plan's
status to `COMPLETE`.

## Milestones and plateaus

At each boundary, check: the assigned requirements are met; the design and ADRs
are still followed; no upstream assumption was falsified; no public interface,
dependency, or persisted format was added beyond this plan; and the trace links
are current. Compatibility decision for every milestone: none. All interfaces
are new and unreleased, so each change updates its callers together. Recovery
for every milestone: revert its pull request; earlier plateaus stand.

- EP-M0. Outcome: the uncertainties are resolved; nothing is committed but
  this plan's notes, or the R16 fallback. Remaining gaps: all implementation.
- EP-M1. Outcome: a session runs behind the headless shell with focus,
  command routing, key routing, and failure handling. Evidence: `OB-CLR-FOCUS`,
  `OB-CLR-MODAL`, `OB-CLR-KEYS`, `OB-CLR-ROUTE`, `OB-CLR-OBS`, `OB-CLR-FAIL`,
  and the guard rows. Remaining gaps: proof, furniture.
- EP-M2. Outcome: the focus kernel is proved, and the proofs run on pull
  requests that touch it. Evidence: `make verus`, `OB-PROOF-MIRROR`.
- EP-M3. Outcome: layout, theme, and view arithmetic exist and the probe
  bundle compiles. Evidence: `OB-CLR-LAYOUT`, `OB-CLR-VIEW`.
- EP-M4. Outcome: furniture is drawn, operated, and described to assistive
  technology. Evidence: `OB-CLR-WINDOW`, `OB-CLR-MENU`, `OB-CLR-DIALOG`,
  `OB-CLR-STATUS`, `OB-CLR-A11Y`.
- EP-M5. Outcome: text input, the palette, and the inspector panel work in
  the contract probe. Evidence: `OB-CLR-TEXT`, `OB-CLR-PALETTE`,
  `OB-CLR-INSPECTOR`, and all of `furniture.feature`.
- EP-M6. Outcome: `hello-window` runs headless with the chapter 36 surface,
  exports, and reviews. Completes the `RM-1.0.2` success criterion. Evidence:
  `OB-APP-RESOURCES`, `OB-APP-SURFACE`, `OB-APP-EXPORT`.
- EP-M7. Outcome: the application is complete under the headless shell.
  `TOR-SM-GEM` holds except for the hosted window. Evidence: `OB-APP-E2E` and
  all of `hello_window.feature`.
- EP-M8. Outcome: the application runs under a real compositor. Evidence:
  `OB-HOST-A11Y`; `OB-CLR-HOST` locally and in CI; coverage ratchet green.
- EP-M9. Outcome: documentation and roadmap match the delivered contracts;
  roadmap 1.0.1 and 1.0.2 are marked done; step 1.0 is complete.

## Concrete steps

Run from the repository root. Gates run one at a time, in this order.

```bash
set -euo pipefail

BRANCH=$(git branch --show-current | tr '/' '-')
make check-fmt 2>&1 | tee "/tmp/check-fmt-cabochon-${BRANCH}.out"
make lint 2>&1 | tee "/tmp/lint-cabochon-${BRANCH}.out"
make test 2>&1 | tee "/tmp/test-cabochon-${BRANCH}.out"
make markdownlint 2>&1 | tee "/tmp/markdownlint-cabochon-${BRANCH}.out"
make spelling 2>&1 | tee "/tmp/spelling-cabochon-${BRANCH}.out"
make nixie 2>&1 | tee "/tmp/nixie-cabochon-${BRANCH}.out"
```

From EP-M2, and from EP-M8:

```bash
set -euo pipefail

BRANCH=$(git branch --show-current | tr '/' '-')
make install-verus
make verus 2>&1 | tee "/tmp/verus-cabochon-${BRANCH}.out"
make test-hosted 2>&1 | tee "/tmp/test-hosted-cabochon-${BRANCH}.out"
```

Focused commands, with the result to expect before and after each change:

```plaintext
Working directory: repository root
Red:   cargo nextest run -p cabochon-clerestory focus_machine -> the reference model disagrees on a scoped Tab
Green: the same command -> all cases pass and the class counters meet their minimums
Red:   make verus -> open goals for the Clerestory lemmas
Green: make verus -> every function verified, 0 errors
Red:   cargo nextest run -p cabochon-hello-window hello_window -> the scenario finds no window
Green: the same command -> six scenarios pass
```

From EP-M6:

```bash
# `review` refuses a foreign non-empty directory, so each run takes a fresh one.
REVIEW_DIR=$(mktemp -d)
cargo run -p cabochon-hello-window -- review --out "$REVIEW_DIR"
ls "$REVIEW_DIR"
```

```plaintext
accessibility.json  clipboard  content-print.pdf  content.pdf  content.png  export.json  hit-map.json  thumbnail.png
```

The `msrv` CI job must keep passing (constraint C7). Reproduce it from outside
the checkout:

```bash
rustup toolchain install 1.85.0 --profile minimal
(cd /tmp && cargo +1.85.0 check --lib --all-features --locked \
  --manifest-path "$OLDPWD/Cargo.toml")
```

## Validation and acceptance

- Tests: `make test` passes with every obligation's tests,
  `furniture.feature`, and `hello_window.feature`; `make test-hosted` passes
  locally and in CI. Each new test is recorded failing before its
  implementation.
- Verification: `make verus` reports every function in the Clerestory proof
  file verified with no error; `OB-PROOF-MIRROR` finds no banned construct; the
  two seeded proof faults and every seeded test fault were seen to fail and
  were reverted.
- Lint and formatting: `make check-fmt` and `make lint` pass with no
  `#[expect]` in this plan's crates outside the hosted float boundary.
- Architecture: the four crates' rows of `OB-ARCH-DEPS`, `OB-ARCH-IMPORTS`,
  and `OB-ARCH-PUBAPI` pass, and each row's seeded violation (Clerestory
  depending on `winit`, a Clerestory module outside `kernel` importing `vstd`,
  and a public Clerestory function returning an AccessKit type) fails.
- Security: no `unsafe`, no direct process-environment access, and
  `make audit` passes after each dependency addition.
- Documentation: `make markdownlint`, `make nixie`, and `make spelling` pass.
- Behaviour, automated: under headless `weston`, `make test-hosted` shows
  the window presented from resources and the same session journal as the
  headless shell. This is the regression guarantee, with the scope stated in
  decision R13.
- Behaviour, manual: in a real GNOME or KDE Plasma session with Orca running
  (which enables the accessibility bus), keys and pointer operate every
  control, Accerciser lists the roles, names, states, and document text, the
  window shows one title, and typing in the palette through an input method
  composes text. Record the desktop, its version, and the outcome in
  `Artefacts and notes`. This is a one-off check backing `AXIOM-WINIT`, not a
  regression guarantee; automated GNOME-like and KDE-like acceptance belongs to
  roadmap 5.1.4.
- Performance: no threshold is set beyond `DEP-3`'s export time. EP-M8
  records the cold-start time of `--exit-after-first-frame` once, as a baseline
  for later work.

## Idempotence and recovery

Every step is additive and rerunnable. `export` and `review` stage beside their
target, rename into place, refuse foreign non-empty directories without
`--force`, and remove a stale staging directory of their own on the next run.
`make test-hosted` stops only the processes it started, on every exit path.
Seeded faults are reverted with `git restore` before any commit, and
`git status` must be clean of them. Accept an `insta` snapshot change only
after confirming an intended contract change.

## Artefacts and notes

Record here, labelled by milestone and revision: the implementation base
commit; EP-M0's probe transcripts, the delivered sibling type names, and the
D11 route; red, green, and refactor transcripts; the open goals from the first
`make verus` run; seeded-fault transcripts; the export time; the hosted test
output; and the manual desktop check.

Reconnaissance observations (2026-11-21, base `d295397`): the facts in
`Surprises & discoveries` dated today, and the crate versions in the next
section, were read from crates.io and upstream documentation on that date.

Script format (`--script FILE`): UTF-8 text, one step per line, `#` starting a
comment. Steps: `key CHORD` (for example `key Control+K`); `text STRING`;
`label STRING-KEY`, which types that key's string in the active locale;
`ime-preedit STRING`, `ime-commit STRING`, and `ime-disable`; `click NAME-KEY`,
a press and release at the centre of the named node; `a11y ACTION NAME-KEY`;
`configure WIDTH HEIGHT SCALE`; and `advance MILLISECONDS`, which moves the
injected clock and delivers due wakes. An unknown step is an error naming its
line.

Journal format (`--journal FILE`): UTF-8 text, one entry per line as a kind
followed by `key=value` fields in a fixed order. The `[session]` section holds
`dispatch` (command, source, outcome), `focus` (owner name key, scope),
`announce` (string key), `shell-error` (kind), and `tree` (node count and a
64-bit FNV-1a digest of the tree's canonical text: role, name key, states,
bounds in logical units, in order). The `[host]` section holds the shell name,
frames presented, the configured size and scale, and the metrics snapshot. The
session sink keeps at most the latest 4,096 entries when no file is given.

## Interfaces and dependencies

The signatures below are the contract at the end of EP-M5. Names of Lapidary,
Burin, and Escutcheon types are those of the sibling plans; EP-M0 records any
that were delivered under another name. `cabochon-clerestory` owns every item
here. Applications use it through `Application` and `Session`, shell adapters
through `HostShell`, and no other crate reimplements focus, routing, layout, or
furniture. The kernel is crate-private.

```rust,no_run
pub trait Application {
    fn commands(&self) -> &[CommandId];
    fn handle_command(&mut self, command: &CommandId, context: &mut CommandContext) -> CommandOutcome;
    fn command_state(&self, command: &CommandId) -> CommandState;
    fn content(&self, view: &ViewContext) -> Option<ContentFrame>;
    fn inspector(&self, window: WindowId) -> Option<InspectorModel>;
    fn commit_property(&mut self, key: &PropertyKey, text: &str, context: &mut CommandContext) -> PropertyOutcome;
}

pub enum HostEvent {
    Configured { window: HostWindowId, size: DeviceSize, scale: Scale120, appearance: Option<Appearance> },
    CloseRequested { window: HostWindowId },
    Key { window: HostWindowId, key: KeyEvent },
    Pointer { window: HostWindowId, event: PointerEvent },
    Ime { window: HostWindowId, event: ImeEvent },
    Accessibility { window: HostWindowId, node: NodeId, action: AccessibilityAction },
    FocusChanged { window: HostWindowId, focused: bool },
    RedrawRequested { window: HostWindowId },
    Woken(WakeToken),
}

pub trait HostShell {
    fn capabilities(&self) -> HostCapabilities;
    fn open_window(&mut self, spec: &WindowSpec) -> Result<HostWindowId, ShellError>;
    fn present(&mut self, window: HostWindowId, frame: &FrameUpdate<'_>) -> Result<(), ShellError>;
    fn request_redraw(&mut self, window: HostWindowId) -> Result<(), ShellError>;
    fn publish_accessibility(&mut self, window: HostWindowId, tree: &AccessibilityTree) -> Result<(), ShellError>;
    fn set_ime_allowed(&mut self, window: HostWindowId, allowed: bool) -> Result<(), ShellError>;
    fn set_ime_cursor_area(&mut self, window: HostWindowId, area: PixelRect) -> Result<(), ShellError>;
    fn request_wake(&mut self, after: WakeDelay, token: WakeToken) -> Result<(), ShellError>;
    fn close_window(&mut self, window: HostWindowId) -> Result<(), ShellError>;
}

pub trait JournalSink {
    fn record(&mut self, entry: JournalEntry);
}

pub struct FrameUpdate<'a> { pub scene: &'a ResolvedScene, pub damage: &'a Damage, pub mapping: &'a DeviceMapping }

pub struct SessionParts {
    pub resources: ResourceBundle,
    pub layout: Box<dyn TextLayout>,
    pub locale: LocaleChain,
    pub journal: Box<dyn JournalSink>,
}

impl<A: Application> Session<A> {
    pub fn new(application: A, parts: SessionParts) -> Result<Self, SessionError>;
    pub fn handle(&mut self, event: HostEvent, shell: &mut dyn HostShell) -> Result<SessionStatus, SessionError>;
}

pub enum Role {
    Window, MenuBar, Menu, MenuItem, MenuItemCheckBox, Dialog, AlertDialog, Button,
    Label, TextInput, Group, Pane, Document, Status, ListBox, ListBoxOption,
}
```

Supporting types, all plain data owned by Clerestory:

- `CommandState`: `Available`, `Unavailable`, or `Inapplicable`, with an
  optional checked mark. The reason for `Unavailable` is the record's
  `unavailable` string.
- `CommandOutcome`: `Handled`, `Unhandled`, or `Vetoed`. `CommandContext`
  lets a handler invalidate content, open a standard sheet, post a status
  message, and read the clock and host capabilities.
- `PropertyOutcome`: `Accepted` or `Rejected` with a string key.
- `ContentFrame`: a shared `Scene` and a revision number. `ViewContext`: the
  document plane's size, scale, zoom, pan, and appearance.
- `InspectorModel`: a title key and rows of key, label key, text value, and
  read-only flag.
- `WindowSpec`: title, minimum and initial size in logical units, and role.
  `WindowId` is the session's identifier; `HostWindowId` the shell's.
- `DeviceSize`: width and height in device pixels. `WakeDelay`:
  milliseconds. `WakeToken` and `NodeId`: opaque 64-bit identifiers.
- `HostCapabilities`: whether the host can print, and its input-method
  support.
- `AccessibilityTree`: nodes with identifier, role, name, `States`
  (unavailable, checked, expanded, selected, busy, read-only, modal, polite
  live region), bounds, children, and the focused node. `AccessibilityAction`:
  focus, activate, or set value.
- `ShellError`: a kind (`WindowLost`, `PresentFailed`,
  `AccessibilityFailed`, `Unsupported`) and a message. `SessionStatus`:
  `Running` or `Exited`. `SessionError`: invalid resources at start (missing
  command, missing standard string, oversized menu bar or sheet) or a lost
  shell.

Behavioural contract of `handle`: it first computes the state transition and a
list of effects with no shell access, then applies the effects. A `WindowLost`
effect failure ends the session with `Exited`. Any other effect failure is
journalled and counted, the window is marked for a full redraw on the next
`RedrawRequested`, and `handle` returns `Running`; state is never rolled back.
A content scene that fails to resolve, and a Burin `RenderError` mapped to
`PresentFailed` by the hosted adapter, both lead to a bounded diagnostic scene
in the document plane (`TDD-FAIL`). `FrameUpdate` carries a resolved scene,
never pixels; the hosted adapter rasterizes through Burin's
`InteractiveSurface`, and the session embeds the content scene as a subscene of
the furniture scene. Error enums use `thiserror`; a large payload sits behind
`Box`. A shortcut matches a `KeyEvent` by Escutcheon's canonical form, falling
back to the physical key on non-Latin layouts, as the 1.0.2 plan defines.

Every bundle must define Clerestory's standard strings (the palette placeholder
and Recent heading, the no-help message, the content-failure message, and the
window-role words used in accessible names); session start fails on a missing
one, so no furniture string is hard-coded.

Dependencies (caret requirements; newest compatible release when each milestone
starts; versions read on 2026-11-21):

- `cabochon-clerestory`: `cabochon-lapidary`, `cabochon-escutcheon`,
  `thiserror` 2, `tracing` 0.1, `metrics` 0.24, `unicode-normalization` 0.1,
  and, on the single-source proof route only, `vstd` and `verus_builtin_macros`
  at the release the 1.0.3 plan pins.
- `cabochon-clerestory-headless`: `cabochon-clerestory`,
  `cabochon-lapidary`, `thiserror` 2.
- `cabochon-clerestory-hosted`: `cabochon-clerestory`, `cabochon-burin`,
  `cabochon-lapidary`, `winit` 0.30.13, `softbuffer` 0.4.8, `accesskit` 0.25.2,
  `accesskit_winit` 0.34.2, `thiserror` 2, `tracing` 0.1.
- `cabochon-hello-window`: the three crates above, `cabochon-escutcheon`,
  `cabochon-burin`, `cabochon-lapidary`, `cabochon-fonts`, `clap` 4, `eyre` 0.6,
  `serde` 1 and `serde_json` 1 (through `export::wire`), `tracing` 0.1,
  `tracing-subscriber` 0.3, `metrics` 0.24, `metrics-util` 0.20, `mockable` 3,
  `cap-std` 4, `camino` 1.
- Development: `rstest`, `rstest-bdd` and `rstest-bdd-macros` 0.6, `proptest`
  1, `proptest-state-machine` 0.9, `googletest` 0.14, `pretty_assertions` 1,
  `insta` 1, `mockall` 0.15, `accesskit_consumer` 0.39, `tracing-test` 0.2,
  `assert_cmd` 2, and `cabochon-substrate-testkit`, at the versions the 1.0.3
  plan's workspace already pins where it pins them.
- Script tooling: Python through `uv`, with the packages
  `docs/scripting-standards.md` prescribes.

## Documentation obligations

- ADR 0012 (in-window furniture, the host-shell port, the focus model and its
  proof, and the considered alternatives) under `docs/adrs/`, indexed in
  `docs/contents.md` and referenced from `docs/cabochon-design.md` §15.
- `docs/cabochon-design.md` §8.1: the Clerestory contract summary, focus
  model, and proof inventory; the Figure 1 bridge proposal; the §15 deferrals
  and departures.
- `docs/users-guide.md`: the hello-window section.
- `docs/developers-guide.md`: the session model, ownership and reuse policy
  for the four crates, the adapters, the contract probe, the script and journal
  formats, `make test-hosted`, the hosted residuals, and the proof inventory.
- `docs/repository-layout.md` and `docs/contents.md`: the new crates.
- `docs/reliable-testing-in-rust-via-dependency-injection.md`: the `mockable`
  version and the `temp_env` advice.
- `docs/roadmap.md`: 1.0.1 and 1.0.2 marked done; the proposed clipboard and
  drag-and-drop task.

## Revision notes

- 2026-10-23: created by decomposing the reviewed step-level plan (commit
  `616a8fa`), then revised the same day by the cross-plan consistency review
  and the architecture-guard decisions (T5 to T15).
- 2026-11-04: the reviewer approved the plan; milestones unchanged.
- 2026-11-05: three review rounds: scenario ordering, the automated and manual
  hosted checks, fresh export directories, and the `#[expect]` scope.
- 2026-11-21: refined for implementation readiness. Why: the UI guidelines
  became a required reference and redefine `hello-window`; the current ExecPlan
  standard requires a proof-sketch-first verification plan, per-category
  budgets, and non-vacuity evidence; and reconnaissance found stale claims
  (roadmap task numbers, the conformance base, `make` targets, the Cranelift
  state) and hosted-test limits (no real input, IME, or fractional scale under
  headless `weston`; lazy AccessKit activation). What changed: status returns
  to DRAFT; decisions R1 to R17, departures `DEP-1` to `DEP-7`, and amendments
  `AM-ESC-1` and `AM-BUR-1` are added; T8 is amended; the acceptance surface
  follows chapter 36, with a contract probe for inspector panels and text
  input; focus gains an abstract model and a Verus-proved kernel; the headless
  shell becomes its own crate; milestones grow from five to ten, delivered as
  stacked pull requests; and the plan is self-contained rather than deferring
  to the 1.0.3 plan. Effect on remaining work: all of it; no implementation had
  started.
- 2026-11-21: applied the expert panel's findings. The focus model now holds
  stable stop identifiers, gives frame kinds a meaning, lives wholly in the
  kernel, and gains `L-INIT-WF`, `L-SCOPE-ESCAPE`, and `L-EDIT-STABLE`. The
  port gains a commit path, assistive-technology actions, wake and redraw
  requests, and a failure policy, and loses the print request and the second
  text layout. View is flat. The palette follows chapter 23. The hosted claim
  is narrowed to what headless `weston` can show, and its runner, CI job, and
  journal comparison are specified. The two largest milestones are split, and
  the amendments have an owner and a fallback.
